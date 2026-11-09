/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! TX composition: actual coding/packing/assembly/interleaving with explicit unavailable-history filler.
use crate::{
    error::invalid,
    signal_processing::{
        interleaving::{InterleaverDirection, SymbolInterleaver},
        modulation::{interleaving::BitInterleaver, randomization::CellRandomizer},
    },
    transmission::{
        layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole, PilotReference},
        ncp::NcpFieldCodec,
        packing::{CodewordId, NcpModulation, PackedSymbol, PackingSettings, SymbolPacker},
        symbols::{assemble_packed, AssembledSymbol, TransportCell},
        TransmissionSettings,
    },
    types::IqPair,
    LabError,
};
use fec_codec::concatenated::{ConcatenatedCodec, EncodedBlock};
/// Complete TX coding boundaries, retained for caller-owned probes/truth only.
pub struct EncodedPayload {
    pub codeword_id: CodewordId,
    pub encoded: EncodedBlock,
    pub labels: Vec<crate::signal_processing::modulation::SymbolLabel>,
}
/// One physical-time reduced grid. Missing branch history is explicitly accounted for.
pub struct PayloadChannelSymbol {
    pub channel_symbol_index: u64,
    pub cells: Vec<Option<TransportCell>>,
    pub reduced: Vec<Option<IqPair>>,
    /// Explicit BPSK startup/drain replacements; diagnostics never enter RX.
    pub unavailable_data_ranks: Vec<usize>,
}
/// Source input and channel output stages at one TX clock step.
pub struct PayloadTransmitSymbol {
    pub source_symbol_index: u64,
    pub packed: PackedSymbol,
    pub assembled: AssembledSymbol,
    pub channel: PayloadChannelSymbol,
}
/// Fixed256-QAM TX state; buffers remain bounded independently of simulation duration.
pub struct PayloadTransmitter {
    origin: u64,
    next_local: u64,
    drain_remaining: Option<usize>,
    layout: CarrierLayout,
    reference: PilotReference,
    codec: ConcatenatedCodec,
    bits: BitInterleaver,
    packer: SymbolPacker,
    control: NcpFieldCodec,
    randomizer: CellRandomizer,
    interleaver: SymbolInterleaver<TransportCell>,
}
impl PayloadTransmitter {
    /// Starts a synchronized payload session at an explicit PLC-frame boundary.
    /// # Arguments
    /// * `settings` - Frozen TX channel numerology/depth.
    /// * `layout` - Explicit validated TX carrier assignments.
    /// * `ncp` - Fixed positive NCP constellation; ProfileA uses256-QAM.
    /// * `origin` - Absolute symbol index for local0, aligned to128 symbols.
    /// # Returns
    /// Empty bounded payload queue and independent coding/control/interleaver history.
    /// # Errors
    /// Rejects geometry/origin/capacity or codec setup failures.
    pub fn new(
        settings: &TransmissionSettings,
        layout: &CarrierLayoutSettings,
        ncp: NcpModulation,
        origin: u64,
    ) -> Result<Self, LabError> {
        crate::signal_processing::receiver::payload::configuration::validate_origin(origin)?;
        let layout = CarrierLayout::new(settings, layout)?;
        let reference = PilotReference::new(&layout, usize::from(settings.time_interleaver_depth))?;
        let packing = PackingSettings {
            subcarrier_spacing: settings.subcarrier_spacing,
            interleaver_cells: reference.settings().cells(),
            ncp_modulation: ncp,
            data_profile_id: crate::transmission::symbols::PAYLOAD_PROFILE_ID,
        };
        let interleaver =
            SymbolInterleaver::new(reference.settings(), InterleaverDirection::Transmit)?;
        Ok(Self {
            origin,
            next_local: 0,
            drain_remaining: None,
            layout,
            reference,
            codec: ConcatenatedCodec::new().map_err(invalid)?,
            bits: BitInterleaver::new(crate::transmission::symbols::PAYLOAD_ORDER)?,
            packer: SymbolPacker::new(packing)?,
            control: NcpFieldCodec::new()?,
            randomizer: CellRandomizer::new(),
            interleaver,
        })
    }
    /// Encodes and queues one actual payload; no truth is passed to RX.
    /// # Arguments
    /// * `payload` - Exactly14232 binary source bits, retained unchanged by caller.
    /// # Returns
    /// Diagnostic queue identity plus BCH/LDPC/label stage outputs for direct inspection.
    /// # Errors
    /// Rejects invalid payload, full queue or use after drain began; existing queue is preserved.
    pub fn enqueue(&mut self, payload: &[u8]) -> Result<EncodedPayload, LabError> {
        if self.drain_remaining.is_some() {
            return Err(invalid("Cannot enqueue after payload drain begins."));
        }
        let encoded = self.codec.encode(payload).map_err(invalid)?;
        let labels = self.bits.encode_labels(&encoded.ldpc_codeword)?;
        let codeword_id = self.packer.enqueue(labels.clone())?;
        Ok(EncodedPayload {
            codeword_id,
            encoded,
            labels,
        })
    }
    /// Reports queued complete/partial words still awaiting transmission.
    /// # Returns
    /// At most16; an empty queue does not mean receiver interleaver drain is finished.
    pub fn queued_codewords(&self) -> usize {
        self.packer.queued_codewords()
    }
    /// Advances one real source symbol, including real NCP and zero-loaded filler when idle.
    /// # Returns
    /// Distinct coding-plan, randomized/mapped and interleaved channel outputs.
    /// # Errors
    /// Propagates internal stage failures; after a post-packing failure start a new session.
    /// No input truth, codeword ID or NCP reservation marker enters the channel cells.
    pub fn transmit(&mut self) -> Result<PayloadTransmitSymbol, LabError> {
        if self.drain_remaining.is_some() {
            return Err(invalid("Payload source is closed for drain."));
        }
        let next = self
            .next_local
            .checked_add(1)
            .ok_or_else(|| invalid("Payload TX symbol overflow."))?;
        self.origin
            .checked_add(next)
            .ok_or_else(|| invalid("Absolute payload TX symbol overflow."))?;
        let packed = self
            .packer
            .pack_symbol(self.reference.roles(self.next_local))?;
        let assembled = assemble_packed(&packed, &self.control, &mut self.randomizer)?;
        let cells = self
            .interleaver
            .process_symbol(self.next_local, &assembled.cells)?;
        let channel = self.materialize(cells)?;
        self.next_local = next;
        Ok(PayloadTransmitSymbol {
            source_symbol_index: channel.channel_symbol_index,
            packed,
            assembled,
            channel,
        })
    }
    /// Drains matched cell-interleaver history after all queued words have been placed.
    /// # Returns
    /// One channel symbol per call, then None after exactly M-1 drain steps.
    /// Source randomizers/packer are not clocked with fabricated drain payload.
    /// # Errors
    /// Rejects pending queue or index overflow; never discards queued source words.
    pub fn drain(&mut self) -> Result<Option<PayloadChannelSymbol>, LabError> {
        if self.packer.queued_codewords() != 0 {
            return Err(invalid("Place all queued payload cells before drain."));
        }
        let remaining = self.drain_remaining.unwrap_or_else(|| {
            if self.next_local == 0 {
                0
            } else {
                self.reference.settings().pair_delay_symbols()
            }
        });
        if remaining == 0 {
            self.drain_remaining = Some(0);
            return Ok(None);
        }
        let next = self
            .next_local
            .checked_add(1)
            .ok_or_else(|| invalid("Payload drain symbol overflow."))?;
        self.origin
            .checked_add(next)
            .ok_or_else(|| invalid("Absolute drain symbol overflow."))?;
        let cells = self.interleaver.process_symbol(
            self.next_local,
            &vec![None; self.reference.settings().cells()],
        )?;
        let output = self.materialize(cells)?;
        self.next_local = next;
        self.drain_remaining = Some(remaining - 1);
        Ok(Some(output))
    }
    fn materialize(
        &self,
        cells: Vec<Option<TransportCell>>,
    ) -> Result<PayloadChannelSymbol, LabError> {
        let channel_symbol_index = self.origin + self.next_local;
        let roles = self.layout.symbol_roles(channel_symbol_index);
        let mut reduced = Vec::with_capacity(cells.len());
        let mut unavailable_data_ranks = Vec::new();
        for (rank, (cell, bin)) in cells.iter().zip(self.reference.physical_bins()).enumerate() {
            match (cell, roles[usize::from(*bin)]) {
                (
                    Some(TransportCell::PilotPlaceholder) | None,
                    PhysicalCarrierRole::ScatteredPilot,
                ) => reduced.push(None),
                (Some(TransportCell::Sample(point)), PhysicalCarrierRole::Data) => {
                    reduced.push(Some(*point))
                }
                (None, PhysicalCarrierRole::Data) => {
                    unavailable_data_ranks.push(rank);
                    reduced.push(Some([1.0, 0.0]));
                }
                _ => {
                    return Err(invalid(
                        "Interleaved payload cell conflicts with physical pilot role.",
                    ))
                }
            }
        }
        Ok(PayloadChannelSymbol {
            channel_symbol_index,
            cells,
            reduced,
            unavailable_data_ranks,
        })
    }
}
