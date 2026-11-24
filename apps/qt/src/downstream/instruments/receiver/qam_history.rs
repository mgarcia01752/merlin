/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent application identities around detached authoritative public receiver histories.
use crate::downstream::simulation::closed_loop::ResultData;
use crate::downstream::workbench::session::Context;
use merlin::downstream::{
    FullLoopReceiverArchive, FullLoopReceiverArchivePolicy, SourceProfileFrame,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// One independent run identity and its bounded scientific owner; no live job/input retention.
pub(crate) struct Snapshot {
    pub id: u8,
    pub backend: FullLoopReceiverArchive,
}
pub(crate) struct Entry {
    id: u8,
    status: String,
    snapshot: Option<Arc<Snapshot>>,
}

/// Clears history after accepted-input invalidation or a fresh run.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// Unit; dropping the last archive releases only its detached observations.
pub(crate) fn clear(context: &Context) {
    context.qam.with(|entries| entries.borrow_mut().clear());
}
/// Registers independently selected source profiles with explicit unstarted states.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `profiles` - Source-catalog identities, bounded by the shared application history policy.
/// # Returns
/// Unit; no backend query, source or simulation is executed.
pub(crate) fn begin(context: &Context, profiles: &[SourceProfileFrame]) {
    clear(context);
    context.qam.with(|entries| {
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
            )
    });
}
/// Retains finite actual history through the public backend without copying whole results.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `id` - Application source-profile identity, separate from actual wire profile IDs.
/// * `result` - Immutable complete/partial high-level result; no scientific work is rerun.
/// # Returns
/// Unit on retention; actual job/performance/FEC ownership remains unchanged on failure.
/// # Errors
/// Preserves public archive-policy, checked-byte and observation failures as instrument errors.
pub(crate) fn retain(context: &Context, id: u8, result: &ResultData) -> Result<(), String> {
    let policy = FullLoopReceiverArchivePolicy::new(Default::default())
        .map_err(|cause| cause.to_string())?;
    let archived = result
        .backend()
        .receiver_archive_retained_bound(&policy)
        .and_then(|target| result.backend().receiver_archive_bounded(policy, target))
        .map_err(|cause| cause.to_string());
    let status = serde_json::to_value(result.job.status).map_err(|cause| cause.to_string())?;
    context.qam.with(|entries| {
        if let Some(entry) = entries.borrow_mut().iter_mut().find(|entry| entry.id == id) {
            entry.status = status.as_str().unwrap_or("unavailable").to_owned();
            entry.snapshot = archived.as_ref().ok().map(|backend| {
                Arc::new(Snapshot {
                    id,
                    backend: backend.clone(),
                })
            });
        }
    });
    archived.map(|_| ())
}
/// Resolves exactly the requested archived run with no fallback to current observations.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `id` - Application source-profile ID selected by the instrument.
/// # Returns
/// Shared immutable history; independent plots never pool separate runs.
/// # Errors
/// Reports unstarted/missing/unavailable history for that exact source profile.
pub(crate) fn get(context: &Context, id: i32) -> Result<Arc<Snapshot>, String> {
    context.qam.with(|entries|entries.borrow().iter().find(|entry|i32::from(entry.id)==id).and_then(|entry|entry.snapshot.clone()))
        .ok_or_else(||format!("Profile {id} has no retained QAM result. Select it in Configure WorkBench and run it."))
}
/// Formats original imported run IDs and actual availability, including unstarted selections.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// Existing bounded catalog JSON, without scientific or captured-profile-ID reconstruction.
pub(crate) fn catalog(context: &Context) -> Value {
    let (mut ids, _) = crate::downstream::workbench::session::profile_catalog(context);
    context.qam.with(|entries| {
        let entries=entries.borrow();ids.extend(entries.iter().map(|entry|entry.id));ids.sort_unstable();ids.dedup();
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
        let context = &crate::downstream::workbench::session::Context::default();
        // The original10003→8192/last10001 oracle is preserved by the public archive's
        // independent core test. Here the actual public saved-run owner exercises
        // the application registry, bounded retention and missing-run isolation.
        let mut document: Value = serde_json::from_slice(include_bytes!(
            "../../../../../../examples/workbench/single/workbench.merlin.json"
        ))
        .unwrap();
        document["receiver"]["duration_ms"] = json!(1);
        let plan = merlin::downstream::inputs::Workbench::prepare_full_loop(
            &serde_json::to_vec(&document).unwrap(),
        )
        .unwrap();
        let result = ResultData::new(
            plan.request(0)
                .unwrap()
                .run(&merlin::downstream::FullLoopControl::new())
                .unwrap(),
        );
        let archive = result
            .backend()
            .receiver_archive(FullLoopReceiverArchivePolicy::new(Default::default()).unwrap())
            .unwrap();
        let info = archive.info();
        assert!(info.payload_cells <= 8192 && info.ncp_cells <= 1024);
        clear(context);
        context.qam.with(|entries| {
            entries.borrow_mut().push(Entry {
                id: 3,
                status: "complete".into(),
                snapshot: Some(Arc::new(Snapshot {
                    id: 3,
                    backend: archive,
                })),
            })
        });
        assert_eq!(get(context, 3).unwrap().id, 3);
        assert!(get(context, 0).is_err());
        clear(context);
        assert!(get(context, 3).is_err());
    }
}
