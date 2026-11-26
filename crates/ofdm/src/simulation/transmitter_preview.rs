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
use merlin_api::Cancellation;
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use std::sync::atomic::AtomicBool;

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
    /// Frozen occupied-band geometry for independent integration and display.
    pub layout: CarrierLayoutSettings,
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
        if profile.profile_id > 15 {
            return Err(invalid("Captured data profile ID must be in 0..15; each independent preview uses bootstrap Profile A."));
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

    /// Copies the existing frozen output settings/layout for capacity preflight.
    ///
    /// # Returns
    /// Independent finite metadata snapshots; no source, waveform or PSD runs.
    /// Layout storage retains its actual cloned capacity until its API owner
    /// normalizes the final destination. The engine has no API/adapter dependency.
    #[cfg(feature = "pnm-input")]
    pub(crate) fn retained_output_snapshot(&self) -> (TransmissionSettings, CarrierLayoutSettings) {
        (self.settings.clone(), self.layout.clone())
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
        progress: impl FnMut(usize, usize),
    ) -> Result<TransmitterPreview, SimulationError> {
        self.generate_with_cancellation(cancel, progress)
    }

    /// Generates the same bounded preview with a shared one-way cancellation query.
    /// # Arguments
    /// * `cancel` - Read-only stop token, checked at the existing symbol/FFT boundaries.
    /// * `progress` - Observation-only completed/total symbol counts.
    /// # Returns
    /// Actual post-overlap samples and PSD with the original source/priming semantics.
    /// # Errors
    /// Preserves existing cancellation, TX and FFT failures without changing inputs.
    pub fn generate_with_cancellation(
        &self,
        cancel: &impl Cancellation,
        mut progress: impl FnMut(usize, usize),
    ) -> Result<TransmitterPreview, SimulationError> {
        let check = || {
            if cancel.is_cancelled() {
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
            layout: self.layout.clone(),
            samples,
            first_sample: (warmup * geometry.stride_samples()) as u64,
            generated_source_bits: source.position(),
            warmup_symbols: warmup,
            source_exhausted: exhausted,
            spectrum,
        })
    }
}

impl RetainedPayload for TransmitterPreviewRequest {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            settings,
            layout,
            loading,
            identity,
            source,
            seed,
        } = self;
        merlin_api::retention_support::sum([
            settings.retained_children(),
            layout.retained_children(),
            loading.retained_children(),
            identity.retained_children(),
            source.retained_children(),
            seed.retained_children(),
        ])
    }
}

impl RetainedPayload for TransmitterPreview {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            settings,
            layout,
            samples,
            first_sample,
            generated_source_bits,
            warmup_symbols,
            source_exhausted,
            spectrum,
        } = self;
        merlin_api::retention_support::sum([
            settings.retained_children(),
            layout.retained_children(),
            samples.retained_children(),
            first_sample.retained_children(),
            generated_source_bits.retained_children(),
            warmup_symbols.retained_children(),
            source_exhausted.retained_children(),
            spectrum.retained_children(),
        ])
    }
}

#[cfg(test)]
mod input_retained_footprint_tests {
    use super::{DescriptorIdentity, PowerSpectrum, TransmitterPreview, TransmitterPreviewRequest};
    use crate::{
        simulation::source::{file::FilePayload, PayloadSourceConfig},
        transmission::{layout::CarrierLayoutSettings, TransmissionSettings},
    };
    use merlin_api::retention_support::retained_footprint;
    use std::{
        mem::{size_of, size_of_val},
        sync::Arc,
    };

    fn storage_layout() -> CarrierLayoutSettings {
        // Storage-only fixture; zero coordinates are never scientifically admitted.
        const EXCLUDED_SLOTS: usize = 3;
        const PILOT_SLOTS: usize = 5;
        CarrierLayoutSettings {
            zero_frequency_hz: 0,
            first_active_bin: 0,
            active_end_exclusive: 0,
            plc_first_bin: 0,
            excluded_bins: Vec::with_capacity(EXCLUDED_SLOTS),
            additional_continuous_pilots: Vec::with_capacity(PILOT_SLOTS),
        }
    }

    #[test]
    fn frozen_tx_request_counts_private_loading_layout_and_original_file() {
        const LOADING_SLOTS: usize = 7; // Distinct private loading spare capacity.
        const FILE_BYTES: &[u8] = b"exact private request source";
        let layout = storage_layout();
        let loading = Vec::with_capacity(LOADING_SLOTS);
        let expected = size_of::<TransmitterPreviewRequest>()
            + (layout.excluded_bins.capacity() + layout.additional_continuous_pilots.capacity())
                * size_of::<u16>()
            + loading.capacity()
            + FILE_BYTES.len();
        let request = TransmitterPreviewRequest {
            settings: TransmissionSettings::default(),
            layout,
            loading,

            // All-zero fixed identity is an accounting fixture, never used for TX.
            identity: DescriptorIdentity {
                channel_id: 0,
                change_count: 0,
                source_mac: [0; 6],
                primary_capable: false,
                ncp_bits: 0,
            },
            source: PayloadSourceConfig::File {
                payload: FilePayload::new(FILE_BYTES, None).unwrap(),
            },
            seed: crate::common::constants::PRBS15_MAX_SEED,
        };
        let settings = serde_json::to_value(&request.settings).unwrap();
        let footprint = retained_footprint(&request).unwrap();
        assert_eq!(footprint.payload_bytes, expected);
        assert_eq!(footprint.shared_roots, 1); // Original immutable file payload.
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(retained_footprint(&request).unwrap(), footprint);
        assert!(request.loading.is_empty());
        assert_eq!(serde_json::to_value(&request.settings).unwrap(), settings);
        let PayloadSourceConfig::File { payload } = &request.source else {
            panic!("Storage fixture must retain its actual file source");
        };
        assert_eq!(payload.bytes(), FILE_BYTES);
    }

    #[test]
    fn retained_tx_owner_layout_samples_and_full_psd_spares_repeat_across_arc_edges() {
        // This storage-only fixture runs no FFT, source or scientific preview.
        const SAMPLE_SLOTS: usize = 7;
        const PSD_SLOTS: usize = 11;
        let layout = storage_layout();
        let samples = Vec::with_capacity(SAMPLE_SLOTS);
        let density = Vec::with_capacity(PSD_SLOTS);
        let owner_bytes = size_of::<TransmitterPreview>()
            + (layout.excluded_bins.capacity() + layout.additional_continuous_pilots.capacity())
                * size_of::<u16>()
            + samples.capacity() * size_of::<[f64; 2]>()
            + density.capacity() * size_of::<f64>();
        let result = Arc::new(TransmitterPreview {
            settings: TransmissionSettings::default(),
            layout,
            samples,
            first_sample: 0,
            generated_source_bits: 0,
            warmup_symbols: 0,
            source_exhausted: false,
            spectrum: PowerSpectrum {
                zero_frequency_hz: 0,
                spacing_hz: 0,
                fft_samples: 0,
                averages: 0,
                equivalent_noise_bandwidth_hz: 0.0,
                density,
            },
        });
        let aliases = [Arc::clone(&result), Arc::clone(&result)];
        let owners = Arc::strong_count(&result);
        let footprint = retained_footprint(&aliases).unwrap();
        assert_eq!(
            footprint.payload_bytes,
            size_of_val(&aliases) + owner_bytes * aliases.len()
        );
        assert_eq!(footprint.shared_roots, aliases.len());
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(Arc::strong_count(&result), owners);
        assert!(Arc::ptr_eq(&aliases[0], &aliases[1]));
        assert!(result.samples.is_empty() && result.spectrum.density.is_empty());
    }
}
