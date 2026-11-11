/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One channel instrument provider with frozen, explicitly identified reference planes.
use crate::simulation::closed_loop::latest;
use docsis_ofdm_fec::{
    signal_processing::{channel::power::channel_power, waveform::spectrum::estimate},
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
};
use serde_json::{json, Value};
const NAMES: [&str; 3] = [
    "Before Amplifier",
    "After Amplifier",
    "Channel Output / RX Input",
];

/// Identifies availability without substituting another boundary or starting a run.
/// # Arguments
/// * `tap` - Before amplifier0, after amplifier1 or receiver input2.
/// # Returns
/// Instrument state bound to the immutable completed run.
pub(crate) fn state(tap: i32) -> Value {
    let index = usize::try_from(tap).ok().filter(|i| *i < 3);
    let result = latest();
    let available = index.zip(result.as_ref()).is_some_and(|(tap, r)| {
        if tap == 2 {
            !r.observations.taps[0].values.is_empty()
        } else {
            !r.observations.tx.waves[3].is_empty()
        }
    });
    json!({"has_result":available,"can_generate":false,"busy":false,
        "tap_name":index.map(|i|NAMES[i]).unwrap_or("Unavailable"),
        "result_token":format!("{:?}-{tap}",result.as_ref().map(|r|r.job.elapsed_ms)),
        "reason":if available {""} else {"Run the simulation to retain this TapPoint."}})
}
/// Calculates the exact scalar boundary from retained time samples and frozen core settings.
/// # Arguments
/// * `tap` - Channel boundary0..2.
/// * `start` - Display first bin, independent of power integration.
/// * `count` - Display width; zero means full span.
/// * `capture_axis` - Select physical frequency mapping.
/// # Returns
/// Shared SA viewport with integrated Channel Power and explicit band/units.
pub(crate) fn spectrum(tap: i32, start: i32, count: i32, capture_axis: bool) -> Value {
    let get = || -> Result<Value, String> {
        if !(0..=2).contains(&tap) {
            return Err("Invalid TapPoint.".into());
        }
        let result = latest().ok_or("Run the simulation to retain this TapPoint.")?;
        let settings = &result.job.settings;
        let input = if tap == 2 {
            &result.observations.taps[0].values
        } else {
            &result.observations.tx.waves[3]
        };
        let gain = if tap == 1 {
            10.0_f64.powf(settings.impairments.power.gain_db() / 20.0)
        } else {
            1.0
        };
        let samples: Vec<_> = input.iter().map(|p| [p[0] * gain, p[1] * gain]).collect();
        let spectrum = estimate(
            settings.transmission.subcarrier_spacing,
            settings.layout.zero_frequency_hz,
            &samples,
        )
        .map_err(|e| e.to_string())?;
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout)
            .map_err(|e| e.to_string())?;
        let mut view = channel_viewport(
            spectrum,
            start,
            count,
            capture_axis,
            NAMES[tap as usize],
            &layout,
            &settings.impairments.power,
        )?;
        if tap == 2 {
            if let Some(level) = view["channel_power"]["equivalent_dbmv_per_6mhz"].as_f64() {
                view["level_warning"] = json!(if !(-12.0..=12.0).contains(&level) {
                    "Outside −12…+12 dBmV / 6 MHz operating target (band average)."
                } else {
                    ""
                });
            }
        }
        Ok(view)
    };
    get().unwrap_or_else(|error| json!({"error":error,"points":[]}))
}
/// Supplies raw input/output cells to one shared channel QAM window.
/// # Arguments
/// * `tap` - Channel boundary0..2.
/// * `document` - Bounded role/bin selection; all coordinates remain Rust-owned.
/// # Returns
/// At most2048 actual or exact scalar-conditioned I/Q points and axis extent.
pub(crate) fn qam(tap: i32, document: &str) -> Value {
    let get = || -> Result<Value, String> {
        let query = crate::instruments::receiver::selection::Query::parse(document)?;
        if !(0..=2).contains(&tap) {
            return Err("Invalid TapPoint.".into());
        }
        let result = latest().ok_or("Run the simulation to retain this TapPoint.")?;
        if tap < 2 {
            let mut view = crate::instruments::transmitter::channel_input::qam(
                query.first_bin,
                query.last_bin,
            );
            let gain = if tap == 1 {
                10.0_f64.powf(result.job.settings.impairments.power.gain_db() / 20.0)
            } else {
                1.0
            };
            if let Some(points) = view["points"].as_array_mut() {
                let extent = points
                    .iter()
                    .flat_map(|p| {
                        [
                            p["i"].as_f64().unwrap_or(0.0) * gain,
                            p["q"].as_f64().unwrap_or(0.0) * gain,
                        ]
                    })
                    .map(f64::abs)
                    .fold(1e-9_f64, f64::max)
                    * 1.1;
                for p in points {
                    let i = p["i"].as_f64().unwrap_or(0.0) * gain;
                    let q = p["q"].as_f64().unwrap_or(0.0) * gain;
                    p["i"] = json!(i);
                    p["q"] = json!(q);
                    p["x"] = json!(0.5 + i / (2.0 * extent));
                    p["y"] = json!(0.5 - q / (2.0 * extent));
                }
                view["extent"] = json!(extent);
            }
            return Ok(view);
        }
        let selected = crate::instruments::receiver::selection::select(&result, &query)?;
        let points = selected.cells.iter().take(2048).collect::<Vec<_>>();
        let extent = points
            .iter()
            .flat_map(|p| p.raw)
            .map(|v| v.abs())
            .fold(1e-9_f64, f64::max)
            * 1.1;
        let samples:Vec<_>=points.iter().map(|p|json!({"i":p.raw[0],"q":p.raw[1],"x":0.5+p.raw[0]/(2.0*extent),"y":0.5-p.raw[1]/(2.0*extent),"index":p.bin,"symbol":p.symbol,"role":match query.signal {0=>"data",1=>"ncp",2=>"plc",_=>"pilot"}})).collect();
        let mut view = selected.metadata;
        view["points"] = json!(samples);
        view["available"] = json!(!points.is_empty());
        view["extent"] = json!(extent);
        Ok(view)
    };
    get().unwrap_or_else(|reason| json!({"available":false,"points":[],"reason":reason}))
}

