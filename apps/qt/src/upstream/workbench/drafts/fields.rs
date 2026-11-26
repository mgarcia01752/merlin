/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Pure upstream draft descriptors and typed edits over the public source document.
use super::constants::{MAX_DECIMAL_TEXT_CHARACTERS, MAX_DRAFT_CHOICES, MAX_DRAFT_FIELDS};
use merlin::upstream::{ChannelConfig, ConfigurationDocument, Simulation};
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

// Independent application transport policies, not scientific value domains.
const SECTIONS: &[&str] = &[
    "ucd",
    "profiles",
    "grants",
    "source",
    "receiver",
    "channel",
    "preEqualization",
];
const FLAT_ESTIMATOR: &str = "frame_flat_ls";
const LINEAR_ESTIMATOR: &str = "frame_pilot_linear";

/// Frozen source destinations; indices never follow an edited preset selection.
#[derive(Clone, Copy, Debug)]
pub(super) enum Binding {
    Preset,
    Ucd {
        preset: usize,
        field: UcdField,
    },
    Iuc,
    Profile {
        preset: usize,
        burst: usize,
        group: usize,
        field: ProfileField,
    },
    Grant(GrantField),
    SourceSeed,
    Receiver(ReceiverField),
    Channel(ChannelField),
    NoiseSeed(usize),
    PreEqInitialId,
}

/// Existing selected-preset channel scalars, with their original public widths.
#[derive(Clone, Copy, Debug)]
pub(super) enum UcdField {
    Prefix,
    Rolloff,
    Spacing,
    CarrierZero,
    Symbols,
    Seed,
    Bandwidth,
}

/// Existing source profile-group scalars; no new group or pilot map is generated.
#[derive(Clone, Copy, Debug)]
pub(super) enum ProfileField {
    Modulation,
    Pilot,
    Additional,
}

/// Explicit synthetic scheduling values, independent of represented raw MAP controls.
#[derive(Clone, Copy, Debug)]
pub(super) enum GrantField {
    Id,
    Epoch,
    FirstFrame,
    Frames,
    Minislots,
}

/// Existing configured receiver engineering policies.
#[derive(Clone, Copy, Debug)]
pub(super) enum ReceiverField {
    Estimator,
    Gap,
    Variance,
    Floor,
    Iterations,
}

/// Existing synthetic channel scalars; taps stay in the complete source document.
#[derive(Clone, Copy, Debug)]
pub(super) enum ChannelField {
    Delay,
    Variance,
    Phase,
    Frequency,
}

/// App-private projection whose actual capacities are admitted by the draft owner.
#[derive(Debug)]
pub(super) struct Fields {
    /// Fixed destinations, in exactly the descriptor order.
    pub(super) bindings: Vec<Binding>,
    /// Plain descriptor/value transport; never a serialized scientific document.
    pub(super) presentation: Value,
}

#[derive(Default)]
struct Builder {
    bindings: Vec<Binding>,
    descriptors: Vec<Value>,
    values: Map<String, Value>,
}

