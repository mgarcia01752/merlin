/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded acquisition-to-frequency-grid demonstration for both FFT geometries.
use docsis_ofdm_fec::{
    simulation::plc::{generate_frame, PlcAcquisitionReceiver},
    transmission::{
        frame::assemble,
        layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
        plc::{
            descriptors::{parse, DescriptorIdentity, FixedPlcDescriptors},
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
        let mut receiver = PlcAcquisitionReceiver::new(spacing)?;
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
            let decoded = receiver.receive_frame(&received, 0.01, 100)?;
            let message = decoded
                .frame
                .message
                .as_ref()
                .ok_or("PLC message recovery failed")?;
            let recovered = message.to_bytes();
            let bit_errors: u32 = original
                .iter()
                .zip(&recovered)
                .map(|(left, right)| (left ^ right).count_ones())
                .sum();
            for admission in &decoded.admissions {
                if let Some(error) = &admission.error {
                    return Err(error.clone().into());
                }
                received_descriptors.push(parse(&admission.packet)?);
            }
            frames.push(serde_json::json!({"frame_index":frame_index,"compared_information_bits":original.len()*8,"bit_errors":bit_errors,"preamble_signs_match":decoded.frame.preamble_signs_match,"ldpc_iterations":decoded.frame.blocks.iter().map(|block|block.iterations).collect::<Vec<_>>(),"first_randomized_labels":&frame.codewords[0].randomized.values()[..8],"first_plc_iq":&frame.codewords[0].iq[..8]}));
        }
        let configuration = receiver.configuration().require_ready()?;
        let received_layout = configuration.channel().layout_settings();
        results.push(serde_json::json!({"spacing_hz":spacing.frequency_hz(),"fft_bins":spacing.fft_samples(),"repeat_frames":scheduler.repeat_frames(),"received_descriptors":received_descriptors,"frames":frames,
            "receiver_configuration": {
                "status":receiver.configuration().status(),"channel_id":configuration.channel_id(),
                "ocd_change_count":configuration.ocd_change_count(),"profile_a_change_count":configuration.profile_a().change_count(),"ncp_change_count":configuration.ncp().change_count(),
                "zero_frequency_hz":received_layout.zero_frequency_hz,"plc_first_bin":received_layout.plc_first_bin,
                "cp_samples":configuration.channel().transmission().cyclic_prefix_samples,
                "interleaver_cells":configuration.interleaver_settings()?.cells(),"interleaver_depth":configuration.interleaver_settings()?.depth(),
                "ncp_constellation_order":configuration.ncp_modulation().order()
            }
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"experiment":"known_phase_plc_frequency_grid","results":results})
        )?
    );
    Ok(())
}
