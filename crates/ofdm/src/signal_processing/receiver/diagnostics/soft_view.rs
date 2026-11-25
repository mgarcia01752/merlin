/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit bounded display statistics over unchanged actual likelihood values.
use super::{llr_statistics, LlrStatistics};
use crate::{error::invalid, SimulationError};
use serde::Serialize;

/// Equal-width intervals across each symmetric likelihood display, with open outer endpoints.
const SOFT_HISTOGRAM_BIN_COUNT: usize = 40;
/// Each half of the symmetric display has the same number of intervals.
const SOFT_HISTOGRAM_BINS_PER_HALF: usize = SOFT_HISTOGRAM_BIN_COUNT / 2;
/// Full display half width in producer LLR units; endpoint values are counted separately.
const FULL_SOFT_HISTOGRAM_HALF_WIDTH: f64 = 100.0;
/// Independent near-zero detail half width in the same unchanged producer units.
const NEAR_ZERO_SOFT_HISTOGRAM_HALF_WIDTH: f64 = 10.0;
/// Maximum contiguous extrema groups; this bounds display copies without sampling input values.
const MAX_SOFT_ENVELOPE_GROUPS: usize = 1024;
/// Nonzero symmetric display extent for empty or sub-unit sequences, not an LLR floor.
const MIN_SOFT_SEQUENCE_ABS_LIMIT: f64 = 1.0;

/// Equal-width histogram with explicit open outer endpoints and original total count.
#[derive(Serialize)]
#[non_exhaustive]
pub struct SoftHistogram {
    pub half_width: f64,
    pub total_values: usize,
    pub shown_values: usize,
    pub bins: Vec<SoftHistogramBin>,
}
/// One original fixed interval; interior lower bounds are inclusive, upper bounds exclusive.
#[derive(Serialize)]
#[non_exhaustive]
pub struct SoftHistogramBin {
    pub low: f64,
    pub high: f64,
    pub count: usize,
}
/// Exact extrema of one contiguous original-index group, never a sampled or interpolated trace.
#[derive(Serialize)]
#[non_exhaustive]
pub struct SoftEnvelope {
    pub first: usize,
    pub last: usize,
    pub minimum: f64,
    pub maximum: f64,
}
/// Actual finite soft values summarized for display, without probability or BER inference.
#[derive(Serialize)]
#[non_exhaustive]
pub struct SoftValueView {
    pub sign_convention: &'static str,
    pub statistics: LlrStatistics,
    pub full: SoftHistogram,
    pub near_zero: SoftHistogram,
    pub negative_limit_count: usize,
    pub positive_limit_count: usize,
    pub sequence_abs_limit: f64,
    pub sequence: Vec<SoftEnvelope>,
}

fn histogram(values: &[f64], half_width: f64) -> SoftHistogram {
    let interval = half_width / SOFT_HISTOGRAM_BINS_PER_HALF as f64;
    let edges: Vec<_> = (0..SOFT_HISTOGRAM_BIN_COUNT)
        .map(|index| -half_width + index as f64 * interval)
        .collect();
    let mut counts = [0usize; SOFT_HISTOGRAM_BIN_COUNT];
    for value in values
        .iter()
        .copied()
        .filter(|value| *value > -half_width && *value < half_width)
    {
        let upper = edges.partition_point(|edge| *edge <= value);
        counts[upper.saturating_sub(1).min(SOFT_HISTOGRAM_BIN_COUNT - 1)] += 1;
    }
    let shown_values = counts.iter().sum();
    let bins = counts
        .into_iter()
        .enumerate()
        .map(|(index, count)| {
            let low = -half_width + index as f64 * interval;
            SoftHistogramBin {
                low,
                high: low + interval,
                count,
            }
        })
        .collect();
    SoftHistogram {
        half_width,
        total_values: values.len(),
        shown_values,
        bins,
    }
}

