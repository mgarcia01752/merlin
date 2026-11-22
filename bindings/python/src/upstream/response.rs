/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared checked native selection for the two public configured-response consumers.
use super::{api, error};
use pyo3::prelude::*;

/// Converts native source names and original coordinates into one public Rust request.
/// # Arguments
/// * `source` - cm_pre_eq, physical_channel or composite; never a received-estimate alias.
/// * `grant_symbol` - Original base-grant symbol, without source or replay advancement.
/// * `first_carrier`, `carrier_count` - Original physical viewport; zero count means remaining.
/// * `maximum_points` - Positive frequency-point budget, validated by the Rust owner.
/// * `include_impulse` - Explicit complete periodic diagnostic opt-in.
/// # Returns
/// Owned typed request with no scientific calculations or session effects.
/// # Errors
/// Unknown source names raise the shared InvalidInput category; domain checks remain in Rust.
pub(super) fn request(
    source: &str,
    grant_symbol: usize,
    first_carrier: usize,
    carrier_count: usize,
    maximum_points: usize,
    include_impulse: bool,
) -> PyResult<api::ConfiguredResponseRequest> {
    let source = match source {
        "cm_pre_eq" => api::ConfiguredResponseSource::CmPreEq,
        "physical_channel" => api::ConfiguredResponseSource::PhysicalChannel,
        "composite" => api::ConfiguredResponseSource::Composite,
        _ => {
            return Err(error(api::PhyError::InvalidInput {
                field: "configured_response.source",
            }))
        }
    };
    Ok(api::ConfiguredResponseRequest {
        source,
        grant_symbol,
        first_carrier,
        carrier_count,
        maximum_points,
        include_impulse,
    })
}
