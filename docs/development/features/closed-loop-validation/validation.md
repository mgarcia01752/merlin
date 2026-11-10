# Closed-loop acceptance evidence

The user-defined minimum is MERLIN TX → channel → MERLIN RX → original-source
comparison. The following existing tests passed in the local workspace regression
on Ubuntu22.04 x64 with pinned Rust/Cargo1.85.1. Scope clarification changes only
documentation and test policy; no new scientific run is claimed for this edit.

| Coverage | Evidence |
| --- | --- |
| Source truth | `prbs_source.rs`, `file_source_channel.rs`, CLI `file_payload.rs`: PRBS recurrence, exact original bytes, separately counted fill/padding and recovered-file equality |
| Full encoded payload/control through samples | `payload_waveform.rs`: acquired PLC configuration, actual BCH/LDPC data and NCP, interleaving, randomization, waveform and original-bit comparison at both spacings |
| QAM | `qam_reference_tables.rs`, `uniform_profile_orders.rs`, `mixed_profile_transport.rs`, `shortened_transport.rs`: all32720 source labels, supported uniform orders through16K, mixed/zero-loaded and shortened coding paths |
| Widths/profiles | `channel_widths.rs` and CLI `pnm_import.rs`:24/90MHz inputs with four profile loadings and zero-error noiseless recovery; receiver-search tests separately include192MHz and both spacings |
| Acquired WAVE-RX | `receiver_search.rs`: twelve width/spacing/channel cases, six zero-error coded-block cases, reflected paths, signed CFO, continuation and rejection |
| Channel effects | `waveform_impairments.rs`, `channel_estimation_rx.rs`, `channel_timeline.rs`: repeatable noise/response application, pilot correction, actual decoder outcomes and source-grid/timeline checks |
| Control truth | `plc_acquisition_roundtrip.rs`, `receiver_configuration.rs`, `dynamic_profiles.rs`: actual control FEC/CRC, recovered descriptors and received generation selection |
| Honest completion | `channel_job.rs`: source-window counts, cancellation, unavailable/pending data and retained outcomes after sink failure |
| API/CLI and observations | CLI `channel.rs`, `file_payload.rs`, `receiver_measurements.rs`; shared science, no-clobber, bounded exports and observer invariance |

The complete workspace tests, final-source focused receiver/QAM tests,
warnings-denied Clippy, formatting and no-default-feature tests passed during
receiver completion. Logs remain in ignored
`docs/development/local/receiver-acquisition/`. Current documentation links,
whitespace and testing-skill discovery paths are checked separately.

The recorded matrices exercise specific supported combinations. For example,
24/90MHz four-profile import/transport uses25kHz spacing, while the acquired
receiver matrix covers all three widths at both spacings. An isolated acquired
coded block is not relabeled as full live NCP/PLC acquisition. External vectors
were not executed and are not required for this closed-loop acceptance.
