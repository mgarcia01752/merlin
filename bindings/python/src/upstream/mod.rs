/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native upstream conversions over the real facade and capture-admission owners.
mod capture;
mod configuration;
mod document;
mod measurements;
mod receiver_observations;
mod reports;
mod response;
mod simulation;
use pyo3::prelude::*;

mod api {
    pub use merlin::upstream::inputs::{
        MeasuredExperiment, PreEqCapture, RxmerCapture, UcdCapture,
    };
    pub use merlin::upstream::*;
}

fn error(value: impl Into<api::ApiError>) -> PyErr {
    let value = value.into();
    Python::attach(|py| {
        let exception = crate::ApiError::new_err(value.to_string());
        let instance = exception.value(py);
        let _ = instance.setattr("code", crate::PyErrorCode::from(value.category()));
        let _ = instance.setattr("message", value.to_string());
        let _ = instance.setattr("legacy_code", value.code());
        let field = match &value {
            api::ApiError::Domain(
                api::PhyError::InvalidInput { field }
                | api::PhyError::ResourceLimit { field }
                | api::PhyError::Unsupported { field }
                | api::PhyError::Association { field },
            )
            | api::ApiError::Capture { field, .. } => Some(*field),
            _ => None,
        };
        let offset = match &value {
            api::ApiError::Capture { offset, .. } => Some(*offset),
            _ => None,
        };
        let _ = instance.setattr("field", field);
        let _ = instance.setattr("offset", offset);
        exception
    })
}

/// Registers real upstream objects inside the one existing private extension.
///
/// # Arguments
/// * `parent` - Native module; receives one private conversion namespace.
///
/// # Returns
/// Unit after registering owned wrappers and shared error categories, without work.
///
/// # Errors
/// Propagates Python allocation/registration errors; no input or engine runs.
pub(crate) fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent.py(), "merlin._native._upstream")?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("ApiError", parent.py().get_type::<crate::ApiError>())?;
    module.add("ErrorCode", parent.py().get_type::<crate::PyErrorCode>())?;
    configuration::register(&module)?;
    document::register(&module)?;
    capture::register(&module)?;
    simulation::register(&module)?;
    reports::register(&module)?;
    module.add_class::<receiver_observations::PyReceiverObservationPolicy>()?;
    measurements::register(&module)?;
    parent.add("_upstream", module)?;
    Ok(())
}
