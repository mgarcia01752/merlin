/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded known-reference TX/RX waveform composition; stage math has separate owners.
pub mod observation;
pub mod receiver;
pub mod transmitter;
pub use receiver::{ReceivedSymbol, WaveformReceiver};
pub use transmitter::{TransmittedSymbol, WaveformTransmitter};

pub mod reference;
