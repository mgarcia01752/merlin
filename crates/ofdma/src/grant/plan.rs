/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Immutable bit-role lengths and stream offsets, without source bits.
use fec_codec::upstream_ldpc::DocsisLdpcCode;
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;

/// One ordered transmitted word with trailing-zero mother-code shortening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CodewordPlan {
    /// Normative upstream mother identity.
    pub code: DocsisLdpcCode,
    /// Retained information count before appended shortening zeros.
    pub information_bits: usize,
    /// Retained information plus full parity.
    pub transmitted_bits: usize,
    /// Offset in the encoded grant before randomization.
    pub coded_offset: usize,
    /// Offset in the concatenation of retained information regions.
    pub information_offset: usize,
}

/// Pure capacity/layout result derived from accepted resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GrantPlan {
    /// Available transmitted coded positions, including any final grant pad.
    pub capacity_bits: usize,
    /// False below the explicit 700-bit no-transmission policy.
    pub transmit: bool,
    /// Ordered full/shortened long, medium and short descriptors.
    pub codewords: Vec<CodewordPlan>,
    /// Whole-byte source capacity; does not include information residue ones.
    pub source_bits: usize,
    /// Final one-valued information residue before FEC.
    pub information_fill_bits: usize,
    /// Final zero pad outside FEC, before grant randomization.
    pub grant_pad_bits: usize,
    /// Capacity not emitted in a no-transmission outcome.
    pub unused_capacity_bits: usize,
}

impl RetainedPayload for CodewordPlan {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            code: _,
            information_bits,
            transmitted_bits,
            coded_offset,
            information_offset,
        } = self;
        sum([
            information_bits.retained_children(),
            transmitted_bits.retained_children(),
            coded_offset.retained_children(),
            information_offset.retained_children(),
        ])
    }
}

impl RetainedPayload for GrantPlan {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            capacity_bits,
            transmit,
            codewords,
            source_bits,
            information_fill_bits,
            grant_pad_bits,
            unused_capacity_bits,
        } = self;
        sum([
            capacity_bits.retained_children(),
            transmit.retained_children(),
            codewords.retained_children(),
            source_bits.retained_children(),
            information_fill_bits.retained_children(),
            grant_pad_bits.retained_children(),
            unused_capacity_bits.retained_children(),
        ])
    }
}