/// Projects one supported section from an already accepted immutable simulation.
///
/// # Arguments
/// * `simulation` - Public accepted source; borrowed without running or changing it.
/// * `section` - Exactly ucd, profiles, grants, source, receiver, channel or preEqualization.
///
/// # Returns
/// Original-index bindings and plain, bounded descriptors with lossless string values.
/// Absent Pre-Eq has no editable fields and an explicit unavailable summary.
///
/// # Errors
/// Rejects unknown sections, missing accepted documents/configuration or transport overflow.
pub(super) fn build(simulation: &Simulation, section: &str) -> Result<Fields, String> {
    if !SECTIONS.contains(&section) {
        return Err("Unknown upstream configuration section.".into());
    }
    let document = simulation
        .document()
        .ok_or("Load an upstream configuration document before editing.")?;
    let configuration = simulation
        .configuration()
        .ok_or("Accepted upstream configuration is unavailable.")?;
    let preset = document
        .presets
        .iter()
        .position(|value| value.id == document.selected_preset)
        .ok_or("The accepted preset is unavailable.")?;
    let selected = &document.presets[preset];
    let run = &document.simulation;
    let mut builder = Builder::default();
    let mut summary = "Accepted native configuration; edits require full public admission.";
    match section {
        "ucd" => {
            builder.choice(
                Binding::Preset,
                "selected_preset",
                "Execution preset",
                &document.selected_preset,
                document
                    .presets
                    .iter()
                    .map(|value| (value.id.clone(), value.id.clone()))
                    .collect(),
            )?;
            let channel = &selected.channel;
            for (field, name, label, value, maximum) in [
                (
                    UcdField::Prefix,
                    "cyclic_prefix_samples",
                    "Cyclic prefix (samples)",
                    channel.cyclic_prefix_samples.to_string(),
                    usize::MAX.to_string().len(),
                ),
                (
                    UcdField::Rolloff,
                    "rolloff_samples",
                    "Rolloff (samples)",
                    channel.rolloff_samples.to_string(),
                    usize::MAX.to_string().len(),
                ),
                (
                    UcdField::Spacing,
                    "subcarrier_spacing_khz",
                    "Subcarrier spacing (kHz)",
                    channel.subcarrier_spacing_khz.to_string(),
                    u8::MAX.to_string().len(),
                ),
                (
                    UcdField::CarrierZero,
                    "subcarrier_zero_hz",
                    "Physical carrier-zero frequency (Hz)",
                    channel.subcarrier_zero_hz.to_string(),
                    u32::MAX.to_string().len(),
                ),
                (
                    UcdField::Symbols,
                    "frame_symbols",
                    "Physical symbols per frame",
                    channel.frame_symbols.to_string(),
                    usize::MAX.to_string().len(),
                ),
                (
                    UcdField::Seed,
                    "randomization_seed",
                    "Channel randomization register seed",
                    channel.randomization_seed.to_string(),
                    u32::MAX.to_string().len(),
                ),
                (
                    UcdField::Bandwidth,
                    "encompassed_bandwidth_hz",
                    "Encompassed bandwidth (Hz)",
                    channel.encompassed_bandwidth_hz.to_string(),
                    u32::MAX.to_string().len(),
                ),
            ] {
                builder.integer(
                    Binding::Ucd { preset, field },
                    name,
                    &format!("{}: {label}", selected.id),
                    value,
                    maximum,
                )?;
            }
        }
        "profiles" => {
            let choices = configuration
                .us_configuration
                .profiles()
                .iter()
                .filter(|profile| selected.assignment.permits(profile.iuc()))
                .map(|profile| (profile.iuc().to_string(), format!("IUC {}", profile.iuc())))
                .collect();
            builder.choice(
                Binding::Iuc,
                "selected_iuc",
                "Scheduled data IUC",
                &run.selected_iuc.to_string(),
                choices,
            )?;
            for (burst, descriptor) in selected.channel.bursts.iter().enumerate() {
                for (group, value) in descriptor
                    .data_profile
                    .as_deref()
                    .unwrap_or_default()
                    .iter()
                    .enumerate()
                {
                    for (field, name, label, value) in [
                        (
                            ProfileField::Modulation,
                            "modulation_index",
                            "Modulation index",
                            value.modulation_index,
                        ),
                        (
                            ProfileField::Pilot,
                            "pilot_pattern",
                            "Pilot pattern",
                            value.pilot_pattern,
                        ),
                        (
                            ProfileField::Additional,
                            "additional_minislots",
                            "Additional minislots",
                            value.additional_minislots,
                        ),
                    ] {
                        builder.integer(
                            Binding::Profile {
                                preset,
                                burst,
                                group,
                                field,
                            },
                            name,
                            &format!(
                                "{} / IUC {} / source group {group}: {label}",
                                selected.id, descriptor.iuc
                            ),
                            value.to_string(),
                            u8::MAX.to_string().len(),
                        )?;
                    }
                }
            }
        }
        "grants" => {
            for (field, name, label, value, maximum) in [
                (
                    GrantField::Id,
                    "grant_id",
                    "First synthetic grant ID",
                    run.grant_id.to_string(),
                    u64::MAX.to_string().len(),
                ),
                (
                    GrantField::Epoch,
                    "epoch_sample",
                    "First-prefix origin (sample ticks)",
                    run.epoch_sample.to_string(),
                    u64::MAX.to_string().len(),
                ),
                (
                    GrantField::FirstFrame,
                    "first_frame",
                    "First physical frame ID",
                    run.first_frame.to_string(),
                    u64::MAX.to_string().len(),
                ),
                (
                    GrantField::Frames,
                    "frames",
                    "Physical frames per grant",
                    run.frames.to_string(),
                    usize::MAX.to_string().len(),
                ),
            ] {
                builder.integer(Binding::Grant(field), name, label, value, maximum)?;
            }
            let slots = configuration
                .us_configuration
                .selected_profile()
                .minislots();
            let choices = slots
                .iter()
                .enumerate()
                .map(|(index, slot)| {
                    (
                        index.to_string(),
                        format!("Minislot {index}: physical carrier {}", slot.first_carrier),
                    )
                })
                .collect();
            builder.selection(
                Binding::Grant(GrantField::Minislots),
                "minislots",
                "Scheduled minislots (source order)",
                run.minislots.iter().map(ToString::to_string).collect(),
                choices,
                slots.len(),
            )?;
        }
        "source" => {
            builder.integer(
                Binding::SourceSeed,
                "source_seed",
                "PRBS15 source seed",
                run.source_seed.to_string(),
                u16::MAX.to_string().len(),
            )?;
        }
        "receiver" => {
            builder.choice(
                Binding::Receiver(ReceiverField::Estimator),
                "estimator",
                "Received-pilot estimator",
                &run.estimator,
                vec![
                    (FLAT_ESTIMATOR.into(), "Frame-flat LS".into()),
                    (LINEAR_ESTIMATOR.into(), "Frame pilot-linear".into()),
                ],
            )?;
            builder.integer(
                Binding::Receiver(ReceiverField::Gap),
                "maximum_pilot_gap",
                "Maximum pilot bracket gap (physical carriers)",
                run.maximum_pilot_gap.to_string(),
                usize::MAX.to_string().len(),
            )?;
            builder.decimal(
                Binding::Receiver(ReceiverField::Variance),
                "reference_variance",
                "Reference complex variance (confidence assumption)",
                run.reference_variance,
            )?;
            builder.decimal(
                Binding::Receiver(ReceiverField::Floor),
                "equalizer_power_floor",
                "Equalizer response-power floor",
                run.equalizer_power_floor,
            )?;
            builder.integer(
                Binding::Receiver(ReceiverField::Iterations),
                "ldpc_iterations",
                "LDPC iteration limit",
                run.ldpc_iterations.to_string(),
                usize::MAX.to_string().len(),
            )?;
        }
        "channel" => {
            let channel = &configuration.channel;
            builder.integer(
                Binding::Channel(ChannelField::Delay),
                "delay_samples",
                "Synthetic channel delay (sample ticks)",
                channel.delay_samples.to_string(),
                usize::MAX.to_string().len(),
            )?;
            builder.decimal(
                Binding::Channel(ChannelField::Variance),
                "noise_variance",
                "Complex sample-noise variance",
                channel.noise_variance,
            )?;
            builder.decimal(
                Binding::Channel(ChannelField::Phase),
                "phase_radians",
                "First-tick phase (radians)",
                channel.phase_radians,
            )?;
            builder.decimal(
                Binding::Channel(ChannelField::Frequency),
                "frequency_offset_hz",
                "Frequency offset (Hz)",
                channel.frequency_offset_hz,
            )?;
            for (index, value) in channel.noise_seed.iter().enumerate() {
                builder.integer(
                    Binding::NoiseSeed(index),
                    "noise_seed",
                    &format!("Synthetic noise seed byte {index}"),
                    value.to_string(),
                    u8::MAX.to_string().len(),
                )?;
            }
        }
        "preEqualization" => {
            if let Some(value) = &run.synthetic_pre_eq {
                builder.integer(
                    Binding::PreEqInitialId,
                    "initial_id",
                    "Initial pre-equalizer bank ID",
                    value.initial_id.to_string(),
                    u64::MAX.to_string().len(),
                )?;
            } else {
                summary = "No explicit synthetic Pre-Eq plan is loaded; native unity correction remains unchanged.";
            }
        }
        _ => unreachable!("section was checked before reading the public owner"),
    }
    Ok(Fields {
        bindings: builder.bindings,
        presentation: json!({"section":section,"options":{"fields":builder.descriptors},"values":builder.values,"summary":summary}),
    })
}

