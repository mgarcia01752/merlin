/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independently transcribed time columns, PHY I20 Tables48/49 PDF151–152.
use docsis_ofdm_fec::transmission::{plc::preamble, SubcarrierSpacing};
#[test]
fn all_preamble_columns_match_both_rendered_tables_and_repeat_phase() {
    for (spacing, columns, width) in [
        (
            SubcarrierSpacing::Khz50,
            [0x4c, 0x3a, 0xa9, 0x22, 0xe8, 0xd8, 0x5b, 0xaf],
            8,
        ),
        (
            SubcarrierSpacing::Khz25,
            [
                0x897c, 0x6bfa, 0x63e5, 0xb6d6, 0x0c8e, 0xd258, 0x1b44, 0x34d1,
            ],
            16,
        ),
    ] {
        for (column, bits) in columns.iter().enumerate() {
            for cycle in [0, 1, 100] {
                let symbol =
                    preamble::generate(spacing, cycle * 128 + 120 + column as u64).unwrap();
                assert_eq!(symbol.bits.len(), width);
                for carrier in 0..width {
                    let expected = ((bits >> (width - 1 - carrier)) & 1) as u8;
                    assert_eq!(symbol.bits[carrier], expected);
                    let sign = if expected == 0 { 1.0 } else { -1.0 };
                    assert_eq!(symbol.iq[carrier], [sign, 0.0]);
                }
            }
        }
        for phase in 0..120 {
            assert!(preamble::generate(spacing, phase).is_err());
        }
    }
}
