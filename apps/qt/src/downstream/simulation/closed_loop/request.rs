/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Passive request translation; scientific admission belongs to the public API.
use crate::downstream::{
    help::AdmissionError,
    inputs::public_inputs::transmitter_inputs,
    workbench::{project_file::decode, session::Inputs},
};
use merlin::downstream::{inputs::CaptureLoader, *};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Shared original sections are supplied explicitly for background Open as well as edits.
pub(crate) type Sections = BTreeMap<String, Value>;
pub(crate) const MAX_DURATION_MS: u64 = MAX_FULL_LOOP_DURATION_MS;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Options {
    pub receiver_mode: ReceiverMode,
    pub channel_filter: ChannelFilterSettings,
    pub mce: MceSettings,
    #[serde(default = "legacy_all_profiles")]
    pub all_profiles: bool,
    pub selected_profile_ids: Option<Vec<u8>>,
    pub duration_ms: u64,
    pub max_iterations: usize,
    pub max_runtime_ms: u64,
    pub noise_seed: u64,
}

impl Default for Options {
    /// Copies the authoritative public run defaults for a newly configured UI.
    /// # Returns
    /// Independent editable options; new sessions select only the applied profile.
    /// Legacy deserialization still explicitly defaults a missing all_profiles to true.
    fn default() -> Self {
        let backend = FullLoopOptions::default();
        Self {
            receiver_mode: backend.receiver_mode,
            channel_filter: ChannelFilterSettings::default(),
            mce: backend.mce,
            all_profiles: false,
            selected_profile_ids: None,
            duration_ms: backend.duration_ms,
            max_iterations: backend.max_iterations,
            max_runtime_ms: backend.max_runtime_ms,
            noise_seed: backend.noise_seed,
        }
    }
}

fn legacy_all_profiles() -> bool {
    true
}

impl Options {
    /// Checks finite public policies and selected identities without requiring all inputs.
    /// # Arguments
    /// * `inputs` - Accepted catalog; empty while the workbench is incomplete.
    /// # Errors
    /// Rejects invalid domain limits, repeated or absent selected identities.
    pub(crate) fn validate(&self, inputs: &Inputs) -> Result<(), String> {
        self.mce.validate().map_err(|cause| cause.to_string())?;
        self.channel_filter
            .validate()
            .map_err(|cause| cause.to_string())?;
        self.backend()
            .validate()
            .map_err(|_| "Invalid workbench run limits.".to_owned())?;
        if let Some(ids) = &self.selected_profile_ids {
            let mut unique = std::collections::BTreeSet::new();
            for id in ids {
                if ProfileId::new(*id).is_err() || !unique.insert(*id) {
                    return Err("Choose unique captured profile IDs from 0 to 15.".into());
                }
                if !inputs
                    .profiles
                    .iter()
                    .chain(inputs.profile.iter())
                    .any(|profile| profile.profile_id == *id)
                {
                    return Err(format!(
                        "Selected profile {id} is not present in the accepted DPD."
                    ));
                }
            }
        }
        Ok(())
    }

    fn backend(&self) -> FullLoopOptions {
        FullLoopOptions {
            duration_ms: self.duration_ms,
            max_iterations: self.max_iterations,
            max_runtime_ms: self.max_runtime_ms,
            noise_seed: self.noise_seed,
            receiver_mode: self.receiver_mode,
            mce: self.mce.clone(),
            ..FullLoopOptions::default()
        }
    }
}

fn original(value: &Value) -> Result<NamedCapture, AdmissionError> {
    let (name, bytes) = decode(value)?;
    NamedCapture::new(name, bytes).map_err(AdmissionError::classified)
}

