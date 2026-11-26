# Generated synthetic CMTS RxMER

These requests and matching binary/provenance pairs were generated in MERLIN.
They are synthetic test inputs, not CMTS field measurements. Their ownership is
separate from the historical upstream fixtures in sibling directories.

| Request | Output | Physical grid | Missing carriers |
| --- | --- | --- | --- |
| [us-50khz.json](us-50khz.json) | [Binary](us-50khz/synthetic-cmts-rxmer.bin), [provenance](us-50khz/synthetic-cmts-rxmer.bin.merlin.json) | First 128, count 512, spacing 50 kHz | 160, 161 |
| [us-25khz.json](us-25khz.json) | [Binary](us-25khz/synthetic-cmts-rxmer.bin), [provenance](us-25khz/synthetic-cmts-rxmer.bin.merlin.json) | First 256, count 1024, spacing 25 kHz | 320, 321 |

All available readings are 35 dB. Both files explicitly use a synthetic CCAP ID,
locally administered CM MAC, fixed timestamp and integrated-CCAP RPD metadata.
The `.bin` files use CCAP-OSSI I18 Table 410's major 2/minor 0 format with
MERLIN's existing most-significant-octet-first adapter profile. Keep each
`.merlin.json` sidecar with its binary: standard bytes alone do not identify
synthetic origin.

See the [generator guide](../../../../docs/user/upstream/synthetic-rxmer.md) for
Rust/Python commands, explicit-value requests, conversion policy and limits.
Generate into a new directory; the examples never overwrite existing paths.

The outputs support offline parser development, including a future RAVEN parser.
They supplement the separately authored [literal decoder vectors](../rxmer/)
and do not establish field interoperability or execute a measured experiment.

Copyright 2026 MERLIN contributors. SPDX-License-Identifier: Apache-2.0.
