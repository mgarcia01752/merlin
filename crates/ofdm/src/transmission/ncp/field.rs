/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded NCP-chain encoding, L-delimited receive discovery and whole-field CRC.
use super::{
    mapping::{labels_from_bits, llrs_from_cells},
    message::{validate_chain, MAX_MESSAGES},
    NcpMessage, NULL_POINTER,
};
use crate::{
    error::invalid,
    signal_processing::modulation::SymbolLabel,
    transmission::{
        packing::{NcpModulation, NcpPlanEntry, PackedCell, PackedSymbol},
        SubcarrierSpacing,
    },
    types::ProfileId,
    SimulationError,
};
use fec_codec::{crc::crc24d, ncp::NcpFecCodec};
use serde::Serialize;

/// Encoded field cells, in high-to-low field order before shared randomization.
#[derive(Debug, Serialize)]
pub struct EncodedNcpField {
    pub messages: Vec<NcpMessage>,
    pub crc: [u8; 3],
    pub labels: Vec<SymbolLabel>,
}
/// CRC-accepted field; diagnostics do not contain transmitter codeword identities.
#[derive(Debug, Serialize)]
pub struct DecodedNcpField {
    pub messages: Vec<NcpMessage>,
    pub crc: [u8; 3],
    pub consumed_cells: usize,
    /// Includes the CRC's own LDPC block as the last entry.
    pub block_iterations: Vec<usize>,
}
/// Data-position interval derived from decoded pointers, excluding NCP and pilots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NcpDataSpan {
    pub start: usize,
    pub end_exclusive: usize,
    /// None for a continuation prefix or ignored zero-loaded profile.
    pub profile_id: Option<ProfileId>,
    pub starts_codeword: bool,
    pub zero_loaded: bool,
}
impl DecodedNcpField {
    /// Resolves received pointers into intervals within a known data segment.
    /// # Arguments
    /// * `data_positions` - Remaining data positions after pilot and received NCP
    ///   cells are removed, including any zero-loaded filler.
    /// # Returns
    /// Ordered nonempty spans; a prefix before the first pointer is continuation.
    /// This does not infer codeword length, prior profile state or payload IDs.
    /// # Errors
    /// Rejects inconsistent chains, oversized domains or pointers outside data.
    pub fn data_spans(&self, data_positions: usize) -> Result<Vec<NcpDataSpan>, SimulationError> {
        validate_chain(&self.messages)?;
        if data_positions > SubcarrierSpacing::Khz25.max_interleaver_cells() {
            return Err(invalid(
                "Data segment exceeds the supported interleaver domain.",
            ));
        }
        let mut spans = Vec::new();
        for (index, message) in self.messages.iter().enumerate() {
            if message.pointer == NULL_POINTER {
                continue;
            }
            let start = usize::from(message.pointer);
            if start >= data_positions {
                return Err(invalid("NCP pointer lies outside the data segment."));
            }
            if index == 0 && start > 0 {
                spans.push(NcpDataSpan {
                    start: 0,
                    end_exclusive: start,
                    profile_id: None,
                    starts_codeword: false,
                    zero_loaded: false,
                });
            }
            let end_exclusive = self
                .messages
                .get(index + 1)
                .filter(|next| next.pointer != NULL_POINTER)
                .map_or(data_positions, |next| usize::from(next.pointer));
            if end_exclusive > data_positions {
                return Err(invalid("NCP pointer lies outside the data segment."));
            }
            spans.push(NcpDataSpan {
                start,
                end_exclusive,
                profile_id: if message.zero_loaded {
                    None
                } else {
                    Some(message.profile_id)
                },
                starts_codeword: !message.zero_loaded,
                zero_loaded: message.zero_loaded,
            });
        }
        if spans.is_empty() && data_positions > 0 {
            spans.push(NcpDataSpan {
                start: 0,
                end_exclusive: data_positions,
                profile_id: None,
                starts_codeword: false,
                zero_loaded: false,
            });
        }
        Ok(spans)
    }
}

