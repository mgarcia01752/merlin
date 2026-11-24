/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in detached target admission; final capacities are separate from construction scratch.
use super::*;
use crate::{
    signal_processing::waveform::WaveformGeometry,
    simulation::channel::{
        constellation::ConstellationSelection, protected_constellation::ProtectedCaptureSelection,
    },
};

impl FullLoopResult {
    /// Computes the complete retained bound for an independent receiver archive before copying.
    ///
    /// # Arguments
    /// * `policy` - Existing validated role counts, decoder prefix and logical-byte ceiling.
    ///
    /// # Returns
    /// Target-layout payload bytes including this archive handle, its fixed Retained value,
    /// exact final buffer lengths/capacities, zero ordered entries and one shared root.
    /// The same original cells/event are selected as receiver_archive; the private live
    /// pilot cache is not copied. No payload clone, source mutation or science occurs.
    /// This excludes temporary builders, allocator bookkeeping and independent query views.
    /// Callers keep source/old-owner charges live while separately reserving this target.
    ///
    /// # Errors
    /// Preserves original logical-byte/selection/decoder diagnostics and returns ResourceLimit
    /// for retained-bound overflow. This operation makes no process-memory or allocation claim.
    pub fn receiver_archive_retained_bound(
        &self,
        policy: &FullLoopReceiverArchivePolicy,
    ) -> ApiResult<merlin_api::RetainedFootprint> {
        let decoder = selected_decoder(self, policy)?;
        preflight(
            self.job.constellation.as_ref(),
            self.job.protected_constellation.as_ref(),
            decoder,
            policy,
        )
        .map(|(bound, _)| bound)
    }

    /// Copies the historical receiver selection only after complete retained-target admission.
    ///
    /// # Arguments
    /// * `policy` - Existing validated role counts, decoder prefix and logical-byte ceiling.
    /// * `limit` - Caller-reserved maximum target payload bytes, ordered entries and shared roots.
    ///
    /// # Returns
    /// Detached archive with exact final capacities and the original retained_bytes statistic,
    /// statuses, coordinates, contexts and query semantics. Every dimension is checked before
    /// copying; passing metadata does not itself reserve application resources. The live source
    /// and old archives remain independently owned/charged throughout construction. Temporary
    /// builders follow existing finite role/event/byte/input limits and are outside this bound;
    /// no standard-library growth multiplier or total preparation-peak bound is assumed.
    ///
    /// # Errors
    /// Preserves original logical-byte/selection/decoder failures. ResourceLimit reports
    /// checked bound overflow or any insufficient target dimension before payload copying.
    /// Standard allocation failure behavior is unchanged; no fallible-allocator promise is added.
    pub fn receiver_archive_bounded(
        &self,
        policy: FullLoopReceiverArchivePolicy,
        limit: merlin_api::RetainedFootprint,
    ) -> ApiResult<FullLoopReceiverArchive> {
        let decoder = selected_decoder(self, &policy)?;
        detach(
            self.job.constellation.as_ref(),
            self.job.protected_constellation.as_ref(),
            self.job.settings.receiver_mode,
            decoder,
            policy,
            limit,
        )
    }
}

fn selected_decoder<'result>(
    result: &'result FullLoopResult,
    policy: &FullLoopReceiverArchivePolicy,
) -> ApiResult<Option<&'result FullLoopStageSnapshot>> {
    if policy.settings().decoder_llrs == 0 {
        return Ok(None);
    }
    match result.stages.events(FullLoopStage::PayloadLdpcInput) {
        Ok(events) => Ok(first_nonempty_decoder(events)),
        Err(cause) if cause.code() == ErrorCode::MissingCoverage => Ok(None),
        Err(cause) => Err(cause),
    }
}

fn first_nonempty_decoder(events: &[FullLoopStageSnapshot]) -> Option<&FullLoopStageSnapshot> {
    events
        .iter()
        .find(|event| matches!(&event.data, FullLoopStageData::Llrs(values) if !values.is_empty()))
}

fn bound_from_legacy(retained_bytes: usize) -> ApiResult<RetainedFootprint> {
    let payload_bytes = retained_bytes
        .checked_add(size_of::<FullLoopReceiverArchive>())
        .ok_or_else(|| super::limit("Receiver archive retained_footprint overflow"))?;
    Ok(RetainedFootprint {
        payload_bytes,
        ordered_entries: 0,
        shared_roots: 1, // The new archive owns exactly one detached Retained Arc.
    })
}

fn preflight(
    data: Option<&ConstellationResult>,
    protected: Option<&ProtectedConstellation>,
    decoder: Option<&FullLoopStageSnapshot>,
    policy: &FullLoopReceiverArchivePolicy,
) -> ApiResult<(RetainedFootprint, usize)> {
    let decoder = decoder.filter(|_| policy.settings().decoder_llrs != 0);
    let retained_bytes = copy::bytes(data, protected, decoder, policy.settings())?;

    // Keep legacy error order: byte admission, protected selection, then decoder representation.
    if let Some(protected) = protected {
        protected.selection.validate()?;
    }
    if decoder.is_some_and(|event| !matches!(&event.data, FullLoopStageData::Llrs(_))) {
        return Err(super::super::super::error::invalid(
            "Receiver archive decoder event is not LLR data",
        ));
    }
    Ok((bound_from_legacy(retained_bytes)?, retained_bytes))
}

