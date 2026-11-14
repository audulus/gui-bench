//! The top bar: the title of the page on the left and, toward the right end, the search input,
//! the live switch, the spinner, the export button and the avatar, over the rule that is its
//! bottom border.

use bough::prelude::*;
use bough::raw::no_hit;

use super::style::{avatar, label, primary_button, rule, spacer};
use super::switch::switch;
use super::{Dashboard, live, spec};

/// The space between the avatar, the export button and the right end.
const SPACING: f32 = 24.0;

/// The top bar of `dashboard`.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let controls = row((
        search(dashboard),
        block(spec::LIVE_X - spec::SEARCH_X - spec::SEARCH_WIDTH, 0.0).shrink(0.0),
        switch(dashboard.live.on).on_click(|dashboard: &mut Dashboard| dashboard.live.toggle()),
        block(spec::SPINNER_X - spec::LIVE_X - spec::SWITCH_WIDTH, 0.0).shrink(0.0),
        live::spinner(dashboard),
        block(spec::EXPORT_X - spec::SPINNER_X - spec::SPINNER_SIZE, 0.0).shrink(0.0),
        primary_button(spec::EXPORT_LABEL, spec::EXPORT_WIDTH).on_click(Dashboard::toggle_menu),
        row((avatar(
            spec::CONTROL_HEIGHT,
            spec::USER_COLOR,
            spec::USER_INITIALS,
            13.0,
        ),))
        .px(SPACING),
    ))
    .shrink(0.0)
    .items(Align::Center);
    column((
        row((
            label(
                spec::NAV_ITEMS[dashboard.nav],
                spec::TITLE_FONT + 4.0,
                spec::TEXT,
            ),
            spacer(),
            controls,
        ))
        .h(spec::TOPBAR_HEIGHT - 1.0)
        .shrink(0.0)
        .pad(spec::TOPBAR_TITLE_X - spec::SIDEBAR_WIDTH, 0.0, 0.0, 0.0)
        .items(Align::Center),
        rule(spec::RULE),
    ))
    .h(spec::TOPBAR_HEIGHT)
    .shrink(0.0)
    .bg(rgb(spec::TOPBAR_BG))
}

/// The search input: Lorelei's `text_input`, which has one look, and over it the placeholder
/// while the text is empty.
fn search(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let placeholder = dashboard.search.is_empty().then(|| {
        no_hit(label(spec::SEARCH_PLACEHOLDER, spec::FONT_SIZE, spec::MUTED).abs(9.0, 8.0))
    });
    row((
        text_input(sig!(dashboard: Dashboard => search))
            .size(spec::SEARCH_WIDTH, spec::CONTROL_HEIGHT),
        placeholder,
    ))
    .relative()
    .shrink(0.0)
}
