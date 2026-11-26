/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Legacy selector transport and chart labels over authoritative live/archive public queries.
use crate::downstream::common::constants::*;
use crate::downstream::simulation::closed_loop::{latest, ResultData};
use crate::downstream::workbench::session::Context;
use crate::shared::presentation::constants::{
    MAX_SCATTER_PLOT_POINTS, SCATTER_AXIS_CENTER, SCATTER_HEADROOM_RATIO,
};
use merlin::downstream::{
    FullLoopCell, FullLoopCellQuery, FullLoopCellStream, FullLoopQamView, ReceiverMode,
    SubcarrierIndex,
};
use serde::Deserialize;
use serde_json::{json, Value};

/// Names the actual backend correction mode without deriving receiver behavior.
/// # Arguments
/// * `mode` - Recorded public correction mode for this independent run.
/// # Returns
/// Existing short user-facing label; no source or simulation state changes.
pub(crate) fn correction_name(mode: ReceiverMode) -> &'static str {
    match mode {
        ReceiverMode::SuppliedChannel => "Supplied CM channel estimation (CMCE)",
        ReceiverMode::PilotEstimated => "Pilot estimation (PE)",
        ReceiverMode::MerlinChannelEstimated => "MERLIN Channel Estimation (MCE)",
    }
}
#[derive(Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Query {
    pub run_profile: i32,
    pub signal: usize,
    pub view: usize,
    pub part: usize,
    pub profile: i32,
    pub order: u16,
    pub first_bin: i32,
    pub last_bin: i32,
    pub first_symbol: i64,
    pub last_symbol: i64,
}
impl Default for Query {
    /// Preserves existing latest-run, payload, equalized and open-coordinate widget selections.
    /// # Returns
    /// Passive application transport defaults; no scientific query is executed.
    fn default() -> Self {
        Self {
            run_profile: LATEST_RECEIVER_RUN_SELECTOR,
            signal: RECEIVER_PAYLOAD_SIGNAL,
            view: RECEIVER_EQUALIZED_VIEW,
            part: RECEIVER_PLC_DATA_PART,
            profile: ANY_RECEIVER_WIRE_PROFILE_SELECTOR,
            order: AUTO_RECEIVER_ORDER_SELECTOR,
            first_bin: OPEN_PHYSICAL_BIN_SELECTOR,
            last_bin: OPEN_PHYSICAL_BIN_SELECTOR,
            first_symbol: OPEN_RECEIVER_SYMBOL_SELECTOR,
            last_symbol: OPEN_RECEIVER_SYMBOL_SELECTOR,
        }
    }
}
fn valid_bin(value: i32) -> bool {
    value == OPEN_PHYSICAL_BIN_SELECTOR
        || value
            .try_into()
            .ok()
            .and_then(|index| SubcarrierIndex::new(index).ok())
            .is_some()
}
impl Query {
    /// Parses bounded legacy widget selectors while preserving their independent UI policies.
    /// # Arguments
    /// * `document` - At most4096 bytes, with known fields and original physical/channel-symbol coordinates.
    /// # Returns
    /// Passive selectors; physical-bin admission reuses the public typed domain owner.
    /// # Errors
    /// Rejects unknown fields, transport limits, unsupported UI indices or reversed ranges.
    pub(crate) fn parse(document: &str) -> Result<Self, String> {
        if document.len() > MAX_RECEIVER_SELECTION_DOCUMENT_BYTES {
            return Err("Receiver selection exceeds bounds.".into());
        }
        let query: Self = serde_json::from_str(document).map_err(|cause| cause.to_string())?;
        query.validate()?;
        Ok(query)
    }

