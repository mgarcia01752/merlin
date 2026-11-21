/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! PHY I20 §7.4.3.1.1 pp53–55 including the two-shortened-word borrowing case.
use super::{CodewordPlan, GrantPlan};
use crate::{
    configuration::{MAX_CODED_BITS, MAX_CODEWORDS},
    error::limit,
    PhyResult,
};
use fec_codec::upstream_ldpc::DocsisLdpcCode;

/// Selects all codewords and bit roles from coded capacity without source use.
///
/// `capacity` is the sum of loaded payload-cell bits, not source bits. Returns
/// owned ordered layout. Rejects the application capacity/descriptor bound.
/// Below 700 positions returns explicit no-transmission; no padding is emitted.
pub fn select(capacity: usize) -> PhyResult<GrantPlan> {
    if capacity > MAX_CODED_BITS {
        return Err(limit("grant.coded_bits"));
    }
    let mut result = GrantPlan {
        capacity_bits: capacity,
        transmit: capacity >= 700,
        codewords: Vec::new(),
        source_bits: 0,
        information_fill_bits: 0,
        grant_pad_bits: 0,
        unused_capacity_bits: 0,
    };
    if !result.transmit {
        result.unused_capacity_bits = capacity;
        return Ok(result);
    }
    let mut remaining = capacity;
    for (code, threshold) in [
        (DocsisLdpcCode::UpstreamLong, 11881),
        (DocsisLdpcCode::UpstreamMedium, 3421),
        (DocsisLdpcCode::UpstreamShort, 281),
    ] {
        let parameters = code.parameters();
        while remaining >= parameters.codeword_bits {
            result.codewords.push(word(code, parameters.codeword_bits));
            remaining -= parameters.codeword_bits;
        }
        if remaining >= threshold {
            if code == DocsisLdpcCode::UpstreamShort && remaining < 700 {
                let previous = result
                    .codewords
                    .last_mut()
                    .ok_or_else(|| limit("grant.borrowing"))?;
                previous.information_bits -= 420;
                previous.transmitted_bits -= 420;
                remaining += 420;
            }
            result.codewords.push(word(code, remaining));
            remaining = 0;
            break;
        }
    }
    if result.codewords.len() > MAX_CODEWORDS {
        return Err(limit("grant.codewords"));
    }
    let mut coded = 0;
    let mut information = 0;
    for word in &mut result.codewords {
        word.coded_offset = coded;
        word.information_offset = information;
        coded += word.transmitted_bits;
        information += word.information_bits;
    }
    result.source_bits = information / 8 * 8;
    result.information_fill_bits = information % 8;
    result.grant_pad_bits = remaining;
    Ok(result)
}

fn word(code: DocsisLdpcCode, transmitted_bits: usize) -> CodewordPlan {
    let dimensions = code.parameters();
    CodewordPlan {
        code,
        information_bits: transmitted_bits
            - (dimensions.codeword_bits - dimensions.information_bits),
        transmitted_bits,
        coded_offset: 0,
        information_offset: 0,
    }
}
