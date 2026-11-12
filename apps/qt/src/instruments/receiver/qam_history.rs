/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded independent-run QAM history without retaining waveform/job payloads.
use crate::simulation::closed_loop::ResultData;
use docsis_ofdm_fec::{
    profile::candidate::CapturedProfile,
    simulation::channel::{
        constellation::ConstellationResult, protected_constellation::ProtectedConstellation,
    },
};
use serde_json::{json, Value};
use std::{cell::RefCell, sync::Arc};

/// Per-run archive caps: 8192 data, 1024 NCP and 1024 per protected stream.
/// With at most 32 runs this retains at most 425984 cells and 518400 decoder LLRs.
pub(crate) struct Snapshot {
    pub id: u8,
    pub data: Option<ConstellationResult>,
    pub protected: Option<ProtectedConstellation>,
    pub decoder_llrs: Vec<f64>,
    pub decoder_word: Option<u64>,
}
struct Entry {
    id: u8,
    status: String,
    snapshot: Option<Arc<Snapshot>>,
}
thread_local! { static ENTRIES:RefCell<Vec<Entry>>=const {RefCell::new(Vec::new())}; }

/// Clears history with accepted-input invalidation or a fresh run.
pub(crate) fn clear() {
    ENTRIES.with(|entries| entries.borrow_mut().clear());
}
/// Registers the selected independent runs, retaining explicit unstarted states.
pub(crate) fn begin(profiles: &[CapturedProfile]) {
    clear();
    ENTRIES.with(|entries| {
        entries
            .borrow_mut()
            .extend(
                profiles
                    .iter()
                    .take(super::performance::MAX_PROFILES)
                    .map(|profile| Entry {
                        id: profile.profile_id,
                        status: "not_started".into(),
                        snapshot: None,
                    }),
            );
    });
}
fn bounded<T: Clone>(values: &[T], limit: usize) -> Vec<T> {
    (0..values.len().min(limit))
        .map(|index| values[index * values.len() / values.len().min(limit)].clone())
        .collect()
}
/// Archives bounded real pairs with their original bin/time/reference identities.
/// # Arguments
/// * `id` - Captured profile ID, separate from wire profile ID 0.
/// * `result` - Actual completed/partial run; no signal is reconstructed.
pub(crate) fn retain(id: u8, result: &ResultData) {
    let data = result
        .job
        .constellation
        .as_ref()
        .map(|data| ConstellationResult {
            schema_version: data.schema_version,
            selection: data.selection.clone(),
            status: data.status,
            collected_pairs: data.collected_pairs,
            reference: data.reference,
            channel_coefficients: data.channel_coefficients,
            pairs: bounded(&data.pairs, 8192),
            ncp_pairs: bounded(&data.ncp_pairs, 1024),
        });
    let protected = result.job.protected_constellation.as_ref().map(|data| {
        let mut copy = data.clone();
        copy.plc_data.pairs = bounded(&data.plc_data.pairs, 1024);
        copy.plc_preamble.pairs = bounded(&data.plc_preamble.pairs, 1024);
        copy.continuous_pilots.pairs = bounded(&data.continuous_pilots.pairs, 1024);
        copy.scattered_pilots.pairs = bounded(&data.scattered_pilots.pairs, 1024);
        copy
    });
    let snapshot = Arc::new(Snapshot {
        id,
        data,
        protected,
        decoder_llrs: result
            .observations
            .decoder_llrs
            .iter()
            .take(16200)
            .copied()
            .collect(),
        decoder_word: result.observations.decoder_word,
    });
    ENTRIES.with(|entries| {
        if let Some(entry) = entries.borrow_mut().iter_mut().find(|entry| entry.id == id) {
            entry.status = serde_json::to_value(result.job.status)
                .unwrap()
                .as_str()
                .unwrap()
                .into();
            entry.snapshot = Some(snapshot);
        }
    });
}
/// Resolves only the requested run; missing/unstarted profiles never use latest samples.
pub(crate) fn get(id: i32) -> Result<Arc<Snapshot>, String> {
    ENTRIES.with(|entries|entries.borrow().iter().find(|entry|i32::from(entry.id)==id)
        .and_then(|entry|entry.snapshot.clone()))
        .ok_or_else(||format!("Profile {id} has no retained QAM result. Select it in Configure WorkBench and run it."))
}
/// Supplies imported profile IDs and actual run availability, including not-run choices.
pub(crate) fn catalog() -> Value {
    let (mut ids, _) = crate::workbench::session::profile_catalog();
    ENTRIES.with(|entries| {
        let entries=entries.borrow();
        ids.extend(entries.iter().map(|entry|entry.id));ids.sort_unstable();ids.dedup();
        json!(ids.into_iter().map(|id| {
            let entry=entries.iter().find(|entry|entry.id==id);
            json!({"id":id,"status":entry.map_or("not_run",|entry|entry.status.as_str()),"available":entry.is_some_and(|entry|entry.snapshot.is_some())})
        }).collect::<Vec<_>>())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn archive_reduction_preserves_real_identity_and_missing_runs_never_fall_back() {
        let original = (0..10003).collect::<Vec<_>>();
        let reduced = bounded(&original, 8192);
        assert_eq!(reduced.len(), 8192);
        assert_eq!(reduced[0], 0);
        assert!(reduced.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(reduced[8191], 10001);
        assert_eq!(bounded::<u16>(&[], 1024), Vec::<u16>::new());
        clear();
        ENTRIES.with(|entries| {
            entries.borrow_mut().push(Entry {
                id: 3,
                status: "complete".into(),
                snapshot: Some(Arc::new(Snapshot {
                    id: 3,
                    data: None,
                    protected: None,
                    decoder_llrs: vec![],
                    decoder_word: None,
                })),
            })
        });
        assert_eq!(get(3).unwrap().id, 3);
        assert!(get(0).is_err());
        clear();
        assert!(get(3).is_err());
    }
}
