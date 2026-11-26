/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded time-axis projections of the actual core transmit-window coefficients.
use crate::downstream::common::constants::{
    TIMING_DRAWING_INTERVALS, TIMING_OVERVIEW_QUANTUM_MICROSECONDS,
    TIMING_PREFIX_ALLOWANCE_SAMPLES, TIMING_ROLL_ALLOWANCE_SAMPLES, TIMING_TAPER_DRAWING_INTERVALS,
    TIMING_TICK_INTERVALS,
};
use crate::shared::presentation::constants::MICROSECONDS_PER_SECOND;
use merlin::downstream::SymbolWindow;
use serde_json::{json, Value};

/// Projects two adjacent windows and their shared tail on real time axes.
/// # Arguments
/// * `window` - Owned public snapshot of active/frozen geometry and unchanged backend weights.
/// # Returns
/// Bounded overview/detail graphs, each with at most 550 actual sample positions.
/// Zero outside a window is explicit; no waveform or extra symbol is synthesized.
pub(crate) fn project(window: &SymbolWindow) -> Value {
    let microseconds = |samples| microseconds(samples, window.sample_rate_hz);
    let stride = window.stride_samples;
    let roll = window.roll_off_samples;
    let prefix = window.prefix_samples;
    let maximum_span =
        2 * (window.fft_samples + TIMING_PREFIX_ALLOWANCE_SAMPLES) + TIMING_ROLL_ALLOWANCE_SAMPLES;
    let full_us = (microseconds(maximum_span) / TIMING_OVERVIEW_QUANTUM_MICROSECONDS).ceil()
        * TIMING_OVERVIEW_QUANTUM_MICROSECONDS;
    let full_end =
        (full_us * f64::from(window.sample_rate_hz) / MICROSECONDS_PER_SECOND).round() as usize;
    let maximum_roll = TIMING_ROLL_ALLOWANCE_SAMPLES;
    let margin = maximum_roll / 2;
    let detail_start = stride.saturating_sub(margin);
    let detail_end = stride + TIMING_PREFIX_ALLOWANCE_SAMPLES + margin;
    json!({
        "prefix_us":microseconds(prefix),"roll_us":microseconds(roll),
        "fft_us":microseconds(window.fft_samples),"symbol_us":microseconds(stride),
        "overview":plot(window, 0, full_end),
        "detail":plot(window, detail_start, detail_end),
        "overlap_label":format!("Overlap {:.3} µs · Usable guard {:.3} µs · CP end / FFT start {:.3} µs",microseconds(roll),microseconds(prefix-roll),microseconds(stride+prefix)),
        "origin_note":"Window weights, not RF signal amplitude."
    })
}

fn microseconds(samples: usize, sample_rate_hz: u32) -> f64 {
    samples as f64 * MICROSECONDS_PER_SECOND / f64::from(sample_rate_hz)
}

fn plot(window: &SymbolWindow, start: usize, end: usize) -> Value {
    let microseconds = |samples| microseconds(samples, window.sample_rate_hz);
    let stride = window.stride_samples;
    let roll = window.roll_off_samples;
    let prefix = window.prefix_samples;
    let mut indices = (0..=TIMING_DRAWING_INTERVALS)
        .map(|index| start + (end - start) * index / TIMING_DRAWING_INTERVALS)
        .collect::<Vec<_>>();
    // Keep both taper shapes visible even on the full two-symbol timeline.
    for origin in [0, stride] {
        for taper_start in [origin, origin + stride] {
            for index in 0..=TIMING_TAPER_DRAWING_INTERVALS {
                indices.push(taper_start + roll * index / TIMING_TAPER_DRAWING_INTERVALS);
            }
        }
        for boundary in [
            origin,
            origin + prefix,
            origin + stride,
            origin + stride + roll,
        ] {
            indices.push(boundary);
            indices.push(boundary.saturating_sub(1));
        }
    }
    indices.retain(|index| *index >= start && *index <= end);
    indices.sort_unstable();
    indices.dedup();
    let weight = |index: usize, origin: usize| {
        index
            .checked_sub(origin)
            .and_then(|local| window.coefficients.get(local))
            .copied()
            .unwrap_or(0.0)
    };
    let coordinate = |index: usize| (index as f64 - start as f64) / (end - start) as f64;
    let points=indices.into_iter().map(|index|{
        let first=weight(index,0);
        let second=weight(index,stride);
        json!({"x":coordinate(index),"first":first,"second":second,
            "sum":if index>=stride && index<stride+roll {
                Some(first+second)
            } else {
                None
            },
            "readout":format!("{:.3} µs · Symbol 1 {first:.4} · Symbol 2 {second:.4}",microseconds(index))})
    }).collect::<Vec<_>>();
    let ticks = (0..=TIMING_TICK_INTERVALS)
        .map(|index| {
            let sample = start + (end - start) * index / TIMING_TICK_INTERVALS;
            json!({"x":coordinate(sample),"label":format!("{:.3}",microseconds(sample))})
        })
        .collect::<Vec<_>>();
    let bands = [0, stride]
        .into_iter()
        .flat_map(|origin| {
            [
                (origin, origin + roll, "overlap", "Overlap"),
                (origin + roll, origin + prefix, "guard", "Usable guard"),
            ]
            .into_iter()
            .filter_map(move |(left, right, kind, label)| {
                let left = left.max(start);
                let right = right.min(end);
                (right > left).then(|| {
                    json!({"x":coordinate(left),"end_x":coordinate(right),
                        "kind":kind,"label":label})
                })
            })
        })
        .collect::<Vec<_>>();
    let markers = [0, stride]
        .into_iter()
        .flat_map(|origin| {
            [
                (origin, "CP start"),
                (origin + roll, "Overlap end"),
                (origin + prefix, "CP end / FFT start"),
            ]
            .into_iter()
            .filter(|(index, _)| *index >= start && *index <= end)
            .map(|(index, label)| {
                json!({"x":coordinate(index),"label":label,
                    "time_label":format!("{:.3} µs",microseconds(index))})
            })
        })
        .collect::<Vec<_>>();
    let fft_labels = [0,stride].into_iter().filter_map(|origin| {
        let left = origin+prefix;
        let right = origin+stride;
        (left>=start && right<=end).then(||json!({"x":coordinate(left+(right-left)/2),"text":format!("FFT · {:.3} µs",microseconds(window.fft_samples))}))
    }).collect::<Vec<_>>();
    json!({"points":points,"ticks":ticks,"bands":bands,"markers":markers,"fft_labels":fft_labels,
        "start_us":microseconds(start),"end_us":microseconds(end)})
}

