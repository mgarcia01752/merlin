/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded original-capture animation; never constructs a scientific replay plan.
use super::{
    document::Measurement,
    model::{Model, Options},
};
use merlin::downstream::{MeasurementViewport, NamedCapture, RxMer};
use serde_json::{json, Value};

impl Model {
    /// Returns one checked original at a display-frame position without copying a collection.
    /// # Arguments
    /// * `options` - Draft selected identities; missing means legacy all.
    /// * `position` - Zero-based position within selected captures in displayed order.
    /// * `start`, `count` - Carrier viewport; zero count requests full span.
    /// # Returns
    /// At most 1024 chart buckets, original identity/time and selected-frame count.
    /// # Errors
    /// Rejects invalid selections, positions, types or viewport bounds.
    pub(crate) fn preview_frame(
        &self,
        options: &Options,
        position: usize,
        start: usize,
        count: usize,
    ) -> Result<Value, String> {
        let captures = options.selected(&self.draft_rx)?;
        let capture = captures
            .get(position)
            .ok_or("Choose an available selected capture.")?;
        let Measurement::Rxmer(frame) = &capture.measurement else {
            return Err("Expected RxMER capture.".into());
        };
        let source_index = self
            .draft_rx
            .iter()
            .position(|source| source.hash == capture.hash)
            .ok_or("Selected capture is unavailable.")?;

        // Preserve the selected capture's exact source separately from decoded/Designer data.
        // The existing public owner validates one immutable frame, never a replay collection.
        let original = NamedCapture::new(&capture.name, capture.original.as_ref().clone())
            .map_err(|cause| cause.message().to_owned())?;
        let input = RxMer::from_frame(&original, frame.as_ref().clone())
            .map_err(|cause| cause.message().to_owned())?;
        let preview = input
            .preview(MeasurementViewport { start, count })
            .map_err(|cause| cause.message().to_owned())?;
        let mut value = serde_json::to_value(preview.view).map_err(|error| error.to_string())?;
        value["frame_index"] = json!(position);
        value["frame_count"] = json!(captures.len());
        value["source_index"] = json!(source_index);
        value["capture_time"] = json!(capture.epoch());
        value["synthetic_time"] = json!(frame.source.file_kind == "merlin_synthetic_rxmer");
        value["sha256"] = json!(capture.hash);
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sparse_animation_uses_original_order_and_bounded_viewports() {
        let project: crate::downstream::workbench::project_file::Project =
            serde_json::from_slice(include_bytes!(
                "../../../../../../examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json"
            ))
            .unwrap();
        let (_, editors) = project.prepare().unwrap();
        let model = editors.measurements;
        let options = Options {
            selected_captures: Some(vec![
                model.draft_rx[5].hash.clone(),
                model.draft_rx[1].hash.clone(),
            ]),
            ..Default::default()
        };
        let before = [1, 5].map(|index| {
            let capture = &model.draft_rx[index];
            let Measurement::Rxmer(frame) = &capture.measurement else {
                panic!("RxMER fixture required")
            };
            (
                capture.original.clone(),
                capture.hash.clone(),
                capture.decoded_hash.clone(),
                serde_json::to_value(frame.as_ref()).unwrap(),
            )
        });
        let first = model.preview_frame(&options, 0, 0, 0).unwrap();
        assert_eq!(first["frame_count"], 2);
        assert_eq!(first["source_index"], 1);
        assert_eq!(first["sha256"], model.draft_rx[1].hash);
        assert!(first["points"].as_array().unwrap().len() <= 1024);
        let second = model.preview_frame(&options, 1, 10, 100).unwrap();
        assert_eq!(second["source_index"], 5);
        assert_eq!(second["count"], 100);
        assert!(model.preview_frame(&options, 2, 0, 0).is_err());
        assert!(model.preview_frame(&options, 0, usize::MAX, 10).is_err());
        assert_eq!(model.accepted_rx.len(), 132);
        for (index, (original, hash, decoded, frame)) in [1, 5].into_iter().zip(before) {
            let capture = &model.draft_rx[index];
            let Measurement::Rxmer(current) = &capture.measurement else {
                panic!("RxMER fixture required")
            };
            assert_eq!(capture.original.as_ref(), original.as_ref());
            assert_eq!(capture.hash, hash);
            assert_eq!(capture.decoded_hash, decoded);
            assert_eq!(serde_json::to_value(current.as_ref()).unwrap(), frame);
        }
    }
}
