/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Channel observation conversion; the public Rust API owns all calibration and scalar math.
use super::*;

#[pymethods]
impl PyFullLoopResult {
    /// Reads actual retained PSD in accepted electrical or explicit relative units.
    /// # Arguments
    /// * `stage`, `snapshot` - Existing actual time-stage selection and retained event.
    /// * `max_averages` - Existing Welch limit, 1..32.
    /// * `start`, `count` - Original FFT viewport; zero count selects the remaining span.
    /// # Returns
    /// Owned PSD, zoom-independent channel power and original source/sample metadata.
    /// Numerical work releases the GIL; no source, input or result is changed.
    /// # Errors
    /// Shared missing/stage/grid/work/window/calibration failures raise ApiError.
    #[pyo3(
        text_signature = "($self, stage='tx.overlap_add.samples', snapshot=0, max_averages=32, start=0, count=0)"
    )]
    #[pyo3(signature=(stage="tx.overlap_add.samples",snapshot=0,max_averages=api::FullLoopSpectrumQuery::default().max_averages,start=0,count=0))]
    fn calibrated_spectrum(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
        max_averages: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let stage = crate::physical::stage_id(stage)?;
        let text = py
            .detach(|| {
                let view = self.0.calibrated_spectrum(api::FullLoopSpectrumQuery {
                    stage,
                    snapshot,
                    max_averages,
                    start,
                    count,
                })?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads a fixed actual channel boundary or the exact amplifier-only scalar view.
    /// # Arguments
    /// * `tap` - before_amplifier, after_amplifier or receiver_input; Rust fixes the source stage.
    /// * `snapshot`, `max_averages` - Actual retained event and existing 1..32 Welch work limit.
    /// * `start`, `count` - Original FFT viewport; zero count selects the remaining span.
    /// # Returns
    /// Owned unit-labelled PSD/power, original context and explicit scalar derivation.
    /// Only receiver_input includes actual modeled CPA/H/noise; work releases the GIL.
    /// # Errors
    /// Unknown taps and shared sample/work/window/calibration failures raise ApiError.
    #[pyo3(
        text_signature = "($self, tap='before_amplifier', snapshot=0, max_averages=32, start=0, count=0)"
    )]
    #[pyo3(signature=(tap="before_amplifier",snapshot=0,max_averages=api::FullLoopSpectrumQuery::default().max_averages,start=0,count=0))]
    fn channel_spectrum(
        &self,
        py: Python<'_>,
        tap: &str,
        snapshot: usize,
        max_averages: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let tap = match tap {
            "before_amplifier" => api::FullLoopChannelTap::BeforeAmplifier,
            "after_amplifier" => api::FullLoopChannelTap::AfterAmplifier,
            "receiver_input" => api::FullLoopChannelTap::ReceiverInput,
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidIdentifier,
                    "Unknown channel spectrum tap",
                )))
            }
        };
        let stage = match tap {
            api::FullLoopChannelTap::ReceiverInput => api::FullLoopStage::ChannelOutput,
            _ => api::FullLoopStage::TxOverlap,
        };
        let text = py
            .detach(|| {
                let view = self.0.channel_spectrum(
                    tap,
                    api::FullLoopSpectrumQuery {
                        stage,
                        snapshot,
                        max_averages,
                        start,
                        count,
                    },
                )?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads actual TX physical bins before or after the accepted scalar amplifier alone.
    /// # Arguments
    /// * `boundary` - before or after; no physical channel/receiver is synthesized.
    /// * `snapshot` - Actual retained TX physical-bin event.
    /// * `start`, `count` - Existing physical viewport; zero count selects remaining bins.
    /// # Returns
    /// Owned original roles/context/missing history and direct or scalar-derived I/Q/power.
    /// Scientific work releases the GIL; edits cannot alter the result's retained source.
    /// # Errors
    /// Unknown boundary and shared missing/grid/window/overflow failures raise ApiError.
    #[pyo3(signature=(boundary="before",snapshot=0,start=0,count=0))]
    fn amplifier_bins(
        &self,
        py: Python<'_>,
        boundary: &str,
        snapshot: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let boundary = match boundary {
            "before" => api::AmplifierBoundary::Before,
            "after" => api::AmplifierBoundary::After,
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidIdentifier,
                    "Unknown amplifier boundary",
                )))
            }
        };
        let text = py
            .detach(|| {
                let view = self.0.amplifier_bins(boundary, snapshot, start, count)?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
