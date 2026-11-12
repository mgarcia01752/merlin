/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Presentation queries over shared immutable response, likelihood and decoder results.
use super::selection::{archive, select_run, Query};
use crate::simulation::closed_loop::latest;
use docsis_ofdm_fec::{
    signal_processing::{
        modulation::slice_symbol,
        receiver::diagnostics::{llr_statistics, response_diagnostics},
    },
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
};
use serde::Deserialize;
use serde_json::{json, Value};

fn empty(reason: &str) -> Value {
    json!({"available":false,"reason":reason,"error":reason,"points":[],"x_ticks":[],"y_ticks":[]})
}
fn curve(x: &[f64], values: &[Option<f64>], axis: &str, coordinate: &str) -> Value {
    let minimum = values
        .iter()
        .flatten()
        .copied()
        .reduce(f64::min)
        .unwrap_or(0.0);
    let maximum = values
        .iter()
        .flatten()
        .copied()
        .reduce(f64::max)
        .unwrap_or(1.0);
    let pad = ((maximum - minimum) * 0.1).max(0.1);
    let lower = if axis.contains("count") {
        (minimum - pad).max(0.0)
    } else {
        minimum - pad
    };
    let upper = if axis.contains("relative to peak") {
        0.0
    } else {
        maximum + pad
    };
    let first = x.first().copied().unwrap_or(0.0);
    let span = (x.last().copied().unwrap_or(1.0) - first).max(f64::EPSILON);
    let mut points = Vec::new();
    let chunk = values.len().div_ceil(1024).max(1);
    let mut gap = true;
    for (i, group) in values.chunks(chunk).enumerate() {
        if group.iter().any(Option::is_none) {
            gap = true;
            continue;
        }
        let low = group
            .iter()
            .flatten()
            .copied()
            .reduce(f64::min)
            .unwrap_or(0.0);
        let high = group
            .iter()
            .flatten()
            .copied()
            .reduce(f64::max)
            .unwrap_or(0.0);
        let index = (i * chunk + group.len() / 2).min(x.len().saturating_sub(1));
        points.push(json!({"x":(x[index]-first)/span,"y":(upper-high)/(upper-lower),"minimum_y":(upper-low)/(upper-lower),"gap":gap,"readout":format!("{:.4} {coordinate} · {:.4}…{:.4} {axis}",x[index],low,high)}));
        gap = false;
    }
    let x_ticks: Vec<_> = (0..=4)
        .map(|i| json!({"x":i as f64/4.0,"label":format!("{:.3}",first+span*i as f64/4.0)}))
        .collect();
    let y_ticks: Vec<_> = (0..=4)
        .map(
            |i| json!({"y":i as f64/4.0,"label":format!("{:.2}",upper-(upper-lower)*i as f64/4.0)}),
        )
        .collect();
    json!({"available":!points.is_empty(),"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,"axis_label":axis,"frequency_label":coordinate})
}
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ResponseQuery {
    source: usize,
    snapshot: usize,
    view: usize,
    velocity_percent: u32,
}
impl Default for ResponseQuery {
    fn default() -> Self {
        Self {
            source: 0,
            snapshot: 0,
            view: 0,
            velocity_percent: 87,
        }
    }
}
/// Formats response H or actual estimated H-hat, keeping their reference planes distinct.
/// # Arguments
/// * `document` - Small response/source/snapshot/view/VF selection JSON.
/// # Returns
/// At most1024 min/max groups plus original snapshot metadata; no run is started.
/// # Errors
/// Rejects invalid selection or unavailable estimates, never substitutes supplied coefficients.
pub(crate) fn response(document: &str) -> Result<Value, String> {
    if document.len() > 4096 {
        return Err("Response selector exceeds bounds.".into());
    }
    let query: ResponseQuery = serde_json::from_str(document).map_err(|e| e.to_string())?;
    if query.source > 2
        || query.view > 4
        || query.snapshot > 3
        || !(1..=100).contains(&query.velocity_percent)
    {
        return Err("Invalid response selector.".into());
    }
    if query.source == 2 {
        let mut document: Value =
            serde_json::from_str(document).map_err(|error| error.to_string())?;
        document["source"] = json!(0);
        let mut applied = response(&document.to_string())?;
        document["source"] = json!(1);
        applied["comparison"] = response(&document.to_string())?;
        return Ok(applied);
    }
    let result = latest().ok_or("Run the closed loop to retain response observations.")?;
    let settings = &result.job.settings;
    let layout =
        CarrierLayout::new(&settings.transmission, &settings.layout).map_err(|e| e.to_string())?;
    let snapshots: Vec<_> = result
        .observations
        .responses
        .iter()
        .map(|(symbol, _)| *symbol)
        .collect();
    let coefficients = if query.source == 0 {
        let plan = settings
            .impairments
            .plan(&settings.transmission, &layout, settings.llr_variance)
            .map_err(|e| e.to_string())?;
        let active: Vec<_> = layout
            .static_roles()
            .iter()
            .map(|role| *role != PhysicalCarrierRole::Excluded)
            .collect();
        settings
            .channel_filter
            .prepare(
                settings.transmission.subcarrier_spacing,
                &plan.response,
                &active,
            )
            .map_err(|error| error.to_string())?
            .map_or(plan.response, |filter| filter.response().to_vec())
    } else {
        match result.observations.responses.get(query.snapshot) {
            Some((_, response)) => response.clone(),
            None => {
                let mut state=empty("No receiver H-hat snapshot. Enable MCE or pilot-estimated correction and run again; supplied H is not an estimate.");
                state["snapshots"] = json!(snapshots);
                return Ok(state);
            }
        }
    };
    let masked: Vec<_> = coefficients
        .into_iter()
        .zip(layout.static_roles())
        .map(|(p, r)| (*r != PhysicalCarrierRole::Excluded).then_some(p))
        .collect();
    let response = response_diagnostics(
        settings.transmission.subcarrier_spacing,
        &masked,
        f64::from(query.velocity_percent) / 100.0,
    )
    .map_err(|e| e.to_string())?;
    let (values, x, axis, coordinate) = if query.view < 4 {
        let values = match query.view {
            0 => response.gain_db.clone(),
            1 => response.phase_degrees.clone(),
            2 => response.real.clone(),
            _ => response.imaginary.clone(),
        };
        let x: Vec<_> = (0..masked.len())
            .map(|bin| layout.frequency_hz(bin as u16).unwrap_or(0) as f64 / 1e6)
            .collect();
        (
            values,
            x,
            [
                "Gain · dB",
                "Wrapped phase · degrees",
                "Real H · linear",
                "Imaginary H · linear",
            ][query.view],
            "Frequency (MHz)",
        )
    } else {
        (
            response
                .impulse_db_relative_peak
                .iter()
                .copied()
                .map(Some)
                .collect(),
            response.delay_microseconds.clone(),
            "Band-limited impulse power · dB relative to peak",
            "Signed delay (µs)",
        )
    };
    let mut state = curve(&x, &values, axis, coordinate);
    state["snapshots"] = json!(snapshots);
    state["summary"] = json!(format!(
        "{} · {} observed bins · correction {}",
        if query.source == 0 {
            "Applied reference H (channel input)"
        } else {
            "Receiver pilot-estimated H-hat"
        },
        response.available_bins,
        super::selection::correction_name(settings.receiver_mode)
    ));
    state["detail"]=json!(format!("Peak delay: {:.4} µs · extra path {:.2} ft · two-way spacing {:.2} ft · VF {:.2}. Band-limited estimate; not guaranteed echo protection.",response.peak_delay_microseconds,response.peak_extra_path_feet,response.peak_two_way_spacing_feet,f64::from(query.velocity_percent)/100.0));
    if let Some(reliability) = &result.job.receiver_reliability {
        state["mce"] = json!({"tracking":reliability.tracking,"raw_noise_power":reliability.raw_noise_power,"noise_text":format!("{:.3e}",reliability.raw_noise_power),"compared_pilots":reliability.compared_pilots});
    }
    state["filter"] = json!(result.job.channel_filter);
    Ok(state)
}
/// Returns exact received-cell slicing and actual post-descrambling soft values.
/// # Arguments
/// * `document` - Same bounded selection used by the QAM analyzer.
/// * `decoder_input` - Select the separately retained coding-order payload LDPC input.
/// # Returns
/// At most128 physical decisions and a bounded likelihood histogram; no FEC inputs change.
/// # Errors
/// Rejects invalid queries or unsupported/nonnumeric retained observations.
pub(crate) fn slicer(document: &str, decoder_input: bool) -> Result<Value, String> {
    let query = Query::parse(document)?;
    let result = latest().ok_or("Run the closed loop to inspect decisions and LLRs.")?;
    let archived = match archive(&query, &result) {
        Ok(snapshot) => snapshot,
        Err(reason) => {
            return Ok(
                json!({"available":false,"rows":[],"reason":reason,"diagnostic":crate::help::static_diagnostic("QamRunUnavailable")}),
            )
        }
    };
    if decoder_input {
        let values = archived
            .as_ref()
            .map_or(&result.observations.decoder_llrs, |snapshot| {
                &snapshot.decoder_llrs
            });
        let statistics = llr_statistics(values).map_err(|e| e.to_string())?;
        let x: Vec<_> = (0..statistics.histogram.len())
            .map(|i| {
                -statistics.histogram_limit
                    + 2.0 * statistics.histogram_limit * (i as f64 + 0.5)
                        / statistics.histogram.len() as f64
            })
            .collect();
        let y: Vec<_> = statistics
            .histogram
            .iter()
            .map(|v| Some(*v as f64))
            .collect();
        return Ok(
            json!({"available":!values.is_empty(),"rows":[],"llr":statistics,"histogram":curve(&x,&y,"Retained LLR count","LLR · log P(0)/P(1)"),"reason":"No payload LDPC-input LLRs retained for this run.","detail":format!("Actual rx.bits.inverse · first retained word {:?} · {} coding-order LLRs. Cell/window selectors apply to the cell view, not this separately retained codeword.",archived.as_ref().map_or(result.observations.decoder_word,|snapshot|snapshot.decoder_word),values.len())}),
        );
    }
    let selection = select_run(&result, archived.as_deref(), &query)?;
    let mut rows = Vec::new();
    for cell in selection.cells.iter().take(128) {
        let point = [cell.eq[0] / cell.amplitude, cell.eq[1] / cell.amplitude];
        let decision = slice_symbol(cell.order, point).map_err(|e| e.to_string())?;
        rows.push(json!({"bin":cell.bin,"symbol":cell.symbol,"source_symbol":cell.source,"order":cell.order,"iq":cell.eq,"label":decision.label,"reference":[decision.reference_iq[0]*cell.amplitude,decision.reference_iq[1]*cell.amplitude],"distance_squared":decision.distance_squared*cell.amplitude*cell.amplitude,"llrs":cell.llrs}));
    }
    let soft: Vec<_> = selection
        .cells
        .iter()
        .take(2048)
        .flat_map(|p| p.llrs.iter().copied())
        .take(32768)
        .collect();
    let statistics = llr_statistics(&soft).map_err(|e| e.to_string())?;
    let x: Vec<_> = (0..statistics.histogram.len())
        .map(|i| {
            -statistics.histogram_limit
                + 2.0 * statistics.histogram_limit * (i as f64 + 0.5)
                    / statistics.histogram.len() as f64
        })
        .collect();
    let y: Vec<_> = statistics
        .histogram
        .iter()
        .map(|v| Some(*v as f64))
        .collect();
    Ok(
        json!({"available":!rows.is_empty(),"rows":rows,"llr":statistics,"histogram":curve(&x,&y,"Retained LLR count","LLR · log P(0)/P(1)"),"reason":"No retained received cells match the selection.","detail":format!("{} selected cells · first128 physical nearest-symbol decisions. Cell LLRs are actual post-descrambling receiver values (first2048 cells); labels are pre-descrambling physical decisions and do not feed FEC. {}",selection.cells.len(),if soft.is_empty(){"This protected stream has no retained soft boundary; no LLRs are synthesized."}else{"Positive LLR favors zero; negative favors one."})}),
    )
}
/// Returns genuine bounded decoder counters and completed word outcomes from the shared job.
/// # Returns
/// Actual scalar totals, fixed iteration histogram and at most64 retained word records.
pub(crate) fn fec() -> Value {
    let Some(result) = latest() else {
        return empty("Run the closed loop, then open FEC Monitor.");
    };
    super::fec::snapshot(&result)
}
