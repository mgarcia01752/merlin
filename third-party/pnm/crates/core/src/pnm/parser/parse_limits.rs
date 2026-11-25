/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::pnm::data_type::SampleCount;
const DEFAULT_MAX_SAMPLES: SampleCount = 65_536;

/// Application resource policy, not a DOCSIS format limit.
#[derive(Debug, Clone, Copy)]
pub struct ParseLimits {
    /// Maximum decoded items accepted by this caller. Nested formats also count
    /// profile/flow containers and records; latency counts bin edges and bins.
    pub max_samples: SampleCount,
}
impl Default for ParseLimits {
    /// Builds the default application sample budget.
    ///
    /// # Returns
    /// A limit of 65,536 samples; this is not a DOCSIS limit.
    fn default() -> Self {
        Self {
            max_samples: DEFAULT_MAX_SAMPLES,
        }
    }
}
