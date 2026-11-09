/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! TX assembly preserves explicit pilots and separates wire cells from diagnostic IDs.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::modulation::randomization::CellRandomizer,
    transmission::{
        layout::ncp::PreInterleavingRole,
        ncp::NcpFieldCodec,
        packing::{NcpModulation, PackedCell, PackingSettings, SymbolPacker},
        symbols::{assemble_packed, TransportCell},
        SubcarrierSpacing,
    },
};
#[test]
fn all_ncp_orders_produce_bounded_mapped_cells_without_tx_identity() {
    for ncp in [
        NcpModulation::Qpsk,
        NcpModulation::Qam16,
        NcpModulation::Qam64,
    ] {
        let mut packer = SymbolPacker::new(PackingSettings {
            subcarrier_spacing: SubcarrierSpacing::Khz25,
            interleaver_cells: 200,
            ncp_modulation: ncp,
            data_profile_id: 0,
        })
        .unwrap();
        packer.enqueue(vec![7; 2025]).unwrap();
        let mut roles = vec![PreInterleavingRole::Data; 200];
        roles[3] = PreInterleavingRole::ScatteredPilotPlaceholder;
        let plan = packer.pack_symbol(&roles).unwrap();
        let mut randomizer = CellRandomizer::new();
        let result =
            assemble_packed(&plan, &NcpFieldCodec::new().unwrap(), &mut randomizer).unwrap();
        assert_eq!(result.cells.len(), 200);
        assert_eq!(result.cells[3], Some(TransportCell::PilotPlaceholder));
        assert_eq!(result.ncp.labels.len(), plan.ncp_reserved_cells);
        let json = serde_json::to_string(&result.cells).unwrap();
        assert!(!json.contains("codeword_id"));
        assert!(!json.contains("field_cell"));
        for cell in result.cells.into_iter().flatten() {
            if let TransportCell::Sample(point) = cell {
                assert!(point.iter().all(|value| value.is_finite()));
            }
        }
    }
}
#[test]
fn rejected_plan_preserves_randomizer_and_the_same_valid_symbol_can_follow() {
    let mut packer = SymbolPacker::new(PackingSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz25,
        interleaver_cells: 200,
        ncp_modulation: NcpModulation::Qpsk,
        data_profile_id: 0,
    })
    .unwrap();
    packer.enqueue(vec![0; 2025]).unwrap();
    let plan = packer
        .pack_symbol(&[PreInterleavingRole::Data; 200])
        .unwrap();
    let mut bad = plan.clone();
    if let PackedCell::Data { label, .. } = &mut bad.cells[0] {
        *label = 256;
    } else {
        panic!("expected payload");
    }
    let mut randomizer = CellRandomizer::new();
    let before = randomizer.state();
    let control = NcpFieldCodec::new().unwrap();
    assert!(assemble_packed(&bad, &control, &mut randomizer).is_err());
    assert_eq!(randomizer.state(), before);
    assert!(assemble_packed(&plan, &control, &mut randomizer).is_ok());
}
