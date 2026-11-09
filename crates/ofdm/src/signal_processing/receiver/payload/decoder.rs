/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Source-symbol NCP discovery, separate soft hypotheses and bounded payload collection.
use super::{
    collector::PayloadCollector,
    control,
    model::{PayloadSymbolResult, SoftDataCell},
};
use crate::{
    error::invalid,
    signal_processing::modulation::{qam_llrs, randomization::CellRandomizer},
    transmission::{
        layout::ncp::PreInterleavingRole, ncp::NcpFieldCodec, packing::PackingSettings,
        symbols::TransportCell, SubcarrierSpacing,
    },
    SimulationError,
};
/// Persistent source-order receiver; a rejected control/collection result closes this session.
pub struct PayloadSymbolDecoder {
    dynamic_profiles: Option<(
        crate::transmission::plc::receiver_configuration::generations::ProfileGenerations,
        crate::transmission::ncp::activation::NcpTransitionReceiver,
    )>,
    settings: PackingSettings,
    bit_loading: Vec<u8>,
    origin: u64,
    next_local: u64,
    control: NcpFieldCodec,
    ncp_randomizer: CellRandomizer,
    data_randomizer: CellRandomizer,
    collector: PayloadCollector,
    variable_collector: Option<super::variable::VariablePayloadCollector>,
    previous_starts: usize,
    closed: bool,
}
impl PayloadSymbolDecoder {
    /// Starts an independently configured fixed-profile source-order decoder.
    /// # Arguments
    /// * `settings` - Acquired fixed256-QAM/NCP reduced-domain configuration.
    /// * `origin` - Absolute source index at a PLC128-symbol boundary.
    /// # Returns
    /// Empty pending history with separate control/data hypotheses and no TX truth input.
    /// # Errors
    /// Rejects geometry/origin or codec initialization failures.
    pub fn new(settings: PackingSettings, origin: u64) -> Result<Self, SimulationError> {
        Self::with_bit_loading(settings, origin, vec![8; settings.interleaver_cells])
    }
    /// Starts source-order decoding with independently received pre-interleaving loading.
    /// # Arguments
    /// * `settings` - Acquired NI, uniform NCP and data profile identity.
    /// * `origin` - Absolute source index at a128-symbol PLC boundary.
    /// * `bit_loading` - Exactly NI source-rank bit counts, obtained via inverse frequency mapping.
    /// # Returns
    /// Independent soft demappers, randomizers and variable-width codeword collector.
    /// # Errors
    /// Rejects geometry, origin, unsupported loading or codec setup failures.
    pub fn with_bit_loading(
        settings: PackingSettings,
        origin: u64,
        bit_loading: Vec<u8>,
    ) -> Result<Self, SimulationError> {
        settings.validate()?;
        if bit_loading.len() != settings.interleaver_cells {
            return Err(invalid("Payload profile does not match NI."));
        }
        super::configuration::validate_origin(origin)?;
        if settings.data_profile_id != crate::transmission::symbols::PAYLOAD_PROFILE_ID {
            return Err(SimulationError::new(
                "UnsupportedPayloadProfile",
                "Only acquired ProfileA is supported.",
            ));
        }
        Ok(Self {
            dynamic_profiles: None,
            settings,
            origin,
            next_local: 0,
            control: NcpFieldCodec::new()?,
            ncp_randomizer: CellRandomizer::new(),
            data_randomizer: CellRandomizer::new(),
            collector: PayloadCollector::with_bit_loading(&bit_loading)?,
            variable_collector: None,
            bit_loading,
            previous_starts: 0,
            closed: false,
        })
    }
    /// Enables received-DPD data/NCP switching on an empty source-order decoder.
    /// # Arguments
    /// * `configuration` - Ready received OCD and initial ProfileA/NCP generations.
    /// # Returns
    /// This decoder with independent generation bank and received U-history state.
    /// # Errors
    /// Rejects mismatched geometry, already-advanced or shortened-word sessions.
    pub fn enable_dynamic_profiles(
        mut self,
        configuration: &crate::transmission::plc::receiver_configuration::ReceiverConfiguration,
    ) -> Result<Self, SimulationError> {
        let packing = super::configuration::packing_settings(configuration)?;
        if self.next_local != 0
            || self.variable_collector.is_some()
            || packing.interleaver_cells != self.settings.interleaver_cells
            || packing.subcarrier_spacing != self.settings.subcarrier_spacing
            || packing.ncp_modulation != self.settings.ncp_modulation
        {
            return Err(invalid(
                "Dynamic profiles require a matching empty full-word decoder.",
            ));
        }
        let bank =
            crate::transmission::plc::receiver_configuration::generations::ProfileGenerations::new(
                configuration,
            );
        self.bit_loading =
            bank.data_loading(0, configuration.profile_a().change_count() & 1 != 0)?;
        self.collector.activate_profile(0, &self.bit_loading)?;
        self.dynamic_profiles = Some((
            bank,
            crate::transmission::ncp::activation::NcpTransitionReceiver::new(
                configuration.ncp().change_count() & 1 != 0,
            ),
        ));
        Ok(self)
    }
    /// Admits a new generation from received DPD packet bytes.
    /// # Arguments
    /// * `bytes` - Complete CRC-protected management packet, not a TX profile object.
    /// # Returns
    /// True for a new generation or false for an identical repeat.
    /// # Errors
    /// Rejects fixed/closed sessions or incompatible/occupied generations atomically.
    pub fn admit_profile_packet(&mut self, bytes: &[u8]) -> Result<bool, SimulationError> {
        if self.closed {
            return Err(invalid("Cannot update a closed decoder."));
        }
        self.dynamic_profiles
            .as_mut()
            .ok_or_else(|| invalid("Enable dynamic profiles before admitting DPD updates."))?
            .0
            .admit(bytes)
    }
    /// Enables independently received-boundary discovery for shortened/full words.
    /// # Returns
    /// This empty decoder with bounded variable-length collection enabled.
    /// # Errors
    /// Rejects an already advanced decoder or codec setup failure.
    pub fn enable_shortened_words(mut self) -> Result<Self, SimulationError> {
        if self.next_local != 0 || self.dynamic_profiles.is_some() {
            return Err(invalid("Enable shortening before receiving any symbols."));
        }
        self.variable_collector = Some(super::variable::VariablePayloadCollector::new(
            &self.bit_loading,
        )?);
        Ok(self)
    }
    /// Reports whether a terminal control/collection failure requires a new session.
    /// # Returns
    /// True after rejection; normal LDPC/BCH failure flags remain completed word results.
    pub fn is_closed(&self) -> bool {
        self.closed
    }
    /// Reports incomplete coded bits retained from accepted source symbols.
    /// # Returns
    /// Bounded pending length; incomplete words are not counted as decoded payload.
    pub fn pending_bits(&self) -> usize {
        self.variable_collector.as_ref().map_or_else(
            || self.collector.pending_bits(),
            |collector| collector.pending_bits(),
        )
    }
    /// Recovers one inverse-interleaved source symbol using only received cells and roles.
    /// # Arguments
    /// * `source_symbol` - Consecutive absolute source index beginning at origin.
    /// * `cells` - Complete restored NI cells, without TX identities or reservation markers.
    /// * `roles` - Independently derived pre-interleaving pilot reference for this source phase.
    /// * `variance` - Positive finite LLR reference variance; not an injected noise source.
    /// * `max_iterations` - Per-control/payload LDPC cap1..100.
    /// # Returns
    /// Actual control/soft/word results. CRC, unsupported selectors or span failure
    /// return a terminal diagnostic while retaining earlier completed words and pending bits.
    /// # Errors
    /// Rejects invalid API shape/sequence/work limits before advancing state. After a
    /// returned terminal diagnostic construct a new session; no fabricated continuation follows.
    pub fn receive(
        &mut self,
        source_symbol: u64,
        cells: &[Option<TransportCell>],
        roles: &[PreInterleavingRole],
        variance: f64,
        max_iterations: usize,
    ) -> Result<PayloadSymbolResult, SimulationError> {
        self.receive_with_variances(
            source_symbol,
            cells,
            roles,
            &vec![Some(variance); cells.len()],
            max_iterations,
        )
    }
    /// Decodes source-order cells with independently transported residual variances.
    /// # Arguments
    /// * `source_symbol` - Consecutive recovered source identity.
    /// * `cells` - Restored cells without transmitter IDs.
    /// * `roles` - Reconstructed pre-interleaving carrier roles.
    /// * `variances` - Matched restored variances, including known pilot placeholders.
    /// * `max_iterations` - Per-codeword iteration cap.
    /// # Returns
    /// Actual NCP/payload soft decisions and decoder outcomes.
    /// # Errors
    /// Rejects missing/invalid cell calibration or existing terminal/sequence failures.
    pub fn receive_with_variances(
        &mut self,
        source_symbol: u64,
        cells: &[Option<TransportCell>],
        roles: &[PreInterleavingRole],
        variances: &[Option<f64>],
        max_iterations: usize,
    ) -> Result<PayloadSymbolResult, SimulationError> {
        if self.closed {
            return Err(SimulationError::new(
                "PayloadSessionClosed",
                "Start a new payload session after rejection.",
            ));
        }
        if self.origin.checked_add(self.next_local) != Some(source_symbol)
            || cells.len() != self.settings.interleaver_cells
            || roles.len() != cells.len()
            || variances.len() != cells.len()
            || variances
                .iter()
                .flatten()
                .any(|variance| !variance.is_finite() || *variance <= 0.0)
            || !(1..=100).contains(&max_iterations)
        {
            return Err(invalid(
                "Invalid source-order payload input or work limits.",
            ));
        }
        let next_local = self
            .next_local
            .checked_add(1)
            .ok_or_else(|| invalid("Source symbol counter overflow."))?;
        let mut points = Vec::new();
        let mut point_variances = Vec::new();
        for ((cell, role), variance) in cells.iter().zip(roles).zip(variances) {
            match (cell, role) {
                (
                    Some(TransportCell::PilotPlaceholder),
                    PreInterleavingRole::ScatteredPilotPlaceholder,
                ) => {}
                (Some(TransportCell::Sample(point)), PreInterleavingRole::Data)
                    if point.iter().all(|value| value.is_finite()) =>
                {
                    let variance = variance
                        .filter(|value| value.is_finite() && *value > 0.0)
                        .ok_or_else(|| invalid("Missing source-cell residual variance."))?;
                    point_variances.push(variance);
                    points.push(*point)
                }
                _ => {
                    return Err(invalid(
                        "Missing or mismatched restored payload/pilot cell.",
                    ))
                }
            }
        }
        let mut result = PayloadSymbolResult {
            source_symbol_index: source_symbol,
            ncp: None,
            ncp_soft: Vec::new(),
            data_soft: Vec::new(),
            recovered: Vec::new(),
            pending_payload_bits: self.variable_collector.as_ref().map_or_else(
                || self.collector.pending_bits(),
                |collector| collector.pending_bits(),
            ),
            error: None,
        };
        let mut control_randomizer = self.ncp_randomizer.clone();
        control_randomizer.begin_symbol(self.next_local)?;
        let mut data_randomizer = self.data_randomizer.clone();
        data_randomizer.begin_symbol(self.next_local)?;
        let decision = (|| -> Result<(), SimulationError> {
            let ncp_modulation = if let Some((bank, activation)) = &self.dynamic_profiles {
                bank.ncp_modulation(activation.expected_odd())?
            } else {
                self.settings.ncp_modulation
            };
            result.ncp_soft = control::hypotheses_with_variances(
                &points,
                ncp_modulation,
                &point_variances,
                &mut control_randomizer,
            )?;
            let hypotheses: Vec<_> = result
                .ncp_soft
                .iter()
                .map(|cell| cell.llrs.clone())
                .collect();
            let field = self
                .control
                .decode_field(&hypotheses, ncp_modulation, max_iterations)?;
            result.ncp = Some(field);
            let field = result
                .ncp
                .as_ref()
                .ok_or_else(|| invalid("Missing accepted NCP field."))?;
            if self.dynamic_profiles.is_none()
                && field.messages.iter().any(|message| {
                    message.data_profile_odd
                        || message.ncp_profile_odd
                        || message.ncp_profile_update
                        || message.tagged
                })
            {
                return Err(SimulationError::new(
                    "UnsupportedProfileActivation",
                    "Fixed payload session does not support odd profiles, NCP updates or tagging.",
                ));
            }
            let mut next_activation = self
                .dynamic_profiles
                .as_ref()
                .map(|(_, activation)| activation.clone());
            if let Some(activation) = &mut next_activation {
                if field.messages.iter().any(|message| message.tagged) {
                    return Err(invalid(
                        "Tagged NCP is outside dynamic full-word transport.",
                    ));
                }
                let first = field
                    .messages
                    .first()
                    .ok_or_else(|| invalid("Empty NCP field."))?;
                activation.accept(first.ncp_profile_odd, first.ncp_profile_update)?;
            }
            let starts = field
                .messages
                .iter()
                .filter(|message| {
                    !message.zero_loaded
                        && message.pointer != crate::transmission::ncp::NULL_POINTER
                })
                .count();
            if self.settings.subcarrier_spacing == SubcarrierSpacing::Khz50
                && starts + self.previous_starts
                    > crate::transmission::ncp::message::MAX_CODEWORD_STARTS
            {
                return Err(invalid("Received NCP exceeds the rolling4K start limit."));
            }
            let data_count = points
                .len()
                .checked_sub(field.consumed_cells)
                .ok_or_else(|| invalid("NCP field exceeds eligible data positions."))?;
            let spans = field.data_spans(data_count)?;
            // Loading follows each received start, including multiple profile IDs
            // within one field. Continuations retain the previous codeword's DPD.
            let source_ranks: Vec<_> = roles
                .iter()
                .enumerate()
                .filter(|(_, role)| **role == PreInterleavingRole::Data)
                .map(|(rank, _)| rank)
                .collect();
            for span in &spans {
                if span.starts_codeword {
                    if let Some((bank, _)) = &self.dynamic_profiles {
                        let message = field
                            .messages
                            .iter()
                            .find(|message| usize::from(message.pointer) == span.start)
                            .ok_or_else(|| invalid("Missing start message."))?;
                        let loading =
                            bank.data_loading(message.profile_id, message.data_profile_odd)?;
                        self.collector
                            .activate_profile(message.profile_id, &loading)?;
                        self.bit_loading = loading;
                    }
                }
                let mut soft = Vec::new();
                for data_position in span.start..span.end_exclusive {
                    if span.zero_loaded {
                        data_randomizer.zero_loaded_bpsk()?;
                        continue;
                    }
                    let bits = self.bit_loading[source_ranks[data_position]];
                    let llrs = if bits == 0 {
                        data_randomizer.zero_loaded_bpsk()?;
                        Vec::new()
                    } else {
                        let order = 1u16 << bits;
                        data_randomizer.descramble_data_llrs(
                            order,
                            &qam_llrs(
                                order,
                                points[data_position],
                                point_variances[data_position],
                            )?,
                        )?
                    };
                    soft.push(SoftDataCell {
                        data_position,
                        llrs,
                    });
                }
                let first_soft = result.data_soft.len();
                result.data_soft.extend(soft);
                let soft = &result.data_soft[first_soft..];
                let completed = if let Some(collector) = &mut self.variable_collector {
                    collector.accept_span(span, soft, source_symbol, max_iterations)?
                } else {
                    self.collector
                        .accept_span(span, soft, source_symbol, max_iterations)?
                };
                if let Some(word) = completed {
                    result.recovered.push(word);
                }
            }
            for _ in data_count..points.len() {
                data_randomizer.zero_loaded_bpsk()?;
            }
            if data_randomizer.state() != control_randomizer.state() {
                return Err(invalid("Data/NCP hypothesis clocks disagree."));
            }
            if let (Some((_, activation)), Some(next)) =
                (&mut self.dynamic_profiles, next_activation)
            {
                *activation = next;
            }
            self.previous_starts = starts;
            Ok(())
        })();
        if let Err(error) = decision {
            log::debug!(
                "payload-session-rejected source-symbol={} error-code={}",
                source_symbol,
                error.code
            );
            self.closed = true;
            result.error = Some(error);
        } else {
            self.ncp_randomizer = control_randomizer;
            self.data_randomizer = data_randomizer;
            self.next_local = next_local;
        }
        result.pending_payload_bits = self.variable_collector.as_ref().map_or_else(
            || self.collector.pending_bits(),
            |collector| collector.pending_bits(),
        );
        Ok(result)
    }
}
