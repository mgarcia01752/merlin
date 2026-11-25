/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Centered arithmetic smoothing with reflected endpoints and conservative gaps.

/// Number of source points in a smoothing window, independent of display pixels.
pub type WindowPointCount = u32;

/// Computes a same-length centered arithmetic moving average.
/// # Arguments
/// * `values` - Original uniformly spaced values; missing/non-finite values are gaps.
/// * `window_points` - Positive window length, including lengths above input size.
/// # Returns
/// None for zero window size or unsupported index arithmetic. Otherwise each
/// output is the mean of its reflected window, or None if that window has a gap
/// or non-finite sum. Even windows include N/2 points before the center and
/// N/2-1 after it, matching PyPNM's reflect-padding convention. Length-one input
/// repeats its sole value. Large windows use full reflection cycles, so work and
/// allocation remain bounded by input length; range queries take logarithmic work
/// per point, independent of window size. Pairwise sums preserve small powers
/// after large steps without subtraction cancellation.
pub fn reflected_window_average(
    values: &[Option<f64>],
    window_points: WindowPointCount,
) -> Option<Vec<Option<f64>>> {
    if window_points == 0 {
        return None;
    }
    if values.len() <= 1 {
        return Some(
            values
                .iter()
                .map(|value| value.filter(|value| value.is_finite()))
                .collect(),
        );
    }
    let length = i64::try_from(values.len()).ok()?;
    let period = length.checked_sub(1)?.checked_mul(2)?;
    let count = i64::from(window_points);
    let start = -(count / 2);
    let sums = RangeSums::new(values)?;
    let complete_cycle = sums
        .range(0, values.len())
        .add(sums.range(1, values.len() - 1));
    let cycles = count / period;
    let mut output = Vec::with_capacity(values.len());
    for index in 0..length {
        let mut window = if cycles > 0 {
            Sum {
                total: complete_cycle.total * cycles as f64,
                gaps: complete_cycle.gaps * cycles as u64,
            }
        } else {
            Sum::default()
        };
        let mut phase = (start + index).rem_euclid(period);
        let mut remaining = count % period;
        while remaining > 0 {
            let available = if phase < length {
                length - phase
            } else {
                period - phase
            };
            let take = remaining.min(available);
            let part = if phase < length {
                sums.range(phase as usize, (phase + take) as usize)
            } else {
                let last = period - phase;
                sums.range((last - take + 1) as usize, (last + 1) as usize)
            };
            window = window.add(part);
            phase = (phase + take) % period;
            remaining -= take;
        }
        let average = window.total / f64::from(window_points);
        output.push((window.gaps == 0 && average.is_finite()).then_some(average));
    }
    Some(output)
}

#[derive(Clone, Copy, Default)]
struct Sum {
    total: f64,
    gaps: u64,
}
impl Sum {
    fn add(self, other: Self) -> Self {
        Self {
            total: self.total + other.total,
            gaps: self.gaps + other.gaps,
        }
    }
}

// Pairwise range sums avoid subtracting a large departing power from a rolling
// total, which can erase small positive powers after a high-to-low transition.
struct RangeSums {
    leaves: usize,
    nodes: Vec<Sum>,
}
impl RangeSums {
    fn new(values: &[Option<f64>]) -> Option<Self> {
        let leaves = values.len().checked_next_power_of_two()?;
        let mut nodes = vec![Sum::default(); leaves.checked_mul(2)?];
        for (index, value) in values.iter().enumerate() {
            nodes[leaves + index] = match value.filter(|value| value.is_finite()) {
                Some(total) => Sum { total, gaps: 0 },
                None => Sum {
                    total: 0.0,
                    gaps: 1,
                },
            };
        }
        for index in (1..leaves).rev() {
            nodes[index] = nodes[index * 2].add(nodes[index * 2 + 1]);
        }
        Some(Self { leaves, nodes })
    }
    fn range(&self, start: usize, end: usize) -> Sum {
        let mut lower = self.leaves + start;
        let mut upper = self.leaves + end;
        let mut result = Sum::default();
        while lower < upper {
            if lower % 2 == 1 {
                result = result.add(self.nodes[lower]);
                lower += 1;
            }
            if upper % 2 == 1 {
                upper -= 1;
                result = result.add(self.nodes[upper]);
            }
            lower /= 2;
            upper /= 2;
        }
        result
    }
}
