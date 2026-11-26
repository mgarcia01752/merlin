/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Read-only measured summaries from admitted public inputs and configuration.
use crate::upstream::session::MeasuredInputInfo;
use merlin::upstream::Simulation;
use serde_json::{json, Value};

/// Projects actual captured geometry without reinterpreting its generic source.
///
/// # Arguments
/// * `simulation` - Accepted public scientific owner, borrowed without mutation.
/// * `input` - Fixed source inventory from the same successful native admission.
///
/// # Returns
/// Bounded scalar rows, or None if configuration is unavailable. Source bytes,
/// association/replay schedules and observations stay with their owning APIs.
pub(super) fn presentation(simulation: &Simulation, input: MeasuredInputInfo) -> Option<Value> {
    let config = simulation.configuration()?;
    let us = &config.us_configuration;
    let waveform = us.waveform();
    let grant = us.grant();
    let profile = us.selected_profile();
    let mut details = json!({
        "ucd":detail("Measured UCD projection admitted with explicit association; synthetic fields are read-only.", &[
            ("Original UCD fragments", input.ucd_fragments.to_string()),
            ("Spacing (Hz)", waveform.numerology().spacing_hz().to_string()),
            ("CP / rolloff (samples)", format!("{} / {}", waveform.prefix(), waveform.rolloff())),
            ("Encompassed bandwidth (Hz)", waveform.bandwidth_hz().to_string())]),
        "profiles":detail("Actual captured profile inventory; no Qt allocation reconstruction.", &[
            ("Selected data IUC", profile.iuc().to_string()),
            ("Profile inventory", us.profiles().len().to_string())]),
        "grants":detail("Explicit plan grant applied to captured geometry; this is not device scheduling.", &[
            ("Grant ID", grant.id.to_string()),
            ("First frame", grant.first_frame.to_string()),
            ("Frames / selected minislots", format!("{} / {}",grant.frames,grant.minislots.len()))]),
        "source":detail("Accepted continuous PRBS15 source; inspection never advances it.", &[
            ("Seed", config.source_seed.to_string())]),
        "transmitter":detail("CM transmitter uses admitted measured-plan configuration; waveform observations require execution.", &[
            ("Selected IUC",profile.iuc().to_string()),
            ("Source seed",config.source_seed.to_string())]),
        "channel":detail("Configured physical H is independent of transmit correction C. Measured Pre-Eq does not supply channel H.", &[
            ("Impulse taps",config.channel.taps.len().to_string()),
            ("Noise variance",config.channel.noise_variance.to_string()),
            ("Delay (samples)",config.channel.delay_samples.to_string())]),
        "receiver":detail("Accepted received-pilot receiver; no transmitter truth is supplied to normal reception.", &[
            ("Estimator",format!("{:?}",config.receiver.estimator())),
            ("Reference variance",config.receiver.variance().to_string()),
            ("LDPC iteration limit",config.receiver.iterations().to_string())]),
        "preEqualization":detail("Admitted transmit correction C precedes physical H; original coefficients and replay context remain distinct from derived effective banks.", &[
            ("Initial applied capture",input.pre_eq_initial.to_string()),
            ("Explicit capture events",input.pre_eq_events.to_string()),
            ("Effective initial generation",config.pre_eq.generation().to_string()),
            ("Effective bank coefficients",config.pre_eq.coefficients().len().to_string())]),
        "rxmer":detail("Explicitly associated upstream RxMER; the backend admits only its supported residual model and compatible C/H composition.", &[
            ("Associated source captures",input.rxmer_captures.to_string()),
            ("Residual playback admitted",config.rxmer.is_some().to_string())]),
        "verification":detail("Recovered-source verification requires an actual upstream report.",&[])
    });
    if !input.pre_eq_initial && input.pre_eq_events == 0 {
        details["preEqualization"]["summary"] =
            "Native unity transmit correction C; this measured plan supplies no Pre-Eq capture."
                .into();
    }
    if input.rxmer_captures == 0 {
        details["rxmer"]["summary"] =
            "This measured plan supplies no RxMER capture; no residual target is invented.".into();
    }
    let mut blocks = serde_json::Map::new();
    for section in [
        "ucd",
        "profiles",
        "grants",
        "source",
        "transmitter",
        "channel",
        "receiver",
        "preEqualization",
    ] {
        blocks.insert(section.into(), json!({"status":"complete","reason":"Measured plan admitted through the native public API; observations require execution."}));
    }
    blocks.insert(
        "rxmer".into(),
        json!({"status":if input.rxmer_captures==0 {"unused"}else{"complete"},
        "reason":details["rxmer"]["summary"]}),
    );
    blocks.insert("verification".into(),json!({"status":"pending","reason":"Start upstream to obtain actual recovered-source verification."}));
    Some(json!({"input_mode":"measured","blocks":blocks,"details":details}))
}

fn detail(summary: &str, rows: &[(&str, String)]) -> Value {
    json!({"summary":summary,"rows":rows.iter().map(|(label,value)|json!({"label":label,"value":value})).collect::<Vec<_>>()})
}
