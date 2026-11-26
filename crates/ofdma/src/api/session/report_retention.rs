/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Preflight all retained report destinations before control claim or source progress.
use super::{next_id, Simulation};
use crate::{
    api::{ApiError, ApiResult, BatchReport, RunControl},
    common::constants::{
        MAX_CONFIGURATION_DOCUMENT_BYTES, UPSTREAM_BATCH_REPORT_FORMAT,
        UPSTREAM_BATCH_REPORT_NOISE_GENERATOR, UPSTREAM_BATCH_REPORT_SCHEMA_VERSION,
    },
    error::limit,
    instruments::{receiver::ReceiverEstimateStore, ObservationStore},
    simulation::{
        preview,
        report_retention::{exact_vec, fits, snapshots, PreparedReport},
        run_observed, RunStatus,
    },
};
use merlin_api::RetainedFootprint;

impl Simulation {
    /// Bounds the next complete/partial report from the current accepted request.
    ///
    /// # Returns
    /// Checked target-layout retained payload, ordered entries and shared roots.
    /// Freezes finite snapshots and source exports without claiming a control,
    /// consuming source/RNG, changing accepted state or executing simulation.
    /// Source and independently retained old/new results need separate host charges;
    /// temporary engine builders/scratch retain their existing finite work limits.
    ///
    /// # Errors
    /// MissingSetting before configuration; existing admission/export errors and
    /// ResourceLimit on source-text or checked bound overflow, before science.
    pub fn batch_report_retained_bound(&self) -> ApiResult<RetainedFootprint> {
        Ok(self.prepare_retained_report()?.bound())
    }

    /// Runs the existing scientific loop after admitting its complete retained product.
    ///
    /// # Arguments
    /// * `control` - Original cooperative lifecycle handle; rejected preflight leaves it unchanged.
    /// * `limit` - Caller-owned final report policy in all three independent dimensions.
    ///
    /// # Returns
    /// Complete, cancelled, time-limited or failed science with exact final capacities,
    /// original exports, missing values and completed decoder/source outcomes intact.
    /// Existing logical scientific/work limits and legacy run behavior are unchanged.
    ///
    /// # Errors
    /// MissingSetting, existing preflight/export/Busy errors, or ResourceLimit for
    /// any insufficient output dimension before control claim/source generation.
    /// There is no late oversized-result rejection that discards completed science.
    pub fn run_with_control_bounded(
        &mut self,
        control: &RunControl,
        limit: RetainedFootprint,
    ) -> ApiResult<BatchReport> {
        let prepared = self.prepare_retained_report()?;
        let required = prepared.bound();
        if !fits(required, limit) {
            return Err(crate::error::limit("batch_report.retained_limit").into());
        }
        let configuration = self
            .configuration
            .as_ref()
            .ok_or(ApiError::MissingSetting)?;
        let report = run_observed(
            configuration,
            &self.options,
            next_id()?,
            self.generation,
            control,
            self.receiver_observations,
        )?;
        Ok(prepared.finish(report))
    }

