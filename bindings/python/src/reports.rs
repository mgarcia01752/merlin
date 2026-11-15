/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Read-only Python views retaining independently owned Rust result snapshots.
use crate::{api, error, object_from_json, options::PyEvaluationOptions, quantities::PyProfileId};
use pyo3::prelude::*;
use std::sync::Arc;
macro_rules! result_quantity {
    ($python:ident,$native:ident,$primitive:ty,$name:literal,$description:literal) => {
        #[doc=concat!("Read-only Rust ",$name,". ",$description," Primitive conversion is explicit through .value. Instances come from owned native reports, not caller construction.")]
        #[pyclass(from_py_object,frozen,module="merlin",name=$name)]
        #[derive(Clone)]
        pub(crate) struct $python(pub(crate) api::$native);
        #[pymethods]
        impl $python {
            /// Read the value in this domain type's explicit primitive units.
            /// # Returns
            /// A primitive scalar in the domain type's declared units.
            #[getter]
            fn value(&self) -> $primitive {
                self.0.value()
            }
        }
    };
}
result_quantity!(
    PyCodewordCount,
    CodewordCount,
    u64,
    "CodewordCount",
    "Completed simulated codewords."
);
result_quantity!(
    PyBitCount,
    BitCount,
    u64,
    "BitCount",
    "Independently compared source bits."
);
result_quantity!(
    PySampleCount,
    SampleCount,
    u64,
    "SampleCount",
    "Valid same-bin diagnostic samples."
);
result_quantity!(
    PyElapsedTime,
    ElapsedTime,
    u64,
    "ElapsedTime",
    "Host elapsed milliseconds; not simulated duration."
);
result_quantity!(
    PyBitErrorRatio,
    BitErrorRatio,
    f64,
    "BitErrorRatio",
    "Post-FEC dimensionless bit-error ratio."
);
result_quantity!(
    PyPayloadRate,
    PayloadRate,
    f64,
    "PayloadRate",
    "Payload information bits per second, not application goodput."
);
result_quantity!(
    PyLinearPower,
    LinearPower,
    f64,
    "LinearPower",
    "Accumulated normalized power; not calibrated RF power."
);
/// Native batch status, independent of decoder success.
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    module = "merlin",
    name = "BatchStatus"
)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PyBatchStatus {
    Complete,
    CompletedWithErrors,
    Cancelled,
}
impl From<api::BatchStatus> for PyBatchStatus {
    /// Read from from the native Rust snapshot.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn from(value: api::BatchStatus) -> Self {
        match value {
            api::BatchStatus::Complete => Self::Complete,
            api::BatchStatus::CompletedWithErrors => Self::CompletedWithErrors,
            api::BatchStatus::Cancelled => Self::Cancelled,
        }
    }
}
/// Native entry lifecycle; Complete is not a FEC correctness claim.
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    module = "merlin",
    name = "RunStatus"
)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum PyRunStatus {
    Complete,
    Cancelled,
    RuntimeExceeded,
    Failed,
    NotStarted,
}
impl From<api::RunStatus> for PyRunStatus {
    /// Read from from the native Rust snapshot.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn from(value: api::RunStatus) -> Self {
        match value {
            api::RunStatus::Complete => Self::Complete,
            api::RunStatus::Cancelled => Self::Cancelled,
            api::RunStatus::RuntimeExceeded => Self::RuntimeExceeded,
            api::RunStatus::Failed => Self::Failed,
            api::RunStatus::NotStarted => Self::NotStarted,
        }
    }
}
/// Actual Rust decoder counters in an explicit scientific scope.
#[pyclass(from_py_object, frozen, module = "merlin", name = "FecReport")]
#[derive(Clone)]
pub(crate) struct PyFecReport(api::FecReport);
#[pymethods]
impl PyFecReport {
    /// Read scope from the native Rust snapshot.
    /// # Returns
    /// The explicit scientific scope, preserving observation versus whole-run distinctions.
    #[getter]
    fn scope(&self) -> String {
        format!("{:?}", self.0.scope())
    }
    /// Read total codewords from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn total_codewords(&self) -> PyCodewordCount {
        PyCodewordCount(self.0.total_codewords())
    }
    /// Read clean codewords from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn clean_codewords(&self) -> PyCodewordCount {
        PyCodewordCount(self.0.clean_codewords())
    }
    /// Read corrected codewords from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn corrected_codewords(&self) -> PyCodewordCount {
        PyCodewordCount(self.0.corrected_codewords())
    }
    /// Read uncorrectable codewords from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn uncorrectable_codewords(&self) -> PyCodewordCount {
        PyCodewordCount(self.0.uncorrectable_codewords())
    }
    /// Read undetected errors from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn undetected_errors(&self) -> PyCodewordCount {
        PyCodewordCount(self.0.undetected_errors())
    }
}
/// Actual observation-window source comparison and payload information rate.
#[pyclass(from_py_object, frozen, module = "merlin", name = "ProfilePerformance")]
#[derive(Clone)]
pub(crate) struct PyProfilePerformance(api::ProfilePerformance);
#[pymethods]
impl PyProfilePerformance {
    /// Read scope from the native Rust snapshot.
    /// # Returns
    /// The explicit scientific scope, preserving observation versus whole-run distinctions.
    #[getter]
    fn scope(&self) -> String {
        format!("{:?}", self.0.scope())
    }
    /// Read profile id from the native Rust snapshot.
    /// # Returns
    /// The original selected ProfileId, distinct from internal bootstrap wire mapping.
    #[getter]
    fn profile_id(&self) -> PyProfileId {
        PyProfileId(self.0.profile_id())
    }
    /// Read compared bits from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn compared_bits(&self) -> PyBitCount {
        PyBitCount(self.0.compared_bits())
    }
    /// Read bit errors from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn bit_errors(&self) -> PyBitCount {
        PyBitCount(self.0.bit_errors())
    }
    /// Read post fec ber from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn post_fec_ber(&self) -> Option<PyBitErrorRatio> {
        self.0.post_fec_ber().map(PyBitErrorRatio)
    }
    /// Read payload rate from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn payload_rate(&self) -> Option<PyPayloadRate> {
        self.0.payload_rate().map(PyPayloadRate)
    }
}
/// Matched-cell MER and power sums from the Rust observer.
#[pyclass(from_py_object, frozen, module = "merlin", name = "CarrierMer")]
#[derive(Clone)]
pub(crate) struct PyCarrierMer(api::CarrierMer);
#[pymethods]
impl PyCarrierMer {
    /// Read subcarrier from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn subcarrier(&self) -> crate::quantities::PySubcarrierIndex {
        crate::quantities::PySubcarrierIndex(self.0.subcarrier())
    }
    /// Read scope from the native Rust snapshot.
    /// # Returns
    /// The explicit scientific scope, preserving observation versus whole-run distinctions.
    #[getter]
    fn scope(&self) -> String {
        format!("{:?}", self.0.scope())
    }
    /// Read samples from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn samples(&self) -> PySampleCount {
        PySampleCount(self.0.samples())
    }
    /// Read signal power from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn signal_power(&self) -> PyLinearPower {
        PyLinearPower(self.0.signal_power())
    }
    /// Read error power from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn error_power(&self) -> PyLinearPower {
        PyLinearPower(self.0.error_power())
    }
    /// Read mer from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn mer(&self) -> Option<crate::quantities::PyMerDb> {
        self.0.mer().map(crate::quantities::PyMerDb)
    }
}
/// Owned native batch; deleting or reconfiguring a controller cannot change it.
#[pyclass(frozen, module = "merlin", name = "BatchReport")]
pub(crate) struct PyBatchReport(pub(crate) Arc<api::BatchReport>);
#[pymethods]
impl PyBatchReport {
    /// Read status from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn status(&self) -> PyBatchStatus {
        self.0.status().into()
    }
    /// Read the number of requested experiment entries, including unstarted entries.
    /// # Returns
    /// The number of requested entries, including not-started entries.
    fn __len__(&self) -> usize {
        self.0.iter().count()
    }
    /// Iterate over the owned native snapshot without borrowing a controller.
    /// # Returns
    /// An iterator retaining the owned native batch independently of its controller.
    fn __iter__(&self) -> PyBatchIterator {
        PyBatchIterator {
            batch: self.0.clone(),
            index: 0,
        }
    }
    /// Export an owned schema-versioned Rust JSON snapshot with scopes and nullability.
    /// # Returns
    /// Owned schema-versioned JSON preserving status, scope, provenance and nullability.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn to_json(&self) -> PyResult<String> {
        self.0.to_json().map_err(error)
    }
    /// Export an independent JSON transport snapshot that cannot mutate native state.
    /// # Returns
    /// An independent JSON-compatible snapshot; caller edits cannot mutate native results.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &self.to_json()?)
    }
    /// Create a new report file, refusing to overwrite an existing destination.
    /// # Arguments
    /// * `path` - Caller-selected input or new output path; existing inputs are never overwritten.
    /// # Returns
    /// None after creating a new destination; an existing destination is unchanged.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn save_json(&self, path: std::path::PathBuf) -> PyResult<()> {
        pnm_input::save_json(path, &self.to_json()?).map_err(error)
    }
}
/// Bounded iterator over owned experiment entries.
#[pyclass(module = "merlin")]
pub(crate) struct PyBatchIterator {
    batch: Arc<api::BatchReport>,
    index: usize,
}
#[pymethods]
impl PyBatchIterator {
    /// Iterate over the owned native snapshot without borrowing a controller.
    /// # Arguments
    /// * `slf` - Owned iterator reference.
    /// # Returns
    /// An iterator retaining the owned native batch independently of its controller.
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    /// Return the next bounded owned entry, or None when iteration is exhausted.
    /// # Returns
    /// The next experiment entry, or None at the end.
    fn __next__(&mut self) -> Option<PyExperimentEntry> {
        if self.index >= self.batch.iter().count() {
            return None;
        }
        let entry = PyExperimentEntry {
            batch: self.batch.clone(),
            index: self.index,
        };
        self.index += 1;
        Some(entry)
    }
}
/// One requested experiment, including an unstarted entry after cancellation.
#[pyclass(frozen, module = "merlin", name = "ExperimentEntry")]
pub(crate) struct PyExperimentEntry {
    batch: Arc<api::BatchReport>,
    index: usize,
}
impl PyExperimentEntry {
    /// Borrow an entry at an internally bounded index in an owned batch.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn entry(&self) -> &api::ExperimentEntry {
        self.batch
            .iter()
            .nth(self.index)
            .expect("private bounded entry index")
    }
}
#[pymethods]
impl PyExperimentEntry {
    /// Read status from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn status(&self) -> PyRunStatus {
        self.entry().status().into()
    }
    /// Read an executed result; unstarted entries remain absent.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn report(&self) -> Option<PyPerformanceReport> {
        self.entry().report().map(|_| PyPerformanceReport {
            batch: self.batch.clone(),
            index: self.index,
        })
    }
    /// Read measurement sources from the native Rust snapshot.
    /// # Returns
    /// Independent contributor, response-selection, epoch and derivation metadata.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn measurement_sources(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(
            py,
            &serde_json::to_string(self.entry().measurement_sources()).unwrap(),
        )
    }
}
/// Read-only performance view that keeps its complete native batch alive.
#[pyclass(frozen, module = "merlin", name = "PerformanceReport")]
pub(crate) struct PyPerformanceReport {
    batch: Arc<api::BatchReport>,
    index: usize,
}
impl PyPerformanceReport {
    /// Read an executed result; unstarted entries remain absent.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn report(&self) -> &api::PerformanceReport {
        self.batch
            .iter()
            .nth(self.index)
            .and_then(|e| e.report())
            .expect("private executed report index")
    }
}
#[pymethods]
impl PyPerformanceReport {
    /// Read status from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn status(&self) -> PyRunStatus {
        self.report().status().into()
    }
    /// Read fec from the native Rust snapshot.
    /// # Returns
    /// Observation-window FEC counters, or None before window establishment.
    #[getter]
    fn fec(&self) -> Option<PyFecReport> {
        self.report().fec().cloned().map(PyFecReport)
    }
    /// Read profile from the native Rust snapshot.
    /// # Returns
    /// Observation-window ProfilePerformance, or None before window establishment.
    #[getter]
    fn profile(&self) -> Option<PyProfilePerformance> {
        self.report().profile().cloned().map(PyProfilePerformance)
    }
    /// Read all completed fec from the native Rust snapshot.
    /// # Returns
    /// Actual whole-run FEC counters, separately labeled AllCompleted.
    #[getter]
    fn all_completed_fec(&self) -> PyFecReport {
        PyFecReport(self.report().all_completed_fec().clone())
    }
    /// Read elapsed from the native Rust snapshot.
    /// # Returns
    /// ElapsedTime in host milliseconds, independent of source observation duration.
    #[getter]
    fn elapsed(&self) -> PyElapsedTime {
        PyElapsedTime(self.report().elapsed())
    }
    /// Read options from the native Rust snapshot.
    /// # Returns
    /// An independent mutable request snapshot; editing it cannot change accepted state.
    #[getter]
    fn options(&self) -> PyEvaluationOptions {
        PyEvaluationOptions::from_native(self.report().options())
    }
    /// Read configuration from the native Rust snapshot.
    /// # Returns
    /// An independent JSON snapshot of resolved values, origins and validated geometry.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn configuration(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(
            py,
            &serde_json::to_string(self.report().configuration()).unwrap(),
        )
    }
    /// Read measurement sources from the native Rust snapshot.
    /// # Returns
    /// Independent contributor, response-selection, epoch and derivation metadata.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn measurement_sources(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(
            py,
            &serde_json::to_string(self.report().measurement_sources()).unwrap(),
        )
    }
    /// Read carrier mer from the native Rust snapshot.
    /// # Returns
    /// An immutable tuple of per-carrier native MER/power observations.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn carrier_mer(&self, py: Python<'_>) -> PyResult<Py<pyo3::types::PyTuple>> {
        Ok(pyo3::types::PyTuple::new(
            py,
            self.report()
                .carrier_mer()
                .iter()
                .cloned()
                .map(PyCarrierMer),
        )?
        .unbind())
    }
    /// Read diagnostics from the native Rust snapshot.
    /// # Returns
    /// An independent scoped native diagnostic snapshot, or None before construction.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn diagnostics(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value: serde_json::Value = serde_json::from_str(&self.to_json()?)
            .map_err(|e| error(api::ApiError::new(api::ErrorCode::Engine, e.to_string())))?;
        object_from_json(py, &value["report"]["diagnostics"].to_string())
    }
    /// Export an owned schema-versioned Rust JSON snapshot with scopes and nullability.
    /// # Returns
    /// Owned schema-versioned JSON preserving status, scope, provenance and nullability.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn to_json(&self) -> PyResult<String> {
        self.report().to_json().map_err(error)
    }
    /// Export an independent JSON transport snapshot that cannot mutate native state.
    /// # Returns
    /// An independent JSON-compatible snapshot; caller edits cannot mutate native results.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &self.to_json()?)
    }
}
/// Registers owned result views and scientific conversion quantities.
/// # Arguments
/// * `module` - Python extension namespace.
/// # Returns
/// Unit after native class registration.
/// # Errors
/// Propagates Python allocation/type registration failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyResponseReal>()?;
    module.add_class::<PyResponseImaginary>()?;
    module.add_class::<PyExperimentId>()?;
    module.add_class::<PyBatchStatus>()?;
    module.add_class::<PyRunStatus>()?;
    module.add_class::<PyBatchReport>()?;
    module.add_class::<PyExperimentEntry>()?;
    module.add_class::<PyPerformanceReport>()?;
    module.add_class::<PyFecReport>()?;
    module.add_class::<PyProfilePerformance>()?;
    module.add_class::<PyCarrierMer>()?;
    module.add_class::<PyCodewordCount>()?;
    module.add_class::<PyBitCount>()?;
    module.add_class::<PySampleCount>()?;
    module.add_class::<PyElapsedTime>()?;
    module.add_class::<PyBitErrorRatio>()?;
    module.add_class::<PyPayloadRate>()?;
    module.add_class::<PyLinearPower>()?;
    Ok(())
}

result_quantity!(
    PyResponseReal,
    ResponseReal,
    f64,
    "ResponseReal",
    "Dimensionless real component of a complex channel response."
);
result_quantity!(
    PyResponseImaginary,
    ResponseImaginary,
    f64,
    "ResponseImaginary",
    "Dimensionless imaginary component of a complex channel response."
);
/// Opaque native experiment identity scoped to its originating controller.
#[pyclass(from_py_object, frozen, module = "merlin", name = "ExperimentId")]
#[derive(Clone)]
pub(crate) struct PyExperimentId(api::ExperimentId);
#[pymethods]
impl PyExperimentEntry {
    /// Read id from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn id(&self) -> PyExperimentId {
        PyExperimentId(self.entry().id())
    }
}

#[pymethods]
impl PyExperimentId {
    /// Format a readable domain identity without enabling unchecked construction.
    /// # Returns
    /// Owned readable diagnostic text; opaque IDs remain unconstructible.
    fn __repr__(&self) -> String {
        format!("ExperimentId({:?})", self.0)
    }
}
