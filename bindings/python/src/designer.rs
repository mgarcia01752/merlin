/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native ownership/transport over the public immutable Designer admission API.
use crate::{
    api, error,
    inputs::{PyDpd, PyRxMer},
    object_from_json,
    profile_capture::PyProfileCapture,
    quantities::{PyCaptureTime, PyMerDb},
};
use merlin::downstream::inputs::{RxMerDesign, RxMerDesignPosition};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "RxMerDesignPosition"
)]
#[derive(Clone)]
pub(crate) struct PyDesignPosition(RxMerDesignPosition);
#[pyclass(from_py_object, frozen, module = "merlin", name = "RxMerDesign")]
#[derive(Clone)]
pub(crate) struct PyDesign(RxMerDesign);

fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}

#[pymethods]
impl PyDesignPosition {
    /// Checks an original one-based collection position through the public Rust type.
    /// # Arguments
    /// * `index`, `count` - Original one-based position and bounded collection size.
    /// # Returns
    /// Immutable position; this is independent of synthetic playback time.
    /// # Errors
    /// Shared invalid count/position errors raise ApiError.
    #[new]
    #[pyo3(signature=(index=1, count=1))]
    fn new(index: usize, count: usize) -> PyResult<Self> {
        RxMerDesignPosition::new(index, count)
            .map(Self)
            .map_err(error)
    }
    /// Reads the original one-based position without changing a design.
    /// # Returns
    /// Checked original collection position.
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }
    /// Reads the full original baseline collection count.
    /// # Returns
    /// Checked original count, 1 through 256.
    #[getter]
    fn count(&self) -> usize {
        self.0.count
    }
}

#[pymethods]
impl PyDesign {
    /// Loads an exact saved Designer envelope through the public native Rust owner.
    /// # Arguments
    /// * `name` - Original display identity, never a format hint.
    /// * `data` - Bounded original Designer bytes; whitespace remains unchanged.
    /// # Returns
    /// Immutable design; recipe materialization starts no PHY/source/noise stream.
    /// # Errors
    /// Shared version/schema/integrity/nesting/work errors raise ApiError.
    #[staticmethod]
    fn from_bytes(py: Python<'_>, name: &str, data: &[u8]) -> PyResult<Self> {
        let input = crate::inputs::named_input(name, data)?;
        py.detach(|| RxMerDesign::from_bytes(&input))
            .map(Self)
            .map_err(error)
    }

    /// Creates a flat original-grid design through Rust's typed baseline operation.
    /// # Arguments
    /// * `profile` - Accepted original identity and carrier roles.
    /// * `baseline` - Explicit checked MER quantity; Rust validates design bounds.
    /// * `epoch` - Original anchor seconds, independent of the playback clock.
    /// * `recipe_json` - Bounded recipe transport; None selects empty effects.
    /// * `position` - Original collection position; None selects 1 of 1.
    /// # Returns
    /// Immutable synthetic design with authoritative role mask and original provenance.
    /// # Errors
    /// Shared recipe/grid/position/baseline/work errors raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(profile, baseline, epoch, recipe_json=None, position=None))]
    fn flat(
        py: Python<'_>,
        profile: &PyDpd,
        baseline: &PyMerDb,
        epoch: &PyCaptureTime,
        recipe_json: Option<&[u8]>,
        position: Option<&PyDesignPosition>,
    ) -> PyResult<Self> {
        let position = position.map(|value| value.0);
        py.detach(|| {
            let recipe = RxMerDesign::recipe_from_json(recipe_json.unwrap_or(b"{}"))?;
            let position = position.map_or_else(|| RxMerDesignPosition::new(1, 1), Ok)?;
            RxMerDesign::flat(&profile.0, baseline.0, epoch.0, recipe, position)
        })
        .map(Self)
        .map_err(error)
    }

    /// Wraps exact ordinary capture bytes in a noncompounding original-baseline design.
    /// # Arguments
    /// * `profile` - Accepted spatial identity/grid, checked by Rust.
    /// * `name`, `data` - Exact ordinary original capture identity and bytes.
    /// * `recipe_json` - Bounded recipe transport; None selects empty effects.
    /// * `position` - Original collection position; None selects 1 of 1.
    /// # Returns
    /// Immutable designed capture; imported baselines reject flat-only random variation.
    /// # Errors
    /// Shared format/nesting/integrity/recipe/grid/work errors raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(profile, name, data, recipe_json=None, position=None))]
    fn imported(
        py: Python<'_>,
        profile: &PyDpd,
        name: &str,
        data: &[u8],
        recipe_json: Option<&[u8]>,
        position: Option<&PyDesignPosition>,
    ) -> PyResult<Self> {
        let position = position.map(|value| value.0);
        let input = crate::inputs::named_input(name, data)?;
        py.detach(|| {
            let recipe = RxMerDesign::recipe_from_json(recipe_json.unwrap_or(b"{}"))?;
            let position = position.map_or_else(|| RxMerDesignPosition::new(1, 1), Ok)?;
            RxMerDesign::imported(&profile.0, &input, recipe, position)
        })
        .map(Self)
        .map_err(error)
    }

