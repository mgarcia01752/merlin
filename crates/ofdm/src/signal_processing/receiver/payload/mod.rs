/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Received-configuration payload recovery with distinct control and data codec paths.
pub mod collector;
pub mod configuration;
pub mod control;
pub mod decoder;
pub mod model;
pub use decoder::PayloadSymbolDecoder;
pub use model::{PayloadSymbolResult, RecoveredPayload, SoftDataCell, SoftNcpCell};

pub mod variable;
