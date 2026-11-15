/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit checked unit conversions; no scientific arithmetic in this adapter.
use crate::{api, error};
use pyo3::prelude::*;
macro_rules! quantity {
    ($python:ident,$rust:ident,$primitive:ty,$name:literal,$description:literal) => {
        #[doc=concat!("Checked Rust ",stringify!($rust),". ",$description,"\n\nConstruct with a primitive value; invalid values raise ApiError (or TypeError/OverflowError for Python conversion failures). The read-only .value property returns that primitive without converting units.")]
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
quantity!(
    PyProfileId,
    ProfileId,
    u8,
    "ProfileId",
    "Captured profile identity, 0 through 15; distinct from bootstrap wire identity."
);
quantity!(
    PyCaptureTime,
    CaptureTime,
    u32,
    "CaptureTime",
    "Original capture epoch in whole seconds, 0 through 4294967295."
);
quantity!(
    PyAssociationTolerance,
    AssociationTolerance,
    u32,
    "AssociationTolerance",
    "Maximum capture/profile epoch distance in whole seconds, 0 through 4294967295."
);
quantity!(
    PyObservationDuration,
    ObservationDuration,
    u64,
    "ObservationDuration",
    "Source observation in milliseconds, 1 through 3600000; excludes acquisition and drain."
);
quantity!(
    PyRuntimeLimit,
    RuntimeLimit,
    u64,
    "RuntimeLimit",
    "Host runtime budget in milliseconds, 1 through 300000."
);
quantity!(
    PyNoiseSeed,
    NoiseSeed,
    u64,
    "NoiseSeed",
    "Reproducible noise RNG seed, 0 through 18446744073709551615."
);
quantity!(
    PyPrbsSeed,
    PrbsSeed,
    u16,
    "PrbsSeed",
    "Nonzero fifteen-bit PRBS seed, 1 through 32767."
);
quantity!(
    PyIterationLimit,
    IterationLimit,
    usize,
    "IterationLimit",
    "Maximum decoder iterations, 1 through 100."
);
quantity!(
    PyRetainedWordLimit,
    RetainedWordLimit,
    usize,
    "RetainedWordLimit",
    "Maximum retained detailed words, 0 through 10000."
);
quantity!(
    PyFrequencyHz,
    FrequencyHz,
    u32,
    "FrequencyHz",
    "Absolute frequency in Hz, 0 through 4294967295."
);
quantity!(
    PyBandwidthHz,
    BandwidthHz,
    u32,
    "BandwidthHz",
    "Occupied bandwidth in Hz, 24000000 through 192000000 in 6000000 Hz steps."
);
quantity!(
    PyCyclicPrefixSamples,
    CyclicPrefixSamples,
    u16,
    "CyclicPrefixSamples",
    "Prefix length at 204.8 MS/s: 192, 256, 512, 768 or 1024 samples."
);
quantity!(
    PyRollOffSamples,
    RollOffSamples,
    u16,
    "RollOffSamples",
    "Roll-off length: 0, 64, 128, 192 or 256 samples; zero is evaluation-only."
);
quantity!(
    PyInterleaverDepth,
    InterleaverDepth,
    u8,
    "InterleaverDepth",
    "Interleaver branches, 1 through 32; OCD validates spacing-specific constraints."
);
quantity!(
    PySubcarrierIndex,
    SubcarrierIndex,
    u16,
    "SubcarrierIndex",
    "Physical FFT bin coordinate, 0 through 8191."
);
quantity!(
    PyCarrierCount,
    CarrierCount,
    usize,
    "CarrierCount",
    "Original carrier count, 1 through 65536."
);
quantity!(
    PyMerDb,
    MerDb,
    f64,
    "MerDb",
    "Finite MER in dB; run preflight additionally validates supported residual-power bounds."
);
quantity!(
    PyWeight,
    Weight,
    f64,
    "Weight",
    "Finite normalized contribution weight, 0 through 1 inclusive."
);
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
