/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::{
        channel::{
            additional_loss::{AdditionalLoss, CableModel, LengthUnit},
            composition::prepare_channel,
            filter::{ChannelFilterSettings, EchoPath},
        },
        receiver::channel_estimation::ReceiverMode,
    },
    simulation::{
        channel::{
            constellation::ConstellationSelection, run_channel, ChannelJobLimits, ChannelJobStatus,
            ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        SubcarrierSpacing,
    },
};
fn settings(cable: bool) -> ChannelRunSettings {
    let (transmission, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(transmission, layout);
    settings.duration = std::time::Duration::from_micros(100);
    settings.impairments.additional_loss = if cable {
        AdditionalLoss::Cable {
            cable: CableModel::Belden1694aV1,
            length: 100.0,
            unit: LengthUnit::Feet,
        }
    } else {
        AdditionalLoss::Manual { tilt_db: -6.0 }
    };
    settings.impairments.power.amplifier_gain_db = 6.0;
    settings.impairments.power.attenuation_db = 3.0;
    settings.channel_filter = ChannelFilterSettings::Echoes {
        paths: vec![
            EchoPath {
                delay_samples: 0,
                gain: [1.0, 0.0],
            },
            EchoPath {
                delay_samples: 8,
                gain: [0.05, 0.02],
            },
        ],
    };
    settings
}
fn expected(settings: &ChannelRunSettings, bin: usize, frequency: u32) -> [f64; 2] {
    // Independent source anchors: fixed fixture spans108–131.95 MHz. Cable uses
    // the pinned100 MHz/1.95 and135 MHz/2.24 dB per100 ft table interval.
    let db = match settings.impairments.additional_loss {
        AdditionalLoss::Manual { .. } => -6.0 * f64::from(frequency - 108_000_000) / 23_950_000.0,
        _ => -(1.95 + (f64::from(frequency) / 1e6 - 100.0) * (2.24 - 1.95) / (135.0 - 100.0)),
    };
    let amplitude = 10.0_f64.powf((db + 3.0) / 20.0);
    let phase = -std::f64::consts::TAU * (bin as f64 - 2048.0) * 8.0 / 4096.0;
    [
        amplitude * (1.0 + 0.05 * phase.cos() - 0.02 * phase.sin()),
        amplitude * (0.05 * phase.sin() + 0.02 * phase.cos()),
    ]
}
#[test]
fn total_response_matches_independent_loss_times_echo_and_scalar_only_once() {
    for cable in [false, true] {
        let settings = settings(cable);
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout).unwrap();
        let prepared = prepare_channel(
            &settings.transmission,
            &layout,
            &settings.impairments,
            settings.llr_variance,
            &settings.channel_filter,
        )
        .unwrap();
        assert_eq!(prepared.filter.as_ref().unwrap().coefficients().len(), 9);
        for (bin, role) in layout.static_roles().iter().enumerate() {
            if *role == PhysicalCarrierRole::Excluded {
                continue;
            }
            let expected = expected(&settings, bin, layout.frequency_hz(bin as u16).unwrap());
            let actual = prepared.plan.response[bin];
            assert!(
                (actual[0] - expected[0]).abs() < 1e-12 && (actual[1] - expected[1]).abs() < 1e-12
            );
        }
    }
}
fn recover(cable: bool, mode: ReceiverMode) {
    let mut settings = settings(cable);
    settings.receiver_mode = mode;
    settings.constellation = Some(ConstellationSelection {
        requested_pairs: 128,
        ..Default::default()
    });
    let result = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &std::sync::atomic::AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Complete);
    assert!(result.summary.comparison.bits_tested > 0);
    assert_eq!(result.summary.comparison.bit_errors, 0);
    assert_eq!(result.summary.channel_filter_tail_samples, 8);
    let pairs = &result.constellation.as_ref().unwrap().pairs;
    assert!(!pairs.is_empty());
    for pair in pairs {
        let reference = pair.reference_iq.unwrap();
        let gain = expected(&settings, usize::from(pair.physical_bin), pair.frequency_hz);
        let expected = [
            reference[0] * gain[0] - reference[1] * gain[1],
            reference[0] * gain[1] + reference[1] * gain[0],
        ];
        let actual = pair.before_equalization;
        assert!(
            (actual[0] - expected[0]).abs() < 1e-8 && (actual[1] - expected[1]).abs() < 1e-8,
            "raw waveform must include loss and echo exactly once"
        );
    }
}
#[test]
fn manual_tilt_and_echo_recover_reference_payload_and_actual_raw_samples() {
    recover(false, ReceiverMode::SuppliedChannel);
}
#[test]
fn cable_and_echo_recover_mce_payload_and_actual_raw_samples() {
    recover(true, ReceiverMode::MerlinEstimated);
}
