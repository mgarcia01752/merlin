/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! RX composition derives its configuration from accepted PLC and consumes only received I/Q.
use crate::{
    error::invalid,
    signal_processing::{
        interleaving::{InterleaverDirection, SymbolInterleaver},
        receiver::payload::{configuration, PayloadSymbolDecoder, PayloadSymbolResult},
    },
    transmission::{
        layout::{PhysicalCarrierRole, PilotReference},
        plc::receiver_configuration::{ReceiverConfiguration, ReceiverConfigurationState},
        symbols::TransportCell,
    },
    types::IqPair,
    SimulationError,
};
/// Actual inverse-interleaver and source-decoder output at one received channel step.
pub struct PayloadReceiveSymbol {
    pub channel_symbol_index: u64,
    pub source_symbol_index: Option<u64>,
    pub restored: Vec<Option<TransportCell>>,
    /// Direct inspection of variance metadata after the same inverse permutation/delay.
    pub restored_variances: Vec<Option<f64>>,
    /// None only for the explicitly known M-1 matched-pair warm-up interval.
    pub decoded: Option<PayloadSymbolResult>,
}
/// Fixed-session received-configuration owner, independent of TX plans and truth.
pub struct PayloadReceiver {
    snapshot: ReceiverConfiguration,
    layout: crate::transmission::layout::CarrierLayout,
    origin: u64,
    next_local: u64,
    reference: PilotReference,
    interleaver: SymbolInterleaver<TransportCell>,
    variance_interleaver: SymbolInterleaver<f64>,
    decoder: PayloadSymbolDecoder,
}
impl PayloadReceiver {
    /// Acquires supported payload settings through the PLC readiness gate.
    /// # Arguments
    /// * `state` - Ready received configuration, with actual OCD/ProfileA/NCP values.
    /// * `origin` - Explicit synchronized payload-session boundary, a multiple of128.
    /// # Returns
    /// Owned snapshot, independent role map, inverse interleaver and control/payload decoders.
    /// # Errors
    /// Rejects missing/rejected configuration, unsupported loading, origin or codec setup.
    pub fn new(state: &ReceiverConfigurationState, origin: u64) -> Result<Self, SimulationError> {
        configuration::validate_origin(origin)?;
        let snapshot = state.require_ready()?.clone();
        let settings = configuration::packing_settings(&snapshot)?;
        let layout = snapshot.channel().physical_layout()?;
        let reference = PilotReference::new(&layout, snapshot.interleaver_settings()?.depth())?;
        let interleaver =
            SymbolInterleaver::new(reference.settings(), InterleaverDirection::Receive)?;
        let physical: Vec<_> = snapshot
            .profile_a()
            .bit_loading()
            .iter()
            .map(|bits| bits.unwrap_or(0))
            .collect();
        let bit_loading =
            crate::transmission::layout::bit_loading::source_bit_loading(&layout, &physical)?;

        Ok(Self {
            variance_interleaver: SymbolInterleaver::new(
                reference.settings(),
                InterleaverDirection::Receive,
            )?,
            snapshot,
            layout,
            origin,
            next_local: 0,
            reference,
            interleaver,
            decoder: PayloadSymbolDecoder::with_bit_loading(settings, origin, bit_loading)?,
        })
    }
    /// Creates an opt-in full/shortened receiver from independently received descriptors.
    /// # Arguments
    /// * `state` - Ready live PLC configuration, including acquired data loading.
    /// * `origin` - PLC-aligned source/channel origin.
    /// # Returns
    /// Receiver which waits for NCP start/filler boundaries to classify word length.
    /// # Errors
    /// Propagates configuration, geometry, origin or codec setup failure.
    pub fn with_shortened_words(
        state: &ReceiverConfigurationState,
        origin: u64,
    ) -> Result<Self, SimulationError> {
        let mut receiver = Self::new(state, origin)?;
        receiver.decoder = receiver.decoder.enable_shortened_words()?;
        Ok(receiver)
    }
    /// Receives extracted physical reduced cells and reverses the continuous cell path.
    /// # Arguments
    /// * `state` - Live PLC gate; must stay Ready with the same immutable snapshot.
    /// * `channel_symbol` - Consecutive absolute channel index, beginning at origin.
    /// * `reduced` - Received normalized samples; None only at known scattered-pilot positions.
    /// * `variance` - Finite positive LLR reference variance, not added channel noise.
    /// * `max_iterations` - Per-control/payload LDPC bound1..100.
    /// # Returns
    /// Restored stage values and actual decoded source results after M-1 warm-up.
    /// Source identity is reconstructed from known pair delay, never TX word IDs.
    /// # Errors
    /// Rejects gate/config/shape/sequence before interleaving. A terminal decoded
    /// diagnostic closes payload recovery; construct a new session after such a failure.
    pub fn receive(
        &mut self,
        state: &ReceiverConfigurationState,
        channel_symbol: u64,
        reduced: &[Option<IqPair>],
        variance: f64,
        max_iterations: usize,
    ) -> Result<PayloadReceiveSymbol, SimulationError> {
        let variances = vec![variance; self.layout.static_roles().len()];
        self.receive_with_variances(state, channel_symbol, reduced, &variances, max_iterations)
    }
    /// Receives physical cells with their post-equalization variance through matching delay paths.
    /// # Arguments
    /// * `state` - Live received configuration; must match the frozen snapshot.
    /// * `channel_symbol` - Consecutive absolute channel index.
    /// * `reduced` - Received physical reduced cells, retaining pilot placeholders.
    /// * `variances` - Positive finite complex variance per full physical FFT bin.
    /// * `max_iterations` - Per-codeword work bound.
    /// # Returns
    /// Restored cells and decisions with per-cell variance preserved across interleaving.
    /// # Errors
    /// Rejects shape/numeric/gate errors; terminal decoded failures close the session.
    pub fn receive_with_variances(
        &mut self,
        state: &ReceiverConfigurationState,
        channel_symbol: u64,
        reduced: &[Option<IqPair>],
        variances: &[f64],
        max_iterations: usize,
    ) -> Result<PayloadReceiveSymbol, SimulationError> {
        if state.require_ready()? != &self.snapshot {
            return Err(invalid(
                "Acquired payload configuration changed; start a new session.",
            ));
        }
        if self.decoder.is_closed() {
            return Err(SimulationError::new(
                "PayloadSessionClosed",
                "Start a new payload session after control rejection.",
            ));
        }
        if self.origin.checked_add(self.next_local) != Some(channel_symbol)
            || reduced.len() != self.reference.settings().cells()
            || variances.len() != self.layout.static_roles().len()
            || variances
                .iter()
                .any(|variance| !variance.is_finite() || *variance <= 0.0)
            || !(1..=100).contains(&max_iterations)
        {
            return Err(invalid("Invalid physical payload input or work limits."));
        }
        let next = self
            .next_local
            .checked_add(1)
            .ok_or_else(|| invalid("Payload RX channel index overflow."))?;
        let roles = self.layout.symbol_roles(channel_symbol);
        let mut cells = Vec::with_capacity(reduced.len());
        let mut cell_variances = Vec::with_capacity(reduced.len());
        for (point, bin) in reduced.iter().zip(self.reference.physical_bins()) {
            cell_variances.push(Some(variances[usize::from(*bin)]));
            cells.push(Some(match (point, roles[usize::from(*bin)]) {
                (None, PhysicalCarrierRole::ScatteredPilot) => TransportCell::PilotPlaceholder,
                (Some(point), PhysicalCarrierRole::Data)
                    if point.iter().all(|value| value.is_finite()) =>
                {
                    TransportCell::Sample(*point)
                }
                _ => {
                    return Err(invalid(
                        "Physical payload sample conflicts with received pilot roles.",
                    ))
                }
            }));
        }
        let restored = self.interleaver.process_symbol(self.next_local, &cells)?;
        let restored_variances = self
            .variance_interleaver
            .process_symbol(self.next_local, &cell_variances)?;
        let source_local = self
            .next_local
            .checked_sub(self.reference.settings().pair_delay_symbols() as u64);
        let source_symbol_index = source_local.map(|index| self.origin + index);
        let decoded = match (source_local, source_symbol_index) {
            (Some(local), Some(source)) => Some(self.decoder.receive_with_variances(
                source,
                &restored,
                self.reference.roles(local),
                &restored_variances,
                max_iterations,
            )?),
            _ => None,
        };
        self.next_local = next;
        Ok(PayloadReceiveSymbol {
            restored_variances,
            channel_symbol_index: channel_symbol,
            source_symbol_index,
            restored,
            decoded,
        })
    }
    /// Reports incomplete accepted coded bits at an explicit end-of-stream boundary.
    /// # Returns
    /// Pending bits only; caller must separately count recovered word results and warm-up/drain.
    pub fn pending_bits(&self) -> usize {
        self.decoder.pending_bits()
    }
}
