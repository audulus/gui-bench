//! The app scenario: the analytics app of `apps/shared/dashboard.rs`. One view struct holds the
//! state; its methods are the routes of the controls, and `view` describes the whole window from
//! it: the sidebar, the top bar, and one scroll box with the page of the active nav item. After
//! a route Lorelei describes the view again and patches what differs; the values that only show
//! text or move something (the search, the live tick, the angle of the spinner, the scroll
//! offset) are signals, which patch their elements directly. The list scenario is `list`.

mod chart;
mod filters;
mod kpis;
pub mod list;
mod live;
mod menu;
mod messages;
mod orders;
mod sidebar;
#[path = "../../../shared/dashboard.rs"]
pub mod spec;
mod style;
mod switch;
mod topbar;

use bough::prelude::*;

use self::live::Live;
use self::spec::{Message, Order, Page, Status};
use self::style::label;
use crate::window;

/// The space between the top of the content and the label of the empty page.
const EMPTY_SPACE: f32 = spec::EMPTY_TOP - spec::CONTENT_PADDING;

/// Opens the app window with `count` orders and `count` messages and runs it until it closes.
pub fn run(title: &str, count: usize) -> ! {
    window::run(title, Dashboard::new(count), live::frame)
}

/// The state of the app.
pub struct Dashboard {
    count: usize,
    /// The orders in the order the table shows them.
    orders: Vec<Order>,
    messages: Vec<Message>,
    /// The active nav item, which picks the page.
    nav: usize,
    /// The text of the search input.
    search: Signal<String>,
    /// The live updates of the metric cards.
    live: Live,
    /// The selected segment of the chart, which is the series it shows.
    segment: usize,
    /// Whether the chart shows the previous period beside the current one.
    compare: bool,
    /// Whether the orders of `Status::ALL[k]` are shown.
    shown: [bool; 4],
    /// Whether the orders are sorted by amount.
    sorted: bool,
    /// Whether the export menu is open.
    menu: bool,
    /// The scroll offset of the content.
    scroll: Signal<f32>,
}

impl Dashboard {
    fn new(count: usize) -> Self {
        Self {
            count,
            orders: spec::orders(count),
            messages: spec::messages(count),
            nav: 0,
            search: Signal::new(String::new()),
            live: Live::new(),
            segment: 0,
            compare: false,
            shown: [true; Status::ALL.len()],
            sorted: false,
            menu: false,
            scroll: Signal::new(0.0),
        }
    }

    /// Makes nav item `i` active and shows its page from the top.
    fn select_nav(&mut self, i: usize) {
        if self.nav != i {
            self.nav = i;
            self.scroll.set(0.0);
        }
    }

    /// Sorts the orders by amount, or brings the order of the data back.
    fn sort(&mut self, sorted: bool) {
        if self.sorted != sorted {
            self.sorted = sorted;
            self.orders = spec::arranged(spec::orders(self.count), sorted);
        }
    }

    fn toggle_sort(&mut self) {
        self.sort(!self.sorted);
    }

    fn toggle_menu(&mut self) {
        self.menu = !self.menu;
    }

    /// Checks every status, turns compare off, selects the first segment, clears the search and
    /// brings the order of the data back.
    fn reset(&mut self) {
        self.shown = [true; Status::ALL.len()];
        self.compare = false;
        self.segment = 0;
        self.search.set(String::new());
        self.sort(false);
    }

    /// The overview: the metric cards, the chart and filter cards, and the orders.
    fn overview(&self) -> impl Element<Self> + '_ {
        column((
            kpis::view(self),
            row((chart::view(self), filters::view(self)))
                .gap(spec::GAP)
                .shrink(0.0),
            orders::view(self),
        ))
        .gap(spec::GAP)
    }

    /// The scroll box of the content with the page of the active nav item.
    fn content(&self) -> impl Element<Self> + '_ {
        let page = spec::page(self.nav);
        let padding = spec::CONTENT_PADDING;
        div()
            .w_pct(1.0)
            .grow(1.0)
            .min_h(0.0)
            // Lorelei lays the content of a scroll box out at the width of the window, whatever
            // the width of the box. The sidebar takes its width from the window, so the content
            // leaves that much room at its right, beyond the box.
            .pad(padding, padding, padding + spec::SIDEBAR_WIDTH, padding)
            .bg(rgb(spec::PAGE_BG))
            .scroll(sig!(self.scroll))
            .child((page == Page::Overview).then(|| self.overview()))
            .child((page == Page::Messages).then(|| messages::view(self)))
            .child((page == Page::Empty).then(empty))
    }
}

/// The empty page: its label, centered across the content.
fn empty() -> impl Element<Dashboard> {
    row((label(spec::EMPTY_LABEL, spec::FONT_SIZE, spec::MUTED),))
        .margin(0.0, EMPTY_SPACE, 0.0, 0.0)
        .justify(Justify::Center)
}

impl View for Dashboard {
    fn view(&self) -> impl Element<Self> + '_ {
        row((
            sidebar::view(self),
            column((topbar::view(self), self.content()))
                .w(0.0)
                .grow(1.0)
                .h_pct(1.0),
            self.menu.then(menu::view),
        ))
        .w_pct(1.0)
        .h_pct(1.0)
        .relative()
        .bg(rgb(spec::PAGE_BG))
    }
}
