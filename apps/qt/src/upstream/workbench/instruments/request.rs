/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Private direction transport selects existing typed public observation operations.
use super::{
    common, Display, FecRequest, QamRequest, ResponseMetric, ResponseRequest, ResponseSource,
    SpectrumRequest,
};
use crate::{
    shared::presentation::constants::MAX_SCATTER_PLOT_POINTS,
    shell::{Retained, RetentionBudget},
    upstream::UpstreamController,
};
use merlin::upstream::{
    ApiError, ConfiguredResponseRequest, ConfiguredResponseSource, PhyError,
    ReceiverEstimateRequest, Tap,
};

/// Complete finite selector owned by an existing passive preparation worker.
pub(in crate::upstream::workbench) enum Request {
    Qam(QamRequest),
    Spectrum(SpectrumRequest),
    Response(ResponseRequest),
    Fec(FecRequest),
}

impl Request {
    /// Parses one explicit instrument destination without accepting scientific data.
    /// # Arguments
    /// * `kind`, `selection` - Compiled destination and strict finite selector JSON.
    /// # Returns
    /// A typed selector; caller validates captured context and transport bytes.
    /// # Errors
    /// Unknown destinations, unknown fields and invalid scalar representations.
    pub(in crate::upstream::workbench) fn parse(
        kind: &str,
        selection: &str,
    ) -> Result<Self, String> {
        match kind {
            "qam" => serde_json::from_str(selection).map(Self::Qam),
            "spectrum" => serde_json::from_str(selection).map(Self::Spectrum),
            "response" => serde_json::from_str(selection).map(Self::Response),
            "fec" => serde_json::from_str(selection).map(Self::Fec),
            _ => return Err("Choose a supported upstream instrument.".into()),
        }
        .map_err(|error| error.to_string())
    }

    /// Selects a passive initial request from actual retained report tap metadata.
    /// # Arguments
    /// * `kind`, `controller` - Existing destination and original idle owner.
    /// # Returns
    /// A finite request; unavailable history is never regenerated or executed.
    /// # Errors
    /// No actual report, a foreign/unsupported destination or absent controller.
    pub(in crate::upstream::workbench) fn initial(
        kind: &str,
        controller: &UpstreamController,
    ) -> Result<Self, String> {
        if kind == "response" {
            return Ok(Self::Response(ResponseRequest {
                source: ResponseSource::PhysicalChannel,
                metric: ResponseMetric::Amplitude,
                grant_symbol: 0,
                retained_index: 0,
                first: 0,
                count: 0,
            }));
        }
        let (_, retained) = controller
            .report()
            .ok_or("Run upstream before inspecting actual observations.")?;
        let report = retained.value();
        match kind {
            "fec" => Ok(Self::Fec(FecRequest::default())),
            "qam" => {
                let preferred = Tap::RxEqualized;
                let tap = report
                    .options
                    .retained_taps
                    .iter()
                    .copied()
                    .find(|tap| *tap == preferred)
                    .or_else(|| {
                        report
                            .options
                            .retained_taps
                            .iter()
                            .copied()
                            .find(|tap| common::cell_tap(*tap))
                    })
                    .unwrap_or(preferred);
                Ok(Self::Qam(QamRequest::Raw {
                    tap,
                    loading: None,
                    maximum_points: MAX_SCATTER_PLOT_POINTS,
                }))
            }
            "spectrum" => {
                let preferred = Tap::TxOutput;
                let tap = report
                    .options
                    .retained_taps
                    .iter()
                    .copied()
                    .find(|tap| *tap == preferred)
                    .or_else(|| {
                        report
                            .options
                            .retained_taps
                            .iter()
                            .copied()
                            .find(|tap| common::time_tap(*tap))
                    })
                    .unwrap_or(preferred);
                Ok(Self::Spectrum(SpectrumRequest {
                    tap,
                    first: 0,
                    count: 0,
                }))
            }
            _ => Err("Choose a supported upstream instrument.".into()),
        }
    }