/// Merges changed fields into a candidate without retargeting frozen destinations.
///
/// # Arguments
/// * `document` - Caller-owned temporary candidate; its original source Arc is preserved.
/// * `fields` - Captured native bindings from this draft's original accepted base.
/// * `changes` - Object containing only opaque descriptor keys and string/string-array values.
///
/// # Returns
/// Unit after all structural/type checks and edits succeed together. Untouched fields,
/// optional values, unselected inventory and original source bytes remain unchanged.
/// The caller must subsequently run complete public scientific admission before adoption.
///
/// # Errors
/// Unknown keys/sections, absent destinations, malformed or overflowing scalar values,
/// nonfinite decimals, invalid choices and excessive/duplicate selections preserve the candidate.
pub(super) fn merge(
    document: &mut ConfigurationDocument,
    fields: &Fields,
    changes: &Value,
) -> Result<(), String> {
    let changes = changes
        .as_object()
        .ok_or("Draft changes must be an object.")?;
    let descriptors = fields
        .presentation
        .pointer("/options/fields")
        .and_then(Value::as_array)
        .ok_or("Draft field metadata is unavailable.")?;
    if fields.bindings.len() > MAX_DRAFT_FIELDS
        || descriptors.len() != fields.bindings.len()
        || changes.len() > fields.bindings.len()
    {
        return Err("Draft field transport is inconsistent or excessive.".into());
    }
    let section = fields
        .presentation
        .get("section")
        .and_then(Value::as_str)
        .ok_or("Draft section is unavailable.")?;
    if !SECTIONS.contains(&section) {
        return Err("Unknown upstream configuration section.".into());
    }
    if changes.is_empty() {
        return Ok(());
    }
    // This finite temporary clone makes parse/index failure atomic even for callers
    // that reuse their preparation candidate. Clone retains immutable original bytes.
    let mut candidate = document.clone();
    for (key, value) in changes {
        let index = key
            .strip_prefix("field/")
            .and_then(|index| index.parse::<usize>().ok())
            .filter(|index| key == &format!("field/{index}"))
            .ok_or("Unknown draft field key.")?;
        let binding = *fields
            .bindings
            .get(index)
            .ok_or("Unknown draft field key.")?;
        if binding.section() != section {
            return Err("Draft field belongs to another section.".into());
        }
        let descriptor = &descriptors[index];
        if descriptor.get("key").and_then(Value::as_str) != Some(key.as_str())
            || descriptor.get("editable") != Some(&Value::Bool(true))
        {
            return Err("Draft field is unavailable for editing.".into());
        }
        check_value(descriptor, value)?;
        apply(&mut candidate, binding, value).map_err(|error| format!("{key}: {error}"))?;
    }
    *document = candidate;
    Ok(())
}

