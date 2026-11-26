/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Upstream application budgets, independent of normative waveform dimensions.
/// Inclusive generic upstream configuration input/output JSON byte budget.
pub const MAX_CONFIGURATION_DOCUMENT_BYTES: usize = 16 * 1024 * 1024;

/// Upstream BatchReport format identity, independent of configuration/FEC documents.
pub(crate) const UPSTREAM_BATCH_REPORT_FORMAT: &str = "merlin.upstream.experiment";
/// Existing upstream experiment report schema; independent of input schema versions.
pub(crate) const UPSTREAM_BATCH_REPORT_SCHEMA_VERSION: u32 = 1;
/// Existing channel-only generator identity and exact dependency pins retained in reports.
pub(crate) const UPSTREAM_BATCH_REPORT_NOISE_GENERATOR: &str =
    "chacha8-standardnormal-v1; rand=0.9.5; rand_chacha=0.9.0; rand_distr=0.5.1";
