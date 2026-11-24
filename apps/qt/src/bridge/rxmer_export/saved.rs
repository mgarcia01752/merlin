/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual queued save replies and fixed truthful publication-status fallbacks.
use super::*;
use crate::{
    downstream::{
        common::constants::{
            DESIGNER_CANCELLED_STATUS_ADMISSION_MESSAGE, DESIGNER_FAILED_STATUS_ADMISSION_MESSAGE,
            DESIGNER_SAVED_STATUS_ADMISSION_MESSAGE,
        },
        inputs::retention as count,
    },
    shell::RetainedResources,
};
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::bridge) enum Status {
    Saved,
    Cancelled,
    Failed,
}

pub(in crate::bridge) enum Saved {
    Detailed(Retained<Value>),
    Rejected(Status),
}

impl Saved {
    /// Admits actual save-completion JSON before publishing its worker terminal.
    /// # Arguments
    /// * `value` - Existing completed save/cancel/error reply; publication is already known.
    /// * `budget` - Same host counter as the retained Prepared and accepted owners.
    /// # Returns
    /// Guarded unchanged reply or a fixed Copy status after unadmitted details drop.
    /// Metadata/host rejection cannot turn a successful file write into a failure.
    pub(super) fn admit(value: Value, budget: &RetentionBudget) -> Self {
        let status = if value["saved"] == true {
            Status::Saved
        } else if value["cancelled"] == true {
            Status::Cancelled
        } else {
            Status::Failed
        };
        let resources = count::json_children(&value).and_then(|children| {
            let mut total = RetainedResources {
                payload_bytes: size_of::<Value>(),
                ..Default::default()
            };
            count::append(&mut total, children)?;
            Ok(total)
        });
        match resources.and_then(|resources| count::admit_model(value, resources, budget)) {
            Ok(value) => Self::Detailed(value),
            Err(_) => Self::Rejected(status),
        }
    }

    /// Builds the existing finite presentation reply while borrowing its guarded source.
    /// # Returns
    /// Unchanged detailed JSON or a static status message. A rejected successful
    /// save omits the saved/path pair because no path was retained; its text states
    /// that publication succeeded. The caller keeps this owner through construction.
    pub(super) fn reply(&self) -> Value {
        match self {
            Self::Detailed(value) => value.value().clone(),
            Self::Rejected(status) => {
                let message = match status {
                    Status::Saved => DESIGNER_SAVED_STATUS_ADMISSION_MESSAGE,
                    Status::Cancelled => DESIGNER_CANCELLED_STATUS_ADMISSION_MESSAGE,
                    Status::Failed => DESIGNER_FAILED_STATUS_ADMISSION_MESSAGE,
                };
                let mut value = json!({"running":false,"error":message});
                if *status == Status::Cancelled {
                    value["cancelled"] = json!(true);
                }
                value
            }
        }
    }
}
