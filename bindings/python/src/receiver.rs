/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed conversion of the existing Rust receiver correction policy.
use crate::api;
use pyo3::prelude::*;
/// Receiver correction source, independent of the channel's applied response.
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    module = "merlin",
    name = "ReceiverMode"
)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    clippy::enum_variant_names,
    reason = "Keep public Python names aligned with the Rust receiver modes"
)]
pub(crate) enum PyReceiverMode {
    CmChannelEstimated,
    PilotEstimated,
    MerlinChannelEstimated,
}
impl PyReceiverMode {
    /// Converts a Python policy to the existing native enum without calculations.
    /// # Returns
    /// The same typed correction mode used by Rust and Qt.
    pub(crate) fn native(self) -> api::ReceiverMode {
        match self {
            Self::CmChannelEstimated => api::ReceiverMode::CmChannelEstimated,
            Self::PilotEstimated => api::ReceiverMode::PilotEstimated,
            Self::MerlinChannelEstimated => api::ReceiverMode::MerlinChannelEstimated,
        }
    }
}
impl From<api::ReceiverMode> for PyReceiverMode {
    /// Copies the selected native policy into an independent Python value.
    /// # Arguments
    /// * `mode` - Accepted or default Rust receiver correction mode.
    /// # Returns
    /// Matching Python policy; no simulation or state mutation occurs.
    fn from(mode: api::ReceiverMode) -> Self {
        match mode {
            api::ReceiverMode::CmChannelEstimated => Self::CmChannelEstimated,
            api::ReceiverMode::PilotEstimated => Self::PilotEstimated,
            api::ReceiverMode::MerlinChannelEstimated => Self::MerlinChannelEstimated,
        }
    }
}
/// Registers typed receiver policies with the API's uppercase alias convention.
/// # Arguments
/// * `module` - Python-owned native extension namespace.
/// # Returns
/// Unit after registering all supported existing receiver modes.
/// # Errors
/// Propagates Python allocation and type-registration errors.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyReceiverMode>()?;
    let class = module.getattr("ReceiverMode")?;
    for (name, mode) in [
        ("CM_CHANNEL_ESTIMATED", PyReceiverMode::CmChannelEstimated),
        ("SUPPLIED_CHANNEL", PyReceiverMode::CmChannelEstimated),
        ("SuppliedChannel", PyReceiverMode::CmChannelEstimated),
        ("PILOT_ESTIMATED", PyReceiverMode::PilotEstimated),
        (
            "MERLIN_CHANNEL_ESTIMATED",
            PyReceiverMode::MerlinChannelEstimated,
        ),
    ] {
        class.setattr(name, mode)?;
    }
    Ok(())
}
