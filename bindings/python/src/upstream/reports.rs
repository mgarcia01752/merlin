/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local facade/input ownership and public namespace.
 */
//! Owned native report/query values; no Python computation or engine replay.
mod recovery;
mod response;
use super::{api, configuration::tap, error};
use api::ReportJson;
use pyo3::prelude::*;

#[pyclass(frozen, module = "merlin.upstream", name = "ExperimentPreview")]
pub(crate) struct PyPreview(pub(crate) api::ExperimentPreview);
#[pymethods]
impl PyPreview {
    /// Returns every retained profile ID discovered by the Rust inventory.
    ///
    /// Values are owned copies; this does not select or execute profiles.
    #[getter]
    fn profile_ids(&self) -> Vec<u16> {
        self.0.profile_ids.clone()
    }
    /// Returns cumulative planned meaningful source count, excluding fill/parity.
    ///
    /// This is a pure plan, not an observed recovery count.
    #[getter]
    fn source_bits(&self) -> usize {
        self.0.source_bits
    }
    /// Returns complete owned preview JSON using the Rust serializer.
    ///
    /// This does not start simulation or change session settings.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(|cause| error(api::ApiError::Serialization(cause)))
    }
}

#[pyclass(frozen, module = "merlin.upstream", name = "QamSnapshot")]
pub(crate) struct PyQam(pub(crate) api::QamSnapshot);
#[pymethods]
impl PyQam {
    /// Returns actual retained I/Q values at the selected stage reference plane.
    ///
    /// Coordinates and roles remain in to_json(); no ideal points replace data.
    #[getter]
    fn values(&self) -> Vec<(f64, f64)> {
        self.0
            .points
            .iter()
            .map(|point| (point.value.re, point.value.im))
            .collect()
    }
    /// Returns the owned complete Rust snapshot including matched physical metadata.
    ///
    /// Serialization/querying cannot rerun science or consume RNG.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(|cause| error(api::ApiError::Serialization(cause)))
    }
}

#[pyclass(frozen, module = "merlin.upstream", name = "SpectrumSnapshot")]
pub(crate) struct PySpectrum(pub(crate) api::SpectrumSnapshot);
#[pymethods]
impl PySpectrum {
    /// Exports all actual spectrum identity, window, sample-rate and unit metadata.
    ///
    /// Returns owned JSON from the immutable Rust result, without regenerating samples.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(|cause| error(api::ApiError::Serialization(cause)))
    }
    /// Returns centered baseband frequencies in hertz, as an owned native copy.
    ///
    /// These are not inferred absolute RF coordinates.
    #[getter]
    fn frequency_hz(&self) -> Vec<f64> {
        self.0.frequency_hz.clone()
    }
    /// Returns linear normalized two-sided power/Hz, including exact zeros.
    ///
    /// No Python scaling, logarithm or synthetic noise floor is applied.
    #[getter]
    fn power_per_hz(&self) -> Vec<f64> {
        self.0.power_per_hz.clone()
    }
    /// Returns original absolute first-sample tick of the retained FFT window.
    ///
    /// This query does not reinterpret a packed useful grid as continuous samples.
    #[getter]
    fn first_sample(&self) -> u64 {
        self.0.first_sample
    }
}

#[pyclass(frozen, module = "merlin.upstream", name = "BatchReport")]
pub(crate) struct PyBatchReport(pub(crate) api::BatchReport);
#[pymethods]
impl PyBatchReport {
    /// Returns the actual terminal lifecycle state separately from source recovery.
    ///
    /// Failed/cancelled runs remain inspectable owned reports.
    #[getter]
    fn status(&self) -> String {
        format!("{:?}", self.0.status)
    }
    /// Returns true only for complete nonempty source/FEC/fill/coverage acceptance.
    ///
    /// Zero observed errors alone is insufficient.
    #[getter]
    fn reference_recovered(&self) -> bool {
        self.0.reference_recovered
    }
    /// Returns actual source Hamming distance, including failed decoder decisions.
    ///
    /// This is independent of parity validity or convergence.
    #[getter]
    fn bit_errors(&self) -> usize {
        self.0.bit_errors
    }
    /// Returns observed BER, or None when no source decisions were compared.
    ///
    /// Unavailable does not become zero, NaN or infinity.
    #[getter]
    fn ber(&self) -> Option<f64> {
        self.0.ber
    }
    /// Returns actual source decisions compared, excluding unverified generation.
    ///
    /// Cancellation can preserve a proper prefix of the originally planned count.
    #[getter]
    fn compared_source_bits(&self) -> usize {
        self.0.compared_source_bits
    }
    /// Returns actual source generated, independent of emission/comparison coverage.
    ///
    /// Owned prior reports survive all subsequent session calls.
    #[getter]
    fn generated_source_bits(&self) -> usize {
        self.0.generated_source_bits
    }
    /// Serializes bounded complete/partial Rust science without opening a path.
    ///
    /// Returns versioned JSON with actual failed decisions/iterations/provenance.
    fn to_json(&self) -> PyResult<String> {
        self.0.to_json().map_err(error)
    }
    /// Queries retained actual constellation data using shared Rust validation.
    ///
    /// `maximum` bounds returned points; unavailable taps remain typed failures.
    #[pyo3(signature=(tap_name,loading=None,maximum=4096))]
    fn qam(&self, tap_name: &str, loading: Option<u8>, maximum: usize) -> PyResult<PyQam> {
        self.0
            .qam(tap(tap_name)?, loading, maximum)
            .map(PyQam)
            .map_err(error)
    }
    /// Computes Rust PSD from retained continuous actual samples at `tap_name`.
    ///
    /// `spacing_khz` selects a bounded 2K/4K analysis; no simulation is replayed.
    fn spectrum(&self, tap_name: &str, spacing_khz: u32) -> PyResult<PySpectrum> {
        let mode = match spacing_khz {
            50 => api::Numerology::Khz50,
            25 => api::Numerology::Khz25,
            _ => {
                return Err(error(api::PhyError::Unsupported {
                    field: "spectrum.spacing_khz",
                }))
            }
        };
        self.0
            .spectrum(tap(tap_name)?, mode)
            .map(PySpectrum)
            .map_err(error)
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyPreview>()?;
    module.add_class::<PyBatchReport>()?;
    module.add_class::<PyQam>()?;
    module.add_class::<PySpectrum>()?;
    Ok(())
}
