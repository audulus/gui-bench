//! The export menu. Lorelei has no menu or popover, so the app places a box of items at the
//! spot under the export button, as the last child of the window: it paints over the content.

use bough::prelude::*;

use super::style::label;
use super::{Dashboard, spec};

/// The space between the left edge of an item and its label.
const ITEM_PADDING: f32 = 12.0;

/// The open menu: one row per item, which takes the hover color; a click on it closes the menu.
pub fn view() -> impl Element<Dashboard> {
    let items = spec::MENU_ITEMS.iter().map(|name| {
        row((label(*name, spec::FONT_SIZE, spec::TEXT),))
            .h(spec::MENU_ITEM_HEIGHT)
            .shrink(0.0)
            .pad(ITEM_PADDING, 0.0, 0.0, 0.0)
            .items(Align::Center)
            .bg(rgb(spec::CARD_BG))
            .rounded(6.0)
            .hover_bg(rgb(spec::MENU_HOVER))
            .on_click(Dashboard::toggle_menu)
    });
    column((each(items),))
        .abs(spec::MENU_X, spec::MENU_Y)
        .w(spec::MENU_WIDTH)
        .p(spec::MENU_PADDING)
        .bg(rgb(spec::CARD_BG))
        .rounded(8.0)
        .border(1.0, rgb(spec::RULE))
}
