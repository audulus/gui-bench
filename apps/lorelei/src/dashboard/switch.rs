//! The switch. Lorelei has no component library, so the app builds it from two boxes: a rounded
//! track and a round knob at its left or right end.

use bough::prelude::*;

use super::spec;

/// The size of the knob and its distance from the track.
const KNOB: f32 = 18.0;
const INSET: f32 = 2.0;

/// A switch that is `on` or off. The caller gives it its route with `.on_click`.
pub fn switch<V: 'static>(on: bool) -> impl Element<V> + Interact + Styled {
    let (track, side) = if on {
        (spec::PRIMARY, Justify::End)
    } else {
        (spec::SWITCH_OFF, Justify::Start)
    };
    row((block(KNOB, KNOB)
        .shrink(0.0)
        .bg(rgb(0xffffff))
        .rounded(KNOB / 2.0),))
    .size(spec::SWITCH_WIDTH, spec::SWITCH_HEIGHT)
    .shrink(0.0)
    .px(INSET)
    .items(Align::Center)
    .justify(side)
    .bg(rgb(track))
    .rounded(spec::SWITCH_HEIGHT / 2.0)
}
