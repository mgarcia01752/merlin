/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native ownership/transport and bounded dispatch over public downstream full-loop contracts.
mod cells;
mod control;
mod instruments;
mod observations;
mod options;
mod request;
mod result;
mod soft;
mod worker;
use crate::{api, error, object_from_json};
use control::PyFullLoopControl;
use options::PyFullLoopOptions;
use pyo3::prelude::*;
use request::PyFullLoopRequest;
use result::PyFullLoopResult;
fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}

/// Registers directional immutable request/control/report conversions in one native extension.
/// # Arguments
/// * `module` - Existing extension; registration performs no scientific work or thread dispatch.
/// # Returns
/// Unit on registration; Python type/allocation failures remain visible.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyFullLoopOptions>()?;
    module.add_class::<PyFullLoopControl>()?;
    module.add_class::<PyFullLoopRequest>()?;
    module.add_class::<cells::PyCellQuery>()?;
    module.add_class::<observations::PyObservationPolicy>()?;
    module.add_class::<PyFullLoopResult>()
}
