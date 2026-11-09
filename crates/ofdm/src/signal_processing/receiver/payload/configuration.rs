/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit supported fixed-profile subset, derived from received PLC descriptors.
use crate::{
    error::invalid,
    transmission::{packing::PackingSettings, plc::receiver_configuration::ReceiverConfiguration},
    types::PLC_FRAME_SYMBOLS,
    LabError,
};
/// Derives supported payload packing geometry without substituting missing/unsupported assignments.
/// # Arguments
/// * `configuration` - Complete snapshot obtained through the readiness gate.
/// # Returns
/// Uniform256-QAM ProfileA and positive uniform NCP geometry. Descriptor counts remain independent.
/// # Errors
/// Rejects zero/mixed data or NCP loading and unsupported profile identity.
pub fn packing_settings(
    configuration: &ReceiverConfiguration,
) -> Result<PackingSettings, LabError> {
    let data = configuration.profile_a();
    let ncp = configuration.ncp();
    let ncp_bits = configuration.ncp_modulation().order().ilog2() as u8;
    if data.profile_id() != crate::transmission::symbols::PAYLOAD_PROFILE_ID
        || configuration.reduced_bins().iter().any(|bin| {
            data.bit_loading()[usize::from(*bin)] != Some(8)
                || ncp.bit_loading()[usize::from(*bin)] != Some(ncp_bits)
        })
    {
        return Err(LabError::new(
            "UnsupportedPayloadProfile",
            "Payload transport requires uniform256-QAM ProfileA and uniform positive NCP loading.",
        ));
    }
    let settings = PackingSettings {
        subcarrier_spacing: configuration.channel().transmission().subcarrier_spacing,
        interleaver_cells: configuration.reduced_bins().len(),
        ncp_modulation: configuration.ncp_modulation(),
        data_profile_id: data.profile_id(),
    };
    settings.validate()?;
    Ok(settings)
}
/// Validates an explicit post-acquisition payload-session origin.
/// # Arguments
/// * `origin` - Absolute channel/source symbol index for local symbol0.
/// # Returns
/// Unit when source randomizer resets align with the physical128-symbol PLC cycle.
/// # Errors
/// Rejects a nonboundary origin; arbitrary midstream randomizer acquisition is not implemented.
pub fn validate_origin(origin: u64) -> Result<(), LabError> {
    if origin % PLC_FRAME_SYMBOLS != 0 {
        return Err(invalid(
            "Payload session must start at a128-symbol PLC boundary.",
        ));
    }
    Ok(())
}
