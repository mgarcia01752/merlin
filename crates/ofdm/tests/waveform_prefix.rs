/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Both directions cover every permitted configured prefix without shared TX state.
use docsis_ofdm_fec::{
    signal_processing::waveform::{cyclic_prefix, WaveformGeometry},
    transmission::{settings::CYCLIC_PREFIX_SAMPLES, SubcarrierSpacing, TransmissionSettings},
};
#[test]
fn all_prefix_lengths_copy_the_tail_and_rx_selects_exact_useful_samples() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for prefix in CYCLIC_PREFIX_SAMPLES {
            let mut settings = TransmissionSettings {
                subcarrier_spacing: spacing,
                cyclic_prefix_samples: prefix,
                roll_off_samples: 64,
                ..Default::default()
            };
            let geometry = WaveformGeometry::new(&settings).unwrap();
            settings.cyclic_prefix_samples = 1024; // Cannot change the active snapshot.
            assert_eq!(geometry.prefix_samples(), usize::from(prefix));
            let useful: Vec<_> = (0..geometry.fft_samples())
                .map(|index| [index as f64, -(index as f64)])
                .collect();
            let prefixed = cyclic_prefix::insert(&geometry, &useful).unwrap();
            assert_eq!(prefixed.len(), geometry.stride_samples());
            assert_eq!(
                &prefixed[..usize::from(prefix)],
                &useful[useful.len() - usize::from(prefix)..]
            );
            assert_eq!(cyclic_prefix::remove(&geometry, &prefixed).unwrap(), useful);
            assert!(cyclic_prefix::remove(&geometry, &prefixed[..prefixed.len() - 1]).is_err());
            assert!(cyclic_prefix::insert(&geometry, &useful[1..]).is_err());
        }
    }
}
#[test]
fn invalid_geometry_and_nonfinite_samples_are_rejected() {
    assert!(WaveformGeometry::new(&TransmissionSettings {
        cyclic_prefix_samples: 191,
        ..Default::default()
    })
    .is_err());
    assert!(WaveformGeometry::new(&TransmissionSettings {
        cyclic_prefix_samples: 192,
        roll_off_samples: 192,
        ..Default::default()
    })
    .is_err());
    let geometry = WaveformGeometry::new(&TransmissionSettings::default()).unwrap();
    assert!(cyclic_prefix::insert(
        &geometry,
        &vec![[f64::INFINITY, 0.0]; geometry.fft_samples()]
    )
    .is_err());
}
