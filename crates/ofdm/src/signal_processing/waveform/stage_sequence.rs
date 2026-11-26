/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit diagnostic ordering of actual frame-stage samples, preserving their physical origins.
use super::spectrum::{MAX_SPECTRUM_AVERAGES, WELCH_HOP_DIVISOR};
use crate::{error::invalid, transmission::SubcarrierSpacing, SimulationError};
use serde::Serialize;

/// Application retention guard; independent of the PHY symbol count or Welch averages.
pub(crate) const MAX_STAGE_SEQUENCE_FRAMES: usize = 64;

pub(crate) struct StageSequenceFrame<'samples> {
    pub retained_index: usize,
    pub channel_symbol: u64,
    /// Original stage-frame origin before any retained element offset.
    pub frame_start: u64,
    pub element_start: usize,
    pub total_elements: usize,
    pub samples: &'samples [[f64; 2]],
    pub context_compatible: bool,
}
/// One contributing original frame interval and its explicit diagnostic sequence coordinates.
#[derive(Serialize)]
#[non_exhaustive]
pub struct StageSequenceSegment {
    pub retained_index: usize,
    pub channel_symbol_index: u64,
    pub original_sample_start: u64,
    pub element_start: usize,
    pub total_elements: usize,
    pub retained_elements: usize,
    pub sequence_start: usize,
    pub sample_count: usize,
    /// Values actually consumed by complete Welch windows, assigned after analysis.
    pub analyzed_samples: usize,
}
/// Explicit termination of finite diagnostic collection; no missing samples are synthesized.
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StageSequenceStop {
    EndOfRetainedHistory,
    WorkLimit,
    FrameLimit,
    IncompatibleContext,
    ChannelSymbolGap,
    OriginalStrideMismatch,
    MissingFramePrefix,
    IncompleteFrame,
}
pub(crate) struct StageSequenceWindow {
    pub samples: Vec<[f64; 2]>,
    pub segments: Vec<StageSequenceSegment>,
    pub stop: StageSequenceStop,
    pub physically_contiguous: bool,
}

