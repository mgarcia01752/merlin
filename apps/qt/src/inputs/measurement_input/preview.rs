/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded original-capture animation; never constructs a scientific replay plan.
use super::{
    document::Measurement,
    model::{Model, Options},
};
use docsis_ofdm_fec::scenario::measurement_view::measurement_view;
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
        let view = measurement_view(&frame.key.grid, &frame.mer_db, start, count, "RxMER (dB)")
            .map_err(|error| error.to_string())?;
        let mut value = serde_json::to_value(view).map_err(|error| error.to_string())?;
        value["frame_index"] = json!(position);
        value["frame_count"] = json!(captures.len());
        value["source_index"] = json!(source_index);
        value["capture_time"] = json!(capture.epoch());
        value["sha256"] = json!(capture.hash);
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sparse_animation_uses_original_order_and_bounded_viewports() {
        let project: crate::workbench::project_file::Project = serde_json::from_slice(
            include_bytes!("../../../../../examples/workbench/multi-rxmer-2/workbench.merlin.json"),
        )
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
    }
}
