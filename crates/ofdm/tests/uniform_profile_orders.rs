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
fn recover_uniform_order(spacing: SubcarrierSpacing, bits: u8) {
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

#[test]
fn khz25_qam64_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 6);
}

#[test]
fn khz25_qam128_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 7);
}

#[test]
fn khz25_qam512_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 9);
}

#[test]
fn khz25_qam1024_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 10);
}

#[test]
fn khz25_qam2048_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 11);
}

#[test]
fn khz25_qam4096_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 12);
}

#[test]
fn khz25_qam8192_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 13);
}

#[test]
fn khz25_qam16384_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz25, 14);
}

#[test]
fn khz50_qam64_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 6);
}

#[test]
fn khz50_qam128_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 7);
}

#[test]
fn khz50_qam512_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 9);
}

#[test]
fn khz50_qam1024_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 10);
}

#[test]
fn khz50_qam2048_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 11);
}

#[test]
fn khz50_qam4096_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 12);
}

#[test]
fn khz50_qam8192_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 13);
}

#[test]
fn khz50_qam16384_recovers_through_live_plc_ncp_and_waveform() {
    recover_uniform_order(SubcarrierSpacing::Khz50, 14);
}