    fn validate(&self) -> Result<(), String> {
        let query = self;
        if !(LATEST_RECEIVER_RUN_SELECTOR..=MAX_RECEIVER_SOURCE_PROFILE_SELECTOR)
            .contains(&query.run_profile)
            || query.signal > RECEIVER_SCATTERED_PILOT_SIGNAL
            || query.view > RECEIVER_PAIRED_VIEW
            || query.part > RECEIVER_PLC_PREAMBLE_PART
            || !(ANY_RECEIVER_WIRE_PROFILE_SELECTOR..=MAX_RECEIVER_WIRE_PROFILE_SELECTOR)
                .contains(&query.profile)
            || !valid_bin(query.first_bin)
            || !valid_bin(query.last_bin)
            || !(OPEN_RECEIVER_SYMBOL_SELECTOR..=MAX_RECEIVER_SELECTION_SYMBOL_INDEX)
                .contains(&query.first_symbol)
            || !(OPEN_RECEIVER_SYMBOL_SELECTOR..=MAX_RECEIVER_SELECTION_SYMBOL_INDEX)
                .contains(&query.last_symbol)
            || (query.first_bin >= 0 && query.last_bin >= 0 && query.first_bin > query.last_bin)
            || (query.first_symbol >= 0
                && query.last_symbol >= 0
                && query.first_symbol > query.last_symbol)
        {
            return Err("Invalid receiver selector or reversed coordinate range.".into());
        }
        Ok(())
    }

