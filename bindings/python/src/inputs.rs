/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native file/byte conveniences and immutable input snapshots.
use crate::{api, error, object_from_json, quantities::*};
use merlin::downstream::inputs::{read_capture, CaptureLoader};
use pyo3::{prelude::*, types::PyTuple};
use std::path::PathBuf;
/// Explicit protocol interpretation, independent of file-format revision.
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    module = "merlin",
    name = "ProtocolScope"
)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PyProtocolScope {
    Docsis31,
    Docsis40Fdd,
}
impl PyProtocolScope {
    /// Copy conversion fields into existing Rust domain models; perform no scientific calculations.
    /// # Returns
    /// The immutable native value named by this read-only property.
    pub(crate) fn native(self) -> api::ProtocolScope {
        match self {
            Self::Docsis31 => api::ProtocolScope::Docsis31,
            Self::Docsis40Fdd => api::ProtocolScope::Docsis40Fdd,
        }
    }
}
/// Supported data modulation; Rust validates actual engine admission.
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    module = "merlin",
    name = "DataModulation"
)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PyDataModulation {
    Qam16,
    Qam64,
    Qam128,
    Qam256,
    Qam512,
    Qam1024,
    Qam2048,
    Qam4096,
    Qam8192,
    Qam16384,
}
impl PyDataModulation {
    /// Copy conversion fields into existing Rust domain models; perform no scientific calculations.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn native(self) -> api::DataModulation {
        match self {
            Self::Qam16 => api::DataModulation::Qam16,
            Self::Qam64 => api::DataModulation::Qam64,
            Self::Qam128 => api::DataModulation::Qam128,
            Self::Qam256 => api::DataModulation::Qam256,
            Self::Qam512 => api::DataModulation::Qam512,
            Self::Qam1024 => api::DataModulation::Qam1024,
            Self::Qam2048 => api::DataModulation::Qam2048,
            Self::Qam4096 => api::DataModulation::Qam4096,
            Self::Qam8192 => api::DataModulation::Qam8192,
            Self::Qam16384 => api::DataModulation::Qam16384,
        }
    }
}
/// Immutable selected modulation profile and original sources.
#[pyclass(from_py_object, frozen, module = "merlin", name = "DPD")]
#[derive(Clone)]
pub(crate) struct PyDpd(pub(crate) api::Dpd);
#[pymethods]
impl PyDpd {
    /// Discovers all validated captured IDs from a binary PNM or decoded JSON file.
    ///
    /// # Arguments
    /// * `path` - Read-only capture path; contents determine the format.
    ///
    /// # Returns
    /// Owned Rust-backed ProfileId values sorted by captured ID; no simulation.
    ///
    /// # Errors
    /// Preserves structured native I/O, decoder, limit and catalog errors.
    #[staticmethod]
    fn profile_ids_from_file(path: PathBuf) -> PyResult<Vec<PyProfileId>> {
        CaptureLoader::profile_ids(&read_capture(path).map_err(error)?)
            .map(|ids| ids.into_iter().map(PyProfileId).collect())
            .map_err(error)
    }
    /// Discovers all validated captured IDs from bounded original capture bytes.
    ///
    /// # Arguments
    /// * `name` - Display identity, not a format hint.
    /// * `data` - Original binary PNM or decoded JSON, bounded by Rust admission.
    ///
    /// # Returns
    /// Independent Rust-backed ProfileId values sorted by captured ID; no run.
    ///
    /// # Errors
    /// Preserves structured Rust decoder, input-limit and invalid-catalog errors.
    #[staticmethod]
    fn profile_ids_from_bytes(name: &str, data: &[u8]) -> PyResult<Vec<PyProfileId>> {
        CaptureLoader::profile_ids(&named_input(name, data)?)
            .map(|ids| ids.into_iter().map(PyProfileId).collect())
            .map_err(error)
    }
    /// Load supported binary PNM or decoded JSON through bounded native Rust validation.
    /// # Arguments
    /// * `path` - Caller-selected input or new output path; existing inputs are never overwritten.
    /// * `profile_id` - Explicit checked captured-profile identifier.
    /// # Returns
    /// An immutable original input with separate original/intermediate provenance.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_file(path: PathBuf, profile_id: &PyProfileId) -> PyResult<Self> {
        CaptureLoader::profile(&read_capture(path).map_err(error)?, profile_id.0)
            .map(Self)
            .map_err(error)
    }
    /// Load bounded named bytes by content, preserving original capture provenance.
    /// # Arguments
    /// * `name` - Bounded original display identity; never used for content detection.
    /// * `data` - Original binary or JSON bytes, bounded before copying into Rust storage.
    /// * `profile_id` - Explicit checked captured-profile identifier.
    /// # Returns
    /// An immutable validated original input; simulation remains unstarted.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_bytes(name: &str, data: &[u8], profile_id: &PyProfileId) -> PyResult<Self> {
        CaptureLoader::profile(&named_input(name, data)?, profile_id.0)
            .map(Self)
            .map_err(error)
    }
    /// Create a profile hypothesis against an already validated OCD; preserve protected roles.
    /// # Arguments
    /// * `ocd` - Validated immutable OCD to preserve.
    /// * `profile_id` - Explicit checked captured-profile identifier.
    /// * `modulation` - Typed payload modulation; protected roles are unchanged.
    /// # Returns
    /// A created profile hypothesis preserving validated protected roles.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_ocd(
        ocd: &PyOcd,
        profile_id: &PyProfileId,
        modulation: PyDataModulation,
    ) -> PyResult<Self> {
        api::Dpd::from_ocd(&ocd.0, profile_id.0, modulation.native())
            .map(Self)
            .map_err(error)
    }
    /// Return an edited profile hypothesis without mutating original captures or protected roles.
    /// # Arguments
    /// * `carriers` - Checked half-open physical carrier range.
    /// * `modulation` - Typed payload modulation; protected roles are unchanged.
    /// # Returns
    /// An independent edited profile hypothesis; original captures remain unchanged.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn with_modulation(
        &self,
        carriers: &PyCarrierRange,
        modulation: PyDataModulation,
    ) -> PyResult<Self> {
        self.0
            .with_modulation(carriers.0, modulation.native())
            .map(Self)
            .map_err(error)
    }
    /// Read profile id from the native Rust snapshot.
    /// # Returns
    /// The original selected ProfileId, distinct from internal bootstrap wire mapping.
    #[getter]
    fn profile_id(&self) -> PyProfileId {
        PyProfileId(self.0.profile_id())
    }
    /// Reads all profile identities retained by the already-loaded DPD.
    ///
    /// # Returns
    /// An independent list of validated ProfileId values in ascending captured
    /// order. No file is reopened or decoded and no simulation runs. Created
    /// profiles have one ID; edited profiles retain their source catalog.
    #[getter]
    fn profile_ids(&self) -> Vec<PyProfileId> {
        self.0
            .profile_ids()
            .iter()
            .copied()
            .map(PyProfileId)
            .collect()
    }
    /// Read origin from the native Rust snapshot.
    /// # Returns
    /// The Captured, Created or Edited profile-origin classification.
    #[getter]
    fn origin(&self) -> String {
        format!("{:?}", self.0.origin())
    }
    /// Read sources from the native Rust snapshot.
    /// # Returns
    /// Independent original-source provenance; created hypotheses have no captured source.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn sources(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(
            py,
            &serde_json::to_string(self.0.sources()).map_err(|e| {
                error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    e.to_string(),
                ))
            })?,
        )
    }
    /// Read grid from the native Rust snapshot.
    /// # Returns
    /// An independent JSON snapshot of the exact original Hz/index grid.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn grid(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &serde_json::to_string(self.0.grid()).unwrap())
    }
}
/// Checked half-open physical carrier edit range.
#[pyclass(from_py_object, frozen, module = "merlin", name = "CarrierRange")]
#[derive(Clone)]
pub(crate) struct PyCarrierRange(api::CarrierRange);
#[pymethods]
impl PyCarrierRange {
    /// Construct a Rust-backed value or request using the declared units and defaults.
    /// # Arguments
    /// * `first` - Inclusive physical carrier index.
    /// * `end` - Exclusive physical carrier index.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[new]
    fn new(first: &PySubcarrierIndex, end: &PySubcarrierIndex) -> PyResult<Self> {
        api::CarrierRange::new(first.0, end.0)
            .map(Self)
            .map_err(error)
    }
}
/// Mutable override request; OCD construction takes a copied validated snapshot.
#[pyclass(from_py_object, module = "merlin", name = "OcdOverrides")]
#[derive(Clone, Default)]
pub(crate) struct PyOcdOverrides {
    #[pyo3(get, set)]
    bandwidth: Option<PyBandwidthHz>,
    #[pyo3(get, set)]
    cyclic_prefix: Option<PyCyclicPrefixSamples>,
    #[pyo3(get, set)]
    roll_off: Option<PyRollOffSamples>,
    #[pyo3(get, set)]
    interleaver_depth: Option<PyInterleaverDepth>,
}
impl PyOcdOverrides {
    /// Copy conversion fields into existing Rust domain models; perform no scientific calculations.
    /// # Returns
    /// The immutable native value named by this read-only property.
    pub(crate) fn native(&self) -> api::OcdOverrides {
        api::OcdOverrides {
            bandwidth: self.bandwidth.as_ref().map(|v| v.0),
            cyclic_prefix: self.cyclic_prefix.as_ref().map(|v| v.0),
            roll_off: self.roll_off.as_ref().map(|v| v.0),
            interleaver_depth: self.interleaver_depth.as_ref().map(|v| v.0),
        }
    }
}
#[pymethods]
impl PyOcdOverrides {
    /// Construct a Rust-backed value or request using the declared units and defaults.
    /// # Arguments
    /// * `bandwidth` - Native domain value or owned conversion data.
    /// * `cyclic_prefix` - Native domain value or owned conversion data.
    /// * `roll_off` - Native domain value or owned conversion data.
    /// * `interleaver_depth` - Native domain value or owned conversion data.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    #[new]
    #[pyo3(signature=(*,bandwidth=None,cyclic_prefix=None,roll_off=None,interleaver_depth=None))]
    fn new(
        bandwidth: Option<PyBandwidthHz>,
        cyclic_prefix: Option<PyCyclicPrefixSamples>,
        roll_off: Option<PyRollOffSamples>,
        interleaver_depth: Option<PyInterleaverDepth>,
    ) -> Self {
        Self {
            bandwidth,
            cyclic_prefix,
            roll_off,
            interleaver_depth,
        }
    }
}
/// Immutable resolved OCD; timing origins remain inspectable.
#[pyclass(from_py_object, frozen, module = "merlin", name = "OCD")]
#[derive(Clone)]
pub(crate) struct PyOcd(pub(crate) api::Ocd);
#[pymethods]
impl PyOcd {
    /// Resolve the selected profile into validated OCD geometry and documented value origins.
    /// # Arguments
    /// * `protocol` - Explicit DOCSIS interpretation, independent of file format.
    /// * `dpd` - Selected immutable captured or hypothesis profile.
    /// * `overrides` - Typed optional overrides; None requests Rust resolution/defaults.
    /// # Returns
    /// An immutable OCD with explicit, derived or defaulted setting origins.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    #[pyo3(signature=(protocol,dpd,overrides=None))]
    fn from_profile(
        protocol: PyProtocolScope,
        dpd: &PyDpd,
        overrides: Option<PyOcdOverrides>,
    ) -> PyResult<Self> {
        api::Ocd::from_profile(
            protocol.native(),
            &dpd.0,
            overrides.unwrap_or_default().native(),
        )
        .map(Self)
        .map_err(error)
    }
    /// Read configuration from the native Rust snapshot.
    /// # Returns
    /// An independent JSON snapshot of resolved values, origins and validated geometry.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn configuration(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &serde_json::to_string(self.0.configuration()).unwrap())
    }
}
macro_rules! measurement {
    ($python:ident,$name:literal,$native:ident,$loader:ident) => {
        #[doc=concat!("Immutable original ",$name,"; decoding and validation execute only in Rust.")]
        #[pyclass(from_py_object,frozen,module="merlin",name=$name)]
        #[derive(Clone)]
        pub(crate) struct $python(pub(crate) api::$native);
        #[pymethods]
        impl $python {
            /// Load supported binary PNM or decoded JSON through bounded native Rust validation.
            /// # Arguments
            /// * `path` - Caller-selected input or new output path; existing inputs are never overwritten.
            /// # Returns
            /// An immutable original input with separate original/intermediate provenance.
            /// # Errors
            /// Preserves structured Rust validation errors and Python conversion failures.
            #[staticmethod]
            fn from_file(path: PathBuf) -> PyResult<Self> {
                CaptureLoader::$loader(&read_capture(path).map_err(error)?)
                    .map(Self)
                    .map_err(error)
            }
            /// Load bounded named bytes by content, preserving original capture provenance.
            /// # Arguments
            /// * `name` - Bounded original display identity; never used for content detection.
            /// * `data` - Original binary or JSON bytes, bounded before copying into Rust storage.
            /// # Returns
            /// An immutable validated original input; simulation remains unstarted.
            /// # Errors
            /// Preserves structured Rust validation errors and Python conversion failures.
            #[staticmethod]
            fn from_bytes(name: &str, data: &[u8]) -> PyResult<Self> {
                CaptureLoader::$loader(&named_input(name, data)?)
                    .map(Self)
                    .map_err(error)
            }
            /// Read provenance from the native Rust snapshot.
            /// # Returns
            /// Independent original/intermediate source metadata with unchanged epochs and hashes.
            /// # Errors
            /// Preserves structured Rust validation errors and Python conversion failures.
            #[getter]
            fn provenance(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                object_from_json(py, &serde_json::to_string(self.0.provenance()).unwrap())
            }
            /// Reads the exact backend scientific source-kind identifier.
            /// # Returns
            /// Original identifier, preserving synthetic/designed/ordinary distinctions.
            #[getter]
            fn source_kind(&self) -> &str {
                self.0.source_kind()
            }
            /// Read grid from the native Rust snapshot.
            /// # Returns
            /// An independent JSON snapshot of the exact original Hz/index grid.
            /// # Errors
            /// Preserves structured Rust validation errors and Python conversion failures.
            #[getter]
            fn grid(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                object_from_json(py, &serde_json::to_string(self.0.grid()).unwrap())
            }
        }
    };
}
measurement!(PyRxMer, "RxMer", RxMer, rxmer);
measurement!(
    PyChannelEstimate,
    "ChannelEstimate",
    ChannelEstimate,
    channel_estimate
);
#[pymethods]
impl PyRxMer {
    /// Read values from the native Rust snapshot.
    /// # Returns
    /// An immutable tuple of original MerDb values or None for missing samples.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn values(&self, py: Python<'_>) -> PyResult<Py<PyTuple>> {
        let values = self
            .0
            .values()
            .into_iter()
            .map(|v| v.map(PyMerDb))
            .collect::<Vec<_>>();
        Ok(PyTuple::new(py, values)?.unbind())
    }
}
/// Registers immutable input types and explicit protocol/modulation enums.
/// # Arguments
/// * `module` - Python extension namespace.
/// # Returns
/// Registered classes; no captures loaded or simulations run.
/// # Errors
/// Propagates Python type registration errors.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyOcdConfiguration>()?;
    module.add_class::<PyComplexResponse>()?;
    module.add_class::<PyProtocolScope>()?;
    module.add_class::<PyDataModulation>()?;
    module.add_class::<PyDpd>()?;
    module.add_class::<PyRxMer>()?;
    module.add_class::<PyChannelEstimate>()?;
    module.add_class::<PyOcdOverrides>()?;
    module.add_class::<PyOcd>()?;
    module.add_class::<PyCarrierRange>()?;
    module
        .getattr("ProtocolScope")?
        .setattr("DOCSIS31", PyProtocolScope::Docsis31)?;
    module
        .getattr("ProtocolScope")?
        .setattr("DOCSIS40_FDD", PyProtocolScope::Docsis40Fdd)?;
    Ok(())
}

