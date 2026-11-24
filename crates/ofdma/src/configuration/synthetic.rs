/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Explicit synthetic input JSON candidates; never passed off as decoded captures.
use crate::api::*;
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};

/// Complete synthetic raw bank candidate, normalized once by the shared Rust owner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BankDocument {
    /// Explicit nonzero coefficient generation.
    pub generation: u64,
    /// Full physical N raw I/Q pairs; excluded entries must be zero.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 4096>")]
    pub coefficients: Vec<[f64; 2]>,
}
impl BankDocument {
    fn materialize(&self, us: &UsConfiguration) -> ApiResult<PreEqBank> {
        Ok(PreEqBank::normalized(
            us,
            self.coefficients
                .iter()
                .map(|z| Iq { re: z[0], im: z[1] })
                .collect(),
            self.generation,
        )?)
    }
}
/// Explicit event operation; original synthetic factors are retained unchanged.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum UpdateDocument {
    /// Complete snapshot replacement; no implicit multiplication with prior bank.
    Snapshot {
        /// Complete raw bank candidate.
        bank: BankDocument,
    },
    /// Replace addressed values in an explicit corresponding-probe base.
    Initialize {
        /// Base bank/event ID.
        base_bank: u64,
        /// Explicit probe reference.
        probe: u64,
        /// Physical index and raw I/Q pair.
        #[serde(
            deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 4096>"
        )]
        values: Vec<(usize, [f64; 2])>,
    },
    /// Multiply addressed values in the explicit corresponding-probe base.
    Adjust {
        /// Base bank/event ID.
        base_bank: u64,
        /// Explicit probe reference.
        probe: u64,
        /// Physical index and complex multiplicative factor.
        #[serde(
            deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 4096>"
        )]
        values: Vec<(usize, [f64; 2])>,
    },
}
/// Bounded source-preserving synthetic activation record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventDocument {
    /// Nonzero event/effective-bank ID.
    pub id: u64,
    /// Physical-symbol offset in each explicit repeated grant.
    pub symbol: usize,
    /// Original synthetic candidate operation, never a guessed capture opcode.
    pub operation: UpdateDocument,
}
/// Exact synthetic bank/probe lineage and finite replay, independent of RxMER.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreEqDocument {
    /// Nonzero immutable initial-bank identity.
    pub initial_id: u64,
    /// None means explicit normative unity defaults; Some is a complete raw bank.
    pub initial: Option<BankDocument>,
    /// Strictly increasing bounded activation records after symbol zero.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 255>")]
    pub events: Vec<EventDocument>,
    /// Explicit synthetic corresponding-probe associations.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 256>")]
    pub probes: Vec<ProbeDocument>,
}
/// Explicit synthetic probe identity and the applied bank used for that probe.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeDocument {
    /// Nonzero unique probe identity.
    pub id: u64,
    /// Initial/event bank identity, retained independently from latest event.
    pub bank_id: u64,
}
impl PreEqDocument {
    pub(crate) fn materialize(
        &self,
        us: &UsConfiguration,
    ) -> ApiResult<(PreEqBank, PreEqPlayback)> {
        let initial = match &self.initial {
            Some(bank) => bank.materialize(us)?,
            None => PreEqBank::unity(us),
        };
        let mut events = Vec::new();
        for event in &self.events {
            let operation = match &event.operation {
                UpdateDocument::Snapshot { bank } => {
                    PreEqOperation::Snapshot(bank.materialize(us)?)
                }
                UpdateDocument::Initialize {
                    base_bank,
                    probe,
                    values,
                } => PreEqOperation::Initialize {
                    base_bank: *base_bank,
                    probe: *probe,
                    values: values
                        .iter()
                        .map(|(bin, z)| (*bin, Iq { re: z[0], im: z[1] }))
                        .collect(),
                },
                UpdateDocument::Adjust {
                    base_bank,
                    probe,
                    values,
                } => PreEqOperation::Adjust {
                    base_bank: *base_bank,
                    probe: *probe,
                    values: values
                        .iter()
                        .map(|(bin, z)| (*bin, Iq { re: z[0], im: z[1] }))
                        .collect(),
                },
            };
            events.push(PreEqEvent {
                id: event.id,
                symbol: event.symbol,
                operation,
            });
        }
        let probes = self
            .probes
            .iter()
            .map(|probe| ProbeReference {
                id: probe.id,
                bank_id: probe.bank_id,
            })
            .collect();
        let playback = PreEqPlayback::new(us, self.initial_id, initial.clone(), events, probes)?;
        Ok((initial, playback))
    }
}
/// One exact full-grid synthetic target, not a modem/CMTS capture claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RxmerDocument {
    /// Explicit nonzero synthetic source ID.
    pub id: u64,
    /// Original physical optional dB grid at normalized equalized unit-BPSK reference.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 4096>")]
    pub mer_db: Vec<Option<f64>>,
}
/// Separate explicit synthetic selection/power playback configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RxmerSelectionDocument {
    /// Exact source inventory selected for the declared one-CM grid.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 256>")]
    pub sources: Vec<RxmerDocument>,
    /// single, average, hold or interpolate; no implicit capture Cartesian product.
    pub policy: String,
    /// None for single/average; required offsets for hold/interpolate, first zero.
    #[serde(
        default,
        deserialize_with = "crate::configuration::serde_limits::optional_vec::<_, _, 256>"
    )]
    pub offsets: Option<Vec<usize>>,
}
impl RxmerSelectionDocument {
    pub(crate) fn materialize(&self, us: &UsConfiguration) -> ApiResult<RxmerPlayback> {
        let sources = self
            .sources
            .iter()
            .map(|source| SyntheticRxmer::new(us, source.id, source.mer_db.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let policy = match (self.policy.as_str(), &self.offsets) {
            ("single", None) => RxmerPolicy::Single,
            ("average", None) => RxmerPolicy::Average,
            ("hold", Some(offsets)) => RxmerPolicy::Hold {
                offsets: offsets.clone(),
            },
            ("interpolate", Some(offsets)) => RxmerPolicy::Interpolate {
                offsets: offsets.clone(),
            },
            _ => {
                return Err(PhyError::InvalidInput {
                    field: "configuration.rxmer_policy",
                }
                .into())
            }
        };
        Ok(RxmerPlayback::new(us, sources, policy)?)
    }
}

// JSON cannot express nonfinite coefficients/targets. Reject direct Rust candidate
// edits before serde could turn NaN/Infinity into a semantically false null.
pub(crate) fn validate_finite(
    pre_eq: Option<&PreEqDocument>,
    rxmer: Option<&RxmerSelectionDocument>,
) -> ApiResult<()> {
    let finite_bank = |bank: &BankDocument| {
        bank.coefficients
            .iter()
            .all(|z| z.iter().all(|value| value.is_finite()))
    };
    if let Some(plan) = pre_eq {
        if plan.initial.as_ref().is_some_and(|bank| !finite_bank(bank))
            || plan.events.iter().any(|event| match &event.operation {
                UpdateDocument::Snapshot { bank } => !finite_bank(bank),
                UpdateDocument::Initialize { values, .. }
                | UpdateDocument::Adjust { values, .. } => values
                    .iter()
                    .any(|(_, z)| z.iter().any(|value| !value.is_finite())),
            })
        {
            return Err(PhyError::InvalidInput {
                field: "configuration.pre_eq_nonfinite",
            }
            .into());
        }
    }
    if rxmer.is_some_and(|plan| {
        plan.sources.iter().any(|source| {
            source
                .mer_db
                .iter()
                .flatten()
                .any(|value| !value.is_finite())
        })
    }) {
        return Err(PhyError::InvalidInput {
            field: "configuration.rxmer_nonfinite",
        }
        .into());
    }
    Ok(())
}

impl RetainedPayload for BankDocument {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            generation,
            coefficients,
        } = self;
        sum([
            generation.retained_children(),
            coefficients.retained_children(),
        ])
    }
}

