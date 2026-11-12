/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Typed channel editor admission and bounded response previews over shared core APIs.
use docsis_ofdm_fec::{
    signal_processing::channel::{
        additional_loss::AdditionalLoss, impairments::ChannelImpairments,
    },
    simulation::channel::measurement::captured_layout,
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        TransmissionSettings,
    },
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
    if saved.schema_version != 1 {
        return Err("Unsupported channel model version.".into());
    }
    saved
        .additional_loss
        .validate()
        .map_err(|error| error.to_string())?;
    Ok(saved.additional_loss)
}
/// Encodes only explicit model settings, preserving embedded original capture bytes.
/// # Arguments
/// * `loss` - Accepted model, already validated.
/// # Returns
/// Version-1 workbench section.
pub(crate) fn saved(loss: &AdditionalLoss) -> Value {
    json!({"schema_version":1,"additional_loss":loss})
}
/// Builds a bounded additional/effective response chart using the shared impairment planner.
/// # Arguments
/// * `loss` - Draft additional response.
/// * `settings` - Accepted OCD and physical numerology.
/// # Returns
/// At most128 gain points, physical band endpoints and endpoint gain summary.
/// # Errors
/// Rejects missing DPD, invalid active coverage or unsupported response/model values.
pub(crate) fn preview(
    loss: &AdditionalLoss,
    settings: &TransmissionSettings,
) -> Result<Value, String> {
    preview_inputs(loss, settings, &crate::workbench::session::snapshot())
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
    inputs: &crate::workbench::session::Inputs,
) -> Result<Value, String> {
    let profile = inputs
        .profile
        .as_ref()
        .ok_or("Apply DPD before configuring additional loss.")?;
    let layout = captured_layout(profile, settings)
        .and_then(|config| CarrierLayout::new(settings, &config))
        .map_err(|error| error.to_string())?;
    let plan = ChannelImpairments {
        response: inputs.response.clone(),
        additional_loss: loss.clone(),
        ..Default::default()
    }
    .plan(settings, &layout, 1.0)
    .map_err(|error| error.to_string())?;
    let bins: Vec<_> = layout
        .static_roles()
        .iter()
        .enumerate()
        .filter(|(_, role)| **role != PhysicalCarrierRole::Excluded)
        .map(|(bin, _)| bin)
        .collect();
    if bins.len() < 2 {
        return Err("Additional loss requires at least two active carrier centers.".into());
    }
    let low = layout
        .frequency_hz(bins[0] as u16)
        .ok_or("Missing low frequency")?;
    let high = layout
        .frequency_hz(*bins.last().ok_or("Missing active bins")? as u16)
        .ok_or("Missing high frequency")?;
    let low_gain = loss
        .gain_db(low, low, high)
        .map_err(|error| error.to_string())?;
    let high_gain = loss
        .gain_db(high, low, high)
        .map_err(|error| error.to_string())?;
    let count = bins.len().min(128);
    let mut points = Vec::with_capacity(count);
    for index in 0..count {
        let bin = bins[index * (bins.len() - 1) / (count - 1)];
        let frequency = layout
            .frequency_hz(bin as u16)
            .ok_or("Missing physical frequency")?;
        let gain_db = loss
            .gain_db(frequency, low, high)
            .map_err(|error| error.to_string())?;
        let iq = plan.response[bin];
        let effective_db = 20.0 * iq[0].hypot(iq[1]).log10();
        points
            .push(json!({"frequency_hz":frequency,"gain_db":gain_db,"effective_db":effective_db}));
    }
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
        - 1.0;
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
        + 1.0;
    let chart:Vec<_>=points.iter().map(|point| json!({"x":(point["frequency_hz"].as_f64().unwrap_or(f64::from(low))-f64::from(low))/f64::from(high-low),
        "additional":(maximum-point["gain_db"].as_f64().unwrap_or(0.0))/(maximum-minimum),
        "effective":(maximum-point["effective_db"].as_f64().unwrap_or(0.0))/(maximum-minimum)})).collect();
    let ticks: Vec<_> = (0..=4)
        .map(|index| {
            let position = f64::from(index) / 4.0;
            json!({"x":position,"label":format!("{:.1}",maximum-position*(maximum-minimum))})
        })
        .collect();
    Ok(
        json!({"error":"","chart":chart,"y_ticks":ticks,"x_ticks":[{"x":0,"label":format!("{:.3} MHz",f64::from(low)/1e6)},{"x":1,"label":format!("{:.3} MHz",f64::from(high)/1e6)}],"points":points,"band":format!("{:.3}–{:.3} MHz",f64::from(low)/1e6,f64::from(high)/1e6),
        "low_gain":format!("{low_gain:.2} dB"),"high_gain":format!("{high_gain:.2} dB"),"tilt":format!("{:.2} dB",high_gain-low_gain)}),
    )
}

/// Restores optional scalar conditioning without resetting legacy additional loss.
/// # Arguments
/// * `value` - Versioned channel_power section or null from a legacy WB.
/// # Returns
/// Validated conditioning; absent controls select unity and relative units.
/// # Errors
/// Rejects unknown fields, unsupported versions and invalid electrical parameters.
pub(crate) fn restore_power(
    value: &Value,
) -> Result<docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning, String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct SavedPower {
        schema_version: u32,
        conditioning: docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning,
    }
    if value.is_null() {
        return Ok(Default::default());
    }
    let saved: SavedPower = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if saved.schema_version != 1 {
        return Err("Unsupported channel power version.".into());
    }
    saved.conditioning.validate().map_err(|e| e.to_string())?;
    Ok(saved.conditioning)
}
/// Encodes accepted scalar conditioning in its own versioned section.
/// # Arguments
/// * `power` - Validated scalar conditioning.
/// # Returns
/// Version1 section, separate from existing channel_model additional loss.
pub(crate) fn saved_power(
    power: &docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning,
) -> Value {
    json!({"schema_version":1,"conditioning":power})
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::signal_processing::channel::additional_loss::{CableModel, LengthUnit};
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