/// Collects retained frame-stage intervals without asserting a continuous physical sample clock.
/// # Arguments
/// * `frames` - Ordered actual borrowed frames with caller-validated context compatibility.
/// * `stride` - Original channel-symbol stride in physical samples, independent of frame length.
/// * `max_samples` - At most 8192 + 31*4096 diagnostic samples; only 64 frames can contribute.
/// # Returns
/// Owned exact samples and original-index segments, with explicit stop reason and contiguity.
/// # Errors
/// Rejects invalid work, retained extents or overflowing original coordinates; starts no stage.
pub(crate) fn collect_sequence<'samples>(
    frames: impl Iterator<Item = StageSequenceFrame<'samples>>,
    stride: usize,
    max_samples: usize,
) -> Result<StageSequenceWindow, SimulationError> {
    let largest_fft_samples = SubcarrierSpacing::Khz25.fft_samples() as usize;
    let maximum_analysis_samples = largest_fft_samples
        + (MAX_SPECTRUM_AVERAGES - 1) * (largest_fft_samples / WELCH_HOP_DIVISOR);
    if stride == 0 || !(1..=maximum_analysis_samples).contains(&max_samples) {
        return Err(invalid("Invalid stage-sequence sample/stride budget"));
    }
    let mut result = StageSequenceWindow {
        samples: Vec::with_capacity(max_samples),
        segments: Vec::new(),
        stop: StageSequenceStop::EndOfRetainedHistory,
        physically_contiguous: true,
    };
    let mut previous: Option<(u64, u64)> = None;
    for frame in frames.take(MAX_STAGE_SEQUENCE_FRAMES + 1) {
        let stopped = if result.segments.len() == MAX_STAGE_SEQUENCE_FRAMES {
            Some(StageSequenceStop::FrameLimit)
        } else if !frame.context_compatible {
            Some(StageSequenceStop::IncompatibleContext)
        } else if let Some((symbol, origin)) = previous {
            if symbol.checked_add(1) != Some(frame.channel_symbol) {
                Some(StageSequenceStop::ChannelSymbolGap)
            } else if origin.checked_add(stride as u64) != Some(frame.frame_start) {
                Some(StageSequenceStop::OriginalStrideMismatch)
            } else if frame.element_start != 0 {
                Some(StageSequenceStop::MissingFramePrefix)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(reason) = stopped {
            result.stop = reason;
            break;
        }
        let available_end = frame
            .element_start
            .checked_add(frame.samples.len())
            .filter(|end| *end <= frame.total_elements)
            .ok_or_else(|| invalid("Stage-sequence retained extent is invalid"))?;
        if frame.samples.is_empty() {
            return Err(invalid("Stage-sequence frame has no retained samples"));
        }
        let origin = frame
            .frame_start
            .checked_add(frame.element_start as u64)
            .ok_or_else(|| invalid("Stage-sequence origin overflows"))?;
        if let Some(last) = result.segments.last() {
            let end = last
                .original_sample_start
                .checked_add(last.sample_count as u64)
                .ok_or_else(|| invalid("Stage-sequence original interval overflows"))?;
            result.physically_contiguous &= end == origin;
        }
        let count = frame.samples.len().min(max_samples - result.samples.len());
        origin
            .checked_add(count as u64)
            .ok_or_else(|| invalid("Stage-sequence original interval overflows"))?;
        result.segments.push(StageSequenceSegment {
            retained_index: frame.retained_index,
            channel_symbol_index: frame.channel_symbol,
            original_sample_start: origin,
            element_start: frame.element_start,
            total_elements: frame.total_elements,
            retained_elements: frame.samples.len(),
            sequence_start: result.samples.len(),
            sample_count: count,
            analyzed_samples: 0,
        });
        result.samples.extend_from_slice(&frame.samples[..count]);
        previous = Some((frame.channel_symbol, frame.frame_start));
        if result.samples.len() == max_samples {
            result.stop = StageSequenceStop::WorkLimit;
            break;
        }
        if available_end != frame.total_elements {
            result.stop = StageSequenceStop::IncompleteFrame;
            break;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn frame(symbol: u64, origin: u64, values: &[[f64; 2]]) -> StageSequenceFrame<'_> {
        StageSequenceFrame {
            retained_index: symbol as usize,
            channel_symbol: symbol,
            frame_start: origin,
            element_start: 0,
            total_elements: values.len(),
            samples: values,
            context_compatible: true,
        }
    }

    #[test]
    fn literal_gap_overlap_and_contiguous_sequences_preserve_all_original_coordinates() {
        let a = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]];
        let b = [[9.0, 10.0], [11.0, 12.0], [13.0, 14.0], [15.0, 16.0]];
        for (stride, contiguous) in [(6, false), (3, false), (4, true)] {
            let result = collect_sequence(
                [frame(8, 100, &a), frame(9, 100 + stride as u64, &b)].into_iter(),
                stride,
                7,
            )
            .unwrap();
            assert_eq!(result.samples, [&a[..], &b[..3]].concat());
            assert_eq!(result.physically_contiguous, contiguous);
            assert_eq!(result.stop, StageSequenceStop::WorkLimit);
            assert_eq!(result.segments[0].original_sample_start, 100);
            assert_eq!(
                result.segments[1].original_sample_start,
                100 + stride as u64
            );
            assert_eq!(result.segments[1].sequence_start, 4);
            assert_eq!(result.segments[1].sample_count, 3);
        }
    }

    #[test]
    fn missing_or_incompatible_history_stops_without_stitching_or_padding() {
        let samples = [[1.0, 0.0]; 4];
        for (case, reason) in [
            StageSequenceStop::ChannelSymbolGap,
            StageSequenceStop::OriginalStrideMismatch,
            StageSequenceStop::MissingFramePrefix,
            StageSequenceStop::IncompatibleContext,
        ]
        .into_iter()
        .enumerate()
        {
            let mut second = frame(9, 106, &samples);
            match case {
                0 => second.channel_symbol = 10,
                1 => second.frame_start = 107,
                2 => {
                    second.element_start = 1;
                    second.total_elements = 5
                }
                _ => second.context_compatible = false,
            }
            let result =
                collect_sequence([frame(8, 100, &samples), second].into_iter(), 6, 20).unwrap();
            assert_eq!(result.samples, samples);
            assert_eq!(result.stop, reason);
        }
        let mut first = frame(8, 100, &samples);
        first.total_elements = 5;
        let result = collect_sequence([first, frame(9, 106, &samples)].into_iter(), 6, 20).unwrap();
        assert_eq!(result.stop, StageSequenceStop::IncompleteFrame);
        assert_eq!(result.samples.len(), 4);
    }

    #[test]
    fn initial_offset_work_frame_and_overflow_bounds_are_explicit() {
        // Independent expected limits pin the accepted policy, without importing its definition.
        const EXPECTED_FRAME_LIMIT: usize = 64;
        const OVER_LIMIT_ANALYSIS_SAMPLES: usize = 135_169;
        const EXCESS_FRAME_INPUTS: u64 = 1000;
        const AMPLE_SAMPLE_BUDGET: usize = 1000;
        let samples = [[2.0, 3.0]; 4];
        let mut first = frame(8, 100, &samples[1..]);
        first.element_start = 1;
        first.total_elements = 4;
        let result = collect_sequence([first, frame(9, 106, &samples)].into_iter(), 6, 20).unwrap();
        assert_eq!(result.segments[0].original_sample_start, 101);
        assert_eq!(result.segments[0].element_start, 1);
        assert_eq!(result.samples.len(), 7);
        assert_eq!(result.stop, StageSequenceStop::EndOfRetainedHistory);
        let result = collect_sequence(
            (0..EXCESS_FRAME_INPUTS).map(|symbol| frame(symbol, symbol * 4, &samples)),
            4,
            AMPLE_SAMPLE_BUDGET,
        )
        .unwrap();
        assert_eq!(result.segments.len(), EXPECTED_FRAME_LIMIT);
        assert_eq!(result.stop, StageSequenceStop::FrameLimit);
        for budget in [0, OVER_LIMIT_ANALYSIS_SAMPLES, usize::MAX] {
            assert!(collect_sequence([frame(0, 0, &samples)].into_iter(), 4, budget).is_err());
        }
        assert!(collect_sequence([frame(0, u64::MAX, &samples)].into_iter(), 4, 20).is_err());
        let mut invalid = frame(0, 0, &samples);
        invalid.element_start = usize::MAX;
        assert!(collect_sequence([invalid].into_iter(), 4, 20).is_err());
    }
}
