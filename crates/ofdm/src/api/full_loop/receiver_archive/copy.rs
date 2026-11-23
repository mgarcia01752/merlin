/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Checked preflight before finite copies; reduction preserves original cells without synthesis.
use super::*;

fn selected<T>(values: &[T], limit: usize) -> impl ExactSizeIterator<Item = &T> {
    let count = values.len().min(limit);
    (0..count).map(move |index| {
        // Wider integer arithmetic preserves floor(k*N/count) on both supported usize widths.
        let original_index = (index as u128 * values.len() as u128 / count as u128) as usize;
        &values[original_index]
    })
}
fn add(bytes: &mut usize, count: usize, element_size: usize) -> ApiResult<()> {
    *bytes = count
        .checked_mul(element_size)
        .and_then(|extra| bytes.checked_add(extra))
        .ok_or_else(|| limit("Receiver archive byte accounting overflow"))?;
    Ok(())
}
fn bytes(
    data: Option<&ConstellationResult>,
    protected: Option<&ProtectedConstellation>,
    decoder: Option<&FullLoopStageSnapshot>,
    policy: &FullLoopReceiverArchiveInput,
) -> ApiResult<usize> {
    let mut bytes = size_of::<Retained>();
    if let Some(data) = data {
        for pair in selected(&data.pairs, policy.payload_cells) {
            add(&mut bytes, 1, size_of::<ConstellationPair>())?;
            add(&mut bytes, pair.llrs.len(), size_of::<f64>())?;
        }
        for pair in selected(&data.ncp_pairs, policy.ncp_cells) {
            add(&mut bytes, 1, size_of::<NcpConstellationPair>())?;
            add(&mut bytes, pair.llrs.len(), size_of::<f64>())?;
        }
    }
    if let Some(protected) = protected {
        for stream in [
            &protected.plc_data,
            &protected.plc_preamble,
            &protected.continuous_pilots,
            &protected.scattered_pilots,
        ] {
            add(
                &mut bytes,
                stream.pairs.len().min(policy.protected_cells_per_stream),
                size_of::<ProtectedPair>(),
            )?;
        }
    }
    if let Some(event) = decoder {
        add(
            &mut bytes,
            event.data.len().min(policy.decoder_llrs),
            size_of::<f64>(),
        )?;
        add(&mut bytes, event.context.run_id.len(), size_of::<u8>())?;
        add(
            &mut bytes,
            event.context.configuration_id.len(),
            size_of::<u8>(),
        )?;
        if let Some(roles) = &event.context.transmit_cell_roles {
            add(
                &mut bytes,
                roles.len(),
                size_of::<crate::simulation::test_points::TransmitCellRole>(),
            )?;
        }
    }
    if bytes > policy.max_retained_bytes {
        return Err(limit("Receiver archive exceeds requested byte budget"));
    }
    Ok(bytes)
}
fn stream(original: &ProtectedStream, cap: usize) -> ProtectedStream {
    ProtectedStream {
        modulation: original.modulation,
        status: original.status,
        collected_pairs: original.collected_pairs,
        pairs: selected(&original.pairs, cap).cloned().collect(),
    }
}

/// Copies only admitted immutable cells/event data after complete checked-byte preflight.
/// # Arguments
/// * `data`, `protected`, `decoder` - Actual optional observations; no whole-result ownership.
/// * `receiver_mode`, `policy` - Recorded correction and accepted finite copy policy.
/// # Returns
/// Detached shareable archive with actual accounted bytes and unchanged original identities.
/// # Errors
/// Rejects byte overflow/exhaustion before copying; preserves protected-selection failures.
pub(super) fn detach(
    data: Option<&ConstellationResult>,
    protected: Option<&ProtectedConstellation>,
    receiver_mode: ReceiverMode,
    decoder: Option<&FullLoopStageSnapshot>,
    policy: FullLoopReceiverArchivePolicy,
) -> ApiResult<FullLoopReceiverArchive> {
    let input = policy.settings();
    let decoder = decoder.filter(|_| input.decoder_llrs != 0);
    let retained_bytes = bytes(data, protected, decoder, input)?;
    let protected = protected
        .map(|original| -> ApiResult<ProtectedConstellation> {
            let mut copy =
                ProtectedConstellation::new(original.selection.clone(), original.receiver_mode)?;
            copy.schema_version = original.schema_version;
            copy.plc_data = stream(&original.plc_data, input.protected_cells_per_stream);
            copy.plc_preamble = stream(&original.plc_preamble, input.protected_cells_per_stream);
            copy.continuous_pilots = stream(
                &original.continuous_pilots,
                input.protected_cells_per_stream,
            );
            copy.scattered_pilots =
                stream(&original.scattered_pilots, input.protected_cells_per_stream);
            Ok(copy)
        })
        .transpose()?;
    let data = data.map(|original| ConstellationResult {
        schema_version: original.schema_version,
        selection: original.selection.clone(),
        status: original.status,
        collected_pairs: original.collected_pairs,
        reference: original.reference,
        channel_coefficients: original.channel_coefficients,
        pairs: selected(&original.pairs, input.payload_cells)
            .cloned()
            .collect(),
        ncp_pairs: selected(&original.ncp_pairs, input.ncp_cells)
            .cloned()
            .collect(),
    });
    let decoder = decoder
        .map(|event| -> ApiResult<FullLoopStageSnapshot> {
            let FullLoopStageData::Llrs(values) = &event.data else {
                return Err(super::super::super::error::invalid(
                    "Receiver archive decoder event is not LLR data",
                ));
            };
            Ok(FullLoopStageSnapshot {
                stage: event.stage,
                retained_index: event.retained_index,
                direction: event.direction,
                context: event.context.clone(),
                domain: event.domain,
                units: event.units,
                element_start: event.element_start,
                total_elements: event.total_elements,
                data: FullLoopStageData::Llrs(
                    values.iter().take(input.decoder_llrs).copied().collect(),
                ),
            })
        })
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
mod tests {
    use super::*;
    #[test]
    fn byte_preflight_rejects_addition_and_product_overflow() {
        let mut total = usize::MAX;
        assert_eq!(
            add(&mut total, 1, 1).err().unwrap().code(),
            ErrorCode::ResourceLimit
        );
        let mut total = 0;
        assert_eq!(
            add(&mut total, usize::MAX, 2).err().unwrap().code(),
            ErrorCode::ResourceLimit
        );
    }
}
