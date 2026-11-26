/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One channel instrument provider with frozen, explicitly identified reference planes.
use crate::downstream::common::constants::{
    CHANNEL_AFTER_AMPLIFIER_TAP, CHANNEL_BEFORE_AMPLIFIER_TAP,
    CHANNEL_LEVEL_TARGET_MAX_DBMV_PER_6MHZ, CHANNEL_LEVEL_TARGET_MIN_DBMV_PER_6MHZ,
    CHANNEL_RECEIVER_INPUT_TAP, CHANNEL_SCATTER_MIN_EXTENT, MAX_INSTRUMENT_TIME_SAMPLES,
    RECEIVER_NCP_SIGNAL, RECEIVER_PAYLOAD_SIGNAL, RECEIVER_PLC_SIGNAL,
};
use crate::downstream::simulation::closed_loop::latest;
use crate::downstream::workbench::session::Context;
use crate::shared::presentation::constants::{
    MAX_SCATTER_PLOT_POINTS, MIN_SPECTRUM_DISPLAY_BINS, SCATTER_AXIS_CENTER, SCATTER_HEADROOM_RATIO,
};
use merlin::downstream::{
    AmplifierBoundary, FullLoopChannelTap, FullLoopResult, FullLoopSpectrumQuery, FullLoopStage,
};
use serde_json::{json, Value};
const NAMES: [&str; 3] = [
    "Before Amplifier",
    "After Amplifier",
    "Channel Output / RX Input",
];

