/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded actual demapper, derandomizer and completed LDPC-input observations.
mod policy;
pub(crate) mod store;
mod views;
pub use policy::{
    SoftObservationPolicy, SoftObservationPolicyInfo, MAX_RECEIVER_SOFT_CELLS,
    MAX_RECEIVER_SOFT_WORDS, MAX_RECEIVER_SOFT_WORD_LLRS,
};
pub use views::{
    SoftCellPage, SoftCellRequest, SoftCellSnapshot, SoftObservationSummary, SoftWordIdentity,
    SoftWordRequest, SoftWordSnapshot, MAX_SOFT_CELL_PAGE_CELLS, MAX_SOFT_CELL_PAGE_LLRS,
    MAX_SOFT_WORD_QUERY_LLRS,
};
