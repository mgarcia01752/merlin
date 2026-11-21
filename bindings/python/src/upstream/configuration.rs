/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local facade/input ownership and public namespace.
 */
//! Typed Python arguments converted into the existing Rust domain constructors.
use super::{api, error};
use pyo3::{
    prelude::*,
    types::{PyBytes, PyList},
};

pub(crate) fn bounded(
    list: &Bound<'_, PyList>,
    maximum: usize,
    field: &'static str,
) -> PyResult<()> {
    if list.len() > maximum {
        return Err(error(api::PhyError::ResourceLimit { field }));
    }
    Ok(())
}

fn mode(spacing_khz: u32) -> PyResult<api::Numerology> {
    match spacing_khz {
        50 => Ok(api::Numerology::Khz50),
        25 => Ok(api::Numerology::Khz25),
        _ => Err(error(api::PhyError::Unsupported {
            field: "waveform.spacing_khz",
        })),
    }
}

pub(crate) fn tap(name: &str) -> PyResult<api::Tap> {
    match name {
        "TxGrid" => Ok(api::Tap::TxGrid),
        "TxPreEq" => Ok(api::Tap::TxPreEq),
        "TxUseful" => Ok(api::Tap::TxUseful),
        "TxOutput" => Ok(api::Tap::TxOutput),
        "ChannelInput" => Ok(api::Tap::ChannelInput),
        "ChannelOutput" => Ok(api::Tap::ChannelOutput),
        "RxInput" => Ok(api::Tap::RxInput),
        "RxFft" => Ok(api::Tap::RxFft),
        "RxEqualized" => Ok(api::Tap::RxEqualized),
        _ => Err(error(api::PhyError::Unsupported {
            field: "observations.tap",
        })),
    }
}

#[pyclass(from_py_object, frozen, module = "merlin.upstream", name = "Waveform")]
#[derive(Clone)]
pub(crate) struct PyWaveform(pub(crate) api::Waveform);
#[pymethods]
impl PyWaveform {
    /// Admits exact spacing kHz, CP/RP sample counts, frame K and encompassed Hz.
    ///
    /// Returns immutable Rust geometry; conversion/domain errors reject input.
    #[new]
    fn new(
        spacing_khz: u32,
        prefix: usize,
        rolloff: usize,
        frame_symbols: usize,
        bandwidth_hz: u32,
    ) -> PyResult<Self> {
        api::Waveform::new(
            mode(spacing_khz)?,
            prefix,
            rolloff,
            frame_symbols,
            bandwidth_hz,
        )
        .map(Self)
        .map_err(error)
    }
}

#[pyclass(from_py_object, frozen, module = "merlin.upstream", name = "Minislot")]
#[derive(Clone)]
pub(crate) struct PyMinislot(pub(crate) api::Minislot);
#[pymethods]
impl PyMinislot {
    /// Retains physical first carrier, data loading and pilot-pattern candidate.
    ///
    /// Returns a typed candidate; Profile/UsConfiguration own shared admission.
    #[new]
    fn new(first_carrier: usize, loading: u8, pilot_pattern: u8) -> Self {
        Self(api::Minislot {
            first_carrier,
            loading,
            pilot_pattern,
        })
    }
}

