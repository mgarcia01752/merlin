/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in final report destinations; temporary engine builders remain separately bounded.
use super::{BatchReport, ExperimentConfig, ExperimentOptions, ExperimentPreview, GrantReport};
use crate::{
    error::limit,
    instruments::{receiver::ReceiverEstimateStore, ObservationStore, ReceiverObservationPolicy},
    receiver::DecodedWord,
    PhyResult,
};
use merlin_api::{
    retention_support::{capacity_bytes, sum},
    RetainedFootprint,
};

/// Preserved failure-text ceiling, distinct from transport/source-export policy.
///
/// Current run failures contain only the closed PhyError/LdpcError variants,
/// static ASCII field literals and at most two decimal usize values. Their full
/// Debug rendering is far below this ceiling on either target pointer width.
/// No user text, captured bytes or arbitrary Display implementation enters it.
/// Keep the finite error-variant/field audit when extending those owners; never
/// truncate a diagnostic or replace completed science on a late size check.
const MAX_BATCH_FAILURE_DIAGNOSTIC_BYTES: usize = 16 * 1024 * 1024;

/// Removes excess requested capacity using the Box<[T]> -> Vec<T> exact-layout contract.
///
/// # Arguments
/// * `values` - Owned finite builder storage, including any initialized nested owners.
///
/// # Returns
/// Identical values/order; the nonzero-sized report payloads used here have
/// capacity equal to length, without a growth-factor assumption.
pub(crate) fn exact_vec<Value>(values: Vec<Value>) -> Vec<Value> {
    values.into_boxed_slice().into_vec()
}

/// Checks each independent caller-owned output dimension without modifying a control.
///
/// # Returns
/// True only if payload, ordered entries and shared owner edges all fit.
pub(crate) fn fits(required: RetainedFootprint, limit: RetainedFootprint) -> bool {
    required.payload_bytes <= limit.payload_bytes
        && required.ordered_entries <= limit.ordered_entries
        && required.shared_roots <= limit.shared_roots
}

/// Prebuilt final snapshots and their checked maximum variable destinations.
pub(crate) struct PreparedReport {
    template: BatchReport,
    bound: RetainedFootprint,
}

impl PreparedReport {
    /// Admits every retained destination after source exports and snapshots are frozen.
    ///
    /// # Returns
    /// Owned final destination and checked target-layout bound; no control/source advances.
    ///
    /// # Errors
    /// ResourceLimit on any plan/count/capacity arithmetic overflow.
    pub(crate) fn new(template: BatchReport, policy: ReceiverObservationPolicy) -> PhyResult<Self> {
        let base = template.retained_footprint()?;
        let additional = additional_bound(&template, policy)
            .ok_or_else(|| limit("batch_report.retained_bound"))?;
        let bound = sum([Some(base), Some(additional)])
            .ok_or_else(|| limit("batch_report.retained_bound"))?;
        Ok(Self { template, bound })
    }

    /// Reads the complete already checked bound without producing science.
    ///
    /// # Returns
    /// The independent final retained payload/entry/shared-edge quantities.
    pub(crate) fn bound(&self) -> RetainedFootprint {
        self.bound
    }

