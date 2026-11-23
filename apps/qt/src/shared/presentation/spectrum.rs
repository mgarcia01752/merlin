/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Spectrum drawing coordinates and labels over authoritative public API observations.
use super::constants::*;
use merlin::downstream::{ChannelSpectrum, PhysicalLayoutView};
use serde_json::{json, Value};

/// Formats an admitted spectrum viewport without generating or calibrating scientific data.
///
/// # Arguments
/// * `spectrum` - Backend-calibrated bounded buckets and original full-band power.
/// * `full` - Same observation at full span, used only to preserve the display scale on zoom.
/// * `layout` - Authoritative configured physical PLC bounds on the same FFT grid.
/// * `start`, `count` - Already admitted original-bin viewport, with at least two bins.
/// * `capture_axis` - Select physical frequency labels instead of centered baseband labels.
/// * `detail` - Actual sample/reference-plane presentation text.
///
/// # Returns
/// Existing bounded SA shape, PLC shading, units and full-band power readout. All
/// PSD, integration, calibration and logarithmic conversion come from the backend.
pub(crate) fn viewport(
    spectrum: &ChannelSpectrum,
    full: &ChannelSpectrum,
    layout: &PhysicalLayoutView,
    start: usize,
    count: usize,
    capture_axis: bool,
    detail: &str,
) -> Value {
    let highest = full
        .buckets
        .iter()
        .filter_map(|bucket| bucket.maximum_db)
        .fold(f64::NEG_INFINITY, f64::max);
    let top = if highest.is_finite() {
        (highest / SPECTRUM_TOP_GRID_DB).ceil() * SPECTRUM_TOP_GRID_DB + SPECTRUM_HEADROOM_DB
    } else {
        0.0
    };
    let bottom = top - SPECTRUM_DISPLAY_SPAN_DB;
    let normalize = |db: Option<f64>| {
        (top - db.unwrap_or(bottom).clamp(bottom, top)) / SPECTRUM_DISPLAY_SPAN_DB
    };
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
    let electrical = spectrum.millivolts_squared_per_unit.is_some();
    let density_label = if electrical {
        // Preserve the backend's explicit electrical PSD reference.
        "dBmV/Hz"
    } else {
        "dB/Hz"
    };
    let points: Vec<_> = spectrum
        .buckets
        .iter()
        .map(|bucket| {
            json!({
                "x":((bucket.first_bin+bucket.last_bin) as f64/2.0-start as f64)/(count-1) as f64,
                "y":normalize(bucket.maximum_db),"minimum_y":normalize(bucket.minimum_db),
                "readout":format!("{:.3}–{:.3} MHz · peak {} · min {} {density_label}",
                    frequency(bucket.first_bin as f64)/HERTZ_PER_MEGAHERTZ,
                    frequency(bucket.last_bin as f64)/HERTZ_PER_MEGAHERTZ,
                    display(bucket.maximum_db),display(bucket.minimum_db))
            })
        })
        .collect();
    let intervals = (count - 1).min(SPECTRUM_FREQUENCY_TICK_INTERVALS);
    let x_ticks: Vec<_> = (0..=intervals)
        .map(|tick| {
            let bin = if tick == intervals {
                start + count - 1
            } else {
                start + count * tick / intervals
            };
            json!({"x":(bin-start) as f64/(count-1) as f64,
            "label":format!("{:.3}",frequency(bin as f64)/HERTZ_PER_MEGAHERTZ)})
        })
        .collect();
    let y_ticks: Vec<_> = (0..=SPECTRUM_POWER_TICK_INTERVALS)
        .map(|tick| {
            let y = f64::from(tick) / f64::from(SPECTRUM_POWER_TICK_INTERVALS);
            json!({"y":y,"label":format!("{:.0}",top-y*SPECTRUM_DISPLAY_SPAN_DB)})
        })
        .collect();
    let reading = &spectrum.channel_power;
    let power_text = reading
        .dbmv
        .map(|value| format!("Channel Power: {value:.2} dBmV"))
        .unwrap_or_else(|| {
            let relative = spectrum
                .normalized_power_db
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "-inf".into());
            format!("Channel Power: {relative} dB re unit²")
        });
    let equivalent = reading
        .equivalent_dbmv_per_6mhz
        .map(|value| format!(" · {value:.2} dBmV / 6 MHz avg"))
        .unwrap_or_else(|| " · Relative units; set TX reference for dBmV".into());
    let sample_rate_millions = f64::from(spectrum.sample_rate_hz) / SAMPLES_PER_MEGASAMPLE;
    let mut view = json!({"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,
        "start":start,"count":count,"total":spectrum.fft_samples,
        "axis_label":if electrical {
            "PSD · dBmV/Hz"
        } else {
            "PSD · dB re 1 normalized unit²/Hz"
        },
        "frequency_label":if capture_axis {
            "Capture frequency mapping (MHz) · no RF upconversion"
        } else {
            "Baseband frequency offset (MHz)"
        },
        "summary":format!("{sample_rate_millions} MS/s · FFT {} · Hann · {} averages · bin {:.1} kHz · ENBW {:.1} kHz",
            spectrum.fft_samples,spectrum.averages,f64::from(spectrum.spacing_hz)/HERTZ_PER_KILOHERTZ,
            spectrum.equivalent_noise_bandwidth_hz/HERTZ_PER_KILOHERTZ),
        "detail":detail,"trace_label":"Peak trace and min/max envelope within each displayed bin group",
        "channel_power":reading,"channel_power_text":format!("{power_text}{equivalent} · {:.3} MHz band",reading.bandwidth_hz/HERTZ_PER_MEGAHERTZ),
        "bands":[],"plc_summary":""});
    let first = usize::from(layout.plc_first_bin);
    let last = usize::from(layout.plc_end_exclusive) - 1;
    if last >= start && first < start + count {
        let denominator = (count - 1) as f64;
        view["bands"] = json!([{"first":((first as f64-start as f64)/denominator).clamp(0.0,1.0),
            "last":((last as f64-start as f64)/denominator).clamp(0.0,1.0),"label":"PLC"}]);
        view["plc_summary"] = json!(format!("PLC shaded · first–last bins {first}–{last}"));
    }
    view
}
