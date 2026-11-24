/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared passive retained-payload metadata, independent of scientific owners.
use serde::Serialize;

pub mod support;

/// Target-layout resource quantities for one retained scientific root.
///
/// Includes the fixed root, requested storage capacity and nested/cached payloads.
/// Shared payloads/edges may be conservatively repeated. Excludes allocator,
/// private ordered-container node and reference-count bookkeeping from bytes;
/// this is not total heap allocation or process RSS. A value alone reserves
/// nothing and does not establish a future-product bound. A documented bounded
/// operation may supply preflight quantities or accept these as caller limits.
/// Temporary execution storage and independently retained consumer copies remain
/// outside the described owner and require their own policies/accounting.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RetainedFootprint {
    /// Fixed owned values plus requested payload storage capacity, in bytes.
    pub payload_bytes: usize,
    /// Ordered-container entries, independently of excluded private node layout.
    pub ordered_entries: usize,
    /// Shared allocation owner edges, independently of excluded refcount headers.
    pub shared_roots: usize,
}

#[cfg(test)]
mod tests;