    /// Translates widget fields into the existing strict public cell query.
    /// # Arguments
    /// * `count` - Public operation row request; zero uses that operation's existing bound.
    /// # Returns
    /// Owned backend-admitted actual-role/coordinate selection, with no local filtering.
    /// # Errors
    /// Preserves public order, coordinate, work and transport admission failures.
    pub(crate) fn backend(&self, count: usize) -> Result<FullLoopCellQuery, String> {
        self.validate()?;
        let stream = match self.signal {
            RECEIVER_PAYLOAD_SIGNAL => FullLoopCellStream::Payload,
            RECEIVER_NCP_SIGNAL => FullLoopCellStream::Ncp,
            RECEIVER_PLC_SIGNAL if self.part == RECEIVER_PLC_DATA_PART => {
                FullLoopCellStream::PlcData
            }
            RECEIVER_PLC_SIGNAL => FullLoopCellStream::PlcPreamble,
            RECEIVER_CONTINUOUS_PILOT_SIGNAL => FullLoopCellStream::ContinuousPilots,
            RECEIVER_SCATTERED_PILOT_SIGNAL => FullLoopCellStream::ScatteredPilots,
            _ => return Err("Invalid received signal selection.".into()),
        };
        let query = FullLoopCellQuery {
            stream,
            profile_id: u8::try_from(self.profile).ok(),
            modulation_order: (self.order != AUTO_RECEIVER_ORDER_SELECTOR).then_some(self.order),
            first_bin: u16::try_from(self.first_bin).ok(),
            last_bin: u16::try_from(self.last_bin).ok(),
            first_symbol: u64::try_from(self.first_symbol).ok(),
            last_symbol: u64::try_from(self.last_symbol).ok(),
            start: 0,
            count,
        };
        let bytes = serde_json::to_vec(&query).map_err(|cause| cause.to_string())?;
        FullLoopCellQuery::from_json(&bytes).map_err(|cause| cause.to_string())
    }
}
/// Bounded public observation plus existing rendering metadata; metrics are already computed.
pub(crate) struct Selected {
    pub cells: Vec<FullLoopCell>,
    pub metadata: Value,
    pub selected_count: usize,
    raw_mer: Value,
    equalized_mer: Value,
    raw_reference_mer: Value,
    equalized_reference_mer: Value,
}
/// Resolves explicit run routing; the current run retains its full public capture resolution.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `query`, `result` - Application run selector and current immutable result.
/// # Returns
/// None for latest/current; otherwise exactly the requested detached history.
/// # Errors
/// Missing/unstarted runs reject without silently substituting the latest observations.
pub(crate) fn archive(
    context: &Context,
    query: &Query,
    result: &ResultData,
) -> Result<Option<std::sync::Arc<super::qam_history::Snapshot>>, String> {
    let latest_id = result
        .job
        .settings
        .playback
        .as_ref()
        .map(|playback| i32::from(playback.profile.profile_id));
    if query.run_profile < 0 || latest_id == Some(query.run_profile) {
        return Ok(None);
    }
    super::qam_history::get(context, query.run_profile).map(Some)
}
/// Queries only the current public receiver observations.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `result`, `query` - Immutable result and passive widget selection.
/// # Returns
/// At most2048 original cells with authoritative all-selected metrics/metadata.
/// # Errors
/// Preserves public selector/work/numerical failures.
pub(crate) fn select(
    context: &Context,
    result: &ResultData,
    query: &Query,
) -> Result<Selected, String> {
    let observation = result
        .backend()
        .qam(query.backend(0)?)
        .map_err(|cause| cause.to_string())?;
    present(context, observation, result, None)
}
/// Queries one actual current or archived run through its accepted public API owner.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `result`, `snapshot`, `query` - Current result, optional exact history, and widget selection.
/// # Returns
/// Bounded original cells, unchanged all-selected metrics and existing labels/availability.
/// # Errors
/// Preserves public query errors; independent runs are never pooled.
pub(crate) fn select_run(
    context: &Context,
    result: &ResultData,
    snapshot: Option<&super::qam_history::Snapshot>,
    query: &Query,
) -> Result<Selected, String> {
    if snapshot.is_none() {
        return select(context, result, query);
    }
    let request = query.backend(0)?;
    let observation = match snapshot {
        Some(snapshot) => snapshot.backend.qam(request),
        None => result.backend().qam(request),
    }
    .map_err(|cause| cause.to_string())?;
    present(context, observation, result, snapshot)
}
fn present(
    context: &Context,
    observation: FullLoopQamView,
    result: &ResultData,
    snapshot: Option<&super::qam_history::Snapshot>,
) -> Result<Selected, String> {
    let selection = &observation.selection;
    let bounds = selection.bounds.as_ref();
    let mode = observation.receiver_mode;
    let (short,help)=match mode {
        ReceiverMode::SuppliedChannel => ("CMCE","The recorded run corrects received samples using the supplied channel/model response. Without a CM capture, correction uses the configured model response."),
        ReceiverMode::PilotEstimated => ("PE","The recorded run estimates channel correction from received pilots. The supplied CM capture impairs the channel; it is not the receiver correction. Decoder confidence uses calibrated noise information."),
        ReceiverMode::MerlinChannelEstimated => ("MCE","The recorded run estimates channel correction and decoder noise confidence from received pilots. The supplied CM capture impairs the channel; it is not the receiver correction."),
    };
    let latest_id = result
        .job
        .settings
        .playback
        .as_ref()
        .map(|playback| playback.profile.profile_id);
    let metadata = json!({"profiles":selection.available_profiles,"orders":selection.available_orders,"selected_order":selection.selected_order.unwrap_or(0),
        "first_bin":bounds.map(|value|value.first_bin),"last_bin":bounds.map(|value|value.last_bin),
        "first_symbol":bounds.map(|value|value.first_symbol),"last_symbol":bounds.map(|value|value.last_symbol),"retained":selection.retained_cells,
        "runs":super::qam_history::catalog(context),"run_profile":snapshot.map(|snapshot|snapshot.id).or(latest_id),"latest_run_profile":latest_id,"archived":snapshot.is_some(),
        "correction_name":correction_name(mode),"correction_mode":short,"correction_help":help});
    let serialize = |metric| serde_json::to_value(metric).map_err(|cause| cause.to_string());
    let raw_mer = format_metric(&serialize(observation.raw_nearest_mer)?, false);
    let equalized_mer = format_metric(&serialize(observation.equalized_nearest_mer)?, false);
    let raw_reference_mer = format_metric(
        &serde_json::to_value(observation.raw_tx_reference_mer)
            .map_err(|cause| cause.to_string())?,
        true,
    );
    let equalized_reference_mer = format_metric(
        &serde_json::to_value(observation.equalized_tx_reference_mer)
            .map_err(|cause| cause.to_string())?,
        true,
    );
    Ok(Selected {
        selected_count: selection.selected_cells,
        cells: observation.cells,
        metadata,
        raw_mer,
        equalized_mer,
        raw_reference_mer,
        equalized_reference_mer,
    })
}
fn format_metric(result: &Value, reference: bool) -> Value {
    if result.is_null() {
        return json!({"text":"TX-reference MER unavailable"});
    }
    let perfect = result["calculation_status"] == "perfect_reference";
    let text = if reference {
        if let Some(db) = result["mer_db"].as_f64() {
            format!(
                "MER (TX reference): {db:.2} dB · {} cells",
                result["sample_count"]
            )
        } else if perfect {
            "MER (TX reference): perfect reference".into()
        } else {
            "TX-reference MER unavailable".into()
        }
    } else if let Some(db) = result["mer_db"].as_f64() {
        format!("Nearest-point estimate: {db:.2} dB")
    } else if perfect {
        "Nearest-point estimate: perfect reference".into()
    } else if result["sample_count"] == 0 {
        "MER unavailable · no selected cells".into()
    } else {
        "MER unavailable".into()
    };
    if reference {
        json!({"text":text,"result":result,"help":"Measured against the actual transmitted I/Q at the same physical bin and symbol. TX reference is diagnostic only and is never passed to the receiver. Uses all cells in this retained selection."})
    } else {
        json!({"text":text,"result":result,"method":"nearest_reference_euclidean","help":"Decision-directed estimate over all selected retained cells. Severe noise may make it optimistic. No gain or phase fitting; raw values include channel distortion. This is not the imported RxMER reading."})
    }
}
/// Formats authoritative matched-TX MER independently of nearest-point diagnostics.
/// # Arguments
/// * `selection`, `raw` - Public result projection and before/after-correction display selection.
/// # Returns
/// Existing label/help plus unchanged backend metric; no sample arithmetic occurs.
pub(crate) fn reference_mer(selection: &Selected, raw: bool) -> Value {
    if raw {
        selection.raw_reference_mer.clone()
    } else {
        selection.equalized_reference_mer.clone()
    }
}
/// Formats authoritative nearest-point MER without inspecting or remeasuring cells.
/// # Arguments
/// * `selection`, `raw` - Public result projection and before/after-correction display selection.
/// # Returns
/// Existing label/help and the unchanged all-selected backend metric.
pub(crate) fn mer(selection: &Selected, raw: bool) -> Value {
    if raw {
        selection.raw_mer.clone()
    } else {
        selection.equalized_mer.clone()
    }
}
/// Formats bounded actual scatter points and original all-selected scientific readings.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `query` - Passive signal/run/profile/coordinate selection.
/// # Returns
/// Existing chart JSON; invalid/missing observations stay explicitly unavailable.
pub(crate) fn qam(context: &Context, query: &Query) -> Value {
    let Some(result) = latest(context) else {
        return json!({"available":false,"points":[],"profiles":[],"orders":[],"runs":super::qam_history::catalog(context),"reason":"Run the closed loop to retain receiver observations."});
    };
    let archived = match archive(context, query, &result) {
        Ok(value) => value,
        Err(reason) => {
            return json!({"available":false,"points":[],"runs":super::qam_history::catalog(context),"reason":reason,"diagnostic":crate::downstream::help::static_diagnostic("QamRunUnavailable")})
        }
    };
    let selection = match select_run(context, &result, archived.as_deref(), query) {
        Ok(value) => value,
        Err(reason) => return json!({"available":false,"points":[],"reason":reason}),
    };
    let extent = selection
        .cells
        .iter()
        .take(MAX_SCATTER_PLOT_POINTS)
        .flat_map(|cell| {
            cell.before_equalization
                .iter()
                .chain(&cell.after_equalization)
        })
        .map(|value| value.abs())
        .fold(RECEIVER_SCATTER_MIN_EXTENT, f64::max)
        * SCATTER_HEADROOM_RATIO;
    let points:Vec<_>=selection.cells.iter().take(MAX_SCATTER_PLOT_POINTS).map(|cell|json!({"raw_x":SCATTER_AXIS_CENTER+cell.before_equalization[0]/(extent+extent),"raw_y":SCATTER_AXIS_CENTER-cell.before_equalization[1]/(extent+extent),"eq_x":SCATTER_AXIS_CENTER+cell.after_equalization[0]/(extent+extent),"eq_y":SCATTER_AXIS_CENTER-cell.after_equalization[1]/(extent+extent),"bin":cell.physical_bin,"symbol":cell.channel_symbol_index,"source_symbol":cell.source_symbol_index,"frequency_hz":cell.frequency_hz})).collect();
    let raw_mer = mer(&selection, true);
    let equalized_mer = mer(&selection, false);
    let raw_reference_mer = reference_mer(&selection, true);
    let equalized_reference_mer = reference_mer(&selection, false);
    let mut state = selection.metadata;
    state["raw_reference_mer"] = raw_reference_mer;
    state["equalized_reference_mer"] = equalized_reference_mer;
    state["raw_mer"] = raw_mer;
    state["equalized_mer"] = equalized_mer;
    state["selected"] = json!(selection.selected_count);
    state["available"] = json!(!points.is_empty());
    state["points"] = json!(points);
    state["reason"] = json!(if state["retained"] == 0 {
        "No cells retained for this received role (NCP requires a CRC-accepted field)."
    } else {
        "No retained cells match this coordinate/modulation selection."
    });
    state["detail"]=json!(format!("{} selected / {} retained cells · at most 2048 displayed · {} QAM · correction {} · shared axes ±{extent:.3}. Coordinates are receive-channel symbols; unretained history is unavailable.",selection.selected_count,state["retained"],state["selected_order"],state["correction_name"].as_str().unwrap_or("Unavailable")));
    state["raw_tap"] = json!("rx.fft.bins");
    state["equalized_tap"] = json!("rx.equalized.iq");
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selector_translation_preserves_open_bounds_raw_ids_and_domain_rejections() {
        let query = Query::default().backend(0).unwrap();
        assert!(query.first_bin.is_none() && query.first_symbol.is_none());
        let selected = Query {
            run_profile: 200,
            profile: 255,
            order: 128,
            first_bin: 356,
            last_bin: 7835,
            ..Default::default()
        }
        .backend(0)
        .unwrap();
        assert_eq!(selected.profile_id, Some(255));
        assert_eq!(selected.first_bin, Some(356));
        // Independent old transport and physical-domain boundary anchors.
        for source in [
            r#"{"first_bin":8192}"#,
            r#"{"run_profile":256}"#,
            r#"{"first_symbol":1000001}"#,
            r#"{"first_bin":4,"last_bin":3}"#,
        ] {
            assert!(Query::parse(source).is_err());
        }
        assert!(Query {
            profile: -2,
            ..Default::default()
        }
        .backend(0)
        .is_err());
        let plc = Query {
            signal: 2,
            part: 1,
            ..Default::default()
        }
        .backend(0)
        .unwrap();
        assert_eq!(serde_json::to_value(plc).unwrap()["stream"], "plc_preamble");
    }
    #[test]
    fn mer_uses_selected_cells_beyond_display_cap_and_separate_reference_planes() {
        // The original4096-cell ideal/noisy vector and actual23.010299956639813/20-dB
        // calculations live in the public archive/live-query owner tests. This adapter
        // preserves those independent numbers, full counts and perfect/empty labels.
        let raw = format_metric(
            &json!({"sample_count":4096,"mer_db":23.010299956639813,"calculation_status":"available"}),
            false,
        );
        assert_eq!(raw["result"]["sample_count"], 4096);
        assert!((raw["result"]["mer_db"].as_f64().unwrap() - 23.010299956639813).abs() < 1e-9);
        assert_eq!(raw["text"], "Nearest-point estimate: 23.01 dB");
        assert_eq!(
            format_metric(
                &json!({"sample_count":4096,"mer_db":null,"calculation_status":"perfect_reference"}),
                false
            )["result"]["calculation_status"],
            "perfect_reference"
        );
        let filtered = format_metric(
            &json!({"sample_count":2048,"mer_db":20.0,"calculation_status":"available"}),
            false,
        );
        assert!((filtered["result"]["mer_db"].as_f64().unwrap() - 20.0).abs() < 1e-9);
        assert!(format_metric(
            &json!({"sample_count":0,"mer_db":null,"calculation_status":"no_samples"}),
            false
        )["text"]
            .as_str()
            .unwrap()
            .contains("no selected cells"));
    }
}
