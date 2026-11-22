/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Full-loop preparation and execution delegate every scientific stage to existing owners.
use super::*;
use crate::{
    scenario::document::ScenarioDocument,
    simulation::{
        channel::{
            job::run_notified_with_cancellation,
            measurement::{captured_layout, MeasurementSelection},
            ChannelJobLimits, ChannelRunSettings,
        },
        test_points::TestPointProbe,
    },
};

/// Reusable frozen single-profile request, separate from legacy controller-owned experiments.
#[derive(Clone)]
pub struct FullLoopRequest {
    pub(super) settings: Arc<ChannelRunSettings>,
    pub(super) options: FullLoopOptions,
    pub(super) inputs: FullLoopInputs,
    pub(super) observation_policy: Option<FullLoopObservationPolicy>,
}
impl FullLoopRequest {
    pub(super) fn prepare(inputs: FullLoopInputs, options: FullLoopOptions) -> ApiResult<Self> {
        options.validate()?;
        inputs.transmission.check_profile(&inputs.profile)?;
        let playback = inputs
            .measurements
            .full_loop_playback(&inputs.transmission, &inputs.profile)?;
        inputs.transmission.check_response(
            &inputs.profile,
            inputs.response.as_ref(),
            AssociationTolerance::new(playback.tolerance_seconds)?,
        )?;
        let settings = inputs.transmission.settings();
        let mut run = ChannelRunSettings::new(
            settings.clone(),
            captured_layout(&inputs.profile.profile, settings)?,
        );
        run.source = inputs.source.configuration.clone();
        run.prbs_seed = inputs
            .source
            .prbs_seed()
            .map_or(0x7fff, |seed| seed.value());
        run.duration = std::time::Duration::from_millis(options.duration_ms);
        run.max_iterations = options.max_iterations;
        run.receiver_mode = options.receiver_mode;
        run.channel_filter = inputs.channel.settings().filter.clone();
        run.mce = options.mce.clone();
        run.impairments.additional_loss = inputs.channel.settings().additional_loss.clone();
        run.impairments.power = inputs.channel.settings().power.clone();
        let scenario = ScenarioDocument {
            schema_version: 1,
            protocol: ProtocolScope::Docsis31,
            profiles: vec![inputs.profile.profile.clone()],
            frames: playback.frames.clone(),
            channels: inputs
                .response
                .iter()
                .map(|response| response.frame.clone())
                .collect(),
        };
        MeasurementSelection {
            use_rxmer_noise: true,
            transitions: vec![],
            frame_index: 0,
            profile_id: inputs.profile.profile_id().value(),
            channel_index: inputs.response.as_ref().map(|_| 0),
            tolerance_seconds: playback.tolerance_seconds,
            noise_seed: options.noise_seed,
        }
        .apply(&scenario, &mut run)?;
        run.playback = Some(playback);
        run.constellation = options.constellation.clone();
        run.protected_constellation = options.protected_constellation.clone();
        run.validate()?;
        Ok(Self {
            settings: Arc::new(run),
            options,
            inputs,
            observation_policy: None,
        })
    }

    /// Borrows the frozen exact scientific settings accepted by the original channel owner.
    /// # Returns
    /// Read-only settings; editing caller options/inputs cannot change the prepared run.
    pub fn settings(&self) -> &crate::simulation::channel::ChannelRunSettings {
        &self.settings
    }

    /// Reads independent accepted options for this request.
    /// # Returns
    /// Borrowed immutable policy; run state belongs to a separate one-shot control.
    pub fn options(&self) -> &FullLoopOptions {
        &self.options
    }

    /// Executes the existing bounded full loop synchronously with safe-boundary controls.
    /// # Arguments
    /// * `control` - Fresh direction-specific token; callers dispatch outside their UI loop.
    /// # Returns
    /// Owned complete/partial actual report preserving every completed retained FEC trial.
    /// # Errors
    /// Busy for a used control; existing construction/admission failures otherwise.
    /// Cancel/runtime/step failures after construction return real partial reports.
    pub fn run(&self, control: &FullLoopControl) -> ApiResult<FullLoopResult> {
        let mut probe = self
            .observation_policy
            .as_ref()
            .map_or_else(|| Ok(TestPointProbe::disabled()), |policy| policy.probe())?;
        let mut stages = observations::RetainedStages::new(
            self.observation_policy
                .as_ref()
                .map_or(0, |policy| policy.settings().max_retained_bytes),
        );
        control.begin()?;
        let limits = ChannelJobLimits {
            max_runtime_ms: self.options.max_runtime_ms,
            max_retained_words: self.options.max_retained_words,
        };
        let job = run_notified_with_cancellation(
            &self.settings,
            &limits,
            &control.stop(),
            control.pause_gate(),
            &mut probe,
            &mut stages,
            |progress| control.progress(progress),
            |paused| control.acknowledge_pause(paused),
        );
        control.finish(job.as_ref().ok());
        Ok(FullLoopResult {
            job: Arc::new(job?),
            inputs: self.inputs.clone(),
            stages: Arc::new(stages),
            observation_policy: self.observation_policy.clone(),
        })
    }
}
