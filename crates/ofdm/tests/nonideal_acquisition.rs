/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::{
        modulation::map_symbol,
        receiver::channel_estimation::acquisition::{
            estimate_pilot_phase, CarrierOffsetCorrection,
        },
        waveform::{SampleBlock, WaveformGeometry},
    },
    simulation::waveform::{WaveformReceiver, WaveformTransmitter},
    transmission::{
        frame::assemble,
        layout::{CarrierLayout, PhysicalCarrierRole},
        settings::SAMPLE_RATE_HZ,
        SubcarrierSpacing,
    },
};
#[test]
fn unknown_timing_fractional_cfo_and_phase_recover_from_noisy_channel_samples() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (settings, raw) = acquisition_support::acquisition_setup(spacing, false);
        let geometry = WaveformGeometry::new(&settings).unwrap();
        let layout = CarrierLayout::new(&settings, &raw).unwrap();
        let phase = 37;
        let reduced: Vec<_> = layout
            .symbol_roles(phase)
            .iter()
            .enumerate()
            .filter_map(|(bin, role)| match role {
                PhysicalCarrierRole::Data => Some(Some(
                    map_symbol(256, ((bin * 73 + 19) % 256) as u16).unwrap(),
                )),
                PhysicalCarrierRole::ScatteredPilot => Some(None),
                _ => None,
            })
            .collect();
        let plc_width = if spacing == SubcarrierSpacing::Khz25 {
            16
        } else {
            8
        };
        let mut physical =
            assemble(&layout, phase, &reduced, &vec![[1.0, 0.0]; plc_width], 1.0).unwrap();
        let expected = physical.bins.clone();
        // Smooth unknown complex response; only received samples enter acquisition.
        for (bin, point) in physical.bins.iter_mut().enumerate() {
            if let Some(point) = point {
                let gain = 0.7 + 0.1 * bin as f64 / geometry.fft_samples() as f64;
                let (sine, cosine) =
                    (0.25 + 0.05 * bin as f64 / geometry.fft_samples() as f64).sin_cos();
                *point = [
                    gain * (point[0] * cosine - point[1] * sine),
                    gain * (point[0] * sine + point[1] * cosine),
                ];
            }
        }
        let transmitted = WaveformTransmitter::new(&geometry, phase, 0)
            .transmit(&physical)
            .unwrap();
        let offset = 23usize;
        let mut samples = vec![[0.0, 0.0]; offset];
        samples.extend_from_slice(transmitted.output.samples());
        samples.extend_from_slice(&[[0.0, 0.0]; 9]);
        let cfo = 1800.0;
        let mut noise = 0x419abcdu32;
        for (index, point) in samples.iter_mut().enumerate() {
            let (sine, cosine) =
                (std::f64::consts::TAU * cfo * index as f64 / f64::from(SAMPLE_RATE_HZ)).sin_cos();
            let rotated = [
                point[0] * cosine - point[1] * sine,
                point[0] * sine + point[1] * cosine,
            ];
            noise = noise.wrapping_mul(1664525).wrapping_add(1013904223);
            *point = [
                rotated[0] + (f64::from(noise & 65535) / 32768.0 - 1.0) * 1e-5,
                rotated[1] + (f64::from(noise >> 16) / 32768.0 - 1.0) * 1e-5,
            ];
        }
        let input = SampleBlock::new(100, samples).unwrap();
        let (_, acquired, received) =
            WaveformReceiver::acquire_nonideal(&geometry, &raw, &input, 32, 0.99).unwrap();
        // CP correlation may choose a nearby ISI-free boundary under noise.
        assert!(acquired.timing.first_symbol_sample.abs_diff(123) <= 2);
        assert!((acquired.carrier_offset_hz - cfo).abs() < 2.0);
        assert_eq!(acquired.pilot_phase, phase);
        assert!(acquired.normalized_pilot_error < 0.01);
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].symbol_index, phase);
        assert!(received[0].channel_estimate.is_some());
        let mut error_power = 0.0;
        let mut signal_power = 0.0;
        for (bin, role) in layout.symbol_roles(phase).iter().enumerate() {
            if *role == PhysicalCarrierRole::Data {
                let original = expected[bin].unwrap();
                let actual = received[0].equalized[bin];
                error_power +=
                    (actual[0] - original[0]).powi(2) + (actual[1] - original[1]).powi(2);
                signal_power += original[0].powi(2) + original[1].powi(2);
            }
        }
        assert!(error_power / signal_power < 0.01);
    }
}
#[test]
fn correction_is_chunk_independent_and_ambiguous_phase_rejects() {
    let (settings, raw) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let geometry = WaveformGeometry::new(&settings).unwrap();
    let correction = CarrierOffsetCorrection::new(&geometry, -900.0, 500).unwrap();
    let source: Vec<_> = (0..100).map(|index| [index as f64 / 100.0, 0.2]).collect();
    let all = correction
        .apply(&SampleBlock::new(500, source.clone()).unwrap())
        .unwrap();
    let first = correction
        .apply(&SampleBlock::new(500, source[..31].to_vec()).unwrap())
        .unwrap();
    let last = correction
        .apply(&SampleBlock::new(531, source[31..].to_vec()).unwrap())
        .unwrap();
    assert_eq!(all.samples(), [first.samples(), last.samples()].concat());
    assert!(correction
        .apply(&SampleBlock::new(499, vec![[1.0, 0.0]]).unwrap())
        .is_err());
    assert!(CarrierOffsetCorrection::new(&geometry, 12500.0, 0).is_err());
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    assert!(estimate_pilot_phase(&layout, &vec![[0.0, 0.0]; geometry.fft_samples()], 1.0).is_err());
    assert!(estimate_pilot_phase(&layout, &vec![[1.0, 0.0]; geometry.fft_samples()], 1.0).is_err());
}
