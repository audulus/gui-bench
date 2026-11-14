//! The orders card: a header of column titles, then one row per order that passes the filters
//! and the search, all of them real elements. A row is a line of fixed-width cells with a hover
//! color and a rule under it. A click on the amount title sorts the orders. The list scenario
//! shows the same header and rows.

use bough::elem::TextSrc;
use bough::prelude::*;

use super::Dashboard;
use super::spec::{self, Status};
use super::style::{avatar, card, label, pill, rule, shape};

/// The space between the avatar and the name.
const NAME_GAP: f32 = 12.0;
/// The space between the amount column and the right edge of the card or the window.
const AMOUNT_INSET: f32 = 24.0;

/// The orders card of `dashboard`.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let rows = dashboard
        .orders
        .iter()
        .filter(|order| spec::visible(order, &dashboard.shown, &dashboard.search))
        .map(|order| {
            order_row(
                order.number_text(),
                order.initials.as_str(),
                order.customer.as_str(),
                order.avatar,
                order.status,
                order.progress,
                order.amount(),
            )
        });
    card()
        .shrink(0.0)
        .child(header(
            spec::CARD_RADIUS,
            dashboard.sorted,
            Dashboard::toggle_sort,
        ))
        .child(each(rows))
}

/// The cells of a header or a row: the first four from their column's left edge to the next
/// one's, and the last from the end of the progress bar to `AMOUNT_INSET` before the right edge,
/// its content at its right end.
fn cells<V: 'static>(
    height: f32,
    order: impl Element<V>,
    customer: impl Element<V>,
    status: impl Element<V>,
    progress: impl Element<V>,
    amount: impl Element<V>,
) -> impl Element<V> {
    let cell = |width: f32| row(()).w(width).shrink(0.0).items(Align::Center);
    row((
        cell(spec::COL_CUSTOMER - spec::COL_ORDER).child(order),
        cell(spec::COL_STATUS - spec::COL_CUSTOMER)
            .gap(NAME_GAP)
            .child(customer),
        cell(spec::COL_PROGRESS - spec::COL_STATUS).child(status),
        cell(spec::PROGRESS_WIDTH).child(progress),
        row((amount,))
            .w(0.0)
            .grow(1.0)
            .items(Align::Center)
            .justify(Justify::End),
    ))
    .w_pct(1.0)
    .h(height)
    .shrink(0.0)
    .px(AMOUNT_INSET)
}

/// The header: the column titles on the header fill with `radius`, and the rule under it. A
/// click on the amount title runs `sort`; the title is in the primary color while the orders are
/// `sorted`.
pub fn header<V: 'static>(radius: f32, sorted: bool, sort: fn(&mut V)) -> impl Element<V> {
    let title = |column: usize| label(spec::COLUMNS[column], 12.0, spec::MUTED);
    let amount = if sorted { spec::PRIMARY } else { spec::MUTED };
    let sort = row((label(spec::COLUMNS[spec::SORT_COLUMN], 12.0, amount),))
        .size(spec::SORT_WIDTH, spec::TABLE_HEADER_HEIGHT - 1.0)
        .shrink(0.0)
        .items(Align::Center)
        .justify(Justify::End)
        .bg(rgb(spec::TABLE_HEADER_BG))
        .on_click(sort);
    column((
        cells(
            spec::TABLE_HEADER_HEIGHT - 1.0,
            title(0),
            title(1),
            title(2),
            title(3),
            sort,
        ),
        rule(spec::RULE),
    ))
    .w_pct(1.0)
    .shrink(0.0)
    .bg(rgb(spec::TABLE_HEADER_BG))
    .rounded(radius)
}

/// One order row with its hover color and the rule under it.
pub fn order_row<V: 'static, S: TextSrc<V>>(
    number: String,
    initials: S,
    customer: S,
    color: u32,
    status: Status,
    progress: u32,
    amount: String,
) -> impl Element<V> {
    let (bg, fg) = status.colors();
    let fill = spec::PROGRESS_WIDTH * progress as f32 / 100.0;
    let bar = row((shape(
        fill,
        spec::PROGRESS_HEIGHT,
        spec::PROGRESS_HEIGHT / 2.0,
        spec::PRIMARY,
    ),))
    .size(spec::PROGRESS_WIDTH, spec::PROGRESS_HEIGHT)
    .bg(rgb(spec::TRACK))
    .rounded(spec::PROGRESS_HEIGHT / 2.0);
    column((
        cells(
            spec::ORDER_ROW_HEIGHT - 1.0,
            label(number, spec::FONT_SIZE, spec::MUTED),
            (
                avatar(spec::AVATAR, color, initials, 11.0),
                label(customer, spec::FONT_SIZE, spec::TEXT),
            ),
            pill(status.label(), bg, fg, 22.0, 10.0),
            bar,
            label(amount, spec::FONT_SIZE, spec::TEXT),
        ),
        rule(spec::ROW_RULE),
    ))
    // A row of the list is laid out on its own, so it takes its width from the list.
    .w_pct(1.0)
    .h(spec::ORDER_ROW_HEIGHT)
    .shrink(0.0)
    .bg(rgb(spec::CARD_BG))
    .hover_bg(rgb(spec::ROW_HOVER))
}
