/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Configuration is learned only through received PLC IQ/FEC/MC packets.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::plc::{generate_frame, PlcAcquisitionReceiver},
    transmission::{
        layout::CarrierLayout,
        plc::{
            descriptors::{DescriptorIdentity, FixedPlcDescriptors},
            receiver_configuration::ReceiverConfigurationStatus,
            scheduling::PlcMessageScheduler,
            ExtendedTimestamp, PlcMessageFrame,
        },
        SubcarrierSpacing,
    },
};
#[test]
fn received_iq_drives_waiting_to_ready_and_supplies_real_consumer_settings() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (settings, mut raw) = acquisition_support::acquisition_setup(spacing, true);
        let plc_width = docsis_ofdm_fec::transmission::plc::PlcFrameGeometry::new(spacing)
            .plc_subcarriers as u16;
        let extra: Vec<_> = (raw.plc_first_bin + 8 * plc_width..raw.active_end_exclusive)
            .step_by(3)
            .filter(|bin| {
                !raw.additional_continuous_pilots.contains(bin) && !raw.excluded_bins.contains(bin)
            })
            .take(40)
            .collect();
        raw.excluded_bins.extend(extra);
        let layout = CarrierLayout::new(&settings, &raw).unwrap();
        let descriptors = FixedPlcDescriptors::new(
            &settings,
            &layout,
            DescriptorIdentity {
                channel_id: 7,
                change_count: 19,
                source_mac: [0, 0xca, 0, 0, 0, 1],
                primary_capable: true,
                ncp_bits: 2,
            },
        )
        .unwrap();
        let mut scheduler = PlcMessageScheduler::new(&settings, &descriptors).unwrap();
        // Receiver receives no settings/layout/profile object from the transmitter.
        let mut receiver = PlcAcquisitionReceiver::new(spacing).unwrap();
        let mut admissions = 0;
        let frame_period_samples =
            u64::from(settings.one_second_timing().unwrap().symbol_period_samples) * 128;
        for frame_index in 0..8 {
            let block = scheduler.next_block().unwrap();
            let tx = generate_frame(
                PlcMessageFrame::new(
                    spacing,
                    ExtendedTimestamp::from_sample_ticks(frame_index * frame_period_samples, 0)
                        .unwrap(),
                    block.block,
                )
                .unwrap(),
                frame_index,
            )
            .unwrap();
            let received: Vec<_> = (0..128)
                .flat_map(|phase| {
                    tx.symbol_iq(frame_index * 128 + phase)
                        .unwrap()
                        .iter()
                        .copied()
                })
                .collect();
            let result = receiver.receive_frame(&received, 0.01, 100).unwrap();
            assert!(result.transport_error.is_none());
            assert!(result
                .admissions
                .iter()
                .all(|outcome| outcome.error.is_none()));
            admissions += result.admissions.len();
            if frame_index == 0 {
                assert_eq!(
                    result.configuration_status,
                    ReceiverConfigurationStatus::Waiting
                );
                assert!(receiver.configuration().require_ready().is_err());
            }
        }
        assert_eq!(admissions, 3);
        let configuration = receiver.configuration().require_ready().unwrap();
        assert_eq!(configuration.channel_id(), 7);
        assert_eq!(configuration.ncp_modulation().order(), 4);
        assert_eq!(
            configuration.channel().layout_settings().plc_first_bin,
            raw.plc_first_bin
        );
        assert_eq!(configuration.ocd_change_count(), 19);
        // Construct downstream components using received configuration only.
        let physical = configuration.channel().physical_layout().unwrap();
        let interleaver = configuration.interleaver_settings().unwrap();
        let reference = docsis_ofdm_fec::transmission::layout::PilotReference::new(
            &physical,
            interleaver.depth(),
        )
        .unwrap();
        assert_eq!(reference.physical_bins(), configuration.reduced_bins());
        assert_eq!(reference.settings().cells(), interleaver.cells());
        let _receive_interleaver =
            docsis_ofdm_fec::signal_processing::interleaving::SymbolInterleaver::<[f64; 2]>::new(
                interleaver,
                docsis_ofdm_fec::signal_processing::interleaving::InterleaverDirection::Receive,
            )
            .unwrap();
        let before = serde_json::to_value(configuration).unwrap();
        let bad = receiver
            .receive_frame(
                &vec![
                    [0.0, 0.0];
                    128 * if spacing == SubcarrierSpacing::Khz25 {
                        16
                    } else {
                        8
                    }
                ],
                0.01,
                5,
            )
            .unwrap();
        assert_eq!(
            bad.configuration_status,
            ReceiverConfigurationStatus::Rejected
        );
        assert!(bad.frame.message_error.is_some());
        assert_eq!(
            serde_json::to_value(receiver.configuration().last_valid().unwrap()).unwrap(),
            before
        );
        assert!(receiver.configuration().require_ready().is_err());
        receiver.reset();
        assert_eq!(
            receiver.configuration().status(),
            ReceiverConfigurationStatus::Waiting
        );
        assert!(receiver.configuration().last_valid().is_none());
    }
}
#[test]
fn invalid_grid_marks_rejection_without_erasing_completed_configuration_history() {
    let mut receiver = PlcAcquisitionReceiver::new(SubcarrierSpacing::Khz50).unwrap();
    assert!(receiver.receive_frame(&[], 0.01, 10).is_err());
    assert_eq!(
        receiver.configuration().status(),
        ReceiverConfigurationStatus::Rejected
    );
    assert!(receiver.configuration().last_rejection().is_some());
    assert!(receiver.configuration().last_valid().is_none());
}
