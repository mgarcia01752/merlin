/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact source ownership and real public project round trips.
use super::*;
use crate::{
    shell::ApplicationSessions,
    upstream::{PreparedSession, UpstreamController},
};
use merlin::{
    common::Project,
    upstream::{
        BatchReport, ConfigurationDocument, ExperimentOptions, ReceiverObservationPolicy,
        ReportJson, Simulation, SoftObservationPolicy, Tap,
    },
};
use serde_json::Value;

const CONFIGURATION_50: &[u8] =
    include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
const CONFIGURATION_25: &[u8] =
    include_bytes!("../../../../../examples/cli/upstream/us-25khz.json");
const MEASURED_50: &[u8] =
    include_bytes!("../../../../../examples/measurements/upstream/measured/capture-driven.json");
const MEASURED_25: &[u8] = include_bytes!(
    "../../../../../examples/measurements/upstream/measured/capture-driven-25khz.json"
);

// Independent small saved overrides, unrelated to production or waveform limits.
const OVERRIDE_GRANTS: usize = 2;
const OVERRIDE_CHUNK_SAMPLES: usize = 313;
const OVERRIDE_RETAINED_POINTS: usize = 29;

// One full estimate at either numerology; two Cartesian slots per FFT bin.
const ESTIMATE_FRAMES: usize = 1;
const ESTIMATE_SCALAR_SLOTS: usize = 8192;

// Independent small soft histories and one maximum-size transmitted word.
const SOFT_CELLS: usize = 4;
const SOFT_WORDS: usize = 1;
const SOFT_WORD_LLRS: usize = 16_200;

fn policy() -> ReceiverObservationPolicy {
    ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALAR_SLOTS)
        .unwrap()
        .with_soft_metrics(
            SoftObservationPolicy::new(SOFT_CELLS, SOFT_WORDS, SOFT_WORD_LLRS).unwrap(),
        )
}

fn controller(application: &ApplicationSessions, prepared: PreparedSession) -> UpstreamController {
    let key = application.configure_upstream(true).unwrap();
    UpstreamController::new(key, prepared).unwrap()
}

fn options(simulation: &Simulation) -> ExperimentOptions {
    ExperimentOptions {
        grants: OVERRIDE_GRANTS,
        chunk_samples: OVERRIDE_CHUNK_SAMPLES,
        retained_points: OVERRIDE_RETAINED_POINTS,
        retained_taps: vec![Tap::TxOutput],
        ..simulation.options().clone()
    }
}

