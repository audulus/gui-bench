//! The sidebar: the brand, the nav items and the user at the bottom. A nav item is a row with a
//! hover color; a click on it makes it active.

use bough::prelude::*;

use super::style::{avatar, label, rule, shape, spacer};
use super::{Dashboard, spec};

/// The space between the left edge of the sidebar and the brand or the user.
const INSET: f32 = 20.0;
/// The top of the logo: centered on the middle of the top bar.
const BRAND_TOP: f32 = (spec::TOPBAR_HEIGHT - spec::LOGO_SIZE) / 2.0;
/// The top of the user avatar under the rule.
const USER_TOP: f32 = 740.0 - spec::USER_RULE_Y - 1.0;
/// The height of the user block, its rule included.
const USER_HEIGHT: f32 = spec::WINDOW.1 - spec::USER_RULE_Y;

/// The sidebar of `dashboard`.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let items = spec::NAV_ITEMS.iter().enumerate().map(|(i, name)| {
        let active = dashboard.nav == i;
        // The hover color belongs to the element that was mounted, so the active and the plain
        // item are two elements, of which one exists.
        (
            active.then(|| nav_item(i, name, true)),
            (!active).then(|| nav_item(i, name, false)),
        )
    });
    column((
        brand(),
        column((each(items),))
            .px(spec::NAV_X)
            .gap(spec::NAV_STRIDE - spec::NAV_HEIGHT)
            .shrink(0.0),
        spacer(),
        user(),
    ))
    .w(spec::SIDEBAR_WIDTH)
    .h_pct(1.0)
    .shrink(0.0)
    .bg(rgb(spec::SIDEBAR_BG))
}

/// The logo square and the app name beside it.
fn brand() -> impl Element<Dashboard> {
    row((
        shape(
            spec::LOGO_SIZE,
            spec::LOGO_SIZE,
            spec::LOGO_RADIUS,
            spec::LOGO,
        ),
        label(spec::APP_NAME, spec::TITLE_FONT, spec::PRIMARY_TEXT),
    ))
    .h(spec::NAV_TOP)
    .shrink(0.0)
    .pad(
        INSET,
        BRAND_TOP,
        0.0,
        spec::NAV_TOP - BRAND_TOP - spec::LOGO_SIZE,
    )
    .gap(12.0)
    .items(Align::Center)
}

/// Nav item `i`: its icon, its label and, on the messages item, the unread badge.
fn nav_item(i: usize, name: &'static str, active: bool) -> impl Element<Dashboard> {
    let (fill, hover, icon, ink) = if active {
        (
            spec::NAV_ACTIVE,
            spec::NAV_ACTIVE,
            spec::NAV_ICON_ACTIVE,
            spec::PRIMARY_TEXT,
        )
    } else {
        (
            spec::SIDEBAR_BG,
            spec::NAV_HOVER,
            spec::NAV_ICON_COLOR,
            spec::SIDEBAR_TEXT,
        )
    };
    let badge = (i == spec::BADGE_ITEM).then(|| {
        row((label(
            spec::BADGE_COUNT.to_string(),
            12.0,
            spec::PRIMARY_TEXT,
        ),))
        .h(20.0)
        .px(7.0)
        .center()
        .bg(rgb(spec::BADGE_BG))
        .rounded(10.0)
    });
    row((
        shape(spec::NAV_ICON, spec::NAV_ICON, 4.0, icon),
        label(name, spec::FONT_SIZE, ink),
        spacer(),
        badge,
    ))
    .h(spec::NAV_HEIGHT)
    .shrink(0.0)
    .pad(16.0, 0.0, 12.0, 0.0)
    .gap(12.0)
    .items(Align::Center)
    .bg(rgb(fill))
    .rounded(spec::NAV_RADIUS)
    .hover_bg(rgb(hover))
    .on_click(move |dashboard: &mut Dashboard| dashboard.select_nav(i))
}

/// The user at the bottom: the rule, the avatar, the name and the role.
fn user() -> impl Element<Dashboard> {
    column((
        rule(spec::SIDEBAR_RULE),
        row((
            avatar(
                spec::USER_AVATAR,
                spec::USER_COLOR,
                spec::USER_INITIALS,
                12.0,
            ),
            column((
                label(spec::USER_NAME, spec::FONT_SIZE, spec::PRIMARY_TEXT),
                label(spec::USER_ROLE, 12.0, spec::SIDEBAR_MUTED),
            )),
        ))
        .pad(INSET, USER_TOP, 0.0, 0.0)
        .gap(12.0)
        .items(Align::Start),
    ))
    .h(USER_HEIGHT)
    .shrink(0.0)
}
