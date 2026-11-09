/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable TX payload/NCP/filler cell assembly before the physical waveform.
pub mod assembly;
pub mod model;
pub use assembly::assemble_packed;
pub use model::{AssembledSymbol, TransportCell, PAYLOAD_BITS, PAYLOAD_ORDER, PAYLOAD_PROFILE_ID};