fn assert_report(actual: &BatchReport, reference: &BatchReport) {
    let actual: Value = serde_json::from_str(&actual.to_json().unwrap()).unwrap();
    let reference: Value = serde_json::from_str(&reference.to_json().unwrap()).unwrap();
    for key in [
        "configuration",
        "options",
        "planned",
        "status",
        "generated_source_bits",
        "compared_source_bits",
        "bit_errors",
        "ber",
        "reference_recovered",
        "not_started_grants",
        "measurement_provenance_json",
        "source_configuration_original",
        "source_configuration_sha256",
    ] {
        assert_eq!(actual[key], reference[key], "{key}");
    }
    let words = |value: &Value| {
        value["grants"]
            .as_array()
            .unwrap()
            .iter()
            .map(|grant| grant["words"].clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(words(&actual), words(&reference));
}

fn export_reference(kind: &str, index: usize, project: &[u8], reference: &BatchReport) {
    if let Some(directory) = std::env::var_os("MERLIN_QT_PROJECT_REFERENCE_DIRECTORY") {
        let destination = std::path::PathBuf::from(directory).join(format!("{kind}-{index}.json"));
        let value = serde_json::json!({
            "project": std::str::from_utf8(project).unwrap(),
            "reference": serde_json::from_str::<Value>(&reference.to_json().unwrap()).unwrap()
        });
        std::fs::write(destination, serde_json::to_vec(&value).unwrap()).unwrap();
    }
}

#[test]
fn configuration_export_preserves_exact_source_actual_overrides_and_idle_owner_both_modes() {
    for (index, bytes) in [CONFIGURATION_50, CONFIGURATION_25].into_iter().enumerate() {
        let application = ApplicationSessions::desktop().unwrap();
        let prepared = PreparedSession::from_document(
            ConfigurationDocument::from_json(bytes).unwrap(),
            policy(),
            application.budget(),
        )
        .unwrap();
        let controller = controller(&application, prepared);
        let accepted = {
            let mut storage = controller.session.value().lock().unwrap();
            let simulation = storage.simulation.as_mut().unwrap();
            let accepted = options(simulation);
            simulation.set_options(accepted.clone()).unwrap();
            accepted
        };
        let footprint = application.budget().usage();
        let exported = controller.project_document().unwrap();
        assert_eq!(application.budget().usage(), footprint);
        assert!(application.active().is_none());
        assert!(controller.report().is_none());
        let wire: Value = serde_json::from_slice(&exported).unwrap();
        assert_eq!(wire["contents"]["input"]["kind"], "configuration");
        assert_eq!(
            wire["contents"]["input"]["document"]
                .as_str()
                .unwrap()
                .as_bytes(),
            bytes
        );
        assert_eq!(
            wire["contents"]["options"],
            serde_json::to_value(&accepted).unwrap()
        );
        let project = Project::from_bytes(&exported).unwrap();
        let mut restored = project.upstream_simulation().unwrap();
        assert_eq!(restored.options(), &accepted);
        assert_eq!(restored.receiver_observation_policy(), policy());
        let mut reference = Simulation::new(Default::default()).unwrap();
        reference.configure_json(bytes).unwrap();
        reference.set_options(accepted).unwrap();
        reference.set_receiver_observation_policy(policy()).unwrap();
        let reference = reference.run().unwrap();
        assert_report(&restored.run().unwrap(), &reference);
        export_reference("configuration", index, &exported, &reference);
    }
}

#[test]
fn measured_export_retains_complete_exact_plan_and_original_provenance_both_modes() {
    for (index, source) in [MEASURED_50, MEASURED_25].into_iter().enumerate() {
        let mut original = b" \n".to_vec();
        original.extend_from_slice(source);
        original.extend_from_slice(b"\n ");
        let application = ApplicationSessions::desktop().unwrap();
        let baseline = application.budget().usage();
        let prepared = PreparedSession::from_measured(
            MeasuredExperiment::from_json(&original).unwrap(),
            policy(),
            application.budget(),
        )
        .unwrap();
        let controller = controller(&application, prepared);
        let expected = {
            let storage = controller.session.value().lock().unwrap();
            assert_eq!(
                storage.measured_source.as_ref().unwrap().value().as_ref(),
                original
            );
            let scientific = storage
                .simulation
                .as_ref()
                .unwrap()
                .retained_footprint()
                .unwrap();
            let mut expected = super::super::session_resources(scientific).unwrap();
            expected.payload_bytes += original.len();
            expected
        };
        let usage = application.budget().usage();
        assert_eq!(
            usage.payload_bytes - baseline.payload_bytes,
            expected.payload_bytes
        );
        assert_eq!(
            usage.ordered_entries - baseline.ordered_entries,
            expected.ordered_entries
        );
        assert_eq!(
            usage.shared_roots - baseline.shared_roots,
            expected.shared_roots
        );
        let exported = controller.project_document().unwrap();
        assert_eq!(application.budget().usage(), usage);
        let wire: Value = serde_json::from_slice(&exported).unwrap();
        assert_eq!(wire["contents"]["input"]["kind"], "measured");
        assert_eq!(
            wire["contents"]["input"]["document"]
                .as_str()
                .unwrap()
                .as_bytes(),
            original
        );
        let mut restored = Project::from_bytes(&exported)
            .unwrap()
            .upstream_simulation()
            .unwrap();
        let mut reference = Simulation::new(Default::default()).unwrap();
        reference
            .configure_measured(
                MeasuredExperiment::from_json(&original)
                    .unwrap()
                    .admit()
                    .unwrap(),
            )
            .unwrap();
        reference.set_receiver_observation_policy(policy()).unwrap();
        let reference = reference.run().unwrap();
        assert_report(&restored.run().unwrap(), &reference);
        export_reference("measured", index, &exported, &reference);
        let alias = std::sync::Arc::clone(&controller.session);
        drop(controller);
        assert_eq!(application.budget().usage(), usage);
        drop(alias);
        assert_eq!(application.budget().usage(), baseline);
    }
}

#[test]
fn typed_measured_fallback_uses_public_export_and_failed_retention_rolls_back() {
    let parsed = MeasuredExperiment::from_json(MEASURED_50).unwrap();
    let typed: MeasuredExperiment =
        serde_json::from_value(serde_json::to_value(&parsed).unwrap()).unwrap();
    assert!(typed.original_bytes().is_none());
    let expected = typed.to_json().unwrap();
    let application = ApplicationSessions::desktop().unwrap();
    let prepared =
        PreparedSession::from_measured(typed, Default::default(), application.budget()).unwrap();
    let controller = controller(&application, prepared);
    let wire: Value = serde_json::from_slice(&controller.project_document().unwrap()).unwrap();
    assert_eq!(wire["contents"]["input"]["document"], expected);

    // Source storage alone fits; final scientific/session admission must fail.
    let tight = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: MEASURED_50.len(),
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        ..Default::default()
    });
    assert!(matches!(
        PreparedSession::from_measured(parsed, Default::default(), &tight),
        Err(AdmissionError::Retention(_))
    ));
    assert_eq!(tight.usage(), RetainedResources::default());
}

