/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real NCP encode/map/demap/decode example; no waveform or dynamic PLC state.
use docsis_ofdm_fec::{
    signal_processing::modulation::{map_symbol, qam_llrs},
    transmission::{
        ncp::{NcpFieldCodec, NcpMessage},
        packing::NcpModulation,
    },
};

/// Prints a CRC-accepted two-message NCP field and actual LDPC iteration counts.
///
/// # Returns
/// Unit after writing the decoded reference experiment as JSON to standard output.
/// # Errors
/// Propagates codec setup, field validation, mapping, decoding or JSON failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut codec = NcpFieldCodec::new()?;
    let start = NcpMessage {
        profile_id: 0,
        zero_loaded: false,
        data_profile_odd: false,
        ncp_profile_odd: false,
        last: false,
        tagged: false,
        ncp_profile_update: false,
        pointer: 0,
    };
    let filler = NcpMessage {
        zero_loaded: true,
        last: true,
        pointer: 2025,
        ..start.clone()
    };
    let modulation = NcpModulation::Qpsk;
    let encoded = codec.encode_messages(&[start, filler], modulation)?;
    // This isolated control example bypasses shared cell randomization. The
    // cross-symbol integration test exercises that stage with payload data.
    let cells = encoded
        .labels
        .iter()
        .map(|label| {
            qam_llrs(
                modulation.order(),
                map_symbol(modulation.order(), *label)?,
                0.01,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let decoded = codec.decode_field(&cells, modulation, 100)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "experiment":"isolated_noiseless_ncp_field",
            "ncp_modulation":modulation,
            "decoded":decoded,
            "data_spans":decoded.data_spans(2100)?,
        }))?
    );
    Ok(())
}
