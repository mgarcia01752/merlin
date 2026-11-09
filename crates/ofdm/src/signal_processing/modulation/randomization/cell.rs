/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Job-owned data/NCP stream; caller supplies ordered pre-interleaving cells.
use super::lfsr::{Lfsr, RandomizerState};
use crate::{
    error::invalid,
    signal_processing::modulation::qam::SymbolLabel,
    types::{
        ofdm_modulation::{ConstellationOrder, OFDM_DATA_MODULATIONS},
        PLC_FRAME_SYMBOLS,
    },
    SimulationError,
};

/// Persistent data/NCP randomizer or receiver inverse, with private sequence state.
///
/// Call begin_symbol once per consecutive pre-interleaving symbol; phase0 is the
/// first symbol after PLC preamble. No codeword boundary resets occur. Inputs
/// remain unchanged; invalid calls leave all state unchanged. Each independent
/// transmitter/receiver stream owns its own instance. Cloning copies its current
/// checkpoint for atomic candidate processing; it does not resynchronize a stream. PLC uses a separate design.
#[derive(Clone)]
pub struct CellRandomizer {
    lfsr: Lfsr,
    symbol_index: Option<u64>,
}
impl Default for CellRandomizer {
    /// Creates an unsynchronized stream; begin_symbol(0) is required before use.
    fn default() -> Self {
        Self::new()
    }
}
impl CellRandomizer {
    /// Creates seeded data/NCP state, awaiting explicit symbol synchronization.
    /// # Returns
    /// Independent stream with D0=0x555,D1=0xAAA and no active symbol.
    pub fn new() -> Self {
        Self {
            lfsr: Lfsr::new(),
            symbol_index: None,
        }
    }

    /// Starts the next OFDM symbol and resets registers at128-symbol boundaries.
    /// # Arguments
    /// * `index` - Consecutive pre-interleaving symbol index, beginning at0 just
    ///   after the PLC preamble; same reference as the scattered-pilot cycle.
    /// # Returns
    /// Unit with an active symbol, even if its first cell is a pilot placeholder.
    /// # Errors
    /// Rejects repeated, skipped or out-of-order symbols before changing state.
    pub fn begin_symbol(&mut self, index: u64) -> Result<(), SimulationError> {
        let expected = match self.symbol_index {
            None => Some(0),
            Some(previous) => previous.checked_add(1),
        };
        if expected != Some(index) {
            return Err(SimulationError::new(
                "InvalidRandomizerSequence",
                "Begin consecutive symbols starting at the post-preamble phase0.",
            ));
        }
        if index % PLC_FRAME_SYMBOLS == 0 {
            self.lfsr = Lfsr::new();
        }
        self.symbol_index = Some(index);
        Ok(())
    }

    /// Copies current registers for diagnostics without exposing mutable state.
    /// # Returns
    /// Snapshot before the next cell operation; not a checkpoint restore API.
    pub fn state(&self) -> RandomizerState {
        self.lfsr.state()
    }

    /// XORs one data cell with its mask and then advances one clock.
    /// # Arguments
    /// * `order` - Supported downstream data order16..16384, including non-square.
    /// * `label` - MSB-first binary cell word, strictly below order.
    /// # Returns
    /// Randomized label; applying the same operation on a synchronized receiver
    /// restores the original hard label.
    /// # Errors
    /// Rejects inactive streams, unsupported orders or invalid labels atomically.
    pub fn apply_data(
        &mut self,
        order: ConstellationOrder,
        label: SymbolLabel,
    ) -> Result<SymbolLabel, SimulationError> {
        let bits = data_bits(order)?;
        self.apply_label(order, label, bits)
    }

    /// XORs one NCP cell and advances the shared data/NCP stream once.
    /// # Arguments
    /// * `order` - NCP constellation size4,16 or64.
    /// * `label` - MSB-first binary cell word below order.
    /// # Returns
    /// Randomized or inverse-randomized label; no NCP message is created here.
    /// # Errors
    /// Rejects inactive streams, invalid NCP order or out-of-range labels.
    pub fn apply_ncp(
        &mut self,
        order: ConstellationOrder,
        label: SymbolLabel,
    ) -> Result<SymbolLabel, SimulationError> {
        let bits = ncp_bits(order)?;
        self.apply_label(order, label, bits)
    }

