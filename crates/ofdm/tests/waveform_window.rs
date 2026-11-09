/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY half-sample taper, independent energy identity and explicit stream lifecycle.
use docsis_ofdm_fec::{
    signal_processing::waveform::{
        cyclic_prefix, overlap_add::OverlapAdd, windowing::TransmitWindow, WaveformGeometry,
    },
    transmission::{
        settings::{CYCLIC_PREFIX_SAMPLES, ROLL_OFF_SAMPLES},
        SubcarrierSpacing, TransmissionSettings,
    },
};
#[test]
fn every_permitted_window_matches_half_sample_sine_definition_and_energy() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for prefix in CYCLIC_PREFIX_SAMPLES {
            for roll in ROLL_OFF_SAMPLES.into_iter().filter(|roll| *roll < prefix) {
                let geometry = WaveformGeometry::new(&TransmissionSettings {
                    subcarrier_spacing: spacing,
                    cyclic_prefix_samples: prefix,
                    roll_off_samples: roll,
                    ..Default::default()
                })
                .unwrap();
                let window = TransmitWindow::new(&geometry);
                let values = window.coefficients();
                let stride = geometry.stride_samples();
                let roll = usize::from(roll);
                assert_eq!(values.len(), stride + roll);
                for index in 0..roll {
                    let angle = std::f64::consts::PI * ((index as f64 + 0.5) / roll as f64 - 0.5);
                    let independently_defined = 0.5 * (1.0 + angle.sin());
                    assert!((values[index] - independently_defined).abs() < 1e-14);
                    assert_eq!(values[index], values[values.len() - 1 - index]);
                    assert!((values[index] + values[stride + index] - 1.0).abs() < 1e-14);
                }
                assert!(values[roll..stride].iter().all(|value| *value == 1.0));
                let energy: f64 = values.iter().map(|value| value * value).sum();
                assert!((energy - (stride as f64 - roll as f64 / 4.0)).abs() < 1e-8);
                let useful: Vec<_> = (0..geometry.fft_samples())
                    .map(|index| [index as f64, 1.0])
                    .collect();
                let prefixed = cyclic_prefix::insert(&geometry, &useful).unwrap();
                let extended = window.apply(&prefixed).unwrap();
                for index in 0..roll {
                    assert_eq!(
                        extended[stride + index],
                        useful[index].map(|value| value * values[stride + index])
                    );
                }
                assert_eq!(&extended[geometry.prefix_samples()..stride], useful);
            }
        }
    }
}
#[test]
fn overlap_add_matches_independent_shifted_sum_and_rejected_calls_preserve_state() {
    let geometry = WaveformGeometry::new(&TransmissionSettings::default()).unwrap();
    let stride = geometry.stride_samples();
    let window = TransmitWindow::new(&geometry);
    let first = window.apply(&vec![[1.0, -2.0]; stride]).unwrap();
    let second = window.apply(&vec![[3.0, 4.0]; stride]).unwrap();
    let mut expected = vec![[0.0, 0.0]; 2 * stride + geometry.roll_off_samples()];
    for (offset, values) in [(0, &first), (stride, &second)] {
        for (index, sample) in values.iter().enumerate() {
            for (sum, value) in expected[offset + index].iter_mut().zip(sample) {
                *sum += value;
            }
        }
    }
    let mut overlap = OverlapAdd::new(&geometry, 19);
    let mut actual = Vec::new();
    actual.extend_from_slice(overlap.push(&first).unwrap().samples());
    assert!(overlap.push(&second[1..]).is_err());
    assert!(overlap
        .push(&vec![[f64::NAN, 0.0]; geometry.extended_samples()])
        .is_err());
    let next = overlap.push(&second).unwrap();
    assert_eq!(next.start_index(), 19 + stride as u64);
    actual.extend_from_slice(next.samples());
    let tail = overlap.finish().unwrap();
    assert_eq!(tail.start_index(), 19 + 2 * stride as u64);
    actual.extend_from_slice(tail.samples());
    assert_eq!(actual, expected);
    assert!(overlap.finish().unwrap().samples().is_empty());
    assert!(overlap.push(&first).is_err());
    overlap.reset(0);
    assert_eq!(overlap.push(&first).unwrap().start_index(), 0);
}
#[test]
fn sample_clock_overflow_and_zero_rolloff_have_explicit_results() {
    let geometry = WaveformGeometry::new(&TransmissionSettings {
        roll_off_samples: 0,
        ..Default::default()
    })
    .unwrap();
    let window = TransmitWindow::new(&geometry);
    let values = vec![[1.0, 0.0]; geometry.stride_samples()];
    assert_eq!(window.apply(&values).unwrap(), values);
    let mut overlap = OverlapAdd::new(&geometry, u64::MAX);
    assert!(overlap.push(&values).is_err());
    overlap.reset(0);
    assert_eq!(overlap.push(&values).unwrap().samples(), values);
    assert!(overlap.finish().unwrap().samples().is_empty());
}
