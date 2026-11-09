/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent symbol capacity, pointer-domain and queue lifecycle expectations.
use docsis_ofdm_fec::transmission::{
    layout::ncp::PreInterleavingRole,
    packing::{NcpModulation, NcpPlanEntry, PackedCell, PackingSettings, SymbolPacker},
    SubcarrierSpacing,
};

fn settings(cells: usize) -> PackingSettings {
    PackingSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz25,
        interleaver_cells: cells,
        ncp_modulation: NcpModulation::Qpsk,
        data_profile_id: 3,
    }
}

#[test]
fn pilot_placeholders_do_not_consume_data_or_pointer_ranks() {
    let mut packer = SymbolPacker::new(settings(100)).unwrap();
    packer.enqueue(vec![0x35; 2025]).unwrap();
    let mut roles = vec![PreInterleavingRole::Data; 100];
    for slot in [0, 5, 99] {
        roles[slot] = PreInterleavingRole::ScatteredPilotPlaceholder;
    }
    let original = roles.clone();
    let symbol = packer.pack_symbol(&roles).unwrap();
    assert_eq!(symbol.symbol_index, 0);
    assert_eq!(symbol.ncp_reserved_cells, 48); // One start plusCRC,24QPSK cells each.
    assert_eq!(
        symbol.ncp_entries,
        [NcpPlanEntry::CodewordStart {
            codeword_id: 0,
            profile_id: 3,
            pointer: 0
        }]
    );
    assert_eq!(
        symbol.cells[1],
        PackedCell::Data {
            codeword_id: 0,
            codeword_cell: 0,
            label: 0x35
        }
    );
    assert_eq!(
        symbol.cells[50],
        PackedCell::Data {
            codeword_id: 0,
            codeword_cell: 48,
            label: 0x35
        }
    );
    assert_eq!(symbol.cells[98], PackedCell::NcpReserved { field_cell: 0 });
    assert_eq!(symbol.cells[0], PackedCell::ScatteredPilotPlaceholder);
    let next = packer.pack_symbol(&roles).unwrap();
    assert_eq!(next.ncp_entries, [NcpPlanEntry::Null]);
    assert_eq!(
        next.cells[1],
        PackedCell::Data {
            codeword_id: 0,
            codeword_cell: 49,
            label: 0x35
        }
    );
    assert_eq!(roles, original);
}

#[test]
fn multiple_starts_and_filler_reserve_their_own_entries_plus_crc() {
    let mut packer = SymbolPacker::new(settings(5000)).unwrap();
    packer.enqueue(vec![1; 2025]).unwrap();
    packer.enqueue(vec![2; 2025]).unwrap();
    let result = packer
        .pack_symbol(&vec![PreInterleavingRole::Data; 5000])
        .unwrap();
    assert_eq!(
        result.ncp_entries,
        [
            NcpPlanEntry::CodewordStart {
                codeword_id: 0,
                profile_id: 3,
                pointer: 0
            },
            NcpPlanEntry::CodewordStart {
                codeword_id: 1,
                profile_id: 3,
                pointer: 2025
            },
            NcpPlanEntry::ZeroLoaded { pointer: 4050 },
        ]
    );
    assert_eq!(result.ncp_reserved_cells, 96);
    assert_eq!(
        result
            .cells
            .iter()
            .filter(|cell| matches!(cell, PackedCell::ZeroLoadedFiller))
            .count(),
        854
    );
    assert_eq!(result.queued_codewords_after, 0);
    let summary = result.summary();
    assert_eq!(summary.data_cells, 4050);
    assert_eq!(summary.zero_loaded_filler_cells, 854);
    assert_eq!(summary.scattered_pilot_placeholders, 0);
    assert_eq!(summary.ncp_reserved_cells, 96);
    assert!(!summary.ncp_wire_encoded);
    let json = serde_json::to_value(summary).unwrap();
    assert_eq!(json["ncp_entries"][1]["pointer"], 2025);
    assert_eq!(json["ncp_wire_encoded"], false);
}

