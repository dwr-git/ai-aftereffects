//! AI-After Effects: the engine's toasts and the status line's notices in Russian.
//!
//! The engine writes its toasts in English (`Session::toast`); the window shows them through
//! [`translate`], which knows their wording and keeps the values (names, counts) they carry. A path
//! is shown as its file name: a project saved to «Мои проекты» lives on the server, and the browser
//! file table's `/name` means nothing to the user. Text it doesn't know stays as it is.

use super::ui::current;

/// The last component of a path (`/Ролик.ecproj` → `Ролик.ecproj`).
fn base(path: &str) -> &str {
    let p = path.trim().trim_end_matches(['.', '/']);
    p.rsplit(['/', '\\']).next().filter(|n| !n.is_empty()).unwrap_or(p)
}

/// `"{n} rest"` → `(n, rest)` when the text starts with a number.
fn count<'a>(text: &'a str, rest: &str) -> Option<&'a str> {
    let (n, tail) = text.split_once(' ')?;
    (n.bytes().all(|b| b.is_ascii_digit()) && !n.is_empty() && tail == rest).then_some(n)
}

/// `message` in the interface language (Russian), or as it is.
pub(crate) fn translate(message: &str) -> String {
    if current() != "ru" {
        return message.to_string();
    }
    ru(message).unwrap_or_else(|| message.to_string())
}

