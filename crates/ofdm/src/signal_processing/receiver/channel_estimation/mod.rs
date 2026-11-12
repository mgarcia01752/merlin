/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Receiver Channel Estimation (CH-EST-RX): independent pilot LS and bounded history.
mod interpolation;
mod reliability;
pub use reliability::{MceSettings, ReceiverReliability, ReliabilityEstimator};
mod model;
mod pilot;
pub use model::{ChannelEstimate, PilotEstimate, ReceiverMode};
pub use pilot::PilotChannelEstimator;

pub mod timing;

pub mod acquisition;

pub mod search;
