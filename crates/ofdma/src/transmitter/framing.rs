/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! TX-01/02: source fill, exact shortening and ordered grant bit roles.
use crate::{
    error::invalid,
    grant::{select, GrantPlan},
    PhyResult,
};
use fec_codec::upstream_ldpc::{LdpcCodec, LdpcSettings};

/// Encodes exact meaningful `source` according to a verified pure `plan`.
///
/// Returns ordered transmitted words plus zero grant pad, without BCH/puncture.
/// Information residue is one-valued. Rejects altered plans, length/nonbinary
/// input and codec errors; borrowed data and source generators are unchanged.
pub fn encode_grant(plan: &GrantPlan, source: &[u8]) -> PhyResult<Vec<u8>> {
    if *plan != select(plan.capacity_bits)?
        || source.len() != plan.source_bits
        || source.iter().any(|bit| *bit > 1)
    {
        return Err(invalid("transmitter.plan_or_source"));
    }
    if !plan.transmit {
        return Ok(Vec::new());
    }
    let mut information = source.to_vec();
    information.extend(std::iter::repeat(1).take(plan.information_fill_bits));
    let mut encoded = Vec::with_capacity(plan.capacity_bits);
    for word in &plan.codewords {
        let codec = LdpcCodec::new(LdpcSettings::new(word.code, word.information_bits, 1)?)?;
        encoded.extend(codec.encode(
            &information[word.information_offset..word.information_offset + word.information_bits],
        )?);
    }
    encoded.resize(plan.capacity_bits, 0);
    Ok(encoded)
}
