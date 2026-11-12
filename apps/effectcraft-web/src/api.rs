//! `window.effectcraft`: the desktop control channel (`docs/control-protocol.md`) as promises,
//! for agents and browser tests. `web/index.html` wraps the exported [`request`] as
//! `effectcraft.request(method, params)`, `effectcraft.execute(command, params)` and friends.
//!
//! AI-After Effects: the same exports also make `window.aiaftereffects` (`web/index.html`), the
//! bridge the assistant panel (`studio/`) drives the editor with:
//!
//! ```js
//! await aiaftereffects.execute("layer.newText", {text: "Заголовок"})  // a command → its result
//! await aiaftereffects.request("ui.inspect")           // a control method → its result
//! await aiaftereffects.commands()                       // every command with its params doc
//! aiaftereffects.open("Ролик.ecproj", bytes)            // open a project or import media
//! aiaftereffects.putFile("logo.png", bytes)             // a file a project uses, not imported
//! await aiaftereffects.addFont("LiberationSans.ttf", bytes)
//! aiaftereffects.wantsKeyboard()                        // a text field has the keyboard
//! ```
//!
//! Save and Save As of a project reach the page as `aiaftereffects:save` on `window`
//! (`detail: {name, path, saveAs, bytes}`) while `aiaftereffects.takesSaves` is true, unsaved
//! changes as `aiaftereffects:dirty` (`detail: {dirty}`), and the AI chip by text asks for the
//! page's AI menu with `aiaftereffects:ai` (`detail: {x, y}`, CSS pixels from the canvas's top
//! left).

use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};

use effectcraft_ui_egui::ControlRequest;
use serde_json::{Value, json};
use wasm_bindgen::prelude::*;

type Pending = (Receiver<Value>, js_sys::Function, js_sys::Function);

thread_local! {
    static TX: RefCell<Option<Sender<ControlRequest>>> = const { RefCell::new(None) };
    static PENDING: RefCell<Vec<Pending>> = const { RefCell::new(Vec::new()) };
    static INFO: RefCell<Value> = const { RefCell::new(Value::Null) };
}

pub(crate) fn set_sender(tx: Sender<ControlRequest>) {
    TX.with(|t| *t.borrow_mut() = Some(tx));
}

/// Environment facts reported by `effectcraft.info()`.
pub fn set_info(key: &str, v: Value) {
    INFO.with(|i| {
        let mut i = i.borrow_mut();
        if !i.is_object() {
            *i = json!({});
        }
        i[key] = v;
    });
}

fn to_js(v: &Value) -> JsValue {
    js_sys::JSON::parse(&v.to_string()).unwrap_or(JsValue::NULL)
}

fn from_js(v: &JsValue) -> Value {
    if v.is_undefined() || v.is_null() {
        return json!({});
    }
    js_sys::JSON::stringify(v).ok().and_then(|s| s.as_string()).and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(json!({}))
}

/// Send a control request to the app; the promise resolves with its `result` or rejects with its
/// `error`.
#[wasm_bindgen]
pub fn request(method: String, params: JsValue) -> js_sys::Promise {
    let params = from_js(&params);
    js_sys::Promise::new(&mut |resolve, reject| {
        let (req, rx) = ControlRequest::new(method.clone(), params.clone());
        let sent = TX.with(|t| t.borrow().as_ref().is_some_and(|tx| tx.send(req).is_ok()));
        if !sent {
            let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("EffectCraft is not running yet"));
            return;
        }
        PENDING.with(|p| p.borrow_mut().push((rx, resolve, reject)));
        crate::repaint();
    })
}

/// Files in the in-memory file table: `[{path, size}]` (picked, dropped, saved and rendered files).
#[wasm_bindgen]
pub fn files() -> JsValue {
    to_js(&Value::Array(crate::files::list().into_iter().map(|(path, size)| json!({"path": path, "size": size})).collect()))
}

/// A file's bytes from the table (e.g. a render), or `undefined`.
#[wasm_bindgen(js_name = readFile)]
pub fn read_file(path: String) -> JsValue {
    crate::files::get(&path).map(|d| js_sys::Uint8Array::from(&d[..]).into()).unwrap_or(JsValue::UNDEFINED)
}

/// The largest file the page may hand over at once.
const MAX_FILE_BYTES: u32 = 2048 * 1024 * 1024 - 1;

