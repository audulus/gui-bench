//! The list scenario: the header of the orders over the full width, and under it the order rows
//! in Lorelei's list region. A `List` is the state of the rows and the region mounts the rows
//! its viewport shows; sorting the `List` moves them.

use std::cmp::Reverse;

use bough::prelude::*;

use super::{orders, spec};
use crate::window;

/// The scroll distance of one wheel line where the host reports lines.
const WHEEL_LINE: f32 = 48.0;

/// Opens the list window with `count` orders and runs it until it closes.
pub fn run(title: &str, count: usize) -> ! {
    window::run(title, OrderList::new(count), |_| {})
}

/// One row of the list: the index of its order.
struct OrderRow {
    index: u32,
}

impl View for OrderRow {
    fn view(&self) -> impl Element<Self> + '_ {
        let order = spec::order(self.index as usize);
        let (number, amount) = (order.number_text(), order.amount());
        orders::order_row(
            number,
            order.initials,
            order.customer,
            order.avatar,
            order.status,
            order.progress,
            amount,
        )
    }
}

/// The orders of the list.
struct OrderList {
    rows: List<OrderRow>,
    /// Whether the orders are sorted by amount.
    sorted: bool,
    /// The scroll offset of the list.
    scroll: Signal<f32>,
}

impl OrderList {
    fn new(count: usize) -> Self {
        Self {
            rows: List::from_iter_in((0..count as u32).map(|index| OrderRow { index })),
            sorted: false,
            scroll: Signal::new(0.0),
        }
    }

    /// Sorts the rows by amount, largest first, or brings the order of the data back.
    fn toggle_sort(&mut self) {
        self.sorted = !self.sorted;
        if self.sorted {
            self.rows
                .sort_by_key(|row| Reverse(spec::cents(row.index as usize)));
        } else {
            self.rows.sort_by_key(|row| row.index);
        }
    }

    /// Returns how far the list scrolls: the height of the rows that the window does not show.
    fn scroll_end(&self) -> f32 {
        let rows = self.rows.len() as f32 * spec::ORDER_ROW_HEIGHT;
        (rows - (spec::WINDOW.1 - spec::LIST_ROWS_TOP)).max(0.0)
    }
}

impl View for OrderList {
    fn view(&self) -> impl Element<Self> + '_ {
        column((
            orders::header(0.0, self.sorted, OrderList::toggle_sort),
            list(sig!(self.rows))
                .w_pct(1.0)
                .grow(1.0)
                .min_h(0.0)
                .bg(rgb(spec::CARD_BG))
                .estimate(spec::ORDER_ROW_HEIGHT)
                .scroll(sig!(self.scroll))
                .scrolls(sig!(self.scroll), WHEEL_LINE, self.scroll_end()),
        ))
        .w_pct(1.0)
        .h_pct(1.0)
        .bg(rgb(spec::CARD_BG))
    }
}
