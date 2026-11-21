/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Frozen source-derived fingerprints and sparse parity, PHY I20 p57.
//! Provenance: docs/development/features/unified-backend/references/ldpc-source-audit-validation.md.
//! Generated from independent original-page transcription, not production.
use super::{ImpulseFixture, SourceFixture};
use crate::upstream_ldpc::DocsisLdpcCode;

pub(super) const FAMILIES: &[SourceFixture] = &[
    SourceFixture {
        code: DocsisLdpcCode::UpstreamLong,
        codeword_bits: 16200,
        information_bits: 14400,
        lift: 360,
        row_fingerprints: [
            8669660463025419690,
            600768246014786622,
            5432302155161054727,
            15743779561196277958,
            17079598747890111335,
        ],
        expanded_fingerprint: 18385344434388079461,
        edge_count: 60840,
        impulses: &[
            ImpulseFixture {
                information_one: 0,
                parity_ones: &[
                    50, 420, 638, 800, 942, 1018, 1147, 1289, 1365, 1468, 1547, 1610, 1752,
                ],
            },
            ImpulseFixture {
                information_one: 359,
                parity_ones: &[
                    49, 419, 637, 799, 941, 1017, 1146, 1288, 1364, 1467, 1546, 1609, 1751,
                ],
            },
            ImpulseFixture {
                information_one: 360,
                parity_ones: &[
                    232, 460, 579, 741, 797, 982, 1088, 1144, 1329, 1527, 1551, 1607, 1792,
                ],
            },
            ImpulseFixture {
                information_one: 7200,
                parity_ones: &[697, 859, 1068, 1206, 1312, 1415, 1518, 1669, 1775, 1788],
            },
            ImpulseFixture {
                information_one: 7560,
                parity_ones: &[
                    338, 566, 613, 728, 775, 1076, 1122, 1239, 1423, 1435, 1526, 1538, 1585, 1702,
                ],
            },
            ImpulseFixture {
                information_one: 14040,
                parity_ones: &[
                    343, 571, 665, 729, 733, 827, 1080, 1174, 1436, 1539, 1543, 1637, 1742,
                ],
            },
            ImpulseFixture {
                information_one: 14399,
                parity_ones: &[
                    342, 570, 664, 728, 732, 826, 1173, 1435, 1439, 1538, 1542, 1636, 1741,
                ],
            },
        ],
    },
    SourceFixture {
        code: DocsisLdpcCode::UpstreamMedium,
        codeword_bits: 5940,
        information_bits: 5040,
        lift: 180,
        row_fingerprints: [
            5061699501546172184,
            17783352157984936883,
            11084649173212144598,
            14907946887328526910,
            10071784143860625809,
        ],
        expanded_fingerprint: 4880007973761502517,
        edge_count: 23580,
        impulses: &[
            ImpulseFixture {
                information_one: 0,
                parity_ones: &[
                    60, 297, 310, 371, 517, 530, 682, 695, 700, 716, 722, 738, 867, 884, 897,
                ],
            },
            ImpulseFixture {
                information_one: 179,
                parity_ones: &[
                    59, 296, 309, 370, 516, 529, 681, 694, 699, 715, 721, 737, 866, 883, 896,
                ],
            },
            ImpulseFixture {
                information_one: 180,
                parity_ones: &[
                    44, 294, 359, 399, 423, 514, 564, 568, 588, 679, 760, 766, 770, 790, 881,
                ],
            },
            ImpulseFixture {
                information_one: 2880,
                parity_ones: &[
                    84, 300, 334, 374, 451, 520, 616, 685, 719, 737, 741, 818, 887,
                ],
            },
            ImpulseFixture {
                information_one: 3060,
                parity_ones: &[118, 180, 188, 384, 400, 408, 549, 565, 573, 767, 775],
            },
            ImpulseFixture {
                information_one: 3240,
                parity_ones: &[286, 388, 506, 553, 661, 671, 755, 761, 863, 873],
            },
            ImpulseFixture {
                information_one: 4860,
                parity_ones: &[267, 487, 652, 710, 732, 758, 854],
            },
            ImpulseFixture {
                information_one: 5039,
                parity_ones: &[266, 486, 651, 709, 731, 757, 853],
            },
        ],
    },
    SourceFixture {
        code: DocsisLdpcCode::UpstreamShort,
        codeword_bits: 1120,
        information_bits: 840,
        lift: 56,
        row_fingerprints: [
            4548053706427846239,
            16584072365359397402,
            17053707282913827342,
            14736954483006763422,
            9372361138265291038,
        ],
        expanded_fingerprint: 3428284507234005429,
        edge_count: 4424,
        impulses: &[
            ImpulseFixture {
                information_one: 0,
                parity_ones: &[
                    5, 76, 88, 137, 139, 149, 174, 175, 218, 220, 245, 261, 263, 273, 274,
                ],
            },
            ImpulseFixture {
                information_one: 55,
                parity_ones: &[
                    4, 75, 87, 136, 138, 148, 173, 174, 217, 219, 244, 260, 262, 272, 273,
                ],
            },
            ImpulseFixture {
                information_one: 56,
                parity_ones: &[
                    52, 79, 97, 123, 140, 158, 180, 183, 204, 221, 226, 247, 264, 275, 279,
                ],
            },
            ImpulseFixture {
                information_one: 784,
                parity_ones: &[20, 103, 114, 164, 189, 195, 198, 232, 241],
            },
            ImpulseFixture {
                information_one: 839,
                parity_ones: &[19, 102, 113, 163, 188, 194, 197, 231, 240],
            },
        ],
    },
];