fn admit(
    transmission: &TransmissionSettings,
    inputs: &Inputs,
    options: &Options,
    sections: &Sections,
    selected: Option<ProfileId>,
) -> Result<(FullLoopRequest, ProfileCapture), AdmissionError> {
    options.validate(inputs)?;
    let current = inputs
        .profile
        .as_ref()
        .ok_or("Apply DPD before configuring Receiver.")?;
    let accepted_source = inputs
        .source
        .as_ref()
        .ok_or("Apply PRBS-TX source before configuring Receiver.")?;
    let playback = inputs
        .playback
        .as_ref()
        .ok_or("Apply RxMER before configuring Receiver.")?;
    let source = transmitter_inputs(
        sections.get("source").unwrap_or(&Value::Null),
        sections.get("profile").unwrap_or(&Value::Null),
    )?
    .ok_or("Apply source and DPD originals before configuring Receiver.")?;
    if source
        .source
        .prbs_seed()
        .is_some_and(|seed| seed.value() != accepted_source.seed)
    {
        return Err("Accepted source seed differs from its original editor section.".into());
    }
    let transmission =
        TransmissionConfiguration::new(transmission.clone()).map_err(AdmissionError::classified)?;
    let profile_id = match selected {
        Some(id) => id,
        None => ProfileId::new(current.profile_id).map_err(AdmissionError::classified)?,
    };
    let profile = source
        .capture
        .simulation_profile(profile_id)
        .map_err(AdmissionError::classified)?;
    let saved = sections
        .get("measurements")
        .ok_or("Apply RxMER originals before configuring Receiver.")?;
    let originals = saved["rxmer"]
        .as_array()
        .ok_or("Missing accepted RxMER originals.")?;
    let captures = playback
        .frames
        .iter()
        .map(|frame| {
            let stored = originals
                .iter()
                .find(|value| value["sha256"].as_str() == Some(frame.source.sha256.as_str()))
                .ok_or("Accepted RxMER has no matching original capture.")?;
            RxMer::from_frame(&original(stored)?, frame.clone()).map_err(AdmissionError::classified)
        })
        .collect::<Result<Vec<_>, AdmissionError>>()?;
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        captures,
        MeasurementSequenceOptions {
            mode: playback.mode,
            interval_symbols: playback.interval_symbols,
            tolerance: AssociationTolerance::new(playback.tolerance_seconds)
                .map_err(AdmissionError::classified)?,
        },
    )
    .map_err(AdmissionError::classified)?;
    let info = measurements.info();
    let span = (
        info.last_transition_symbol,
        info.last_transition_milliseconds,
    );
    let response = if inputs.response.is_some() {
        Some(
            CaptureLoader::channel_estimate(&original(&saved["response"])?)
                .map_err(AdmissionError::classified)?,
        )
    } else {
        None
    };
    let tolerance = saved["response_tolerance"]
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or("Missing accepted channel-response tolerance.")?;
    let channel = ChannelConfiguration::new(ChannelConfigurationInput {
        additional_loss: inputs.additional_loss.clone(),
        power: inputs.power.clone(),
        filter: options.channel_filter.clone(),
    })
    .map_err(AdmissionError::classified)?;
    let prepared_source = serde_json::to_value((source.source.kind(), source.source.file_info()))
        .map_err(|cause| cause.to_string())?;
    let request = Simulation::prepare_full_loop_with_response_tolerance(
        FullLoopInputs {
            transmission: transmission.clone(),
            profile,
            source: source.source,
            measurements,
            channel,
            response,
        },
        options.backend(),
        AssociationTolerance::new(tolerance).map_err(AdmissionError::classified)?,
    )
    .map_err(|cause| AdmissionError::from_run(cause, &transmission, options.duration_ms, span))?;

    // Compare existing bounded source metadata, never serialize or reconstruct raw payloads.
    // The public request owns exact file bytes; this catches mismatched background snapshots.
    let accepted = serde_json::to_value((
        accepted_source.config.kind(),
        accepted_source.config.file_info(),
    ))
    .map_err(|cause| cause.to_string())?;
    if accepted != prepared_source {
        return Err("Accepted source differs from its original editor section.".into());
    }
    let policy = FullLoopObservationPolicy::new(FullLoopObservationInput::default())
        .map_err(AdmissionError::classified)?;
    let request = request
        .with_observations(policy)
        .map_err(AdmissionError::classified)?;
    Ok((request, source.capture))
}

/// Prepares one immutable public request from accepted inputs and exact originals.
/// # Arguments
/// * `transmission`, `inputs`, `options` - Accepted geometry, snapshots and run policy.
/// * `sections` - Original saved editor inputs paired with those snapshots.
/// # Returns
/// An idle request; no source, RNG or receiver state advances.
/// # Errors
/// Preserves original integrity, scientific admission and contextual help failures.
pub(crate) fn prepare(
    transmission: &TransmissionSettings,
    inputs: &Inputs,
    options: &Options,
    sections: &Sections,
) -> Result<FullLoopRequest, AdmissionError> {
    admit(transmission, inputs, options, sections, None).map(|(request, _)| request)
}

/// Preflights a finite sorted profile sweep through the public catalog owner.
/// # Arguments
/// * `transmission`, `inputs`, `options`, `sections` - Paired accepted state and originals.
/// # Returns
/// An immutable serial dispatch plan with every selected profile validated before execution.
/// # Errors
/// Rejects missing, duplicate, unsupported or incompatible profiles and aggregate resource bounds.
pub(crate) fn prepare_sweep(
    transmission: &TransmissionSettings,
    inputs: &Inputs,
    options: &Options,
    sections: &Sections,
) -> Result<FullLoopSweepPlan, AdmissionError> {
    let selection = if let Some(ids) = &options.selected_profile_ids {
        FullLoopProfileSelection::Selected(
            ids.iter()
                .map(|id| ProfileId::new(*id))
                .collect::<ApiResult<Vec<_>>>()
                .map_err(AdmissionError::classified)?,
        )
    } else if options.all_profiles {
        FullLoopProfileSelection::AllCaptured
    } else {
        FullLoopProfileSelection::Current
    };
    // The old UI preflights selected profiles only, even when the applied profile is excluded.
    let first = match &selection {
        FullLoopProfileSelection::Selected(ids) => ids.iter().min_by_key(|id| id.value()).copied(),
        FullLoopProfileSelection::AllCaptured => inputs
            .profiles
            .iter()
            .map(|profile| profile.profile_id)
            .min()
            .map(ProfileId::new)
            .transpose()
            .map_err(AdmissionError::classified)?,
        FullLoopProfileSelection::Current => None,
    };
    let (request, catalog) = admit(transmission, inputs, options, sections, first)?;
    FullLoopSweepPlan::new(&request, &catalog, selection).map_err(AdmissionError::classified)
}