/// Identifies availability without substituting another boundary or starting a run.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `tap` - Before amplifier0, after amplifier1 or receiver input2.
/// # Returns
/// Instrument state bound to the immutable completed run.
pub(crate) fn state(context: &Context, tap: i32) -> Value {
    let index = usize::try_from(tap).ok().filter(|i| *i < NAMES.len());
    let result = latest(context);
    let available = index.zip(result.as_ref()).is_some_and(|(tap, r)| {
        if tap == CHANNEL_RECEIVER_INPUT_TAP as usize {
            !r.observations.taps[0].values.is_empty()
        } else {
            // Fourth retained TX boundary is overlap-add output; this is an
            // established presentation array position, not a new engine tap.
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
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `tap` - Channel boundary0..2.
/// * `start` - Display first bin, independent of power integration.
/// * `count` - Display width; zero means full span.
/// * `capture_axis` - Select physical frequency mapping.
/// # Returns
/// Shared SA viewport with integrated Channel Power and explicit band/units.
pub(crate) fn spectrum(
    context: &Context,
    tap: i32,
    start: i32,
    count: i32,
    capture_axis: bool,
) -> Value {
    let get = || -> Result<Value, String> {
        if !(CHANNEL_BEFORE_AMPLIFIER_TAP..=CHANNEL_RECEIVER_INPUT_TAP).contains(&tap) {
            return Err("Invalid TapPoint.".into());
        }
        let result = latest(context).ok_or("Run the simulation to retain this TapPoint.")?;
        let (boundary, stage) = match tap {
            CHANNEL_BEFORE_AMPLIFIER_TAP => (
                FullLoopChannelTap::BeforeAmplifier,
                FullLoopStage::TxOverlap,
            ),
            CHANNEL_AFTER_AMPLIFIER_TAP => {
                (FullLoopChannelTap::AfterAmplifier, FullLoopStage::TxOverlap)
            }
            _ => (
                FullLoopChannelTap::ReceiverInput,
                FullLoopStage::ChannelOutput,
            ),
        };
        let mut view = observed_spectrum(
            result.backend(),
            stage,
            Some(boundary),
            start,
            count,
            capture_axis,
            NAMES[tap as usize],
        )?;
        if tap == CHANNEL_RECEIVER_INPUT_TAP {
            if let Some(level) = view["channel_power"]["equivalent_dbmv_per_6mhz"].as_f64() {
                view["level_warning"] = json!(if !(CHANNEL_LEVEL_TARGET_MIN_DBMV_PER_6MHZ
                    ..=CHANNEL_LEVEL_TARGET_MAX_DBMV_PER_6MHZ)
                    .contains(&level)
                {
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
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `tap` - Channel boundary0..2.
/// * `document` - Bounded role/bin selection; all coordinates remain Rust-owned.
/// # Returns
/// At most2048 actual or exact scalar-conditioned I/Q points and axis extent.
pub(crate) fn qam(context: &Context, tap: i32, document: &str) -> Value {
    let get = || -> Result<Value, String> {
        let query = crate::downstream::instruments::receiver::selection::Query::parse(document)?;
        if !(CHANNEL_BEFORE_AMPLIFIER_TAP..=CHANNEL_RECEIVER_INPUT_TAP).contains(&tap) {
            return Err("Invalid TapPoint.".into());
        }
        let result = latest(context).ok_or("Run the simulation to retain this TapPoint.")?;
        if tap < CHANNEL_RECEIVER_INPUT_TAP {
            return Ok(
                crate::downstream::instruments::transmitter::channel_input::qam_at_boundary(
                    context,
                    query.first_bin,
                    query.last_bin,
                    if tap == CHANNEL_AFTER_AMPLIFIER_TAP {
                        AmplifierBoundary::After
                    } else {
                        AmplifierBoundary::Before
                    },
                ),
            );
        }
        let archived =
            crate::downstream::instruments::receiver::selection::archive(context, &query, &result)?;
        let selected = crate::downstream::instruments::receiver::selection::select_run(
            context,
            &result,
            archived.as_deref(),
            &query,
        )?;
        let points = selected
            .cells
            .iter()
            .take(MAX_SCATTER_PLOT_POINTS)
            .collect::<Vec<_>>();
        let extent = points
            .iter()
            .flat_map(|p| p.before_equalization)
            .map(|v| v.abs())
            .fold(CHANNEL_SCATTER_MIN_EXTENT, f64::max)
            * SCATTER_HEADROOM_RATIO;
        let samples:Vec<_>=points.iter().map(|p|json!({"i":p.before_equalization[0],"q":p.before_equalization[1],"x":SCATTER_AXIS_CENTER+p.before_equalization[0]/(extent+extent),"y":SCATTER_AXIS_CENTER-p.before_equalization[1]/(extent+extent),"index":p.physical_bin,"symbol":p.channel_symbol_index,"role":match query.signal {RECEIVER_PAYLOAD_SIGNAL=>"data",RECEIVER_NCP_SIGNAL=>"ncp",RECEIVER_PLC_SIGNAL=>"plc",_=>"pilot"}})).collect();
        let mer = crate::downstream::instruments::receiver::selection::mer(&selected, true);
        let reference_mer =
            crate::downstream::instruments::receiver::selection::reference_mer(&selected, true);
        let mut view = selected.metadata;
        view["reference_mer"] = reference_mer;
        view["mer"] = mer;
        view["selected"] = json!(selected.selected_count);
        view["points"] = json!(samples);
        view["available"] = json!(!points.is_empty());
        view["extent"] = json!(extent);
        Ok(view)
    };
    get().unwrap_or_else(|reason| json!({"available":false,"points":[],"runs":crate::downstream::instruments::receiver::qam_history::catalog(context),"reason":reason,"diagnostic":if tap==CHANNEL_RECEIVER_INPUT_TAP && crate::downstream::instruments::receiver::selection::Query::parse(document).is_ok_and(|query|query.run_profile>=0) {crate::downstream::help::static_diagnostic("QamRunUnavailable")} else {Value::Null}}))
}

/// Formats an authoritative public time-domain PSD using the existing retention policy.
/// # Arguments
/// * `result` - Immutable actual completed or partial result.
/// * `stage` - Original time-domain observation, never raw FFT bin power.
/// * `tap` - Optional fixed amplifier/channel reference; None preserves the stage reference.
/// * `start`, `count` - Original FFT-bin viewport; zero count keeps the old full-width rule.
/// * `capture_axis` - Physical frequency labels instead of baseband offsets.
/// * `detail` - Existing presentation text for the selected reference plane.
/// # Returns
/// Bounded shared spectrum drawing with full-band power independent of zoom.
/// # Errors
/// Preserves unavailable samples, public numerical errors and invalid viewport selection.
pub(crate) fn observed_spectrum(
    result: &FullLoopResult,
    stage: FullLoopStage,
    tap: Option<FullLoopChannelTap>,
    start: i32,
    count: i32,
    capture_axis: bool,
    detail: &str,
) -> Result<Value, String> {
    let transmission = &result.inputs().transmission;
    let query = FullLoopSpectrumQuery {
        stage,
        max_averages: transmission
            .spectrum_average_capacity(MAX_INSTRUMENT_TIME_SAMPLES)
            .map_err(|error| error.to_string())?,
        ..Default::default()
    };
    let observe = |query| match tap {
        Some(tap) => result.channel_spectrum(tap, query),
        None => result.calibrated_spectrum(query),
    };
    let full = observe(query.clone())
        .map_err(|error| error.to_string())?
        .power;
    let start = usize::try_from(start).map_err(|_| "Invalid spectrum start.")?;
    let count = if count == 0 {
        full.fft_samples
    } else {
        usize::try_from(count).map_err(|_| "Invalid spectrum count.")?
    };
    if count < MIN_SPECTRUM_DISPLAY_BINS {
        return Err("Display at least two spectrum bins.".into());
    }
    let selected = if start == 0 && count == full.fft_samples {
        None
    } else {
        Some(
            observe(FullLoopSpectrumQuery {
                start,
                count,
                ..query
            })
            .map_err(|error| error.to_string())?
            .power,
        )
    };
    let layout = transmission
        .physical_layout(&result.inputs().profile, 0, 0, 0)
        .map_err(|error| error.to_string())?;
    Ok(crate::shared::presentation::spectrum::viewport(
        selected.as_ref().unwrap_or(&full),
        &full,
        &layout.layout,
        start,
        count,
        capture_axis,
        detail,
    ))
}
