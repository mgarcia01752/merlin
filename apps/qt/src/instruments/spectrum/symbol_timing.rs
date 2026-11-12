/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded time-axis projections of the actual core transmit-window coefficients.
use docsis_ofdm_fec::{
    signal_processing::waveform::{windowing::TransmitWindow, WaveformGeometry},
    transmission::settings::{CYCLIC_PREFIX_SAMPLES, ROLL_OFF_SAMPLES, SAMPLE_RATE_HZ},
};
use serde_json::{json, Value};

/// Projects two adjacent windows and their shared tail on real time axes.
/// # Arguments
/// * `geometry` - Validated active or frozen geometry, with unchanged core weights.
/// # Returns
/// Bounded overview/detail graphs, each with at most 550 actual sample positions.
/// Zero outside a window is explicit; no waveform or extra symbol is synthesized.
pub(crate) fn project(geometry: &WaveformGeometry) -> Value {
    let stride = geometry.stride_samples();
    let roll = geometry.roll_off_samples();
    let prefix = geometry.prefix_samples();
    let window = TransmitWindow::new(geometry);
    let maximum_span = 2
        * (geometry.fft_samples() + usize::from(*CYCLIC_PREFIX_SAMPLES.last().unwrap()))
        + usize::from(*ROLL_OFF_SAMPLES.last().unwrap());
    let full_us = (microseconds(maximum_span) / 10.0).ceil() * 10.0;
    let full_end = (full_us * f64::from(SAMPLE_RATE_HZ) / 1e6).round() as usize;
    let maximum_roll = usize::from(*ROLL_OFF_SAMPLES.last().unwrap());
    let margin = maximum_roll / 2;
    let detail_start = stride.saturating_sub(margin);
    let detail_end = stride + usize::from(*CYCLIC_PREFIX_SAMPLES.last().unwrap()) + margin;
    json!({
        "prefix_us":microseconds(prefix),"roll_us":microseconds(roll),
        "fft_us":microseconds(geometry.fft_samples()),"symbol_us":microseconds(stride),
        "overview":plot(&window, geometry, 0, full_end),
        "detail":plot(&window, geometry, detail_start, detail_end),
        "overlap_label":format!("Overlap {:.3} µs · Usable guard {:.3} µs · CP end / FFT start {:.3} µs",microseconds(roll),microseconds(prefix-roll),microseconds(stride+prefix)),
        "origin_note":"Window weights, not RF signal amplitude."
    })
}

fn microseconds(samples: usize) -> f64 {
    samples as f64 * 1e6 / f64::from(SAMPLE_RATE_HZ)
}

fn plot(window: &TransmitWindow, geometry: &WaveformGeometry, start: usize, end: usize) -> Value {
    let stride = geometry.stride_samples();
    let roll = geometry.roll_off_samples();
    let prefix = geometry.prefix_samples();
    let mut indices = (0..=256)
        .map(|index| start + (end - start) * index / 256)
        .collect::<Vec<_>>();
    // Keep both taper shapes visible even on the full two-symbol timeline.
    for origin in [0, stride] {
        for taper_start in [origin, origin + stride] {
            for index in 0..=64 {
                indices.push(taper_start + roll * index / 64);
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
            .and_then(|local| window.coefficients().get(local))
            .copied()
            .unwrap_or(0.0)
    };
    let coordinate = |index: usize| (index as f64 - start as f64) / (end - start) as f64;
    let points=indices.into_iter().map(|index|{
        let first=weight(index,0);
        let second=weight(index,stride);
        json!({"x":coordinate(index),"first":first,"second":second,
            "sum":if index>=stride && index<stride+roll {Some(first+second)} else {None},
            "readout":format!("{:.3} µs · Symbol 1 {first:.4} · Symbol 2 {second:.4}",microseconds(index))})
    }).collect::<Vec<_>>();
    let ticks = (0..=4)
        .map(|index| {
            let sample = start + (end - start) * index / 4;
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
        (left>=start && right<=end).then(||json!({"x":coordinate(left+(right-left)/2),"text":format!("FFT · {:.3} µs",microseconds(geometry.fft_samples()))}))
    }).collect::<Vec<_>>();
    json!({"points":points,"ticks":ticks,"bands":bands,"markers":markers,"fft_labels":fft_labels,
        "start_us":microseconds(start),"end_us":microseconds(end)})
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::transmission::{SubcarrierSpacing, TransmissionSettings};
    #[test]
    fn both_ffts_and_all_valid_prefix_tapers_preserve_core_weights_and_overlap() {
        for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
            for prefix in CYCLIC_PREFIX_SAMPLES {
                for roll in ROLL_OFF_SAMPLES {
                    if roll >= prefix {
                        continue;
                    }
                    let settings = TransmissionSettings {
                        subcarrier_spacing: spacing,
                        cyclic_prefix_samples: prefix,
                        roll_off_samples: roll,
                        ..Default::default()
                    };
                    let geometry = WaveformGeometry::new(&settings).unwrap();
                    let report = project(&geometry);
                    let overview = &report["overview"];
                    let detail = &report["detail"];
                    let cp_end_us = microseconds(geometry.stride_samples() + usize::from(prefix));
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
                    assert!((guard_us - f64::from(prefix - roll) / 204.8).abs() < 1e-12);
                    assert!(detail["markers"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|marker| marker["label"] == "CP end / FFT start"));
                    assert!(overview["points"].as_array().unwrap().len() <= 550);
                    assert!(
                        overview["end_us"].as_f64().unwrap()
                            > microseconds(2 * geometry.stride_samples() + usize::from(roll))
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
        let first = project(&WaveformGeometry::new(&settings).unwrap());
        let changed = TransmissionSettings {
            cyclic_prefix_samples: 1024,
            roll_off_samples: 256,
            ..settings
        };
        let second = project(&WaveformGeometry::new(&changed).unwrap());
        assert_eq!(first["overview"]["end_us"], second["overview"]["end_us"]);
        assert_ne!(first["symbol_us"], second["symbol_us"]);
        assert_eq!(second["symbol_us"], 45.0);
        assert_eq!(second["roll_us"], 1.25);
    }
}
