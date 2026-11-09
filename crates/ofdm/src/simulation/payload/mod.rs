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
    EncodedPayload, PayloadChannelSymbol, PayloadTransmitSymbol, PayloadTransmitter,
};

pub mod reference;