    /// Queries an immutable public report on the existing finite background worker.
    /// # Arguments
    /// * `controller`, `budget` - Original owner and canonical display admission.
    /// # Returns
    /// One separately admitted observation, without retaining a report clone.
    /// # Errors
    /// Missing report, public request failure or display admission rejection.
    pub(in crate::upstream::workbench) fn observe(
        self,
        controller: &UpstreamController,
        budget: &RetentionBudget,
    ) -> Result<Retained<Display>, String> {
        if let Self::Response(request) = self {
            return response(controller, request, budget);
        }
        let (_, report) = controller
            .report()
            .ok_or("No upstream report is available.")?;
        match self {
            Self::Qam(request) => super::qam(report.value(), request, budget),
            Self::Spectrum(request) => super::spectrum(report.value(), request, budget),
            Self::Fec(request) => {
                super::admit(super::fec::project(report.value(), request)?, budget)
            }
            Self::Response(_) => {
                unreachable!("Response dispatches before requiring an actual report")
            }
        }
    }
}

fn response(
    controller: &UpstreamController,
    request: ResponseRequest,
    budget: &RetentionBudget,
) -> Result<Retained<Display>, String> {
    let retained = controller.report();
    let report = retained.as_ref().map(|(_, report)| report.value());
    let summary = report.map(|report| report.receiver_observations());
    let snapshot = if request.source == ResponseSource::ReceivedEstimate
        && request.metric == ResponseMetric::Impulse
    {
        super::response::unavailable_request(
            &request,
            summary.as_ref(),
            "Actual receiver estimates have no public inverse/impulse operation.",
        )?
    } else if request.source == ResponseSource::ReceivedEstimate {
        if let Some(report) = report {
            match report.receiver_estimate(ReceiverEstimateRequest {
                retained_index: request.retained_index,
                first_carrier: request.first,
                carrier_count: request.count,
                ..Default::default()
            }) {
                Ok(snapshot) => super::response::project_received(
                    &snapshot,
                    &request,
                    summary
                        .as_ref()
                        .expect("A real report supplies its observation inventory"),
                )?,
                Err(error @ PhyError::Unsupported { .. }) => super::response::unavailable_request(
                    &request,
                    summary.as_ref(),
                    &error.to_string(),
                )?,
                Err(error) => return Err(error.to_string()),
            }
        } else {
            super::response::unavailable_request(
                &request,
                None,
                "No actual upstream receiver-estimate report is available.",
            )?
        }
    } else {
        let source = match request.source {
            ResponseSource::PhysicalChannel => ConfiguredResponseSource::PhysicalChannel,
            ResponseSource::CmPreEq => ConfiguredResponseSource::CmPreEq,
            ResponseSource::Composite => ConfiguredResponseSource::Composite,
            ResponseSource::ReceivedEstimate => unreachable!(),
        };
        let query = ConfiguredResponseRequest {
            source,
            grant_symbol: request.grant_symbol,
            first_carrier: if request.metric == ResponseMetric::Impulse {
                0
            } else {
                request.first
            },
            carrier_count: if request.metric == ResponseMetric::Impulse {
                0
            } else {
                request.count
            },
            include_impulse: request.metric == ResponseMetric::Impulse,
            ..Default::default()
        };
        let result = if let Some(report) = report {
            report.configured_response(query).map_err(ApiError::from)
        } else {
            controller
                .inspect(|simulation| simulation.configured_response(query))
                .map_err(|error| format!("{error:?}"))?
        };
        match result {
            Ok(snapshot) => {
                super::response::project_configured(&snapshot, &request, summary.as_ref())?
            }
            Err(ApiError::Domain(error @ PhyError::Unsupported { .. })) => {
                super::response::unavailable_request(
                    &request,
                    summary.as_ref(),
                    &error.to_string(),
                )?
            }
            Err(error) => return Err(error.to_string()),
        }
    };
    super::admit(snapshot, budget)
}