    /// Moves all completed/partial science into exact final capacities.
    ///
    /// # Arguments
    /// * `built` - Original engine result, including failed words and missing values.
    ///
    /// # Returns
    /// The complete report, without a late ResourceLimit path or discarded science.
    /// The debug assertion audits the pre-production proof, not caller admission.
    pub(crate) fn finish(self, built: BatchReport) -> BatchReport {
        let BatchReport {
            format,
            schema_version,
            source_configuration_json: _,
            source_configuration_original: _,
            source_configuration_sha256: _,
            measurement_provenance_json: _,
            noise_generator,
            configuration: _,
            options: _,
            planned: _,
            status,
            grants,
            not_started_grants,
            generated_source_bits,
            compared_source_bits,
            unverified_source_bits,
            bit_errors,
            ber,
            reference_recovered,
            mut failure,
            elapsed_ms,
            mut observations,
        } = built;
        let grants = grants.into_iter().map(compact_grant).collect();
        if let Some(failure) = &mut failure {
            let super::RunFailure {
                diagnostic,
                category: _,
                field: _,
            } = failure;
            *diagnostic = std::mem::take(diagnostic).into_boxed_str().into_string();
        }
        observations.compact_report_payload();
        let BatchReport {
            format: _,
            schema_version: _,
            source_configuration_json,
            source_configuration_original,
            source_configuration_sha256,
            measurement_provenance_json,
            noise_generator: _,
            configuration,
            options,
            planned,
            status: _,
            grants: _,
            not_started_grants: _,
            generated_source_bits: _,
            compared_source_bits: _,
            unverified_source_bits: _,
            bit_errors: _,
            ber: _,
            reference_recovered: _,
            failure: _,
            elapsed_ms: _,
            observations: _,
        } = self.template;
        let report = BatchReport {
            format,
            schema_version,
            source_configuration_json,
            source_configuration_original,
            source_configuration_sha256,
            measurement_provenance_json,
            noise_generator,
            configuration,
            options,
            planned,
            status,
            grants: exact_vec(grants),
            not_started_grants: exact_vec(not_started_grants),
            generated_source_bits,
            compared_source_bits,
            unverified_source_bits,
            bit_errors,
            ber,
            reference_recovered,
            failure,
            elapsed_ms,
            observations,
        };
        debug_assert!(report
            .retained_footprint()
            .is_ok_and(|actual| fits(actual, self.bound)));
        report
    }
}

/// Counts each possible initialized word's three independently retained decision vectors.
fn decision_bytes(plan: &crate::grant::GrantPlan, grants: usize) -> Option<usize> {
    plan.codewords
        .iter()
        .try_fold(0_usize, |bytes, word| {
            bytes.checked_add(
                word.code
                    .parameters()
                    .codeword_bits
                    .checked_add(word.information_bits.checked_mul(2)?)?,
            )
        })?
        .checked_mul(grants)
}

/// Bounds future capacities from the accepted plan, not from observed completion.
fn additional_bound(
    report: &BatchReport,
    policy: ReceiverObservationPolicy,
) -> Option<RetainedFootprint> {
    let grants = report.options.grants;
    let plan = &report.planned.grant_plan;
    let words = plan.codewords.len().checked_mul(grants)?;
    let frames = report
        .configuration
        .us_configuration
        .grant()
        .frames
        .checked_mul(grants)?;
    let word_llrs = plan
        .codewords
        .iter()
        .try_fold(0_usize, |sum, word| sum.checked_add(word.transmitted_bits))?
        .checked_mul(grants)?;
    sum([
        capacity_bytes::<GrantReport>(grants),
        capacity_bytes::<u64>(grants),
        capacity_bytes::<DecodedWord>(words),
        capacity_bytes::<u8>(decision_bytes(plan, grants)?),
        capacity_bytes::<u8>(MAX_BATCH_FAILURE_DIAGNOSTIC_BYTES),
        ObservationStore::report_retained_extra(
            report.options.retained_points,
            &report.options.retained_taps,
        ),
        ReceiverEstimateStore::report_retained_extra(
            policy,
            report
                .configuration
                .us_configuration
                .waveform()
                .numerology(),
            frames,
            words,
            word_llrs,
        ),
    ])
}

/// Compacts final nested decisions without re-decoding or changing any flags/source bytes.
fn compact_grant(mut grant: GrantReport) -> GrantReport {
    let GrantReport {
        identity: _,
        generated_source: _,
        emitted_samples: _,
        received_samples: _,
        compared_bits: _,
        unverified_source_bits: _,
        planned_transmit_samples: _,
        pending_transmit_samples: _,
        unprocessed_codewords: _,
        bit_errors: _,
        words,
        completed: _,
        missing_symbols: _,
        matched_data_mer: _,
        matched_data_mer_value: _,
    } = &mut grant;
    for word in words.iter_mut() {
        let DecodedWord {
            ordinal: _,
            payload_offset: _,
            payload,
            information_fill_valid: _,
            decoded,
        } = word;
        *payload = exact_vec(std::mem::take(payload));
        let fec_codec::upstream_ldpc::DecodeResult {
            information_bits,
            mother_codeword,
            iterations: _,
            converged: _,
            parity_valid: _,
            shortened_zeros_valid: _,
            hard_decision_changes: _,
        } = decoded;
        *information_bits = exact_vec(std::mem::take(information_bits));
        *mother_codeword = exact_vec(std::mem::take(mother_codeword));
    }
    *words = exact_vec(std::mem::take(words));
    grant
}

