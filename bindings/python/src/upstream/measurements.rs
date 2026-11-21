/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local facade/input ownership and public namespace.
 */
//! Synthetic record conversion and dispatch; every numerical operation is Rust.
use super::{
    api,
    configuration::{bounded, PyPreEq, PyUs},
    error,
};
use pyo3::{prelude::*, types::PyList};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "SyntheticRxmer"
)]
#[derive(Clone)]
pub(crate) struct PyRxmer(pub(crate) api::SyntheticRxmer);
#[pymethods]
impl PyRxmer {
    /// Retains an explicit synthetic full-grid optional-dB input through Rust.
    ///
    /// Returns immutable input; rejects nonfinite/overflow and never calls a decoder.
    #[new]
    fn new(config: PyRef<'_, PyUs>, id: u64, mer_db: &Bound<'_, PyList>) -> PyResult<Self> {
        bounded(mer_db, 4096, "rxmer.readings")?;
        api::SyntheticRxmer::new(&config.0, id, mer_db.extract()?)
            .map(Self)
            .map_err(error)
    }
}
#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "RxmerPlayback"
)]
#[derive(Clone)]
pub(crate) struct PyRxmerPlayback(pub(crate) api::RxmerPlayback);
#[pymethods]
impl PyRxmerPlayback {
    /// Admits explicit synthetic sources and single/average/hold/interpolate policy.
    ///
    /// Returns pure Rust playback; offsets are physical symbols, never capture epochs.
    #[new]
    #[pyo3(signature=(config,sources,policy="single",offsets=None))]
    fn new(
        config: PyRef<'_, PyUs>,
        sources: &Bound<'_, PyList>,
        policy: &str,
        offsets: Option<&Bound<'_, PyList>>,
    ) -> PyResult<Self> {
        bounded(sources, 256, "rxmer.sources")?;
        let sources = sources
            .iter()
            .map(|item| {
                item.extract::<PyRef<'_, PyRxmer>>()
                    .map(|value| value.0.clone())
                    .map_err(PyErr::from)
            })
            .collect::<PyResult<Vec<_>>>()?;
        let offsets = match offsets {
            Some(values) => {
                bounded(values, 256, "rxmer.offsets")?;
                Some(values.extract::<Vec<usize>>()?)
            }
            None => None,
        };
        let policy = match (policy, offsets) {
            ("single", None) => api::RxmerPolicy::Single,
            ("average", None) => api::RxmerPolicy::Average,
            ("hold", Some(offsets)) => api::RxmerPolicy::Hold { offsets },
            ("interpolate", Some(offsets)) => api::RxmerPolicy::Interpolate { offsets },
            _ => {
                return Err(error(api::PhyError::InvalidInput {
                    field: "rxmer.policy",
                }))
            }
        };
        api::RxmerPlayback::new(&config.0, sources, policy)
            .map(Self)
            .map_err(error)
    }
    /// Queries exact derived powers/source weights at one physical symbol.
    ///
    /// Returns JSON without advancing playback or RNG; out-of-range queries reject.
    fn at_json(&self, symbol: usize) -> PyResult<String> {
        let snapshot = self.0.at(symbol).map_err(error)?;
        serde_json::to_string(&snapshot).map_err(|cause| error(api::ApiError::Serialization(cause)))
    }
}
#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ProbeReference"
)]
#[derive(Clone)]
pub(crate) struct PyProbe(api::ProbeReference);
#[pymethods]
impl PyProbe {
    /// Retains explicit synthetic probe ID and corresponding applied base bank ID.
    ///
    /// Returns a candidate; PreEqPlayback validates references and uniqueness.
    #[new]
    fn new(id: u64, bank_id: u64) -> Self {
        Self(api::ProbeReference { id, bank_id })
    }
}
#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "PreEqEvent"
)]
#[derive(Clone)]
pub(crate) struct PyPreEqEvent(api::PreEqEvent);
fn values(input: &Bound<'_, PyList>) -> PyResult<Vec<(usize, api::Iq)>> {
    bounded(input, 4096, "pre_eq.update_values")?;
    let values: Vec<(usize, (f64, f64))> = input.extract()?;
    Ok(values
        .into_iter()
        .map(|(bin, (re, im))| (bin, api::Iq { re, im }))
        .collect())
}
#[pymethods]
impl PyPreEqEvent {
    /// Retains a complete snapshot replacement at an explicit physical symbol.
    ///
    /// Returns a candidate; no multiplication or activation occurs in Python.
    #[staticmethod]
    fn snapshot(id: u64, symbol: usize, bank: PyRef<'_, PyPreEq>) -> Self {
        Self(api::PreEqEvent {
            id,
            symbol,
            operation: api::PreEqOperation::Snapshot(bank.0.clone()),
        })
    }
    /// Retains addressed initialization values with explicit base and probe IDs.
    ///
    /// Returns a candidate; complete-bank derivation and normalization belong to Rust.
    #[staticmethod]
    fn initialize(
        id: u64,
        symbol: usize,
        base_bank: u64,
        probe: u64,
        updates: &Bound<'_, PyList>,
    ) -> PyResult<Self> {
        Ok(Self(api::PreEqEvent {
            id,
            symbol,
            operation: api::PreEqOperation::Initialize {
                base_bank,
                probe,
                values: values(updates)?,
            },
        }))
    }
    /// Retains addressed multiplicative factors with explicit corresponding probe.
    ///
    /// Returns a candidate; ambiguous/future bases reject in shared admission.
    #[staticmethod]
    fn adjust(
        id: u64,
        symbol: usize,
        base_bank: u64,
        probe: u64,
        updates: &Bound<'_, PyList>,
    ) -> PyResult<Self> {
        Ok(Self(api::PreEqEvent {
            id,
            symbol,
            operation: api::PreEqOperation::Adjust {
                base_bank,
                probe,
                values: values(updates)?,
            },
        }))
    }
}
#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "PreEqPlayback"
)]
#[derive(Clone)]
pub(crate) struct PyPreEqPlayback(pub(crate) api::PreEqPlayback);
#[pymethods]
impl PyPreEqPlayback {
    /// Preflights complete synthetic banks, events and explicit probe associations.
    ///
    /// Returns immutable Rust replay; no source advances and failures are atomic.
    #[new]
    fn new(
        config: PyRef<'_, PyUs>,
        initial_id: u64,
        initial: PyRef<'_, PyPreEq>,
        events: &Bound<'_, PyList>,
        probes: &Bound<'_, PyList>,
    ) -> PyResult<Self> {
        bounded(events, 255, "pre_eq.events")?;
        bounded(probes, 256, "pre_eq.probes")?;
        let events = events
            .iter()
            .map(|item| {
                item.extract::<PyRef<'_, PyPreEqEvent>>()
                    .map(|value| value.0.clone())
                    .map_err(PyErr::from)
            })
            .collect::<PyResult<Vec<_>>>()?;
        let probes = probes
            .iter()
            .map(|item| {
                item.extract::<PyRef<'_, PyProbe>>()
                    .map(|value| value.0.clone())
                    .map_err(PyErr::from)
            })
            .collect::<PyResult<Vec<_>>>()?;
        api::PreEqPlayback::new(&config.0, initial_id, initial.0.clone(), events, probes)
            .map(Self)
            .map_err(error)
    }
    /// Queries the full effective coefficients through the Rust schedule owner.
    ///
    /// Returns owned `(I,Q)` values; querying cannot advance a running experiment.
    fn coefficients(&self, symbol: usize) -> PyResult<Vec<(f64, f64)>> {
        Ok(self
            .0
            .at(symbol)
            .map_err(error)?
            .coefficients()
            .iter()
            .map(|z| (z.re, z.im))
            .collect())
    }
}
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyRxmer>()?;
    module.add_class::<PyRxmerPlayback>()?;
    module.add_class::<PyProbe>()?;
    module.add_class::<PyPreEqEvent>()?;
    module.add_class::<PyPreEqPlayback>()?;
    Ok(())
}
