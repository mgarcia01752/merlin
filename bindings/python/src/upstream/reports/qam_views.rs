/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Public Rust QAM normalization/reference and original-coordinate association dispatch.
use super::*;

#[pymethods]
impl PyBatchReport {
    /// Queries compatible actual unit-reference QAM with authoritative upstream labels.
    /// # Arguments
    /// * `tap_name` - TxGrid or RxEqualized; other raw planes have no unit-reference overlay.
    /// * `loading` - Original payload bits; zero selects known pilots with BPSK references.
    /// * `maximum_points`, `maximum_references` - Independent actual/reference copy ceilings.
    /// # Returns
    /// Owned Rust-derived dictionary preserving original raw points, recorded role normalization,
    /// canonical labels, units and capped availability. Rust work releases the GIL.
    /// # Errors
    /// Checked tap/conversion, unavailable/unsupported-plane/order, budget and numerical failures.
    #[pyo3(
        text_signature = "($self, tap_name='RxEqualized', loading=4, maximum_points=4096, maximum_references=4096)"
    )]
    #[pyo3(signature=(tap_name="RxEqualized", loading=api::QamConstellationRequest::default().loading, maximum_points=api::MAX_QAM_QUERY_POINTS, maximum_references=api::MAX_QAM_CONSTELLATION_REFERENCES))]
    fn qam_constellation(
        &self,
        py: Python<'_>,
        tap_name: &str,
        loading: u8,
        maximum_points: usize,
        maximum_references: usize,
    ) -> PyResult<Py<PyAny>> {
        let request = api::QamConstellationRequest {
            tap: tap(tap_name)?,
            loading,
            maximum_points,
            maximum_references,
        };
        snapshot(py, || {
            let view = self.0.qam_constellation(request)?;
            serde_json::to_string(&view).map_err(api::ApiError::Serialization)
        })
    }

    /// Matches two actual retained stages by complete original physical identities in Rust.
    /// # Arguments
    /// * `left`, `right` - Actual retained stage names, with independent raw reference planes.
    /// * `loading` - Optional original payload loading; zero selects pilots without payload relabelling.
    /// * `maximum_pairs` - Positive copied-pair application ceiling, independent of single-point queries.
    /// # Returns
    /// Owned original pairs and whole retained selected/matched/unpaired counts in left order.
    /// Rust work releases the GIL; no native association, channel correction or source replay.
    /// # Errors
    /// Checked tap/conversion, unretained-stage, budget and duplicate/nonfinite coordinate failures.
    #[pyo3(
        text_signature = "($self, left='TxGrid', right='RxEqualized', loading=None, maximum_pairs=4096)"
    )]
    #[pyo3(signature=(left="TxGrid", right="RxEqualized", loading=None, maximum_pairs=api::MAX_QAM_PAIR_POINTS))]
    fn qam_pairing(
        &self,
        py: Python<'_>,
        left: &str,
        right: &str,
        loading: Option<u8>,
        maximum_pairs: usize,
    ) -> PyResult<Py<PyAny>> {
        let request = api::QamPairRequest {
            left: tap(left)?,
            right: tap(right)?,
            loading,
            maximum_pairs,
        };
        snapshot(py, || {
            let view = self.0.qam_pairing(request)?;
            serde_json::to_string(&view).map_err(api::ApiError::Serialization)
        })
    }
}

fn snapshot(
    py: Python<'_>,
    query: impl FnOnce() -> api::ApiResult<String> + Send,
) -> PyResult<Py<PyAny>> {
    let text = py.detach(query).map_err(error)?;
    crate::object_from_json(py, &text)
}
