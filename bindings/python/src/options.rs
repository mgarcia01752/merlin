/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Copied request objects and opaque controller-scoped identities.
use crate::{api, quantities::*, receiver::PyReceiverMode};
use pyo3::prelude::*;
/// Opaque RxMER identity; no integer construction or unscoped fallback.
#[pyclass(from_py_object, frozen, module = "merlin", name = "RxMerId")]
#[derive(Clone)]
pub(crate) struct PyRxMerId(pub(crate) api::RxMerId);
/// Opaque response identity returned by one native controller.
#[pyclass(from_py_object, frozen, module = "merlin", name = "ChannelEstimateId")]
#[derive(Clone)]
pub(crate) struct PyChannelId(pub(crate) api::ChannelEstimateId);
/// Static Rust selection policy; no Python power or time calculations.
#[pyclass(from_py_object, frozen, module = "merlin", name = "RxMerSelection")]
#[derive(Clone)]
pub(crate) struct PyRxMerSelection(pub(crate) api::RxMerSelection);
#[pymethods]
impl PyRxMerSelection {
    /// Select independent experiments over original captures in insertion order.
    /// # Returns
    /// A typed policy requesting independent experiments in insertion order.
    #[staticmethod]
    fn each() -> Self {
        Self(api::RxMerSelection::Each)
    }
    /// Select one original RxMER by its controller-scoped identifier.
    /// # Arguments
    /// * `capture_id` - Opaque identifier returned by this controller.
    /// # Returns
    /// A typed policy selecting only the specified original observation.
    #[staticmethod]
    fn individual(capture_id: &PyRxMerId) -> Self {
        Self(api::RxMerSelection::Individual(capture_id.0))
    }
    /// Select one equal-capture average in residual power, computed entirely in Rust.
    /// # Returns
    /// A typed equal-capture residual-power averaging policy.
    #[staticmethod]
    fn average_all() -> Self {
        Self(api::RxMerSelection::AverageAll)
    }
    /// Select one static observation by native residual-power interpolation in capture time.
    /// # Arguments
    /// * `target` - Native domain value or owned conversion data.
    /// # Returns
    /// A typed static time-selection policy; Rust rejects extrapolation and ambiguity.
    #[staticmethod]
    fn interpolate_at(target: &PyCaptureTime) -> Self {
        Self(api::RxMerSelection::InterpolateAt(target.0))
    }
}
/// Explicit response policy, independent of RxMER selection.
#[pyclass(from_py_object, frozen, module = "merlin", name = "ChannelSelection")]
#[derive(Clone)]
pub(crate) struct PyChannelSelection(pub(crate) api::ChannelSelection);
#[pymethods]
impl PyChannelSelection {
    /// Choose an explicitly synthetic unity response; no measured response is implied.
    /// # Returns
    /// An explicit synthetic unity-response policy.
    #[staticmethod]
    fn unity() -> Self {
        Self(api::ChannelSelection::Unity)
    }
    /// Choose one original response for explicit reuse across the selected experiments.
    /// # Arguments
    /// * `capture_id` - Opaque identifier returned by this controller.
    /// # Returns
    /// An explicit original-response reuse policy.
    #[staticmethod]
    fn explicit(capture_id: &PyChannelId) -> Self {
        Self(api::ChannelSelection::Explicit(capture_id.0))
    }
    /// Choose unique nearest-time matching within an explicit Rust-validated tolerance.
    /// # Arguments
    /// * `tolerance` - Native domain value or owned conversion data.
    /// # Returns
    /// A native nearest-time policy with explicit seconds tolerance.
    #[staticmethod]
    fn nearest_within(tolerance: &PyAssociationTolerance) -> Self {
        Self(api::ChannelSelection::NearestWithin(tolerance.0))
    }
}
/// Finite execution and retained-detail limits, copied when options are accepted.
///
/// Defaults: RuntimeLimit(30000) milliseconds of host execution,
/// IterationLimit(50) and RetainedWordLimit(64). Host runtime is independent
/// of ObservationDuration source time. Runtime exhaustion preserves partial results.
#[pyclass(from_py_object, frozen, module = "merlin", name = "RunLimits")]
#[derive(Clone)]
pub(crate) struct PyRunLimits(std::sync::Arc<std::sync::Mutex<api::RunLimits>>);
impl PyRunLimits {
    /// Create an independent Python conversion snapshot of validated Rust values.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn from_native(value: api::RunLimits) -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(value)))
    }
    /// Copy conversion fields into existing Rust domain models; perform no scientific calculations.
    /// # Returns
    /// The immutable native value named by this read-only property.
    fn native(&self) -> api::RunLimits {
        self.0.lock().expect("private limits request lock").clone()
    }
}
#[pymethods]
impl PyRunLimits {
    /// Construct a Rust-backed value or request using the declared units and defaults.
    /// # Arguments
    /// * `runtime` - Validated host runtime limit in milliseconds.
    /// * `retained_words` - Validated maximum retained detailed words.
    /// * `iterations` - Validated maximum decoder iterations.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    #[new]
    #[pyo3(signature=(*,runtime=None,retained_words=None,iterations=None))]
    fn new(
        runtime: Option<PyRuntimeLimit>,
        retained_words: Option<PyRetainedWordLimit>,
        iterations: Option<PyIterationLimit>,
    ) -> Self {
        let mut value = api::RunLimits::default();
        if let Some(runtime) = runtime {
            value.runtime = runtime.0;
        }
        if let Some(retained_words) = retained_words {
            value.retained_words = retained_words.0;
        }
        if let Some(iterations) = iterations {
            value.iterations = iterations.0;
        }
        Self::from_native(value)
    }
    /// Read runtime from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn runtime(&self) -> PyRuntimeLimit {
        PyRuntimeLimit(self.native().runtime)
    }
    /// Assign the typed request field atomically.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// None after accepting the typed request.
    #[setter]
    fn set_runtime(&self, value: PyRuntimeLimit) {
        self.0.lock().expect("private limits request lock").runtime = value.0;
    }
    /// Read retained words from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn retained_words(&self) -> PyRetainedWordLimit {
        PyRetainedWordLimit(self.native().retained_words)
    }
    /// Assign the typed request field atomically.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// None after accepting the typed request.
    #[setter]
    fn set_retained_words(&self, value: PyRetainedWordLimit) {
        self.0
            .lock()
            .expect("private limits request lock")
            .retained_words = value.0;
    }
    /// Read iterations from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    #[getter]
    fn iterations(&self) -> PyIterationLimit {
        PyIterationLimit(self.native().iterations)
    }
    /// Assign the typed request field atomically.
    /// # Arguments
    /// * `value` - Value in the declared domain units, or an already validated Rust-backed quantity.
    /// # Returns
    /// None after accepting the typed request.
    #[setter]
    fn set_iterations(&self, value: PyIterationLimit) {
        self.0
            .lock()
            .expect("private limits request lock")
            .iterations = value.0;
    }
}
/// Mutable simulation request; set_options accepts a validated independent copy.
///
/// Defaults: Each RxMER, unset channel selection, SuppliedChannel receiver,
/// 100 ms source observation, noise seed 41, PRBS seed 0x7fff and zero-second
/// profile tolerance. Select a channel explicitly before run. RunLimits defaults
/// to 30,000 ms host runtime, 50 iterations and 64 retained detailed words.
/// Scientific quantities require their named Rust-validated wrapper types;
/// Python None selects the native default, not a fabricated zero.
#[pyclass(from_py_object, module = "merlin", name = "EvaluationOptions")]
#[derive(Clone)]
pub(crate) struct PyEvaluationOptions {
    /// RxMerSelection: static Each, Individual, AverageAll or InterpolateAt policy.
    #[pyo3(get, set)]
    rxmer_selection: PyRxMerSelection,
    /// ChannelSelection or None: explicit response, nearest-time or unity; None fails run preflight.
    #[pyo3(get, set)]
    channel_selection: Option<PyChannelSelection>,
    /// ReceiverMode: correction policy independent of the applied channel response.
    #[pyo3(get, set)]
    receiver_mode: PyReceiverMode,
    processing: api::ProcessingOptions,
    /// ObservationDuration: source observation in milliseconds, excluding acquisition/drain.
    #[pyo3(get, set)]
    observation: PyObservationDuration,
    /// NoiseSeed: reproducible noise generator seed.
    #[pyo3(get, set)]
    noise_seed: PyNoiseSeed,
    /// PrbsSeed: nonzero fifteen-bit PRBS source seed.
    #[pyo3(get, set)]
    prbs_seed: PyPrbsSeed,
    /// AssociationTolerance: maximum profile/capture epoch separation in seconds.
    #[pyo3(get, set)]
    profile_tolerance: PyAssociationTolerance,
    /// RunLimits: finite host runtime, decoder iterations and retained detail budget.
    #[pyo3(get, set)]
    limits: PyRunLimits,
}
impl PyEvaluationOptions {
    /// Copy conversion fields into existing Rust domain models; perform no scientific calculations.
    /// # Returns
    /// The immutable native value named by this read-only property.
    pub(crate) fn native(&self) -> api::EvaluationOptions {
        api::EvaluationOptions {
            rxmer_selection: self.rxmer_selection.0,
            channel_selection: self.channel_selection.as_ref().map(|v| v.0),
            receiver_mode: self.receiver_mode.native(),
            processing: self.processing.clone(),
            observation: self.observation.0,
            noise_seed: self.noise_seed.0,
            prbs_seed: self.prbs_seed.0,
            profile_tolerance: self.profile_tolerance.0,
            limits: self.limits.native(),
        }
    }
    /// Create an independent Python conversion snapshot of validated Rust values.
    /// # Returns
    /// The immutable native value named by this read-only property.
    pub(crate) fn from_native(value: &api::EvaluationOptions) -> Self {
        Self {
            rxmer_selection: PyRxMerSelection(value.rxmer_selection),
            channel_selection: value.channel_selection.map(PyChannelSelection),
            receiver_mode: value.receiver_mode.into(),
            processing: value.processing.clone(),
            observation: PyObservationDuration(value.observation),
            noise_seed: PyNoiseSeed(value.noise_seed),
            prbs_seed: PyPrbsSeed(value.prbs_seed),
            profile_tolerance: PyAssociationTolerance(value.profile_tolerance),
            limits: PyRunLimits::from_native(value.limits.clone()),
        }
    }
}
#[pymethods]
impl PyEvaluationOptions {
    /// Returns an independent dictionary of existing MCE/filter/loss/power settings.
    /// # Returns
    /// Copied settings for inspection; editing the dictionary does not mutate options.
    /// # Errors
    /// Propagates serialization or Python allocation failures.
    #[getter]
    fn processing(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        crate::object_from_json(
            py,
            &serde_json::to_string(&self.processing).map_err(|e| {
                crate::error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    e.to_string(),
                ))
            })?,
        )
    }
    /// Construct a Rust-backed value or request using the declared units and defaults.
    /// # Arguments
    /// * `rxmer_selection` - Typed static selection policy; None requests the Rust default Each.
    /// * `channel_selection` - Explicit typed response policy; None remains unset and fails preflight.
    /// * `receiver_mode` - Typed correction source; None retains supplied-channel default.
    /// * `observation` - Validated requested source duration, separate from host runtime.
    /// * `noise_seed` - Validated reproducible noise RNG seed.
    /// * `prbs_seed` - Validated nonzero fifteen-bit PRBS seed.
    /// * `profile_tolerance` - Independent explicit profile-time tolerance in seconds.
    /// * `limits` - Typed finite runtime, iteration and retained-detail limits.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    #[new]
    #[pyo3(signature=(*,rxmer_selection=None,channel_selection=None,receiver_mode=None,observation=None,noise_seed=None,prbs_seed=None,profile_tolerance=None,limits=None))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        rxmer_selection: Option<PyRxMerSelection>,
        channel_selection: Option<PyChannelSelection>,
        receiver_mode: Option<PyReceiverMode>,
        observation: Option<PyObservationDuration>,
        noise_seed: Option<PyNoiseSeed>,
        prbs_seed: Option<PyPrbsSeed>,
        profile_tolerance: Option<PyAssociationTolerance>,
        limits: Option<PyRunLimits>,
    ) -> Self {
        let mut result = Self::from_native(&api::EvaluationOptions::default());
        if let Some(value) = rxmer_selection {
            result.rxmer_selection = value;
        }
        result.channel_selection = channel_selection;
        if let Some(value) = receiver_mode {
            result.receiver_mode = value;
        }
        if let Some(value) = observation {
            result.observation = value;
        }
        if let Some(value) = noise_seed {
            result.noise_seed = value;
        }
        if let Some(value) = prbs_seed {
            result.prbs_seed = value;
        }
        if let Some(value) = profile_tolerance {
            result.profile_tolerance = value;
        }
        if let Some(value) = limits {
            result.limits = value;
        }
        result
    }
}
/// Registers native request and opaque identity types.
/// # Arguments
/// * `module` - Python extension namespace.
/// # Returns
/// Unit after registration.
/// # Errors
/// Propagates Python type allocation errors.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyRxMerId>()?;
    module.add_class::<PyChannelId>()?;
    module.add_class::<PyRxMerSelection>()?;
    module.add_class::<PyChannelSelection>()?;
    module.add_class::<PyRunLimits>()?;
    module.add_class::<PyEvaluationOptions>()?;
    Ok(())
}
