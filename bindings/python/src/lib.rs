/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Optional PyO3 conversions only; the high-level Rust API owns all science.
mod admission;
mod channel;
mod constants;
mod controller;
mod designer;
mod full_loop;
mod inputs;
mod measurement;
mod options;
mod physical;
mod profile_capture;
mod project;
mod quantities;
mod receiver;
mod references;
mod reports;
mod retention;
mod rxmer_export;
mod sequence;
mod transmitter;
mod workbench;
use merlin::downstream as api;
mod upstream;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
pyo3::create_exception!(
    merlin,
    ApiError,
    PyException,
    "Structured native API failure. Read code (ErrorCode) and message (str); do not parse text for categories. Run outcomes and partial science remain in report status."
);
/// Maps a downstream public failure into the existing structured native exception.
///
/// # Arguments
/// * `error` - Owned public failure with its category, message and optional original cause.
///
/// # Returns
/// Exception preserving category/message and copying the optional original engine code.
/// Source codes come from the typed Rust getter, without parsing display text.
pub(crate) fn error(error: api::ApiError) -> PyErr {
    Python::attach(|py| {
        let exception = ApiError::new_err(error.message().to_owned());
        let _ = exception
            .value(py)
            .setattr("code", PyErrorCode::from(error.code()));
        let _ = exception.value(py).setattr("message", error.message());
        let _ = exception
            .value(py)
            .setattr("engine_code", error.engine_code());
        exception
    })
}
/// Read object from json from the native Rust snapshot.
/// # Returns
/// The immutable native value named by this read-only property.
/// # Errors
/// Preserves structured Rust validation errors and Python conversion failures.
pub(crate) fn object_from_json(py: Python<'_>, json: &str) -> PyResult<Py<PyAny>> {
    Ok(py.import("json")?.call_method1("loads", (json,))?.unbind())
}
/// Registers native conversion types; importing this module runs no simulation.
/// # Arguments
/// * `module` - Python-owned extension namespace.
/// # Returns
/// Registered types and a version string.
/// # Errors
/// Propagates Python allocation/registration failures.
#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyErrorCode>()?;
    module.add("ApiError", module.py().get_type::<ApiError>())?;
    for name in ["field", "offset", "legacy_code", "engine_code"] {
        module
            .py()
            .get_type::<ApiError>()
            .setattr(name, module.py().None())?;
    }
    quantities::register(module)?;
    admission::register(module)?;
    channel::register(module)?;
    profile_capture::register(module)?;
    designer::register(module)?;
    sequence::register(module)?;
    transmitter::register(module)?;
    full_loop::register(module)?;
    inputs::register(module)?;
    receiver::register(module)?;
    options::register(module)?;
    reports::register(module)?;
    controller::register(module)?;
    workbench::register(module)?;
    project::register(module)?;
    module.add_class::<retention::PyRetainedFootprint>()?;
    upstream::register(module)?;
    Ok(())
}

/// Structured Rust error categories; never derived by parsing a message.
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    module = "merlin",
    name = "ErrorCode"
)]
#[derive(Clone, Copy, PartialEq)]
enum PyErrorCode {
    InvalidInput,
    UnsupportedFormat,
    UnsupportedDecoderContract,
    UnsupportedCapability,
    MissingSetting,
    IncompatibleGrid,
    IncompatibleRoles,
    InvalidIdentifier,
    AssociationFailed,
    AmbiguousTimestamp,
    MissingCoverage,
    ResourceLimit,
    Io,
    Busy,
    Engine,
}
impl From<api::ErrorCode> for PyErrorCode {
    /// Read from from the native Rust snapshot.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn from(value: api::ErrorCode) -> Self {
        match value {
            api::ErrorCode::InvalidInput => Self::InvalidInput,
            api::ErrorCode::UnsupportedFormat => Self::UnsupportedFormat,
            api::ErrorCode::UnsupportedDecoderContract => Self::UnsupportedDecoderContract,
            api::ErrorCode::UnsupportedCapability => Self::UnsupportedCapability,
            api::ErrorCode::MissingSetting => Self::MissingSetting,
            api::ErrorCode::IncompatibleGrid => Self::IncompatibleGrid,
            api::ErrorCode::IncompatibleRoles => Self::IncompatibleRoles,
            api::ErrorCode::InvalidIdentifier => Self::InvalidIdentifier,
            api::ErrorCode::AssociationFailed => Self::AssociationFailed,
            api::ErrorCode::AmbiguousTimestamp => Self::AmbiguousTimestamp,
            api::ErrorCode::MissingCoverage => Self::MissingCoverage,
            api::ErrorCode::ResourceLimit => Self::ResourceLimit,
            api::ErrorCode::Io => Self::Io,
            api::ErrorCode::Busy => Self::Busy,
            api::ErrorCode::Engine => Self::Engine,
        }
    }
}