#[pyclass(from_py_object, frozen, module = "merlin.upstream", name = "Profile")]
#[derive(Clone)]
pub(crate) struct PyProfile(pub(crate) api::Profile);
#[pymethods]
impl PyProfile {
    /// Admits explicit ID, IUC, raw 23-bit seed and ordered physical minislots.
    ///
    /// Returns immutable native profile; no fixed IUC/QAM/LDPC mapping is added.
    #[new]
    fn new(id: u16, iuc: u8, seed: u32, minislots: &Bound<'_, PyList>) -> PyResult<Self> {
        bounded(minislots, 256, "profile.minislots")?;
        let minislots = minislots
            .iter()
            .map(|item| {
                item.extract::<PyRef<'_, PyMinislot>>()
                    .map(|slot| slot.0.clone())
                    .map_err(PyErr::from)
            })
            .collect::<PyResult<Vec<_>>>()?;
        api::Profile::new(id, iuc, seed, minislots)
            .map(Self)
            .map_err(error)
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "SyntheticGrant"
)]
#[derive(Clone)]
pub(crate) struct PyGrant(pub(crate) api::SyntheticGrant);
#[pymethods]
impl PyGrant {
    /// Retains finite grant IDs, configured ticks/frames and explicit slot indices.
    ///
    /// Returns a candidate; UsConfiguration validates budgets/associations.
    #[new]
    fn new(
        id: u64,
        profile_id: u16,
        epoch_sample: u64,
        first_frame: u64,
        frames: usize,
        minislots: &Bound<'_, PyList>,
    ) -> PyResult<Self> {
        bounded(minislots, 256, "grant.minislots")?;
        Ok(Self(api::SyntheticGrant {
            id,
            profile_id,
            epoch_sample,
            first_frame,
            frames,
            minislots: minislots.extract()?,
        }))
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "UsConfiguration"
)]
#[derive(Clone)]
pub(crate) struct PyUs(pub(crate) api::UsConfiguration);
#[pymethods]
impl PyUs {
    /// Admits complete typed waveform/inventory/grant and excluded physical bins.
    ///
    /// Returns immutable Rust settings; failed admission has no session side effect.
    #[new]
    #[pyo3(signature=(waveform,profiles,grant,excluded=None))]
    fn new(
        waveform: PyRef<'_, PyWaveform>,
        profiles: &Bound<'_, PyList>,
        grant: PyRef<'_, PyGrant>,
        excluded: Option<&Bound<'_, PyList>>,
    ) -> PyResult<Self> {
        bounded(profiles, 256, "configuration.profiles")?;
        let profiles = profiles
            .iter()
            .map(|item| {
                item.extract::<PyRef<'_, PyProfile>>()
                    .map(|profile| profile.0.clone())
                    .map_err(PyErr::from)
            })
            .collect::<PyResult<Vec<_>>>()?;
        let excluded = match excluded {
            Some(values) => {
                bounded(values, 4096, "configuration.excluded")?;
                values.extract()?
            }
            None => Vec::new(),
        };
        api::UsConfiguration::new(waveform.0.clone(), profiles, grant.0.clone(), excluded)
            .map(Self)
            .map_err(error)
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ReceiverConfig"
)]
#[derive(Clone)]
pub(crate) struct PyReceiver(pub(crate) api::ReceiverConfig);
#[pymethods]
impl PyReceiver {
    /// Admits configured-timing estimator, reference variance, floor and iterations.
    ///
    /// Returns shared Rust settings; estimator names select explicit policies only.
    #[new]
    #[pyo3(signature=(estimator="frame_flat_ls",variance=0.02,power_floor=1e-12,iterations=50,maximum_gap=64))]
    fn new(
        estimator: &str,
        variance: f64,
        power_floor: f64,
        iterations: usize,
        maximum_gap: usize,
    ) -> PyResult<Self> {
        let estimator = match estimator {
            "frame_flat_ls" => api::Estimator::FrameFlatLs,
            "frame_pilot_linear" => api::Estimator::FramePilotLinear { maximum_gap },
            _ => {
                return Err(error(api::PhyError::Unsupported {
                    field: "receiver.estimator",
                }))
            }
        };
        api::ReceiverConfig::new(estimator, variance, power_floor, iterations)
            .map(Self)
            .map_err(error)
    }
}

