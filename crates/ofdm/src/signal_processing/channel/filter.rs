/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded causal complex channels, with explicit reconstruction policies.
use crate::{
    signal_processing::waveform::{fft::ForwardTransform, ifft::InverseTransform, SampleBlock},
    transmission::SubcarrierSpacing,
    types::IqPair,
    SimulationError,
};
use serde::{Deserialize, Serialize};

/// Maximum memory is ten microseconds at the fixed 204.8 MHz sample clock.
pub const MAX_FILTER_DELAY_SAMPLES: usize = 2048;
/// Maximum independently specified paths; the first must be the direct path.
pub const MAX_ECHO_PATHS: usize = 32;
/// Complex path gain at an integer sample delay; no inferred physical cable length.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EchoPath {
    pub delay_samples: usize,
    pub gain: IqPair,
}
/// Explicit channel-model selection, independent of receiver correction mode.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChannelFilterSettings {
    #[default]
    Diagonal,
    /// Synthetic paths only; imported response/tilt composition is rejected.
    Echoes { paths: Vec<EchoPath> },
    /// Explicit unity extension of excluded bins, causal shift and hard truncation.
    /// This is a chosen band-limited reconstruction, not recovered physical truth.
    CapturedFir {
        max_delay_samples: usize,
        causal_shift_samples: usize,
        maximum_rms_error: f64,
        maximum_bin_error: f64,
    },
}
/// Reproducible fit diagnostics; original captures remain unchanged.
#[derive(Debug, Clone, Serialize)]
pub struct FilterDiagnostics {
    pub taps: usize,
    pub causal_shift_samples: usize,
    pub rms_complex_error: f64,
    pub maximum_complex_error: f64,
    pub discarded_energy_fraction: f64,
}
/// Owned frozen filter plan, separately inspectable before constructing a stream.
pub struct ChannelFilterPlan {
    coefficients: Vec<IqPair>,
    response: Vec<IqPair>,
    diagnostics: FilterDiagnostics,
}
fn fail(message: &str) -> SimulationError {
    SimulationError::new("ChannelFilterInvalid", message)
}
fn multiply(left: IqPair, right: IqPair) -> IqPair {
    [
        left[0] * right[0] - left[1] * right[1],
        left[0] * right[1] + left[1] * right[0],
    ]
}
impl ChannelFilterSettings {
    /// Validates bounded work and explicit reconstruction tolerances.
    /// # Returns
    /// Unit for supported static integer-delay settings.
    /// # Errors
    /// Rejects excessive taps/delays, invalid gains/tolerances or absent direct path.
    pub fn validate(&self) -> Result<(), SimulationError> {
        match self {
            Self::Diagonal => {}
            Self::Echoes { paths } => {
                if paths.is_empty() || paths.len() > MAX_ECHO_PATHS || paths[0].delay_samples != 0 {
                    return Err(fail(
                        "Echoes need a direct path followed by at most 31 delayed paths.",
                    ));
                }
                let mut delays = std::collections::BTreeSet::new();
                let mut sum = 0.0;
                for path in paths {
                    let amplitude = path.gain[0].hypot(path.gain[1]);
                    if path.delay_samples > MAX_FILTER_DELAY_SAMPLES
                        || !delays.insert(path.delay_samples)
                        || !amplitude.is_finite()
                        || !(1e-6..=4.0).contains(&amplitude)
                    {
                        return Err(fail("Echo delays must be unique and at most 2048 samples; path amplitudes must be finite from 1e-6 to 4."));
                    }
                    sum += amplitude;
                }
                if sum > 8.0 {
                    return Err(fail(
                        "Total echo amplitude exceeds the bounded channel gain.",
                    ));
                }
            }
            Self::CapturedFir {
                max_delay_samples,
                causal_shift_samples,
                maximum_rms_error,
                maximum_bin_error,
            } => {
                if *max_delay_samples > MAX_FILTER_DELAY_SAMPLES
                    || *causal_shift_samples > *max_delay_samples
                    || !maximum_rms_error.is_finite()
                    || !(1e-9..=0.25).contains(maximum_rms_error)
                    || !maximum_bin_error.is_finite()
                    || !(1e-9..=1.0).contains(maximum_bin_error)
                {
                    return Err(fail("Captured FIR needs delay/shift within 2048 samples and finite RMS/bin error limits up to 0.25/1."));
                }
            }
        }
        Ok(())
    }
    /// Builds a frozen causal channel and reports its realized frequency response.
    /// # Arguments
    /// * `spacing` - Source 4K/8K centered frequency convention.
    /// * `response` - Full finite response; excluded bins explicitly extended to unity.
    /// * `active` - Original valid simulation bins on which errors are checked.
    /// # Returns
    /// None for the unchanged diagonal model; otherwise a bounded filter plan.
    /// # Errors
    /// Rejects invalid grids or a CM reconstruction outside declared error tolerances.
    pub fn prepare(
        &self,
        spacing: SubcarrierSpacing,
        response: &[IqPair],
        active: &[bool],
    ) -> Result<Option<ChannelFilterPlan>, SimulationError> {
        self.validate()?;
        let size = spacing.fft_samples() as usize;
        if response.len() != size
            || active.len() != size
            || response.iter().flatten().any(|value| !value.is_finite())
            || !active.iter().any(|value| *value)
        {
            return Err(fail(
                "Channel filter requires the exact finite physical FFT grid and active mask.",
            ));
        }
        if matches!(self, Self::Diagonal) {
            return Ok(None);
        }
        let mut target: Vec<_> = response
            .iter()
            .zip(active)
            .map(|(gain, active)| if *active { *gain } else { [1.0, 0.0] })
            .collect();
        let (coefficients, shift, discarded) = match self {
            Self::Echoes { paths } => {
                let gain = response
                    .iter()
                    .zip(active)
                    .find_map(|(point, active)| active.then_some(*point))
                    .ok_or_else(|| fail("No active echo reference."))?;
                if response.iter().zip(active).any(|(point, active)| {
                    *active
                        && ((point[0] - gain[0]).abs() > 1e-12
                            || (point[1] - gain[1]).abs() > 1e-12)
                }) {
                    return Err(fail("Synthetic echoes require a constant input response; use captured FIR for frequency-selective input."));
                }
                let length = paths
                    .iter()
                    .map(|path| path.delay_samples)
                    .max()
                    .unwrap_or(0)
                    + 1;
                let mut coefficients = vec![[0.0, 0.0]; length];
                for path in paths {
                    coefficients[path.delay_samples] = multiply(path.gain, gain);
                }
                (coefficients, 0, 0.0)
            }
            Self::CapturedFir {
                max_delay_samples,
                causal_shift_samples,
                ..
            } => {
                for (bin, gain) in target.iter_mut().enumerate() {
                    let phase = -std::f64::consts::TAU
                        * (bin as f64 - size as f64 / 2.0)
                        * (*causal_shift_samples as f64)
                        / size as f64;
                    *gain = multiply(*gain, [phase.cos(), phase.sin()]);
                }
                let full = InverseTransform::new(spacing).apply(&target)?;
                let total = full
                    .iter()
                    .map(|value| value[0].powi(2) + value[1].powi(2))
                    .sum::<f64>();
                let retained = full[..=*max_delay_samples]
                    .iter()
                    .map(|value| value[0].powi(2) + value[1].powi(2))
                    .sum::<f64>();
                if !total.is_finite() || total <= 0.0 {
                    return Err(fail("Captured FIR has no finite nonzero channel energy."));
                }
                (
                    full[..=*max_delay_samples]
                        .iter()
                        .map(|value| value.map(|component| component / (size as f64).sqrt()))
                        .collect(),
                    *causal_shift_samples,
                    (1.0 - retained / total).max(0.0),
                )
            }
            Self::Diagonal => unreachable!(),
        };
        let mut impulse = vec![[0.0, 0.0]; size];
        impulse[..coefficients.len()].copy_from_slice(&coefficients);
        let realized: Vec<_> = ForwardTransform::new(spacing)
            .apply(&impulse)?
            .iter()
            .map(|value| value.map(|component| component * (size as f64).sqrt()))
            .collect();
        let mut maximum: f64 = 0.0;
        let mut sum = 0.0;
        let mut count = 0;
        if matches!(self, Self::CapturedFir { .. }) {
            for ((got, wanted), active) in realized.iter().zip(&target).zip(active) {
                if *active {
                    let error = (got[0] - wanted[0]).hypot(got[1] - wanted[1]);
                    maximum = maximum.max(error);
                    sum += error * error;
                    count += 1;
                }
            }
        }
        let rms = if count > 0 {
            (sum / count as f64).sqrt()
        } else {
            0.0
        };
        if let Self::CapturedFir {
            maximum_rms_error,
            maximum_bin_error,
            ..
        } = self
        {
            if rms > *maximum_rms_error || maximum > *maximum_bin_error {
                return Err(SimulationError::new("ChannelFilterFitFailed",format!("CM filter reconstruction exceeds limits: RMS {rms:.6}, maximum {maximum:.6}. Keep the diagonal model or review the explicit delay/fit policy.")));
            }
        }
        if realized.iter().zip(active).any(|(gain, active)| {
            *active && (!gain[0].hypot(gain[1]).is_finite() || gain[0].hypot(gain[1]) < 1e-6)
        }) {
            return Err(fail("The realized filter contains an active-bin null below the supported inverse floor."));
        }
        Ok(Some(ChannelFilterPlan {
            diagnostics: FilterDiagnostics {
                taps: coefficients.len(),
                causal_shift_samples: shift,
                rms_complex_error: rms,
                maximum_complex_error: maximum,
                discarded_energy_fraction: discarded,
            },
            coefficients,
            response: realized,
        }))
    }
}
impl ChannelFilterPlan {
    /// Borrows causal coefficients in increasing integer sample delay.
    /// # Returns
    /// Read-only taps; their count is bounded to 2049.
    pub fn coefficients(&self) -> &[IqPair] {
        &self.coefficients
    }
    /// Borrows the realized response on the original FFT grid.
    /// # Returns
    /// Complete 4K/8K complex frequency response in centered bin order.
    pub fn response(&self) -> &[IqPair] {
        &self.response
    }
    /// Borrows fit errors and explicit delay/truncation policies.
    /// # Returns
    /// Immutable finite diagnostics for this frozen filter.
    pub fn diagnostics(&self) -> &FilterDiagnostics {
        &self.diagnostics
    }
}
/// Streaming linear convolution, with sparse paths or bounded FFT overlap-add.
pub struct StreamingChannelFilter {
    coefficients: Vec<IqPair>,
    paths: Vec<(usize, IqPair)>,
    history: Vec<IqPair>,
    overlap: Vec<IqPair>,
    forward: ForwardTransform,
    inverse: InverseTransform,
    response: Vec<IqPair>,
    next: u64,
    finished: bool,
}
impl StreamingChannelFilter {
    /// Creates a stream with zero prehistory at an explicit sample origin.
    /// # Arguments
    /// * `plan` - Validated causal coefficients and fit diagnostics.
    /// * `first_sample` - Expected first contiguous input tick.
    /// # Returns
    /// Bounded state; sparse paths use delay lines, dense filters use 8K FFTs.
    /// # Errors
    /// Propagates numerical transform failures.
    pub fn new(plan: &ChannelFilterPlan, first_sample: u64) -> Result<Self, SimulationError> {
        let coefficients = plan.coefficients.clone();
        let paths = coefficients
            .iter()
            .enumerate()
            .filter(|(_, value)| value[0] != 0.0 || value[1] != 0.0)
            .map(|(delay, value)| (delay, *value))
            .collect();
        let mut impulse = vec![[0.0, 0.0]; 8192];
        impulse[..coefficients.len()].copy_from_slice(&coefficients);
        let forward = ForwardTransform::new(SubcarrierSpacing::Khz25);
        let response = forward
            .apply(&impulse)?
            .iter()
            .map(|value| value.map(|component| component * (8192.0_f64).sqrt()))
            .collect();
        Ok(Self {
            history: vec![[0.0, 0.0]; coefficients.len() - 1],
            overlap: vec![[0.0, 0.0]; coefficients.len() - 1],
            coefficients,
            paths,
            forward,
            inverse: InverseTransform::new(SubcarrierSpacing::Khz25),
            response,
            next: first_sample,
            finished: false,
        })
    }
    /// Filters contiguous samples without resetting at symbols or blocks.
    /// # Arguments
    /// * `input` - Caller-owned finite sample block.
    /// # Returns
    /// Same-length channel output at the same sample coordinates.
    /// # Errors
    /// Rejects discontinuity or a closed stream; numerical failures close the run.
    pub fn transfer(&mut self, input: &SampleBlock) -> Result<SampleBlock, SimulationError> {
        if self.finished || input.start_index() != self.next {
            return Err(fail(
                "Channel filter is closed or sample input is discontinuous.",
            ));
        }
        let mut output = Vec::with_capacity(input.samples().len());
        for chunk in input.samples().chunks(4096) {
            let memory = self.coefficients.len() - 1;
            if self.paths.len() <= MAX_ECHO_PATHS {
                let mut source = self.history.clone();
                source.extend_from_slice(chunk);
                for index in 0..chunk.len() {
                    let mut point = [0.0, 0.0];
                    for (delay, gain) in &self.paths {
                        let product = multiply(source[memory + index - delay], *gain);
                        point[0] += product[0];
                        point[1] += product[1];
                    }
                    output.push(point);
                }
                self.history
                    .copy_from_slice(&source[source.len() - memory..]);
            } else {
                let mut padded = vec![[0.0, 0.0]; 8192];
                padded[..chunk.len()].copy_from_slice(chunk);
                let mut bins = self.forward.apply(&padded)?;
                for (point, gain) in bins.iter_mut().zip(&self.response) {
                    *point = multiply(*point, *gain);
                }
                let mut convolved = self.inverse.apply(&bins)?;
                for (point, previous) in convolved.iter_mut().zip(&self.overlap) {
                    point[0] += previous[0];
                    point[1] += previous[1];
                }
                output.extend_from_slice(&convolved[..chunk.len()]);
                self.overlap
                    .copy_from_slice(&convolved[chunk.len()..chunk.len() + memory]);
            }
        }
        let result = SampleBlock::new(input.start_index(), output)?;
        self.next = result.end_index();
        Ok(result)
    }
    /// Flushes exactly L-1 zero-input samples once; no payload symbols are invented.
    /// # Returns
    /// Finite channel-memory tail at the next stream sample tick.
    /// # Errors
    /// Rejects repeated finish or numeric/index overflow.
    pub fn finish(&mut self) -> Result<SampleBlock, SimulationError> {
        if self.finished {
            return Err(fail("Channel filter tail was already drained."));
        }
        let zeros = SampleBlock::new(self.next, vec![[0.0, 0.0]; self.coefficients.len() - 1])?;
        let tail = self.transfer(&zeros)?;
        self.finished = true;
        Ok(tail)
    }
}
