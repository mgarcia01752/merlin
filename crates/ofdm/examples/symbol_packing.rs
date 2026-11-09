/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pack real FEC words using synthetic pre-interleaving pilot masks; print summaries.
use docsis_ofdm_fec::{
    signal_processing::modulation::interleaving::BitInterleaver,
    simulation::source::SeededSource,
    transmission::{
        layout::ncp::PreInterleavingRole,
        packing::{NcpModulation, PackingSettings, SymbolPacker},
        SubcarrierSpacing,
    },
};
use fec_codec::concatenated::ConcatenatedCodec;

/// Prints bounded symbol plans; no actual NCP wire messages or pilot signals.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut packer = SymbolPacker::new(PackingSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz25,
        interleaver_cells: 800,
        ncp_modulation: NcpModulation::Qpsk,
        data_profile_id: 0,
    })?;
    let codec = ConcatenatedCodec::new()?;
    let interleaver = BitInterleaver::new(256)?;
    let mut source = SeededSource::new(1);
    for _ in 0..3 {
        let encoded = codec.encode(&source.next_payload())?;
        packer.enqueue(interleaver.encode_labels(&encoded.ldpc_codeword)?)?;
    }
    let mut summaries = Vec::new();
    while packer.queued_codewords() > 0 {
        if summaries.len() >= 100 {
            return Err("Demo exceeded bounded symbol count.".into());
        }
        let mut roles = vec![PreInterleavingRole::Data; 800];
        for slot in [0, 1 + summaries.len() % 128, 799] {
            roles[slot] = PreInterleavingRole::ScatteredPilotPlaceholder;
        }
        summaries.push(packer.pack_symbol(&roles)?.summary());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "domain":"synthetic_pre_interleaving_demo",
            "symbols":summaries,
        }))?
    );
    Ok(())
}
