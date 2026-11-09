/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::{
        receiver::channel_estimation::timing::estimate_symbol_boundary,
        waveform::{SampleBlock, WaveformGeometry},
    },
    transmission::{SubcarrierSpacing, TransmissionSettings},
};
#[test]
fn independent_prefix_vectors_recover_unknown_offset_and_reject_ambiguity() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let geometry = WaveformGeometry::new(&TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        })
        .unwrap();
        let fft = geometry.fft_samples();
        let prefix = geometry.prefix_samples();
        // A deterministic independent broadband sequence, not TX/RX round-trip output.
        let mut state = 0x9182_6751u32;
        let mut sample = || {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            [
                f64::from(state & 65535) / 32768.0 - 1.0,
                f64::from(state >> 16) / 32768.0 - 1.0,
            ]
        };
        let useful: Vec<_> = (0..fft).map(|_| sample()).collect();
        let offset = 23;
        let mut samples: Vec<_> = (0..offset).map(|_| sample()).collect();
        samples.extend_from_slice(&useful[fft - prefix..]);
        // The leading overlap/taper region is not a clean cyclic copy. Without
        // this, several prefix-internal boundaries are genuinely ambiguous.
        for point in &mut samples[offset..offset + geometry.roll_off_samples()] {
            point[0] *= 0.5;
            point[1] *= 0.5;
        }
        samples.extend_from_slice(&useful);
        samples.extend((0..32).map(|_| sample()));
        let input = SampleBlock::new(100, samples).unwrap();
        let estimate = estimate_symbol_boundary(&geometry, &input, 32, 0.99).unwrap();
        assert_eq!(estimate.first_symbol_sample, 123);
        assert!((estimate.normalized_correlation - 1.0).abs() < 1e-12);
        let constant =
            SampleBlock::new(0, vec![[1.0, 0.0]; geometry.stride_samples() + 32]).unwrap();
        assert!(estimate_symbol_boundary(&geometry, &constant, 32, 0.99).is_err());
        let silence =
            SampleBlock::new(0, vec![[0.0, 0.0]; geometry.stride_samples() + 32]).unwrap();
        assert!(estimate_symbol_boundary(&geometry, &silence, 32, 0.5).is_err());
        assert!(
            estimate_symbol_boundary(&geometry, &input, geometry.stride_samples() + 1, 0.99)
                .is_err()
        );
        assert!(estimate_symbol_boundary(&geometry, &input, 32, 0.0).is_err());
    }
}

#[test]
fn actual_windowed_waveform_acquires_timing_without_boundary_or_channel_truth() {
    use docsis_ofdm_fec::{
        simulation::waveform::{WaveformReceiver, WaveformTransmitter},
        transmission::{
            frame::assemble,
            layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
        },
    };
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (first, end, plc, pilots) = if spacing == SubcarrierSpacing::Khz25 {
            (
                320,
                1280,
                752,
                vec![352, 448, 544, 920, 1008, 1080, 1176, 1256],
            )
        } else {
            (160, 640, 376, vec![176, 224, 272, 460, 504, 540, 588, 628])
        };
        let settings = TransmissionSettings {
            subcarrier_spacing: spacing,
            occupied_bandwidth_hz: 24_000_000,
            ..Default::default()
        };
        let geometry = WaveformGeometry::new(&settings).unwrap();
        let raw = CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: first,
            active_end_exclusive: end,
            plc_first_bin: plc,
            additional_continuous_pilots: pilots,
            excluded_bins: vec![],
        };
        let layout = CarrierLayout::new(&settings, &raw).unwrap();
        let reduced: Vec<_> = layout
            .symbol_roles(0)
            .iter()
            .enumerate()
            .filter_map(|(bin, role)| match role {
                PhysicalCarrierRole::Data => Some(Some([
                    if bin % 3 == 0 { -1.0 } else { 1.0 },
                    if bin % 5 == 0 { -1.0 } else { 1.0 },
                ])),
                PhysicalCarrierRole::ScatteredPilot => Some(None),
                _ => None,
            })
            .collect();
        let width = if spacing == SubcarrierSpacing::Khz25 {
            16
        } else {
            8
        };
        let physical = assemble(&layout, 0, &reduced, &vec![[1.0, 0.0]; width], 1.0).unwrap();
        let transmitted = WaveformTransmitter::new(&geometry, 0, 0)
            .transmit(&physical)
            .unwrap();
        let mut samples = vec![[0.0, 0.0]; 23];
        samples.extend_from_slice(transmitted.output.samples());
        samples.extend_from_slice(&[[0.0, 0.0]; 9]);
        let input = SampleBlock::new(0, samples).unwrap();
        let (_, timing, received) =
            WaveformReceiver::acquire_from_pilots(&geometry, &raw, &input, 0, 32, 0.99).unwrap();
        assert_eq!(timing.first_symbol_sample, 23);
        assert_eq!(received.len(), 1);
        for (actual, expected) in received[0].equalized.iter().zip(&physical.bins) {
            let expected = expected.unwrap();
            assert!((actual[0] - expected[0]).hypot(actual[1] - expected[1]) < 1e-10);
        }
    }
}
