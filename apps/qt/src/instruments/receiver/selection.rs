/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded immutable selection of actual matched receiver cells.
use crate::simulation::closed_loop::{latest, ResultData};
use docsis_ofdm_fec::{signal_processing::modulation::mer::MerCalculator, types::IqPair};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;

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
    fn default() -> Self {
        Self {
            run_profile: -1,
            signal: 0,
            view: 1,
            part: 0,
            profile: -1,
            order: 0,
            first_bin: -1,
            last_bin: -1,
            first_symbol: -1,
            last_symbol: -1,
        }
    }
}
impl Query {
    /// Parses one small viewport selection without accepting source arrays or paths.
    /// # Arguments
    /// * `document` - At most 4096 bytes of selector JSON.
    /// # Returns
    /// Validated selector values; -1 bounds mean all retained coordinates.
    /// # Errors
    /// Rejects unknown fields, reversed ranges or unsupported indices.
    pub(crate) fn parse(document: &str) -> Result<Self, String> {
        if document.len() > 4096 {
            return Err("Receiver selection exceeds bounds.".into());
        }
        let query: Self = serde_json::from_str(document).map_err(|e| e.to_string())?;
        if !(-1..=255).contains(&query.run_profile)
            || query.signal > 4
            || query.view > 2
            || query.part > 1
            || query.profile < -1
            || query.profile > 255
            || query.first_bin < -1
            || query.last_bin < -1
            || query.first_bin > 8191
            || query.last_bin > 8191
            || query.first_symbol < -1
            || query.last_symbol < -1
            || query.first_symbol > 1_000_000
            || query.last_symbol > 1_000_000
            || (query.first_bin >= 0 && query.last_bin >= 0 && query.first_bin > query.last_bin)
            || (query.first_symbol >= 0
                && query.last_symbol >= 0
                && query.first_symbol > query.last_symbol)
        {
            return Err("Invalid receiver selector or reversed coordinate range.".into());
        }
        Ok(query)
    }
}
pub(crate) struct Cell<'a> {
    pub raw: &'a IqPair,
    pub eq: &'a IqPair,
    pub reference: Option<IqPair>,
    pub llrs: &'a [f64],
    pub bin: u16,
    pub frequency: u32,
    pub source: Option<u64>,
    pub symbol: u64,
    pub profile: Option<u8>,
    pub order: u16,
    pub amplitude: f64,
}
pub(crate) struct Selected<'a> {
    pub cells: Vec<Cell<'a>>,
    pub metadata: Value,
}
/// Selects only retained received roles, preserving matched bin/time identities.
/// # Arguments
/// * `result` - Shared immutable job result.
/// * `query` - Validated signal and inclusive coordinate windows.
/// # Returns
/// Borrowed selected cells plus bounded available-selector metadata.
/// # Errors
/// Reports unsupported modulation/profile or no retained selected cells without stale samples.
pub(crate) fn select<'a>(result: &'a ResultData, query: &Query) -> Result<Selected<'a>, String> {
    select_streams(
        result.job.constellation.as_ref(),
        result.job.protected_constellation.as_ref(),
        query,
    )
}
fn select_streams<'a>(
    data: Option<&'a docsis_ofdm_fec::simulation::channel::constellation::ConstellationResult>,
    protected: Option<
        &'a docsis_ofdm_fec::simulation::channel::protected_constellation::ProtectedConstellation,
    >,
    query: &Query,
) -> Result<Selected<'a>, String> {
    let mut cells = Vec::new();
    if query.signal == 0 {
        if let Some(data) = data {
            cells.extend(data.pairs.iter().map(|p| Cell {
                raw: &p.before_equalization,
                eq: &p.after_equalization,
                reference: p.reference_iq,
                llrs: &p.llrs,
                bin: p.physical_bin,
                frequency: p.frequency_hz,
                source: Some(p.source_symbol_index),
                symbol: p.channel_symbol_index,
                profile: Some(p.profile_id),
                order: p.modulation_order,
                amplitude: 1.0,
            }));
        }
    } else if query.signal == 1 {
        if let Some(data) = data {
            cells.extend(data.ncp_pairs.iter().map(|p| Cell {
                raw: &p.before_equalization,
                eq: &p.after_equalization,
                reference: p.reference_iq,
                llrs: &p.llrs,
                bin: p.physical_bin,
                frequency: p.frequency_hz,
                source: Some(p.source_symbol_index),
                symbol: p.channel_symbol_index,
                profile: None,
                order: p.modulation_order,
                amplitude: 1.0,
            }));
        }
    } else if let Some(data) = protected {
        let (stream, order, amplitude) = match query.signal {
            2 if query.part == 0 => (&data.plc_data, 16, 1.0),
            2 => (&data.plc_preamble, 2, 1.0),
            3 => (&data.continuous_pilots, 2, 2.0),
            _ => (&data.scattered_pilots, 2, 2.0),
        };
        cells.extend(stream.pairs.iter().map(|p| Cell {
            raw: &p.before_equalization,
            eq: &p.after_equalization,
            reference: p.reference_iq,
            llrs: &[],
            bin: p.physical_bin,
            frequency: p.frequency_hz,
            source: None,
            symbol: p.channel_symbol_index,
            profile: None,
            order,
            amplitude,
        }));
    }
    let profiles: Vec<_> = cells
        .iter()
        .filter_map(|p| p.profile)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let orders: Vec<_> = cells
        .iter()
        .filter(|p| query.profile < 0 || p.profile == Some(query.profile as u8))
        .map(|p| p.order)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let order = if query.order == 0 {
        orders.first().copied().unwrap_or(0)
    } else {
        query.order
    };
    let metadata = json!({"profiles":profiles,"orders":orders,"selected_order":order,
        "first_bin":cells.iter().map(|p|p.bin).min(),"last_bin":cells.iter().map(|p|p.bin).max(),
        "first_symbol":cells.iter().map(|p|p.symbol).min(),"last_symbol":cells.iter().map(|p|p.symbol).max(),"retained":cells.len()});
    cells.retain(|p| {
        (query.profile < 0 || p.profile == Some(query.profile as u8))
            && p.order == order
            && (query.first_bin < 0 || i32::from(p.bin) >= query.first_bin)
            && (query.last_bin < 0 || i32::from(p.bin) <= query.last_bin)
            && (query.first_symbol < 0 || p.symbol >= query.first_symbol as u64)
            && (query.last_symbol < 0 || p.symbol <= query.last_symbol as u64)
    });
    Ok(Selected { cells, metadata })
}
/// Borrows a requested archived run; latest keeps its full capture resolution.
pub(crate) fn archive(
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
    super::qam_history::get(query.run_profile).map(Some)
}
/// Selects original coordinates from one identified run, never pooling independent runs.
pub(crate) fn select_run<'a>(
    result: &'a ResultData,
    snapshot: Option<&'a super::qam_history::Snapshot>,
    query: &Query,
) -> Result<Selected<'a>, String> {
    let mut selected = if let Some(snapshot) = snapshot {
        select_streams(snapshot.data.as_ref(), snapshot.protected.as_ref(), query)?
    } else {
        select(result, query)?
    };
    selected.metadata["runs"] = super::qam_history::catalog();
    selected.metadata["run_profile"] =
        json!(snapshot.map(|snapshot| snapshot.id).or_else(|| result
            .job
            .settings
            .playback
            .as_ref()
            .map(|playback| playback.profile.profile_id)));
    selected.metadata["latest_run_profile"] = json!(result
        .job
        .settings
        .playback
        .as_ref()
        .map(|playback| playback.profile.profile_id));
    selected.metadata["archived"] = json!(snapshot.is_some());
    Ok(selected)
}
/// Measures actual complex error against matched TX symbols, never slicer decisions.
pub(crate) fn reference_mer(selection: &Selected<'_>, raw: bool) -> Value {
    if selection.cells.is_empty() || selection.cells.iter().any(|cell| cell.reference.is_none()) {
        return json!({"text":"TX-reference MER unavailable"});
    }
    let result = docsis_ofdm_fec::signal_processing::modulation::mer::calculate_mer_pairs(
        selection.cells.iter().map(|cell| {
            (
                if raw { *cell.raw } else { *cell.eq },
                cell.reference.unwrap(),
            )
        }),
    );
    let text = if let Some(db) = result.mer_db {
        format!(
            "MER (TX reference): {db:.2} dB · {} cells",
            result.sample_count
        )
    } else if result.error_power == Some(0.0)
        && result.signal_power.is_some_and(|power| power > 0.0)
    {
        "MER (TX reference): perfect reference".into()
    } else {
        "TX-reference MER unavailable".into()
    };
    json!({"text":text,"result":result,"help":"Measured against the actual transmitted I/Q at the same physical bin and symbol. TX reference is diagnostic only and is never passed to the receiver. Uses all cells in this retained selection."})
}
/// Calculates decision-directed MER over all selected retained cells.
/// # Arguments
/// * `selection` - One modulation/role with its actual reference amplitude.
/// * `raw` - Before equalization when true, otherwise after correction.
/// # Returns
/// Typed core statistics plus presentation labels; never a display-point estimate.
pub(crate) fn mer(selection: &Selected<'_>, raw: bool) -> Value {
    let Some(first) = selection.cells.first() else {
        return json!({"text":"MER unavailable · no selected cells"});
    };
    let calculator = match MerCalculator::new(first.order, first.amplitude) {
        Ok(calculator) => calculator,
        Err(error) => return json!({"text":"MER unavailable", "reason":error.to_string()}),
    };
    let result =
        calculator.calculate(selection.cells.iter().map(
            |cell| {
                if raw {
                    *cell.raw
                } else {
                    *cell.eq
                }
            },
        ));
    let text = if let Some(db) = result.mer_db {
        format!("Nearest-point estimate: {db:.2} dB")
    } else if result.calculation_status
        == docsis_ofdm_fec::signal_processing::modulation::mer::MerStatus::PerfectReference
    {
        "Nearest-point estimate: perfect reference".to_string()
    } else {
        "MER unavailable".to_string()
    };
    json!({"text":text,"result":result,
        "method":"nearest_reference_euclidean",
        "help":"Decision-directed estimate over all selected retained cells. Severe noise may make it optimistic. No gain or phase fitting; raw values include channel distortion. This is not the imported RxMER reading."})
}
/// Queries bounded actual scatter points for a validated selection, with core MER statistics.
/// # Arguments
/// * `query` - Signal/profile/modulation and inclusive bin/symbol bounds.
/// # Returns
/// At most 2048 matched points, available bounds and explicit empty-state diagnostics.
pub(crate) fn qam(query: &Query) -> Value {
    let Some(result) = latest() else {
        return json!({"available":false,"points":[],"profiles":[],"orders":[],"runs":super::qam_history::catalog(),"reason":"Run the closed loop to retain receiver observations."});
    };
    let archived = match archive(query, &result) {
        Ok(value) => value,
        Err(reason) => {
            return json!({"available":false,"points":[],"runs":super::qam_history::catalog(),"reason":reason,"diagnostic":crate::help::static_diagnostic("QamRunUnavailable")})
        }
    };
    let selection = match select_run(&result, archived.as_deref(), query) {
        Ok(s) => s,
        Err(e) => return json!({"available":false,"points":[],"reason":e}),
    };
    let extent = selection
        .cells
        .iter()
        .take(2048)
        .flat_map(|p| p.raw.iter().chain(p.eq))
        .map(|v| v.abs())
        .fold(1.0_f64, f64::max)
        * 1.1;
    let points:Vec<_>=selection.cells.iter().take(2048).map(|p|json!({"raw_x":0.5+p.raw[0]/(2.0*extent),"raw_y":0.5-p.raw[1]/(2.0*extent),"eq_x":0.5+p.eq[0]/(2.0*extent),"eq_y":0.5-p.eq[1]/(2.0*extent),"bin":p.bin,"symbol":p.symbol,"source_symbol":p.source,"frequency_hz":p.frequency})).collect();
    let raw_mer = mer(&selection, true);
    let equalized_mer = mer(&selection, false);
    let raw_reference_mer = reference_mer(&selection, true);
    let equalized_reference_mer = reference_mer(&selection, false);
    let mut state = selection.metadata;
    state["raw_reference_mer"] = raw_reference_mer;
    state["equalized_reference_mer"] = equalized_reference_mer;
    state["raw_mer"] = raw_mer;
    state["equalized_mer"] = equalized_mer;
    state["selected"] = json!(selection.cells.len());
    state["available"] = json!(!points.is_empty());
    state["points"] = json!(points);
    state["reason"] = json!(if state["retained"] == 0 {
        "No cells retained for this received role (NCP requires a CRC-accepted field)."
    } else {
        "No retained cells match this coordinate/modulation selection."
    });
    state["detail"]=json!(format!("{} selected / {} retained cells · at most 2048 displayed · {} QAM · correction {:?} · shared axes ±{extent:.3}. Coordinates are receive-channel symbols; unretained history is unavailable.",selection.cells.len(),state["retained"],state["selected_order"],result.job.settings.receiver_mode));
    state["raw_tap"] = json!("rx.fft.bins");
    state["equalized_tap"] = json!("rx.equalized.iq");
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mer_uses_selected_cells_beyond_display_cap_and_separate_reference_planes() {
        let ideal = [1.0, 0.0];
        let noisy = [1.0, 0.1];
        let cells = (0..4096)
            .map(|index| Cell {
                raw: if index < 2048 { &ideal } else { &noisy },
                eq: &ideal,
                reference: Some(ideal),
                llrs: &[],
                bin: 320,
                frequency: 100_000_000,
                source: None,
                symbol: index,
                profile: Some(0),
                order: 2,
                amplitude: 1.0,
            })
            .collect();
        let selection = Selected {
            cells,
            metadata: json!({}),
        };
        let raw = mer(&selection, true);
        assert_eq!(raw["result"]["sample_count"], 4096);
        assert!((raw["result"]["mer_db"].as_f64().unwrap() - 23.010299956639813).abs() < 1e-9);
        assert_eq!(
            mer(&selection, false)["result"]["calculation_status"],
            "perfect_reference"
        );
        let filtered = Selected {
            cells: selection.cells.into_iter().skip(2048).collect(),
            metadata: json!({}),
        };
        assert!((mer(&filtered, true)["result"]["mer_db"].as_f64().unwrap() - 20.0).abs() < 1e-9);
        assert!(mer(
            &Selected {
                cells: vec![],
                metadata: json!({})
            },
            true
        )["text"]
            .as_str()
            .unwrap()
            .contains("no selected cells"));
    }
}
