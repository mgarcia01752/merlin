/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Typed channel editor admission and bounded response previews over shared core APIs.
use crate::downstream::workbench::session::Context;
use crate::{
    downstream::{
        common::constants::{
            CHANNEL_GAIN_AXIS_PADDING_DB, CHANNEL_GAIN_TICK_INTERVALS,
            CHANNEL_MODEL_SECTION_VERSION, CHANNEL_POWER_SECTION_VERSION,
        },
        workbench::session::profile_source,
    },
    shared::presentation::constants::HERTZ_PER_MEGAHERTZ,
};
use merlin::downstream::{
    AdditionalLoss, AssociationTolerance, ChannelConfiguration, ChannelConfigurationInput,
    ChannelFilterSettings, PowerConditioning, TransmissionConfiguration, TransmissionSettings,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    schema_version: u32,
    additional_loss: AdditionalLoss,
}
/// Restores a versioned model independently of incomplete source/profile editors.
/// # Arguments
/// * `value` - Optional saved section; null selects legacy Off behavior.
/// # Returns
/// Validated model parameters, without modifying measured captures.
/// # Errors
/// Rejects unknown fields/versions and nonfinite or out-of-range parameters.
pub(crate) fn restore(value: &Value) -> Result<AdditionalLoss, String> {
    if value.is_null() {
        return Ok(AdditionalLoss::Off);
    }
    let saved: Saved = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    if saved.schema_version != CHANNEL_MODEL_SECTION_VERSION {
        return Err("Unsupported channel model version.".into());
    }
    ChannelConfiguration::new(ChannelConfigurationInput {
        additional_loss: saved.additional_loss.clone(),
        ..Default::default()
    })
    .map_err(|cause| cause.message().to_owned())?;
    Ok(saved.additional_loss)
}
/// Encodes only explicit model settings, preserving embedded original capture bytes.
/// # Arguments
/// * `loss` - Accepted model, already validated.
/// # Returns
/// Version-1 workbench section.
pub(crate) fn saved(loss: &AdditionalLoss) -> Value {
    json!({"schema_version":CHANNEL_MODEL_SECTION_VERSION,"additional_loss":loss})
}
/// Builds a bounded additional/effective response chart using the shared impairment planner.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `loss` - Draft additional response.
/// * `settings` - Accepted OCD and physical numerology.
/// # Returns
/// At most128 gain points, physical band endpoints and endpoint gain summary.
/// # Errors
/// Rejects missing DPD, invalid active coverage or unsupported response/model values.
pub(crate) fn preview(
    context: &Context,
    loss: &AdditionalLoss,
    settings: &TransmissionSettings,
) -> Result<Value, String> {
    preview_inputs(
        loss,
        settings,
        &crate::downstream::workbench::session::snapshot(context),
    )
}

