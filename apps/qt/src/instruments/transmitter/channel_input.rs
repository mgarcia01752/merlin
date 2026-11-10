/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual pre-channel physical bins; no reconstruction from received or ideal data.
use crate::closed_loop::model::latest;
use docsis_ofdm_fec::{
    simulation::test_points::TestPointId,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
};
use serde_json::{json, Value};

/// Selects a finite bin window from the retained TX physical symbol.
/// # Arguments
/// * `first` - Inclusive physical bin, -1 means first available.
/// * `last` - Inclusive physical bin, -1 means last available.
/// # Returns
/// At most2048 actual I/Q points with original bin/frequency/symbol coordinates.
/// Missing or excluded bins remain absent; physical role labels are not guessed.
pub(crate) fn qam(first: i32, last: i32) -> Value {
    let get = || -> Result<Value, String> {
        if first < -1
            || last < -1
            || first > 8191
            || last > 8191
            || (first >= 0 && last >= 0 && first > last)
        {
            return Err("Invalid physical-bin range.".into());
        }
        let result = latest().ok_or("Run the simulation to capture channel input.")?;
        let snapshot = result
            .observations
            .tx
            .snapshots
            .get(&TestPointId::PhysicalBins)
            .ok_or("No physical TX bins retained.")?;
        let values = snapshot.data["values"]
            .as_array()
            .ok_or("No physical bin array retained.")?;
        let layout = CarrierLayout::new(
            &result.job.settings.transmission,
            &result.job.settings.layout,
        )
        .map_err(|error| error.to_string())?;
        let mut selected = Vec::new();
        for (offset, value) in values.iter().enumerate() {
            let bin = snapshot.offset + offset;
            if layout
                .static_roles()
                .get(bin)
                .is_none_or(|role| *role == PhysicalCarrierRole::Excluded)
                || (first >= 0 && bin < first as usize)
                || (last >= 0 && bin > last as usize)
            {
                continue;
            }
            let Some(i) = value.get(0).and_then(Value::as_f64) else {
                continue;
            };
            let Some(q) = value.get(1).and_then(Value::as_f64) else {
                continue;
            };
            selected.push((bin, [i, q]));
        }
        let extent = selected
            .iter()
            .flat_map(|(_, iq)| iq)
            .map(|v| v.abs())
            .fold(1.0_f64, f64::max)
            * 1.1;
        let stride = selected.len().div_ceil(2048).max(1);
        let points:Vec<_>=selected.iter().step_by(stride).map(|(bin,iq)|json!({"index":bin,"frequency_hz":layout.frequency_hz(*bin as u16),"symbol":snapshot.symbol,"i":iq[0],"q":iq[1],"x":0.5+iq[0]/(2.0*extent),"y":0.5-iq[1]/(2.0*extent),"role":"unknown"})).collect();
        Ok(
            json!({"available":!points.is_empty(),"points":points,"symbol":snapshot.symbol,"selected":selected.len(),"reason":if selected.is_empty(){"No retained active bins match this range."}else{""}}),
        )
    };
    get().unwrap_or_else(|reason| json!({"available":false,"points":[],"reason":reason}))
}
