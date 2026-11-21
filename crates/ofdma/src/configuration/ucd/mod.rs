/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Pinned MULPI I25 OFDMA configuration, separate from synthetic run settings.
mod channel;
mod controls;
mod profiles;
pub use channel::{Band, OfdmaChannel, TimestampSnapshot};
pub use controls::{CmAssignment, MapControl, MapElement, ProbeControl, ProbeElement};
pub use profiles::{BurstDescriptor, IrPowerControl, ProfileGroup};
pub(crate) fn data_iuc(iuc: u8) -> bool {
    matches!(iuc, 5 | 6 | 9..=13)
}
