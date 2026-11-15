/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit checked unit conversions; no scientific arithmetic in this adapter.
use crate::{api, error};
use pyo3::prelude::*;
macro_rules! quantity {
    ($python:ident,$rust:ident,$primitive:ty,$name:literal) => {
        #[doc=concat!("Checked Rust ",stringify!($rust),"; .value is the explicit primitive conversion.")]
        #[pyclass(from_py_object,frozen,module="merlin",name=$name)]
        #[derive(Clone)]
        pub(crate) struct $python(pub(crate) api::$rust);
        #[pymethods]
        impl $python {
            /// Construct a Rust-backed value or request using the declared units and defaults.
            /// # Arguments
            /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
            /// # Returns
            /// A fresh Rust-backed value or request, with native defaults where documented.
            /// # Errors
            /// Preserves structured Rust validation errors and Python conversion failures.
            #[new]
            fn new(value:$primitive)->PyResult<Self>{api::$rust::new(value).map(Self).map_err(error)}
            /// Read the value in this domain type's explicit primitive units.
            /// # Returns
            /// A primitive scalar in the domain type's declared units.
            #[getter]
            fn value(&self)->$primitive{self.0.value()}
            /// Format a readable domain identity without enabling unchecked construction.
            /// # Returns
            /// Owned readable diagnostic text; opaque IDs remain unconstructible.
            fn __repr__(&self)->String{format!("{}({:?})",stringify!($rust),self.0.value())}
        }
    }
}
quantity!(PyProfileId, ProfileId, u8, "ProfileId");
quantity!(PyCaptureTime, CaptureTime, u32, "CaptureTime");
quantity!(
    PyAssociationTolerance,
    AssociationTolerance,
    u32,
    "AssociationTolerance"
);
quantity!(
    PyObservationDuration,
    ObservationDuration,
    u64,
    "ObservationDuration"
);
quantity!(PyRuntimeLimit, RuntimeLimit, u64, "RuntimeLimit");
quantity!(PyNoiseSeed, NoiseSeed, u64, "NoiseSeed");
quantity!(PyPrbsSeed, PrbsSeed, u16, "PrbsSeed");
quantity!(PyIterationLimit, IterationLimit, usize, "IterationLimit");
quantity!(
    PyRetainedWordLimit,
    RetainedWordLimit,
    usize,
    "RetainedWordLimit"
);
quantity!(PyFrequencyHz, FrequencyHz, u32, "FrequencyHz");
quantity!(PyBandwidthHz, BandwidthHz, u32, "BandwidthHz");
quantity!(
    PyCyclicPrefixSamples,
    CyclicPrefixSamples,
    u16,
    "CyclicPrefixSamples"
);
quantity!(PyRollOffSamples, RollOffSamples, u16, "RollOffSamples");
quantity!(PyInterleaverDepth, InterleaverDepth, u8, "InterleaverDepth");
quantity!(PySubcarrierIndex, SubcarrierIndex, u16, "SubcarrierIndex");
quantity!(PyCarrierCount, CarrierCount, usize, "CarrierCount");
quantity!(PyMerDb, MerDb, f64, "MerDb");
quantity!(PyWeight, Weight, f64, "Weight");
/// Registers checked unit types into the extension namespace.
/// # Arguments
/// * `module` - Python extension module.
/// # Returns
/// Unit after registering conversions.
/// # Errors
/// Propagates Python allocation/type registration errors.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyProfileId>()?;
    module.add_class::<PyCaptureTime>()?;
    module.add_class::<PyAssociationTolerance>()?;
    module.add_class::<PyObservationDuration>()?;
    module.add_class::<PyRuntimeLimit>()?;
    module.add_class::<PyNoiseSeed>()?;
    module.add_class::<PyPrbsSeed>()?;
    module.add_class::<PyIterationLimit>()?;
    module.add_class::<PyRetainedWordLimit>()?;
    module.add_class::<PyFrequencyHz>()?;
    module.add_class::<PyBandwidthHz>()?;
    module.add_class::<PyCyclicPrefixSamples>()?;
    module.add_class::<PyRollOffSamples>()?;
    module.add_class::<PyInterleaverDepth>()?;
    module.add_class::<PySubcarrierIndex>()?;
    module.add_class::<PyCarrierCount>()?;
    module.add_class::<PyMerDb>()?;
    module.add_class::<PyWeight>()?;
    // Class attributes are immutable native quantity instances, not unchecked integers.
    module
        .getattr("ProfileId")?
        .setattr("PROFILE_A", PyProfileId(api::ProfileId::PROFILE_A))?;
    module.getattr("AssociationTolerance")?.setattr(
        "ZERO",
        PyAssociationTolerance(api::AssociationTolerance::ZERO),
    )?;
    Ok(())
}

#[pymethods]
impl PyCaptureTime {
    /// Convert original whole epoch seconds through the checked Rust constructor.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// A checked CaptureTime in original whole epoch seconds.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_epoch_seconds(value: u32) -> PyResult<Self> {
        Self::new(value)
    }
}
#[pymethods]
impl PyAssociationTolerance {
    /// Convert explicit association seconds through the checked Rust constructor.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// A checked AssociationTolerance in whole seconds.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_seconds(value: u32) -> PyResult<Self> {
        Self::new(value)
    }
}
#[pymethods]
impl PyObservationDuration {
    /// Convert explicit milliseconds through the checked Rust constructor.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// A checked duration in this class's declared milliseconds.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_milliseconds(value: u64) -> PyResult<Self> {
        Self::new(value)
    }
}
#[pymethods]
impl PyRuntimeLimit {
    /// Convert explicit milliseconds through the checked Rust constructor.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// A checked duration in this class's declared milliseconds.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[staticmethod]
    fn from_milliseconds(value: u64) -> PyResult<Self> {
        Self::new(value)
    }
}