/// Job-owned NCP FEC setup; no dynamic CMTS or PLC profile-state machine.
pub struct NcpFieldCodec {
    fec: NcpFecCodec,
}
impl NcpFieldCodec {
    /// Builds independent control-code encoder/decoder state for a job.
    /// # Returns
    /// Reusable codec whose decoder scratch state is owned by this instance.
    /// # Errors
    /// Reports invalid mother-code setup with a stable scientific error category.
    pub fn new() -> Result<Self, SimulationError> {
        Ok(Self {
            fec: NcpFecCodec::new()
                .map_err(|message| SimulationError::new("NcpCodecSetup", message))?,
        })
    }
    /// Encodes a bounded validated chain and its separate CRC block.
    /// # Arguments
    /// * `messages` - One through11 messages; L only on last, terminal NULL/Z last.
    ///   N/U must agree across messages. Caller owns temporal profile semantics.
    /// * `modulation` - Uniform QPSK,16QAM or64QAM NCP profile.
    /// # Returns
    /// MSB-first message/CRC bytes and encoded labels before randomization.
    /// # Errors
    /// Rejects invalid chains; no partial output is returned.
    pub fn encode_messages(
        &self,
        messages: &[NcpMessage],
        modulation: NcpModulation,
    ) -> Result<EncodedNcpField, SimulationError> {
        validate_chain(messages)?;
        let mut bytes = Vec::new();
        let mut labels = Vec::new();
        for message in messages {
            let payload = message.to_bytes()?;
            bytes.extend(payload);
            labels.extend(labels_from_bits(&self.fec.encode(&payload), modulation)?);
        }
        let crc = crc24d(&bytes);
        labels.extend(labels_from_bits(&self.fec.encode(&crc), modulation)?);
        Ok(EncodedNcpField {
            messages: messages.to_vec(),
            crc,
            labels,
        })
    }
    /// Encodes an existing packer plan with fixed even profiles and no OPT tagging.
    /// # Arguments
    /// * `symbol` - Plan containing NCP meanings and indexed reserved positions.
    /// # Returns
    /// Real encoded NCP labels matching field_cell indices; diagnostic codeword
    /// IDs are deliberately discarded and do not become transmitted fields.
    /// # Errors
    /// Rejects invalid meanings or mismatched/duplicate reserved positions.
    pub fn encode_packed(&self, symbol: &PackedSymbol) -> Result<EncodedNcpField, SimulationError> {
        if symbol.ncp_entries.is_empty()
            || symbol.ncp_entries.len() > MAX_MESSAGES
            || symbol.cells.len() > SubcarrierSpacing::Khz25.max_interleaver_cells()
        {
            return Err(invalid(
                "Packed NCP plan exceeds bounded message/domain limits.",
            ));
        }
        let messages: Vec<_> = symbol
            .ncp_entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let (profile_id, zero_loaded, pointer) = match entry {
                    NcpPlanEntry::CodewordStart {
                        profile_id,
                        pointer,
                        ..
                    } => (*profile_id, false, *pointer),
                    NcpPlanEntry::ZeroLoaded { pointer } => (0, true, *pointer),
                    NcpPlanEntry::Null => (0, false, NULL_POINTER),
                };
                NcpMessage {
                    profile_id,
                    zero_loaded,
                    pointer,
                    last: index + 1 == symbol.ncp_entries.len(),
                    data_profile_odd: false,
                    ncp_profile_odd: false,
                    tagged: false,
                    ncp_profile_update: false,
                }
            })
            .collect();
        let field = self.encode_messages(&messages, symbol.ncp_modulation)?;
        if field.labels.len() != symbol.ncp_reserved_cells {
            return Err(invalid(
                "NCP encoding does not match the planned reservation.",
            ));
        }
        let mut expected = 0;
        let mut data_positions = 0;
        for cell in symbol.cells.iter().rev() {
            if matches!(cell, PackedCell::ScatteredPilotPlaceholder) {
                continue;
            }
            if expected < field.labels.len() {
                if !matches!(cell, PackedCell::NcpReserved {field_cell} if *field_cell == expected)
                {
                    return Err(invalid(
                        "NCP cells must occupy the highest eligible positions in field order.",
                    ));
                }
                expected += 1;
            } else if matches!(cell, PackedCell::NcpReserved { .. }) {
                return Err(invalid("Unexpected extra NCP reserved cell."));
            } else {
                data_positions += 1;
            }
        }
        if expected != field.labels.len() {
            return Err(invalid("NCP reserved cell count is inconsistent."));
        }
        DecodedNcpField {
            messages: field.messages.clone(),
            crc: field.crc,
            consumed_cells: expected,
            block_iterations: Vec::new(),
        }
        .data_spans(data_positions)?;
        Ok(field)
    }
    /// Discovers a field from high-to-low, descrambled NCP-hypothesis cell LLRs.
    /// # Arguments
    /// * `cells` - Available eligible cells, beginning at the highest active data
    ///   position. May include trailing data hypotheses after the field; at most7537.
    /// * `modulation` - Known uniform NCP profile supplied independently by PLC/setup.
    /// * `max_iterations` - Per-block decoder limit,1..100.
    /// # Returns
    /// Complete CRC-accepted messages, consumed cells and actual block iterations.
    /// Reads until L then one CRC block; never uses transmitter boundaries or IDs.
    /// # Errors
    /// Rejects truncation, invalid shapes, FEC failure, bad chain or CRC mismatch.
    /// No partial messages are returned on any failure.
    pub fn decode_field(
        &mut self,
        cells: &[Vec<f64>],
        modulation: NcpModulation,
        max_iterations: usize,
    ) -> Result<DecodedNcpField, SimulationError> {
        if cells.len() > SubcarrierSpacing::Khz25.max_interleaver_cells()
            || !(1..=100).contains(&max_iterations)
        {
            return Err(invalid("Invalid NCP receive domain or iteration limit."));
        }
        let block_cells = modulation.cells_per_block();
        let mut messages = Vec::new();
        let mut bytes = Vec::new();
        let mut block_iterations = Vec::new();
        let mut consumed = 0;
        for _ in 0..=MAX_MESSAGES {
            let block = cells
                .get(consumed..consumed + block_cells)
                .ok_or_else(|| SimulationError::new("NcpTruncated", "Missing NCP or CRC block."))?;
            let decoded = self
                .fec
                .decode(&llrs_from_cells(block, modulation)?, max_iterations)
                .map_err(|message| SimulationError::new("NcpDecodeInput", message))?;
            if !decoded.success {
                return Err(SimulationError::new(
                    "NcpFecFailure",
                    "NCP block did not converge to a valid shortened codeword.",
                ));
            }
            consumed += block_cells;
            block_iterations.push(decoded.iterations);
            if messages
                .last()
                .is_some_and(|message: &NcpMessage| message.last)
            {
                if decoded.payload != crc24d(&bytes) {
                    return Err(SimulationError::new(
                        "NcpCrcMismatch",
                        "Reject entire NCP field: CRC-24-D mismatch.",
                    ));
                }
                validate_chain(&messages)?;
                return Ok(DecodedNcpField {
                    messages,
                    crc: decoded.payload,
                    consumed_cells: consumed,
                    block_iterations,
                });
            }
            let message = NcpMessage::from_bytes(&decoded.payload)?;
            bytes.extend(decoded.payload);
            messages.push(message);
        }
        Err(invalid(
            "NCP field exceeds11 data messages or has no last marker.",
        ))
    }
}
