/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Role-preserving bounded QAM presentation; no role inference from I/Q positions.
use super::capture::Snapshot;
use crate::downstream::common::constants::TX_CELL_VIEW_MIN_EXTENT;
use crate::shared::presentation::constants::{
    MAX_SCATTER_PLOT_POINTS, SCATTER_AXIS_CENTER, SCATTER_HEADROOM_RATIO,
};
use merlin::downstream::{
    FullLoopCellRole as TransmitCellRole, FullLoopResult, FullLoopStage as TestPointId,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Reads authoritative roles aligned with one retained mapped/interleaved snapshot.
/// # Arguments
/// * `result` - Immutable public run result supplying actual transport provenance.
/// * `id` - Selected TX test point.
/// * `snapshot` - Immutable retained data and original element offset.
/// # Returns
/// Roles parallel to retained values, or None for unavailable provenance. Absolute
/// cell-rank alignment and first-symbol transport are owned by the Rust backend.
pub(super) fn roles(
    result: &FullLoopResult,
    id: TestPointId,
    snapshot: &Snapshot,
) -> Option<Vec<Option<TransmitCellRole>>> {
    let observation = result.transmit_cells(id, 0, 0, 0).ok()?;
    if observation.element_start != snapshot.offset
        || observation.context.channel_symbol_index != snapshot.symbol
        || observation.role_source == "unavailable"
    {
        return None;
    }
    Some(observation.cells.iter().map(|cell| cell.role).collect())
}

fn role_name(role: Option<TransmitCellRole>, id: TestPointId) -> &'static str {
    match role {
        Some(TransmitCellRole::Data) => "data",
        Some(TransmitCellRole::Ncp) => "ncp",
        Some(TransmitCellRole::Filler) => "filler",
        Some(TransmitCellRole::Pilot) => "pilot",
        None if matches!(id, TestPointId::PlcIq | TestPointId::PlcPreamble) => "plc",
        None => "unknown",
    }
}

/// Reduces real I/Q points within a shared cap while retaining rare symbol roles.
/// # Arguments
/// * `values` - Original bounded snapshot values, including missing history.
/// * `offset` - Absolute first retained cell index.
/// * `roles` - Optional backend provenance parallel to the retained values.
/// * `id` - Test point, distinguishing PLC from mixed payload/control cells.
/// # Returns
/// At most2048 points, unchanged I/Q, common scale and exact retained role counts.
pub(super) fn scatter(
    values: &[Value],
    offset: usize,
    roles: Option<&[Option<TransmitCellRole>]>,
    id: TestPointId,
) -> Value {
    let mut groups: BTreeMap<&str, Vec<(usize, [f64; 2])>> = BTreeMap::new();
    let mut extent = TX_CELL_VIEW_MIN_EXTENT;
    for (index, value) in values.iter().enumerate() {
        let raw = if value.is_array() {
            value
        } else {
            &value["iq"]
        };
        let Some(iq) = raw
            .get(0)
            .and_then(Value::as_f64)
            .zip(raw.get(1).and_then(Value::as_f64))
        else {
            continue;
        };
        if !iq.0.is_finite() || !iq.1.is_finite() {
            continue;
        }
        extent = extent.max(iq.0.abs()).max(iq.1.abs());
        let role = roles.and_then(|items| items.get(index)).copied().flatten();
        groups
            .entry(role_name(role, id))
            .or_default()
            .push((offset + index, [iq.0, iq.1]));
    }
    extent *= SCATTER_HEADROOM_RATIO;
    let span = extent + extent;
    let base = MAX_SCATTER_PLOT_POINTS / groups.len().max(1);
    let mut quotas: BTreeMap<_, _> = groups
        .iter()
        .map(|(role, values)| (*role, values.len().min(base)))
        .collect();
    let mut remaining = MAX_SCATTER_PLOT_POINTS - quotas.values().sum::<usize>();
    for (role, values) in &groups {
        let quota = quotas.get_mut(role).unwrap();
        let extra = remaining.min(values.len() - *quota);
        *quota += extra;
        remaining -= extra;
    }
    let mut counts = serde_json::Map::new();
    let mut points = Vec::new();
    for (role, values) in &groups {
        counts.insert((*role).to_string(), json!(values.len()));
        let quota = quotas[role];
        for position in 0..quota {
            let (index, iq) = values[position * values.len() / quota];
            points.push(json!({"index":index,"i":iq[0],"q":iq[1],"x":SCATTER_AXIS_CENTER+iq[0]/span,"y":SCATTER_AXIS_CENTER-iq[1]/span,"role":role}));
        }
    }
    json!({"points":points,"role_counts":counts,"extent":extent,"iq_count":groups.values().map(Vec::len).sum::<usize>()})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rare_ncp_survives_reduction_and_unknown_is_never_colored_as_data() {
        let mut values = vec![json!([1.0, -1.0]); 8192];
        values[9] = Value::Null;
        let mut labels = vec![Some(TransmitCellRole::Data); values.len()];
        labels[8191] = Some(TransmitCellRole::Ncp);
        let view = scatter(&values, 0, Some(&labels), TestPointId::PayloadMapped);
        let points = view["points"].as_array().unwrap();
        assert_eq!(points.len(), 2048);
        assert_eq!(view["role_counts"]["ncp"], 1);
        assert!(points
            .iter()
            .any(|p| p["role"] == "ncp" && p["index"] == 8191));
        let unknown = scatter(&values[..2], 0, None, TestPointId::PayloadMapped);
        assert_eq!(unknown["role_counts"]["unknown"], 2);
        assert!(unknown["role_counts"].get("data").is_none());
    }

    #[test]
    fn nonzero_retained_offsets_use_parallel_backend_roles_and_preserve_missing_values() {
        // Supplied roles are already aligned by the backend; no permutation runs here.
        let values = [json!([1.0, -1.0]), Value::Null, json!([0.0, 0.0])];
        let roles = [
            Some(TransmitCellRole::Ncp),
            None,
            Some(TransmitCellRole::Data),
        ];
        let view = scatter(&values, 37, Some(&roles), TestPointId::PayloadInterleaved);
        assert_eq!(view["iq_count"], 2);
        assert_eq!(view["role_counts"]["ncp"], 1);
        assert_eq!(view["role_counts"]["data"], 1);
        let points = view["points"].as_array().unwrap();
        assert!(points
            .iter()
            .any(|point| point["index"] == 37 && point["role"] == "ncp"));
        assert!(points
            .iter()
            .any(|point| point["index"] == 39 && point["role"] == "data"));
    }
}
