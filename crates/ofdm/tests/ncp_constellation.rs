/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::interleaving::FrequencyInterleaver,
    simulation::{
        channel::{
            constellation::ConstellationSelection, run_channel, ChannelJobLimits,
            ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{
        layout::{ncp::PreInterleavingRole, CarrierLayout, PilotReference},
        SubcarrierSpacing,
    },
};
use std::{sync::atomic::AtomicBool, time::Duration};
#[test]
fn accepted_ncp_uses_real_inverse_coordinates_and_does_not_change_recovery() {
    let (tx, raw) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let mut settings = ChannelRunSettings::new(tx, raw);
    settings.duration = Duration::from_millis(1);
    let run = |settings: &ChannelRunSettings| {
        run_channel(
            settings,
            &ChannelJobLimits::default(),
            &AtomicBool::new(false),
            &mut TestPointProbe::disabled(),
            &mut NoopObserver,
            |_| {},
        )
        .unwrap()
    };
    let baseline = run(&settings);
    settings.constellation = Some(ConstellationSelection {
        requested_pairs: 256,
        capture_ncp: true,
        ..Default::default()
    });
    let observed = run(&settings);
    assert_eq!(
        serde_json::to_value(&baseline.summary).unwrap(),
        serde_json::to_value(&observed.summary).unwrap()
    );
    let origin = observed.summary.payload_origin_symbol.unwrap();
    let pairs = observed.constellation.unwrap();
    assert_eq!(pairs.ncp_pairs.len(), 256);
    assert_eq!(pairs.pairs.len(), 256);
    assert!(pairs.ncp_pairs.iter().any(|p| p.source_symbol_index
        > pairs
            .pairs
            .iter()
            .map(|p| p.source_symbol_index)
            .max()
            .unwrap()));
    let layout = CarrierLayout::new(&settings.transmission, &settings.layout).unwrap();
    let reference = PilotReference::new(
        &layout,
        usize::from(settings.transmission.time_interleaver_depth),
    )
    .unwrap();
    let permutation = FrequencyInterleaver::new(
        settings.transmission.subcarrier_spacing,
        reference.settings().cells(),
    )
    .unwrap();
    for pair in &pairs.ncp_pairs {
        assert_eq!(pair.modulation_order, settings.ncp_modulation.order());
        assert_eq!(pair.llrs.len(), pair.modulation_order.ilog2() as usize);
        assert!(pair.llrs.iter().all(|v| v.is_finite()));
        let rank = reference
            .roles(pair.source_symbol_index - origin)
            .iter()
            .enumerate()
            .filter(|(_, r)| **r == PreInterleavingRole::Data)
            .nth(pair.data_position)
            .unwrap()
            .0;
        assert_eq!(
            pair.channel_symbol_index,
            pair.source_symbol_index + (rank % reference.settings().depth()) as u64
        );
        let physical = permutation
            .output_to_input()
            .iter()
            .position(|&v| v == rank)
            .unwrap();
        assert_eq!(pair.physical_bin, reference.physical_bins()[physical]);
        assert!(!pairs
            .pairs
            .iter()
            .any(|p| p.channel_symbol_index == pair.channel_symbol_index
                && p.physical_bin == pair.physical_bin));
        for (a, b) in pair.before_equalization.iter().zip(pair.after_equalization) {
            assert!((a - b).abs() < 1e-11);
        }
    }
}
