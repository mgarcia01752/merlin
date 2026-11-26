/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual inline Options children and persistent error ownership on its host.
use super::{Model, Options};
use crate::{
    downstream::inputs::retention as count,
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::{Retained, RetainedResources, RetentionBudget},
};
use merlin::downstream::{ChannelFilterSettings, EchoPath, MceSettings, ReceiverMode};

/// Counts only dynamic children of the existing inline Options value.
/// # Arguments
/// * `options` - Actual final owner, preserving requested spare capacities.
/// # Returns
/// Selected-ID and Echoes buffers; the outer Model owns all fixed fields once.
/// # Errors
/// Checked actual buffer arithmetic, without mutation or scientific execution.
pub(super) fn options_children(options: &Options) -> Result<RetainedResources, String> {
    let Options {
        receiver_mode,
        channel_filter,
        mce,
        all_profiles,
        selected_profile_ids,
        duration_ms,
        max_iterations,
        max_runtime_ms,
        noise_seed,
    } = options;
    match receiver_mode {
        ReceiverMode::CmChannelEstimated
        | ReceiverMode::PilotEstimated
        | ReceiverMode::MerlinChannelEstimated => {}
    }
    let MceSettings {
        minimum_variance,
        averaging_symbols,
    } = mce;
    let mut total = count::fixed_children(&(
        *receiver_mode,
        *minimum_variance,
        *averaging_symbols,
        *all_profiles,
        *duration_ms,
        *max_iterations,
        *max_runtime_ms,
        *noise_seed,
    ));
    if let Some(ids) = selected_profile_ids {
        count::append(&mut total, count::vector_storage(ids)?)?;
    }
    match channel_filter {
        ChannelFilterSettings::Diagonal => {}
        ChannelFilterSettings::Echoes { paths } => {
            count::append(&mut total, count::vector_storage(paths)?)?;
            for EchoPath {
                delay_samples,
                gain,
            } in paths
            {
                count::append(&mut total, count::fixed_children(&(*delay_samples, *gain)))?;
            }
        }
        ChannelFilterSettings::CapturedFir {
            max_delay_samples,
            causal_shift_samples,
            maximum_rms_error,
            maximum_bin_error,
        } => {
            count::append(
                &mut total,
                count::fixed_children(&(
                    *max_delay_samples,
                    *causal_shift_samples,
                    *maximum_rms_error,
                    *maximum_bin_error,
                )),
            )?;
        }
    }
    Ok(total)
}

/// Reserves only final inline Options children before committing their owner.
/// # Arguments
/// * `options` - Actual selected-ID/echo buffers, or no configured Options.
/// * `budget` - Exact accepting host, including still-live old owners.
/// # Returns
/// A host-identity witness for every Some, including zero dynamic children.
/// Its fixed layout belongs to Model and is not counted again here.
/// # Errors
/// Checked actual child metadata or any host limit before mutation.
pub(super) fn reserve_options(
    options: Option<&Options>,
    budget: &RetentionBudget,
) -> Result<Option<Retained<()>>, String> {
    options
        .map(|options| {
            budget
                .reserve_resources(options_children(options)?)
                .map(|guard| guard.retain(()))
                .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())
        })
        .transpose()
}

impl Model {
    /// Admits new text and a real prior Diagnostic clone before replacing status.
    /// # Arguments
    /// * `message` - Independent finite candidate; original source stays guarded.
    /// # Returns
    /// Unit after normal detailed admission or borrowed fallback, without science changes.
    pub(super) fn set_message(&mut self, message: String) {
        self.error = self.error.replace_message(
            message,
            self.context.application.budget(),
            WORKER_FAILURE_ADMISSION_MESSAGE,
        );
    }
    /// Borrows literal status unless preserving prior Diagnostic needs admission.
    /// # Arguments
    /// * `message` - Existing static status wording.
    /// # Returns
    /// Unit after replacing status while retaining original help when admitted.
    pub(super) fn set_static_message(&mut self, message: &'static str) {
        self.error = self.error.replace_static_message(
            message,
            self.context.application.budget(),
            WORKER_FAILURE_ADMISSION_MESSAGE,
        );
    }
    /// Moves eligible queued detail, or admits a message/prior-help replacement.
    /// # Arguments
    /// * `error` - Still-guarded worker source, disposed only after replacement admission.
    /// # Returns
    /// Unit without copying an eligible same-host pointee or losing completed science.
    pub(super) fn set_queued_error(&mut self, error: WorkerFailure) {
        if self.error.diagnostic().is_none() && error.owns(self.context.application.budget()) {
            self.error = error;
        } else {
            self.set_message(error.message().to_owned());
        }
    }
    /// Checks Options/error target coverage before attachment or project adoption.
    /// # Arguments
    /// * `budget` - Canonical target identity, not merely an equal ceiling.
    /// # Returns
    /// Unit for every present owner and its matching child-only host witness.
    /// # Errors
    /// Missing/foreign witnesses or detailed status, without mutations or fallback.
    pub(super) fn check_metadata_target(&self, budget: &RetentionBudget) -> Result<(), String> {
        if !self.error.owns(budget)
            || match (&self.options, &self.options_guard) {
                (Some(_), Some(guard)) => !budget.owns(guard),
                (None, None) => false,
                _ => true,
            }
        {
            return Err("Prepared receiver metadata belongs to another application host.".into());
        }
        Ok(())
    }
}
