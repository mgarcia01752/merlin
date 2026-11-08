# DOFT architecture

DOFT is a standalone DOCSIS scientific toolkit with a Rust library and CLI.
Downstream OFDM is the current focus. OFDMA is deferred. All signal processing,
channel modeling, modulation, soft demapping and FEC are implemented in Rust.

```text
ofdm-fec CLI                 Future optional adapters
                             Python bindings / Qt presentation
          \                       /
           docsis-ofdm-fec (crates/ofdm)
             types/       identities, units, carrier roles and modulation support
             math/        targets, OFDM rates and constellation references
             profile/     scenarios, association, candidates, evaluation,
                          documents and simulation
                         |
                    fec-codec
                    Rust LDPC + Rust BCH
```

The scientific crates forbid unsafe code and have no Qt, Python-runtime or native
codec dependency. Python remains useful for development/release tooling only.
The optional future Python and Qt adapters call the same public scientific API;
scientific functionality takes priority over either adapter.

Two simulation modes are planned:

- **Ideal/reference:** known symbols and channel, noiseless baseline or controlled
  AWGN; actual FEC outcomes establish reference behavior.
- **Measurement-driven:** exact-grid RxMER, downstream channel estimates and an
  applied modulation profile describe the supplied OFDM conditions. RxMER-based
  residual AWGN is implemented for the initial coding experiment. The complete
  channel-estimate-driven model remains pending. Establish measurement reference
  points before combining them; do not count equalized impairment twice.

Typed source inputs retain identity, units, timestamps, original grid positions,
missing readings, exclusions and provenance. Applications that decode binary
capture files retain original bytes and compute source hashes. A supplied JSON
hash is metadata, not authentication. No silent interpolation or resampling occurs.

Scientific results distinguish measured observations, analytical target screening
and simulated decoder outcomes. Individual frames and worst intervals remain
visible; an average curve must not hide transient impairments. MER-derived noise
averages linear error powers rather than arithmetic dB values.

`fec-codec` provides safe Rust encoder/decoder APIs. LDPC reuses the pinned
ldpc-toolbox matrix/decoder; BCH uses a bounded Rust implementation with documented
algorithm provenance and retained licensing. A coding round trip does not prove
DOCSIS interoperability. See [coding evidence](reference/coding-mode.md).

`docsis-ofdm-fec` owns shared scenario and candidate models, exact association,
analytical evaluation, simulation settings/results and document validation. Its
optional `simulation` feature allows analysis-only builds without codecs.
The CLI owns bounded file I/O, cancellation signals and safe new-file output.

Candidate edits preserve verified pilot, PLC and excluded roles. Saved result
claims are recalculated against supplied observations, and captured roles are
reverified. Internal candidate JSON does not deploy or activate a profile.

The downstream modulation target covers the pinned DOCSIS3.1/4.0 FDD definitions
through16384-QAM, while implementation status remains explicit per mode. Device
support requirements and implemented scientific capabilities are separate.
See [modulation coverage](reference/ofdm-support.md) and [public API](reference/public-api.md).
