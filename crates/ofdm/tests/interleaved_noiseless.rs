/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real NCP/payload recovery after cell interleaving and physical pilot association.
#![cfg(feature = "simulation")]
mod support;
use docsis_ofdm_fec::{
    signal_processing::{
        interleaving::{InterleaverDirection, SymbolInterleaver},
        modulation::{
            interleaving::BitInterleaver, map_symbol, qam_llrs, randomization::CellRandomizer,
        },
    },
    simulation::source::SeededSource,
    transmission::{
        layout::{ncp::PreInterleavingRole, PhysicalCarrierRole, PilotReference},
        ncp::NcpFieldCodec,
        packing::{NcpModulation, PackedCell, PackedSymbol, PackingSettings, SymbolPacker},
        SubcarrierSpacing,
    },
    types::IqPair,
};
use fec_codec::concatenated::{compare_payload, ConcatenatedCodec};

#[derive(Clone)]
enum Cell {
    PilotPlaceholder,
    Sample(IqPair),
}
fn transmit(
    symbol: PackedSymbol,
    control: &NcpFieldCodec,
    randomizer: &mut CellRandomizer,
) -> Vec<Option<Cell>> {
    let field = control.encode_packed(&symbol).unwrap();
    randomizer.begin_symbol(symbol.symbol_index).unwrap();
    symbol
        .cells
        .into_iter()
        .map(|cell| {
            Some(match cell {
                PackedCell::BitData { .. } => {
                    panic!("Legacy fixture unexpectedly selected variable-width packing")
                }
                PackedCell::Data { label, .. } => Cell::Sample(
                    map_symbol(256, randomizer.apply_data(256, label).unwrap()).unwrap(),
                ),
                PackedCell::NcpReserved { field_cell } => Cell::Sample(
                    map_symbol(
                        4,
                        randomizer.apply_ncp(4, field.labels[field_cell]).unwrap(),
                    )
                    .unwrap(),
                ),
                PackedCell::ZeroLoadedFiller => {
                    Cell::Sample(map_symbol(2, randomizer.zero_loaded_bpsk().unwrap()).unwrap())
                }
                PackedCell::ScatteredPilotPlaceholder => {
                    randomizer.skip_scattered_pilot().unwrap();
                    Cell::PilotPlaceholder
                }
            })
        })
        .collect()
}
struct Receiver {
    control: NcpFieldCodec,
    data_randomizer: CellRandomizer,
    ncp_randomizer: CellRandomizer,
    pending: Vec<f64>,
    recovered: usize,
    starts: usize,
    fec: ConcatenatedCodec,
    bits: BitInterleaver,
}
impl Receiver {
    fn new() -> Self {
        Self {
            control: NcpFieldCodec::new().unwrap(),
            data_randomizer: CellRandomizer::new(),
            ncp_randomizer: CellRandomizer::new(),
            pending: Vec::new(),
            recovered: 0,
            starts: 0,
            fec: ConcatenatedCodec::new().unwrap(),
            bits: BitInterleaver::new(256).unwrap(),
        }
    }
    fn receive(
        &mut self,
        index: u64,
        cells: &[Option<Cell>],
        roles: &[PreInterleavingRole],
        payloads: &[Vec<u8>],
    ) {
        self.data_randomizer.begin_symbol(index).unwrap();
        self.ncp_randomizer.begin_symbol(index).unwrap();
        let mut points = Vec::new();
        for (cell, role) in cells.iter().zip(roles) {
            match (cell, role) {
                (Some(Cell::PilotPlaceholder), PreInterleavingRole::ScatteredPilotPlaceholder) => {}
                (Some(Cell::Sample(point)), PreInterleavingRole::Data) => points.push(*point),
                _ => panic!("Missing/wrong restored cell after warm-up"),
            }
        }
        let mut ncp_soft: Vec<_> = points
            .iter()
            .map(|point| {
                self.ncp_randomizer
                    .descramble_ncp_llrs(4, &qam_llrs(4, *point, 0.01).unwrap())
                    .unwrap()
            })
            .collect();
        ncp_soft.reverse();
        let field = self
            .control
            .decode_field(&ncp_soft, NcpModulation::Qpsk, 100)
            .unwrap();
        let data_count = points.len() - field.consumed_cells;
        let spans = field.data_spans(data_count).unwrap();
        let mut is_payload = vec![false; points.len()];
        for span in &spans {
            if !span.zero_loaded {
                is_payload[span.start..span.end_exclusive].fill(true);
            }
        }
        let data_soft: Vec<_> = points
            .iter()
            .zip(is_payload)
            .map(|(point, payload)| {
                if payload {
                    Some(
                        self.data_randomizer
                            .descramble_data_llrs(256, &qam_llrs(256, *point, 0.01).unwrap())
                            .unwrap(),
                    )
                } else {
                    self.data_randomizer.zero_loaded_bpsk().unwrap();
                    None
                }
            })
            .collect();
        assert_eq!(self.data_randomizer.state(), self.ncp_randomizer.state());
        for span in spans {
            if span.zero_loaded {
                assert!(self.pending.is_empty());
                continue;
            }
            if span.starts_codeword {
                assert!(self.pending.is_empty());
                assert_eq!(span.profile_id, Some(0));
                self.starts += 1;
            } else {
                assert!(!self.pending.is_empty());
            }
            for cell in &data_soft[span.start..span.end_exclusive] {
                self.pending.extend_from_slice(cell.as_ref().unwrap());
                assert!(self.pending.len() <= 16200);
                if self.pending.len() == 16200 {
                    let decoded = self
                        .fec
                        .decode(&self.bits.decode_llrs(&self.pending).unwrap(), 30)
                        .unwrap();
                    assert!(decoded.ldpc.success && decoded.bch.success);
                    assert_eq!(
                        compare_payload(&payloads[self.recovered], &decoded)
                            .unwrap()
                            .bit_errors,
                        0
                    );
                    self.pending.clear();
                    self.recovered += 1;
                }
            }
        }
    }
}
#[test]
fn both_spacings_recover_real_ncp_and_payloads_with_physical_pilots_and_drain() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let layout = support::layout(spacing);
        let depth = usize::from(spacing.max_time_interleaver_depth());
        let reference = PilotReference::new(&layout, depth).unwrap();
        let settings = reference.settings();
        let mut transmitter =
            SymbolInterleaver::new(settings, InterleaverDirection::Transmit).unwrap();
        let mut inverse = SymbolInterleaver::new(settings, InterleaverDirection::Receive).unwrap();
        let mut packer = SymbolPacker::new(PackingSettings {
            subcarrier_spacing: spacing,
            interleaver_cells: settings.cells(),
            ncp_modulation: NcpModulation::Qpsk,
            data_profile_id: 0,
        })
        .unwrap();
        let mut source = SeededSource::new(123);
        let payloads: Vec<_> = (0..4).map(|_| source.next_payload()).collect();
        let encoder = ConcatenatedCodec::new().unwrap();
        let bits = BitInterleaver::new(256).unwrap();
        let control = NcpFieldCodec::new().unwrap();
        let mut randomizer = CellRandomizer::new();
        let mut receiver = Receiver::new();
        //160source symbols cross the128-symbol pilot/randomizer boundary; payloads are
        // spaced out so filler, continuation, startup and drain are all exercised.
        for symbol_index in 0..160 + settings.pair_delay_symbols() {
            let input = if symbol_index < 160 {
                if symbol_index < 128 && symbol_index % 32 == 0 {
                    packer
                        .enqueue(
                            bits.encode_labels(
                                &encoder
                                    .encode(&payloads[symbol_index / 32])
                                    .unwrap()
                                    .ldpc_codeword,
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }
                transmit(
                    packer
                        .pack_symbol(reference.roles(symbol_index as u64))
                        .unwrap(),
                    &control,
                    &mut randomizer,
                )
            } else {
                vec![None; settings.cells()]
            };
            let on_channel = transmitter
                .process_symbol(symbol_index as u64, &input)
                .unwrap();
            let physical = layout.symbol_roles(symbol_index as u64);
            // Receiver gets I/Q plus known physical pilot locations. No diagnostic
            // data IDs, codeword offsets or NCP-reserved positions cross this boundary.
            let received: Vec<_> = on_channel
                .into_iter()
                .zip(reference.physical_bins())
                .map(|(cell, bin)| match cell {
                    Some(Cell::PilotPlaceholder) => {
                        assert_eq!(
                            physical[usize::from(*bin)],
                            PhysicalCarrierRole::ScatteredPilot
                        );
                        Some(Cell::PilotPlaceholder)
                    }
                    Some(Cell::Sample(point)) => {
                        assert_eq!(physical[usize::from(*bin)], PhysicalCarrierRole::Data);
                        Some(Cell::Sample(point))
                    }
                    None => None,
                })
                .collect();
            let restored = inverse
                .process_symbol(symbol_index as u64, &received)
                .unwrap();
            if let Some(source_index) = symbol_index.checked_sub(settings.pair_delay_symbols()) {
                receiver.receive(
                    source_index as u64,
                    &restored,
                    reference.roles(source_index as u64),
                    &payloads,
                );
            } else {
                assert!(restored.iter().all(Option::is_none));
            }
        }
        assert_eq!(packer.queued_codewords(), 0);
        assert!(receiver.pending.is_empty());
        assert_eq!(receiver.starts, 4);
        assert_eq!(receiver.recovered, 4);
    }
}
