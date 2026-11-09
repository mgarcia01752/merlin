/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Full known-phase PLC I/Q path, protected physical bins and independent receive input.
#![cfg(feature = "simulation")]
mod support;
use docsis_ofdm_fec::{
    simulation::plc::{generate_frame, PlcFrameReceiver},
    transmission::{
        frame::assemble,
        layout::PhysicalCarrierRole,
        plc::{mapping, ExtendedTimestamp, PlcMessageFrame},
        SubcarrierSpacing,
    },
};
#[test]
fn full_frames_map_assemble_extract_and_decode_without_tx_truth() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let layout = support::layout(spacing);
        let mut receiver = PlcFrameReceiver::new(spacing).unwrap();
        for frame_index in 0..3 {
            let timestamp = ExtendedTimestamp::new(100 + frame_index, 19, 0).unwrap();
            let message = PlcMessageFrame::idle(spacing, timestamp).unwrap();
            let expected = message.to_bytes();
            let signals = generate_frame(message, frame_index).unwrap();
            let mut received = Vec::new();
            for phase in 0..128 {
                let index = frame_index * 128 + phase;
                let roles = layout.symbol_roles(index);
                let reduced: Vec<_> = roles
                    .iter()
                    .filter_map(|role| match role {
                        PhysicalCarrierRole::Data => Some(Some([0.25, -0.5])),
                        PhysicalCarrierRole::ScatteredPilot => Some(None),
                        _ => None,
                    })
                    .collect();
                let plc = signals.symbol_iq(index).unwrap();
                let saved = reduced.clone();
                let physical = assemble(&layout, index, &reduced, plc, 1.0).unwrap();
                assert_eq!(reduced, saved);
                assert_eq!(physical.bins.len(), spacing.fft_samples() as usize);
                let mut plc_rank = 0;
                for (role, actual) in physical.roles.iter().zip(&physical.bins) {
                    match role {
                        PhysicalCarrierRole::Excluded => assert_eq!(*actual, Some([0.0, 0.0])),
                        PhysicalCarrierRole::Data => assert_eq!(*actual, Some([0.25, -0.5])),
                        PhysicalCarrierRole::Plc => {
                            let point = actual.unwrap();
                            assert_eq!(point, plc[plc_rank]);
                            received.push(point);
                            plc_rank += 1;
                        }
                        _ => {
                            let point = actual.unwrap();
                            assert_eq!(point[0].abs(), 2.0);
                            assert_eq!(point[1], 0.0);
                        }
                    }
                }
            }
            let result = receiver.decode(&received, 0.01, 100).unwrap();
            assert!(result.preamble_signs_match);
            assert!(result.message_error.is_none());
            assert!(result.blocks.iter().all(|block| block.success));
            assert_eq!(result.message.unwrap().to_bytes(), expected);
            assert!(signals.symbol_iq((frame_index + 1) * 128).is_none());
            if frame_index > 0 {
                assert!(signals.symbol_iq(frame_index * 128 - 1).is_none());
            }
        }
    }
}
#[test]
fn plc_mapper_uses_annex_a_coordinates_energy_and_soft_signs() {
    use docsis_ofdm_fec::transmission::plc::{InterleavedPlcBlock, PlcRandomizer};
    let zero = InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz25, &[0; 96]).unwrap();
    let mask = PlcRandomizer::new(SubcarrierSpacing::Khz25)
        .apply(&zero)
        .unwrap();
    let input: Vec<_> = (0..96)
        .map(|index| (index % 16) as u8 ^ mask.values()[index])
        .collect();
    let block = InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz25, &input).unwrap();
    let coded = PlcRandomizer::new(SubcarrierSpacing::Khz25)
        .apply(&block)
        .unwrap();
    let iq = mapping::map(&coded).unwrap();
    let llrs = mapping::demap(&iq, 0.01).unwrap();
    let mut power = 0.0;
    for (index, point) in iq.iter().enumerate() {
        let label = (index % 16) as u8;
        let real = if label & 2 == 0 { 3.0 } else { 1.0 };
        let imag = if label & 1 == 0 { 3.0 } else { 1.0 };
        let sign_i = if label & 8 == 0 { 1.0 } else { -1.0 };
        let sign_q = if label & 4 == 0 { 1.0 } else { -1.0 };
        assert_eq!(
            *point,
            [sign_i * real / 10_f64.sqrt(), sign_q * imag / 10_f64.sqrt()]
        );
        for bit in 0..4 {
            assert_eq!(llrs[index * 4 + bit] > 0.0, label & (8 >> bit) == 0);
        }
        power += point[0] * point[0] + point[1] * point[1];
    }
    assert!((power / 96.0 - 1.0).abs() < 1e-12);
    assert!(mapping::demap(&iq[..95], 0.01).is_err());
    assert!(mapping::demap(&iq, 0.0).is_err());
}
#[test]
fn assembly_preserves_missing_cells_and_rejects_bad_roles_counts_and_values() {
    let layout = support::layout(SubcarrierSpacing::Khz25);
    let roles = layout.symbol_roles(0);
    let mut reduced: Vec<_> = roles
        .iter()
        .filter_map(|role| match role {
            PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot => Some(None),
            _ => None,
        })
        .collect();
    let plc = vec![[1.0, 0.0]; 16];
    let grid = assemble(&layout, 0, &reduced, &plc, 1.0).unwrap();
    assert!(grid
        .roles
        .iter()
        .zip(&grid.bins)
        .all(|(role, point)| *role != PhysicalCarrierRole::Data || point.is_none()));
    assert!(assemble(&layout, 0, &reduced[..reduced.len() - 1], &plc, 1.0).is_err());
    assert!(assemble(&layout, 0, &reduced, &plc[..15], 1.0).is_err());
    assert!(assemble(&layout, 0, &reduced, &plc, f64::INFINITY).is_err());
    let rank = roles
        .iter()
        .filter(|role| {
            matches!(
                role,
                PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
            )
        })
        .position(|role| *role == PhysicalCarrierRole::ScatteredPilot)
        .unwrap();
    reduced[rank] = Some([1.0, 0.0]);
    assert!(assemble(&layout, 0, &reduced, &plc, 1.0).is_err());
    reduced[rank] = None;
    let rank = roles
        .iter()
        .filter(|role| {
            matches!(
                role,
                PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
            )
        })
        .position(|role| *role == PhysicalCarrierRole::Data)
        .unwrap();
    reduced[rank] = Some([f64::NAN, 0.0]);
    assert!(assemble(&layout, 0, &reduced, &plc, 1.0).is_err());
    assert!(generate_frame(
        PlcMessageFrame::idle(
            SubcarrierSpacing::Khz25,
            ExtendedTimestamp::new(0, 0, 0).unwrap()
        )
        .unwrap(),
        u64::MAX
    )
    .is_err());
}

