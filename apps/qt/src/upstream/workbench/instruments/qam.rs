/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Screen-only scatter coordinates over original or backend-normalized QAM.
use super::{common, QamRequest};
use crate::shared::presentation::constants::{SCATTER_AXIS_CENTER, SCATTER_HEADROOM_RATIO};
use merlin::upstream::{
    BatchReport, CellRole, Iq, PhyError, QamPoint, Tap, MAX_QAM_CONSTELLATION_REFERENCES,
    MAX_QAM_PAIR_POINTS, MAX_QAM_QUERY_POINTS,
};
use serde_json::{json, Value};

// Symmetric display extent for an all-zero plot, never a scientific amplitude floor.
const ZERO_SCATTER_DISPLAY_EXTENT: f64 = 1.0;

struct Marker {
    value: Iq,
    role: Value,
    series: &'static str,
    readout: String,
    detail: Value,
}

/// Queries one existing QAM operation and projects only screen coordinates.
/// # Arguments
/// * `report`, `request` - Immutable actual owner and complete typed selection.
/// # Returns
/// An unadmitted finite candidate with original details and operation copy limits.
/// # Errors
/// Public request failures, invalid values or screen extent overflow.
pub(super) fn project(report: &BatchReport, request: QamRequest) -> Result<Value, String> {
    let tap = match request {
        QamRequest::Raw { tap, .. } => tap,
        QamRequest::Constellation(request) => request.tap,
        QamRequest::Pairing(request) => request.left,
    };
    let mut display = common::envelope("qam", choices(report, tap)?, selection(request));
    display["limits"] = json!({"maximum_points":MAX_QAM_QUERY_POINTS,
        "maximum_references":MAX_QAM_CONSTELLATION_REFERENCES,"maximum_pairs":MAX_QAM_PAIR_POINTS});
    let mut metadata = common::report_metadata(report);
    let mut points = Vec::new();
    let mut references = Vec::new();
    match request {
        QamRequest::Raw {
            tap,
            loading,
            maximum_points,
        } => {
            let source = match report.qam(tap, loading, maximum_points) {
                Ok(source) => source,
                Err(error) => return common::unavailable(display, error),
            };
            metadata["tap"] = json!(source.tap);
            metadata["reference_plane"] =
                "Original actual stage I/Q; no unit-reference normalization.".into();
            metadata["copied_points"] = source.points.len().into();
            metadata["copy_limit"] = maximum_points.into();
            for point in &source.points {
                points.push(marker(point, point.value, "actual")?);
            }
        }
        QamRequest::Constellation(request) => {
            let source = match report.qam_constellation(request) {
                Ok(source) => source,
                Err(error) => return common::unavailable(display, error),
            };
            metadata["tap"] = json!(source.tap);
            metadata["normalization"] = json!(source.normalization);
            metadata["units"] = source.units.into();
            metadata["bit_order"] = source.bit_order.into();
            metadata["payload_loading"] = source.payload_loading.into();
            metadata["reference_bits"] = source.reference_bits.into();
            metadata["retained_matching_points"] = source.retained_matching_points.into();
            metadata["capped"] = source.capped.into();
            for point in &source.points {
                let mut value = marker(&point.observation, point.unit_value, "unit_reference")?;
                value.detail["normalization_divisor"] = json!(point.normalization_divisor);
                value.detail["unit_value"] = json!(point.unit_value);
                points.push(value);
            }
            for point in &source.references {
                references.push(Marker {
                    value: point.value,
                    role: "reference".into(),
                    series: "reference",
                    readout: format!(
                        "Canonical MSB-first label {} · I {} · Q {}",
                        point.label, point.value.re, point.value.im
                    ),
                    detail: json!({"label":point.label,"value":point.value}),
                });
            }
        }
        QamRequest::Pairing(request) => {
            let source = match report.qam_pairing(request) {
                Ok(source) => source,
                Err(error) => return common::unavailable(display, error),
            };
            metadata["left"] = json!(source.left);
            metadata["right"] = json!(source.right);
            metadata["reference_plane"] =
                "Exact actual-stage pairs; each side retains its original amplitude plane.".into();
            metadata["left_selected"] = source.left_selected.into();
            metadata["right_selected"] = source.right_selected.into();
            metadata["matched_total"] = source.matched_total.into();
            metadata["unpaired_left"] = source.unpaired_left.into();
            metadata["unpaired_right"] = source.unpaired_right.into();
            metadata["capped"] = source.capped.into();
            for pair in &source.pairs {
                points.push(marker(&pair.left, pair.left.value, "left")?);
                points.push(marker(&pair.right, pair.right.value, "right")?);
            }
            if source.pairs.is_empty() {
                display["status"] = "unavailable".into();
                display["message"] =
                    "No exact retained physical-coordinate pairs are available.".into();
            }
        }
    }
    display["metadata"] = metadata;
    display["scatter"] = scatter(points, references)?;
    Ok(display)
}

