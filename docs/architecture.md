# Architecture

DOCSIS OFDM FEC Toolkit is an offline scientific dependency of PNM Viewer Toolkit.
It has no Qt dependency, PNM binary parser, modem control or network service.

```text
PNM Viewer Toolkit                         This repository
  PNM parsers / source bytes ──typed──> docsis-ofdm-fec (crates/ofdm)
  collection scanner                    types/       domain aliases and roles
  Qt views and background jobs          math/        targets, OFDM rate, QAM
  session/persistence adapters          profile/     models, association,
                                             evaluation, documents, simulation
                                                    │
                                                    v
                                              fec-codec
                                               Rust LDPC / CXX BCH

                                      ofdm-fec CLI -> same public library
```

`fec-codec` owns a replaceable low-level encoder/decoder boundary. Its Rust API
validates bit lengths and values before native entry. LDPC uses the pinned
ldpc-toolbox matrix and layered decoder. The native BCH kernel is a bounded
adaptation of AFF3CT, without its simulation framework or Qt.

`docsis-ofdm-fec` owns original grid identity, provenance metadata, individual
RxMER frames, optional captured profiles/channel observations, candidates,
analytical temporal evaluation and simulations. No IO or GUI types appear in its
public models. Its Rust source forbids unsafe code. Consumers retain the original
capture bytes; source hashes in a decoded document are metadata, not proof that
the toolkit authenticated those bytes.

The `simulation` Cargo feature is on by default. Disabling default features leaves
analytical profile functionality without native codecs. PNM-only decoding in the
viewer need not depend on this toolkit.

The CLI is a bounded JSON adapter with cancellation and safe new-file output.
It uses the same algorithms that a future viewer adapter will call. Work limits
are policy, not invented DOCSIS format constraints. See [documents](reference/documents.md).

Scientific results retain their meaning across transports: measured MER is an
observation; target compliance is an analytical estimate; decoder trials are
simulated outcomes. The toolkit never turns thresholds into measured FEC counters.

Profiles are local hypotheses. Range edits preserve special and unknown roles.
Captured roles must be rechecked against supplied source profiles before a loaded
candidate is trusted. A saved result is historical; evaluation recalculates it.
The internal candidate schema is not a vendor PMA or CMTS configuration format.

The extracted Qt prototype is preserved under ignored
`development/migration/pnm-viewer-prototype/`. It is reference material, not a
second desktop application or a shipped toolkit component.