/// Summarizes finite actual soft values with explicit histogram boundaries and original indices.
/// # Arguments
/// * `values` - At most 32768 finite producer LLRs; positive favors zero.
/// * `first_element` - Original coding-order index, or zero for an explicit selected-cell sequence.
/// # Returns
/// Existing likelihood statistics, two 40-bin histograms, full-range endpoint counts and at most
/// 1024 exact contiguous extrema groups. No clipping, decoder change or probability estimate occurs.
/// # Errors
/// Rejects nonfinite/oversized values and original-index overflow without discarding observations.
pub(crate) fn prepare(
    values: &[f64],
    first_element: usize,
) -> Result<SoftValueView, SimulationError> {
    let statistics = llr_statistics(values)?;
    first_element
        .checked_add(values.len())
        .ok_or_else(|| invalid("Soft-value original index overflows"))?;
    let group_size = values.len().div_ceil(MAX_SOFT_ENVELOPE_GROUPS).max(1);
    let sequence = values
        .chunks(group_size)
        .enumerate()
        .map(|(group, values)| {
            let first = first_element + group * group_size;
            SoftEnvelope {
                first,
                last: first + values.len() - 1,
                minimum: values.iter().copied().fold(f64::INFINITY, f64::min),
                maximum: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            }
        })
        .collect();
    Ok(SoftValueView {
        sign_convention: "log_p_zero_over_p_one",
        statistics,
        full: histogram(values, FULL_SOFT_HISTOGRAM_HALF_WIDTH),
        near_zero: histogram(values, NEAR_ZERO_SOFT_HISTOGRAM_HALF_WIDTH),
        negative_limit_count: values
            .iter()
            .filter(|value| **value <= -FULL_SOFT_HISTOGRAM_HALF_WIDTH)
            .count(),
        positive_limit_count: values
            .iter()
            .filter(|value| **value >= FULL_SOFT_HISTOGRAM_HALF_WIDTH)
            .count(),
        sequence_abs_limit: values
            .iter()
            .fold(MIN_SOFT_SEQUENCE_ABS_LIMIT, |maximum, value| {
                maximum.max(value.abs())
            }),
        sequence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_edges_preserve_tiny_signed_values_and_adjacent_float_interval_membership() {
        let values = [-f64::MIN_POSITIVE, 0.0, f64::MIN_POSITIVE];
        let view = prepare(&values, 0).unwrap();
        assert_eq!(view.full.bins[19].count, 1);
        assert_eq!(view.full.bins[20].count, 2);
        assert_eq!(view.near_zero.bins[19].count, 1);
        assert_eq!(view.near_zero.bins[20].count, 2);
        let five = 5.0f64.to_bits();
        let view = prepare(
            &[f64::from_bits(five - 1), 5.0, f64::from_bits(five + 1)],
            0,
        )
        .unwrap();
        assert_eq!(view.full.bins[20].count, 1);
        assert_eq!(view.full.bins[21].count, 2);
        assert_eq!(view.near_zero.bins[29].count, 1);
        assert_eq!(view.near_zero.bins[30].count, 2);
    }
    #[test]
    fn fixed_histograms_preserve_literal_edges_endpoints_zero_and_original_indices() {
        let view = prepare(
            &[-100.0, -99.0, -10.0, -1.0, 0.0, 1.0, 10.0, 99.0, 100.0],
            64,
        )
        .unwrap();
        assert_eq!(view.statistics.count, 9);
        assert_eq!(view.full.total_values, 9);
        assert_eq!(view.full.shown_values, 7);
        assert_eq!(view.near_zero.shown_values, 3);
        assert_eq!(view.full.bins.len(), 40);
        assert_eq!(view.full.bins[0].count, 1);
        assert_eq!(view.full.bins[18].count, 1);
        assert_eq!(view.full.bins[19].count, 1);
        assert_eq!(view.full.bins[20].count, 2);
        assert_eq!(view.full.bins[22].count, 1);
        assert_eq!(view.full.bins[39].count, 1);
        assert_eq!(view.near_zero.bins[18].count, 1);
        assert_eq!(view.near_zero.bins[20].count, 1);
        assert_eq!(view.near_zero.bins[22].count, 1);
        assert_eq!(view.negative_limit_count, 1);
        assert_eq!(view.positive_limit_count, 1);
        assert_eq!(view.sequence[0].first, 64);
        assert_eq!(view.sequence.last().unwrap().last, 72);
        assert_eq!(view.sign_convention, "log_p_zero_over_p_one");
    }
    #[test]
    fn bounded_extrema_keep_every_value_and_invalid_observations_never_disappear() {
        let mut values = vec![0.0; 32768];
        values[15] = -100.0;
        values[16] = 100.0;
        let view = prepare(&values, 128).unwrap();
        assert_eq!(view.sequence.len(), 1024);
        assert_eq!(view.sequence[0].first, 128);
        assert_eq!(view.sequence[0].last, 159);
        assert_eq!(view.sequence[0].minimum, -100.0);
        assert_eq!(view.sequence[0].maximum, 100.0);
        assert_eq!(view.sequence.last().unwrap().last, 32895);
        assert_eq!(
            view.full.shown_values + view.negative_limit_count + view.positive_limit_count,
            32768
        );
        let empty = prepare(&[], 0).unwrap();
        assert_eq!(empty.statistics.count, 0);
        assert!(empty.sequence.is_empty());
        assert_eq!(empty.sequence_abs_limit, 1.0);
        assert!(prepare(&[f64::NAN], 0).is_err());
        assert!(prepare(&[f64::INFINITY], 0).is_err());
        assert!(prepare(&vec![0.0; 32769], 0).is_err());
        assert!(prepare(&[0.0], usize::MAX).is_err());
    }
}
