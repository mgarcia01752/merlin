/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! NCP message fields, real control FEC/CRC and receiver-side pointer discovery.
#[cfg(feature = "simulation")]
mod field;
#[cfg(feature = "simulation")]
pub mod mapping;
pub mod message;
#[cfg(feature = "simulation")]
pub use field::{DecodedNcpField, EncodedNcpField, NcpDataSpan, NcpFieldCodec};
pub use message::{NcpMessage, NcpStartPointer, NULL_POINTER};
