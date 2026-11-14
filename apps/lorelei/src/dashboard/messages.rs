//! The messages page: one card per message, whose body wraps to the width of the card.

use bough::prelude::*;

use super::Dashboard;
use super::spec::{self, Message};
use super::style::{avatar, card, label};

/// The space between the avatar and the text.
const TEXT_GAP: f32 = spec::MESSAGE_TEXT_X - spec::MESSAGE_PADDING.1 - spec::MESSAGE_AVATAR;
/// The space between the author and the time.
const TIME_GAP: f32 = 8.0;

/// The feed of the messages of `dashboard`.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    column((each(dashboard.messages.iter().map(message_card)),)).gap(spec::MESSAGE_GAP)
}

/// One message: the avatar, the author and the time on one line, and the body below.
fn message_card(message: &Message) -> impl Element<Dashboard> {
    let (down, across) = spec::MESSAGE_PADDING;
    let head = row((
        label(message.author.as_str(), spec::FONT_SIZE, spec::TEXT),
        label(message.time.as_str(), 13.0, spec::MUTED),
    ))
    .h(spec::MESSAGE_LINE)
    .gap(TIME_GAP)
    .items(Align::Center);
    let body = para(message.body.as_str())
        .font(spec::FONT_SIZE, spec::MESSAGE_LINE)
        .color(rgb(spec::TEXT));
    card()
        .layout(Flex::row().gap(TEXT_GAP))
        .shrink(0.0)
        .pad(across, down, across, down)
        .child(avatar(
            spec::MESSAGE_AVATAR,
            message.avatar,
            message.initials.as_str(),
            13.0,
        ))
        .child(column((head, body)).w(0.0).grow(1.0))
}
