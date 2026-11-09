/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.6 and Table21: PLC dimensions and explicit puncture coordinates.
/// Information bits; no shortened positions are introduced for PLC.
pub const PLC_INFORMATION_BITS: usize = 288;
/// Unpunctured systematic mother-code length.
pub const PLC_MOTHER_BITS: usize = 480;
/// Transmitted PLC codeword after deleting two48-coordinate ranges.
pub const PLC_CODEWORD_BITS: usize = 384;
/// Bytes before PLC FEC encoding.
pub const PLC_INFORMATION_BYTES: usize = PLC_INFORMATION_BITS / 8;
pub(super) const LIFTING_FACTOR: usize = 48;
pub(super) const PARITY_CHECKS: usize = PLC_MOTHER_BITS - PLC_INFORMATION_BITS;
/// Application work limit; not a mandated DOCSIS receiver iteration count.
pub const MAX_PLC_ITERATIONS: usize = 100;
/// Numerical saturation applied before decoder arithmetic; not a known-zero prior.
pub const MAX_PLC_LLR_MAGNITUDE: f64 = 100.0;
const INFORMATION_PUNCTURE: std::ops::Range<usize> = 48..96;
const PARITY_PUNCTURE: std::ops::Range<usize> = 384..432;
/// Ascending retained mother coordinates:0..47,96..383,432..479.
pub const PLC_TRANSMITTED_COORDINATES: [usize; PLC_CODEWORD_BITS] = retained_coordinates();
const fn retained_coordinates() -> [usize; PLC_CODEWORD_BITS] {
    let mut retained = [0; PLC_CODEWORD_BITS];
    let mut output_index = 0;
    let mut coordinate = 0;
    while coordinate < PLC_MOTHER_BITS {
        if !(coordinate >= INFORMATION_PUNCTURE.start && coordinate < INFORMATION_PUNCTURE.end
            || coordinate >= PARITY_PUNCTURE.start && coordinate < PARITY_PUNCTURE.end)
        {
            retained[output_index] = coordinate;
            output_index += 1;
        }
        coordinate += 1;
    }
    retained
}
