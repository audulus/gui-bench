//! The live updates: the switch of the top bar, the tick that the metric cards show and the
//! spinner. Lorelei's application surface has no timer and no rotation, so both come from the
//! host's frame callback (`window::run`): while live is on, every refresh runs one turn that sets
//! the angle and the tick from the time since the switch was turned on, and the spinner is an
//! inline SVG square that the scene-level `svg_image` turns by the angle signal.

use std::f64::consts::TAU;
use std::time::Instant;

use bough::app::App;
use bough::prelude::*;
use bough::raw::{anchor, svg_image};

use super::{Dashboard, spec};

/// The spinner: a rounded square in the primary color.
const SQUARE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"><rect width="24" height="24" rx="6" fill="#4f46e5"/></svg>"##;

/// The live state.
pub struct Live {
    /// Whether live is on.
    pub on: bool,
    /// When live was turned on.
    since: Instant,
    /// The live intervals since then, 0 while live is off.
    pub tick: Signal<u64>,
    /// The angle of the spinner in radians.
    pub angle: Signal<f32>,
}

impl Live {
    pub fn new() -> Self {
        Self {
            on: false,
            since: Instant::now(),
            tick: Signal::new(0),
            angle: Signal::new(0.0),
        }
    }

    /// Turns live on or off; off brings the startup values back.
    pub fn toggle(&mut self) {
        self.on = !self.on;
        self.since = Instant::now();
        self.tick.set(0);
        self.angle.set(0.0);
    }

    /// Sets the angle and the tick for the current time.
    fn advance(&mut self) {
        let elapsed = self.since.elapsed().as_secs_f64();
        let period = spec::SPINNER_PERIOD_MS as f64 / 1000.0;
        self.angle.set((elapsed % period / period * TAU) as f32);
        let interval = spec::LIVE_INTERVAL_MS as f64 / 1000.0;
        self.tick.set((elapsed / interval) as u64);
    }
}

/// Runs after every frame of the window: one turn per refresh while live is on. A turn that
/// changes the window draws a frame, after which this runs again; with live off nothing is drawn
/// and the frames stop.
pub fn frame(app: &mut App<Dashboard>) {
    if app.owner.live.on {
        app.turn(|dashboard| dashboard.live.advance());
    }
}

/// The room of the spinner, and the turning square in it while live is on.
pub fn spinner(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let size = spec::SPINNER_SIZE;
    let square = dashboard.live.on.then(|| {
        let angle = sig!(dashboard: Dashboard => live.angle);
        anchor(
            size,
            size,
            svg_image(0.0, 0.0, size, size, SQUARE).angle(angle),
        )
    });
    row((square,)).size(size, size).shrink(0.0)
}