#[test]
fn assembler_consumes_actual_cell_interleaver_history_without_filling_missing_data() {
    use docsis_ofdm_fec::{
        signal_processing::interleaving::{InterleaverDirection, SymbolInterleaver},
        transmission::layout::{ncp::PreInterleavingRole, PilotReference},
    };
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let layout = support::layout(spacing);
        let reference = PilotReference::new(&layout, 8).unwrap();
        let mut interleaver =
            SymbolInterleaver::new(reference.settings(), InterleaverDirection::Transmit).unwrap();
        let message =
            PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(0, 0, 0).unwrap()).unwrap();
        let signals = generate_frame(message, 0).unwrap();
        let mut saw_missing = false;
        for symbol in 0..128 {
            let cells: Vec<_> = reference
                .roles(symbol)
                .iter()
                .map(|role| match role {
                    PreInterleavingRole::ScatteredPilotPlaceholder => None,
                    _ => Some([0.25, -0.5]),
                })
                .collect();
            let output = interleaver.process_symbol(symbol, &cells).unwrap();
            let assembled = assemble(
                &layout,
                symbol,
                &output,
                signals.symbol_iq(symbol).unwrap(),
                1.0,
            )
            .unwrap();
            for (role, point) in assembled.roles.iter().zip(&assembled.bins) {
                if *role == PhysicalCarrierRole::Data && point.is_none() {
                    saw_missing = true;
                    assert!(symbol < 8);
                }
            }
        }
        assert!(saw_missing);
    }
}