    /// Freezes all source exports and final snapshot allocations before execution admission.
    fn prepare_retained_report(&self) -> ApiResult<PreparedReport> {
        let configuration = self
            .configuration
            .as_ref()
            .ok_or(ApiError::MissingSetting)?;
        self.receiver_observations
            .validate_mode(configuration.us_configuration.waveform().numerology())?;
        let planned = preview(configuration, &self.options)?;
        let source_configuration_json = self
            .document
            .as_ref()
            .map(|document| document.to_json())
            .transpose()?
            .map(|text| text.into_boxed_str().into_string());
        let source_configuration_original = self
            .document
            .as_ref()
            .and_then(|document| document.original_bytes())
            .map(|bytes| {
                check_source_size(bytes.len())?;
                Ok::<_, ApiError>(exact_vec(bytes.to_vec()))
            })
            .transpose()?;
        let source_configuration_sha256 = self
            .document
            .as_ref()
            .and_then(|document| document.original_sha256())
            .map(|text| text.into_boxed_str().into_string());
        let measurement_provenance_json = self
            .measured_provenance
            .as_ref()
            .map(|text| {
                check_source_size(text.len())?;
                Ok::<_, ApiError>(text.clone().into_boxed_str().into_string())
            })
            .transpose()?;
        let (configuration, options, planned) =
            snapshots(configuration.clone(), self.options.clone(), planned);
        let mut observations =
            ObservationStore::new(options.retained_points, &options.retained_taps)?;
        observations.receiver_estimates = ReceiverEstimateStore::new(self.receiver_observations);
        let template = BatchReport {
            format: UPSTREAM_BATCH_REPORT_FORMAT,
            schema_version: UPSTREAM_BATCH_REPORT_SCHEMA_VERSION,
            source_configuration_json,
            source_configuration_original,
            source_configuration_sha256,
            measurement_provenance_json,
            noise_generator: UPSTREAM_BATCH_REPORT_NOISE_GENERATOR,
            configuration,
            options,
            planned,
            status: RunStatus::Idle,
            grants: Vec::new(),
            not_started_grants: Vec::new(),
            generated_source_bits: 0,
            compared_source_bits: 0,
            unverified_source_bits: 0,
            bit_errors: 0,
            ber: None,
            reference_recovered: false,
            failure: None,
            elapsed_ms: 0,
            observations,
        };
        Ok(PreparedReport::new(template, self.receiver_observations)?)
    }
}