    /// Creates a flat design from exact original source roles without run-ID admission.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable source catalog and original byte identity.
    /// * `baseline`, `epoch` - Checked MER and explicit original anchor seconds.
    /// * `recipe_json`, `position` - Existing optional recipe and original collection position.
    /// # Returns
    /// Immutable synthetic design evaluated entirely in Rust with the GIL released.
    /// # Errors
    /// Missing source and existing recipe/grid/position/baseline/work errors raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(capture, profile_id, baseline, epoch, recipe_json=None, position=None))]
    fn flat_capture(
        py: Python<'_>,
        capture: &PyProfileCapture,
        profile_id: u8,
        baseline: &PyMerDb,
        epoch: &PyCaptureTime,
        recipe_json: Option<&[u8]>,
        position: Option<&PyDesignPosition>,
    ) -> PyResult<Self> {
        let position = position.map(|value| value.0);
        py.detach(|| {
            let recipe = RxMerDesign::recipe_from_json(recipe_json.unwrap_or(b"{}"))?;
            let position = position.map_or_else(|| RxMerDesignPosition::new(1, 1), Ok)?;
            RxMerDesign::flat_capture(
                &capture.0, profile_id, baseline.0, epoch.0, recipe, position,
            )
        })
        .map(Self)
        .map_err(error)
    }

    /// Wraps an ordinary baseline using a source catalog's original spatial identity.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable source catalog and original byte identity.
    /// * `name`, `data` - Exact ordinary baseline identity and bytes.
    /// * `recipe_json`, `position` - Existing optional edits and original collection position.
    /// # Returns
    /// Immutable noncompounding design evaluated in Rust with the GIL released.
    /// # Errors
    /// Missing source and existing nested/integrity/recipe/grid/work errors raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(capture, profile_id, name, data, recipe_json=None, position=None))]
    fn imported_capture(
        py: Python<'_>,
        capture: &PyProfileCapture,
        profile_id: u8,
        name: &str,
        data: &[u8],
        recipe_json: Option<&[u8]>,
        position: Option<&PyDesignPosition>,
    ) -> PyResult<Self> {
        let position = position.map(|value| value.0);
        let input = crate::inputs::named_input(name, data)?;
        py.detach(|| {
            let recipe = RxMerDesign::recipe_from_json(recipe_json.unwrap_or(b"{}"))?;
            let position = position.map_or_else(|| RxMerDesignPosition::new(1, 1), Ok)?;
            RxMerDesign::imported_capture(&capture.0, profile_id, &input, recipe, position)
        })
        .map(Self)
        .map_err(error)
    }

    /// Checks exact source-catalog spatial association without admitting a run ID.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable catalog and original byte identity.
    /// # Returns
    /// None on exact device/channel/grid equality, without changing inputs or epochs.
    /// # Errors
    /// Missing sources and incompatible grids raise the shared ApiError.
    fn check_capture(&self, capture: &PyProfileCapture, profile_id: u8) -> PyResult<()> {
        self.0.check_capture(&capture.0, profile_id).map_err(error)
    }

    /// Replaces effects using the retained original baseline, never the previous output.
    /// # Arguments
    /// * `recipe_json` - Bounded UTF-8 configuration transport evaluated entirely in Rust.
    /// # Returns
    /// Fresh immutable design; previous results remain usable and unchanged.
    /// # Errors
    /// Shared malformed/unknown/recipe/work errors raise ApiError atomically.
    fn with_recipe(&self, py: Python<'_>, recipe_json: &[u8]) -> PyResult<Self> {
        py.detach(|| {
            self.0
                .with_recipe(RxMerDesign::recipe_from_json(recipe_json)?)
        })
        .map(Self)
        .map_err(error)
    }

    /// Checks exact original spatial association before adopting a loaded design.
    /// # Arguments
    /// * `profile` - Accepted device/channel/frequency-grid identity.
    /// # Returns
    /// None on equality, preserving original data and timestamps.
    /// # Errors
    /// IncompatibleGrid raises the shared ApiError.
    fn check_profile(&self, profile: &PyDpd) -> PyResult<()> {
        self.0.check_profile(&profile.0).map_err(error)
    }

    /// Copies the already evaluated capture into independent native ownership.
    /// # Returns
    /// Existing immutable RxMer, usable after this design is released.
    fn materialize(&self, py: Python<'_>) -> PyRxMer {
        PyRxMer(py.detach(|| self.0.materialize()))
    }

    /// Copies exact original envelope bytes, including loaded whitespace.
    /// # Returns
    /// Independently owned Python bytes; no recipe or source state changes.
    #[getter]
    fn original_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.0.original_bytes())
    }

    /// Returns an owned view of the accepted Rust recipe configuration.
    /// # Returns
    /// Independent dictionary; caller changes do not alter the baseline or recipe.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    #[getter]
    fn recipe(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(self.0.recipe())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads the original baseline collection position, separate from playback time.
    /// # Returns
    /// Existing immutable RxMerDesignPosition.
    #[getter]
    fn position(&self) -> PyDesignPosition {
        PyDesignPosition(self.0.position())
    }

    /// Distinguishes a flat synthetic baseline from an imported ordinary observation.
    /// # Returns
    /// True for synthetic flat, false for a designed imported baseline.
    #[getter]
    fn is_synthetic(&self) -> bool {
        self.0.is_synthetic()
    }
}

#[pymethods]
impl PyDpd {
    /// Reads exact original spatial identity from the public Rust profile.
    /// # Returns
    /// Owned device/channel/grid dictionary; editing it cannot change the profile.
    /// # Errors
    /// Preserves transport and Python allocation failures.
    #[getter]
    fn key(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(self.0.key())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

/// Registers immutable converters in the existing single native extension.
/// # Arguments
/// * `module` - Existing native namespace; import starts no scientific operation.
/// # Returns
/// Unit on registration, preserving Python type/allocation errors.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyDesignPosition>()?;
    module.add_class::<PyDesign>()
}
