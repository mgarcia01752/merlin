/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real namespace sessions, exact old type identity and cross-direction stop tokens.
#![cfg(all(feature = "downstream", feature = "upstream"))]
use merlin::{common, downstream, upstream};
use std::{
    any::TypeId,
    time::{Duration, Instant},
};
const UPSTREAM: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");

#[test]
fn downstream_exports_keep_exact_legacy_nominal_types_and_idle_behavior() {
    assert_eq!(
        TypeId::of::<downstream::Simulation>(),
        TypeId::of::<docsis_ofdm_fec::api::Simulation>()
    );
    assert_eq!(
        TypeId::of::<downstream::Merlin>(),
        TypeId::of::<docsis_ofdm_fec::api::Merlin>()
    );
    assert_eq!(
        TypeId::of::<common::RunControl>(),
        TypeId::of::<downstream::RunControl>()
    );
    assert_eq!(
        TypeId::of::<common::ErrorCode>(),
        TypeId::of::<downstream::ErrorCode>()
    );
    assert_eq!(
        TypeId::of::<common::ErrorCode>(),
        TypeId::of::<upstream::ErrorCode>()
    );
    let legacy = docsis_ofdm_fec::api::Simulation::new();
    let mut current: downstream::Simulation = legacy;
    assert!(current.profile_ids().is_empty());
    assert_eq!(
        current.options().unwrap_err().code(),
        common::ErrorCode::InvalidInput
    );
    assert_eq!(
        current.run().err().unwrap().code(),
        common::ErrorCode::InvalidInput
    );
}

#[test]
fn upstream_namespace_executes_real_loop_and_preserves_structured_errors() {
    let mut simulation = upstream::Simulation::new(upstream::ExperimentOptions::default()).unwrap();
    assert_eq!(
        simulation.preview().unwrap_err().category(),
        common::ErrorCode::MissingSetting
    );
    simulation.configure_json(UPSTREAM).unwrap();
    assert!(!simulation.preview().unwrap().profile_ids.is_empty());
    let report = simulation.run().unwrap();
    assert!(report.reference_recovered);
    assert_eq!(report.bit_errors, 0);
    assert!(report.compared_source_bits > 0);
    let bad = simulation.configure_json(b"{bad").unwrap_err();
    assert_eq!(bad.category(), common::ErrorCode::InvalidInput);
    assert_eq!(bad.code(), "Serialization");
    assert!(std::error::Error::source(&bad).is_some());
    drop(simulation);
    assert!(report.reference_recovered);
}