#[test]
fn competing_inspection_rejects_project_export_without_changing_source_or_charge() {
    let application = ApplicationSessions::desktop().unwrap();
    let prepared = PreparedSession::from_document(
        ConfigurationDocument::from_json(CONFIGURATION_50).unwrap(),
        Default::default(),
        application.budget(),
    )
    .unwrap();
    let controller = controller(&application, prepared);
    let charge = application.budget().usage();
    let storage = controller.session.value().lock().unwrap();
    assert_eq!(controller.project_document().unwrap_err(), "Busy");
    assert_eq!(application.budget().usage(), charge);
    assert_eq!(
        storage
            .simulation
            .as_ref()
            .unwrap()
            .document()
            .unwrap()
            .original_bytes(),
        Some(CONFIGURATION_50)
    );
    drop(storage);
    assert!(controller.project_document().is_ok());
}

#[test]
fn canonical_export_keeps_accepted_signed_zero_edit_distinct_from_original() {
    let mut original = ConfigurationDocument::from_json(CONFIGURATION_50).unwrap();
    original.simulation.channel = Some(Default::default());
    let original = original.to_json().unwrap();
    let mut accepted = ConfigurationDocument::from_json(original.as_bytes()).unwrap();

    // Signed zero is a transport representation boundary, not a new phase limit.
    accepted.simulation.channel.as_mut().unwrap().phase_radians = -0.0;
    let application = ApplicationSessions::desktop().unwrap();
    let prepared =
        PreparedSession::from_document(accepted, Default::default(), application.budget()).unwrap();
    let controller = controller(&application, prepared);
    let bytes = controller.project_document().unwrap();
    let restored = Project::from_bytes(&bytes)
        .unwrap()
        .upstream_simulation()
        .unwrap();
    let document = restored.document().unwrap();
    assert!(document
        .simulation
        .channel
        .as_ref()
        .unwrap()
        .phase_radians
        .is_sign_negative());
    assert_eq!(document.original_bytes(), Some(original.as_bytes()));
}
