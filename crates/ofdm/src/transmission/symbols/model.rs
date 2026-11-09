/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Cell values carry scientific roles, never transmitter codeword identities to RX.
use crate::{transmission::ncp::EncodedNcpField, types::IqPair};
/// Supported payload constellation; mode expansion must verify coding and placement first.
pub const PAYLOAD_ORDER: crate::types::ofdm_modulation::ConstellationOrder =
    1 << crate::transmission::settings::DATA_BITS_PER_CELL;
/// Coded bits per supported payload cell, distinct from NCP modulation width.
pub const PAYLOAD_BITS: usize = crate::transmission::settings::DATA_BITS_PER_CELL as usize;
/// Fixed initial downstream data profile.
pub const PAYLOAD_PROFILE_ID: crate::types::ProfileId = 0;
/// Interleaver value; outer None represents unavailable history, not a pilot.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(tag = "kind", content = "iq", rename_all = "snake_case")]
pub enum TransportCell {
    PilotPlaceholder,
    Sample(IqPair),
}
/// Completed pre-interleaving TX signal stages; field diagnostics remain TX-only.
pub struct AssembledSymbol {
    pub local_symbol_index: u64,
    pub ncp: EncodedNcpField,
    pub cells: Vec<Option<TransportCell>>,
}
