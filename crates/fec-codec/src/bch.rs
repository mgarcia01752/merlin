/*
 * Copyright (c) 2017-2026 aff3ct
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0 AND MIT
 *
 * Rust replacement for the bounded AFF3CT-derived BCH adapter. Retains the
 * upstream notice in crates/fec-codec/LICENSE-MIT. Implements binary-field
 * syndrome calculation, Berlekamp-Massey and Chien search with safe Rust.
 * Parameters: ETSI EN302769 V1.2.1 Tables3(b)/4(b), referenced by PHY I20.
 */
//! Shortened BCH(14400,14232), bounded-distance correction of up to12 errors.
use crate::{
    binary_bits,
    parameters::{BCH_INPUT_BITS, LDPC_INPUT_BITS},
};
use std::sync::OnceLock;

/// Complete corrected BCH word, with decoder status separate from truth BER.
pub struct BchResult {
    pub codeword: Vec<u8>,
    pub corrected_bits: u32,
    pub success: bool,
}

type FieldElement = u16;
const BCH_FIELD_BITS: usize = 14;
const BCH_FIELD_ORDER: usize = (1 << BCH_FIELD_BITS) - 1;
const BCH_CORRECTION_BITS: usize = 12;
const BCH_SYNDROMES: usize = 2 * BCH_CORRECTION_BITS;
const BCH_PARITY_BITS: usize = LDPC_INPUT_BITS - BCH_INPUT_BITS;
// ETSI Table4(b), g1..g12 in ascending coefficient order. g1 is primitive.
const BCH_GENERATOR_FACTORS: [FieldElement; BCH_CORRECTION_BITS] = [
    0x402b, 0x4941, 0x4647, 0x5591, 0x6b55, 0x6389, 0x6ce5, 0x4f21, 0x460f, 0x5a49, 0x5811, 0x65ef,
];
static BCH_KERNEL: OnceLock<Result<BchKernel, String>> = OnceLock::new();
struct BchKernel {
    powers: Vec<FieldElement>,
    logarithms: Vec<Option<usize>>,
    generator: Vec<u8>,
}
impl BchKernel {
    fn new() -> Result<Self, String> {
        let mut generator = vec![1u8];
        for factor in BCH_GENERATOR_FACTORS {
            let mut product = vec![0; generator.len() + BCH_FIELD_BITS];
            for (degree, coefficient) in generator.iter().enumerate() {
                for term in 0..=BCH_FIELD_BITS {
                    product[degree + term] ^= coefficient & ((factor >> term) as u8 & 1);
                }
            }
            generator = product;
        }
        let mut powers = vec![0; BCH_FIELD_ORDER];
        let mut logarithms = vec![None; BCH_FIELD_ORDER + 1];
        let mut element: FieldElement = 1;
        for (exponent, power) in powers.iter_mut().enumerate() {
            if logarithms[usize::from(element)].is_some() {
                return Err("BCH field polynomial is not primitive.".into());
            }
            *power = element;
            logarithms[usize::from(element)] = Some(exponent);
            element <<= 1;
            if element & (1 << BCH_FIELD_BITS) != 0 {
                element ^= BCH_GENERATOR_FACTORS[0];
            }
        }
        if element != 1 || generator.len() != BCH_PARITY_BITS + 1 {
            return Err("Invalid BCH field or generator.".into());
        }
        Ok(Self {
            powers,
            logarithms,
            generator,
        })
    }
    fn multiply(&self, first: FieldElement, second: FieldElement) -> FieldElement {
        match (
            self.logarithms[usize::from(first)],
            self.logarithms[usize::from(second)],
        ) {
            (Some(first), Some(second)) => self.powers[(first + second) % BCH_FIELD_ORDER],
            _ => 0,
        }
    }
    fn divide_nonzero(&self, numerator: FieldElement, denominator: FieldElement) -> FieldElement {
        // Only called for nonzero Berlekamp-Massey discrepancies.
        let numerator = self.logarithms[usize::from(numerator)].expect("Nonzero numerator");
        let denominator = self.logarithms[usize::from(denominator)].expect("Nonzero denominator");
        self.powers[(numerator + BCH_FIELD_ORDER - denominator) % BCH_FIELD_ORDER]
    }
    fn syndromes(&self, codeword: &[u8]) -> [FieldElement; BCH_SYNDROMES] {
        let mut syndromes = [0; BCH_SYNDROMES];
        for (root, syndrome) in syndromes.iter_mut().enumerate() {
            for (position, bit) in codeword.iter().rev().enumerate() {
                if *bit != 0 {
                    *syndrome ^= self.powers[((root + 1) * position) % BCH_FIELD_ORDER];
                }
            }
        }
        syndromes
    }
    fn locator(&self, syndromes: &[FieldElement; BCH_SYNDROMES]) -> Option<Vec<FieldElement>> {
        let mut locator = vec![0; BCH_SYNDROMES + 1];
        locator[0] = 1;
        let mut previous = locator.clone();
        let mut degree = 0;
        let mut shift = 1;
        let mut previous_discrepancy = 1;
        for step in 0..BCH_SYNDROMES {
            let mut discrepancy = syndromes[step];
            for coefficient in 1..=degree {
                discrepancy ^= self.multiply(locator[coefficient], syndromes[step - coefficient]);
            }
            if discrepancy == 0 {
                shift += 1;
                continue;
            }
            let unchanged = locator.clone();
            let scale = self.divide_nonzero(discrepancy, previous_discrepancy);
            for offset in 0..=BCH_SYNDROMES - shift {
                locator[offset + shift] ^= self.multiply(scale, previous[offset]);
            }
            if 2 * degree <= step {
                degree = step + 1 - degree;
                previous = unchanged;
                previous_discrepancy = discrepancy;
                shift = 1;
            } else {
                shift += 1;
            }
        }
        if degree == 0 || degree > BCH_CORRECTION_BITS {
            return None;
        }
        locator.truncate(degree + 1);
        Some(locator)
    }
    fn locations(&self, locator: &[FieldElement]) -> Option<Vec<usize>> {
        let degree = locator.len() - 1;
        let mut locations = Vec::with_capacity(degree);
        for position in 0..BCH_FIELD_ORDER {
            let inverse = self.powers[(BCH_FIELD_ORDER - position) % BCH_FIELD_ORDER];
            let value = locator.iter().rev().fold(0, |value, coefficient| {
                self.multiply(value, inverse) ^ coefficient
            });
            if value == 0 {
                // A root outside the transmitted shortened word is not correctable.
                if position >= LDPC_INPUT_BITS || locations.len() == degree {
                    return None;
                }
                locations.push(position);
            }
        }
        (locations.len() == degree).then_some(locations)
    }
}
fn kernel() -> Result<&'static BchKernel, String> {
    BCH_KERNEL
        .get_or_init(BchKernel::new)
        .as_ref()
        .map_err(Clone::clone)
}
/// Encodes a binary payload in systematic MSB-first order.
/// # Arguments
/// * `message` - Exactly14232 binary bits.
/// # Returns
/// The unchanged information bits followed by168BCH parity bits.
/// # Errors
/// Rejects invalid bit lengths/values and internal field/generator parameters.
pub fn encode_bch(message: &[u8]) -> Result<Vec<u8>, String> {
    binary_bits(message, BCH_INPUT_BITS)?;
    let kernel = kernel()?;
    let mut remainder = [0; BCH_PARITY_BITS];
    for bit in message {
        let feedback = bit ^ remainder[BCH_PARITY_BITS - 1];
        for degree in (1..BCH_PARITY_BITS).rev() {
            remainder[degree] = remainder[degree - 1] ^ (kernel.generator[degree] & feedback);
        }
        remainder[0] = kernel.generator[0] & feedback;
    }
    let mut result = Vec::with_capacity(LDPC_INPUT_BITS);
    result.extend_from_slice(message);
    result.extend(remainder.iter().rev());
    Ok(result)
}
/// Decodes a binary word; failures retain the original decisions.
/// # Arguments
/// * `codeword` - Exactly14400 binary bits in MSB-first wire order.
/// # Returns
/// Complete corrected word, correction count and post-syndrome-checked status.
/// # Errors
/// Rejects invalid bit lengths/values and internal field/generator parameters.
pub fn decode_bch(codeword: &[u8]) -> Result<BchResult, String> {
    binary_bits(codeword, LDPC_INPUT_BITS)?;
    let kernel = kernel()?;
    let failure = || {
        log::debug!("payload-bch-not-correctable information-bits={BCH_INPUT_BITS}");
        BchResult {
            codeword: codeword.to_vec(),
            corrected_bits: 0,
            success: false,
        }
    };
    let syndromes = kernel.syndromes(codeword);
    if syndromes.iter().all(|syndrome| *syndrome == 0) {
        return Ok(BchResult {
            codeword: codeword.to_vec(),
            corrected_bits: 0,
            success: true,
        });
    }
    let Some(locator) = kernel.locator(&syndromes) else {
        return Ok(failure());
    };
    let Some(locations) = kernel.locations(&locator) else {
        return Ok(failure());
    };
    let mut corrected = codeword.to_vec();
    for position in &locations {
        corrected[LDPC_INPUT_BITS - 1 - position] ^= 1;
    }
    if kernel
        .syndromes(&corrected)
        .iter()
        .any(|syndrome| *syndrome != 0)
    {
        return Ok(failure());
    }
    Ok(BchResult {
        codeword: corrected,
        corrected_bits: locations.len() as u32,
        success: true,
    })
}
