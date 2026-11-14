//! The window of every scenario: Lorelei's Wayland host with the configuration that
//! `Application::run` gives a window. The scenarios open it through `bough::backend::run`, the
//! entry below `Application::run`, since only that takes the application id and a frame
//! callback.

use std::sync::OnceLock;

use bough::app::{App, Config};
use bough::backend::Run;
use bough::prelude::View;
use bough::raw::Caps;

use crate::dashboard::spec;
use crate::font;

const APP_ID: &str = "gui-bench.lorelei";

/// The capacities `Application::run` gives its window.
const CAPS: Caps = Caps {
    instances: 1 << 16,
    pages: 1 << 13,
    glyphs: 1 << 19,
    runs: 1 << 16,
    spaces: 1 << 12,
    clips: 1 << 12,
    strikes: 16,
};

/// The font of the window, loaded once: the host asks for the configuration twice.
static FONT: OnceLock<Vec<u8>> = OnceLock::new();

/// Returns the configuration of the window at a display scale.
fn config(scale: f32) -> Config {
    let font = FONT.get_or_init(font::load).clone();
    let mut config = Config::new(spec::WINDOW.0, spec::WINDOW.1, scale, font);
    config.caps = CAPS;
    config
}

/// Opens the window with `title`, mounts `view` and runs it until the window closes. `frame`
/// runs after every frame that the window shows.
pub fn run<V: View>(title: &str, view: V, frame: fn(&mut App<V>)) -> ! {
    let after_frame = move |app: &mut App<V>, _seconds: f64| {
        frame(app);
        reclaim(app);
    };
    let run = Run::default().app_id(APP_ID).linked(after_frame);
    bough::backend::run(title, config, view, run)
}

/// Returns the elements that left the tree to Lorelei's free lists. Lorelei does that a few
/// thousand at a time in every turn and in every frame without damage, and its Wayland host runs
/// neither after input: without this, each page that the app replaces keeps its glyphs until the
/// glyph column is full and the app panics. An empty turn is the documented place of that work,
/// so the app runs empty turns after a frame until nothing waits.
fn reclaim<V: View>(app: &mut App<V>) {
    while app.reclaiming() {
        app.poll();
    }
}