#[pyclass(from_py_object, frozen, module = "merlin.upstream", name = "PreEqBank")]
#[derive(Clone)]
pub(crate) struct PyPreEq(pub(crate) api::PreEqBank);
#[pymethods]
impl PyPreEq {
    /// Constructs normative unity/excluded-zero defaults using shared Rust code.
    ///
    /// Returns an immutable bank; never creates an injected channel estimate.
    #[staticmethod]
    fn unity(config: PyRef<'_, PyUs>) -> Self {
        Self(api::PreEqBank::unity(&config.0))
    }
    /// Borrows no Rust memory: returns owned effective `(I,Q)` coefficient pairs.
    ///
    /// Excluded zeros and unused physical indices remain in their original places.
    fn coefficients(&self) -> Vec<(f64, f64)> {
        self.0.coefficients().iter().map(|z| (z.re, z.im)).collect()
    }
    /// Returns the actual immutable Rust bank generation without changing state.
    ///
    /// This identity is independent from a receiver estimate or injected H.
    #[getter]
    fn generation(&self) -> u64 {
        self.0.generation()
    }
    /// Admits complete raw `(I,Q)` coefficients and a nonzero bank generation.
    ///
    /// Returns the Rust-normalized bank; Python performs no coefficient arithmetic.
    #[staticmethod]
    fn normalized(
        config: PyRef<'_, PyUs>,
        coefficients: &Bound<'_, PyList>,
        generation: u64,
    ) -> PyResult<Self> {
        bounded(coefficients, 4096, "pre_eq.coefficients")?;
        let coefficients: Vec<(f64, f64)> = coefficients.extract()?;
        api::PreEqBank::normalized(
            &config.0,
            coefficients
                .into_iter()
                .map(|(re, im)| api::Iq { re, im })
                .collect(),
            generation,
        )
        .map(Self)
        .map_err(error)
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ChannelConfig"
)]
#[derive(Clone)]
pub(crate) struct PyChannel(pub(crate) api::ChannelConfig);
#[pymethods]
impl PyChannel {
    /// Admits explicit synthetic FIR/noise/integer-delay/phase settings in Rust.
    ///
    /// Returns immutable configuration; zero settings select exact unity bypass.
    #[new]
    #[pyo3(signature=(taps=None,noise_variance=0.0,noise_seed=None,delay_samples=0,phase_radians=0.0,frequency_offset_hz=0.0))]
    fn new(
        taps: Option<&Bound<'_, PyList>>,
        noise_variance: f64,
        noise_seed: Option<&Bound<'_, PyBytes>>,
        delay_samples: usize,
        phase_radians: f64,
        frequency_offset_hz: f64,
    ) -> PyResult<Self> {
        let mut config = api::ChannelConfig {
            noise_variance,
            noise_seed: match noise_seed {
                Some(seed) => seed.as_bytes().try_into().map_err(|_| {
                    error(api::PhyError::InvalidInput {
                        field: "channel.noise_seed",
                    })
                })?,
                None => [1; 32],
            },
            delay_samples,
            phase_radians,
            frequency_offset_hz,
            ..api::ChannelConfig::default()
        };
        if let Some(taps) = taps {
            bounded(taps, 256, "channel.taps")?;
            let taps: Vec<(f64, f64)> = taps.extract()?;
            config.taps = taps
                .into_iter()
                .map(|(re, im)| api::Iq { re, im })
                .collect();
        }
        config.validate().map_err(error)?;
        Ok(Self(config))
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ExperimentConfig"
)]
#[derive(Clone)]
pub(crate) struct PyConfig(pub(crate) api::ExperimentConfig);
#[pymethods]
impl PyConfig {
    /// Composes typed US settings, source seed and optional receiver/C/H policies.
    ///
    /// Returns the same Rust candidate admitted by Simulation.configure.
    /// Construction remains idle and rejects shared domain failures.
    #[new]
    // Named Python constructor arguments translate directly into one Rust request.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature=(us_configuration,source_seed,receiver=None,pre_eq=None,channel=None,rxmer=None,pre_eq_updates=None,leading_guard_samples=0,trailing_guard_samples=0))]
    fn new(
        us_configuration: PyRef<'_, PyUs>,
        source_seed: u16,
        receiver: Option<PyRef<'_, PyReceiver>>,
        pre_eq: Option<PyRef<'_, PyPreEq>>,
        channel: Option<PyRef<'_, PyChannel>>,
        rxmer: Option<PyRef<'_, super::measurements::PyRxmerPlayback>>,
        pre_eq_updates: Option<PyRef<'_, super::measurements::PyPreEqPlayback>>,
        leading_guard_samples: usize,
        trailing_guard_samples: usize,
    ) -> PyResult<Self> {
        let receiver = match receiver {
            Some(value) => value.0.clone(),
            None => api::ReceiverConfig::new(api::Estimator::FrameFlatLs, 0.02, 1e-12, 50)
                .map_err(error)?,
        };
        let mut config =
            api::ExperimentConfig::new(us_configuration.0.clone(), source_seed, receiver)
                .map_err(error)?;
        if let Some(bank) = pre_eq {
            config.pre_eq = bank.0.clone();
        }
        if let Some(channel) = channel {
            config.channel = channel.0.clone();
        }
        config.guards = api::GuardIntervals {
            leading: leading_guard_samples,
            trailing: trailing_guard_samples,
        };
        config.rxmer = rxmer.map(|value| value.0.clone());
        config.pre_eq_updates = pre_eq_updates.map(|value| value.0.clone());
        config.validate().map_err(error)?;
        Ok(Self(config))
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ExperimentOptions"
)]
#[derive(Clone)]
pub(crate) struct PyOptions(pub(crate) api::ExperimentOptions);
#[pymethods]
impl PyOptions {
    /// Admits finite counts, milliseconds, sample chunks and selected passive taps.
    ///
    /// Returns shared Rust options; negative/overflowing integers reject before
    /// narrowing, and unknown taps never fall back to another reference plane.
    #[new]
    #[pyo3(signature=(grants=1,maximum_samples=16_777_216,chunk_samples=4096,timeout_ms=30_000,retained_points=4096,retained_taps=None))]
    fn new(
        grants: usize,
        maximum_samples: usize,
        chunk_samples: usize,
        timeout_ms: u64,
        retained_points: usize,
        retained_taps: Option<&Bound<'_, PyList>>,
    ) -> PyResult<Self> {
        let mut options = api::ExperimentOptions {
            grants,
            maximum_samples,
            chunk_samples,
            timeout_ms,
            retained_points,
            ..api::ExperimentOptions::default()
        };
        if let Some(taps) = retained_taps {
            bounded(taps, 9, "observations.taps")?;
            options.retained_taps = taps
                .iter()
                .map(|name| tap(name.extract::<&str>()?))
                .collect::<PyResult<_>>()?;
        }
        options.validate().map_err(error)?;
        Ok(Self(options))
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyWaveform>()?;
    module.add_class::<PyMinislot>()?;
    module.add_class::<PyProfile>()?;
    module.add_class::<PyGrant>()?;
    module.add_class::<PyUs>()?;
    module.add_class::<PyReceiver>()?;
    module.add_class::<PyPreEq>()?;
    module.add_class::<PyChannel>()?;
    module.add_class::<PyConfig>()?;
    module.add_class::<PyOptions>()?;
    Ok(())
}