impl Builder {
    fn add(
        &mut self,
        binding: Binding,
        field: &str,
        label: &str,
        value: Value,
        mut descriptor: Value,
    ) -> Result<(), String> {
        if self.bindings.len() >= MAX_DRAFT_FIELDS {
            return Err("Upstream draft exceeds the field transport limit.".into());
        }
        let key = format!("field/{}", self.bindings.len());
        descriptor["key"] = key.clone().into();
        descriptor["field"] = field.into();
        descriptor["label"] = label.into();
        descriptor["editable"] = true.into();
        self.bindings.push(binding);
        self.descriptors.push(descriptor);
        self.values.insert(key, value);
        Ok(())
    }

    fn integer(
        &mut self,
        binding: Binding,
        field: &str,
        label: &str,
        value: String,
        maximum: usize,
    ) -> Result<(), String> {
        self.add(
            binding,
            field,
            label,
            value.into(),
            json!({"kind":"integer","max_length":maximum}),
        )
    }

    fn decimal(
        &mut self,
        binding: Binding,
        field: &str,
        label: &str,
        value: f64,
    ) -> Result<(), String> {
        if !value.is_finite() {
            return Err("Accepted decimal field is not finite.".into());
        }
        self.add(
            binding,
            field,
            label,
            value.to_string().into(),
            json!({"kind":"decimal","max_length":MAX_DECIMAL_TEXT_CHARACTERS}),
        )
    }

