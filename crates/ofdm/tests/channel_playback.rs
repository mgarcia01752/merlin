/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    scenario::{
        document::ScenarioDocument,
        pnm_json::{import_profiles, import_rxmer, JsonCapture},
    },
    simulation::{
        channel::{
            measurement::{captured_layout, MeasurementSelection},
            measurement_playback::{PlaybackConfiguration, PlaybackMode},
            run_channel, ChannelJobLimits, ChannelRunSettings,
        },
        test_points::{
            NoopObserver, ProbeConfig, ProbeContext, ProbeData, ProbeSelection, RunPhase,
            SignalDomain, TestPointId, TestPointProbe,
        },
    },
    transmission::TransmissionSettings,
    types::ProtocolScope,
};
use std::{sync::atomic::AtomicBool, time::Duration};
#[test]
fn actual_channel_playback_preserves_rng_fec_epochs_and_rejects_out_of_run_schedule() {
    let profile = import_profiles(JsonCapture {
        name: "dpd",
        bytes: include_bytes!(
            "../../../examples/measurements/single-channel/modulation_profile.json"
        ),
    })
    .unwrap()
    .into_iter()
    .find(|p| p.profile_id == 0)
    .unwrap();
    let frame = import_rxmer(JsonCapture {
        name: "rxmer",
        bytes: include_bytes!("../../../examples/measurements/single-channel/rxmer.json"),
    })
    .unwrap();
    let transmission = TransmissionSettings::default();
    let mut settings = ChannelRunSettings::new(
        transmission.clone(),
        captured_layout(&profile, &transmission).unwrap(),
    );
    settings.duration = Duration::from_millis(1);
    // Selection and later residual playback must retain the explicit extra response.
    settings.impairments.additional_loss =
        docsis_ofdm_fec::signal_processing::channel::additional_loss::AdditionalLoss::Manual {
            tilt_db: -6.0,
        };
    let conditioning = docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning {
        amplifier_gain_db: 6.0,
        attenuation_db: 6.0,
        input_data_level_dbmv_per_6mhz: Some(0.0),
        receiver_noise_dbmv_hz: Some(-120.0),
        ..Default::default()
    };
    settings.impairments.power = conditioning.clone();
    MeasurementSelection {
        use_rxmer_noise: true,
        transitions: vec![],
        frame_index: 0,
        profile_id: 0,
        channel_index: None,
        tolerance_seconds: 90000,
        noise_seed: 41,
    }
    .apply(
        &ScenarioDocument {
            schema_version: 1,
            protocol: ProtocolScope::Docsis31,
            profiles: vec![profile.clone()],
            frames: vec![frame.clone()],
            channels: vec![],
        },
        &mut settings,
    )
    .unwrap();
    assert_eq!(
        settings.impairments.additional_loss,
        docsis_ofdm_fec::signal_processing::channel::additional_loss::AdditionalLoss::Manual {
            tilt_db: -6.0
        }
    );
    assert_eq!(settings.impairments.power, conditioning);
    let baseline = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    let mut next = frame.clone();
    next.capture_time += 1;
    next.source.capture_time = Some(next.capture_time);
    let original = serde_json::to_vec(&vec![frame.clone(), next.clone()]).unwrap();
    settings.playback = Some(PlaybackConfiguration {
        profile,
        frames: vec![frame, next],
        mode: PlaybackMode::Interpolate,
        interval_symbols: 2,
        tolerance_seconds: 90000,
    });
    let result = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(result.summary).unwrap(),
        serde_json::to_value(baseline.summary).unwrap()
    );
    assert_eq!(
        serde_json::to_vec(&settings.playback.as_ref().unwrap().frames).unwrap(),
        original
    );
    settings.playback.as_mut().unwrap().interval_symbols = u64::MAX / 2;
    let error = settings.validate().unwrap_err();
    assert_eq!(error.code, "PlaybackObservationTooShort");
    assert_eq!(
        error.message,
        "Playback final transition exceeds the observation window."
    );
}
#[test]
fn phase_filter_does_not_spend_the_observation_budget_on_warmup() {
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::SourcePayload,
            elements: 0..1,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 1,
        max_total_elements: 1,
    })
    .unwrap();
    probe.set_phase_filter(Some(RunPhase::Observation));
    let mut context = ProbeContext::new("run", "settings", RunPhase::Warmup).unwrap();
    probe
        .emit(
            TestPointId::SourcePayload,
            &context,
            SignalDomain::CodewordBit,
            ProbeData::Bits(&[1]),
            &mut NoopObserver,
        )
        .unwrap();
    assert_eq!(probe.stats().dispatched_events, 0);
    context.phase = RunPhase::Observation;
    probe
        .emit(
            TestPointId::SourcePayload,
            &context,
            SignalDomain::CodewordBit,
            ProbeData::Bits(&[1]),
            &mut NoopObserver,
        )
        .unwrap();
    assert_eq!(probe.stats().dispatched_events, 1);
}

#[test]
fn per_stage_limit_preserves_remaining_global_observation_budget() {
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::SourcePayload,
            elements: 0..1,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 2,
        max_elements_per_event: 1,
        max_total_elements: 2,
    })
    .unwrap();
    probe
        .limit_stage_events(TestPointId::SourcePayload, 1)
        .unwrap();
    let context = ProbeContext::new("run", "settings", RunPhase::Observation).unwrap();
    for _ in 0..3 {
        probe
            .emit(
                TestPointId::SourcePayload,
                &context,
                SignalDomain::CodewordBit,
                ProbeData::Bits(&[1]),
                &mut NoopObserver,
            )
            .unwrap();
    }
    assert_eq!(probe.stats().dispatched_events, 1);
    assert_eq!(probe.stats().dispatched_elements, 1);
    assert!(probe
        .limit_stage_events(TestPointId::SourcePayload, 2)
        .is_err());
}
