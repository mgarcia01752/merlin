#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Presentation only: plot unchanged Rust-exported pilot and PLC receiver pairs."""
import argparse
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt


def main():
    """Render two rows of actual paired observations; never overwrite output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("result", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--group", choices=["pilots", "plc"], required=True)
    args = parser.parse_args()
    with args.result.open(encoding="utf-8") as source:
        result = json.load(source)
    rows = ([('continuous_pilots', 'Continuous pilots · boosted BPSK'),
             ('scattered_pilots', 'Scattered pilots · boosted BPSK')]
            if args.group == 'pilots' else
            [('plc_data', 'PLC data · 16-QAM'), ('plc_preamble', 'PLC preamble · BPSK')])
    figure, axes = plt.subplots(2, 2, figsize=(12, 10), squeeze=False, sharex='row', sharey='row')
    figure.set_facecolor('#0b1422')
    for row_index, (stream, label) in enumerate(rows):
        pairs = result['streams'][stream]
        for column, (field, stage, color) in enumerate([
                ('before_equalization', 'Before correction', '#ffc15a'),
                ('after_equalization', 'After correction · 1/H', '#36dfce')]):
            axis = axes[row_index][column]
            points = [pair[field] for pair in pairs]
            axis.scatter([point[0] for point in points], [point[1] for point in points],
                         s=13, alpha=0.65, color=color, linewidths=0)
            axis.set_facecolor('#112033')
            axis.set_title(f'{label}\n{stage} · {len(pairs):,} observations', color='#edf5ff', fontsize=12)
            axis.set_xlabel('In-phase I · reference amplitude', color='#b1c4db')
            axis.set_ylabel('Quadrature Q · reference amplitude', color='#b1c4db')
            axis.set_aspect('equal', adjustable='box')
            axis.grid(alpha=0.2, color='#91a9c4')
            axis.set_axisbelow(True)
            axis.tick_params(colors='#b1c4db')
            for spine in axis.spines.values():
                spine.set_color('#42556f')
    figure.suptitle('MERLIN · measured channel response\nNoise disabled · supplied-H reference receiver',
                    color='#edf5ff', fontsize=17)
    figure.text(0.5, 0.025, 'Same carrier/time pairs · shared axes per row · actual boost preserved',
                ha='center', color='#b1c4db', fontsize=10)
    figure.tight_layout(rect=(0, 0.05, 1, 0.92))
    with args.output.open('xb') as destination:
        figure.savefig(destination, format='png', dpi=150, facecolor=figure.get_facecolor())
    plt.close(figure)


if __name__ == '__main__':
    main()
