/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded tagged-cell example; payload/NCP integration is tested separately.
use docsis_ofdm_fec::{
    signal_processing::interleaving::{
        InterleaverDirection, InterleaverSettings, SymbolInterleaver,
    },
    transmission::SubcarrierSpacing,
};

/// Prints matched interleaver latency and restored tag prefixes as JSON.
/// # Returns
/// Unit after validating four input symbols plus drain and writing JSON to stdout.
/// # Errors
/// Propagates invalid configuration, sequencing, tag mismatch or JSON failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = InterleaverSettings::new(SubcarrierSpacing::Khz25, 129, 3)?;
    let mut transmit = SymbolInterleaver::new(settings, InterleaverDirection::Transmit)?;
    let mut receive = SymbolInterleaver::new(settings, InterleaverDirection::Receive)?;
    let sources: Vec<Vec<_>> = (0..4)
        .map(|symbol| {
            (0..settings.cells())
                .map(|rank| Some(symbol * 1000 + rank))
                .collect()
        })
        .collect();
    let mut summaries = Vec::new();
    for symbol in 0..sources.len() + settings.pair_delay_symbols() {
        let input = sources
            .get(symbol)
            .cloned()
            .unwrap_or(vec![None; settings.cells()]);
        let channel = transmit.process_symbol(symbol as u64, &input)?;
        let recovered = receive.process_symbol(symbol as u64, &channel)?;
        let source = symbol.checked_sub(settings.pair_delay_symbols());
        let expected = source
            .map(|index| sources[index].clone())
            .unwrap_or(vec![None; settings.cells()]);
        if recovered != expected {
            return Err("Interleaver did not restore the expected delayed tags.".into());
        }
        summaries.push(
            serde_json::json!({"output_symbol":symbol,"source_symbol":source,
            "restored_cells":recovered.iter().flatten().count(),"prefix":&recovered[..4]}),
        );
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "experiment":"synthetic_tagged_cells","interleaver_cells":settings.cells(),
            "time_depth":settings.depth(),"pair_delay_symbols":settings.pair_delay_symbols(),
            "symbols":summaries,
        }))?
    );
    Ok(())
}
