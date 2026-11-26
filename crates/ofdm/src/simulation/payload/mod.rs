/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Stateful encoded payload/NCP transport, separate from waveform math and reference experiments.
pub mod observation;
pub mod receiver;
pub mod transmitter;
pub use receiver::{PayloadReceiveSymbol, PayloadReceiver};
pub use transmitter::{
    EncodedPayload, EncodedShortenedPayload, PayloadChannelSymbol, PayloadTransmitSymbol,
    PayloadTransmitter,
};

pub mod reference;

#[cfg(feature = "pnm-input")]
pub(crate) mod role_projection;
pub mod shortened;
