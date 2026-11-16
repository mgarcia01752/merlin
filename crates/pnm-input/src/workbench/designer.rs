/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Saved Designer input adapter; numerical evaluation stays in the Rust engine.
use super::{invalid, unsupported, Stored};
use crate::CaptureLoader;
use docsis_ofdm_fec::{
    api::{ApiResult, NamedCapture, RxMer},
    scenario::{RxMerFrame, ScenarioKey, SourceProvenance},
    signal_processing::rxmer_designer::RxmerRecipe,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
enum Baseline {
    Flat {
        key: ScenarioKey,
        anchor_epoch: u32,
        valid: Vec<bool>,
        baseline_db: f64,
    },
    Imported {
        original: Stored,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RecipeInput {
    merlin_rxmer_designer: u32,
    baseline: Baseline,
    recipe: RxmerRecipe,
    capture_index: usize,
    capture_count: usize,
}
fn is_designer(bytes: &[u8]) -> bool {
    // Presence, including an invalid null/string version, must route to validation.
    serde_json::from_slice::<std::collections::BTreeMap<String, serde::de::IgnoredAny>>(bytes)
        .is_ok_and(|fields| fields.contains_key("merlin_rxmer_designer"))
}
/// Loads a saved Designer envelope or delegates an ordinary capture to its decoder.
/// # Arguments
/// * `input` - Integrity-checked bounded original from the saved workbench.
/// # Returns
/// Owned idle RxMER with exact original bytes and materialized sample provenance.
/// # Errors
/// Rejects invalid versions, nested recipes, corrupt baselines and work limits.
pub(super) fn load(input: &NamedCapture) -> ApiResult<RxMer> {
    if !is_designer(input.bytes()) {
        return CaptureLoader::rxmer(input);
    }
    let record: RecipeInput =
        serde_json::from_slice(input.bytes()).map_err(|error| invalid(error.to_string()))?;
    if record.merlin_rxmer_designer != 1 {
        return Err(unsupported("Unsupported RxMER Designer format version"));
    }
    let synthetic = matches!(&record.baseline, Baseline::Flat { .. });
    let (key, epoch, baseline) = match record.baseline {
        Baseline::Flat {
            key,
            anchor_epoch,
            valid,
            baseline_db,
        } => {
            key.grid.validate()?;
            if valid.len() != key.grid.sample_count
                || !baseline_db.is_finite()
                || !(-30.0..=120.0).contains(&baseline_db)
            {
                return Err(invalid("Invalid Designer flat baseline or grid mask"));
            }
            let values = valid
                .into_iter()
                .map(|present| present.then_some(baseline_db))
                .collect();
            (key, anchor_epoch, values)
        }
        Baseline::Imported { original } => {
            if record.recipe.random_variation.is_some() {
                return Err(invalid("Random variation requires a new flat sequence"));
            }
            let capture = original.decode(&mut 0)?;
            if is_designer(capture.bytes()) {
                return Err(unsupported("Nested Designer baselines are not supported"));
            }
            let baseline = CaptureLoader::rxmer(&capture)?;
            let frame = baseline.frame();
            (frame.key.clone(), frame.capture_time, frame.mer_db.clone())
        }
    };
    if record
        .capture_count
        .checked_mul(key.grid.sample_count)
        .is_none_or(|count| count > 4_000_000)
    {
        return Err(invalid("Designer collection exceeds 4,000,000 values"));
    }
    let values = record.recipe.apply(
        &key.grid,
        &baseline,
        record.capture_index,
        record.capture_count,
    )?;
    let source = SourceProvenance {
        name: input.name().into(),
        sha256: format!("{:x}", Sha256::digest(input.bytes())),
        byte_length: input.bytes().len(),
        capture_time: Some(epoch),
        file_kind: if synthetic {
            "merlin_synthetic_rxmer"
        } else {
            "merlin_designed_rxmer"
        }
        .into(),
        format_major: 1,
        format_minor: 0,
    };
    RxMer::from_frame(
        input,
        RxMerFrame {
            key,
            source,
            capture_time: epoch,
            mer_db: values,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::STANDARD, Engine};
    use serde_json::{json, Value};
    const FLAT: &[u8] = include_bytes!("../../tests/fixtures/designer-flat.json");
    fn named(value: &Value) -> NamedCapture {
        NamedCapture::new("Designer fixture", serde_json::to_vec(value).unwrap()).unwrap()
    }
    fn stored(bytes: &[u8]) -> Value {
        json!({"name":"baseline", "byte_length":bytes.len(),
            "sha256":format!("{:x}",Sha256::digest(bytes)), "base64":STANDARD.encode(bytes)})
    }
    #[test]
    fn ui_flat_recipe_preserves_samples_gaps_and_provenance() {
        let mut value: Value = serde_json::from_slice(FLAT).unwrap();
        value["baseline"]["valid"][100] = json!(false);
        let input = named(&value);
        let capture = load(&input).unwrap();
        let frame = capture.frame();
        let recipe: RxmerRecipe = serde_json::from_value(value["recipe"].clone()).unwrap();
        let valid: Vec<bool> = serde_json::from_value(value["baseline"]["valid"].clone()).unwrap();
        let baseline: Vec<_> = valid.iter().map(|v| v.then_some(48.0)).collect();
        assert_eq!(
            frame.mer_db,
            recipe.apply(&frame.key.grid, &baseline, 1, 1).unwrap()
        );
        assert_eq!(frame.mer_db[100], None);
        assert_eq!(frame.source.file_kind, "merlin_synthetic_rxmer");
        assert_eq!(capture.provenance().original_bytes(), input.bytes());
        assert_eq!(frame.source.sha256, capture.provenance().original_sha256());
        assert_ne!(capture.provenance().decoded_bytes(), input.bytes());
    }
    #[test]
    fn imported_recipe_preserves_baseline_and_applies_once() {
        let original =
            include_bytes!("../../../../examples/measurements/single-channel/rxmer.json");
        let baseline =
            CaptureLoader::rxmer(&NamedCapture::new("baseline", original.to_vec()).unwrap())
                .unwrap();
        let record = json!({"merlin_rxmer_designer":1,"baseline":{"mode":"imported","original":stored(original)},
            "recipe":{},"capture_index":1,"capture_count":1});
        let capture = load(&named(&record)).unwrap();
        assert_eq!(capture.frame().mer_db, baseline.frame().mer_db);
        assert_eq!(capture.frame().key, baseline.frame().key);
        assert_eq!(capture.frame().capture_time, baseline.frame().capture_time);
        assert_eq!(capture.frame().source.file_kind, "merlin_designed_rxmer");
        let mut corrupt = record.clone();
        corrupt["baseline"]["original"]["sha256"] = json!("bad");
        assert!(load(&named(&corrupt)).is_err());
        corrupt = record.clone();
        corrupt["recipe"]["random_variation"] = json!({"standard_deviation_db":1.5,"seed":41});
        assert!(load(&named(&corrupt)).is_err());
        corrupt = record;
        corrupt["baseline"]["original"] = stored(FLAT);
        assert!(load(&named(&corrupt)).is_err());
    }
    #[test]
    fn rejects_versions_masks_positions_bounds_and_invalid_recipes() {
        let value: Value = serde_json::from_slice(FLAT).unwrap();
        for (path, replacement) in [
            ("/merlin_rxmer_designer", json!(2)),
            ("/merlin_rxmer_designer", json!("one")),
            ("/merlin_rxmer_designer", Value::Null),
            ("/capture_index", json!(0)),
            ("/capture_index", json!(2)),
            ("/capture_count", json!(257)),
            ("/baseline/valid", json!([])),
            ("/baseline/baseline_db", json!(121)),
            ("/recipe/random_variation/standard_deviation_db", json!(-1)),
        ] {
            let mut invalid = value.clone();
            *invalid.pointer_mut(path).unwrap() = replacement;
            assert!(load(&named(&invalid)).is_err(), "{path}");
        }
    }
}