#[cfg(test)]
mod tests {
    use super::*;
    use merlin::downstream::{SubcarrierSpacing, TransmissionConfiguration, TransmissionSettings};
    // Independent original parameter matrix, not the production display-padding definitions.
    const PREFIX_CASES: [u16; 5] = [192, 256, 512, 768, 1024];
    const ROLL_CASES: [u16; 5] = [0, 64, 128, 192, 256];
    #[test]
    fn both_ffts_and_all_valid_prefix_tapers_preserve_core_weights_and_overlap() {
        for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
            for prefix in PREFIX_CASES {
                for roll in ROLL_CASES {
                    if roll >= prefix {
                        continue;
                    }
                    let settings = TransmissionSettings {
                        subcarrier_spacing: spacing,
                        cyclic_prefix_samples: prefix,
                        roll_off_samples: roll,
                        ..Default::default()
                    };
                    let window = TransmissionConfiguration::new(settings)
                        .unwrap()
                        .window()
                        .unwrap();
                    let microseconds = |samples| microseconds(samples, window.sample_rate_hz);
                    let report = project(&window);
                    let overview = &report["overview"];
                    let detail = &report["detail"];
                    let cp_end_us = microseconds(window.stride_samples + usize::from(prefix));
                    assert!(detail["end_us"].as_f64().unwrap() > cp_end_us);
                    let guard = detail["bands"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|band| band["kind"] == "guard")
                        .unwrap();
                    let span =
                        detail["end_us"].as_f64().unwrap() - detail["start_us"].as_f64().unwrap();
                    let guard_us =
                        (guard["end_x"].as_f64().unwrap() - guard["x"].as_f64().unwrap()) * span;
                    // Independent sample clock: 204.8 samples/us, with picosecond-scale tolerance.
                    assert!((guard_us - f64::from(prefix - roll) / 204.8).abs() < 1e-12);
                    assert!(detail["markers"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|marker| marker["label"] == "CP end / FFT start"));
                    assert!(overview["points"].as_array().unwrap().len() <= 550);
                    assert!(
                        overview["end_us"].as_f64().unwrap()
                            > microseconds(2 * window.stride_samples + usize::from(roll))
                    );
                    for point in report["detail"]["points"].as_array().unwrap() {
                        if let Some(sum) = point["sum"].as_f64() {
                            assert!((sum - 1.0).abs() < 1e-12);
                        }
                    }
                    if roll > 0 {
                        let first = overview["points"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|point| point["x"] == 0.0)
                            .unwrap()["first"]
                            .as_f64()
                            .unwrap();
                        // Independent half-sample raised-cosine anchor, not backend coefficients.
                        let expected =
                            0.5 * (1.0 - (std::f64::consts::PI * 0.5 / f64::from(roll)).cos());
                        assert!((first - expected).abs() < 1e-14);
                    }
                }
            }
        }
    }
    #[test]
    fn cp_and_roll_changes_move_timing_without_self_rescaling_the_overview() {
        let settings = TransmissionSettings::default();
        let first = project(
            &TransmissionConfiguration::new(settings.clone())
                .unwrap()
                .window()
                .unwrap(),
        );
        let changed = TransmissionSettings {
            cyclic_prefix_samples: 1024,
            roll_off_samples: 256,
            ..settings
        };
        let second = project(
            &TransmissionConfiguration::new(changed)
                .unwrap()
                .window()
                .unwrap(),
        );
        assert_eq!(first["overview"]["end_us"], second["overview"]["end_us"]);
        assert_ne!(first["symbol_us"], second["symbol_us"]);
        assert_eq!(second["symbol_us"], 45.0);
        assert_eq!(second["roll_us"], 1.25);
    }
}
