/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Role-preserving bounded QAM presentation; no role inference from I/Q positions.
use super::capture::Snapshot;
use docsis_ofdm_fec::{
    signal_processing::interleaving::{InterleaverDirection, SymbolInterleaver},
    simulation::test_points::{TestPointId, TransmitCellRole},
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Aligns retained TX packing roles with a mapped/interleaved snapshot.
/// # Arguments
/// * `id` - Selected TX test point.
/// * `snapshot` - Immutable retained data and original element offset.
/// * `mapped` - First mapped source snapshot carrying the actual packing roles.
/// # Returns
/// Source-rank or transported roles; None means unavailable provenance.
/// Only the first aligned source symbol is transported; later history is not guessed.
pub(super) fn roles(
    id: TestPointId,
    snapshot: &Snapshot,
    mapped: Option<&Snapshot>,
) -> Option<Vec<Option<TransmitCellRole>>> {
    let source = mapped?;
    let grid = source.grid?;
    let values = source.roles.as_ref()?;
    if id == TestPointId::PayloadMapped {
        return Some(values.iter().copied().map(Some).collect());
    }
    if id != TestPointId::PayloadInterleaved
        || source.offset != 0
        || source.source_symbol != Some(grid.origin_symbol)
        || snapshot.symbol != Some(grid.origin_symbol)
        || snapshot.source_symbol != source.source_symbol
        || snapshot.grid != source.grid
        || values.len() != grid.settings.cells()
    {
        return None;
    }
    SymbolInterleaver::new(grid.settings, InterleaverDirection::Transmit)
        .ok()?
        .process_symbol(0, &values.iter().copied().map(Some).collect::<Vec<_>>())
        .ok()
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
/// * `roles` - Optional full-grid provenance, indexed by absolute source/channel rank.
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
    let mut extent = 1.0f64;
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
        let role = roles
            .and_then(|items| items.get(offset + index))
            .copied()
            .flatten();
        groups
            .entry(role_name(role, id))
            .or_default()
            .push((offset + index, [iq.0, iq.1]));
    }
    extent *= 1.1;
    let base = 2048 / groups.len().max(1);
    let mut quotas: BTreeMap<_, _> = groups
        .iter()
        .map(|(role, values)| (*role, values.len().min(base)))
        .collect();
    let mut remaining = 2048 - quotas.values().sum::<usize>();
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
            points.push(json!({"index":index,"i":iq[0],"q":iq[1],"x":0.5+iq[0]/(2.0*extent),"y":0.5-iq[1]/(2.0*extent),"role":role}));
        }
    }
    json!({"points":points,"role_counts":counts,"extent":extent,"iq_count":groups.values().map(Vec::len).sum::<usize>()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::{
        signal_processing::interleaving::InterleaverSettings,
        simulation::test_points::CellProbeGrid,
        transmission::{packing::NcpModulation, SubcarrierSpacing},
    };

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
    fn roles_follow_actual_interleaver_and_unavailable_history_stays_unknown() {
        let settings = InterleaverSettings::new(SubcarrierSpacing::Khz50, 128, 4).unwrap();
        let grid = CellProbeGrid {
            settings,
            origin_symbol: 128,
            ncp_modulation: NcpModulation::Qpsk,
        };
        let mut labels = vec![TransmitCellRole::Data; 128];
        labels[0] = TransmitCellRole::Ncp;
        labels[4] = TransmitCellRole::Filler;
        let mut snapshot = Snapshot {
            data: Value::Null,
            total: 128,
            offset: 0,
            symbol: Some(128),
            source_symbol: Some(128),
            phase: "Observation".into(),
            grid: Some(grid),
            roles: Some(labels.clone()),
        };
        let actual = roles(TestPointId::PayloadInterleaved, &snapshot, Some(&snapshot)).unwrap();
        let iq: Vec<_> = labels
            .iter()
            .enumerate()
            .map(|(index, role)| Some((*role, index)))
            .collect();
        let expected = SymbolInterleaver::new(settings, InterleaverDirection::Transmit)
            .unwrap()
            .process_symbol(0, &iq)
            .unwrap();
        for (role, cell) in actual.iter().zip(expected) {
            assert_eq!(*role, cell.map(|(role, _)| role));
        }
        assert_eq!(
            actual
                .iter()
                .filter(|v| **v == Some(TransmitCellRole::Ncp))
                .count(),
            1
        );
        assert_eq!(actual.iter().filter(|v| v.is_none()).count(), 96);
        snapshot.symbol = Some(129);
        assert!(roles(TestPointId::PayloadInterleaved, &snapshot, Some(&snapshot)).is_none());
    }
}
