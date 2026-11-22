/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Legacy project settings admitted through the existing public full-loop owners.
use super::*;

impl Workbench {
    /// Prepares a saved downstream project for explicit public full-loop execution.
    ///
    /// # Arguments
    /// * `bytes` - Complete schema 1/2 project, at most the existing 144 MiB limit.
    ///
    /// # Returns
    /// An owned idle sweep plan preserving original selected inputs, file/fill or
    /// PRBS source, measurement playback, receiver and channel settings. Neither
    /// source nor RNG advances. Existing Workbench loaders retain their contract.
    ///
    /// # Errors
    /// Preserves strict schema, integrity, resource and scientific admission errors.
    /// Unsupported versions or modes reject before any run or caller-state change.
    pub fn prepare_full_loop(bytes: &[u8]) -> ApiResult<FullLoopSweepPlan> {
        if bytes.len() > MAX_WORKBENCH_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "WB exceeds the project byte budget",
            ));
        }
        let document: Document =
            serde_json::from_slice(bytes).map_err(|cause| invalid(cause.to_string()))?;
        if ![WORKBENCH_SCHEMA_V1, WORKBENCH_SCHEMA_V2].contains(&document.schema_version)
            || document.application != "MERLIN"
        {
            return Err(unsupported("Unsupported WB schema/application"));
        }
        let receiver = document.receiver.ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingSetting,
                "WB has no accepted receiver settings",
            )
        })?;
        if document.schema_version == WORKBENCH_SCHEMA_V1 && receiver.selected_profile_ids.is_some()
        {
            return Err(invalid("Explicit profile selection requires WB schema 2"));
        }
        let transmission = TransmissionConfiguration::new(document.transmission)?;
        let sections = document.sections;
        check_original_budget(&sections)?;
        let mut decoded_bytes = 0;
        let source = source(&sections.source, &mut decoded_bytes)?;
        let profile_input = sections.profile.file.decode(&mut decoded_bytes)?;
        let catalog = CaptureLoader::profile_capture(&profile_input)?;
        let active = ProfileId::new(sections.profile.profile_id)?;
        let profile = catalog.simulation_profile(active)?;
        let selection = if let Some(ids) = receiver.selected_profile_ids {
            FullLoopProfileSelection::Selected(
                ids.into_iter()
                    .map(ProfileId::new)
                    .collect::<ApiResult<_>>()?,
            )
        } else if receiver.all_profiles {
            FullLoopProfileSelection::AllCaptured
        } else {
            FullLoopProfileSelection::Current
        };
        let measurements = measurements(
            &sections.measurements,
            &transmission,
            &profile,
            receiver.duration_ms,
            &mut decoded_bytes,
        )?;
        let response = sections
            .measurements
            .response
            .as_ref()
            .map(|stored| CaptureLoader::channel_estimate(&stored.decode(&mut decoded_bytes)?))
            .transpose()?;
        transmission.check_response(
            &profile,
            response.as_ref(),
            AssociationTolerance::new(sections.measurements.response_tolerance)?,
        )?;
        let mut channel = ChannelConfigurationInput {
            filter: receiver.channel_filter,
            ..Default::default()
        };
        if let Some(loss) = sections.channel_model {
            if loss.schema_version != CHANNEL_MODEL_SCHEMA_VERSION {
                return Err(unsupported("Unsupported WB channel model version"));
            }
            channel.additional_loss = loss.additional_loss;
        }
        if let Some(power) = sections.channel_power {
            if power.schema_version != CHANNEL_POWER_SCHEMA_VERSION {
                return Err(unsupported("Unsupported WB channel power version"));
            }
            channel.power = power.conditioning;
        }
        let request = Simulation::prepare_full_loop_with_response_tolerance(
            FullLoopInputs {
                transmission,
                profile,
                source,
                measurements,
                channel: ChannelConfiguration::new(channel)?,
                response,
            },
            FullLoopOptions {
                duration_ms: receiver.duration_ms,
                max_iterations: receiver.max_iterations,
                max_runtime_ms: receiver.max_runtime_ms,
                noise_seed: receiver.noise_seed,
                receiver_mode: receiver.receiver_mode,
                mce: receiver.mce,
                ..Default::default()
            },
            AssociationTolerance::new(sections.measurements.response_tolerance)?,
        )?;
        FullLoopSweepPlan::new(&request, &catalog, selection)
    }
}