fn ru(m: &str) -> Option<String> {
    const EXACT: &[(&str, &str)] = &[
        ("Purged undo history", "История отмен очищена"),
        ("No expression errors", "Ошибок в выражениях нет"),
        ("Copied frame to the clipboard", "Кадр скопирован в буфер обмена"),
        ("The system is low on memory: cache sizes were reduced", "В системе мало памяти: размеры кэшей уменьшены"),
        ("Render finished", "Рендеринг завершён"),
        (
            "Classic 3D doesn't light Environment light layers: they light with the Advanced 3D renderer",
            "Классический 3D не освещает слоями света «Окружение»: они светят в расширенном 3D-рендеринге",
        ),
        (
            "Classic 3D doesn't draw 3D model layers: they show with the Advanced 3D renderer",
            "Классический 3D не рисует слои 3D-моделей: они видны в расширенном 3D-рендеринге",
        ),
    ];
    if let Some((_, r)) = EXACT.iter().find(|(e, _)| *e == m) {
        return Some((*r).to_string());
    }
    // Prefix + value (path shown as its file name where `true`).
    const PREFIX: &[(&str, &str, bool)] = &[
        ("Saved an XML copy to ", "Копия в XML сохранена: ", true),
        ("Saved a multi-layer EXR to ", "Многослойный EXR сохранён: ", true),
        ("Saved frame to ", "Кадр сохранён: ", true),
        ("Saved preview to ", "Просмотр сохранён: ", true),
        ("Saved template ", "Шаблон сохранён: ", false),
        ("Auto-saved to ", "Автосохранение: ", true),
        ("Auto-saved ", "Автосохранено: ", true),
        ("Imported template ", "Шаблон импортирован: ", false),
        ("Exported template ", "Шаблон экспортирован: ", false),
        ("New project from template ", "Новый проект из шаблона ", false),
        ("Expression disabled: ", "Выражение выключено: ", false),
        ("Tutorial complete: ", "Урок пройден: ", false),
        ("Shortcut conflict: also used by ", "Сочетание клавиш уже занято: ", false),
        ("Watching ", "Слежу за папкой: ", true),
        ("Proxy set from ", "Прокси: ", true),
        ("Cleared browser storage: ", "Хранилище браузера очищено: ", false),
        ("Scheduled script task: ", "Задача сценария: ", false),
        ("The default shortcuts can't be changed: created the preset ", "Стандартные сочетания менять нельзя: создан набор ", false),
        (
            "This project had values outside EffectCraft's limits, corrected when it opened: ",
            "В проекте были значения вне допустимых пределов, они исправлены при открытии: ",
            false,
        ),
    ];
    for (en, r, path) in PREFIX {
        if let Some(rest) = m.strip_prefix(en) {
            return Some(format!("{r}{}", if *path { base(rest) } else { rest }));
        }
    }
    if m.starts_with("This project was saved by EffectCraft ") {
        return Some("Проект сохранён более новой версией программы: настройки, которых эта версия не знает, пропадут при сохранении.".into());
    }
    if let Some(rest) = m.strip_prefix("Imported ") {
        // "Imported {path} ({n} warnings)"
        return Some(match rest.rsplit_once(" (") {
            Some((p, w)) => format!("Импортировано: {} ({}", base(p), w.replace("warnings", "предупр.")),
            None => format!("Импортировано: {}", base(rest)),
        });
    }
    if let Some(rest) = m.strip_prefix("Saved ") {
        // "Saved {n} layers to {path}" or "Saved {path}".
        if let Some((n, p)) = rest.split_once(" layers to ")
            && n.bytes().all(|b| b.is_ascii_digit())
        {
            return Some(format!("Слои сохранены ({n}): {}", base(p)));
        }
        return Some(format!("Сохранено: {}", base(rest)));
    }
    if let Some(rest) = m.strip_prefix("Purged ")
        && let Some(what) = rest.strip_suffix(" cache")
    {
        return Some(format!("Кэш очищен: {what}"));
    }
    if let Some(rest) = m.strip_prefix("Collected ")
        && let Some((n, folder)) = rest.split_once(" file(s) into ")
    {
        return Some(format!("Собрано файлов: {n} → {}", base(folder)));
    }
    if let Some(rest) = m.strip_prefix("Removed ") {
        if let Some(n) = count(rest, "unused footage item(s)") {
            return Some(format!("Удалено неиспользуемых элементов: {n}"));
        }
        if let Some(n) = count(rest, "item(s)") {
            return Some(format!("Удалено элементов: {n}"));
        }
    }
    if let Some(rest) = m.strip_prefix("Consolidated ")
        && let Some(n) = count(rest, "duplicate footage item(s)")
    {
        return Some(format!("Объединено повторяющихся элементов: {n}"));
    }
    if let Some(rest) = m.strip_prefix("Found ")
        && let Some(n) = count(rest, "footage item(s) in the project's new folder")
    {
        return Some(format!("Найдено в новой папке проекта: {n}"));
    }
    if let Some(rest) = m.strip_prefix("Also relinked ")
        && let Some(n) = count(rest, "other missing footage item(s)")
    {
        return Some(format!("Заодно найдено отсутствующих файлов: {n}"));
    }
    if let Some(rest) = m.strip_prefix("Loaded ") {
        if let Some(n) = count(rest, "effect plug-in(s)") {
            return Some(format!("Загружено подключаемых эффектов: {n}"));
        }
        if let Some(n) = count(rest, "effect(s) through a helper") {
            return Some(format!("Эффектов загружено через помощника: {n}"));
        }
    }
    if let Some(n) = count(m, "missing effect(s)") {
        return Some(format!("Отсутствующих эффектов: {n}"));
    }
    if let Some(n) = count(m, "layer(s) with missing fonts") {
        return Some(format!("Слоёв с отсутствующими шрифтами: {n}"));
    }
    if let Some(n) = count(m, "missing footage item(s)") {
        return Some(format!("Отсутствующих файлов: {n}"));
    }
    if let Some(n) = count(m, "effect(s) from effect helpers") {
        return Some(format!("Эффектов от помощников эффектов: {n}"));
    }
    if let Some((n, _)) = m.split_once(" footage item(s) missing")
        && n.bytes().all(|b| b.is_ascii_digit())
    {
        return Some(format!("Отсутствует файлов: {n} (Файл ▸ Зависимости ▸ Найти отсутствующие файлы)"));
    }
    if let Some((head, why)) = m.split_once(" marked missing: ")
        && let Some(n) = head.split(' ').next().filter(|n| n.bytes().all(|b| b.is_ascii_digit()))
    {
        return Some(format!("Отмечено отсутствующими: {n} ({})", why.trim_end_matches('.')));
    }
    if let Some(rest) = m.strip_prefix("Preview cut to ")
        && let Some((fit, tail)) = rest.split_once(" of ")
        && let Some(n) = tail.split(' ').next()
    {
        return Some(format!("Просмотр сокращён до {fit} из {n} кадров: больше не помещается в память (Настройки ▸ Память и процессор)"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::ru;

    #[test]
    fn toasts_read_in_russian_with_file_names() {
        assert_eq!(ru("Saved /Ролик.ecproj").as_deref(), Some("Сохранено: Ролик.ecproj"));
        assert_eq!(ru("Saved 3 layers to /a/b.json").as_deref(), Some("Слои сохранены (3): b.json"));
        assert_eq!(ru("Saved frame to /x/frame.png").as_deref(), Some("Кадр сохранён: frame.png"));
        assert_eq!(ru("Removed 2 unused footage item(s)").as_deref(), Some("Удалено неиспользуемых элементов: 2"));
        assert_eq!(ru("Imported /c.aep (2 warnings)").as_deref(), Some("Импортировано: c.aep (2 предупр.)"));
        assert_eq!(ru("Something else"), None);
        assert_eq!(ru(""), None);
        assert_eq!(ru("Saved "), Some("Сохранено: ".into()));
    }
}
