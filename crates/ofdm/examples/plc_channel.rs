/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded acquisition-to-frequency-grid demonstration for both FFT geometries.
use docsis_ofdm_fec::{
    simulation::plc::{generate_frame, PlcFrameReceiver},
    transmission::{
        frame::assemble,
        layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
        plc::{
            descriptors::{parse, DescriptorIdentity, FixedPlcDescriptors},
            reassembly::PlcMessageReassembler,
            scheduling::PlcMessageScheduler,
            ExtendedTimestamp, PlcMessageFrame,
        },
        SubcarrierSpacing, TransmissionSettings,
    },
};
/// Runs two known-phase PLC frames per spacing and prints compact JSON results.
/// # Returns
/// Unit after reporting received descriptors, actual decoder iterations and samples.
/// # Errors
/// Propagates configuration, scientific, CRC, reassembly or serialization failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let divisor = match spacing {
            SubcarrierSpacing::Khz25 => 1,
            SubcarrierSpacing::Khz50 => 2,
        };
        let settings = TransmissionSettings {
            occupied_bandwidth_hz: 24_000_000,
            subcarrier_spacing: spacing,
            ..Default::default()
        };
        let layout = CarrierLayout::new(
            &settings,
            &CarrierLayoutSettings {
                zero_frequency_hz: 100_000_000,
                first_active_bin: 320 / divisor,
                active_end_exclusive: 1280 / divisor,
                plc_first_bin: 752 / divisor,
                excluded_bins: vec![1200 / divisor],
                additional_continuous_pilots: [352, 448, 544, 920, 1008, 1080, 1176, 1256]
                    .map(|bin| bin / divisor)
                    .to_vec(),
            },
        )?;
        let descriptors = FixedPlcDescriptors::new(
            &settings,
            &layout,
            DescriptorIdentity {
                channel_id: 7,
                change_count: 0,
                source_mac: [0, 0xca, 0, 0, 0, 1],
                primary_capable: true,
                ncp_bits: 4,
            },
        )?;
        let mut scheduler = PlcMessageScheduler::new(&settings, &descriptors)?;
        let mut receiver = PlcFrameReceiver::new(spacing)?;
        let mut messages = PlcMessageReassembler::new();
        let frame_samples = u64::from(settings.one_second_timing()?.symbol_period_samples) * 128;
        let mut frames = Vec::new();
        let mut received_descriptors = Vec::new();
        for frame_index in 0..2 {
            let mc = scheduler.next_block()?;
            let frame = generate_frame(
                PlcMessageFrame::new(
                    spacing,
                    ExtendedTimestamp::from_sample_ticks(frame_index * frame_samples, 0)?,
                    mc.block,
                )?,
                frame_index,
            )?;
            let original = frame.message.to_bytes();
            let mut received = Vec::new();
            for phase in 0..128 {
                let symbol = frame_index * 128 + phase;
                let roles = layout.symbol_roles(symbol);
                // This PLC-focused example supplies an explicit normalized data/NCP
                // plane. Full payload/NCP waveform orchestration remains a later stage.
                let reduced: Vec<_> = roles
                    .iter()
                    .filter_map(|role| match role {
                        PhysicalCarrierRole::Data => Some(Some([0.25, -0.5])),
                        PhysicalCarrierRole::ScatteredPilot => Some(None),
                        _ => None,
                    })
                    .collect();
                let grid = assemble(
                    &layout,
                    symbol,
                    &reduced,
                    frame.symbol_iq(symbol).ok_or("Missing PLC symbol")?,
                    1.0,
                )?;
                for (role, point) in grid.roles.iter().zip(grid.bins) {
                    if *role == PhysicalCarrierRole::Plc {
                        received.push(point.ok_or("Missing received PLC cell")?);
                    }
                }
            }
            let decoded = receiver.decode(&received, 0.01, 100)?;
            let message = decoded
                .message
                .as_ref()
                .ok_or("PLC message recovery failed")?;
            let recovered = message.to_bytes();
            let bit_errors: u32 = original
                .iter()
                .zip(&recovered)
                .map(|(left, right)| (left ^ right).count_ones())
                .sum();
            for packet in messages.push(
                &docsis_ofdm_fec::transmission::plc::MessageChannelBlock::from_bytes(
                    &recovered[12..],
                )?,
            )? {
                received_descriptors.push(parse(&packet)?);
            }
            frames.push(serde_json::json!({"frame_index":frame_index,"compared_information_bits":original.len()*8,"bit_errors":bit_errors,"preamble_signs_match":decoded.preamble_signs_match,"ldpc_iterations":decoded.blocks.iter().map(|block|block.iterations).collect::<Vec<_>>(),"first_randomized_labels":&frame.codewords[0].randomized.values()[..8],"first_plc_iq":&frame.codewords[0].iq[..8]}));
        }
        results.push(serde_json::json!({"spacing_hz":spacing.frequency_hz(),"fft_bins":spacing.fft_samples(),"repeat_frames":scheduler.repeat_frames(),"received_descriptors":received_descriptors,"frames":frames}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"experiment":"known_phase_plc_frequency_grid","results":results})
        )?
    );
    Ok(())
}