/// Builds the same channel preview from an explicit independent input snapshot.
/// # Arguments
/// * `loss` - Proposed additional channel response.
/// * `settings` - Accepted OCD geometry.
/// * `inputs` - Restored or GUI-owned inputs; no thread-local reads occur.
/// # Returns
/// Bounded gain/effective-response presentation from those exact inputs.
/// # Errors
/// Preserves profile, response, geometry and additional-loss validation errors.
pub(crate) fn preview_inputs(
    loss: &AdditionalLoss,
    settings: &TransmissionSettings,
    inputs: &crate::downstream::workbench::session::Inputs,
) -> Result<Value, String> {
    let profile = inputs
        .profile
        .as_ref()
        .ok_or("Apply DPD before configuring additional loss.")?;
    let (catalog, profile_id) = profile_source(Some(profile), inputs.profile_capture.as_ref())?;
    let preview = ChannelConfiguration::new(ChannelConfigurationInput {
        additional_loss: loss.clone(),
        ..Default::default()
    })
    .and_then(|configuration| {
        configuration.preview_capture(
            &TransmissionConfiguration::new(settings.clone())?,
            catalog,
            profile_id,
            inputs.response.as_ref(),
            AssociationTolerance::new(inputs.response_tolerance)?,
        )
    })
    .map_err(|cause| cause.message().to_owned())?;
    let low = preview.active_low_frequency_hz;
    let high = preview.active_high_frequency_hz;
    let low_gain = preview.additional_low_gain_db;
    let high_gain = preview.additional_high_gain_db;
    let points: Vec<_> = preview
        .additional_points
        .iter()
        .map(|point| {
            json!({
                "frequency_hz":point.frequency_hz,"gain_db":point.additional_gain_db,
                "effective_db":point.effective_gain_db
            })
        })
        .collect();
    let minimum = points
        .iter()
        .flat_map(|point| {
            [
                point["gain_db"].as_f64().unwrap_or(0.0),
                point["effective_db"].as_f64().unwrap_or(0.0),
            ]
        })
        .fold(0.0_f64, f64::min)
        .floor()
        - CHANNEL_GAIN_AXIS_PADDING_DB;
    let maximum = points
        .iter()
        .flat_map(|point| {
            [
                point["gain_db"].as_f64().unwrap_or(0.0),
                point["effective_db"].as_f64().unwrap_or(0.0),
            ]
        })
        .fold(0.0_f64, f64::max)
        .ceil()
        + CHANNEL_GAIN_AXIS_PADDING_DB;
    let chart:Vec<_>=points.iter().map(|point| json!({"x":(point["frequency_hz"].as_f64().unwrap_or(f64::from(low))-f64::from(low))/f64::from(high-low),
        "additional":(maximum-point["gain_db"].as_f64().unwrap_or(0.0))/(maximum-minimum),
        "effective":(maximum-point["effective_db"].as_f64().unwrap_or(0.0))/(maximum-minimum)})).collect();
    let ticks: Vec<_> = (0..=CHANNEL_GAIN_TICK_INTERVALS)
        .map(|index| {
            let position = f64::from(index) / f64::from(CHANNEL_GAIN_TICK_INTERVALS);
            json!({"x":position,"label":format!("{:.1}",maximum-position*(maximum-minimum))})
        })
        .collect();
    Ok(
        json!({"error":"","chart":chart,"y_ticks":ticks,"x_ticks":[{"x":0,"label":format!("{:.3} MHz",f64::from(low)/HERTZ_PER_MEGAHERTZ)},{"x":1,"label":format!("{:.3} MHz",f64::from(high)/HERTZ_PER_MEGAHERTZ)}],"points":points,"band":format!("{:.3}–{:.3} MHz",f64::from(low)/HERTZ_PER_MEGAHERTZ,f64::from(high)/HERTZ_PER_MEGAHERTZ),
        "low_gain":format!("{low_gain:.2} dB"),"high_gain":format!("{high_gain:.2} dB"),"tilt":format!("{:.2} dB",high_gain-low_gain)}),
    )
}

/// Previews accepted total gain without starting a run or reading retained results.
/// # Arguments
/// * `settings` - Accepted OCD grid and numerology.
/// * `inputs` - Frozen accepted DPD, response, loss and power settings.
/// * `filter` - Accepted continuous-filter policy, independently of receiver mode.
/// # Returns
/// At most1024 min/max groups, physical frequency/gain axes and known-code help.
/// # Errors
/// Rejects missing DPD, invalid models or unsupported composition without stale points.
pub(crate) fn combined_preview(
    settings: &TransmissionSettings,
    inputs: &crate::downstream::workbench::session::Inputs,
    filter: &ChannelFilterSettings,
) -> Result<Value, crate::downstream::help::AdmissionError> {
    let profile = inputs
        .profile
        .as_ref()
        .ok_or("Apply DPD to preview the channel response.")?;
    let (catalog, profile_id) = profile_source(Some(profile), inputs.profile_capture.as_ref())?;
    let preview = ChannelConfiguration::new(ChannelConfigurationInput {
        additional_loss: inputs.additional_loss.clone(),
        power: inputs.power.clone(),
        filter: filter.clone(),
    })
    .and_then(|configuration| {
        configuration.preview_capture(
            &TransmissionConfiguration::new(settings.clone())?,
            catalog,
            profile_id,
            inputs.response.as_ref(),
            AssociationTolerance::new(inputs.response_tolerance)?,
        )
    })
    .map_err(crate::downstream::help::AdmissionError::classified)?;
    let mut view = serde_json::to_value(preview.gain).map_err(|cause| cause.to_string())?;
    view["available"] = json!(view["points"]
        .as_array()
        .is_some_and(|points| points.iter().any(|point| point["y"].is_number())));
    view["axis_label"] = json!("Gain · dB");
    view["frequency_label"] = json!("Frequency (MHz)");
    for tick in view["y_ticks"]
        .as_array_mut()
        .ok_or("Missing channel gain ticks.")?
    {
        tick["y"] = tick["x"].clone();
    }
    view["summary"] = json!("Accepted channel response · before receiver correction");
    Ok(view)
}

