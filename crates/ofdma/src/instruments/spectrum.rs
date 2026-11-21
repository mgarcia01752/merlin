/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Rectangular two-sided PSD from retained actual contiguous sample intervals.
use super::{ObservationStore, Tap};
use crate::{
    configuration::{Numerology, SAMPLE_RATE_HZ},
    error::invalid,
    signal_processing::Transform,
    PhyError, PhyResult,
};
use serde::Serialize;

/// Owned normalized complex-baseband power spectral density, not RF calibration.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpectrumSnapshot {
    /// Actual continuous-time test point.
    pub tap: Tap,
    /// Run/configuration/grant identity of this actually retained interval.
    pub identity: crate::types::Identity,
    /// Actual fixed complex-baseband clock in samples per second.
    pub sample_rate_hz: u32,
    /// Explicit linear PSD units, distinct from calibrated RF power.
    pub units: &'static str,
    /// Analysis window, independent from TX taper.
    pub analysis_window: &'static str,
    /// Exact bounded analysis FFT length.
    pub fft_len: usize,
    /// Absolute first sample tick of this FFT window.
    pub first_sample: u64,
    /// Ascending centered baseband frequency coordinates in hertz.
    pub frequency_hz: Vec<f64>,
    /// Linear two-sided normalized power/Hz; no logarithm of zero is fabricated.
    pub power_per_hz: Vec<f64>,
}

impl ObservationStore {
    /// Calculates a rectangular PSD from already-retained samples at `tap`.
    ///
    /// `mode` chooses a 2048/4096-sample analysis window. Returns owned frequency
    /// and |unitary FFT|²/fs values. Missing/discontinuous/short retention returns
    /// typed unavailable. A retained useful-symbol interval is analyzed independently;
    /// frequency-grid taps are never treated as time samples.
    pub fn spectrum(&self, tap: Tap, mode: Numerology) -> PhyResult<SpectrumSnapshot> {
        if !matches!(
            tap,
            Tap::TxUseful | Tap::TxOutput | Tap::ChannelInput | Tap::ChannelOutput | Tap::RxInput
        ) {
            return Err(invalid("spectrum.tap_plane"));
        }
        let trace = self.trace(tap).ok_or(PhyError::Unsupported {
            field: "spectrum.unretained_tap",
        })?;
        let n = mode.fft_len();
        if trace.samples.len() < n {
            return Err(PhyError::Unsupported {
                field: "spectrum.unavailable",
            });
        }
        let bins = Transform::new(mode).forward(&trace.samples[..n])?;
        let power: Vec<_> = bins
            .iter()
            .map(|z| z.power() / f64::from(SAMPLE_RATE_HZ))
            .collect();
        if power.iter().any(|value| !value.is_finite()) {
            return Err(invalid("spectrum.power"));
        }
        Ok(SpectrumSnapshot {
            tap,
            identity: trace.identity.ok_or(PhyError::Unsupported {
                field: "spectrum.identity",
            })?,
            sample_rate_hz: SAMPLE_RATE_HZ,
            units: "normalized-complex-baseband-power-per-hz",
            analysis_window: "rectangular",
            fft_len: n,
            first_sample: trace.origin.ok_or(PhyError::Unsupported {
                field: "spectrum.origin",
            })?,
            frequency_hz: (0..n)
                .map(|k| (k as f64 - n as f64 / 2.0) * f64::from(mode.spacing_hz()))
                .collect(),
            power_per_hz: power,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Identity, Iq, SampleBlock};
    #[test]
    fn independent_constant_tone_psd_units_identity_and_no_cross_grant_join() {
        let identity = Identity {
            run: 9,
            generation: 3,
            grant: 7,
        };
        for mode in [Numerology::Khz50, Numerology::Khz25] {
            let n = mode.fft_len();
            let mut store = ObservationStore::new(8192, &[Tap::TxOutput, Tap::TxUseful]).unwrap();
            let block = SampleBlock {
                identity,
                start: 100,
                samples: vec![Iq { re: 1.0, im: 0.0 }; n],
            };
            store.record_samples(Tap::TxOutput, &block);
            store.record_samples(Tap::TxUseful, &block);
            for tap in [Tap::TxOutput, Tap::TxUseful] {
                let spectrum = store.spectrum(tap, mode).unwrap();
                assert_eq!(spectrum.identity, identity);
                assert_eq!(spectrum.first_sample, 100);
                assert_eq!(spectrum.sample_rate_hz, 102_400_000);
                assert_eq!(spectrum.fft_len, n);
                assert_eq!(spectrum.analysis_window, "rectangular");
                assert!(
                    (spectrum.power_per_hz[n / 2] - 1.0 / f64::from(mode.spacing_hz())).abs()
                        < 1e-17
                );
                let integrated =
                    spectrum.power_per_hz.iter().sum::<f64>() * f64::from(mode.spacing_hz());
                assert!((integrated - 1.0).abs() < 1e-13);
            }
            let before = store.trace(Tap::TxOutput).unwrap().samples.clone();
            store.record_samples(
                Tap::TxOutput,
                &SampleBlock {
                    identity: Identity {
                        grant: 8,
                        ..identity
                    },
                    start: 100 + n as u64,
                    samples: vec![Iq::default(); 32],
                },
            );
            assert_eq!(store.trace(Tap::TxOutput).unwrap().samples, before);
        }
    }
}
