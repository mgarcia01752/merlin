/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate Rust-process preflight/copy references over original saved workbenches.
use docsis_ofdm_fec::api::{
    ErrorCode, FullLoopCellQuery, FullLoopCellStream, FullLoopControl, FullLoopObservationPolicy,
    FullLoopReceiverArchiveInput, FullLoopReceiverArchivePolicy,
};
use pnm_input::Workbench;
use serde_json::{json, Value};

/// Independent reduced payload population, unrelated to the query page size.
const REDUCED_PAYLOAD_CELLS: usize = 3;
/// Independent reduced NCP population.
const REDUCED_NCP_CELLS: usize = 2;
/// Independent reduced capacity for each protected stream.
const REDUCED_PROTECTED_CELLS: usize = 4;
/// Short original decoder prefix, independent of physical-cell LLR widths.
const REDUCED_DECODER_LLRS: usize = 7;
/// Tiny original-cell page used equally in the independent native reference.
const QUERY_ROWS: usize = 1;

/// Writes real preflight bounds, exact copied metadata and unchanged scientific views.
///
/// # Returns
/// JSON for both original projects, observation/cancellation modes and three policies.
///
/// # Errors
/// Preserves project, engine, archive, query and serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let projects: [(&str, &[u8]); 2] = [
        (
            "single",
            include_bytes!("../../../examples/workbench/single/workbench.merlin.json"),
        ),
        (
            "multi-rxmer",
            include_bytes!("../../../examples/workbench/multi-rxmer/workbench.merlin.json"),
        ),
    ];
    let streams = [
        ("payload", FullLoopCellStream::Payload),
        ("ncp", FullLoopCellStream::Ncp),
        ("plc_data", FullLoopCellStream::PlcData),
        ("plc_preamble", FullLoopCellStream::PlcPreamble),
        ("continuous_pilots", FullLoopCellStream::ContinuousPilots),
        ("scattered_pilots", FullLoopCellStream::ScatteredPilots),
    ];
    let mut cases = Vec::new();
    for (name, bytes) in projects {
        for observed in [false, true] {
            for cancelled in [false, true] {
                let request = Workbench::prepare_full_loop(bytes)?.request(0)?;
                let request = if observed {
                    request
                        .with_observations(FullLoopObservationPolicy::new(Default::default())?)?
                } else {
                    request
                };
                let control = FullLoopControl::new();
                if cancelled {
                    control.cancel();
                }
                let result = request.clone().run(&control)?;
                for (policy_name, input) in [
                    ("default", FullLoopReceiverArchiveInput::default()),
                    (
                        "reduced",
                        FullLoopReceiverArchiveInput {
                            payload_cells: REDUCED_PAYLOAD_CELLS,
                            ncp_cells: REDUCED_NCP_CELLS,
                            protected_cells_per_stream: REDUCED_PROTECTED_CELLS,
                            decoder_llrs: REDUCED_DECODER_LLRS,
                            ..Default::default()
                        },
                    ),
                    (
                        "omitted",
                        FullLoopReceiverArchiveInput {
                            payload_cells: 0,
                            ncp_cells: 0,
                            protected_cells_per_stream: 0,
                            decoder_llrs: 0,
                            ..Default::default()
                        },
                    ),
                ] {
                    let policy = FullLoopReceiverArchivePolicy::new(input)?;
                    let bound = result.receiver_archive_retained_bound(&policy)?;
                    let legacy = result.receiver_archive(policy.clone())?;
                    let archive = result.receiver_archive_bounded(policy.clone(), bound)?;
                    let mut views = serde_json::Map::new();
                    for (name, stream) in streams {
                        let query = FullLoopCellQuery {
                            stream,
                            count: QUERY_ROWS,
                            ..Default::default()
                        };
                        views.insert(
                            name.to_owned(),
                            json!({
                                "qam": archive.qam(query.clone())?,
                                "decisions": archive.cell_decisions(query.clone())?,
                                "soft": archive.cell_soft_view(query)?,
                            }),
                        );
                    }
                    let decoder = match archive.decoder_soft_view() {
                        Ok(value) => serde_json::to_value(value)?,
                        Err(error) if error.code() == ErrorCode::MissingCoverage => Value::Null,
                        Err(error) => return Err(error.into()),
                    };
                    cases.push(json!({
                        "project": name, "observed": observed, "cancelled": cancelled,
                        "policy": policy_name, "settings": policy.settings(),
                        "bound": bound, "actual": archive.retained_footprint()?,
                        "info": archive.info(), "legacy_info": legacy.info(),
                        "views": views, "decoder": decoder,
                        "status": result.report().status,
                    }));
                }
            }
        }
    }
    println!("{}", serde_json::to_string(&json!({"cases": cases}))?);
    Ok(())
}