impl RetainedPayload for EventDocument {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            id,
            symbol,
            operation,
        } = self;
        sum([
            id.retained_children(),
            symbol.retained_children(),
            operation.retained_children(),
        ])
    }
}

impl RetainedPayload for PreEqDocument {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            initial_id,
            initial,
            events,
            probes,
        } = self;
        sum([
            initial_id.retained_children(),
            initial.retained_children(),
            events.retained_children(),
            probes.retained_children(),
        ])
    }
}

impl RetainedPayload for ProbeDocument {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { id, bank_id } = self;
        sum([id.retained_children(), bank_id.retained_children()])
    }
}

impl RetainedPayload for RxmerDocument {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { id, mer_db } = self;
        sum([id.retained_children(), mer_db.retained_children()])
    }
}

impl RetainedPayload for RxmerSelectionDocument {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            sources,
            policy,
            offsets,
        } = self;
        sum([
            sources.retained_children(),
            policy.retained_children(),
            offsets.retained_children(),
        ])
    }
}

impl RetainedPayload for UpdateDocument {
    /// Visits the original operation's complete owned input and spare capacity.
    ///
    /// # Returns
    /// Checked dynamic child quantities; no bank is materialized or normalized.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Snapshot { bank } => bank.retained_children(),
            Self::Initialize {
                base_bank,
                probe,
                values,
            }
            | Self::Adjust {
                base_bank,
                probe,
                values,
            } => sum([
                base_bank.retained_children(),
                probe.retained_children(),
                values.retained_children(),
            ]),
        }
    }
}
