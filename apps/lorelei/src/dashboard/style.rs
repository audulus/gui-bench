//! The shared look: text, cards, avatars, pills and buttons as plain flow elements. Lorelei has
//! one font face per window and no shadows in its application surface, so every text is regular
//! and every card is flat.

use bough::elem::TextSrc;
use bough::layout::{LBox, Para};
use bough::prelude::*;

use super::spec;

/// Returns the line height of text of a font size: 20 px for the 14 px of the body.
fn line(size: f32) -> f32 {
    (size * 1.4).ceil()
}

/// One line of text in a font size and a spec color.
pub fn label<S>(content: S, size: f32, color: u32) -> Para<S> {
    text(content).font(size, line(size)).color(rgb(color))
}

/// A filled box of a fixed size with rounded corners.
pub fn shape(width: f32, height: f32, radius: f32, color: u32) -> LBox {
    block(width, height)
        .shrink(0.0)
        .bg(rgb(color))
        .rounded(radius)
}

/// A circle of `size` with initials in white.
pub fn avatar<V: 'static, S: TextSrc<V>>(
    size: f32,
    color: u32,
    initials: S,
    font: f32,
) -> impl Element<V> {
    row((label(initials, font, 0xffffff),))
        .size(size, size)
        .shrink(0.0)
        .center()
        .bg(rgb(color))
        .rounded(size / 2.0)
}

/// A pill of `height` around a 12 px label, with `padding` left and right.
pub fn pill<V: 'static, S: TextSrc<V>>(
    content: S,
    bg: u32,
    fg: u32,
    height: f32,
    padding: f32,
) -> impl Element<V> {
    row((label(content, 12.0, fg),))
        .h(height)
        .px(padding)
        .shrink(0.0)
        .center()
        .bg(rgb(bg))
        .rounded(height / 2.0)
}

/// The box of every card: white with rounded corners. Children go in with `.child`.
pub fn card() -> LBox {
    div().bg(rgb(spec::CARD_BG)).rounded(spec::CARD_RADIUS)
}

/// A primary button of `width`: a row styled the way Lorelei's `button` is built, in the colors
/// of the spec. Routes go on with `.on_click`.
pub fn primary_button(content: &'static str, width: f32) -> LBox<Color, (Para<&'static str>,)> {
    row((label(content, spec::FONT_SIZE, spec::PRIMARY_TEXT),))
        .size(width, spec::CONTROL_HEIGHT)
        .shrink(0.0)
        .center()
        .bg(rgb(spec::PRIMARY))
        .rounded(8.0)
}

/// A secondary button of `width`: white with a border.
pub fn secondary_button(content: &'static str, width: f32) -> LBox<Color, (Para<&'static str>,)> {
    row((label(content, spec::FONT_SIZE, spec::TEXT),))
        .size(width, spec::CONTROL_HEIGHT)
        .shrink(0.0)
        .center()
        .bg(rgb(spec::SECONDARY_BG))
        .rounded(8.0)
        .border(1.0, rgb(spec::SECONDARY_BORDER))
}

/// A 1 px horizontal rule over the full width.
pub fn rule(color: u32) -> LBox {
    block(0.0, 1.0).w_pct(1.0).shrink(0.0).bg(rgb(color))
}

/// Empty room that takes the free space of a row or a column.
pub fn spacer() -> LBox {
    div().grow(1.0)
}