fn check_original_budget(sections: &Sections) -> ApiResult<()> {
    if sections.measurements.rxmer.is_empty() || sections.measurements.rxmer.len() > MAX_CAPTURES {
        return Err(ApiError::new(
            ErrorCode::ResourceLimit,
            "WB requires 1..256 RxMER originals",
        ));
    }
    let mut originals = std::iter::once(&sections.profile.file)
        .chain(sections.source.file.iter())
        .chain(sections.measurements.rxmer.iter())
        .chain(sections.measurements.response.iter());
    let total = originals.try_fold(0usize, |total, stored| {
        if stored.byte_length > MAX_EMBEDDED_BYTES {
            return None;
        }
        total.checked_add(stored.byte_length)
    });
    if total.is_none_or(|total| total > MAX_TOTAL_ORIGINAL_BYTES) {
        return Err(ApiError::new(
            ErrorCode::ResourceLimit,
            "Combined WB originals exceed the input byte budget",
        ));
    }
    Ok(())
}

fn source(source: &Source, decoded_bytes: &mut usize) -> ApiResult<SourceInput> {
    // Dormant staged files are still integrity-checked, as in the UI and legacy loader.
    let file = source
        .file
        .as_ref()
        .map(|stored| stored.decode(decoded_bytes))
        .transpose()?;
    match source.options.mode.as_str() {
        "prbs15" => Ok(SourceInput::prbs15(PrbsSeed::new(source.options.seed)?)),
        "file" => {
            let file = file.ok_or_else(|| {
                ApiError::new(ErrorCode::MissingSetting, "WB file source has no original")
            })?;
            let fill = if source.options.append {
                Some(FileFillPolicy::new(
                    source.options.target_bits,
                    PrbsSeed::new(source.options.fill_seed)?,
                )?)
            } else {
                None
            };
            SourceInput::file(file, fill)
        }
        _ => Err(unsupported("Unsupported WB source mode")),
    }
}

fn measurements(
    saved: &Measurements,
    transmission: &TransmissionConfiguration,
    profile: &Dpd,
    duration_ms: u64,
    decoded_bytes: &mut usize,
) -> ApiResult<MeasurementSequence> {
    let mode = match saved.options.mode.as_str() {
        "single" => MeasurementMode::Single,
        "average" => MeasurementMode::Average,
        "hold" => MeasurementMode::Hold,
        "interpolate" => MeasurementMode::Interpolate,
        _ => return Err(unsupported("Unsupported WB RxMER playback mode")),
    };
    let interval_symbols = saved
        .options
        .interval_symbols
        .parse::<u64>()
        .map_err(|cause| invalid(cause.to_string()))?;
    let selection = saved.options.selected_captures.as_ref();
    if let Some(hashes) = selection {
        let unique: BTreeSet<_> = hashes.iter().collect();
        if hashes.is_empty()
            || unique.len() != hashes.len()
            || hashes.len() > MAX_CAPTURES
            || hashes
                .iter()
                .any(|hash| !saved.rxmer.iter().any(|stored| &stored.sha256 == hash))
        {
            return Err(invalid("Empty, duplicate or unknown WB RxMER selection"));
        }
    }
    let mut captures = Vec::new();
    let mut hashes = BTreeSet::new();
    for stored in &saved.rxmer {
        if !hashes.insert(&stored.sha256) {
            return Err(invalid("Duplicate WB RxMER original"));
        }
        let capture = designer::load(&stored.decode(decoded_bytes)?)?;
        if selection.is_none_or(|hashes| hashes.contains(&stored.sha256)) {
            captures.push(capture);
        }
    }
    if mode == MeasurementMode::Single {
        // Legacy single means the first selected original in stored inventory order.
        captures.truncate(1);
    }
    let sequence = MeasurementSequence::new(
        profile,
        transmission,
        captures,
        MeasurementSequenceOptions {
            mode,
            interval_symbols,
            tolerance: AssociationTolerance::new(saved.options.tolerance_seconds)?,
        },
    )?;
    if sequence.info().source_count > sequence.capacity(ObservationDuration::new(duration_ms)?)? {
        return Err(ApiError::new(
            ErrorCode::ResourceLimit,
            "WB playback selection exceeds the observation duration capacity",
        ));
    }
    Ok(sequence)
}
