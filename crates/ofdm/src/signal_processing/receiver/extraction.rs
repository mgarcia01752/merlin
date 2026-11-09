/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! RX protected-bin extraction, using received/bootstrap layout and channel phase.
use crate::{
    signal_processing::waveform::samples::validate_samples,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    types::{IqPair, SubcarrierIdx},
    SimulationError,
};
/// Received pilot with its physical frequency-bin identity and role.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReceivedPilot {
    pub bin: SubcarrierIdx,
    pub role: PhysicalCarrierRole,
    pub iq: IqPair,
}
/// Separate recovered PLC, pilot and reduced data/scattered domains.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExtractedSignals {
    pub plc: Vec<IqPair>,
    pub pilots: Vec<ReceivedPilot>,
    /// Scattered-pilot positions are None; these are not missing input samples.
    pub reduced: Vec<Option<IqPair>>,
}
/// Separates normalized physical bins without relocating protected carriers.
/// # Arguments
/// * `layout` - Receiver-owned map from explicit bootstrap or received configuration.
/// * `symbol_index` - Received PLC-aligned channel phase, not source-codeword index.
/// * `bins` - Complete finite normalized FFT grid in increasing physical order.
/// # Returns
/// PLC in ascending-bin order, pilot records and reduced rank with pilot placeholders.
/// # Errors
/// Rejects wrong grid length or nonfinite I/Q; never substitutes TX cell values.
pub fn extract(
    layout: &CarrierLayout,
    symbol_index: u64,
    bins: &[IqPair],
) -> Result<ExtractedSignals, SimulationError> {
    let roles = layout.symbol_roles(symbol_index);
    validate_samples(bins, roles.len())?;
    let mut output = ExtractedSignals {
        plc: Vec::new(),
        pilots: Vec::new(),
        reduced: Vec::new(),
    };
    for (bin, (role, iq)) in roles.iter().zip(bins).enumerate() {
        match role {
            PhysicalCarrierRole::Data => output.reduced.push(Some(*iq)),
            PhysicalCarrierRole::ScatteredPilot => {
                output.reduced.push(None);
                output.pilots.push(ReceivedPilot {
                    bin: bin as SubcarrierIdx,
                    role: *role,
                    iq: *iq,
                });
            }
            PhysicalCarrierRole::ContinuousPilot => output.pilots.push(ReceivedPilot {
                bin: bin as SubcarrierIdx,
                role: *role,
                iq: *iq,
            }),
            PhysicalCarrierRole::Plc => output.plc.push(*iq),
            PhysicalCarrierRole::Excluded => {}
        }
    }
    Ok(output)
}
