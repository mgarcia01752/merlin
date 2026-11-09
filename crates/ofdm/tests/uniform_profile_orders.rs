/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! QAM-03: actual full-word profile transport at every newly admitted uniform order.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        channel::{ChannelPhase, ChannelRun, ChannelRunSettings},
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::SubcarrierSpacing,
};
#[test]
fn all_new_uniform_orders_recover_through_live_plc_ncp_and_waveform() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for bits in [6, 7, 9, 10, 11, 12, 13, 14] {
            let (transmission, layout) = acquisition_support::acquisition_setup(spacing, false);
            let mut settings = ChannelRunSettings::new(transmission, layout);
            settings.duration = std::time::Duration::from_micros(100);
            settings.data_bit_loading = Some(vec![bits; spacing.fft_samples() as usize]);
            let mut run = ChannelRun::new(&settings).unwrap();
            while run
                .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
                .unwrap()
                .is_some()
            {}
            assert_eq!(run.summary().phase, ChannelPhase::Finished);
            assert!(
                run.summary().comparison.bits_tested > 0,
                "spacing={spacing:?},bits={bits}"
            );
            assert_eq!(
                run.summary().comparison.bit_errors,
                0,
                "spacing={spacing:?},bits={bits}"
            );
        }
    }
}
