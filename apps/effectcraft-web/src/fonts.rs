//! Fonts the page adds (AI-After Effects: the metric-compatible open fonts shipped in `fonts/`,
//! and fonts an organisation keeps on its server), for the page's text engine and its workers.
//!
//! The page registers each font with the text engine at once. Frame and render workers run their
//! own engine instance, so they get the same bytes as a file under [`PREFIX`] with their next job
//! ([`crate::frames`], [`crate::worker`]) and register it there ([`crate::worker::worker_file`]).

use std::cell::RefCell;
use std::sync::Arc;

/// Table paths of fonts sent to workers: `/__fonts__/<name>`. Never a footage path (a picked or
/// dropped file is `/<name>`).
pub const PREFIX: &str = "/__fonts__/";

/// The most fonts kept for workers (a runaway page can't fill the memory).
const MAX_FONTS: usize = 256;

thread_local! {
    static FONTS: RefCell<Vec<(String, Arc<[u8]>)>> = const { RefCell::new(Vec::new()) };
}

/// Register `bytes` as a font named `name`; returns the faces added (0: not a font, or already
/// added under that name).
pub fn add(name: &str, bytes: Vec<u8>) -> usize {
    let path = format!("{PREFIX}{name}");
    if FONTS.with(|f| f.borrow().iter().any(|(p, _)| *p == path)) {
        return 0;
    }
    let data: Arc<[u8]> = bytes.clone().into();
    let n = effectcraft_engine::text::fonts::add_font_data(bytes).len();
    if n > 0 {
        FONTS.with(|f| {
            let mut f = f.borrow_mut();
            if f.len() < MAX_FONTS {
                f.push((path, data));
            }
        });
    }
    n
}

/// Every font added so far: (table path, bytes), for a worker's next job.
pub fn all() -> Vec<(String, Arc<[u8]>)> {
    FONTS.with(|f| f.borrow().clone())
}

/// Whether a file sent to a worker is a font (registered with the text engine, not the media pool).
pub fn is_font_path(path: &str) -> bool {
    path.starts_with(PREFIX)
}
