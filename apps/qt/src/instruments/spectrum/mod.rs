/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared spectrum viewport formatting; the core owns the actual PSD calculation.
pub(crate) mod symbol_timing;
use docsis_ofdm_fec::signal_processing::waveform::spectrum::{density_db, PowerSpectrum};
use serde_json::{json, Value};
/// Formats one bounded viewport over an immutable Rust spectrum.
/// # Arguments
/// * `spectrum` - Actual TX/RX Hann PSD with explicit normalization.
/// * `start` - First bin; negative values reject.
/// * `count` - Bin count or zero for full span.
/// * `capture_axis` - Select captured frequency coordinates, without upconversion.
/// * `detail` - Explicit sample/reference-plane provenance for callers.
/// # Returns
/// At most 2048 min/max groups with Rust-derived coordinates and labels.
/// # Errors
/// Reports an invalid viewport without returning stale points.
pub(crate) fn viewport(
    spectrum: &PowerSpectrum,
    start: i32,
    count: i32,
    capture_axis: bool,
    detail: &str,
) -> Result<Value, String> {
    let start = usize::try_from(start).map_err(|_| "Invalid spectrum start.")?;
    let count = if count == 0 {
        spectrum.fft_samples
    } else {
        usize::try_from(count).map_err(|_| "Invalid spectrum count.")?
    };
    if count < 2 {
        return Err("Display at least two spectrum bins.".into());
    }
    let buckets = spectrum
        .buckets(start, count)
        .map_err(|error| error.to_string())?;
    let highest = spectrum
        .density
        .iter()
        .filter_map(|power| density_db(*power))
        .fold(f64::NEG_INFINITY, f64::max);
    let top = if highest.is_finite() {
        (highest / 10.0).ceil() * 10.0 + 10.0
    } else {
        0.0
    };
    let bottom = top - 100.0;
    let normalize = |db: Option<f64>| (top - db.unwrap_or(bottom).clamp(bottom, top)) / 100.0;
    let frequency = |bin: f64| {
        if capture_axis {
            f64::from(spectrum.zero_frequency_hz) + bin * f64::from(spectrum.spacing_hz)
        } else {
            (bin - spectrum.fft_samples as f64 / 2.0) * f64::from(spectrum.spacing_hz)
        }
    };
    let display = |db: Option<f64>| {
        db.map(|value| format!("{value:.2}"))
            .unwrap_or_else(|| "−∞ (zero)".into())
    };
    let points:Vec<_>=buckets.iter().map(|bucket|json!({
            "x":((bucket.first_bin+bucket.last_bin) as f64/2.0-start as f64)/(count-1) as f64,
            "y":normalize(bucket.maximum_db),"minimum_y":normalize(bucket.minimum_db),
            "readout":format!("{:.3}–{:.3} MHz · peak {} · min {} dB/Hz",frequency(bucket.first_bin as f64)/1e6,frequency(bucket.last_bin as f64)/1e6,display(bucket.maximum_db),display(bucket.minimum_db))
        })).collect();
    let intervals = (count - 1).min(4);
    let x_ticks: Vec<_> = (0..=intervals)
        .map(|tick| {
            let bin = if tick == intervals {
                start + count - 1
            } else {
                start + count * tick / intervals
            };
            let x = (bin - start) as f64 / (count - 1) as f64;
            json!({"x": x, "label": format!("{:.3}", frequency(bin as f64) / 1e6)})
        })
        .collect();
    let y_ticks: Vec<_> = (0..=5)
        .map(|tick| {
            let y = f64::from(tick) / 5.0;
            json!({"y":y,"label":format!("{:.0}",top-y*100.0)})
        })
        .collect();
    Ok(
        json!({"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,"start":start,"count":count,"total":spectrum.fft_samples,
            "axis_label":"PSD · dB re 1 normalized unit²/Hz","frequency_label":if capture_axis {"Capture frequency mapping (MHz) · no RF upconversion"} else {"Baseband frequency offset (MHz)"},
            "summary":format!("204.8 MS/s · FFT {} · Hann · {} averages · bin {:.1} kHz · ENBW {:.1} kHz",spectrum.fft_samples,spectrum.averages,f64::from(spectrum.spacing_hz)/1000.0,spectrum.equivalent_noise_bandwidth_hz/1000.0),
            "detail":detail,
            "trace_label":"Peak trace and min/max envelope within each displayed bin group"}),
    )
}
