/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Transient configuration summaries from the accepted public owner, never RX truth.
use super::Model;
use serde_json::{json, Value};

impl Model {
    /// Projects bounded accepted configuration labels for the existing startup blocks.
    /// # Returns
    /// Native configuration summaries without source/sample copies or simulated
    /// observations. Busy/unavailable inspectors leave details explicitly unavailable.
    pub(super) fn configuration_presentation(&self) -> Value {
        if !self.current() {
            return json!({"blocks":{},"details":{}});
        }
        self.controller.as_ref().and_then(|controller| controller.inspect(|simulation| {
            let document = simulation.document()?;
            let config = simulation.configuration()?;
            let preset = document.presets.iter().find(|preset| preset.id == document.selected_preset)?;
            let run = &document.simulation;
            let mut details = json!({
                "ucd":{"summary":"Accepted native UCD / OFDMA configuration.","rows":[
                    {"label":"Preset","value":document.selected_preset},
                    {"label":"Spacing (kHz)","value":preset.channel.subcarrier_spacing_khz.to_string()},
                    {"label":"CP / rolloff (samples)","value":format!("{} / {}",preset.channel.cyclic_prefix_samples,preset.channel.rolloff_samples)}]},
                "profiles":{"summary":"Accepted source burst inventory; no Qt resource reconstruction.","rows":[
                    {"label":"Selected data IUC","value":run.selected_iuc.to_string()},
                    {"label":"Source burst descriptors","value":preset.channel.bursts.len().to_string()}]},
                "grants":{"summary":"Accepted explicit synthetic scheduling; represented MAP/probe controls remain separate.","rows":[
                    {"label":"Grant ID","value":run.grant_id.to_string()},
                    {"label":"First frame","value":run.first_frame.to_string()},
                    {"label":"Frames / selected minislots","value":format!("{} / {}",run.frames,run.minislots.len())}]},
                "source":{"summary":"Accepted continuous PRBS15 source configuration; no source is advanced by inspection.","rows":[
                    {"label":"Seed","value":run.source_seed.to_string()}]},
                "transmitter":{"summary":"CM transmitter configuration accepted; waveform observations require an actual run.","rows":[
                    {"label":"Preset / IUC","value":format!("{} / {}",document.selected_preset,run.selected_iuc)},
                    {"label":"Source seed","value":run.source_seed.to_string()}]},
                "channel":{"summary":"Accepted synthetic physical channel H; transmit correction C is independent.","rows":[
                    {"label":"Impulse taps","value":config.channel.taps.len().to_string()},
                    {"label":"Noise variance","value":config.channel.noise_variance.to_string()},
                    {"label":"Delay (samples)","value":config.channel.delay_samples.to_string()}]},
                "receiver":{"summary":"Accepted received-pilot receiver policy; received observations are unavailable until execution.","rows":[
                    {"label":"Estimator","value":run.estimator},
                    {"label":"Reference variance","value":run.reference_variance.to_string()},
                    {"label":"LDPC iteration limit","value":run.ldpc_iterations.to_string()}]},
                "preEqualization":{"summary":if run.synthetic_pre_eq.is_some(){"Explicit synthetic TX correction C; not a measured capture or CMTS estimate."}else{"Native unity TX correction C; no measured Pre-Eq capture is loaded."},"rows":[]},
                "rxmer":{"summary":if run.synthetic_rxmer.is_some(){"Explicit synthetic RxMER plan configured; measured input admission remains separate."}else{"No upstream RxMER measurement or synthetic target is loaded."},"rows":[]},
                "verification":{"summary":"Recovered-source verification is unavailable until an actual upstream report exists.","rows":[]}
            });
            if let Some(pre_eq) = &run.synthetic_pre_eq {
                details["preEqualization"]["rows"] = json!([
                    {"label":"Initial bank ID","value":pre_eq.initial_id.to_string()},
                    {"label":"Bank coefficients","value":pre_eq.initial.as_ref().map_or_else(||"Native unity".into(),|bank|bank.coefficients.len().to_string())},
                    {"label":"Events / probes","value":format!("{} / {}",pre_eq.events.len(),pre_eq.probes.len())}]);
            }
            let mut blocks = serde_json::Map::new();
            for section in ["ucd","profiles","grants","source","transmitter","channel","receiver","preEqualization"] {
                blocks.insert(section.into(),json!({"status":"complete","reason":"Native configuration accepted; simulation observations are separate."}));
            }
            blocks.insert("rxmer".into(),json!({"status":"attention","reason":"Measured input admission and association remain separate."}));
            blocks.insert("verification".into(),json!({"status":"attention","reason":"No accepted upstream run report is shown here."}));
            Some(json!({"blocks":blocks,"details":details}))
        }).ok().flatten()).unwrap_or_else(||json!({"blocks":{},"details":{}}))
    }
}
