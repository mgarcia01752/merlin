/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned pilot-stage outputs, separate from generation and observation.
use crate::{
    signal_processing::pilots::{PilotPower, PilotValue},
    transmission::SubcarrierSpacing,
};
use serde::Serialize;
/// Owned independent outputs for one physical symbol, without PLC or frame insertion.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PilotStageResult {
    pub symbol_index: u64,
    pub spacing: SubcarrierSpacing,
    pub power: PilotPower,
    pub continuous: Vec<PilotValue>,
    pub scattered: Vec<PilotValue>,
}
