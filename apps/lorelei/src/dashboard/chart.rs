//! The chart card: the title, the segmented control of the series, and the plot: bars on the
//! baseline over four grid lines.

use bough::prelude::*;

use super::style::{card, label, shape};
use super::{Dashboard, spec};

/// The space between the top of the card and the segmented control.
const HEADER_TOP: f32 = 16.0;
const TRACK_PADDING: f32 = 2.0;
/// The height of the header row: the segmented control with its track padding.
const HEADER_HEIGHT: f32 = spec::SEGMENT_HEIGHT + 2.0 * TRACK_PADDING;
/// The top of the chart card in the window.
const CARD_TOP: f32 = spec::TABLE_TOP - spec::GAP - spec::ROW2_HEIGHT;
/// The height of the plot.
const PLOT_HEIGHT: f32 = spec::PLOT_BOTTOM - spec::PLOT_TOP;

/// The chart card of `dashboard`, which takes the width the filter card leaves.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let header = row((
        row((label(spec::CHART_TITLE, spec::TITLE_FONT, spec::TEXT),)).pad(0.0, 4.0, 0.0, 0.0),
        segments(dashboard.segment),
    ))
    .h(HEADER_HEIGHT)
    .shrink(0.0)
    .justify(Justify::SpaceBetween);
    card()
        .w(0.0)
        .grow(1.0)
        .h(spec::ROW2_HEIGHT)
        .pad(
            spec::CARD_PADDING,
            HEADER_TOP,
            spec::CARD_PADDING,
            spec::CARD_PADDING,
        )
        .gap(spec::PLOT_TOP - CARD_TOP - HEADER_TOP - HEADER_HEIGHT)
        .child(header)
        .child(plot(dashboard.segment, dashboard.compare))
}

/// The segmented control: a track around one box per series, the selected one white.
fn segments(selected: usize) -> impl Element<Dashboard> {
    let items = spec::SEGMENTS.iter().enumerate().map(move |(i, name)| {
        let (fill, ink) = if i == selected {
            (spec::CARD_BG, spec::TEXT)
        } else {
            (spec::SEGMENT_TRACK, spec::MUTED)
        };
        row((label(*name, 13.0, ink),))
            .size(spec::SEGMENT_WIDTH, spec::SEGMENT_HEIGHT)
            .shrink(0.0)
            .center()
            .bg(rgb(fill))
            .rounded(6.0)
            .on_click(move |dashboard: &mut Dashboard| dashboard.segment = i)
    });
    row((each(items),))
        .p(TRACK_PADDING)
        .shrink(0.0)
        .bg(rgb(spec::SEGMENT_TRACK))
        .rounded(8.0)
}

/// The plot: the grid lines, then the bars of `series`, each in its slot; with `compare`, a
/// narrow bar of the previous period before each narrow current one.
fn plot(series: usize, compare: bool) -> impl Element<Dashboard> {
    let lines = spec::GRID_LINES.iter().map(|y| {
        block(0.0, 1.0)
            .w_pct(1.0)
            .abs(0.0, y - spec::PLOT_TOP)
            .bg(rgb(spec::GRID))
    });
    let slots = (0..spec::CHART_BARS).map(move |i| {
        let current = spec::bar_height(series, i);
        let single =
            (!compare).then(|| shape(17.0, current, 4.0, spec::BAR).margin(4.0, 0.0, 4.0, 0.0));
        let pair = compare.then(|| {
            (
                shape(
                    8.0,
                    spec::previous_height(series, i),
                    4.0,
                    spec::BAR_PREVIOUS,
                )
                .margin(3.0, 0.0, 2.0, 0.0),
                shape(8.0, current, 4.0, spec::BAR).margin(0.0, 0.0, 4.0, 0.0),
            )
        });
        row((single, pair))
            .size(spec::CHART_SLOT, PLOT_HEIGHT)
            .shrink(0.0)
            .items(Align::End)
    });
    let bars = row((each(slots),)).abs(0.0, 0.0).h(PLOT_HEIGHT);
    row((each(lines), bars))
        .h(PLOT_HEIGHT)
        .shrink(0.0)
        .relative()
}