fn choices(report: &BatchReport, selected: Tap) -> Result<Value, String> {
    let taps: Vec<_> = report
        .options
        .retained_taps
        .iter()
        .copied()
        .filter(|tap| common::cell_tap(*tap))
        .map(|tap| {
            Ok(json!({"value":tap,"label":format!("{tap:?}"),
            "available":available(report,tap,None)?,
            "unit_reference":matches!(tap,Tap::TxGrid|Tap::RxEqualized)}))
        })
        .collect::<Result<Vec<_>, String>>()?;
    // Discover the complete public selector domain through the existing filter.
    // Base profile loading alone omits complementary-cell loadings; deriving
    // those numerically here would duplicate science. A capped unfiltered query
    // also cannot prove that later retained points contain no other loading.
    // Exactly 256 filters scan the existing bounded history (currently at most
    // 65536 retained points); each copies at most one point, off the GUI thread.
    let mut loadings = Vec::new();
    for loading in u8::MIN..=u8::MAX {
        if available(report, selected, Some(loading))? {
            loadings.push(loading);
        }
    }
    Ok(
        json!({"taps":taps,"modes":[{"label":"Actual raw I/Q","value":"raw"},
        {"label":"Backend unit reference","value":"constellation"},
        {"label":"Exact actual-stage pairs","value":"pairing"}],
        "loadings":std::iter::once(json!({"value":null,"label":"All retained loadings"})).chain(loadings.into_iter().map(|loading|json!({
        "value":loading,"label":if loading==0 {"Known pilots (zero payload bits)".into()}else{format!("{loading} payload bits per cell")}
    }))).collect::<Vec<_>>()}),
    )
}

fn available(report: &BatchReport, tap: Tap, loading: Option<u8>) -> Result<bool, String> {
    match report.qam(tap, loading, 1) {
        Ok(_) => Ok(true),
        Err(PhyError::Unsupported { .. }) => Ok(false),
        Err(error) => Err(error.to_string()),
    }
}

/// Echoes exactly the complete flat request without deriving scientific settings.
/// # Arguments
/// * `request` - Actual typed mode, tap/loading selections and copy limits.
/// # Returns
/// Serde-compatible scalar transport for this same operation.
pub(super) fn selection(request: QamRequest) -> Value {
    match request {
        QamRequest::Raw {
            tap,
            loading,
            maximum_points,
        } => json!({"mode":"raw","tap":tap,"loading":loading,"maximum_points":maximum_points}),
        QamRequest::Constellation(request) => json!({"mode":"constellation","tap":request.tap,
            "loading":request.loading,"maximum_points":request.maximum_points,"maximum_references":request.maximum_references}),
        QamRequest::Pairing(request) => {
            json!({"mode":"pairing","left":request.left,"right":request.right,
            "loading":request.loading,"maximum_pairs":request.maximum_pairs})
        }
    }
}

fn marker(point: &QamPoint, value: Iq, series: &'static str) -> Result<Marker, String> {
    Ok(Marker {
        value,
        role: match point.role {
            CellRole::Data => "data",
            CellRole::Complementary => "complementary",
            CellRole::KnownPilot => "known_pilot",
        }
        .into(),
        series,
        readout: format!(
            "{series} · frame {} · symbol {} · carrier {} · loading {} · raw I {} · raw Q {}",
            point.frame, point.symbol, point.carrier, point.loading, point.value.re, point.value.im
        ),
        detail: common::point_detail(point)?,
    })
}

fn scatter(points: Vec<Marker>, references: Vec<Marker>) -> Result<Value, String> {
    let mut extent = 0.0_f64;
    for point in points.iter().chain(&references) {
        if !point.value.re.is_finite() || !point.value.im.is_finite() {
            return Err("Nonfinite upstream scatter value.".into());
        }
        extent = extent.max(point.value.re.abs()).max(point.value.im.abs());
    }
    if extent == 0.0 {
        extent = ZERO_SCATTER_DISPLAY_EXTENT;
    }
    extent *= SCATTER_HEADROOM_RATIO;
    if !extent.is_finite() {
        return Err("Upstream scatter screen extent overflowed.".into());
    }
    let project = |point: Marker| {
        json!({"x":SCATTER_AXIS_CENTER+(point.value.re/extent)*SCATTER_AXIS_CENTER,
        "y":SCATTER_AXIS_CENTER-(point.value.im/extent)*SCATTER_AXIS_CENTER,
        "role":point.role,"series":point.series,"readout":point.readout,"detail":point.detail})
    };
    Ok(
        json!({"extent":extent,"points":points.into_iter().map(project).collect::<Vec<_>>(),
        "references":references.into_iter().map(project).collect::<Vec<_>>()}),
    )
}
