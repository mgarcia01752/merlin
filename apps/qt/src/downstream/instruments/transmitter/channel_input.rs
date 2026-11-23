/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual pre-channel physical-bin presentation from the authoritative public observation.
use crate::downstream::{
    common::constants::{OPEN_PHYSICAL_BIN_SELECTOR, TX_PHYSICAL_SCATTER_MIN_EXTENT},
    simulation::closed_loop::latest,
};
use crate::shared::presentation::constants::{
    MAX_SCATTER_PLOT_POINTS, SCATTER_AXIS_CENTER, SCATTER_HEADROOM_RATIO,
};
use merlin::downstream::{AmplifierBoundary, ErrorCode, PhysicalCarrierRole, SubcarrierIndex};
use serde_json::{json, Value};

fn selected_bound(value: i32) -> Result<Option<u16>, String> {
    if value == OPEN_PHYSICAL_BIN_SELECTOR {
        return Ok(None);
    }
    let index = u16::try_from(value)
        .ok()
        .and_then(|value| SubcarrierIndex::new(value).ok())
        .ok_or("Invalid physical-bin range.")?;
    Ok(Some(index.value()))
}

/// Selects a finite bin window from the actual public TX physical observation.
/// # Arguments
/// * `first`, `last` - Inclusive public bin coordinates; -1 keeps that end open.
/// # Returns
/// Existing at-most-2048 point view with unchanged I/Q and original frequency/symbol
/// coordinates. Excluded/missing bins remain absent. Physical Data does not prove
/// transported payload/NCP identity, so the existing unknown role label is retained.
pub(crate) fn qam(first: i32, last: i32) -> Value {
    qam_at_boundary(first, last, AmplifierBoundary::Before)
}

/// Formats exact public physical bins at the requested amplifier-only boundary.
/// # Arguments
/// * `first`, `last` - Existing inclusive bin selection, with open ends at -1.
/// * `boundary` - Actual pre-amplifier or backend-derived post-amplifier I/Q.
/// # Returns
/// Existing bounded scatter and original coordinates; no gain calculation occurs here.
pub(crate) fn qam_at_boundary(first: i32, last: i32, boundary: AmplifierBoundary) -> Value {
    let get = || -> Result<Value, String> {
        let first = selected_bound(first)?;
        let last = selected_bound(last)?;
        if first.zip(last).is_some_and(|(first, last)| first > last) {
            return Err("Invalid physical-bin range.".into());
        }
        let result = latest().ok_or("Run the simulation to capture channel input.")?;
        let observation = result
            .backend()
            .amplifier_bins(boundary, 0, 0, 0)
            .map_err(|cause| match cause.code() {
                ErrorCode::MissingCoverage | ErrorCode::InvalidIdentifier => {
                    "No physical TX bins retained.".to_owned()
                }
                _ => cause.message().to_owned(),
            })?
            .observation;
        let selected = observation
            .bins
            .iter()
            .zip(&observation.layout.carriers)
            .filter(|(bin, carrier)| {
                carrier.role != PhysicalCarrierRole::Excluded
                    && first.is_none_or(|first| bin.physical_bin >= first)
                    && last.is_none_or(|last| bin.physical_bin <= last)
            })
            .filter_map(|(bin, carrier)| {
                bin.iq
                    .map(|iq| (bin.physical_bin, iq, carrier.frequency_hz))
            })
            .collect::<Vec<_>>();
        // Only drawing coordinates use this extent; backend samples/roles are unchanged.
        let stride = selected.len().div_ceil(MAX_SCATTER_PLOT_POINTS).max(1);
        let extent = selected
            .iter()
            .step_by(if boundary == AmplifierBoundary::After {
                stride
            } else {
                1
            })
            .flat_map(|(_, iq, _)| iq)
            .map(|value| value.abs())
            .fold(
                if boundary == AmplifierBoundary::After {
                    crate::downstream::common::constants::CHANNEL_SCATTER_MIN_EXTENT
                } else {
                    TX_PHYSICAL_SCATTER_MIN_EXTENT
                },
                f64::max,
            )
            * SCATTER_HEADROOM_RATIO;
        let span = extent + extent;
        let symbol = observation.context.channel_symbol_index;
        let points = selected.iter().step_by(stride).map(|(bin,iq,frequency)| json!({
            "index":bin,"frequency_hz":frequency,"symbol":symbol,"i":iq[0],"q":iq[1],
            "x":SCATTER_AXIS_CENTER+iq[0]/span,"y":SCATTER_AXIS_CENTER-iq[1]/span,"role":"unknown"
        })).collect::<Vec<_>>();
        let reason = match selected.is_empty() {
            true => "No retained active bins match this range.",
            false => "",
        };
        let mut view = json!({"available":!points.is_empty(),"points":points,"symbol":symbol,
            "selected":selected.len(),"reason":reason});
        if boundary == AmplifierBoundary::After {
            view["extent"] = json!(extent);
        }
        Ok(view)
    };
    get().unwrap_or_else(|reason| json!({"available":false,"points":[],"reason":reason}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_domains_and_order_reject_before_requesting_an_observation() {
        // Independent old UI boundaries; they do not derive expected limits from production.
        for (first, last) in [
            (-2, -1),
            (-1, -2),
            (8192, -1),
            (-1, 8192),
            (4, 3),
            (i32::MIN, 0),
            (0, i32::MAX),
        ] {
            let view = qam(first, last);
            assert_eq!(view["available"], false);
            assert_eq!(view["reason"], "Invalid physical-bin range.");
        }
        for (first, last) in [(-1, -1), (0, 0), (8191, 8191)] {
            let view = qam(first, last);
            assert_eq!(view["available"], false);
            assert_eq!(
                view["reason"],
                "Run the simulation to capture channel input."
            );
        }
    }
}
