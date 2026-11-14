//! The filter card: the status checkboxes, the compare switch, and the Reset and Apply buttons,
//! top to bottom. Lorelei has no checkbox, so a checkbox is a row with a box that is filled
//! while it is checked.

use bough::prelude::*;

use super::Dashboard;
use super::spec::{self, Status};
use super::style::{card, label, primary_button, secondary_button, shape};
use super::switch::switch;

/// The top of the filter card in the window.
const CARD_TOP: f32 = spec::TABLE_TOP - spec::GAP - spec::ROW2_HEIGHT;
/// The height of the compare row.
const COMPARE_HEIGHT: f32 = 28.0;
/// The size of the mark in a checked box.
const MARK: f32 = 8.0;

/// The filter card of `dashboard`.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let checks = Status::ALL
        .iter()
        .enumerate()
        .map(|(k, status)| checkbox(k, status.label(), dashboard.shown[k]));
    let checks_end = spec::CHECK_TOP + Status::ALL.len() as f32 * spec::CHECK_STRIDE;
    let compare = row((
        label(spec::COMPARE_LABEL, spec::FONT_SIZE, spec::TEXT),
        switch(dashboard.compare)
            .on_click(|dashboard: &mut Dashboard| dashboard.compare = !dashboard.compare),
    ))
    .h(COMPARE_HEIGHT)
    .shrink(0.0)
    .margin(0.0, spec::COMPARE_Y - checks_end, 0.0, 0.0)
    .items(Align::Center)
    .justify(Justify::SpaceBetween);
    let buttons = row((
        secondary_button(spec::RESET_LABEL, spec::BUTTON_WIDTH).on_click(Dashboard::reset),
        // Apply does nothing.
        primary_button(spec::APPLY_LABEL, spec::BUTTON_WIDTH),
    ))
    .shrink(0.0)
    .margin(
        0.0,
        spec::BUTTONS_Y - spec::COMPARE_Y - COMPARE_HEIGHT,
        0.0,
        0.0,
    )
    .gap(spec::APPLY_X - spec::RESET_X - spec::BUTTON_WIDTH)
    .justify(Justify::End);
    card()
        .size(spec::FILTERS_WIDTH, spec::ROW2_HEIGHT)
        .shrink(0.0)
        .p(spec::CARD_PADDING)
        .child(
            row((label(spec::FILTERS_TITLE, spec::TITLE_FONT, spec::TEXT),))
                .h(spec::CHECK_TOP - CARD_TOP - spec::CARD_PADDING)
                .shrink(0.0),
        )
        .child(each(checks))
        .child(compare)
        .child(buttons)
}

/// The checkbox of status `k`, a row of `CHECK_STRIDE`; a click on it toggles the status.
fn checkbox(k: usize, name: &'static str, checked: bool) -> impl Element<Dashboard> {
    let (fill, border, mark) = if checked {
        (spec::PRIMARY, spec::PRIMARY, spec::PRIMARY_TEXT)
    } else {
        (spec::CARD_BG, spec::INPUT_BORDER, spec::CARD_BG)
    };
    let check = row((shape(MARK, MARK, 2.0, mark),))
        .size(spec::CHECK_SIZE, spec::CHECK_SIZE)
        .shrink(0.0)
        .center()
        .bg(rgb(fill))
        .rounded(4.0)
        .border(1.0, rgb(border));
    row((check, label(name, spec::FONT_SIZE, spec::TEXT)))
        .h(spec::CHECK_STRIDE)
        .shrink(0.0)
        .gap(10.0)
        .items(Align::Center)
        .bg(rgb(spec::CARD_BG))
        .on_click(move |dashboard: &mut Dashboard| dashboard.shown[k] = !dashboard.shown[k])
}
