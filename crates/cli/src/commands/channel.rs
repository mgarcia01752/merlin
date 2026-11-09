/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin local-path adapter; scientific accounting and job lifecycle belong to Rust core.
use crate::{
    arguments::Arguments,
    json_io::{read_json, write_json},
};
use docsis_ofdm_fec::simulation::{
    channel::{document::ChannelDocument, run_channel, ChannelJobStatus},
    test_points::{export::MAX_EXPORT_BYTES, JsonlProbeExporter, NoopObserver, TestPointProbe},
};
use std::{io::Write, sync::atomic::AtomicBool};
/// Runs the shared channel job and writes its complete or partial typed result.
/// # Arguments
/// * `arguments` - Input/result paths and optional independent JSONL export path.
/// * `cancel` - Process signal token, passed unchanged into the shared job.
/// # Returns
/// Unit after non-overwriting export; cancellation/runtime stops retain a partial result.
/// # Errors
/// Rejects unrelated options, bad schema/configuration, existing destinations or
/// I/O/scientific failures. A failed scientific job is written before reporting failure.
pub(crate) fn run(arguments: Arguments, cancel: &AtomicBool) -> Result<(), String> {
    if arguments.candidate.is_some()
        || arguments.settings.is_some()
        || arguments.profile.is_some()
        || arguments.frame_supplied
    {
        return Err(
            "channel accepts --input, --output, --scenario and --probes-output only.".into(),
        );
    }
    let document: ChannelDocument = read_json(&arguments.input)?;
    let scenario = arguments.scenario.as_deref().map(read_json).transpose()?;
    let settings = document
        .settings_with_scenario(scenario.as_ref())
        .map_err(|error| error.to_string())?;
    if arguments.output.exists() {
        return Err("Output already exists.".into());
    }
    if document.probes.is_some() != arguments.probes_output.is_some() {
        return Err("Specify both request.probes and --probes-output, or neither.".into());
    }
    let mut probe = match document.probes {
        Some(config) => TestPointProbe::new(config).map_err(|error| error.to_string())?,
        None => TestPointProbe::disabled(),
    };
    let mut stream = if let Some(path) = arguments.probes_output {
        if path == arguments.output {
            return Err("Result and probe export paths must differ.".into());
        }
        Some(
            JsonlProbeExporter::new(
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)
                    .map_err(|error| error.to_string())?,
                MAX_EXPORT_BYTES,
            )
            .map_err(|error| error.to_string())?,
        )
    } else {
        None
    };
    let progress = |progress: docsis_ofdm_fec::simulation::channel::ChannelProgress| {
        if progress.completed_source_symbols > 0 && progress.completed_source_symbols % 1024 == 0 {
            log::debug!(
                "channel-progress source-symbols={} recovered-payloads={}",
                progress.completed_source_symbols,
                progress.recovered_payloads
            );
        }
    };
    let mut result = if let Some(exporter) = stream.as_mut() {
        run_channel(
            &settings,
            &document.limits,
            cancel,
            &mut probe,
            exporter,
            progress,
        )
    } else {
        run_channel(
            &settings,
            &document.limits,
            cancel,
            &mut probe,
            &mut NoopObserver,
            progress,
        )
    }
    .map_err(|error| error.to_string())?;
    // Final export flush/sync can fail after all scientific steps completed.
    // Record that adapter failure before writing the preserved scientific result.
    if let Some(exporter) = stream {
        let finalized = exporter.finish().and_then(|mut file| {
            file.flush()
                .and_then(|()| file.sync_all())
                .map_err(|error| {
                    docsis_ofdm_fec::SimulationError::new(
                        "ProbeExportIo",
                        format!("Probe output sync failed: {error}"),
                    )
                })
        });
        if let Err(error) = finalized {
            result.status = ChannelJobStatus::Failed;
            if result.error.is_none() {
                result.error = Some(error);
            }
        }
    }
    write_json(&result, &arguments.output)?;
    if result.status == ChannelJobStatus::Failed {
        return Err(result
            .error
            .map_or_else(|| "Channel job failed.".into(), |error| error.to_string()));
    }
    log::info!(
        "channel-complete status={:?} compared-bits={} payload-errors={}",
        result.status,
        result.summary.comparison.bits_tested,
        result.summary.comparison.bit_errors
    );
    Ok(())
}
