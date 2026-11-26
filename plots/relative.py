"""Every interaction against a reference framework: the median response time of each framework
in a table with one row per interaction and one column per framework, at one size of the app and
at every size of the list. The color of a cell shows the framework against the reference on a log
scale, blue for faster and red for slower; the number is the absolute cost.

Usage: python3 plots/relative.py [run dir] [--reference gpui-kit] [--rows 3000]
"""

import argparse
import math

from matplotlib.patches import FancyBboxPatch

import common as c

# The reference framework when it is in the run; otherwise the first framework.
REFERENCE = "gpui-kit"
# The ratio against the reference that takes the strongest color, either way.
SPAN = 30.0
# The ends and the middle of the ramp: faster, as fast, slower.
FASTER, EVEN, SLOWER = "#2f63b0", "#f1efeb", "#c0392f"
# The responses of the list scenario.
LIST_RESPONSES = [("hover-row", "Hover a row"), ("click-sort", "Sort the data")]


def median(cell, name):
    """Returns (median ms, valid) of a probe of a cell, or None."""
    report = cell.report(name) if cell else None
    values = c.response_samples(report)
    if not values:
        return None
    return c.percentile(values, 0.5), c.valid(report)


def find(run, scenario, framework, rows):
    """Returns the cell of a framework at a size at the main rate, or None."""
    hz = run.index["hz"]["main"]
    return next(iter(run.of(scenario, framework=framework, hz=hz, rows=rows)), None)


def tone(ratio):
    """Returns (fill, ink) of a cell whose cost is `ratio` times the reference's."""
    t = max(-1.0, min(1.0, math.log(ratio) / math.log(SPAN)))
    fill = c._mix(EVEN, SLOWER if t > 0 else FASTER, abs(t) ** 0.8)
    return fill, ("#ffffff" if abs(t) > 0.5 else c.TEXT)


def number(ms):
    """Formats milliseconds with about three significant digits."""
    if ms >= 100:
        return f"{ms:.0f}"
    if ms >= 10:
        return f"{ms:.1f}"
    return f"{ms:.2f}" if ms < 1 else f"{ms:.1f}"


def section(run, scenario, rows, probes, frameworks, reference):
    """Returns the table rows of one scenario at one size: (title, [(text, fill, ink)])."""
    out = []
    for name, title in probes:
        base = median(find(run, scenario, reference, rows), name)
        cells = []
        for framework in frameworks:
            cell = find(run, scenario, framework, rows)
            if cell is not None and cell.stopped:
                cells.append(("over", c.BAND, c.MUTED))
                continue
            value = median(cell, name)
            if value is None:
                cells.append(("–", c.BAND, c.MUTED))
                continue
            ms, ok = value
            part = c.PARTS.get(name) if scenario == "app" else "list"
            hand = run.flagged(framework, part) is not None
            text = number(ms) + ("†" if hand else "") + \
                ("" if ok else "*")
            fill, ink = tone(ms / base[0]) if base and base[0] > 0 else (c.BAND, c.TEXT)
            cells.append((text, fill, ink))
        out.append((title, cells))
    return out


def draw(ax, frameworks, reference, sections):
    """Draws the table: a heading per section, a row per interaction, a column per framework."""
    lines = sum(1 + len(rows) for _, rows in sections)
    ax.set_xlim(0, len(frameworks))
    ax.set_ylim(lines, -0.9)
    ax.axis("off")
    ax._card = True
    for col, framework in enumerate(frameworks):
        ax.text(col + 0.5, -0.35, framework, ha="center", va="center", fontsize=9.5,
                color=c.ink(framework), fontweight="bold" if framework == reference else "normal")
    y = 0
    for heading, rows in sections:
        ax.text(-0.12, y + 0.55, heading, ha="right", va="center", fontsize=10.2,
                fontweight="bold", color=c.TEXT)
        y += 1
        for title, cells in rows:
            ax.text(-0.12, y + 0.5, title, ha="right", va="center", fontsize=9.5,
                    color=c.TEXT_2)
            for col, (text, fill, ink) in enumerate(cells):
                ax.add_patch(FancyBboxPatch((col + 0.04, y + 0.06), 0.92, 0.88,
                                            boxstyle="round,pad=0,rounding_size=0.06",
                                            facecolor=fill, edgecolor="none"))
                bold = frameworks[col] == reference
                ax.text(col + 0.5, y + 0.5, text, ha="center", va="center", fontsize=9.2,
                        color=ink, fontweight="bold" if bold else "normal")
            y += 1