/// Checks transport-owned original/provenance text independently of numerical storage.
fn check_source_size(bytes: usize) -> ApiResult<()> {
    if bytes > MAX_CONFIGURATION_DOCUMENT_BYTES {
        return Err(limit("batch_report.source_text").into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_text_ceiling_accepts_exact_and_rejects_one_extra_byte() {
        check_source_size(MAX_CONFIGURATION_DOCUMENT_BYTES).unwrap();
        assert_eq!(
            check_source_size(MAX_CONFIGURATION_DOCUMENT_BYTES + 1)
                .unwrap_err()
                .code(),
            "ResourceLimit"
        );
    }

    #[test]
    fn source_export_rejection_leaves_accepted_state_and_control_untouched() {
        let document = crate::api::ConfigurationDocument::from_json(include_bytes!(
            "../../../../../examples/cli/upstream/us-50khz.json"
        ))
        .unwrap();
        let (_, options) = document.prepare().unwrap();
        let mut session = Simulation::new(options).unwrap();
        session.configure_document(document).unwrap();
        // Exercise the defensive export check at its private owner. The public
        // measured constructor already enforces this ceiling; no API accepts
        // this deliberately invalid internal-state fixture.
        session.measured_provenance = Some(" ".repeat(MAX_CONFIGURATION_DOCUMENT_BYTES + 1));
        let configuration = session.configuration.clone();
        let generation = session.generation;
        let control = RunControl::new();
        let progress = control.progress();
        assert_eq!(
            session
                .run_with_control_bounded(
                    &control,
                    RetainedFootprint {
                        payload_bytes: usize::MAX,
                        ordered_entries: usize::MAX,
                        shared_roots: usize::MAX,
                    }
                )
                .unwrap_err()
                .code(),
            "ResourceLimit"
        );
        assert_eq!(session.configuration, configuration);
        assert_eq!(session.generation, generation);
        assert_eq!(control.progress(), progress);
        session.measured_provenance = None;
        let bound = session.batch_report_retained_bound().unwrap();
        assert!(
            session
                .run_with_control_bounded(&control, bound)
                .unwrap()
                .reference_recovered
        );
    }

    // Independent owner-test policies, not new production/scientific limits.
    const FINALIZATION_FIXTURE_GRANTS: usize = 1;
    const CANCELLATION_FIXTURE_GRANTS: usize = 2;
    const COMPLETED_GRANTS_BEFORE_CANCELLATION: usize = 1;
    const FIXTURE_ESTIMATE_FRAMES: usize = 2;
    const FIXTURE_SOFT_CELLS: usize = 8;
    const FIXTURE_SOFT_WORDS: usize = 2;
    const SPARE_TERMINAL_FIXTURE_SLOTS: usize = 29;
    const LINEAR_ESTIMATOR_NAME: &str = "frame_pilot_linear";

    fn observed_linear_session(grants: usize) -> Simulation {
        let mut document = crate::api::ConfigurationDocument::from_json(include_bytes!(
            "../../../../../examples/cli/upstream/us-50khz.json"
        ))
        .unwrap();
        document.simulation.estimator = LINEAR_ESTIMATOR_NAME.into();
        document.simulation.grants = grants;
        let (_, options) = document.prepare().unwrap();
        let mut session = Simulation::new(options).unwrap();
        session.configure_document(document).unwrap();
        let policy = crate::instruments::ReceiverObservationPolicy::new(
            FIXTURE_ESTIMATE_FRAMES,
            crate::instruments::MAX_RECEIVER_OBSERVATION_SCALARS,
        )
        .unwrap()
        .with_soft_metrics(
            crate::instruments::SoftObservationPolicy::new(
                FIXTURE_SOFT_CELLS,
                FIXTURE_SOFT_WORDS,
                crate::instruments::MAX_RECEIVER_SOFT_WORD_LLRS,
            )
            .unwrap(),
        );
        session.set_receiver_observation_policy(policy).unwrap();
        session
    }

    // Owner representation evidence only: terminal metadata is synthetic, while
    // every completed source/word/observation value comes from actual engine work.
    // These fixtures do not execute the engine's failure or timeout algorithm.
    fn assert_synthetic_terminal_finalization(status: RunStatus) {
        let mut session = observed_linear_session(FINALIZATION_FIXTURE_GRANTS);
        let prepared = session.prepare_retained_report().unwrap();
        let bound = prepared.bound();
        let mut fixture = session.run().unwrap();
        assert_eq!(fixture.status, RunStatus::Completed);
        assert!(fixture.compared_source_bits > 0);
        assert!(fixture.receiver_observations().retained_frames > 0);
        assert!(fixture.soft_observations().retained_cells > 0);
        assert!(fixture.soft_observations().retained_words > 0);
        fixture.status = status;
        fixture.reference_recovered = false;
        fixture.failure = match status {
            RunStatus::Failed => {
                let error = crate::PhyError::ResourceLimit {
                    field: "experiment.samples",
                };
                let mut diagnostic = error.to_string();
                diagnostic.reserve_exact(SPARE_TERMINAL_FIXTURE_SLOTS);
                Some(crate::simulation::RunFailure {
                    diagnostic,
                    category: "ResourceLimit",
                    field: Some("experiment.samples"),
                })
            }
            RunStatus::TimeLimit => None,
            _ => panic!("terminal owner fixture must be explicitly selected"),
        };
        fixture.grants.reserve_exact(SPARE_TERMINAL_FIXTURE_SLOTS);
        for grant in &mut fixture.grants {
            grant.words.reserve_exact(SPARE_TERMINAL_FIXTURE_SLOTS);
            for word in &mut grant.words {
                word.payload.reserve_exact(SPARE_TERMINAL_FIXTURE_SLOTS);
                word.decoded
                    .information_bits
                    .reserve_exact(SPARE_TERMINAL_FIXTURE_SLOTS);
                word.decoded
                    .mother_codeword
                    .reserve_exact(SPARE_TERMINAL_FIXTURE_SLOTS);
            }
        }
        let expected = serde_json::to_value(&fixture).unwrap();
        let compared_bits = fixture.compared_source_bits;
        let normalized = prepared.finish(fixture);
        // This compares the same report before/after a representation-only move;
        // it does not compare timing or elapsed values from separate executions.
        assert_eq!(serde_json::to_value(&normalized).unwrap(), expected);
        assert_eq!(normalized.status, status);
        assert_eq!(normalized.compared_source_bits, compared_bits);
        assert!(normalized.receiver_observations().retained_frames > 0);
        assert!(normalized.soft_observations().retained_cells > 0);
        assert!(normalized.soft_observations().retained_words > 0);
        assert!(fits(normalized.retained_footprint().unwrap(), bound));
        assert_eq!(normalized.grants.capacity(), normalized.grants.len());
        for grant in &normalized.grants {
            assert_eq!(grant.words.capacity(), grant.words.len());
            for word in &grant.words {
                assert_eq!(word.payload.capacity(), word.payload.len());
                assert_eq!(
                    word.decoded.information_bits.capacity(),
                    word.decoded.information_bits.len()
                );
                assert_eq!(
                    word.decoded.mother_codeword.capacity(),
                    word.decoded.mother_codeword.len()
                );
            }
        }
        if let Some(failure) = &normalized.failure {
            assert_eq!(failure.diagnostic.capacity(), failure.diagnostic.len());
        }
    }

    #[test]
    fn synthetic_failed_terminal_fixture_normalizes_without_losing_completed_science() {
        assert_synthetic_terminal_finalization(RunStatus::Failed);
    }

    #[test]
    fn synthetic_time_limit_terminal_fixture_normalizes_without_losing_completed_science() {
        assert_synthetic_terminal_finalization(RunStatus::TimeLimit);
    }

    #[test]
    fn pilot_linear_observed_cancel_preserves_completed_estimate_soft_and_decoder_records() {
        let mut session = observed_linear_session(CANCELLATION_FIXTURE_GRANTS);
        let bound = session.batch_report_retained_bound().unwrap();
        let control = RunControl::new();
        control.cancel_after_completed_grants_for_test(COMPLETED_GRANTS_BEFORE_CANCELLATION);
        let cancelled = session.run_with_control_bounded(&control, bound).unwrap();
        assert_eq!(cancelled.status, RunStatus::Cancelled);
        assert_eq!(cancelled.grants.len(), COMPLETED_GRANTS_BEFORE_CANCELLATION);
        assert_eq!(
            cancelled.not_started_grants.len(),
            CANCELLATION_FIXTURE_GRANTS - COMPLETED_GRANTS_BEFORE_CANCELLATION
        );
        assert!(cancelled.compared_source_bits > 0);
        assert!(cancelled.receiver_observations().retained_frames > 0);
        assert!(cancelled.soft_observations().retained_cells > 0);
        assert!(cancelled.soft_observations().retained_words > 0);
        assert!(fits(cancelled.retained_footprint().unwrap(), bound));
        assert_eq!(control.progress().status, RunStatus::Cancelled);

        // A new legacy control completes the same frozen science; compare the
        // actual retained prefix without requiring equal run identities or time.
        let reference = session.run().unwrap();
        let mut grant = cancelled.grants[0].clone();
        grant.identity.run = reference.grants[0].identity.run;
        assert_eq!(grant, reference.grants[0]);
        let mut estimate = cancelled
            .receiver_estimate(crate::instruments::ReceiverEstimateRequest::default())
            .unwrap();
        let reference_estimate = reference
            .receiver_estimate(crate::instruments::ReceiverEstimateRequest::default())
            .unwrap();
        estimate.frame.identity.run = reference_estimate.frame.identity.run;
        assert_eq!(estimate, reference_estimate);
        assert!(matches!(
            estimate.estimator,
            crate::configuration::Estimator::FramePilotLinear { .. }
        ));
        let mut word = cancelled
            .soft_word(crate::instruments::SoftWordRequest::default())
            .unwrap();
        let reference_word = reference
            .soft_word(crate::instruments::SoftWordRequest::default())
            .unwrap();
        word.word.identity.run = reference_word.word.identity.run;
        assert_eq!(word, reference_word);
        let mut cells = cancelled
            .soft_cells(crate::instruments::SoftCellRequest::default())
            .unwrap();
        let reference_cells = reference
            .soft_cells(crate::instruments::SoftCellRequest::default())
            .unwrap();
        for (cell, expected) in cells.cells.iter_mut().zip(&reference_cells.cells) {
            cell.identity.run = expected.identity.run;
        }
        assert_eq!(cells, reference_cells);
    }
}
