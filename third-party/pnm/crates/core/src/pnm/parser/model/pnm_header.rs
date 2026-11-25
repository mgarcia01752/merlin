/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::pnm::data_type::{CaptureTime, FileVersionComponent};
use serde::Serialize;

/// PyPNM-compatible header fields; file_type_version is the type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PnmHeaderParameters {
    pub file_type: &'static str,
    pub file_type_version: u8,
    pub major_version: FileVersionComponent,
    pub minor_version: FileVersionComponent,
    pub capture_time: Option<CaptureTime>,
}
/// Header output wrapper; binary data is explicitly opt-in hexadecimal text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PnmHeaderModel {
    pub pnm_header: PnmHeaderParameters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
