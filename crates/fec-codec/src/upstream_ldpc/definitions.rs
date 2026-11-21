/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Implementation constants transcribed from PHYv3.1 I20 §7.4.3.2, PDF p57.
//! A -1 is the all-zero block; other entries shift identity columns to the right.
use crate::upstream_ldpc::LdpcError;
use ldpc_toolbox::sparse::SparseMatrix;
use serde::{Deserialize, Serialize};

/// Upstream data code identity; none denotes MERLIN's downstream DVB matrix.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsisLdpcCode {
    /// Rate 8/9, N=16200 and K=14400.
    UpstreamLong,
    /// Rate 28/33, N=5940 and K=5040.
    UpstreamMedium,
    /// Rate 3/4, N=1120 and K=840.
    UpstreamShort,
}

/// Immutable mother-code dimensions from Table 8 and the p57 matrices.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct LdpcParameters {
    /// Mother information count before shortening.
    pub information_bits: usize,
    /// Complete mother codeword count, information followed by parity.
    pub codeword_bits: usize,
    /// Circulant lifting factor in bits.
    pub lifting_factor: usize,
}

impl DocsisLdpcCode {
    /// Returns normative dimensions without allocation or mutable state.
    ///
    /// # Returns
    /// Mother information/codeword counts and circulant lift, not a grant policy.
    pub fn parameters(self) -> LdpcParameters {
        match self {
            Self::UpstreamLong => LdpcParameters {
                information_bits: 14400,
                codeword_bits: 16200,
                lifting_factor: 360,
            },
            Self::UpstreamMedium => LdpcParameters {
                information_bits: 5040,
                codeword_bits: 5940,
                lifting_factor: 180,
            },
            Self::UpstreamShort => LdpcParameters {
                information_bits: 840,
                codeword_bits: 1120,
                lifting_factor: 56,
            },
        }
    }

    pub(crate) fn shifts(self) -> Vec<&'static [i16]> {
        match self {
            Self::UpstreamLong => LONG.iter().map(|row| row.as_slice()).collect(),
            Self::UpstreamMedium => MEDIUM.iter().map(|row| row.as_slice()).collect(),
            Self::UpstreamShort => SHORT.iter().map(|row| row.as_slice()).collect(),
        }
    }

    pub(crate) fn matrix(self) -> Result<SparseMatrix, LdpcError> {
        let parameters = self.parameters();
        let lift = parameters.lifting_factor;
        let mut matrix = SparseMatrix::new(
            parameters.codeword_bits - parameters.information_bits,
            parameters.codeword_bits,
        );
        for (block_row, shifts) in self.shifts().iter().enumerate() {
            for (block_column, &shift) in shifts.iter().enumerate() {
                if shift == -1 {
                    continue;
                }
                if shift < 0 || shift as usize >= lift {
                    return Err(LdpcError::MatrixDefinition);
                }
                for row in 0..lift {
                    matrix.insert(
                        block_row * lift + row,
                        block_column * lift + (row + shift as usize) % lift,
                    );
                }
            }
        }
        Ok(matrix)
    }
}

// Source: CM-SP-PHYv3.1-I20-230419 §7.4.3.2, PDF p57.
// These upstream matrices are not the DVB-S2/C2 R8_9short matrix.
const LONG: [[i16; 45]; 5] = [
    [
        93, 271, -1, 83, 26, 208, 245, 200, -1, 175, 331, 17, 86, -1, 337, -1, 238, 81, -1, 307,
        -1, 165, -1, 47, 76, 73, 150, 349, 139, 331, 118, 345, 27, 294, -1, 145, 279, 97, 106, 160,
        143, -1, -1, -1, -1,
    ],
    [
        274, 115, 329, 338, 124, -1, 293, -1, 69, 64, 342, -1, 88, 139, -1, 137, 212, -1, 157, 195,
        357, 81, 194, 1, 159, 56, 72, 126, 277, 156, 32, 111, 175, -1, 306, 224, -1, 206, -1, 29,
        106, 334, -1, -1, -1,
    ],
    [
        134, 355, 175, 24, 253, 242, -1, 187, 94, 26, 87, 302, -1, 191, 323, 22, -1, 245, 294, 240,
        84, 76, 342, 345, 174, 269, 329, -1, 214, -1, -1, -1, -1, 218, 104, 40, 197, 73, 229, 63,
        -1, 270, 72, -1, -1,
    ],
    [
        -1, -1, 184, 70, 247, 14, 22, 7, 285, 54, -1, 352, 26, 108, 10, 298, 123, 139, 117, -1,
        336, 49, 202, 359, 342, -1, 224, 106, -1, 273, 177, 245, 98, 355, 178, 176, 147, -1, 280,
        -1, -1, -1, 221, 208, -1,
    ],
    [
        253, 273, 90, -1, -1, 151, 311, 320, 339, -1, 295, 148, 48, 91, 62, 100, 232, 146, 200,
        135, 12, -1, 179, -1, -1, 232, -1, 21, 331, 313, 349, 34, 97, 187, 38, -1, 235, 52, 170,
        58, -1, -1, -1, 257, 0,
    ],
];

const MEDIUM: [[i16; 33]; 5] = [
    [
        142, 158, 113, 124, 92, 44, 93, 70, 172, 3, 25, 44, 141, 160, 50, 45, 118, 84, -1, 64, 66,
        97, 1, 115, 8, 108, -1, -1, 22, -1, -1, -1, -1,
    ],
    [
        54, 172, 145, 28, 55, 19, 159, 22, 96, 12, 85, -1, 128, 5, 158, 120, 51, 171, 65, 141, -1,
        42, 83, 7, -1, 39, 121, 84, 101, 171, -1, -1, -1,
    ],
    [
        63, 11, 112, 114, 61, 123, 72, 55, 114, 20, 53, 114, 42, 33, 4, 66, 163, 50, 46, 17, 175,
        -1, -1, 92, -1, 41, 138, -1, -1, 34, 74, -1, -1,
    ],
    [
        28, 160, 102, 44, 8, 84, 126, 9, 169, 174, 147, 24, 145, -1, 26, -1, -1, -1, 67, 82, 4,
        177, 151, 131, 139, 117, 36, 18, -1, -1, 23, 8, -1,
    ],
    [
        52, 159, 75, 74, 46, 71, 42, 11, 108, 153, -1, 72, -1, 163, -1, 9, 2, 168, 158, -1, 1, 49,
        89, 63, 179, 10, 75, 161, -1, -1, -1, 177, 19,
    ],
];

const SHORT: [[i16; 20]; 5] = [
    [
        5, 14, 12, 1, 2, 37, 45, 26, 24, 0, 3, -1, 34, 7, 46, 10, -1, -1, -1, -1,
    ],
    [
        0, 35, 1, 26, 0, 10, 16, 16, 34, 4, 2, 23, 0, 51, -1, 49, 20, -1, -1, -1,
    ],
    [
        12, 28, 22, 46, 3, 16, 51, 2, 25, 29, 19, 18, 52, -1, 37, -1, 34, 39, -1, -1,
    ],
    [
        0, 51, 16, 31, 13, 39, 27, 33, 8, 27, 53, 13, -1, 52, 33, -1, -1, 38, 7, -1,
    ],
    [
        36, 6, 3, 51, 4, 19, 4, 45, 48, 9, -1, 11, 22, 23, 43, -1, -1, -1, 14, 1,
    ],
];
