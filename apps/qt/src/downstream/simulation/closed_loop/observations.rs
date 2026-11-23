/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded copies of authoritative public observations for existing presentation consumers.
use crate::downstream::common::constants::MAX_INSTRUMENT_TIME_SAMPLES;
use merlin::downstream::{
    ApiError, ApiResult, ErrorCode, FullLoopResult, FullLoopStage as Stage,
    FullLoopStageData as Data, FullLoopStagePhase, FullLoopStageSnapshot, FullLoopStageViewport,
    MAX_STAGE_QUERY_ELEMENTS,
};

const RX_TAPS: &[Stage] = &[
    Stage::ChannelOutput,
    Stage::RxFramed,
    Stage::RxUseful,
    Stage::RxFft,
];
type IqPair = [f64; 2];

#[derive(Default)]
pub(crate) struct Samples {
    pub values: Vec<IqPair>,
    next: Option<u64>,
}

#[derive(Default)]
pub(crate) struct Observations {
    pub tx: crate::transmitter_instruments::capture::Capture,
    pub taps: [Samples; RX_TAPS.len()],
    pub responses: Vec<(u64, Vec<IqPair>)>,
    pub decoder_llrs: Vec<f64>,
    pub decoder_word: Option<u64>,
    pub warnings: Vec<String>,
}

fn append(target: &mut Data, source: Data) -> ApiResult<()> {
    match (target, source) {
        (Data::Bits(a), Data::Bits(b)) | (Data::Bytes(a), Data::Bytes(b)) => a.extend(b),
        (Data::Iq(a), Data::Iq(b)) => a.extend(b),
        (Data::OptionalIq(a), Data::OptionalIq(b)) => a.extend(b),
        (Data::QamLabels(a), Data::QamLabels(b)) => a.extend(b),
        (Data::TransportCells(a), Data::TransportCells(b)) => a.extend(b),
        (Data::Pilots(a), Data::Pilots(b)) => a.extend(b),
        (Data::Llrs(a), Data::Llrs(b)) => a.extend(b),
        // Keep future unsupported representations explicit rather than inventing values.
        _ => {
            return Err(ApiError::new(
                ErrorCode::UnsupportedCapability,
                "Unsupported public stage representation during presentation paging",
            ))
        }
    }
    Ok(())
}

fn snapshot(
    result: &FullLoopResult,
    stage: Stage,
    index: usize,
) -> ApiResult<FullLoopStageSnapshot> {
    let mut event = result.stage(stage, index, FullLoopStageViewport::default())?;
    let cap = result
        .observation_policy()
        .map(|policy| policy.settings().max_elements_per_event)
        .unwrap_or(event.data.len());
    let mut page_len = event.data.len();
    while page_len == MAX_STAGE_QUERY_ELEMENTS && event.data.len() < cap {
        let next = result.stage(
            stage,
            index,
            FullLoopStageViewport {
                start: event.data.len(),
                count: 0,
            },
        );
        match next {
            Ok(page) => {
                page_len = page.data.len();
                append(&mut event.data, page.data)?;
            }
            // A count-zero query at the exact end of this immutable event is exhausted.
            // The public policy caps work even if the original producer total is larger.
            Err(cause) if cause.code() == ErrorCode::InvalidInput => break,
            Err(cause) => return Err(cause),
        }
    }
    Ok(event)
}

impl Observations {
    /// Copies retained public stage windows without rerunning or reconstructing science.
    /// # Arguments
    /// * `result` - Authoritative completed or partial public result with bounded retention.
    /// # Returns
    /// Existing instrument containers; unavailable snapshots remain absent. Projection
    /// failures are separate warnings and never discard completed FEC trials.
    pub(crate) fn from_result(result: &FullLoopResult) -> Self {
        let mut output = Self::default();
        for info in result.stage_inventory() {
            for index in 0..info.snapshots {
                match snapshot(result, info.stage, index) {
                    Ok(event) => output.observe(&event),
                    Err(cause) => output.warnings.push(cause.message().to_owned()),
                }
            }
        }
        output
    }

    fn observe(&mut self, event: &FullLoopStageSnapshot) {
        self.tx.observe(event);
        if event.context.phase != FullLoopStagePhase::Observation {
            return;
        }
        match (&event.stage, &event.data) {
            (Stage::RxChannelEstimate, Data::Iq(values)) => {
                // Event count is already bounded by the accepted public observation policy.
                self.responses.push((
                    event.context.channel_symbol_index.unwrap_or(0),
                    values.clone(),
                ));
                return;
            }
            (Stage::PayloadLdpcInput, Data::Llrs(values)) => {
                if self.decoder_llrs.is_empty() {
                    self.decoder_llrs.clone_from(values);
                    self.decoder_word = event.context.codeword_index;
                }
                return;
            }
            _ => {}
        }
        let Some(index) = RX_TAPS.iter().position(|stage| *stage == event.stage) else {
            return;
        };
        let Data::Iq(values) = &event.data else {
            return;
        };
        let target = &mut self.taps[index];
        let first_event_only = matches!(event.stage, Stage::RxUseful | Stage::RxFft);
        if first_event_only && !target.values.is_empty() {
            return;
        }
        if target.next.is_some() && target.next != event.context.sample_start_index {
            return;
        }
        let count = values
            .len()
            .min(MAX_INSTRUMENT_TIME_SAMPLES - target.values.len());
        target.values.extend_from_slice(&values[..count]);
        target.next = event
            .context
            .sample_start_index
            .and_then(|start| start.checked_add(values.len() as u64));
    }
}