def ramp(fig):
    """Draws the color key in the legend row under the title block: faster on the left, slower
    on the right. It is drawn in figure coordinates, so the layout of the cards leaves it be."""
    from matplotlib.patches import Rectangle
    width, height = fig.get_figwidth(), fig.get_figheight()
    fig._legend_row = True
    y = 1 - fig._header - 0.2 / height
    x0, length, thick, steps = 1.55 / width, 3.6 / width, 0.13 / height, 60
    for i in range(steps):
        t = -1 + 2 * (i + 0.5) / steps
        fig.patches.append(Rectangle((x0 + length * i / steps, y - thick / 2), length / steps,
                                     thick, transform=fig.transFigure, linewidth=0,
                                     facecolor=tone(SPAN ** t)[0]))
    fig.text(x0 - 0.08 / width, y, f"{SPAN:g}× faster", ha="right", va="center", fontsize=9,
             color=c.TEXT_2)
    fig.text(x0 + length + 0.08 / width, y, f"{SPAN:g}× slower", ha="left", va="center",
             fontsize=9, color=c.TEXT_2)


def plot(run, reference=None, rows=None):
    frameworks = run.frameworks("app") or run.frameworks("list")
    if not frameworks:
        return None
    reference = reference or (REFERENCE if REFERENCE in frameworks else frameworks[0])
    app_rows = sorted({cell.rows for cell in run.of("app", framework=reference)})
    rows = rows or (app_rows[-1] if app_rows else None)
    sections = []
    if rows is not None:
        cell = find(run, "app", reference, rows)
        elements = f", {c.compact(cell.elements)} elements" if cell and cell.elements else ""
        sections.append((f"App, {c.compact(rows)} rows{elements}",
                         section(run, "app", rows, c.RESPONSES, frameworks, reference)))
    for size in sorted({cell.rows for cell in run.of("list")}):
        sections.append((f"List, {c.compact(size)} rows, virtualized",
                         section(run, "list", size, LIST_RESPONSES, frameworks, reference)))
    if not sections:
        return None
    lines = sum(1 + len(r) for _, r in sections)
    width = 3.4 + 1.05 * len(frameworks)
    table = 0.42 * (lines + 0.9)
    height = 1.08 + 0.5 + table + 1.2
    fig = c.figure(width, height, f"Each interaction against {reference}",
                   c.subtitle(run, f"Median ms from the input to the finished frame. Color shows "
                                   f"each framework against {reference}, not the absolute "
                                   f"cost"),
                   note=f"† A part of the app built by hand.  * The host disturbed the "
                        f"measurement.  over: not run, over 250 ms at a smaller size.  "
                        f"–: no data.")
    ax = fig.subplots()
    fig.subplots_adjust(left=2.9 / width, right=1 - 0.3 / width, top=1 - 1.75 / height,
                        bottom=1.15 / height)
    draw(ax, frameworks, reference, sections)
    ramp(fig)
    return c.save(fig, run, "relative")


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("run", nargs="?", default="results/latest")
    parser.add_argument("--reference", help=f"framework to compare to (default {REFERENCE})")
    parser.add_argument("--rows", type=int, help="size of the app (default: the largest)")
    args = parser.parse_args()
    print(plot(c.load(args.run), args.reference, args.rows))


if __name__ == "__main__":
    main()
