/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Strict persisted upstream settings composed through existing public owners.
use super::{invalid, limit, ProjectError};
use crate::{common::constants::MAX_MEASURED_EXPERIMENT_BYTES, upstream::MeasuredExperiment};
use docsis_ofdma::api;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UpstreamContents {
    input: Input,
    options: Options,
    receiver_observations: Observations,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum Input {
    Configuration {
        document: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        original_document: Option<String>,
    },
    Measured {
        document: String,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Options {
    grants: usize,
    maximum_samples: usize,
    chunk_samples: usize,
    timeout_ms: u64,
    retained_points: usize,
    retained_taps: Vec<api::Tap>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Observations {
    maximum_frames: usize,
    maximum_scalar_slots: usize,
    soft: Option<Soft>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Soft {
    maximum_cells: usize,
    maximum_words: usize,
    maximum_word_llrs: usize,
}

impl UpstreamContents {
    /// Composes a fresh idle session from existing configuration and policy owners.
    /// # Returns
    /// Owned configured session and checked original input byte count.
    /// # Errors
    /// Preserves source, association, execution-budget and observation-policy failures.
    pub(super) fn simulation(&self) -> Result<(api::Simulation, usize), ProjectError> {
        let options = api::ExperimentOptions {
            grants: self.options.grants,
            maximum_samples: self.options.maximum_samples,
            chunk_samples: self.options.chunk_samples,
            timeout_ms: self.options.timeout_ms,
            retained_points: self.options.retained_points,
            retained_taps: self.options.retained_taps.clone(),
        };
        let mut simulation = api::Simulation::new(options.clone())?;
        let original_bytes = match &self.input {
            Input::Configuration {
                document,
                original_document,
            } => {
                let accepted = configuration(document, original_document.as_deref())?;
                simulation.configure_document(accepted)?;
                original_document.as_ref().unwrap_or(document).len()
            }
            Input::Measured { document } => {
                if document.len() > MAX_MEASURED_EXPERIMENT_BYTES {
                    return Err(limit("Upstream measured input exceeds its byte budget"));
                }
                let measured = MeasuredExperiment::from_json(document.as_bytes())?;
                let mut sources = measured
                    .ucd_fragments
                    .iter()
                    .chain(measured.rxmer.iter())
                    .chain(measured.pre_eq_initial.iter())
                    .chain(measured.pre_eq_events.iter().map(|event| &event.source));
                let bytes = sources
                    .try_fold(document.len(), |total, source| {
                        total.checked_add(source.bytes.len())
                    })
                    .ok_or_else(|| limit("Upstream original input byte count overflow"))?;
                simulation.configure_measured(measured.admit()?)?;
                bytes
            }
        };
        // Original document options remain inspectable; these saved overrides are explicit.
        simulation.set_options(options)?;
        let mut observations = api::ReceiverObservationPolicy::new(
            self.receiver_observations.maximum_frames,
            self.receiver_observations.maximum_scalar_slots,
        )
        .map_err(api::ApiError::from)?;
        if let Some(soft) = &self.receiver_observations.soft {
            observations = observations.with_soft_metrics(
                api::SoftObservationPolicy::new(
                    soft.maximum_cells,
                    soft.maximum_words,
                    soft.maximum_word_llrs,
                )
                .map_err(api::ApiError::from)?,
            );
        }
        simulation.set_receiver_observation_policy(observations)?;
        // A validated but unconfigured session is not a valid saved upstream project.
        if simulation.configuration().is_none() {
            return Err(invalid("Upstream project has no accepted configuration"));
        }
        Ok((simulation, original_bytes))
    }
}

/// Restores accepted public fields without changing their independent source provenance.
///
/// # Arguments
/// * `document` - Complete current configuration JSON; the public owner validates it.
/// * `original` - Optional exact earlier configuration source, independently bounded
///   and validated. Absent preserves the original single-document contract.
///
/// # Returns
/// Current public fields attached to the original owner's private byte/hash identity.
/// This copies the public transport model only; `configure_document` still owns
/// scientific preparation, execution compatibility and observation validation.
///
/// # Errors
/// Preserves either document's public format, domain and byte-limit errors.
fn configuration(
    document: &str,
    original: Option<&str>,
) -> Result<api::ConfigurationDocument, ProjectError> {
    let current = api::ConfigurationDocument::from_json(document.as_bytes())?;
    let Some(original) = original else {
        return Ok(current);
    };
    let mut source = api::ConfigurationDocument::from_json(original.as_bytes())?;

    // Keep the source owner's private original Arc; assign every public field.
    source.format = current.format;
    source.schema_version = current.schema_version;
    source.presets = current.presets;
    source.selected_preset = current.selected_preset;
    source.simulation = current.simulation;
    Ok(source)
}