/// Read named input from the native Rust snapshot.
/// # Arguments
/// * `name` - Bounded original display identity; never used for content detection.
/// * `data` - Original binary or JSON bytes, bounded before copying into Rust storage.
/// # Returns
/// The immutable native value named by this read-only property.
/// # Errors
/// Preserves structured Rust validation errors and Python conversion failures.
pub(crate) fn named_input(name: &str, data: &[u8]) -> PyResult<api::NamedCapture> {
    if data.len() > api::MAX_CAPTURE_BYTES || name.len() > 4096 {
        return Err(error(api::ApiError::new(
            api::ErrorCode::ResourceLimit,
            "Named capture exceeds input bounds",
        )));
    }
    api::NamedCapture::new(name, data.to_vec()).map_err(error)
}
/// Checked manual geometry request; JSON is an I/O boundary, never a scientific engine.
#[pyclass(from_py_object, frozen, module = "merlin", name = "OcdConfiguration")]
#[derive(Clone)]
pub(crate) struct PyOcdConfiguration(api::OcdConfiguration);
#[pymethods]
impl PyOcdConfiguration {
    /// Validate a manual geometry request at the JSON transport boundary in Rust.
    /// # Arguments
    /// * `key_json` - Bounded JSON identity/grid transport document.
    /// * `layout_json` - Bounded JSON protected-layout transport document.
    /// * `overrides` - Typed optional overrides; None requests Rust resolution/defaults.
    /// # Returns
    /// A checked manual OCD configuration request, ready for native admission.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_json(key_json: &str, layout_json: &str, overrides: &PyOcdOverrides) -> PyResult<Self> {
        if key_json.len() + layout_json.len() > api::MAX_CAPTURE_BYTES {
            return Err(error(api::ApiError::new(
                api::ErrorCode::ResourceLimit,
                "OCD request exceeds input bounds",
            )));
        }
        let key = serde_json::from_str(key_json).map_err(|e| {
            error(api::ApiError::new(
                api::ErrorCode::InvalidInput,
                e.to_string(),
            ))
        })?;
        let layout = serde_json::from_str(layout_json).map_err(|e| {
            error(api::ApiError::new(
                api::ErrorCode::InvalidInput,
                e.to_string(),
            ))
        })?;
        api::OcdConfiguration::new(key, layout, overrides.native())
            .map(Self)
            .map_err(error)
    }
}
#[pymethods]
impl PyOcd {
    /// Admit manual OCD geometry through the canonical Rust configuration validator.
    /// # Arguments
    /// * `protocol` - Explicit DOCSIS interpretation, independent of file format.
    /// * `configuration` - Complete checked manual geometry request.
    /// # Returns
    /// An immutable validated OCD; no simulation is started.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn create(protocol: PyProtocolScope, configuration: &PyOcdConfiguration) -> PyResult<Self> {
        api::Ocd::new(protocol.native(), configuration.0.clone())
            .map(Self)
            .map_err(error)
    }
}
#[pymethods]
impl PyDpd {
    /// Read carriers from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn carriers(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &serde_json::to_string(self.0.carriers()).unwrap())
    }
}
/// Original complex H as a read-only Rust domain value.
#[pyclass(from_py_object, frozen, module = "merlin", name = "ComplexResponse")]
#[derive(Clone)]
pub(crate) struct PyComplexResponse(api::ComplexResponse);
#[pymethods]
impl PyComplexResponse {
    /// Read real from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn real(&self) -> crate::reports::PyResponseReal {
        crate::reports::PyResponseReal(self.0.real())
    }
    /// Read imaginary from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn imaginary(&self) -> crate::reports::PyResponseImaginary {
        crate::reports::PyResponseImaginary(self.0.imaginary())
    }
}
#[pymethods]
impl PyChannelEstimate {
    /// Read response from the native Rust snapshot.
    /// # Returns
    /// An immutable tuple of original complex H values or None for missing samples.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn response(&self, py: Python<'_>) -> PyResult<Py<PyTuple>> {
        Ok(PyTuple::new(
            py,
            self.0
                .response()
                .into_iter()
                .map(|point| point.map(PyComplexResponse)),
        )?
        .unbind())
    }
}

#[pymethods]
impl PyOcd {
    /// Exports complete validated geometry and resolved origins as an owned snapshot.
    /// # Returns
    /// Independent dictionary; changes never mutate the accepted OCD.
    /// # Errors
    /// Propagates Python JSON allocation errors.
    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &serde_json::to_string(&self.0).unwrap())
    }
}
