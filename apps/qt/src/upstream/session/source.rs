/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Original project input ownership over the existing admitted scientific session.
use super::{
    project::{self, Input},
    AdmissionError, ControllerError, MeasuredExperiment, Retained, RetainedResources,
    RetentionBudget, TryLockError, UpstreamController,
};

/// Retains exact measured-plan input after public preflight, before final adoption.
///
/// # Arguments
/// * `plan` - Complete admitted capture/association/replay source, never reconstructed.
/// * `budget` - Existing application counter, including old and candidate owners.
///
/// # Returns
/// A separately charged exact-size byte owner. Its inline guard is counted once
/// by `SessionStorage`'s fixed wrapper; only its boxed bytes are charged here.
/// Typed candidates without original JSON use the public validated exporter.
///
/// # Errors
/// Public export or aggregate admission failure drops only the candidate. No run
/// begins and no previously accepted input, controller or result is modified.
pub(super) fn retain_measured_source(
    plan: &MeasuredExperiment,
    budget: &RetentionBudget,
) -> Result<Retained<Box<[u8]>>, AdmissionError> {
    let exported;
    let bytes = match plan.original_bytes() {
        Some(bytes) => bytes,
        None => {
            exported = plan.to_json().map_err(AdmissionError::Backend)?;
            exported.as_bytes()
        }
    };
    let reservation = budget
        .reserve_resources(RetainedResources {
            payload_bytes: bytes.len(),
            ..Default::default()
        })
        .map_err(AdmissionError::Retention)?;
    Ok(reservation.retain(bytes.into()))
}

impl UpstreamController {
    /// Serializes an idle upstream direction project through public admission.
    ///
    /// # Returns
    /// Complete owned schema-3 bytes with the exact original configuration or
    /// measured plan, actual accepted options and receiver observation policy.
    /// This finite operation must execute off the GUI thread. The returned bytes
    /// are temporary construction data; a caller retaining them must separately
    /// admit their actual capacity under the existing application budget.
    /// No source, run, destination or accepted session state advances.
    ///
    /// # Errors
    /// Busy while execution/another inspector owns the scientific slot, missing
    /// source for unsupported typed-only construction, invalid public project or
    /// serialization/byte limits. Existing results and original inputs survive.
    pub fn project_document(&self) -> Result<Vec<u8>, String> {
        if self.active.is_some() {
            return Err(format!("{:?}", ControllerError::Busy));
        }
        let storage = match self.session.value().try_lock() {
            Ok(storage) => storage,
            Err(TryLockError::Poisoned(cause)) => cause.into_inner(),
            Err(TryLockError::WouldBlock) => {
                return Err(format!("{:?}", ControllerError::Busy));
            }
        };
        let simulation = storage
            .simulation
            .as_ref()
            .ok_or("Upstream scientific session is busy.")?;
        let current;
        let input = match &storage.measured_source {
            Some(source) => Input::Measured(source.value()),
            None => {
                let document = simulation
                    .document()
                    .ok_or("Upstream project has no retained configuration document.")?;
                current = document.to_json().map_err(|error| error.to_string())?;
                match document.original_bytes() {
                    Some(bytes) => {
                        let original = merlin::upstream::ConfigurationDocument::from_json(bytes)
                            .map_err(|error| error.to_string())?;
                        let canonical = original.to_json().map_err(|error| error.to_string())?;
                        if canonical == current {
                            Input::Configuration(bytes)
                        } else {
                            Input::EditedConfiguration {
                                document: current.as_bytes(),
                                original_document: bytes,
                            }
                        }
                    }
                    None => Input::Configuration(current.as_bytes()),
                }
            }
        };
        project::encode(
            input,
            simulation.options(),
            simulation.receiver_observation_policy(),
        )
    }
}

#[cfg(test)]
#[path = "source_tests.rs"]
mod tests;
