/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded transmit-only preview using existing coding, signaling and waveform stages.
use crate::{
    error::invalid,
    profile::candidate::CapturedProfile,
    signal_processing::{
        modulation::interleaving::mixed::PayloadBitOrder,
        waveform::{
            spectrum::{self, PowerSpectrum, MAX_SPECTRUM_AVERAGES},
            WaveformGeometry,
        },
    },
    simulation::{
        channel::measurement::captured_layout, payload::PayloadTransmitter, plc::generate_frame,
        source::PayloadSourceConfig, waveform::WaveformTransmitter,
    },
    transmission::{
        frame::assemble,
        layout::{
            bit_loading::source_bit_loading, CarrierLayout, CarrierLayoutSettings,
            PhysicalCarrierRole,
        },
        packing::NcpModulation,
        plc::{
            descriptors::{DescriptorIdentity, FixedPlcDescriptors},
            scheduling::PlcMessageScheduler,
            ExtendedTimestamp, PlcMessageFrame,
        },
        TransmissionSettings,
    },
    types::{modulation::CarrierType, IqPair, PLC_FRAME_SYMBOLS},
    SimulationError,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// Frozen supported inputs, independent of Qt, filenames and active simulation jobs.
#[derive(Clone)]
pub struct TransmitterPreviewRequest {
    settings: TransmissionSettings,
    layout: CarrierLayoutSettings,
    loading: Vec<u8>,
    identity: DescriptorIdentity,
    source: PayloadSourceConfig,
    seed: u16,
}

/// Actual samples and full PSD retained by the caller; no fabricated spectrum trace.
pub struct TransmitterPreview {
    /// Frozen settings that produced this sample snapshot.
    pub settings: TransmissionSettings,
    pub samples: Vec<IqPair>,
    pub first_sample: u64,
    pub generated_source_bits: u64,
    pub warmup_symbols: usize,
    pub source_exhausted: bool,
    pub spectrum: PowerSpectrum,
}

impl TransmitterPreviewRequest {
    /// Validates a Profile A preview against exact captured roles and accepted inputs.
    /// # Arguments
    /// * `settings` - Accepted channel numerology/timing, copied into this request.
    /// * `profile` - Exact captured Profile A, including protected assignments.
    /// * `source` - Immutable validated PRBS/file source; bytes remain shared.
    /// * `seed` - Nonzero continuous PRBS seed; file fill retains its own seed.
    /// # Returns
    /// Frozen request usable on a Rust worker; no source stream is advanced.
    /// # Errors
    /// Rejects unsupported profile IDs, inconsistent roles/loading/geometry or seeds.
    pub fn new(
        settings: &TransmissionSettings,
        profile: &CapturedProfile,
        source: PayloadSourceConfig,
        seed: u16,
    ) -> Result<Self, SimulationError> {
        settings.validate()?;
        if profile.profile_id != 0 {
            return Err(invalid("TX spectrum currently requires Profile A (ID 0), matching the live descriptor API."));
        }
        let layout_settings = captured_layout(profile, settings)?;
        let layout = CarrierLayout::new(settings, &layout_settings)?;
        let mut loading = vec![0; layout.static_roles().len()];
        for (offset, assignment) in profile.assignments.iter().enumerate() {
            let bin = usize::from(profile.key.grid.first_subcarrier_index) + offset;
            let expected = match layout.static_roles()[bin] {
                PhysicalCarrierRole::Data => {
                    match assignment.carrier_type {
                        CarrierType::Data | CarrierType::ZeroBitLoaded => {
                            loading[bin] = assignment
                                .bit_loading
                                .ok_or_else(|| invalid("Missing data bit loading."))?;
                        }
                        _ => return Err(invalid("Captured data role differs from TX layout.")),
                    }
                    continue;
                }
                PhysicalCarrierRole::ContinuousPilot => CarrierType::ContinuousPilot,
                PhysicalCarrierRole::Plc => CarrierType::Plc,
                PhysicalCarrierRole::Excluded => CarrierType::Exclusion,
                PhysicalCarrierRole::ScatteredPilot => unreachable!(),
            };
            if assignment.carrier_type != expected {
                return Err(invalid("Captured protected role differs from TX layout."));
            }
        }
        let order = source_bit_loading(&layout, &loading)?;
        PayloadBitOrder::new(&order)?;
        let identity = DescriptorIdentity {
            channel_id: profile.key.channel_id,
            change_count: 0,
            source_mac: profile.key.mac_address,
            primary_capable: true,
            ncp_bits: 2,
        };
        FixedPlcDescriptors::with_data_loading(settings, &layout, identity, Some(&loading))?;
        source.stages(seed)?;
        Ok(Self {
            settings: settings.clone(),
            layout: layout_settings,
            loading,
            identity,
            source,
            seed,
        })
    }

    /// Generates actual post-window/overlap samples and their bounded averaged FFT.
    /// # Arguments
    /// * `cancel` - Cooperative cancellation flag checked between symbol/analysis steps.
    /// * `progress` - Scalar completed/total TX symbol counts; observes state only.
    /// # Returns
    /// Contiguous sample snapshot after idle priming plus normalized Hann PSD.
    /// Finite sources stop at EOF without repeat; any pending queue is preview-only.
    /// # Errors
    /// Returns cancellation or existing TX/FFT failures; never changes accepted input.
    pub fn generate(
        &self,
        cancel: &AtomicBool,
        mut progress: impl FnMut(usize, usize),
    ) -> Result<TransmitterPreview, SimulationError> {
        let check = || {
            if cancel.load(Ordering::Relaxed) {
                return Err(invalid("TX spectrum generation cancelled."));
            }
            Ok(())
        };
        check()?;
        let geometry = WaveformGeometry::new(&self.settings)?;
        let needed =
            geometry.fft_samples() + (MAX_SPECTRUM_AVERAGES - 1) * (geometry.fft_samples() / 2);
        let warmup = usize::from(self.settings.time_interleaver_depth) - 1;
        let symbols = needed.div_ceil(geometry.stride_samples());
        let layout = CarrierLayout::new(&self.settings, &self.layout)?;
        let descriptors = FixedPlcDescriptors::with_data_loading(
            &self.settings,
            &layout,
            self.identity,
            Some(&self.loading),
        )?;
        let mut scheduler = PlcMessageScheduler::new(&self.settings, &descriptors)?;
        let mut payload = PayloadTransmitter::with_profile(
            &self.settings,
            &self.layout,
            NcpModulation::Qpsk,
            0,
            &self.loading,
        )?;
        let mut waveform = WaveformTransmitter::new(&geometry, 0, 0);
        let (mut source, _) = self.source.stages(self.seed)?;
        let mut exhausted = false;
        let mut plc = None;
        let mut samples = Vec::with_capacity(needed);
        for index in 0..warmup + symbols {
            check()?;
            if index >= warmup {
                while payload.queued_codewords() < 8 && !exhausted {
                    check()?;
                    if let Some(bits) = source.next_payload()? {
                        payload.enqueue(&bits)?;
                    } else {
                        exhausted = true;
                    }
                }
            }
            let index64 = index as u64;
            if index64 % PLC_FRAME_SYMBOLS == 0 {
                plc = Some(generate_frame(
                    PlcMessageFrame::new(
                        self.settings.subcarrier_spacing,
                        ExtendedTimestamp::from_sample_ticks(
                            index64 * geometry.stride_samples() as u64,
                            0,
                        )?,
                        scheduler.next_block()?.block,
                    )?,
                    index64 / PLC_FRAME_SYMBOLS,
                )?);
            }
            let transmitted = payload.transmit()?;
            let column = plc
                .as_ref()
                .and_then(|frame| frame.symbol_iq(index64))
                .ok_or_else(|| invalid("Missing PLC preview column."))?;
            let physical = assemble(&layout, index64, &transmitted.channel.reduced, column, 1.0)?;
            let output = waveform.transmit(&physical)?;
            if index >= warmup {
                let take = (needed - samples.len()).min(output.output.samples().len());
                samples.extend_from_slice(&output.output.samples()[..take]);
            }
            progress(index + 1, warmup + symbols);
        }
        check()?;
        let spectrum = spectrum::estimate(
            self.settings.subcarrier_spacing,
            self.layout.zero_frequency_hz,
            &samples,
        )?;
        check()?;
        Ok(TransmitterPreview {
            settings: self.settings.clone(),
            samples,
            first_sample: (warmup * geometry.stride_samples()) as u64,
            generated_source_bits: source.position(),
            warmup_symbols: warmup,
            source_exhausted: exhausted,
            spectrum,
        })
    }
}
