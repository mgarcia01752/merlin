/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.3, Tables48/49, PDF151–152: rows are ascending local carriers.
use super::geometry::{PLC_DATA_SYMBOLS, PLC_PREAMBLE_SYMBOLS};
use crate::{
    error::invalid,
    transmission::SubcarrierSpacing,
    types::{IqPair, PLC_FRAME_SYMBOLS},
    SimulationError,
};
use serde::Serialize;
const ROWS_4K: [u8; 8] = [
    0b00101101, 0b10001110, 0b01111001, 0b01000110, 0b11101111, 0b10000001, 0b01010011, 0b00100011,
];
const ROWS_8K: [u8; 16] = [
    0b10010100, 0b01100100, 0b01110001, 0b00010111, 0b11001010, 0b00011001, 0b01110110, 0b11100010,
    0b01111001, 0b11110111, 0b11100000, 0b11010101, 0b11001100, 0b10111010, 0b01011000, 0b00100001,
];
/// One known PLC preamble column; values are unit-energy BPSK, never randomized.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlcPreambleSymbol {
    pub symbol_index: u64,
    pub spacing: SubcarrierSpacing,
    pub bits: Vec<u8>,
    pub iq: Vec<IqPair>,
}
/// Generates a preamble column using the shared post-preamble phase convention.
/// # Arguments
/// * `spacing` - Fixed FFT geometry.
/// * `symbol_index` - Absolute channel index, modulo128 in120..128.
/// # Returns
///8or16 ascending-carrier bits and BPSK values,0→+1 and1→−1.
/// # Errors
/// Rejects data phases; no implicit phase wrapping from a data symbol to preamble.
pub fn generate(
    spacing: SubcarrierSpacing,
    symbol_index: u64,
) -> Result<PlcPreambleSymbol, SimulationError> {
    let phase = (symbol_index % PLC_FRAME_SYMBOLS) as usize;
    if phase < PLC_DATA_SYMBOLS {
        return Err(invalid("PLC preamble requires channel phase120..127."));
    }
    let column = phase - PLC_DATA_SYMBOLS;
    let rows: &[u8] = match spacing {
        SubcarrierSpacing::Khz25 => &ROWS_8K,
        SubcarrierSpacing::Khz50 => &ROWS_4K,
    };
    let bits: Vec<_> = rows
        .iter()
        .map(|row| (row >> (PLC_PREAMBLE_SYMBOLS - 1 - column)) & 1)
        .collect();
    let iq = bits
        .iter()
        .map(|bit| if *bit == 0 { [1.0, 0.0] } else { [-1.0, 0.0] })
        .collect();
    Ok(PlcPreambleSymbol {
        symbol_index,
        spacing,
        bits,
        iq,
    })
}
