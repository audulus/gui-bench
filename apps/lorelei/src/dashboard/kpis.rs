//! The metric cards: four cards in a row that share the width, each with its label, its value,
//! its delta pill and a sparkline of bars at the right. The value is a port of the live tick, so
//! a tick changes the four texts and nothing else.

use bough::prelude::*;

use super::Dashboard;
use super::spec::{self, Kpi};
use super::style::{card, label, pill, shape, spacer};

/// The top of the label, the value and the delta pill in a card.
const LABEL_Y: f32 = 18.0;
const VALUE_Y: f32 = 40.0;
const DELTA_Y: f32 = 78.0;
const DELTA_HEIGHT: f32 = 22.0;
/// The bottom of the sparkline bars in a card.
const SPARK_BOTTOM: f32 = 92.0;

/// The row of metric cards of `dashboard`.
pub fn view(dashboard: &Dashboard) -> impl Element<Dashboard> {
    let cards = spec::KPIS
        .iter()
        .enumerate()
        .map(|(k, kpi)| metric(dashboard, k, kpi));
    row((each(cards),)).gap(spec::GAP).shrink(0.0)
}

/// Metric card `k`.
fn metric(dashboard: &Dashboard, k: usize, kpi: &'static Kpi) -> impl Element<Dashboard> {
    let (bg, fg) = if kpi.up {
        (spec::UP_BG, spec::UP_TEXT)
    } else {
        (spec::DOWN_BG, spec::DOWN_TEXT)
    };
    let value = sig!(dashboard: Dashboard => live.tick).map(move |tick| spec::kpi_value(k, *tick));
    let texts = column((
        row((label(kpi.label, 13.0, spec::MUTED),))
            .h(VALUE_Y - LABEL_Y)
            .shrink(0.0),
        row((label(value, 24.0, spec::TEXT),))
            .h(DELTA_Y - VALUE_Y)
            .shrink(0.0),
        row((pill(kpi.delta, bg, fg, DELTA_HEIGHT, 8.0),)),
    ))
    .pad(0.0, LABEL_Y, 0.0, 0.0)
    .shrink(0.0);
    card()
        .layout(Flex::row())
        .w(0.0)
        .grow(1.0)
        .h(spec::KPI_HEIGHT)
        .px(spec::CARD_PADDING)
        .child(texts)
        .child(spacer())
        .child(sparkline(&kpi.spark))
}

/// The sparkline: bars on a common bottom, the newest at the right.
fn sparkline(heights: &'static [f32]) -> impl Element<Dashboard> {
    let bars = heights
        .iter()
        .map(|height| shape(spec::SPARK_BAR_WIDTH, *height, 2.0, spec::SPARK));
    row((each(bars),))
        .h(SPARK_BOTTOM)
        .shrink(0.0)
        .gap(spec::SPARK_STEP - spec::SPARK_BAR_WIDTH)
        .items(Align::End)
}
