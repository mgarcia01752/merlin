/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Private restart snapshots preserve actual canonical protection records.
use super::*;
use crate::shared::update_recovery::OriginalPath;

#[allow(clippy::mutable_key_type)]
impl InputPaths {
    /// Copies and admits actual original records for one finite recovery worker.
    /// # Arguments
    /// * `budget` - Same canonical counter, including the still-retained registry.
    /// # Returns
    /// Guarded vector with independent native path buffers and direction flags.
    /// # Errors
    /// Foreign budget, checked capacity arithmetic or actual target admission.
    pub(in crate::shell) fn snapshot(
        &self,
        budget: &RetentionBudget,
    ) -> Result<Box<Retained<Vec<OriginalPath>>>, String> {
        if !budget.owns(self.storage.as_ref()) {
            return Err("Original source paths belong to another application host.".into());
        }
        let records = self.records();
        let paths: Vec<_> = records
            .iter()
            .map(|record| {
                let owners = record.owners.get();
                OriginalPath {
                    path: record.path.clone(),
                    downstream: owners.downstream,
                    upstream: owners.upstream,
                }
            })
            .collect();
        let mut bytes = size_of::<Retained<Vec<OriginalPath>>>()
            .checked_add(
                paths
                    .capacity()
                    .checked_mul(size_of::<OriginalPath>())
                    .ok_or("Original path snapshot capacity overflowed.")?,
            )
            .ok_or("Original path snapshot capacity overflowed.")?;
        for record in &paths {
            bytes = bytes
                .checked_add(record.path.capacity())
                .ok_or("Original path snapshot capacity overflowed.")?;
        }
        Ok(Box::new(
            budget
                .reserve_resources(RetainedResources {
                    payload_bytes: bytes,
                    ..Default::default()
                })
                .map_err(|error| format!("{error:?}"))?
                .retain(paths),
        ))
    }

    /// Admits every replacement record without modifying the live registry.
    /// # Arguments
    /// * `paths` - Strictly decoded actual canonical originals and owner flags.
    /// * `budget` - Accepting host, covering both old and candidate records.
    /// # Returns
    /// Separate complete registry ready for an infallible final swap.
    /// # Errors
    /// Duplicate/relative/unowned records, path population or retained admission.
    pub(in crate::shell) fn prepare(
        paths: Vec<OriginalPath>,
        budget: &RetentionBudget,
    ) -> Result<Self, String> {
        let candidate = Self::new(budget).map_err(|error| format!("{error:?}"))?;
        for record in paths {
            if !record.path.is_absolute()
                || candidate.contains(&record.path)
                || !(record.downstream || record.upstream)
            {
                return Err("Invalid or duplicate original source path in recovery.".into());
            }
            if record.downstream && record.upstream {
                candidate.register(budget, Direction::Downstream, record.path.clone())?;
                candidate.register(budget, Direction::Upstream, record.path)?;
            } else {
                let direction = if record.downstream {
                    Direction::Downstream
                } else {
                    Direction::Upstream
                };
                candidate.register(budget, direction, record.path)?;
            }
        }
        Ok(candidate)
    }

    /// Rechecks an exact private snapshot without allocating or following paths.
    /// # Arguments
    /// * `paths` - Original worker snapshot, including direction ownership flags.
    /// # Returns
    /// True only while all records and flags still match the live registry.
    pub(in crate::shell) fn matches(&self, paths: &[OriginalPath]) -> bool {
        let records = self.records();
        records.len() == paths.len()
            && paths.iter().all(|path| {
                records.get(path.path.as_path()).is_some_and(|record| {
                    let owners = record.owners.get();
                    owners.downstream == path.downstream && owners.upstream == path.upstream
                })
            })
    }

    /// Commits both metadata owners while the exact original-path registry is locked.
    /// # Arguments
    /// * `candidate`, `expected` - Admitted replacement and captured live inventory.
    /// * `commit` - Last fallible canonical pair transition, with no registry reentry.
    /// # Returns
    /// Original commit result after one infallible registry swap.
    /// # Errors
    /// Changed path inventory or commit rejection preserves every old record.
    pub(in crate::shell) fn replace_with<T>(
        &self,
        candidate: Self,
        expected: &[OriginalPath],
        commit: impl FnOnce() -> Result<T, crate::shell::ApplicationError>,
    ) -> Result<T, crate::shell::ApplicationError> {
        let mut records = self.records();
        if records.len() != expected.len()
            || !expected.iter().all(|path| {
                records.get(path.path.as_path()).is_some_and(|record| {
                    let owners = record.owners.get();
                    owners.downstream == path.downstream && owners.upstream == path.upstream
                })
            })
        {
            return Err(crate::shell::ApplicationError::Session(
                crate::shell::SessionError::StaleGuard,
            ));
        }
        let result = commit()?;
        std::mem::swap(&mut *records, &mut *candidate.records());
        Ok(result)
    }

    /// Verifies this prepared registry belongs to the receiving host counter.
    /// # Arguments
    /// * `budget` - Canonical destination counter.
    /// # Returns
    /// True for the same admitted root and all transitively registered children.
    pub(in crate::shell) fn belongs_to(&self, budget: &RetentionBudget) -> bool {
        budget.owns(self.storage.as_ref())
    }
}
