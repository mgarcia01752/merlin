/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent configured-transfer/power anchors through the public channel API.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const CHANNEL: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");
fn profile() -> Dpd {
    Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap()
}
fn transmission() -> TransmissionConfiguration {
    TransmissionConfiguration::new(TransmissionSettings::default()).unwrap()
}
fn close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "{left} != {right}");
}
fn empty() -> ChannelConfiguration {
    ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap()
}

#[test]
fn unity_preview_retains_original_grid_exclusions_and_bounded_before_receiver_results() {
    let profile = profile();
    let configuration = empty();
    let result = configuration
        .preview(&transmission(), &profile, None, AssociationTolerance::ZERO)
        .unwrap();
    assert_eq!(
        result.reference_plane,
        "configured_transfer_before_receiver"
    );
    assert_eq!(result.key, *profile.key());
    assert_eq!(result.profile_id, 0);
    assert_eq!(result.effective_response.len(), 7480);
    assert!(result
        .effective_response
        .iter()
        .flatten()
        .all(|sample| *sample == [1.0, 0.0]));
    assert!(result.gain.points.len() <= 1024);
    assert_eq!(result.additional_points.len(), 128);
    assert_eq!(result.additional_low_gain_db, 0.0);
    assert_eq!(result.additional_high_gain_db, 0.0);
    assert!(result.filter_coefficients.is_empty());
    assert!(result.filter_diagnostics.is_none());
    assert_eq!(result.power.millivolts_squared_per_unit, None);
    for (response, assignment) in result.effective_response.iter().zip(profile.carriers()) {
        assert_eq!(
            response.is_none(),
            assignment.carrier_type == ProfileCarrierRole::Exclusion
        );
    }
    drop(configuration);
    drop(profile);
    assert_eq!(result.sources[0].original_bytes(), PROFILE);
}

#[test]
fn manual_loss_and_conditioning_use_literal_endpoint_gains_and_electrical_reference() {
    let configuration = ChannelConfiguration::new(ChannelConfigurationInput {
        additional_loss: AdditionalLoss::Manual { tilt_db: -6.0 },
        power: PowerConditioning {
            amplifier_gain_db: 6.0,
            cpa_attenuation_db: 2.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
    let result = configuration
        .preview(
            &transmission(),
            &profile(),
            None,
            AssociationTolerance::ZERO,
        )
        .unwrap();
    close(
        result.additional_points.first().unwrap().additional_gain_db,
        0.0,
    );
    close(
        result.additional_points.last().unwrap().additional_gain_db,
        -6.0,
    );
    close(
        result
            .additional_points
            .first()
            .unwrap()
            .effective_gain_db
            .unwrap(),
        4.0,
    );
    close(
        result
            .additional_points
            .last()
            .unwrap()
            .effective_gain_db
            .unwrap(),
        -2.0,
    );
    close(result.power.net_amplitude, 1.5848931924611136);
    let calibrated = ChannelConfiguration::new(ChannelConfigurationInput {
        power: PowerConditioning {
            input_data_level_dbmv_per_6mhz: Some(0.0),
            receiver_noise_dbmv_hz: Some(-100.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
    let power = calibrated.power_reference();
    close(
        power.millivolts_squared_per_unit.unwrap(),
        34.13333333333333,
    );
    close(power.receiver_noise_power, 0.0006);
}

#[test]
fn raw_source_identity_and_actual_echo_plan_keep_original_data_and_composition_rules() {
    let mut document: Value = serde_json::from_slice(PROFILE).unwrap();
    document["profiles"][0]["profile_id"] = json!(200);
    let input = NamedCapture::new("source", serde_json::to_vec(&document).unwrap()).unwrap();
    let source = ProfileCapture::from_bytes(&input).unwrap();
    let configuration = empty();
    let raw = configuration
        .preview_capture(
            &transmission(),
            &source,
            200,
            None,
            AssociationTolerance::ZERO,
        )
        .unwrap();
    let accepted = configuration
        .preview(
            &transmission(),
            &profile(),
            None,
            AssociationTolerance::ZERO,
        )
        .unwrap();
    assert_eq!(raw.profile_id, 200);
    assert_eq!(raw.effective_response, accepted.effective_response);
    assert!(ProfileId::new(200).is_err());
    let echoes = ChannelConfiguration::new(ChannelConfigurationInput {
        filter: ChannelFilterSettings::Echoes {
            paths: vec![
                EchoPath {
                    delay_samples: 0,
                    gain: [1.0, 0.0],
                },
                EchoPath {
                    delay_samples: 1,
                    gain: [0.25, 0.0],
                },
            ],
        },
        ..Default::default()
    })
    .unwrap();
    let plan = echoes
        .preview(
            &transmission(),
            &profile(),
            None,
            AssociationTolerance::ZERO,
        )
        .unwrap();
    assert_eq!(plan.filter_coefficients, vec![[1.0, 0.0], [0.25, 0.0]]);
    assert_eq!(plan.filter_diagnostics.unwrap().taps, 2);
    let response =
        ChannelEstimate::from_bytes(&NamedCapture::new("original H", CHANNEL.to_vec()).unwrap())
            .unwrap();
    assert!(echoes
        .preview(
            &transmission(),
            &profile(),
            Some(&response),
            AssociationTolerance::new(90000).unwrap()
        )
        .is_err());
    assert_eq!(response.provenance().original_bytes(), CHANNEL);
}

#[test]
fn malformed_parameters_epoch_identity_and_work_limits_reject_without_mutating_inputs() {
    for bytes in [b"{bad".as_slice(), b"{\"unknown\":1}".as_slice()] {
        assert!(ChannelConfiguration::from_json(bytes).is_err());
    }
    assert_eq!(
        ChannelConfiguration::from_json(&vec![b' '; 65537])
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    assert!(ChannelConfiguration::new(ChannelConfigurationInput {
        power: PowerConditioning {
            amplifier_gain_db: 61.0,
            ..Default::default()
        },
        ..Default::default()
    })
    .is_err());
    assert!(ChannelConfiguration::new(ChannelConfigurationInput {
        filter: ChannelFilterSettings::Echoes { paths: vec![] },
        ..Default::default()
    })
    .is_err());
    let configuration = empty();
    let timing = transmission();
    let profile = profile();
    let response =
        ChannelEstimate::from_bytes(&NamedCapture::new("H", CHANNEL.to_vec()).unwrap()).unwrap();
    assert_eq!(
        configuration
            .preview(
                &timing,
                &profile,
                Some(&response),
                AssociationTolerance::ZERO
            )
            .err()
            .unwrap()
            .code(),
        ErrorCode::AssociationFailed
    );
    configuration
        .preview(
            &timing,
            &profile,
            Some(&response),
            AssociationTolerance::new(90000).unwrap(),
        )
        .unwrap();
    let mut wrong: Value = serde_json::from_slice(CHANNEL).unwrap();
    wrong["channel_id"] = json!(33);
    let wrong = ChannelEstimate::from_bytes(
        &NamedCapture::new("wrong identity", serde_json::to_vec(&wrong).unwrap()).unwrap(),
    )
    .unwrap();
    assert!(configuration
        .preview(
            &timing,
            &profile,
            Some(&wrong),
            AssociationTolerance::new(90000).unwrap()
        )
        .is_err());
    assert_eq!(profile.sources()[0].original_bytes(), PROFILE);
    assert_eq!(response.provenance().original_bytes(), CHANNEL);
    configuration
        .preview(&timing, &profile, None, AssociationTolerance::ZERO)
        .unwrap();
}
