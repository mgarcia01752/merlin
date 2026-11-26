/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact bounded actual-stage association without source truth or interpolation.
use super::{QamPoint, Tap};
use crate::{
    error::{invalid, limit},
    simulation::BatchReport,
    PhyError, PhyResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Copied-pair ceiling, independent of single-point and reference-geometry budgets.
pub const MAX_QAM_PAIR_POINTS: usize = 4096;

/// Two original retained stages with a loading selection and independent pair limit.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QamPairRequest {
    /// Original left stage; output preserves its retention order.
    pub left: Tap,
    /// Original right stage; no channel normalization is inferred.
    pub right: Tap,
    /// Original payload loading; zero selects pilots and None selects all.
    pub loading: Option<u8>,
    /// Positive copied-pair budget, at most MAX_QAM_PAIR_POINTS.
    pub maximum_pairs: usize,
}

/// Unchanged actually observed points at exactly the same original physical coordinate.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct QamPair {
    /// Original actual left point; amplitude remains at its own stage plane.
    pub left: QamPoint,
    /// Original actual right point, without interpolation or truth substitution.
    pub right: QamPoint,
}

/// Whole retained-history association counts and bounded original pairs.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct QamPairSnapshot {
    /// Original left actual stage.
    pub left: Tap,
    /// Original right actual stage.
    pub right: Tap,
    /// Original optional payload-loading selection.
    pub loading: Option<u8>,
    /// Selected left retained points before display capping.
    pub left_selected: usize,
    /// Selected right retained points before display capping.
    pub right_selected: usize,
    /// Exact original-coordinate matches over selected retained histories.
    pub matched_total: usize,
    /// Selected left retained points without a selected right partner.
    pub unpaired_left: usize,
    /// Selected right retained points without a selected left partner.
    pub unpaired_right: usize,
    /// True when the independent copy budget omits further retained matches.
    pub capped: bool,
    /// Actual original pairs in left retention order; stage planes stay distinct.
    pub pairs: Vec<QamPair>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    run: u64,
    generation: u64,
    grant: u64,
    frame: u64,
    symbol: usize,
    carrier: usize,
}

impl Default for QamPairRequest {
    /// Selects TX-grid versus recovered cells with all loadings and a finite pair cap.
    /// # Returns
    /// Read-only original-coordinate association; no scientific state advances.
    fn default() -> Self {
        Self {
            left: Tap::TxGrid,
            right: Tap::RxEqualized,
            loading: None,
            maximum_pairs: MAX_QAM_PAIR_POINTS,
        }
    }
}

impl BatchReport {
    /// Associates two actual retained stages using complete original physical identities.
    /// # Arguments
    /// * `request` - Actual left/right taps, optional loading and positive copied-pair cap.
    /// # Returns
    /// Owned unchanged points, whole retained selected/matched/unpaired counts and explicit capping.
    /// Empty or partial retained histories can have no pairs; counts do not establish FEC/source
    /// coverage. No channel correction, scientific replay or source-truth matching occurs.
    /// # Errors
    /// Unretained taps are unavailable; invalid budgets and duplicate/nonfinite coordinates reject.
    pub fn qam_pairing(&self, request: QamPairRequest) -> PhyResult<QamPairSnapshot> {
        if !(1..=MAX_QAM_PAIR_POINTS).contains(&request.maximum_pairs) {
            return Err(limit("qam_pairing.maximum_pairs"));
        }
        let left = self
            .observations
            .trace(request.left)
            .ok_or(PhyError::Unsupported {
                field: "qam_pairing.left_unretained",
            })?;
        let right = self
            .observations
            .trace(request.right)
            .ok_or(PhyError::Unsupported {
                field: "qam_pairing.right_unretained",
            })?;
        let selected = |point: &&QamPoint| {
            request
                .loading
                .is_none_or(|loading| point.loading == loading)
        };
        let mut right_index = BTreeMap::new();
        for point in right.points.iter().filter(selected) {
            validate(point)?;
            if right_index.insert(coordinate(point), point).is_some() {
                return Err(invalid("qam_pairing.duplicate_coordinate"));
            }
        }
        let mut left_keys = BTreeSet::new();
        let mut matched_total = 0;
        let mut pairs = Vec::new();
        for point in left.points.iter().filter(selected) {
            validate(point)?;
            let key = coordinate(point);
            if !left_keys.insert(key) {
                return Err(invalid("qam_pairing.duplicate_coordinate"));
            }
            if let Some(partner) = right_index.get(&key) {
                matched_total += 1;
                if pairs.len() < request.maximum_pairs {
                    pairs.push(QamPair {
                        left: point.clone(),
                        right: (*partner).clone(),
                    });
                }
            }
        }
        Ok(QamPairSnapshot {
            left: request.left,
            right: request.right,
            loading: request.loading,
            left_selected: left_keys.len(),
            right_selected: right_index.len(),
            matched_total,
            unpaired_left: left_keys.len() - matched_total,
            unpaired_right: right_index.len() - matched_total,
            capped: pairs.len() < matched_total,
            pairs,
        })
    }
}

fn coordinate(point: &QamPoint) -> Coordinate {
    Coordinate {
        run: point.identity.run,
        generation: point.identity.generation,
        grant: point.identity.grant,
        frame: point.frame,
        symbol: point.symbol,
        carrier: point.carrier,
    }
}

fn validate(point: &QamPoint) -> PhyResult<()> {
    point.identity.validate()?;
    if !point.value.is_finite() {
        return Err(invalid("qam_pairing.observation"));
    }
    Ok(())
}