#[test]
fn tight_end_boundary_adds_null_and_carries_tail_instead_of_overfilling_ncp() {
    let mut packer = SymbolPacker::new(settings(2073)).unwrap();
    packer.enqueue(vec![7; 2025]).unwrap();
    let first = packer
        .pack_symbol(&vec![PreInterleavingRole::Data; 2073])
        .unwrap();
    assert_eq!(first.ncp_reserved_cells, 72);
    assert_eq!(first.ncp_entries.last(), Some(&NcpPlanEntry::Null));
    assert_eq!(
        first.cells[2000],
        PackedCell::Data {
            codeword_id: 0,
            codeword_cell: 2000,
            label: 7
        }
    );
    assert_eq!(first.queued_codewords_after, 1);
    let second = packer
        .pack_symbol(&vec![PreInterleavingRole::Data; 2073])
        .unwrap();
    assert_eq!(
        second.cells[0],
        PackedCell::Data {
            codeword_id: 0,
            codeword_cell: 2001,
            label: 7
        }
    );
    assert_eq!(
        second.ncp_entries,
        [NcpPlanEntry::ZeroLoaded { pointer: 24 }]
    );
    assert_eq!(second.queued_codewords_after, 0);
}

#[test]
fn ncp_modulations_empty_symbols_and_4k_limits_are_explicit() {
    for (modulation, cell_count) in [
        (NcpModulation::Qpsk, 48),
        (NcpModulation::Qam16, 24),
        (NcpModulation::Qam64, 16),
    ] {
        let mut config = settings(100);
        config.ncp_modulation = modulation;
        let mut packer = SymbolPacker::new(config).unwrap();
        let symbol = packer
            .pack_symbol(&[PreInterleavingRole::Data; 100])
            .unwrap();
        assert_eq!(symbol.ncp_reserved_cells, cell_count);
        assert_eq!(
            symbol.ncp_entries,
            [NcpPlanEntry::ZeroLoaded { pointer: 0 }]
        );
        assert_eq!(symbol.queued_codewords_after, 0);
    }
    let mut config = settings(3745);
    config.subcarrier_spacing = SubcarrierSpacing::Khz50;
    let mut packer = SymbolPacker::new(config).unwrap();
    for _ in 0..16 {
        packer.enqueue(vec![0; 2025]).unwrap();
    }
    let mut previous = 0;
    for _ in 0..12 {
        let symbol = packer
            .pack_symbol(&vec![PreInterleavingRole::Data; 3745])
            .unwrap();
        let starts = symbol
            .ncp_entries
            .iter()
            .filter(|entry| matches!(entry, NcpPlanEntry::CodewordStart { .. }))
            .count();
        assert!(previous + starts <= 10);
        previous = starts;
    }
    config.interleaver_cells = 3746;
    assert!(SymbolPacker::new(config).is_err());
}

#[test]
fn rejected_calls_preserve_pending_data_and_symbol_sequence() {
    let mut packer = SymbolPacker::new(settings(100)).unwrap();
    assert!(packer.enqueue(vec![]).is_err());
    assert!(packer.enqueue(vec![256; 2025]).is_err());
    assert_eq!(packer.enqueue(vec![1; 2025]).unwrap(), 0);
    assert!(packer
        .pack_symbol(&[PreInterleavingRole::Data; 99])
        .is_err());
    assert!(packer
        .pack_symbol(&[PreInterleavingRole::ScatteredPilotPlaceholder; 100])
        .is_err());
    assert!(packer
        .pack_symbol(&[PreInterleavingRole::ZeroBitLoaded; 100])
        .is_err());
    assert_eq!(packer.queued_codewords(), 1);
    let symbol = packer
        .pack_symbol(&[PreInterleavingRole::Data; 100])
        .unwrap();
    assert_eq!(symbol.symbol_index, 0);
    assert_eq!(
        symbol.cells[0],
        PackedCell::Data {
            codeword_id: 0,
            codeword_cell: 0,
            label: 1
        }
    );
    for expected in 1..16 {
        assert_eq!(packer.enqueue(vec![0; 2025]).unwrap(), expected);
    }
    assert!(packer.enqueue(vec![0; 2025]).is_err());
    let mut bad = settings(100);
    bad.data_profile_id = 16;
    assert!(SymbolPacker::new(bad).is_err());
    assert!(SymbolPacker::new(settings(47)).is_err());
    assert!(SymbolPacker::new(settings(7538)).is_err());
}