/// Freezes every snapshot buffer before the run; existing cached values are only copied.
///
/// # Returns
/// Original accepted values with exact requested nested capacities.
pub(crate) fn snapshots(
    mut configuration: ExperimentConfig,
    mut options: ExperimentOptions,
    mut planned: ExperimentPreview,
) -> (ExperimentConfig, ExperimentOptions, ExperimentPreview) {
    configuration.compact_report_payload();
    let ExperimentOptions {
        grants: _,
        maximum_samples: _,
        chunk_samples: _,
        timeout_ms: _,
        retained_points: _,
        retained_taps,
    } = &mut options;
    *retained_taps = exact_vec(std::mem::take(retained_taps));
    let ExperimentPreview {
        grant_plan,
        scheduled_samples: _,
        received_samples: _,
        source_bits: _,
        codewords: _,
        profile_ids,
    } = &mut planned;
    let crate::grant::GrantPlan {
        capacity_bits: _,
        transmit: _,
        codewords,
        source_bits: _,
        information_fill_bits: _,
        grant_pad_bits: _,
        unused_capacity_bits: _,
    } = grant_plan;
    *codewords = exact_vec(std::mem::take(codewords));
    *profile_ids = exact_vec(std::mem::take(profile_ids));
    (configuration, options, planned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{ConfigurationDocument, Simulation};

    const SOURCE: &[u8] = include_bytes!("../../../../examples/cli/upstream/us-50khz.json");

    #[test]
    fn plan_decision_and_capacity_products_reject_checked_overflow() {
        let document = ConfigurationDocument::from_json(SOURCE).unwrap();
        let (config, options) = document.prepare().unwrap();
        let planned = crate::simulation::preview(&config, &options).unwrap();
        assert!(!planned.grant_plan.codewords.is_empty());
        assert!(decision_bytes(&planned.grant_plan, usize::MAX).is_none());
        assert!(capacity_bytes::<GrantReport>(usize::MAX).is_none());
        // Drive the actual private aggregate path with an impossible future count,
        // without running or inventing a valid scientific configuration for it.
        let mut session = Simulation::new(options).unwrap();
        session.configure(config).unwrap();
        let mut report = session.run().unwrap();
        report.options.grants = usize::MAX;
        assert!(additional_bound(&report, ReceiverObservationPolicy::default()).is_none());
    }

    #[test]
    fn exact_slice_conversion_removes_spare_slots_preserving_every_value() {
        const SPARE_TEST_SLOTS: usize = 29;
        let mut original = Vec::with_capacity(SPARE_TEST_SLOTS);
        original.extend([Some(1_u64), None, Some(u64::MAX)]);
        let before = original.clone();
        let exact = exact_vec(original);
        assert_eq!(exact, before);
        assert_eq!(exact.capacity(), exact.len());
    }

    #[test]
    fn closed_codec_error_variants_and_full_usize_text_fit_diagnostic_policy() {
        use fec_codec::upstream_ldpc::LdpcError;
        let errors = [
            LdpcError::InformationLength,
            LdpcError::IterationLimit,
            LdpcError::InputLength {
                expected: usize::MAX,
                actual: usize::MAX,
            },
            LdpcError::NonBinary,
            LdpcError::NonFiniteLlr,
            LdpcError::MatrixDefinition,
        ];
        for error in errors {
            let error = crate::PhyError::Codec { error };
            assert!(error.to_string().len() <= MAX_BATCH_FAILURE_DIAGNOSTIC_BYTES);
        }
        // Actual longest domain path is source-audited separately; these are real
        // existing static engine diagnostics, not truncated transport substitutes.
        for error in [
            crate::PhyError::InvalidInput {
                field: "receiver.capacity",
            },
            crate::PhyError::Unsupported {
                field: "configuration.execution_mode",
            },
            crate::PhyError::ResourceLimit {
                field: "experiment.result_decisions",
            },
            crate::PhyError::Association {
                field: "source_ledger.coverage",
            },
        ] {
            assert!(error.to_string().len() <= MAX_BATCH_FAILURE_DIAGNOSTIC_BYTES);
        }
    }
}
