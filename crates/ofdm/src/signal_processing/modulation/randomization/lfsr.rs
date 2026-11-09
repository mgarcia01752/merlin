/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Figures52/68 shared field arithmetic and LSB-first mask extraction.
use super::super::qam::SymbolLabel;

/// Storage for a12-bit GF element; the live LFSR enforces its field width.
pub type RandomizerRegister = u16;
const FIELD_BITS: usize = 12;
const FIELD_MASK: RandomizerRegister = 0x0fff;
// alpha^12 = alpha^6 + alpha^4 + alpha +1 over GF(2).
const FIELD_REDUCTION: RandomizerRegister = 0x0053;
const FIELD_HIGH_BIT: RandomizerRegister = 0x0800;
const FEEDBACK_ALPHA_EXPONENT: usize = 11;
const DATA_INITIAL_D0: RandomizerRegister = 0x0555;
const DATA_INITIAL_D1: RandomizerRegister = 0x0aaa;
const PLC_INITIAL_D0: RandomizerRegister = 0x04a7;
const PLC_INITIAL_D1: RandomizerRegister = 0x0b4c;

/// Copy of internal registers; editing a snapshot cannot modify a live stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct RandomizerState {
    /// D0 polynomial coefficients, bit0 representing alpha^0.
    pub register_d0: RandomizerRegister,
    /// D1 polynomial coefficients, bit0 representing alpha^0.
    pub register_d1: RandomizerRegister,
}

#[derive(Clone)]
pub(crate) struct Lfsr {
    state: RandomizerState,
}
impl Lfsr {
    /// Constructs the specified data/NCP seed state.
    /// # Returns
    /// D0=0x555,D1=0xAAA with both registers confined to12bits.
    pub(super) fn new() -> Self {
        Self {
            state: RandomizerState {
                register_d0: DATA_INITIAL_D0,
                register_d1: DATA_INITIAL_D1,
            },
        }
    }
    /// Constructs an independent PLC register pair from PHY I20 Figure68.
    /// # Returns
    /// D0=0x4A7,D1=0xB4C, without sharing data/NCP stream state.
    pub(crate) fn plc() -> Self {
        Self {
            state: RandomizerState {
                register_d0: PLC_INITIAL_D0,
                register_d1: PLC_INITIAL_D1,
            },
        }
    }
    /// Copies the current register values without changing the sequence.
    /// # Returns
    /// Independent read-only snapshot of live register values.
    pub(crate) fn state(&self) -> RandomizerState {
        self.state
    }

    /// Forms a label mask with D0[0] at the MSB, extended by D1 for13/14bits.
    /// # Arguments
    /// * `bits` - Validated cell width, at most14.
    /// # Returns
    /// Integer mask in the mapper's MSB-first label convention.
    pub(crate) fn mask(&self, bits: usize) -> SymbolLabel {
        let mut mask = 0;
        for bit_index in 0..bits {
            let bit = if bit_index < FIELD_BITS {
                (self.state.register_d0 >> bit_index) & 1
            } else {
                (self.state.register_d1 >> (bit_index - FIELD_BITS)) & 1
            };
            mask = (mask << 1) | bit;
        }
        mask
    }

    /// Advances both registers simultaneously using their old values.
    /// # Returns
    /// Unit with D0'=alpha^11*D1 and D1'=D0 XOR D1 in GF(2^12).
    pub(crate) fn clock(&mut self) {
        let mut feedback = self.state.register_d1;
        for _ in 0..FEEDBACK_ALPHA_EXPONENT {
            let carry = feedback & FIELD_HIGH_BIT != 0;
            feedback = (feedback << 1) & FIELD_MASK;
            if carry {
                feedback ^= FIELD_REDUCTION;
            }
        }
        self.state = RandomizerState {
            register_d0: feedback,
            register_d1: self.state.register_d0 ^ self.state.register_d1,
        };
    }
}
