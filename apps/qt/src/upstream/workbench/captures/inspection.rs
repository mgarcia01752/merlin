/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Passive source evidence and bounded rows from existing public capture owners.
use super::{constants::CAPTURE_PAGE_ROWS, Family};
use crate::shared::presentation::bytes::compact_lower_hex as octets;
use crate::upstream::workbench::error_text;
use merlin::upstream::{
    inputs::{PreEqCapture, PreEqCaptureKind, RxmerCapture, UcdCapture, UcdTlv},
    ApiResult,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// Exact decoded binary source and one finite presentation; neither is associated.
pub(super) struct Inspection {
    pub(super) original: Box<[u8]>,
    pub(super) view: Value,
}

/// Inspects one actual binary or lossless source-JSON delivery without simulation.
///
/// # Arguments
/// * `family` - Explicit typed capture family selected by the application.
/// * `delivery` - Original bounded file delivery; never rewritten or reconstructed.
/// * `first` - Original zero-based record index, including missing-value positions.
///
/// # Returns
/// An independent original binary copy and at most the configured page-row count.
/// All observations and source coordinates come from existing public decoders.
///
/// # Errors
/// Preserves both original admission diagnostics in bounded text when neither
/// binary nor source JSON admits. Invalid page origins leave the source unchanged.
pub(super) fn inspect(family: Family, delivery: &[u8], first: usize) -> Result<Inspection, String> {
    match family {
        Family::Ucd => ucd(delivery, first),
        Family::Rxmer => rxmer(delivery, first),
        Family::PreEq => pre_eq(delivery, first),
    }
}

fn decode<T>(
    delivery: &[u8],
    binary: fn(&[u8]) -> ApiResult<T>,
    source_json: fn(&[u8]) -> ApiResult<T>,
) -> Result<T, String> {
    match binary(delivery) {
        Ok(capture) => Ok(capture),
        Err(binary_error) => source_json(delivery).map_err(|json_error| {
            error_text(format!(
                "Binary capture: {binary_error}; capture-source JSON: {json_error}"
            ))
        }),
    }
}

fn row(label: &str, value: impl ToString) -> Value {
    json!({"label":label,"value":value.to_string()})
}

fn data_row(coordinate: usize, first: impl ToString, second: impl ToString) -> Value {
    json!({"coordinate":coordinate.to_string(),"first":first.to_string(),"second":second.to_string()})
}

fn evidence(original: &[u8], delivery: &[u8]) -> Vec<Value> {
    vec![
        row("Original binary bytes", original.len()),
        row(
            "Original binary SHA-256",
            format!("{:x}", Sha256::digest(original)),
        ),
        row("Delivery bytes", delivery.len()),
        row(
            "Delivery SHA-256",
            format!("{:x}", Sha256::digest(delivery)),
        ),
    ]
}

fn page(first: usize, count: usize) -> Result<(usize, Value), String> {
    if (count == 0 && first != 0) || (count != 0 && first >= count) {
        return Err("Capture page origin is outside the original record inventory.".into());
    }
    let end = first.saturating_add(CAPTURE_PAGE_ROWS).min(count);
    let has_next = end < count;
    Ok((
        end,
        json!({
            "summary":if count == 0 {"No original records.".into()} else {format!("Records {}–{} of {}",first + 1,end,count)},
            "previous_first":first.saturating_sub(CAPTURE_PAGE_ROWS),
            "next_first":if has_next {end} else {first},
            "has_previous":first != 0,"has_next":has_next,
        }),
    ))
}

enum UcdRecord<'capture> {
    Channel(&'capture UcdTlv),
    Iuc { offset: usize, iuc: u8 },
    Attribute { iuc: u8, tlv: &'capture UcdTlv },
}

impl UcdRecord<'_> {
    fn offset(&self) -> usize {
        match self {
            Self::Channel(tlv) | Self::Attribute { tlv, .. } => tlv.offset,
            Self::Iuc { offset, .. } => *offset,
        }
    }

    fn view(&self) -> Value {
        match self {
            Self::Channel(tlv) => data_row(
                tlv.offset,
                format!("Channel TLV {}", tlv.kind),
                octets(&tlv.value),
            ),
            Self::Iuc { offset, iuc } => {
                data_row(*offset, "Burst IUC (attributes follow)", octets(&[*iuc]))
            }
            Self::Attribute { iuc, tlv } => data_row(
                tlv.offset,
                format!("IUC {iuc} attribute {}", tlv.kind),
                octets(&tlv.value),
            ),
        }
    }
}

fn ucd(delivery: &[u8], first: usize) -> Result<Inspection, String> {
    let capture = decode(
        delivery,
        UcdCapture::from_mac_management,
        UcdCapture::from_source_json,
    )?;
    let header = capture.header();
    let mut metadata = evidence(capture.original_bytes(), delivery);
    metadata.extend([
        row("Destination MAC (hex)", octets(&header.destination)),
        row("Source MAC (hex)", octets(&header.source)),
        row("Message length", header.message_length),
        row("Management version", header.version),
        row("Message type", header.message_type),
        row("Multipart raw", header.multipart),
        row("Upstream channel ID", header.upstream_channel_id),
        row(
            "Configuration change count",
            header.configuration_change_count,
        ),
        row("Minislot size ticks (raw)", header.minislot_size_ticks),
        row("Downstream channel ID", header.downstream_channel_id),
        row("Channel TLVs", capture.channel_parameters().len()),
        row("Burst descriptors", capture.bursts().len()),
    ]);
    let mut records: Vec<_> = capture
        .channel_parameters()
        .iter()
        .map(UcdRecord::Channel)
        .collect();
    for burst in capture.bursts() {
        // Coordinate is the API's original wrapper offset, not a reconstructed
        // binary address. The value is only its admitted raw IUC octet; all
        // attribute values retain their separate original API coordinates.
        records.push(UcdRecord::Iuc {
            offset: burst.offset,
            iuc: burst.iuc,
        });
        records.extend(burst.attributes.iter().map(|tlv| UcdRecord::Attribute {
            iuc: burst.iuc,
            tlv,
        }));
    }
    records.sort_by_key(UcdRecord::offset);
    let (end, paging) = page(first, records.len())?;
    let rows: Vec<_> = records[first..end].iter().map(UcdRecord::view).collect();
    Ok(Inspection {
        original: capture.original_bytes().into(),
        view: json!({"title":"Upstream UCD Capture","summary":"Structural inspection only; unassociated original UCD records do not establish executable configuration or readiness.",
            "reference_plane":"Original MAC-management envelope, DA through payload; no capture time or CM association is inferred.",
            "metadata_rows":metadata,"columns":["Original byte offset","Record","Value octets (hex)"],
            "data_rows":rows,"page":paging,"error":""}),
    })
}

fn rxmer(delivery: &[u8], first: usize) -> Result<Inspection, String> {
    let capture = decode(
        delivery,
        RxmerCapture::from_pnm,
        RxmerCapture::from_source_json,
    )?;
    let header = capture.header();
    let mut metadata = evidence(capture.original_bytes(), delivery);
    metadata.extend([
        row("Major version", header.major_version),
        row("Minor version (raw)", header.minor_version),
        row("Capture epoch seconds", header.capture_time_epoch_seconds),
        row("Interface index", header.if_index),
        row("CCAP ID raw octets (hex)", octets(&header.ccap_id_raw)),
        row(
            "CCAP ID text",
            capture.ccap_id_text().unwrap_or("Unavailable"),
        ),
        row("RPD ID (hex)", octets(&header.rpd_id)),
        row("RPD port (raw)", header.rpd_port),
        row("CM MAC (hex)", octets(&header.cm_mac)),
        row("Number of averages (raw)", header.number_of_averages),
        row("Pre-Eq flag (raw)", header.pre_eq_on_off),
        row(
            "Pre-Eq enabled",
            capture
                .pre_eq_enabled()
                .map_or_else(|| "Unavailable".into(), |value| value.to_string()),
        ),
        row("Subcarrier zero (Hz)", header.subcarrier_zero_hz),
        row("First physical subcarrier", header.first_active_subcarrier),
        row("Spacing (kHz)", header.subcarrier_spacing_khz),
        row("Declared data bytes", header.data_length_bytes),
    ]);
    let (end, paging) = page(first, capture.readings_db().len())?;
    let rows: Vec<_> = (first..end)
        .map(|index| {
            data_row(
                usize::from(header.first_active_subcarrier) + index,
                capture.readings_db()[index]
                    .map_or_else(|| "Unavailable".into(), |value| value.to_string()),
                octets(&capture.raw_readings()[index..index + 1]),
            )
        })
        .collect();
    Ok(Inspection {
        original: capture.original_bytes().into(),
        view: json!({"title":"Upstream RxMER Capture","summary":"Inspection only; these unassociated measured observations do not supply a complete measured plan or simulation readiness.",
            "reference_plane":"CMTS equalized upstream probe relative to ideal BPSK. Missing readings remain unavailable at their original physical subcarriers.",
            "metadata_rows":metadata,"columns":["Physical subcarrier","RxMER (dB)","Raw octet (hex)"],
            "data_rows":rows,"page":paging,"error":""}),
    })
}

fn pre_eq(delivery: &[u8], first: usize) -> Result<Inspection, String> {
    let capture = decode(
        delivery,
        PreEqCapture::from_pnm,
        PreEqCapture::from_source_json,
    )?;
    let header = capture.header();
    let (kind, summary, reference) = match capture.kind() {
        PreEqCaptureKind::Applied => ("Applied C", "Inspection only; unassociated applied TX correction C, not a channel estimate or simulation readiness.", "Original applied CM transmit-correction bank C; coefficients alone do not establish a measured association."),
        PreEqCaptureKind::LastUpdate => ("Last-update operand", "Inspection only; an unassociated last-update operand requires its explicit operation, probe and base bank before use.", "Original last-update operand; Set/Adjust semantics, probe context and base-bank lineage must come from a complete measured plan."),
    };
    let mut metadata = evidence(capture.original_bytes(), delivery);
    metadata.extend([
        row("Capture kind", kind),
        row("Major version", header.major_version),
        row("Minor version (raw)", header.minor_version),
        row("Capture epoch seconds", header.capture_time_epoch_seconds),
        row("Upstream channel ID", header.upstream_channel_id),
        row("CM MAC (hex)", octets(&header.cm_mac)),
        row("CMTS MAC (hex)", octets(&header.cmts_mac)),
        row("Subcarrier zero (Hz)", header.subcarrier_zero_hz),
        row("First physical subcarrier", header.first_active_subcarrier),
        row("Spacing (kHz)", header.subcarrier_spacing_khz),
        row("Declared data bytes", header.data_length_bytes),
    ]);
    let (end, paging) = page(first, capture.coefficients().len())?;
    let rows: Vec<_> = (first..end)
        .map(|index| {
            let coefficient = &capture.coefficients()[index];
            data_row(
                usize::from(header.first_active_subcarrier) + index,
                coefficient.re,
                coefficient.im,
            )
        })
        .collect();
    Ok(Inspection {
        original: capture.original_bytes().into(),
        view: json!({"title":"Upstream Pre-Eq Capture","summary":summary,"reference_plane":reference,
            "metadata_rows":metadata,"columns":["Physical subcarrier","Real coefficient","Imaginary coefficient"],
            "data_rows":rows,"page":paging,"error":""}),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const UCD: &str =
        include_str!("../../../../../../examples/measurements/upstream/ucd/minimal.hex");
    const RXMER: &str =
        include_str!("../../../../../../examples/measurements/upstream/rxmer/ccap-50khz.hex");
    const APPLIED: &str =
        include_str!("../../../../../../examples/measurements/upstream/pre-eq/applied.hex");
    const UPDATE: &str =
        include_str!("../../../../../../examples/measurements/upstream/pre-eq/last-update.hex");
    const LONG_RXMER: &str =
        include_str!("../../../../../../examples/measurements/upstream/measured/rxmer-40db.hex");

    fn hex(text: &str) -> Vec<u8> {
        let digits: String = text.split_whitespace().collect();
        digits
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
            .collect()
    }

    fn metadata<'view>(view: &'view Value, label: &str) -> &'view str {
        view["metadata_rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["label"] == label)
            .unwrap()["value"]
            .as_str()
            .unwrap()
    }

    #[test]
    fn actual_binary_and_source_json_keep_originals_and_distinct_delivery_evidence() {
        let ucd = hex(UCD);
        let rxmer = hex(RXMER);
        let pre_eq = hex(APPLIED);
        let sources = [
            (
                Family::Ucd,
                ucd.clone(),
                UcdCapture::from_mac_management(&ucd)
                    .unwrap()
                    .to_source_json()
                    .unwrap(),
            ),
            (
                Family::Rxmer,
                rxmer.clone(),
                RxmerCapture::from_pnm(&rxmer)
                    .unwrap()
                    .to_source_json()
                    .unwrap(),
            ),
            (
                Family::PreEq,
                pre_eq.clone(),
                PreEqCapture::from_pnm(&pre_eq)
                    .unwrap()
                    .to_source_json()
                    .unwrap(),
            ),
        ];
        for (family, original, document) in sources {
            let binary = inspect(family, &original, 0).unwrap();
            let mut delivery = document.into_bytes();
            let restored = inspect(family, &delivery, 0).unwrap();
            assert_eq!(binary.original.as_ref(), original);
            assert_eq!(restored.original, binary.original);
            assert_eq!(restored.view["data_rows"], binary.view["data_rows"]);
            assert_eq!(
                metadata(&restored.view, "Original binary SHA-256"),
                format!("{:x}", Sha256::digest(&original))
            );
            assert_eq!(
                metadata(&restored.view, "Delivery SHA-256"),
                format!("{:x}", Sha256::digest(&delivery))
            );
            assert_ne!(
                metadata(&restored.view, "Original binary SHA-256"),
                metadata(&restored.view, "Delivery SHA-256")
            );
            delivery.fill(0);
            assert_eq!(restored.original.as_ref(), original);
            assert_eq!(restored.view["error"], "");
        }
    }

    #[test]
    fn literal_rxmer_nulls_keep_their_original_carrier_and_raw_octet() {
        let inspected = inspect(Family::Rxmer, &hex(RXMER), 0).unwrap();
        let expected = [
            "0",
            "23.75",
            "31.75",
            "32",
            "40",
            "63.25",
            "63.5",
            "Unavailable",
        ];
        let raw = ["00", "5f", "7f", "80", "a0", "fd", "fe", "ff"];
        let rows = inspected.view["data_rows"].as_array().unwrap();
        assert_eq!(rows.len(), 8);
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(row["coordinate"], (128 + index).to_string());
            assert_eq!(row["first"], expected[index]);
            assert_eq!(row["second"], raw[index]);
        }
        assert_eq!(
            metadata(&inspected.view, "Original binary SHA-256"),
            "ffeadba3dcf58825fad71aa949a61dff0c24adad08f8b656d4a9c8cc552de03c"
        );
        assert_eq!(
            metadata(&inspected.view, "Capture epoch seconds"),
            "19088743"
        );
        assert_eq!(metadata(&inspected.view, "Interface index"), "270544960");
    }

    #[test]
    fn pre_eq_applied_and_update_keep_original_kind_and_signed_core_values() {
        for (source, kind, minimum, maximum) in [
            (APPLIED, "Applied C", "-4", "3.9998779296875"),
            (UPDATE, "Last-update operand", "-2", "1.99993896484375"),
        ] {
            let inspected = inspect(Family::PreEq, &hex(source), 0).unwrap();
            assert_eq!(metadata(&inspected.view, "Capture kind"), kind);
            assert_eq!(
                inspected.view["data_rows"][1],
                json!({"coordinate":"129","first":"1.125","second":"-0.25"})
            );
            assert_eq!(
                inspected.view["data_rows"][2],
                json!({"coordinate":"130","first":minimum,"second":maximum})
            );
            assert!(inspected.view["summary"]
                .as_str()
                .unwrap()
                .contains("unassociated"));
        }
    }

    #[test]
    fn ucd_opaque_channel_and_burst_attributes_keep_api_offsets_and_octets() {
        let mut original = hex(UCD);
        original.extend([200, 2, 0xa5, 0, 23, 6, 5, 201, 3, 0xff, 0, 0x80]);
        original[12..14].copy_from_slice(&22u16.to_be_bytes());
        let inspected = inspect(Family::Ucd, &original, 0).unwrap();
        assert_eq!(inspected.original.as_ref(), original);
        assert_eq!(
            inspected.view["data_rows"],
            json!([
                {"coordinate":"24","first":"Channel TLV 200","second":"a500"},
                {"coordinate":"28","first":"Burst IUC (attributes follow)","second":"05"},
                {"coordinate":"31","first":"IUC 5 attribute 201","second":"ff0080"}
            ])
        );
        assert_eq!(
            metadata(&inspected.view, "Configuration change count"),
            "254"
        );
        assert_eq!(
            metadata(&inspected.view, "Source MAC (hex)"),
            "020000000002"
        );
    }

    #[test]
    fn pages_use_the_independent_64_row_policy_and_exact_original_extents() {
        let original = hex(LONG_RXMER);
        let first = inspect(Family::Rxmer, &original, 0).unwrap();
        // Independent application-policy anchor, not imported from its implementation.
        assert_eq!(first.view["data_rows"].as_array().unwrap().len(), 64);
        assert_eq!(first.view["page"]["next_first"], 64);
        assert_eq!(first.view["page"]["has_previous"], false);
        let middle = inspect(Family::Rxmer, &original, 64).unwrap();
        assert_eq!(middle.view["data_rows"][0]["coordinate"], "192");
        assert_eq!(middle.view["page"]["previous_first"], 0);
        assert_eq!(middle.view["page"]["next_first"], 128);
        let last = inspect(Family::Rxmer, &original, 384).unwrap();
        assert_eq!(last.view["data_rows"].as_array().unwrap().len(), 16);
        assert_eq!(last.view["page"]["has_next"], false);
        assert_eq!(last.view["page"]["previous_first"], 320);
        assert!(inspect(Family::Rxmer, &original, 400).is_err());
        assert!(inspect(Family::Rxmer, &original, usize::MAX).is_err());
    }

    #[test]
    fn empty_original_inventories_accept_only_the_zero_page_origin() {
        let mut rxmer = hex(RXMER)[..297].to_vec();
        rxmer[293..297].fill(0);
        let mut pre_eq = hex(APPLIED)[..34].to_vec();
        pre_eq[30..34].fill(0);
        for (family, bytes) in [
            (Family::Ucd, hex(UCD)),
            (Family::Rxmer, rxmer),
            (Family::PreEq, pre_eq),
        ] {
            let inspected = inspect(family, &bytes, 0).unwrap();
            assert!(inspected.view["data_rows"].as_array().unwrap().is_empty());
            assert_eq!(inspected.view["page"]["has_next"], false);
            assert_eq!(inspected.view["page"]["has_previous"], false);
            assert!(inspect(family, &bytes, 1).is_err());
        }
    }

    #[test]
    fn opaque_header_flags_padding_and_raw_unsigned_time_are_never_normalized() {
        let mut original = hex(RXMER);
        original[5] = 165;
        original[6..10].copy_from_slice(&u32::MAX.to_be_bytes());
        original[269] = 0xff;
        original[285] = 200;
        let inspected = inspect(Family::Rxmer, &original, 0).unwrap();
        assert_eq!(inspected.original.as_ref(), original);
        assert_eq!(metadata(&inspected.view, "Minor version (raw)"), "165");
        assert_eq!(
            metadata(&inspected.view, "Capture epoch seconds"),
            "4294967295"
        );
        assert_eq!(metadata(&inspected.view, "Pre-Eq flag (raw)"), "200");
        assert_eq!(metadata(&inspected.view, "Pre-Eq enabled"), "Unavailable");
        assert!(metadata(&inspected.view, "CCAP ID raw octets (hex)").ends_with("ff"));
    }

    #[test]
    fn both_original_errors_survive_and_observation_json_is_not_a_source() {
        let observation = RxmerCapture::from_pnm(&hex(RXMER))
            .unwrap()
            .to_json()
            .unwrap();
        let binary = RxmerCapture::from_pnm(observation.as_bytes())
            .unwrap_err()
            .to_string();
        let source = RxmerCapture::from_source_json(observation.as_bytes())
            .unwrap_err()
            .to_string();
        let error = inspect(Family::Rxmer, observation.as_bytes(), 0)
            .err()
            .unwrap();
        assert!(error.contains(&binary));
        assert!(error.contains(&source));
        assert!(error.starts_with("Binary capture:"));
    }
}