    /// Reverses data-cell randomization on soft bits before inverse bit processing.
    /// # Arguments
    /// * `order` - Supported downstream data constellation.
    /// * `llrs` - Finite MSB-first ln(P0/P1) values in randomized-label order.
    /// # Returns
    /// Owned LLRs with signs flipped wherever the randomizer mask bit is1;
    /// magnitudes and input are preserved, then the stream advances once.
    /// # Errors
    /// Rejects inactive streams, unsupported order, wrong length or nonfinite LLRs.
    pub fn descramble_data_llrs(
        &mut self,
        order: ConstellationOrder,
        llrs: &[f64],
    ) -> Result<Vec<f64>, SimulationError> {
        self.apply_llrs(data_bits(order)?, llrs)
    }

    /// Reverses NCP-cell randomization on soft bits using the shared sequence.
    /// # Arguments
    /// * `order` - NCP order4,16 or64.
    /// * `llrs` - Finite MSB-first ln(P0/P1) values for this cell.
    /// # Returns
    /// Owned inverse-randomized LLRs and one stream clock.
    /// # Errors
    /// Rejects inactive streams, invalid order, wrong length or nonfinite LLRs.
    pub fn descramble_ncp_llrs(
        &mut self,
        order: ConstellationOrder,
        llrs: &[f64],
    ) -> Result<Vec<f64>, SimulationError> {
        self.apply_llrs(ncp_bits(order)?, llrs)
    }

    /// Generates the zero-loaded cell's specified BPSK label and clocks once.
    /// # Returns
    /// D0[0] as label0(+1) or1(-1); zero-loaded cells carry no payload bits.
    /// Receiver streams call this too to preserve synchronization and discard it.
    /// # Errors
    /// Rejects an inactive stream without advancing it.
    pub fn zero_loaded_bpsk(&mut self) -> Result<SymbolLabel, SimulationError> {
        self.require_symbol()?;
        let label = self.lfsr.mask(1);
        self.lfsr.clock();
        Ok(label)
    }

    /// Processes a scattered-pilot placeholder without clocking the LFSR.
    /// # Returns
    /// Unit with unchanged registers. Continuous pilots/PLC/exclusions are outside
    /// this data/NCP cell stream and must not be passed as data.
    /// # Errors
    /// Rejects an inactive stream.
    pub fn skip_scattered_pilot(&mut self) -> Result<(), SimulationError> {
        self.require_symbol()
    }

    fn require_symbol(&self) -> Result<(), SimulationError> {
        if self.symbol_index.is_none() {
            return Err(SimulationError::new(
                "RandomizerNotSynchronized",
                "Call begin_symbol(0) before processing cells.",
            ));
        }
        Ok(())
    }
    fn apply_label(
        &mut self,
        order: ConstellationOrder,
        label: SymbolLabel,
        bits: usize,
    ) -> Result<SymbolLabel, SimulationError> {
        self.require_symbol()?;
        if label >= order {
            return Err(invalid("Cell label exceeds constellation order."));
        }
        let output = label ^ self.lfsr.mask(bits);
        self.lfsr.clock();
        Ok(output)
    }
    fn apply_llrs(&mut self, bits: usize, llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
        self.require_symbol()?;
        if llrs.len() != bits || llrs.iter().any(|value| !value.is_finite()) {
            return Err(invalid("Finite LLRs matching the cell width are required."));
        }
        let mask = self.lfsr.mask(bits);
        let output = llrs
            .iter()
            .enumerate()
            .map(|(bit, value)| {
                if (mask >> (bits - 1 - bit)) & 1 == 1 {
                    -*value
                } else {
                    *value
                }
            })
            .collect();
        self.lfsr.clock();
        Ok(output)
    }
}
fn data_bits(order: ConstellationOrder) -> Result<usize, SimulationError> {
    OFDM_DATA_MODULATIONS
        .iter()
        .find(|entry| entry.order == order)
        .map(|entry| usize::from(entry.bits_per_symbol))
        .ok_or_else(|| {
            SimulationError::new(
                "UnsupportedModulation",
                "Unsupported downstream data randomization order.",
            )
        })
}
fn ncp_bits(order: ConstellationOrder) -> Result<usize, SimulationError> {
    match order {
        4 => Ok(2),
        16 => Ok(4),
        64 => Ok(6),
        _ => Err(SimulationError::new(
            "UnsupportedModulation",
            "NCP randomization requires QPSK,16-QAM or64-QAM.",
        )),
    }
}
