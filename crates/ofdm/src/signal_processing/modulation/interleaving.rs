/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Verified bit-order permutation for the current coding experiment.
use fec_codec::parameters::{LDPC_CODEWORD_BITS, LDPC_INPUT_BITS};
const INTERLEAVER_COLUMNS: usize = 8;
const INTERLEAVER_ROWS: usize = 2025;
// PHY I20 Tables24/25; ETSI EN302769 V1.2.1 Table7(a), short16-QAM.
const COLUMN_TWISTS: [usize; 8] = [0, 0, 0, 1, 7, 20, 20, 21];
// ETSI EN302769 V1.2.1 Table10(a), rate8/9, 16-QAM input -> substream.
const DEMULTIPLEXER: [usize; 8] = [7, 1, 4, 2, 5, 3, 6, 0];
/// Returns output-bit -> original-codeword-bit order for this coding experiment.
/// # Returns
/// A full16200-element bijection: parity interleave, twist, then16-QAM demux.
pub fn transmit_bit_order() -> Vec<usize> {
    let mut parity: Vec<usize> = (0..LDPC_CODEWORD_BITS).collect();
    for group in 0..5 {
        for offset in 0..360 {
            parity[LDPC_INPUT_BITS + 360 * group + offset] = LDPC_INPUT_BITS + 5 * offset + group;
        }
    }
    let mut twisted = vec![0; LDPC_CODEWORD_BITS];
    for (input, original) in parity.into_iter().enumerate() {
        let column = input / INTERLEAVER_ROWS;
        let row = (input % INTERLEAVER_ROWS + COLUMN_TWISTS[column]) % INTERLEAVER_ROWS;
        twisted[row * INTERLEAVER_COLUMNS + column] = original;
    }
    let mut order = vec![0; LDPC_CODEWORD_BITS];
    for (input, original) in twisted.into_iter().enumerate() {
        let output = input / INTERLEAVER_COLUMNS * INTERLEAVER_COLUMNS
            + DEMULTIPLEXER[input % INTERLEAVER_COLUMNS];
        order[output] = original;
    }
    order
}
