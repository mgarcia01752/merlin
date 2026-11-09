/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independently transcribed PHY Tables24–37 / ETSI Table10(a), not round trips alone.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::signal_processing::modulation::{
    interleaving::{shortened::ShortenedBitOrder, BitInterleaver},
    map_symbol, qam_llrs,
    randomization::CellRandomizer,
};
use fec_codec::{
    concatenated::ConcatenatedCodec,
    shortening::{compare_payload, ShortenedCodec, Shortening},
};
struct Mode {
    order: u16,
    rows: usize,
    shifts: &'static [usize],
    demux: &'static [usize],
    cells: usize,
}
fn modes() -> Vec<Mode> {
    vec![
        Mode {
            order: 16,
            rows: 2025,
            shifts: &[0, 0, 0, 1, 7, 20, 20, 21],
            demux: &[7, 1, 4, 2, 5, 3, 6, 0],
            cells: 4050,
        },
        Mode {
            order: 64,
            rows: 1350,
            shifts: &[0, 0, 0, 2, 2, 2, 3, 3, 3, 6, 7, 7],
            demux: &[11, 7, 3, 10, 6, 2, 9, 5, 1, 8, 4, 0],
            cells: 2700,
        },
        Mode {
            order: 128,
            rows: 2315,
            shifts: &[0, 1, 2, 2, 2, 3, 3],
            demux: &[6, 5, 4, 1, 2, 3, 0],
            cells: 2315,
        },
        Mode {
            order: 256,
            rows: 2025,
            shifts: &[0, 0, 0, 1, 7, 20, 20, 21],
            demux: &[7, 3, 1, 5, 2, 6, 4, 0],
            cells: 2025,
        },
        Mode {
            order: 512,
            rows: 1800,
            shifts: &[0, 1, 2, 3, 5, 6, 7, 9, 11],
            demux: &[8, 7, 6, 1, 2, 3, 4, 5, 0],
            cells: 1800,
        },
        Mode {
            order: 1024,
            rows: 810,
            shifts: &[0, 0, 0, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 7, 7, 7, 7, 8, 8, 10],
            demux: &[
                8, 3, 7, 10, 19, 4, 9, 5, 17, 6, 14, 11, 2, 18, 16, 15, 0, 1, 13, 12,
            ],
            cells: 1620,
        },
        Mode {
            order: 2048,
            rows: 1473,
            shifts: &[0, 0, 0, 0, 0, 3, 3, 4, 4, 4, 4],
            demux: &[10, 9, 8, 7, 2, 3, 4, 5, 6, 1, 0],
            cells: 1473,
        },
        Mode {
            order: 4096,
            rows: 675,
            shifts: &[
                0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 7, 9, 9, 9, 10, 10, 10, 10, 10, 11,
            ],
            demux: &[
                10, 15, 4, 19, 21, 16, 23, 18, 11, 14, 22, 5, 6, 17, 13, 20, 1, 3, 9, 2, 7, 8, 12,
                0,
            ],
            cells: 1350,
        },
        Mode {
            order: 8192,
            rows: 1247,
            shifts: &[0, 0, 0, 0, 0, 0, 2, 3, 5, 8, 8, 9, 9],
            demux: &[12, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 0],
            cells: 1247,
        },
        Mode {
            order: 16384,
            rows: 1158,
            shifts: &[0, 0, 2, 2, 2, 2, 2, 4, 4, 6, 6, 8, 9, 9],
            demux: &[13, 12, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 0],
            cells: 1158,
        },
    ]
}
#[test]
fn all_full_uniform_maps_match_independent_matrices_and_padding_positions() {
    for mode in modes() {
        let columns = mode.shifts.len();
        let mut expected = vec![usize::MAX; mode.rows * columns];
        for column in 0..columns {
            for source_row in 0..mode.rows {
                let position = column * mode.rows + source_row;
                let original = if (14400..16200).contains(&position) {
                    let parity = position - 14400;
                    14400 + 5 * (parity % 360) + parity / 360
                } else {
                    position
                };
                let row = (source_row + mode.shifts[column]) % mode.rows;
                expected[row * columns + mode.demux[column]] = original;
            }
        }
        let interleaver = BitInterleaver::new(mode.order).unwrap();
        assert_eq!(
            interleaver.output_to_codeword(),
            expected,
            "order={}",
            mode.order
        );
        let mut sorted = expected.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..mode.rows * columns).collect::<Vec<_>>());
        let words = interleaver.encode_labels(&vec![1; 16200]).unwrap();
        assert_eq!(words.len(), mode.cells);
        let width = mode.order.ilog2() as usize;
        for (cell, label) in words.iter().enumerate() {
            for bit in 0..width {
                let index = expected[cell * width + bit];
                assert_eq!((label >> (width - 1 - bit)) & 1, u16::from(index < 16200));
            }
        }
        let soft: Vec<_> = expected.iter().map(|index| *index as f64 + 0.25).collect();
        assert_eq!(
            interleaver.decode_llrs(&soft).unwrap(),
            (0..16200)
                .map(|index| index as f64 + 0.25)
                .collect::<Vec<_>>()
        );
    }
}
#[test]
fn shortened_holes_match_column_writer_and_never_become_transmitted_zeros() {
    let shifts = [
        0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 7, 9, 9, 9, 10, 10, 10, 10, 10, 11,
    ];
    for payload_bits in [1, 7, 8, 1032, 6136, 14216, 14231] {
        let geometry = Shortening::new(payload_bits).unwrap();
        let count = payload_bits + 1968;
        let rows = count.div_ceil(24);
        let mut matrix = vec![vec![None; rows]; 24];
        for (column, values) in matrix.iter_mut().enumerate() {
            for source_row in 0..rows {
                let position = column * rows + source_row;
                if position < count {
                    let original = if position < payload_bits + 168 {
                        position
                    } else {
                        let parity = position - payload_bits - 168;
                        payload_bits + 168 + 5 * (parity % 360) + parity / 360
                    };
                    values[(source_row + shifts[column]) % rows] = Some(original);
                }
            }
        }
        let expected: Vec<_> = (0..rows)
            .flat_map(|row| matrix.iter().filter_map(move |column| column[row]))
            .collect();
        let order = ShortenedBitOrder::new(geometry).unwrap();
        assert_eq!(order.permutation(), expected);
        let mut sorted = expected;
        sorted.sort_unstable();
        assert_eq!(sorted, (0..count).collect::<Vec<_>>());
        let soft: Vec<_> = order
            .permutation()
            .iter()
            .map(|index| *index as f64 - 99.5)
            .collect();
        assert_eq!(
            order.decode(&soft).unwrap(),
            (0..count)
                .map(|index| index as f64 - 99.5)
                .collect::<Vec<_>>()
        );
        assert!(order.encode(&vec![2; count]).is_err());
        assert!(order.decode(&vec![f64::NAN; count]).is_err());
    }
    assert!(ShortenedBitOrder::new(Shortening::new(14232).unwrap()).is_err());
}
#[test]
fn every_uniform_order_passes_actual_randomized_soft_fec_loop_without_errors() {
    let payload: Vec<_> = (0..14232)
        .map(|index| u8::from((index * 17 + index / 11) % 19 < 9))
        .collect();
    let mut codec = ConcatenatedCodec::new().unwrap();
    let encoded = codec.encode(&payload).unwrap();
    for mode in modes() {
        let bits = BitInterleaver::new(mode.order).unwrap();
        let labels = bits.encode_labels(&encoded.ldpc_codeword).unwrap();
        let soft = soft_cells(mode.order, &labels);
        let decoded = codec.decode(&bits.decode_llrs(&soft).unwrap(), 30).unwrap();
        assert!(
            decoded.ldpc.success && decoded.bch.success,
            "order={}",
            mode.order
        );
        assert_eq!(&decoded.bch.codeword[..14232], payload);
    }
}
fn soft_cells(order: u16, labels: &[u16]) -> Vec<f64> {
    let mut tx = CellRandomizer::new();
    let mut rx = CellRandomizer::new();
    tx.begin_symbol(0).unwrap();
    rx.begin_symbol(0).unwrap();
    labels
        .iter()
        .flat_map(|label| {
            let point = map_symbol(order, tx.apply_data(order, *label).unwrap()).unwrap();
            rx.descramble_data_llrs(order, &qam_llrs(order, point, 0.01).unwrap())
                .unwrap()
        })
        .collect()
}
#[test]
fn shortened_square_and_cross_constellations_recover_after_final_cell_padding() {
    for payload_bits in [7, 1032, 6136] {
        let geometry = Shortening::new(payload_bits).unwrap();
        let bit_order = ShortenedBitOrder::new(geometry).unwrap();
        let mut codec = ShortenedCodec::new(geometry).unwrap();
        let payload: Vec<_> = (0..payload_bits).map(|index| (index % 2) as u8).collect();
        let encoded = codec.encode(&payload).unwrap();
        let bits = bit_order.encode(&encoded.transmitted).unwrap();
        for order in [16u16, 128, 256, 16384] {
            let width = order.ilog2() as usize;
            let labels: Vec<_> = bits
                .chunks(width)
                .map(|chunk| {
                    chunk
                        .iter()
                        .fold(0, |label, bit| (label << 1) | u16::from(*bit))
                        << (width - chunk.len())
                })
                .collect();
            let soft = soft_cells(order, &labels);
            let decoded = codec
                .decode(&bit_order.decode(&soft[..bits.len()]).unwrap(), 30)
                .unwrap();
            assert!(decoded.success(), "K={payload_bits},order={order}");
            assert_eq!(compare_payload(&payload, &decoded).unwrap().bit_errors, 0);
        }
    }
}
