/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.6.2/Figures56–58, PDF130–132.
use super::InterleaverSettings;
use crate::{error::invalid, transmission::SubcarrierSpacing, SimulationError};
const ROW_BITS: u32 = 7;
const ROW_COUNT: usize = 1 << ROW_BITS;
const ROTATION_SEED: usize = 17;
/// Reusable frequency permutation; same NI gives the same mapping each symbol.
pub struct FrequencyInterleaver {
    output_to_input: Vec<usize>,
    input_to_output: Vec<usize>,
}
impl FrequencyInterleaver {
    /// Builds bit-reversed store indices and GF(64) column rotations.
    /// # Arguments
    /// * `spacing` - Numerology selecting the maximum NI.
    /// * `cells` - Fixed reduced-domain NI, including pilot placeholders.
    /// # Returns
    /// Owned permutation and exact inverse, computed once per configuration.
    /// # Errors
    /// Rejects invalid NI or an internally inconsistent permutation.
    pub fn new(spacing: SubcarrierSpacing, cells: usize) -> Result<Self, SimulationError> {
        InterleaverSettings::new(spacing, cells, 1)?;
        let columns = cells.div_ceil(ROW_COUNT);
        let last_size = cells - ROW_COUNT * (columns - 1);
        let mut store = vec![vec![usize::MAX; ROW_COUNT]; columns];
        let mut source = 0;
        for row_counter in 0..ROW_COUNT {
            let row = reverse_bits(row_counter, ROW_BITS);
            let width = if row < last_size {
                columns
            } else {
                columns - 1
            };
            for column in store.iter_mut().take(width) {
                column[row] = source;
                source += 1;
            }
        }
        let mut rotation = ROTATION_SEED;
        for column in store.iter_mut().take(columns - 1) {
            column.rotate_right(rotation);
            let feedback = rotation & 1;
            rotation = (rotation >> 1) ^ (feedback << 5) ^ (feedback << 4);
        }
        let column_bits = usize::BITS - (columns - 1).leading_zeros();
        let mut output_to_input = Vec::with_capacity(cells);
        for counter in 0..columns {
            let reversed = reverse_bits(counter, column_bits);
            let column = if reversed < columns {
                reversed
            } else {
                counter
            };
            let count = if column == columns - 1 {
                last_size
            } else {
                ROW_COUNT
            };
            output_to_input.extend_from_slice(&store[column][..count]);
        }
        if output_to_input.len() != cells {
            return Err(invalid("Frequency permutation did not fill the domain."));
        }
        let mut input_to_output = vec![usize::MAX; cells];
        for (output, &input) in output_to_input.iter().enumerate() {
            if input >= cells || input_to_output[input] != usize::MAX {
                return Err(invalid("Frequency permutation is not a bijection."));
            }
            input_to_output[input] = output;
        }
        Ok(Self {
            output_to_input,
            input_to_output,
        })
    }
    /// Borrows indices identifying the input source of each output rank.
    /// # Returns
    /// Read-only NI-element map, valid for this object's lifetime.
    pub fn output_to_input(&self) -> &[usize] {
        &self.output_to_input
    }
    /// Applies frequency interleaving without changing its borrowed input.
    /// # Arguments
    /// * `input` - Exactly NI arbitrary cells; elements are cloned into output order.
    /// # Returns
    /// Owned output with no numerical changes or extra time delay.
    /// # Errors
    /// Rejects a length inconsistent with the fixed permutation.
    pub fn interleave<CellValue: Clone>(
        &self,
        input: &[CellValue],
    ) -> Result<Vec<CellValue>, SimulationError> {
        reorder(input, &self.output_to_input)
    }
    /// Reverses frequency interleaving without changing its borrowed input.
    /// # Arguments
    /// * `input` - Exactly NI cells in frequency-interleaved order.
    /// # Returns
    /// Owned cells restored to their pre-frequency ranks.
    /// # Errors
    /// Rejects a length inconsistent with the fixed permutation.
    pub fn deinterleave<CellValue: Clone>(
        &self,
        input: &[CellValue],
    ) -> Result<Vec<CellValue>, SimulationError> {
        reorder(input, &self.input_to_output)
    }
}
fn reverse_bits(value: usize, width: u32) -> usize {
    if width == 0 {
        return 0;
    }
    value.reverse_bits() >> (usize::BITS - width)
}
fn reorder<CellValue: Clone>(
    input: &[CellValue],
    map: &[usize],
) -> Result<Vec<CellValue>, SimulationError> {
    if input.len() != map.len() {
        return Err(invalid("Frequency interleaver length mismatch."));
    }
    Ok(map.iter().map(|index| input[*index].clone()).collect())
}