    fn choice(
        &mut self,
        binding: Binding,
        field: &str,
        label: &str,
        value: &str,
        choices: Vec<(String, String)>,
    ) -> Result<(), String> {
        if choices.len() > MAX_DRAFT_CHOICES {
            return Err("Upstream draft exceeds the choice transport limit.".into());
        }
        let choices: Vec<_> = choices
            .into_iter()
            .map(|(value, label)| json!({"value":value,"label":label}))
            .collect();
        self.add(
            binding,
            field,
            label,
            value.into(),
            json!({"kind":"choice","choices":choices}),
        )
    }

    fn selection(
        &mut self,
        binding: Binding,
        field: &str,
        label: &str,
        values: Vec<String>,
        choices: Vec<(String, String)>,
        maximum: usize,
    ) -> Result<(), String> {
        if choices.len() > MAX_DRAFT_CHOICES || values.len() > MAX_DRAFT_CHOICES {
            return Err("Upstream draft exceeds the selection transport limit.".into());
        }
        let choices: Vec<_> = choices
            .into_iter()
            .map(|(value, label)| json!({"value":value,"label":label}))
            .collect();
        self.add(
            binding,
            field,
            label,
            json!(values),
            json!({"kind":"selection","choices":choices,"max_items":maximum}),
        )
    }
}

impl Binding {
    fn section(self) -> &'static str {
        match self {
            Self::Preset | Self::Ucd { .. } => "ucd",
            Self::Iuc | Self::Profile { .. } => "profiles",
            Self::Grant(_) => "grants",
            Self::SourceSeed => "source",
            Self::Receiver(_) => "receiver",
            Self::Channel(_) | Self::NoiseSeed(_) => "channel",
            Self::PreEqInitialId => "preEqualization",
        }
    }
}

fn check_value(descriptor: &Value, value: &Value) -> Result<(), String> {
    let kind = descriptor
        .get("kind")
        .and_then(Value::as_str)
        .ok_or("Draft field kind is unavailable.")?;
    if kind == "selection" {
        let values = value
            .as_array()
            .ok_or("Draft selection must contain strings.")?;
        let maximum = descriptor
            .get("max_items")
            .and_then(Value::as_u64)
            .ok_or("Draft selection bound is unavailable.")?;
        if values.len() as u64 > maximum {
            return Err("Draft selection exceeds its supplied count limit.".into());
        }
        let mut seen = BTreeSet::new();
        for value in values {
            let text = value
                .as_str()
                .ok_or("Draft selection must contain strings.")?;
            if !seen.insert(text) || !has_choice(descriptor, text) {
                return Err("Draft selection contains an unknown or duplicate choice.".into());
            }
        }
        return Ok(());
    }
    let text = value.as_str().ok_or("Draft values must be strings.")?;
    if kind == "choice" {
        if !has_choice(descriptor, text) {
            return Err("Draft choice is not in the supplied inventory.".into());
        }
    } else if matches!(kind, "integer" | "decimal") {
        let maximum = descriptor
            .get("max_length")
            .and_then(Value::as_u64)
            .ok_or("Draft text bound is unavailable.")?;
        if text.is_empty() || text.len() as u64 > maximum {
            return Err("Draft numeric text exceeds its supplied length limit.".into());
        }
    } else {
        return Err("Unknown draft field kind.".into());
    }
    Ok(())
}

fn has_choice(descriptor: &Value, text: &str) -> bool {
    descriptor
        .get("choices")
        .and_then(Value::as_array)
        .is_some_and(|choices| {
            choices
                .iter()
                .any(|choice| choice.get("value").and_then(Value::as_str) == Some(text))
        })
}

fn unsigned<T: std::str::FromStr>(value: &Value) -> Result<T, String> {
    let text = value.as_str().ok_or("Expected unsigned integer text.")?;
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("Expected unsigned integer text.".into());
    }
    text.parse()
        .map_err(|_| "Unsigned integer exceeds its destination type.".into())
}