fn admit(required: RetainedFootprint, limit: RetainedFootprint) -> ApiResult<()> {
    if required.payload_bytes > limit.payload_bytes {
        return Err(super::limit(
            "Receiver archive exceeds target payload-byte limit",
        ));
    }
    if required.ordered_entries > limit.ordered_entries {
        return Err(super::limit(
            "Receiver archive exceeds target ordered-entry limit",
        ));
    }
    if required.shared_roots > limit.shared_roots {
        return Err(super::limit(
            "Receiver archive exceeds target shared-root limit",
        ));
    }
    Ok(())
}

fn exact_vec<Value>(values: impl IntoIterator<Item = Value>) -> Vec<Value> {
    // Box-to-Vec fixes retained capacity to length for these nonzero-sized concrete payloads.
    // Collection/shrinking scratch remains separate from the retained target contract.
    Vec::from(values.into_iter().collect::<Box<[_]>>())
}

fn exact_string(value: &str) -> String {
    String::from(Box::<str>::from(value))
}

fn payload_selection(original: &ConstellationSelection) -> ConstellationSelection {
    let ConstellationSelection {
        requested_pairs,
        profile_id,
        start_hz,
        stop_hz,
        capture_ncp,
    } = original;
    ConstellationSelection {
        requested_pairs: *requested_pairs,
        profile_id: *profile_id,
        start_hz: *start_hz,
        stop_hz: *stop_hz,
        capture_ncp: *capture_ncp,
    }
}

fn protected_selection(original: &ProtectedCaptureSelection) -> ProtectedCaptureSelection {
    let ProtectedCaptureSelection {
        pairs_per_stream,
        first_channel_symbol,
        end_channel_symbol_exclusive,
    } = original;
    ProtectedCaptureSelection {
        pairs_per_stream: *pairs_per_stream,
        first_channel_symbol: *first_channel_symbol,
        end_channel_symbol_exclusive: *end_channel_symbol_exclusive,
    }
}

fn payload_pair(original: &ConstellationPair) -> ConstellationPair {
    let ConstellationPair {
        channel_symbol_index,
        source_symbol_index,
        physical_bin,
        frequency_hz,
        data_position,
        profile_id,
        modulation_order,
        before_equalization,
        after_equalization,
        reference_iq,
        llrs,
    } = original;
    ConstellationPair {
        channel_symbol_index: *channel_symbol_index,
        source_symbol_index: *source_symbol_index,
        physical_bin: *physical_bin,
        frequency_hz: *frequency_hz,
        data_position: *data_position,
        profile_id: *profile_id,
        modulation_order: *modulation_order,
        before_equalization: *before_equalization,
        after_equalization: *after_equalization,
        reference_iq: *reference_iq,
        llrs: exact_vec(llrs.iter().copied()),
    }
}

fn ncp_pair(original: &NcpConstellationPair) -> NcpConstellationPair {
    let NcpConstellationPair {
        channel_symbol_index,
        source_symbol_index,
        physical_bin,
        frequency_hz,
        data_position,
        modulation_order,
        before_equalization,
        after_equalization,
        reference_iq,
        llrs,
    } = original;
    NcpConstellationPair {
        channel_symbol_index: *channel_symbol_index,
        source_symbol_index: *source_symbol_index,
        physical_bin: *physical_bin,
        frequency_hz: *frequency_hz,
        data_position: *data_position,
        modulation_order: *modulation_order,
        before_equalization: *before_equalization,
        after_equalization: *after_equalization,
        reference_iq: *reference_iq,
        llrs: exact_vec(llrs.iter().copied()),
    }
}

fn payload(
    original: &ConstellationResult,
    policy: &FullLoopReceiverArchiveInput,
) -> ConstellationResult {
    let ConstellationResult {
        schema_version,
        selection,
        status,
        collected_pairs,
        reference,
        channel_coefficients,
        pairs,
        ncp_pairs,
    } = original;
    ConstellationResult {
        schema_version: *schema_version,
        selection: payload_selection(selection),
        status: *status,
        collected_pairs: *collected_pairs,
        reference,
        channel_coefficients,
        pairs: exact_vec(copy::selected(pairs, policy.payload_cells).map(payload_pair)),
        ncp_pairs: exact_vec(copy::selected(ncp_pairs, policy.ncp_cells).map(ncp_pair)),
    }
}

fn protected_pair(original: &ProtectedPair) -> ProtectedPair {
    let ProtectedPair {
        physical_bin,
        frequency_hz,
        channel_symbol_index,
        plc_phase,
        before_equalization,
        after_equalization,
        reference_iq,
        receiver_estimate_ready,
        used_in_current_fit,
        prior_channel_prediction,
        pilot_reference,
    } = original;
    ProtectedPair {
        physical_bin: *physical_bin,
        frequency_hz: *frequency_hz,
        channel_symbol_index: *channel_symbol_index,
        plc_phase: *plc_phase,
        before_equalization: *before_equalization,
        after_equalization: *after_equalization,
        reference_iq: *reference_iq,
        receiver_estimate_ready: *receiver_estimate_ready,
        used_in_current_fit: *used_in_current_fit,
        prior_channel_prediction: *prior_channel_prediction,
        pilot_reference: *pilot_reference,
    }
}