/// Restores optional scalar conditioning without resetting legacy additional loss.
/// # Arguments
/// * `value` - Versioned channel_power section or null from a legacy WB.
/// # Returns
/// Validated conditioning; absent controls select unity and relative units.
/// # Errors
/// Rejects unknown fields, unsupported versions and invalid electrical parameters.
pub(crate) fn restore_power(value: &Value) -> Result<PowerConditioning, String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SavedPower {
        schema_version: u32,
        conditioning: PowerConditioning,
    }
    if value.is_null() {
        return Ok(Default::default());
    }
    let saved: SavedPower = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if saved.schema_version != CHANNEL_POWER_SECTION_VERSION {
        return Err("Unsupported channel power version.".into());
    }
    ChannelConfiguration::new(ChannelConfigurationInput {
        power: saved.conditioning.clone(),
        ..Default::default()
    })
    .map_err(|cause| cause.message().to_owned())?;
    Ok(saved.conditioning)
}
/// Encodes accepted scalar conditioning in its own versioned section.
/// # Arguments
/// * `power` - Validated scalar conditioning.
/// # Returns
/// Version1 section, separate from existing channel_model additional loss.
pub(crate) fn saved_power(power: &PowerConditioning) -> Value {
    json!({"schema_version":CHANNEL_POWER_SECTION_VERSION,"conditioning":power})
}

#[cfg(test)]
mod tests {
    use super::*;
    use merlin::downstream::{CableModel, LengthUnit};
    #[test]
    fn combined_preview_is_bounded_and_preserves_admission_diagnostics() {
        use merlin::downstream::{ChannelFilterSettings, EchoPath};
        let project: crate::downstream::workbench::project_file::Project =
            serde_json::from_slice(include_bytes!(
                "../../../../../examples/workbench/downstream/single/workbench.merlin.json"
            ))
            .unwrap();
        let original = project.validate().unwrap();
        let mut inputs = original.clone();
        inputs.response = None;
        inputs.additional_loss = AdditionalLoss::Manual { tilt_db: -6.0 };
        let filter = ChannelFilterSettings::Echoes {
            paths: vec![
                EchoPath {
                    delay_samples: 0,
                    gain: [1.0, 0.0],
                },
                EchoPath {
                    delay_samples: 8,
                    gain: [0.05, 0.02],
                },
            ],
        };
        let preview = combined_preview(&project.transmission, &inputs, &filter).unwrap();
        assert_eq!(preview["available"], true);
        assert_eq!(preview["axis_label"], "Gain · dB");
        assert_eq!(preview["frequency_label"], "Frequency (MHz)");
        let points = preview["points"].as_array().unwrap();
        assert!(!points.is_empty() && points.len() <= 1024);
        assert!(points
            .iter()
            .all(|point| point["y"].as_f64().is_some_and(f64::is_finite)));
        assert!(original.response.is_some());
        let error = combined_preview(&project.transmission, &original, &filter).unwrap_err();
        assert!(error.diagnostic.is_some());
        assert!(error
            .message
            .contains("Synthetic echoes require Flat / unity"));
        inputs.profile = None;
        let error = combined_preview(&project.transmission, &inputs, &filter).unwrap_err();
        assert!(error.message.contains("Apply DPD"));
        assert!(error.diagnostic.is_none());
    }
    #[test]
    fn saved_models_preserve_units_and_legacy_off_without_accepting_version_drift() {
        assert_eq!(restore(&Value::Null).unwrap(), AdditionalLoss::Off);
        let loss = AdditionalLoss::Cable {
            cable: CableModel::Belden1694aV1,
            length: 30.48,
            unit: LengthUnit::Metres,
        };
        assert_eq!(restore(&saved(&loss)).unwrap(), loss);
        let mut document = saved(&loss);
        document["schema_version"] = json!(2);
        assert!(restore(&document).is_err());
        document = saved(&loss);
        document["additional_loss"]["length"] = json!(-1);
        assert!(restore(&document).is_err());
    }
}