#[test]
fn shared_precancellation_reaches_both_directions_and_retains_source_context() {
    let common = common::RunControl::new();
    let downstream: downstream::RunControl = common.clone();
    let upstream = upstream::RunControl::with_cancellation(common.clone());
    downstream.cancel();
    assert!(common.is_cancelled());
    assert!(upstream.is_cancelled());
    assert!(common::Cancellation::is_cancelled(&upstream));
    let profile_bytes =
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
    let rxmer_bytes = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
    let profile = downstream::Dpd::from_bytes(
        &downstream::NamedCapture::new("profile", profile_bytes.to_vec()).unwrap(),
        downstream::ProfileId::PROFILE_A,
    )
    .unwrap();
    let ocd = downstream::Ocd::from_profile(
        downstream::ProtocolScope::Docsis31,
        &profile,
        downstream::OcdOverrides {
            bandwidth: Some(downstream::BandwidthHz::new(192_000_000).unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    let mut downstream_session = downstream::Simulation::with_ocd(ocd);
    downstream_session.set_profile(profile).unwrap();
    downstream_session
        .set_options(
            downstream::EvaluationOptions::default()
                .with_channel_selection(downstream::ChannelSelection::Unity)
                .with_profile_tolerance(downstream::AssociationTolerance::new(u32::MAX).unwrap()),
        )
        .unwrap();
    downstream_session
        .add_rxmer(
            downstream::RxMer::from_bytes(
                &downstream::NamedCapture::new("rxmer", rxmer_bytes.to_vec()).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    let downstream_report = downstream_session.run_with_control(&downstream).unwrap();
    assert_eq!(
        downstream_report.status(),
        downstream::BatchStatus::Cancelled
    );
    assert_eq!(downstream_report.iter().count(), 1);
    assert!(downstream_report
        .iter()
        .all(|entry| entry.status() == downstream::RunStatus::NotStarted));
    let mut simulation = upstream::Simulation::new(upstream::ExperimentOptions::default()).unwrap();
    simulation.configure_json(UPSTREAM).unwrap();
    let report = simulation.run_with_control(&upstream).unwrap();
    assert_eq!(report.status, upstream::RunStatus::Cancelled);
    assert_eq!(report.generated_source_bits, 0);
    assert!(report.source_configuration_original.is_some());
    let other = common::RunControl::new();
    upstream::RunControl::with_cancellation(other.clone()).cancel();
    assert!(other.is_cancelled());
}

#[test]
fn common_cancellation_unblocks_an_acknowledged_upstream_pause() {
    let common = common::RunControl::new();
    let control = upstream::RunControl::with_cancellation(common.clone());
    control.pause();
    let worker_control = control.clone();
    let worker = std::thread::spawn(move || {
        let mut simulation =
            upstream::Simulation::new(upstream::ExperimentOptions::default()).unwrap();
        simulation.configure_json(UPSTREAM).unwrap();
        simulation.run_with_control(&worker_control).unwrap()
    });
    let deadline = Instant::now() + Duration::from_secs(5);
    while control.progress().status != upstream::RunStatus::Paused {
        assert!(Instant::now() < deadline, "worker never acknowledged pause");
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(control.progress().emitted_samples, 0);
    common.cancel();
    while !worker.is_finished() {
        assert!(
            Instant::now() < deadline,
            "common cancellation did not wake paused worker"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
    let report = worker.join().unwrap();
    assert_eq!(report.status, upstream::RunStatus::Cancelled);
    assert_eq!(report.generated_source_bits, 0);
    assert_eq!(control.progress().status, upstream::RunStatus::Cancelled);
}

#[test]
fn every_downstream_export_retains_legacy_type_identity() {
    assert_eq!(
        TypeId::of::<downstream::ApiError>(),
        TypeId::of::<docsis_ofdm_fec::api::ApiError>()
    );
    assert_eq!(
        TypeId::of::<downstream::ApiResult<()>>(),
        TypeId::of::<docsis_ofdm_fec::api::ApiResult<()>>()
    );
    assert_eq!(
        TypeId::of::<downstream::AssociationTolerance>(),
        TypeId::of::<docsis_ofdm_fec::api::AssociationTolerance>()
    );
    assert_eq!(
        TypeId::of::<downstream::BandwidthHz>(),
        TypeId::of::<docsis_ofdm_fec::api::BandwidthHz>()
    );
    assert_eq!(
        TypeId::of::<downstream::BatchReport>(),
        TypeId::of::<docsis_ofdm_fec::api::BatchReport>()
    );
    assert_eq!(
        TypeId::of::<downstream::BatchStatus>(),
        TypeId::of::<docsis_ofdm_fec::api::BatchStatus>()
    );
    assert_eq!(
        TypeId::of::<downstream::BitCount>(),
        TypeId::of::<docsis_ofdm_fec::api::BitCount>()
    );
    assert_eq!(
        TypeId::of::<downstream::BitErrorRatio>(),
        TypeId::of::<docsis_ofdm_fec::api::BitErrorRatio>()
    );
    assert_eq!(
        TypeId::of::<downstream::CaptureKind>(),
        TypeId::of::<docsis_ofdm_fec::api::CaptureKind>()
    );
    assert_eq!(
        TypeId::of::<downstream::CaptureProvenance>(),
        TypeId::of::<docsis_ofdm_fec::api::CaptureProvenance>()
    );
    assert_eq!(
        TypeId::of::<downstream::CaptureTime>(),
        TypeId::of::<docsis_ofdm_fec::api::CaptureTime>()
    );
    assert_eq!(
        TypeId::of::<downstream::CarrierCount>(),
        TypeId::of::<docsis_ofdm_fec::api::CarrierCount>()
    );
    assert_eq!(
        TypeId::of::<downstream::CarrierMer>(),
        TypeId::of::<docsis_ofdm_fec::api::CarrierMer>()
    );
    assert_eq!(
        TypeId::of::<downstream::CarrierRange>(),
        TypeId::of::<docsis_ofdm_fec::api::CarrierRange>()
    );
    assert_eq!(
        TypeId::of::<downstream::ChannelEstimate>(),
        TypeId::of::<docsis_ofdm_fec::api::ChannelEstimate>()
    );
    assert_eq!(
        TypeId::of::<downstream::ChannelEstimateId>(),
        TypeId::of::<docsis_ofdm_fec::api::ChannelEstimateId>()
    );
    assert_eq!(
        TypeId::of::<downstream::ChannelSelection>(),
        TypeId::of::<docsis_ofdm_fec::api::ChannelSelection>()
    );
    assert_eq!(
        TypeId::of::<downstream::CodewordCount>(),
        TypeId::of::<docsis_ofdm_fec::api::CodewordCount>()
    );
    assert_eq!(
        TypeId::of::<downstream::ComplexResponse>(),
        TypeId::of::<docsis_ofdm_fec::api::ComplexResponse>()
    );
    assert_eq!(
        TypeId::of::<downstream::CyclicPrefixSamples>(),
        TypeId::of::<docsis_ofdm_fec::api::CyclicPrefixSamples>()
    );
    assert_eq!(
        TypeId::of::<downstream::DataModulation>(),
        TypeId::of::<docsis_ofdm_fec::api::DataModulation>()
    );
    assert_eq!(
        TypeId::of::<downstream::Dpd>(),
        TypeId::of::<docsis_ofdm_fec::api::Dpd>()
    );
    assert_eq!(
        TypeId::of::<downstream::ElapsedTime>(),
        TypeId::of::<docsis_ofdm_fec::api::ElapsedTime>()
    );
    assert_eq!(
        TypeId::of::<downstream::ErrorCode>(),
        TypeId::of::<docsis_ofdm_fec::api::ErrorCode>()
    );
    assert_eq!(
        TypeId::of::<downstream::EvaluationOptions>(),
        TypeId::of::<docsis_ofdm_fec::api::EvaluationOptions>()
    );
    assert_eq!(
        TypeId::of::<downstream::ExperimentEntry>(),
        TypeId::of::<docsis_ofdm_fec::api::ExperimentEntry>()
    );
    assert_eq!(
        TypeId::of::<downstream::ExperimentId>(),
        TypeId::of::<docsis_ofdm_fec::api::ExperimentId>()
    );
    assert_eq!(
        TypeId::of::<downstream::FecReport>(),
        TypeId::of::<docsis_ofdm_fec::api::FecReport>()
    );
    assert_eq!(
        TypeId::of::<downstream::FrequencyHz>(),
        TypeId::of::<docsis_ofdm_fec::api::FrequencyHz>()
    );
    assert_eq!(
        TypeId::of::<downstream::InterleaverDepth>(),
        TypeId::of::<docsis_ofdm_fec::api::InterleaverDepth>()
    );
    assert_eq!(
        TypeId::of::<downstream::IterationLimit>(),
        TypeId::of::<docsis_ofdm_fec::api::IterationLimit>()
    );
    assert_eq!(
        TypeId::of::<downstream::LinearPower>(),
        TypeId::of::<docsis_ofdm_fec::api::LinearPower>()
    );
    assert_eq!(
        TypeId::of::<downstream::MeasurementSources>(),
        TypeId::of::<docsis_ofdm_fec::api::MeasurementSources>()
    );
    assert_eq!(
        TypeId::of::<downstream::MerDb>(),
        TypeId::of::<docsis_ofdm_fec::api::MerDb>()
    );
    assert_eq!(
        TypeId::of::<downstream::Merlin>(),
        TypeId::of::<docsis_ofdm_fec::api::Merlin>()
    );
    assert_eq!(
        TypeId::of::<downstream::MetricScope>(),
        TypeId::of::<docsis_ofdm_fec::api::MetricScope>()
    );
    assert_eq!(
        TypeId::of::<downstream::NamedCapture>(),
        TypeId::of::<docsis_ofdm_fec::api::NamedCapture>()
    );
    assert_eq!(
        TypeId::of::<downstream::NoiseSeed>(),
        TypeId::of::<docsis_ofdm_fec::api::NoiseSeed>()
    );
    assert_eq!(
        TypeId::of::<downstream::ObservationDuration>(),
        TypeId::of::<docsis_ofdm_fec::api::ObservationDuration>()
    );
    assert_eq!(
        TypeId::of::<downstream::Ocd>(),
        TypeId::of::<docsis_ofdm_fec::api::Ocd>()
    );
    assert_eq!(
        TypeId::of::<downstream::OcdConfiguration>(),
        TypeId::of::<docsis_ofdm_fec::api::OcdConfiguration>()
    );
    assert_eq!(
        TypeId::of::<downstream::OcdOverrides>(),
        TypeId::of::<docsis_ofdm_fec::api::OcdOverrides>()
    );
    assert_eq!(
        TypeId::of::<downstream::PayloadRate>(),
        TypeId::of::<docsis_ofdm_fec::api::PayloadRate>()
    );
    assert_eq!(
        TypeId::of::<downstream::PerformanceReport>(),
        TypeId::of::<docsis_ofdm_fec::api::PerformanceReport>()
    );
    assert_eq!(
        TypeId::of::<downstream::PrbsSeed>(),
        TypeId::of::<docsis_ofdm_fec::api::PrbsSeed>()
    );
    assert_eq!(
        TypeId::of::<downstream::PreparedExperiment>(),
        TypeId::of::<docsis_ofdm_fec::api::PreparedExperiment>()
    );
    assert_eq!(
        TypeId::of::<downstream::ProcessingOptions>(),
        TypeId::of::<docsis_ofdm_fec::api::ProcessingOptions>()
    );
    assert_eq!(
        TypeId::of::<downstream::ProfileId>(),
        TypeId::of::<docsis_ofdm_fec::api::ProfileId>()
    );
    assert_eq!(
        TypeId::of::<downstream::ProfileOrigin>(),
        TypeId::of::<docsis_ofdm_fec::api::ProfileOrigin>()
    );
    assert_eq!(
        TypeId::of::<downstream::ProfilePerformance>(),
        TypeId::of::<docsis_ofdm_fec::api::ProfilePerformance>()
    );
    assert_eq!(
        TypeId::of::<downstream::ProtocolScope>(),
        TypeId::of::<docsis_ofdm_fec::api::ProtocolScope>()
    );
    assert_eq!(
        TypeId::of::<downstream::ReceiverMode>(),
        TypeId::of::<docsis_ofdm_fec::api::ReceiverMode>()
    );
    assert_eq!(
        TypeId::of::<downstream::Resolved<u32>>(),
        TypeId::of::<docsis_ofdm_fec::api::Resolved<u32>>()
    );
    assert_eq!(
        TypeId::of::<downstream::ResolvedConfiguration>(),
        TypeId::of::<docsis_ofdm_fec::api::ResolvedConfiguration>()
    );
    assert_eq!(
        TypeId::of::<downstream::ResponseImaginary>(),
        TypeId::of::<docsis_ofdm_fec::api::ResponseImaginary>()
    );
    assert_eq!(
        TypeId::of::<downstream::ResponseReal>(),
        TypeId::of::<docsis_ofdm_fec::api::ResponseReal>()
    );
    assert_eq!(
        TypeId::of::<downstream::RetainedWordLimit>(),
        TypeId::of::<docsis_ofdm_fec::api::RetainedWordLimit>()
    );
    assert_eq!(
        TypeId::of::<downstream::RollOffSamples>(),
        TypeId::of::<docsis_ofdm_fec::api::RollOffSamples>()
    );
    assert_eq!(
        TypeId::of::<downstream::RunControl>(),
        TypeId::of::<docsis_ofdm_fec::api::RunControl>()
    );
    assert_eq!(
        TypeId::of::<downstream::RunLimits>(),
        TypeId::of::<docsis_ofdm_fec::api::RunLimits>()
    );
    assert_eq!(
        TypeId::of::<downstream::RunStatus>(),
        TypeId::of::<docsis_ofdm_fec::api::RunStatus>()
    );
    assert_eq!(
        TypeId::of::<downstream::RuntimeLimit>(),
        TypeId::of::<docsis_ofdm_fec::api::RuntimeLimit>()
    );
    assert_eq!(
        TypeId::of::<downstream::RxMer>(),
        TypeId::of::<docsis_ofdm_fec::api::RxMer>()
    );
    assert_eq!(
        TypeId::of::<downstream::RxMerId>(),
        TypeId::of::<docsis_ofdm_fec::api::RxMerId>()
    );
    assert_eq!(
        TypeId::of::<downstream::RxMerSelection>(),
        TypeId::of::<docsis_ofdm_fec::api::RxMerSelection>()
    );
    assert_eq!(
        TypeId::of::<downstream::SampleCount>(),
        TypeId::of::<docsis_ofdm_fec::api::SampleCount>()
    );
    assert_eq!(
        TypeId::of::<downstream::SettingOrigin>(),
        TypeId::of::<docsis_ofdm_fec::api::SettingOrigin>()
    );
    assert_eq!(
        TypeId::of::<downstream::Simulation>(),
        TypeId::of::<docsis_ofdm_fec::api::Simulation>()
    );
    assert_eq!(
        TypeId::of::<downstream::SubcarrierIndex>(),
        TypeId::of::<docsis_ofdm_fec::api::SubcarrierIndex>()
    );
    assert_eq!(
        TypeId::of::<downstream::SubcarrierSpacing>(),
        TypeId::of::<docsis_ofdm_fec::api::SubcarrierSpacing>()
    );
    assert_eq!(
        TypeId::of::<downstream::Weight>(),
        TypeId::of::<docsis_ofdm_fec::api::Weight>()
    );
}

#[test]
fn upstream_error_categories_keep_offsets_and_original_domain_causes() {
    use upstream::{ApiError, CaptureFailure, PhyError};
    for (error, expected) in [
        (
            ApiError::Capture {
                category: CaptureFailure::InvalidInput,
                field: "fixture",
                offset: 19,
            },
            common::ErrorCode::InvalidInput,
        ),
        (
            ApiError::Capture {
                category: CaptureFailure::Unsupported,
                field: "fixture",
                offset: 19,
            },
            common::ErrorCode::UnsupportedCapability,
        ),
        (
            ApiError::Capture {
                category: CaptureFailure::ResourceLimit,
                field: "fixture",
                offset: 19,
            },
            common::ErrorCode::ResourceLimit,
        ),
    ] {
        assert_eq!(error.category(), expected);
        assert!(matches!(
            error,
            ApiError::Capture {
                field: "fixture",
                offset: 19,
                ..
            }
        ));
    }
    for (domain, expected) in [
        (
            PhyError::InvalidInput { field: "fixture" },
            common::ErrorCode::InvalidInput,
        ),
        (
            PhyError::Unsupported { field: "fixture" },
            common::ErrorCode::UnsupportedCapability,
        ),
        (
            PhyError::ResourceLimit { field: "fixture" },
            common::ErrorCode::ResourceLimit,
        ),
        (
            PhyError::Association { field: "fixture" },
            common::ErrorCode::AssociationFailed,
        ),
        (
            PhyError::Association {
                field: "control.busy",
            },
            common::ErrorCode::Busy,
        ),
    ] {
        let error = ApiError::from(domain);
        assert_eq!(error.category(), expected);
        assert!(std::error::Error::source(&error).is_some());
    }
}

#[test]
fn upstream_facade_retains_actual_receiver_estimates_through_independent_owner() {
    const RETAINED_FRAME_BUDGET: usize = 1;
    const RETENTION_SCALAR_BUDGET: usize = 4096;
    const EXPECTED_FFT_SAMPLES: usize = 2048;
    let policy =
        upstream::ReceiverObservationPolicy::new(RETAINED_FRAME_BUDGET, RETENTION_SCALAR_BUDGET)
            .unwrap();
    assert_eq!(
        TypeId::of::<upstream::ReceiverObservationPolicy>(),
        TypeId::of::<docsis_ofdma::api::ReceiverObservationPolicy>()
    );
    let mut simulation = upstream::Simulation::new(Default::default()).unwrap();
    simulation
        .configure_json(include_bytes!(
            "../../../examples/cli/upstream/us-50khz.json"
        ))
        .unwrap();
    simulation.set_receiver_observation_policy(policy).unwrap();
    assert_eq!(simulation.receiver_observation_policy(), policy);
    let report = simulation.run().unwrap();
    assert!(report.reference_recovered);
    assert_eq!(
        report.receiver_observations().retained_frames,
        RETAINED_FRAME_BUDGET
    );
    let estimate = report
        .receiver_estimate(upstream::ReceiverEstimateRequest::default())
        .unwrap();
    assert_eq!(estimate.points.len(), EXPECTED_FFT_SAMPLES);
    assert_eq!(
        estimate.reference_plane,
        "received_pilot_effective_c_times_h"
    );
}
