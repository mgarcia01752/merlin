/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned Python conversion of public physical layout queries; no placement or power math.
use crate::{
    admission::PyTransmission, api, error, inputs::PyDpd, object_from_json,
    profile_capture::PyProfileCapture,
};
use pyo3::prelude::*;

#[pymethods]
impl PyTransmission {
    /// Copies authoritative configured physical roles using Rust's original-grid admission.
    /// # Arguments
    /// * `profile` - Accepted original immutable DPD.
    /// * `channel_symbol`, `start`, `count` - Original symbol and physical bin window.
    /// # Returns
    /// Owned geometry dictionary; no source, RNG or transmitter advances.
    /// # Errors
    /// Shared grid/layout/window/budget errors raise ApiError.
    #[pyo3(signature=(profile,channel_symbol=0,start=0,count=0))]
    fn physical_layout(
        &self,
        py: Python<'_>,
        profile: &PyDpd,
        channel_symbol: u64,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self
                    .0
                    .physical_layout(&profile.0, channel_symbol, start, count)?;
                serde_json::to_string(&result).map_err(crate::transmitter::transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Inspects an original byte-sized catalog identity without widening legacy run IDs.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable source catalog and encoded identity.
    /// * `channel_symbol`, `start`, `count` - Original symbol and physical bin window.
    /// # Returns
    /// Owned configured geometry; inspection does not establish full-loop support.
    /// # Errors
    /// Shared identity/grid/layout/window/budget errors raise ApiError.
    #[pyo3(signature=(capture,profile_id,channel_symbol=0,start=0,count=0))]
    fn physical_layout_capture(
        &self,
        py: Python<'_>,
        capture: &PyProfileCapture,
        profile_id: u8,
        channel_symbol: u64,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.physical_layout_capture(
                    &capture.0,
                    profile_id,
                    channel_symbol,
                    start,
                    count,
                )?;
                serde_json::to_string(&result).map_err(crate::transmitter::transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

/// Converts a bounded stage identifier without scientific preparation.
/// # Arguments
/// * `name` - Existing registered stage string, at most 128 UTF-8 bytes.
/// # Returns
/// Typed public stage identity for a subsequent Rust query.
/// # Errors
/// Unknown or oversized identities raise the existing structured ApiError.
pub(crate) fn stage_id(name: &str) -> PyResult<api::FullLoopStage> {
    if name.len() > 128 {
        return Err(error(api::ApiError::new(
            api::ErrorCode::ResourceLimit,
            "Stage ID exceeds 128 bytes",
        )));
    }
    serde_json::from_value(serde_json::Value::String(name.into())).map_err(|cause| {
        error(api::ApiError::new(
            api::ErrorCode::InvalidIdentifier,
            cause.to_string(),
        ))
    })
}