/// A file name the page passed: its last component, trimmed, at most 255 characters, `fallback`
/// when empty.
fn clean_name(name: &str, fallback: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let name: String = base.trim().chars().filter(|c| !c.is_control()).take(255).collect();
    if name.is_empty() || name == "." || name == ".." { fallback.to_string() } else { name }
}

/// Put bytes into the file table under `/<name>` without opening or importing them: the media a
/// project uses, fetched from the server before the project opens. Returns the table path.
#[wasm_bindgen(js_name = putFile)]
pub fn put_file(name: String, data: js_sys::Uint8Array) -> Result<String, JsValue> {
    if data.length() > MAX_FILE_BYTES {
        return Err(js_sys::Error::new("the file is too large").into());
    }
    let path = crate::files::path_for(&clean_name(&name, "file"));
    crate::files::put(&path, data.to_vec().into());
    Ok(path)
}

/// Add a font for every text layer (the metric-compatible fonts shipped with the page, and the
/// fonts an organisation keeps on its server). The frame and render workers get it with their
/// next job. Returns the number of faces added (0 when the bytes are not a font).
#[wasm_bindgen(js_name = addFont)]
pub fn add_font(name: String, data: js_sys::Uint8Array) -> Result<u32, JsValue> {
    if data.length() > 64 * 1024 * 1024 {
        return Err(js_sys::Error::new("the font is too large").into());
    }
    let name = clean_name(&name, "font.ttf");
    let n = crate::fonts::add(&name, data.to_vec());
    crate::repaint();
    Ok(u32::try_from(n).unwrap_or(u32::MAX))
}

/// Whether a text field of the editor has the keyboard (the page then leaves keys alone).
#[wasm_bindgen(js_name = wantsKeyboard)]
pub fn wants_keyboard() -> bool {
    crate::text_edit_focused()
}

/// Tell the page something happened: `aiaftereffects:<what>` on `window`, with `detail`.
pub(crate) fn notify(what: &str, detail: &Value) -> Result<(), String> {
    let window = web_sys::window().ok_or("no window")?;
    let init = web_sys::CustomEventInit::new();
    init.set_detail(&to_js(detail));
    let event = web_sys::CustomEvent::new_with_event_init_dict(&format!("aiaftereffects:{what}"), &init).map_err(|e| format!("{e:?}"))?;
    window.dispatch_event(&event).map(|_| ()).map_err(|e| format!("{e:?}"))
}

