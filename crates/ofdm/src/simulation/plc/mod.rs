/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin PLC stage composition; numerical implementations remain under transmission.
pub mod interleaving;
pub use interleaving::PlcInterleavingStages;

pub mod receiver;
pub mod transmitter;
pub use receiver::{PlcFrameReceiver, ReceivedPlcFrame};
pub use transmitter::{generate_frame, PlcFrameSignals};

pub mod observation;

pub mod acquisition;
pub use acquisition::{PlcAcquisitionReceiver, PlcAcquisitionResult};
