#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Presentation only: draw the Rust-exported raw/corrected points without DSP."""
import argparse
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("result", type=Path, help="Channel JSON containing constellation pairs")
    parser.add_argument("output", type=Path, help="New PNG destination; never replaced")
    parser.add_argument("--view", choices=["paired", "before", "after"], default="paired",
                        help="Show both stages or one standalone constellation")
    args = parser.parse_args()
    with args.result.open(encoding="utf-8") as source:
        result = json.load(source)
        capture = result["constellation"]
    pairs = capture["pairs"]
    if not pairs:
        parser.error("No paired observations to draw")
    panels = [
        ("before_equalization", "Before equalization · raw FFT", "#ffc15a"),
        ("after_equalization", "After equalization · 1/H", "#36dfce"),
    ]
    if args.view == "before":
        panels = panels[:1]
    elif args.view == "after":
        panels = panels[1:]
    figure, axes_grid = plt.subplots(1, len(panels), squeeze=False,
                                    figsize=(12, 6) if len(panels) == 2 else (7, 7),
                                    sharex=True, sharey=True)
    axes = axes_grid[0]
    figure.set_facecolor("#0b1422")
    for axis, (field, title, color) in zip(axes, panels):
        points = [pair[field] for pair in pairs]
        axis.scatter([point[0] for point in points], [point[1] for point in points],
                     s=2, alpha=0.65, color=color, linewidths=0)
        axis.set_facecolor("#112033")
        axis.set_title(title, color="#edf5ff", fontsize=14, pad=15)
        axis.set_xlabel("In-phase I · normalized amplitude", color="#b1c4db")
        axis.set_aspect("equal", adjustable="box")
        axis.grid(alpha=0.2, color="#91a9c4")
        axis.set_axisbelow(True)
        axis.tick_params(colors="#b1c4db")
        for spine in axis.spines.values():
            spine.set_color("#42556f")
    axes[0].set_ylabel("Quadrature Q · normalized amplitude", color="#b1c4db")
    title = (f"MERLIN · {capture['collected_pairs']:,} paired payload QAM observations"
             if args.view == "paired" else f"MERLIN · {capture['collected_pairs']:,} payload QAM observations")
    noise_disabled = result["settings"]["impairments"]["noise"]["mode"] == "noiseless"
    if noise_disabled:
        title += "\nCE only · noise disabled" if result["settings"]["impairments"]["response"] is not None else "\nNoise disabled"
    figure.suptitle(title, color="#edf5ff", fontsize=18 if args.view == "paired" else 15)
    caption = ("Same carrier/time pairs · shared axes · no independent normalization"
               if args.view == "paired" else "Profile 0 · 256-QAM · raw exported I/Q · no display normalization")
    figure.text(0.5, 0.035, caption, ha="center", color="#b1c4db", fontsize=10)
    figure.tight_layout(rect=(0, 0.065, 1, 0.88 if noise_disabled else 0.93))
    with args.output.open("xb") as output:
        figure.savefig(output, format="png", dpi=150, facecolor=figure.get_facecolor())
    plt.close(figure)


if __name__ == "__main__":
    main()