fn protected_stream(original: &ProtectedStream, cap: usize) -> ProtectedStream {
    let ProtectedStream {
        modulation,
        status,
        collected_pairs,
        pairs,
    } = original;
    ProtectedStream {
        modulation,
        status: *status,
        collected_pairs: *collected_pairs,
        pairs: exact_vec(copy::selected(pairs, cap).map(protected_pair)),
    }
}

fn protected(original: &ProtectedConstellation, cap: usize) -> ApiResult<ProtectedConstellation> {
    // The private outer cache cannot be destructured here. Reuse the legacy constructor:
    // pilot_sequence stays None, then copy every current public field/stream explicitly.
    let mut result = ProtectedConstellation::new(
        protected_selection(&original.selection),
        original.receiver_mode,
    )?;
    result.schema_version = original.schema_version;
    result.plc_data = protected_stream(&original.plc_data, cap);
    result.plc_preamble = protected_stream(&original.plc_preamble, cap);
    result.continuous_pilots = protected_stream(&original.continuous_pilots, cap);
    result.scattered_pilots = protected_stream(&original.scattered_pilots, cap);
    Ok(result)
}

fn waveform_grid(original: &FullLoopWaveformGrid) -> FullLoopWaveformGrid {
    let FullLoopWaveformGrid {
        geometry,
        sample_rate_hz,
    } = original;
    FullLoopWaveformGrid {
        // This existing geometry owns only fixed TransmissionSettings, with no heap children.
        geometry: WaveformGeometry::clone(geometry),
        sample_rate_hz: *sample_rate_hz,
    }
}

fn context(original: &FullLoopStageContext) -> FullLoopStageContext {
    let FullLoopStageContext {
        run_id,
        configuration_id,
        phase,
        source_symbol_index,
        channel_symbol_index,
        sample_start_index,
        codeword_index,
        profile_id,
        plc_phase,
        reference,
        plc_grid,
        physical_grid,
        waveform_grid: waveform,
        cell_grid,
        transmit_cell_roles,
    } = original;
    FullLoopStageContext {
        run_id: exact_string(run_id),
        configuration_id: exact_string(configuration_id),
        phase: *phase,
        source_symbol_index: *source_symbol_index,
        channel_symbol_index: *channel_symbol_index,
        sample_start_index: *sample_start_index,
        codeword_index: *codeword_index,
        profile_id: *profile_id,
        plc_phase: *plc_phase,
        reference: *reference,
        plc_grid: *plc_grid,
        physical_grid: *physical_grid,
        waveform_grid: waveform.as_ref().map(waveform_grid),
        cell_grid: *cell_grid,
        transmit_cell_roles: transmit_cell_roles
            .as_ref()
            .map(|roles| exact_vec(roles.iter().copied())),
    }
}

fn decoder(original: &FullLoopStageSnapshot, cap: usize) -> ApiResult<FullLoopStageSnapshot> {
    let FullLoopStageSnapshot {
        stage,
        retained_index,
        direction,
        context: original_context,
        domain,
        units,
        element_start,
        total_elements,
        data,
    } = original;
    let FullLoopStageData::Llrs(values) = data else {
        return Err(super::super::super::error::invalid(
            "Receiver archive decoder event is not LLR data",
        ));
    };
    Ok(FullLoopStageSnapshot {
        stage: *stage,
        retained_index: *retained_index,
        direction: *direction,
        context: context(original_context),
        domain: *domain,
        units: *units,
        element_start: *element_start,
        total_elements: *total_elements,
        data: FullLoopStageData::Llrs(exact_vec(values.iter().take(cap).copied())),
    })
}

fn detach(
    data: Option<&ConstellationResult>,
    original_protected: Option<&ProtectedConstellation>,
    receiver_mode: ReceiverMode,
    original_decoder: Option<&FullLoopStageSnapshot>,
    policy: FullLoopReceiverArchivePolicy,
    limit: RetainedFootprint,
) -> ApiResult<FullLoopReceiverArchive> {
    let original_decoder = original_decoder.filter(|_| policy.settings().decoder_llrs != 0);
    let (required, retained_bytes) =
        preflight(data, original_protected, original_decoder, &policy)?;
    admit(required, limit)?;

    // No payload construction occurs until every target dimension has been admitted.
    let input = policy.settings();
    let protected = original_protected
        .map(|original| protected(original, input.protected_cells_per_stream))
        .transpose()?;
    let data = data.map(|original| payload(original, input));
    let decoder = original_decoder
        .map(|original| decoder(original, input.decoder_llrs))
        .transpose()?;
    Ok(FullLoopReceiverArchive(Arc::new(Retained {
        policy,
        receiver_mode,
        data,
        protected,
        decoder,
        retained_bytes,
    })))
}

#[cfg(test)]
mod tests;