fn decimal(value: &Value) -> Result<f64, String> {
    let parsed: f64 = value
        .as_str()
        .ok_or("Expected decimal text.")?
        .parse()
        .map_err(|_| "Expected decimal text.")?;
    if !parsed.is_finite() {
        return Err("Decimal value must be finite.".into());
    }
    Ok(parsed)
}

fn apply(
    document: &mut ConfigurationDocument,
    binding: Binding,
    value: &Value,
) -> Result<(), String> {
    let run = &mut document.simulation;
    match binding {
        Binding::Preset => {
            document.selected_preset = value.as_str().ok_or("Expected preset identity.")?.into()
        }
        Binding::Ucd { preset, field } => {
            let channel = &mut document
                .presets
                .get_mut(preset)
                .ok_or("Original preset is unavailable.")?
                .channel;
            match field {
                UcdField::Prefix => channel.cyclic_prefix_samples = unsigned(value)?,
                UcdField::Rolloff => channel.rolloff_samples = unsigned(value)?,
                UcdField::Spacing => channel.subcarrier_spacing_khz = unsigned(value)?,
                UcdField::CarrierZero => channel.subcarrier_zero_hz = unsigned(value)?,
                UcdField::Symbols => channel.frame_symbols = unsigned(value)?,
                UcdField::Seed => channel.randomization_seed = unsigned(value)?,
                UcdField::Bandwidth => channel.encompassed_bandwidth_hz = unsigned(value)?,
            }
        }
        Binding::Iuc => run.selected_iuc = unsigned(value)?,
        Binding::Profile {
            preset,
            burst,
            group,
            field,
        } => {
            let value_group = document
                .presets
                .get_mut(preset)
                .and_then(|preset| preset.channel.bursts.get_mut(burst))
                .and_then(|burst| burst.data_profile.as_mut())
                .and_then(|groups| groups.get_mut(group))
                .ok_or("Original profile group is unavailable.")?;
            match field {
                ProfileField::Modulation => value_group.modulation_index = unsigned(value)?,
                ProfileField::Pilot => value_group.pilot_pattern = unsigned(value)?,
                ProfileField::Additional => value_group.additional_minislots = unsigned(value)?,
            }
        }
        Binding::Grant(field) => match field {
            GrantField::Id => run.grant_id = unsigned(value)?,
            GrantField::Epoch => run.epoch_sample = unsigned(value)?,
            GrantField::FirstFrame => run.first_frame = unsigned(value)?,
            GrantField::Frames => run.frames = unsigned(value)?,
            GrantField::Minislots => {
                run.minislots = value
                    .as_array()
                    .ok_or("Expected explicit minislot selection.")?
                    .iter()
                    .map(unsigned)
                    .collect::<Result<_, _>>()?
            }
        },
        Binding::SourceSeed => run.source_seed = unsigned(value)?,
        Binding::Receiver(field) => match field {
            ReceiverField::Estimator => {
                run.estimator = value.as_str().ok_or("Expected estimator identity.")?.into()
            }
            ReceiverField::Gap => run.maximum_pilot_gap = unsigned(value)?,
            ReceiverField::Variance => run.reference_variance = decimal(value)?,
            ReceiverField::Floor => run.equalizer_power_floor = decimal(value)?,
            ReceiverField::Iterations => run.ldpc_iterations = unsigned(value)?,
        },
        Binding::Channel(field) => {
            let channel = run.channel.get_or_insert_with(ChannelConfig::default);
            match field {
                ChannelField::Delay => channel.delay_samples = unsigned(value)?,
                ChannelField::Variance => channel.noise_variance = decimal(value)?,
                ChannelField::Phase => channel.phase_radians = decimal(value)?,
                ChannelField::Frequency => channel.frequency_offset_hz = decimal(value)?,
            }
        }
        Binding::NoiseSeed(index) => {
            let channel = run.channel.get_or_insert_with(ChannelConfig::default);
            *channel
                .noise_seed
                .get_mut(index)
                .ok_or("Original noise seed byte is unavailable.")? = unsigned(value)?;
        }
        Binding::PreEqInitialId => {
            run.synthetic_pre_eq
                .as_mut()
                .ok_or("No explicit synthetic Pre-Eq plan is loaded.")?
                .initial_id = unsigned(value)?
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "fields/tests.rs"]
mod tests;
