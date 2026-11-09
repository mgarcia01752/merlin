/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Engineering interpolation: remove observed adjacent-bin phase slope, then interpolate I/Q.
use crate::{error::invalid, types::IqPair, SimulationError};
pub(super) const MIN_RESPONSE_POWER: f64 = 1e-12;
fn rotate(point: IqPair, phase: f64) -> IqPair {
    let (sine, cosine) = phase.sin_cos();
    [
        point[0] * cosine - point[1] * sine,
        point[0] * sine + point[1] * cosine,
    ]
}
pub(super) fn interpolate(
    nodes: &[(usize, IqPair)],
    size: usize,
) -> Result<(Vec<IqPair>, f64), SimulationError> {
    if nodes.len() < 2 {
        return Err(invalid(
            "CH-EST-RX requires at least two usable pilot observations.",
        ));
    }
    // Adjacent observations resolve slope modulo 2pi per bin without comb-spacing alias.
    let mut direction = [0.0, 0.0];
    for pair in nodes.windows(2).filter(|pair| pair[1].0 == pair[0].0 + 1) {
        let left = pair[0].1;
        let right = pair[1].1;
        let cross = [
            right[0] * left[0] + right[1] * left[1],
            right[1] * left[0] - right[0] * left[1],
        ];
        let power = cross[0].hypot(cross[1]);
        if power > MIN_RESPONSE_POWER {
            direction[0] += cross[0] / power;
            direction[1] += cross[1] / power;
        }
    }
    let slope = direction[1].atan2(direction[0]);
    let mut cursor = 0;
    let mut output = Vec::with_capacity(size);
    for bin in 0..size {
        while cursor + 1 < nodes.len() && nodes[cursor + 1].0 <= bin {
            cursor += 1;
        }
        let (left_bin, left) = nodes[cursor];
        let residual = if bin <= nodes[0].0 || cursor + 1 == nodes.len() {
            rotate(left, -slope * left_bin as f64)
        } else {
            let (right_bin, right) = nodes[cursor + 1];
            let left = rotate(left, -slope * left_bin as f64);
            let right = rotate(right, -slope * right_bin as f64);
            let fraction = (bin - left_bin) as f64 / (right_bin - left_bin) as f64;
            [
                left[0] + fraction * (right[0] - left[0]),
                left[1] + fraction * (right[1] - left[1]),
            ]
        };
        let response = rotate(residual, slope * bin as f64);
        let response_power = response[0] * response[0] + response[1] * response[1];
        if response.iter().any(|value| !value.is_finite())
            || !response_power.is_finite()
            || response_power < MIN_RESPONSE_POWER
        {
            return Err(invalid(
                "CH-EST-RX response is nonfinite or below the inverse-power floor.",
            ));
        }
        output.push(response);
    }
    Ok((output, slope))
}