/// Formats any time-domain SA route with one fixed channel band and electrical reference.
/// # Arguments
/// * `spectrum` - Owned full-resolution PSD, safe to calibrate without changing captures.
/// * `start` - First display bin.
/// * `count` - Display bin count, zero for full span.
/// * `capture_axis` - Select physical frequency labels.
/// * `detail` - Signal reference-plane name.
/// * `layout` - Frozen occupied/excluded carrier geometry.
/// * `power` - Frozen calibration, independently of plot zoom.
/// # Returns
/// Shared viewport and integrated channel reading.
/// # Errors
/// Rejects malformed geometry, calibration or display selection.
pub(crate) fn channel_viewport(
    mut spectrum: docsis_ofdm_fec::signal_processing::waveform::spectrum::PowerSpectrum,
    start: i32,
    count: i32,
    capture_axis: bool,
    detail: &str,
    layout: &CarrierLayout,
    power: &docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning,
) -> Result<Value, String> {
    let mut active = layout
        .static_roles()
        .iter()
        .enumerate()
        .filter(|(_, r)| **r != PhysicalCarrierRole::Excluded)
        .map(|(i, _)| i);
    let first = active.next().ok_or("No occupied channel band.")?;
    let last = active.last().unwrap_or(first);
    let reading = channel_power(&spectrum, first, last, power).map_err(|e| e.to_string())?;
    let power_text = reading
        .dbmv
        .map(|v| format!("Channel Power: {v:.2} dBmV"))
        .unwrap_or_else(|| {
            format!(
                "Channel Power: {:.2} dB re unit²",
                10.0 * reading.normalized_power.log10()
            )
        });
    let equivalent = reading
        .equivalent_dbmv_per_6mhz
        .map(|v| format!(" · {v:.2} dBmV / 6 MHz avg"))
        .unwrap_or_else(|| " · Relative units; set TX reference for dBmV".into());
    if let Some(scale) = power.millivolts_squared_per_unit() {
        for p in &mut spectrum.density {
            *p *= scale;
        }
    }
    let mut view = crate::spectrum_view::viewport(&spectrum, start, count, capture_axis, detail)?;
    if reading.dbmv.is_some() {
        view["axis_label"] = json!("PSD · dBmV/Hz");
        if let Some(points) = view["points"].as_array_mut() {
            for point in points {
                if let Some(text) = point["readout"].as_str() {
                    point["readout"] = json!(text.replace("dB/Hz", "dBmV/Hz"));
                }
            }
        }
    }
    view["channel_power"] = json!(reading);
    view["channel_power_text"] = json!(format!(
        "{power_text}{equivalent} · {:.3} MHz band",
        reading.bandwidth_hz / 1e6
    ));

    Ok(view)
}