/// Whether the page said it keeps saved projects itself (`aiaftereffects.takesSaves = true`).
pub(crate) fn page_takes_saves() -> bool {
    web_sys::window()
        .and_then(|w| js_sys::Reflect::get(&w, &"aiaftereffects".into()).ok())
        .filter(|o| o.is_object())
        .and_then(|o| js_sys::Reflect::get(&o, &"takesSaves".into()).ok())
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

/// Hand a saved project to the page (`aiaftereffects:save`), which keeps it in the user's
/// projects on the server. `Ok(false)` when the page doesn't take saves (it downloads instead).
pub(crate) fn save_to_page(path: &str, bytes: &[u8]) -> Result<bool, String> {
    if !page_takes_saves() {
        return Ok(false);
    }
    let window = web_sys::window().ok_or("no window")?;
    let detail = js_sys::Object::new();
    let put = |k: &str, v: &JsValue| js_sys::Reflect::set(&detail, &k.into(), v).map(|_| ()).map_err(|e| format!("{e:?}"));
    let name = path.rsplit('/').next().filter(|n| !n.is_empty()).unwrap_or("project.ecproj");
    put("name", &name.into())?;
    put("path", &path.into())?;
    put("saveAs", &JsValue::FALSE)?;
    put("bytes", &js_sys::Uint8Array::from(bytes).into())?;
    let init = web_sys::CustomEventInit::new();
    init.set_detail(&detail);
    let event = web_sys::CustomEvent::new_with_event_init_dict("aiaftereffects:save", &init).map_err(|e| format!("{e:?}"))?;
    window.dispatch_event(&event).map(|_| true).map_err(|e| format!("{e:?}"))
}

/// Whether the project has unsaved changes, told to the page when it changes.
pub(crate) fn dirty(now: bool) {
    thread_local! {
        static LAST: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    }
    if LAST.with(|l| l.replace(Some(now))) != Some(now) {
        let _ = notify("dirty", &json!({ "dirty": now }));
    }
}

/// Ask the page to open its AI menu at this point of the canvas (CSS pixels from its top left).
pub(crate) fn ai_menu(x: f32, y: f32) {
    let _ = notify("ai", &json!({ "x": x, "y": y }));
}

/// Put bytes into the file table under `/<name>` and open (`.ecproj`) or import them. Resolves
/// with the table path.
#[wasm_bindgen(js_name = addFile)]
pub fn add_file(name: String, data: js_sys::Uint8Array) -> String {
    let path = crate::files::path_for(&clean_name(&name, "file"));
    crate::files::put(&path, data.to_vec().into());
    let p = path.clone();
    crate::post(move |app| crate::open_or_import(app, vec![p]));
    path
}

/// Backend and build facts.
#[wasm_bindgen]
pub fn info() -> JsValue {
    let mut v = INFO.with(|i| i.borrow().clone());
    if !v.is_object() {
        v = json!({});
    }
    v["version"] = json!(env!("CARGO_PKG_VERSION"));
    v["threads"] = json!(false);
    v["workers"] = from_js(&crate::worker::stats());
    v["audio"] = from_js(&crate::audio::state());
    if let Some(e) = crate::persist::last_error() {
        v["storage"]["error"] = json!(e);
    }
    to_js(&v)
}

/// Save the project to browser storage without downloading it: `path` (default: its current
/// path, or `/<project name>.ecproj`). Resolves with `{path, bytes}`.
#[wasm_bindgen(js_name = saveToBrowser)]
pub fn save_to_browser(path: Option<String>) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, reject| {
        let path = path.clone();
        crate::post(move |app| {
            let p = path.or_else(|| app.session.path.clone()).unwrap_or_else(|| format!("/{}.ecproj", app.session.project_name()));
            let p = if p.starts_with('/') { p } else { format!("/{p}") };
            crate::files::save_quietly(&p);
            match app.session.execute("file.saveAs", json!({"path": p})) {
                Ok(v) => {
                    let _ = resolve.call1(&JsValue::NULL, &to_js(&v));
                }
                Err(e) => {
                    let _ = reject.call1(&JsValue::NULL, &JsValue::from_str(&e.to_string()));
                }
            }
        });
    })
}

/// What browser storage holds: `{backend, usage, quota, persisted, files: [{path, size,
/// modified}], config: [name]}` (render outputs, which are not stored, are left out).
#[wasm_bindgen(js_name = listStored)]
pub async fn list_stored() -> JsValue {
    let est = wasm_bindgen_futures::JsFuture::from(crate::persist::store_estimate()).await.map(|v| from_js(&v)).unwrap_or(json!({}));
    let files: Vec<Value> = crate::files::STORE
        .files()
        .into_iter()
        .filter(|f| f.3)
        .map(|(path, size, modified, _)| json!({"path": path, "size": size, "modified": modified}))
        .collect();
    let config: Vec<Value> =
        crate::files::STORE.lock().list(crate::store::CONFIG).into_iter().map(|(k, ..)| json!(k.trim_start_matches(crate::store::CONFIG))).collect();
    let mut v = est;
    v["files"] = json!(files);
    v["config"] = json!(config);
    v["pending"] = json!(crate::files::STORE.lock().has_pending());
    to_js(&v)
}

/// Remove a stored file (media, project, auto-save). Resolves with whether it existed.
#[wasm_bindgen(js_name = removeStored)]
pub fn remove_stored(path: String) -> bool {
    crate::files::STORE.remove_file(&path)
}

/// Resolves once every change is written to browser storage; rejects when a write failed (it is
/// tried again).
#[wasm_bindgen]
pub fn flush() -> js_sys::Promise {
    crate::persist::flushed()
}

/// Settle the promises whose replies arrived (called every frame).
pub(crate) fn poll_replies() {
    PENDING.with(|p| {
        p.borrow_mut().retain(|(rx, resolve, reject)| match rx.try_recv() {
            Ok(v) => {
                if v["ok"].as_bool() == Some(true) {
                    let _ = resolve.call1(&JsValue::NULL, &to_js(&v["result"]));
                } else {
                    let msg = v["error"].as_str().map(str::to_string).unwrap_or_else(|| v.to_string());
                    let _ = reject.call1(&JsValue::NULL, &JsValue::from_str(&msg));
                }
                false
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => true,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("request dropped"));
                false
            }
        });
    });
}
