/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded immutable selection of actual matched receiver cells.
use crate::closed_loop::model::{latest, ResultData};
use docsis_ofdm_fec::types::IqPair;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Query {
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
        if query.signal > 4
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
    let mut cells = Vec::new();
    if query.signal == 0 {
        if let Some(data) = &result.job.constellation {
            cells.extend(data.pairs.iter().map(|p| Cell {
                raw: &p.before_equalization,
                eq: &p.after_equalization,
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
        if let Some(data) = &result.job.constellation {
            cells.extend(data.ncp_pairs.iter().map(|p| Cell {
                raw: &p.before_equalization,
                eq: &p.after_equalization,
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
    } else if let Some(data) = &result.job.protected_constellation {
        let (stream, order, amplitude) = match query.signal {
            2 if query.part == 0 => (&data.plc_data, 16, 1.0),
            2 => (&data.plc_preamble, 2, 1.0),
            3 => (&data.continuous_pilots, 2, 2.0),
            _ => (&data.scattered_pilots, 2, 2.0),
        };
        cells.extend(stream.pairs.iter().map(|p| Cell {
            raw: &p.before_equalization,
            eq: &p.after_equalization,
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
/// Queries bounded actual scatter points for a validated selection, without running science.
/// # Arguments
/// * `query` - Signal/profile/modulation and inclusive bin/symbol bounds.
/// # Returns
/// At most 2048 matched points, available bounds and explicit empty-state diagnostics.
pub(crate) fn qam(query: &Query) -> Value {
    let Some(result) = latest() else {
        return json!({"available":false,"points":[],"profiles":[],"orders":[],"reason":"Run the closed loop to retain receiver observations."});
    };
    let selection = match select(&result, query) {
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
    let mut state = selection.metadata;
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
