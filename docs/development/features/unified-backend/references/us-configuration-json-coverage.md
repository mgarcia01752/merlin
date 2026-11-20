> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/us-configuration-json/coverage.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# OFDMA configuration field coverage

Pinned source: **CM-SP-MULPIv3.1-I25-230419**; dependent waveform source:
**CM-SP-PHYv3.1-I20-230419**. Original MULPI pp107,111,112,116,117,122,126,127
were rendered/visually inspected; the complete table prose and referenced
§6.4.3.3–4, §10.5.1, Annex C.1.5.1.11/12 and C.1.3.1.16 were reviewed.
The original PHY rendering and independent numerical cases are recorded in the
[backend validation](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/backend-stages/validation.md). Private corpora are not shipped.

This matrix has **108 atomic field/row dispositions**, covering every Table28
and Table29 row, applicable fixed fields, MAP/P-MAP and declared assignment scope.
It covers source configuration representation/domain validation. Scheduled PRBS
data is executable; ranging, probes, profile-testing bursts and raw MAP scheduling
are inspection-only and reject selected execution. Full MAC/service-flow/security,
TDMA/S-CDMA, RF power management and raw wire encoding are outside this scope.
No unresolved row is hidden in an extension blob. Reserved values reject.

All pointers are JSON pointers with `*` for array items. Rust owner paths are
relative to `crates/phy/src/configuration/`; fixture functions are in
`crates/morgana/tests/configuration_json.rs` unless a JSON example is named.
Required source fields have no guessed defaults. Optional controls retain
omission/null as None; ranging-required absence means source value0. Source
control metadata stays in the complete generic document, including unused presets;
it is never reinterpreted as TX truth or silently executed as a MAC procedure.

| Source / endpoint | Force / applicability | Encoding, units, options / reserved policy | JSON mapping | Rust owner / support | Independent fixtures |
| --- | --- | --- | --- | --- | --- |
| MULPI §6.4.3/Fig32 p107: message_type | Required/conditional as cited | 8-bit =51; other UCD families excluded | /presets/*/channel/message_type | ucd/channel.rs | configuration_json.rs: source_reserved_width_range_and_cross_field_boundaries_reject; examples roundtrip |
| MULPI §6.4.3/Fig32 p107: management_version | Required/conditional as cited | 8-bit =5 | /presets/*/channel/management_version | ucd/channel.rs | configuration_json.rs: source_reserved_width_range_and_cross_field_boundaries_reject; examples roundtrip |
| MULPI §6.4.3/Fig32 p107: upstream_channel_id | Required/conditional as cited | 8-bit nonzero; distinct from CCAP ifIndex | /presets/*/channel/upstream_channel_id | ucd/channel.rs | configuration_json.rs: source_reserved_width_range_and_cross_field_boundaries_reject; examples roundtrip |
| MULPI §6.4.3/Fig32 p107: configuration_change_count | Required/conditional as cited | 8-bit modulo counter; snapshots alone do not increment | /presets/*/channel/configuration_change_count | ucd/channel.rs | configuration_json.rs: source_reserved_width_range_and_cross_field_boundaries_reject; examples roundtrip |
| MULPI §6.4.3/Fig32 p107: minislot_size_ticks | Required/conditional as cited | 8-bit retained, no OFDMA timing effect | /presets/*/channel/minislot_size_ticks | ucd/channel.rs | configuration_json.rs: source_reserved_width_range_and_cross_field_boundaries_reject; examples roundtrip |
| MULPI §6.4.3/Fig32 p107: downstream_channel_id | Required/conditional as cited | 8-bit nonzero management association | /presets/*/channel/downstream_channel_id | ucd/channel.rs | configuration_json.rs: source_reserved_width_range_and_cross_field_boundaries_reject; examples roundtrip |
| MULPI Table28 channel TLV3, p108 | Conditional | 1..128 bytes, MSB-first; only ranging; omitted/null outside that use | /presets/*/channel/preamble_pattern | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV16, p109 | Optional | one byte 0 none,1 unicast IR,2 broadcast IR,3 probe; omitted/null means 0 | /presets/*/channel/ranging_required | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV18, p109 | Optional | u32: classes CM/eRouter/eMTA-or-EDVA/DSG-or-eSTB/eTR bits0..4, reserved5..15=0, extension16..31 | /presets/*/channel/ranging_holdoff_priority | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV19, p109 | Optional | u32 same class/extension domains; reserved5..15=0 | /presets/*/channel/channel_class_id | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV23, p110 | Conditional; IUC13 required | repeated type23, low-nibble IUC; descriptors follow channel fields in wire; JSON retains source inventory | /presets/*/channel/bursts | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV24, p110 | Required | u16 bits0 exclusions,1 unused,2 other channel,3..9 IUC5/6/9/10/11/12/13,10 ranging; others0 | /presets/*/channel/change_indicator_bitmask | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV25, p111 | Required | 9 bytes: reserved4=0, timestamp32, divide-by20-MSB4, minislot32; no simulator epoch inference | /presets/*/channel/timestamp_snapshot | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV26, p111 | Required | wire enums1..11 map to 96/128/160/192/224/256/288/320/384/512/640 samples | /presets/*/channel/cyclic_prefix_samples | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV27, p111 | Required | wire enums1..8 map to 0/32/64/96/128/160/192/224 samples; R<C for scheduled data; IR separate | /presets/*/channel/rolloff_samples | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV28, p111 | Required | wire1=25kHz/4K/Q16,2=50kHz/2K/Q8; no other enum | /presets/*/channel/subcarrier_spacing_khz | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV29, p111 | Required | u32 Hz, multiple of selected spacing; physical bin0 origin retained | /presets/*/channel/subcarrier_zero_hz | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV30, p111 | Required | 4n bytes, inclusive u16 pairs, <=63 pairs/represented TLV; 2K outer0..73/1974..2047,4K0..147/3948..4095 | /presets/*/channel/excluded_bands | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV31, p112 | Conditional for unused bins | 4n bytes, inclusive u16 pairs; no overlap/exclusion ambiguity; remaining bins form contiguous Q-bin slots | /presets/*/channel/unused_bands | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV32, p112 | Required | u8 6..36 plus PHY numerology/encompassed-bandwidth limit | /presets/*/channel/frame_symbols | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28 channel TLV33, p112 | Required | 3 bytes, raw23 bits right justified; no PRBS15 conflation | /presets/*/channel/randomization_seed | ucd/channel.rs | configuration_json.rs: source/control/enum cases; literal examples |
| MULPI Table28.25/Fig33 pp111,113: reserved | Required/conditional as cited | four bits, fixed0 | /presets/*/channel/timestamp_snapshot/reserved | ucd/channel.rs | configuration_json.rs: timestamp literal/reserved/range cases |
| MULPI Table28.25/Fig33 pp111,113: docsis_timestamp | Required/conditional as cited | u32 source timestamp portion | /presets/*/channel/timestamp_snapshot/docsis_timestamp | ucd/channel.rs | configuration_json.rs: timestamp literal/reserved/range cases |
| MULPI Table28.25/Fig33 pp111,113: divide_by_20_msb | Required/conditional as cited | four source bits 0..15 | /presets/*/channel/timestamp_snapshot/divide_by_20_msb | ucd/channel.rs | configuration_json.rs: timestamp literal/reserved/range cases |
| MULPI Table28.25/Fig33 pp111,113: minislot_count | Required/conditional as cited | u32 lowest-frequency frame minislot | /presets/*/channel/timestamp_snapshot/minislot_count | ucd/channel.rs | configuration_json.rs: timestamp literal/reserved/range cases |
| MULPI Table28 channel TLV1, pp108–110 | Inapplicable (T/S or deprecated) | TDMA/S-CDMA modulation rate | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV2, pp108–110 | Inapplicable (T/S or deprecated) | TDMA/S-CDMA center frequency | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV4, pp108–110 | Inapplicable (T/S or deprecated) | legacy DOCSIS1.x descriptor | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV5, pp108–110 | Inapplicable (T/S or deprecated) | legacy DOCSIS2.0/3.0 descriptor | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV6, pp108–110 | Inapplicable (T/S or deprecated) | TDMA/S-CDMA extended preamble | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV7, pp108–110 | Inapplicable (T/S or deprecated) | TDMA/S-CDMA mode | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV8, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA spreading intervals | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV9, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA codes/minislot | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV10, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA active codes | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV11, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA hopping seed | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV12, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA ratio numerator | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV13, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA ratio denominator | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV14, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA timestamp | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV15, pp108–110 | Inapplicable (T/S or deprecated) | deprecated maintain PSD | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV17, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA max scheduled codes | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV20, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA code/hopping selection | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV21, pp108–110 | Inapplicable (T/S or deprecated) | S-CDMA active-code string | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table28 channel TLV22, pp108–110 | Inapplicable (T/S or deprecated) | higher UCD bitmap, explicitly not OFDMA | No JSON property; unknown fields reject | Strict OfdmaChannel | configuration_json.rs: unknown inapplicable fields |
| MULPI Table29 burst23.3, pp114–117; §6.4.3.4 p121 | Conditional per IUC; IR power optional | u16,1..512; IUC3=Mir*Nir,Mir1..8; IUC4=Nfr | /presets/*/channel/bursts/*/preamble_length_bits | ucd/profiles.rs | configuration_json.rs: source/conditional/enum fixtures |
| MULPI Table29 burst23.4, pp114–117; §6.4.3.4 p121 | Conditional per IUC; IR power optional | u16 bit offset; substring within channel preamble; QPSK fine-ranging symbol alignment | /presets/*/channel/bursts/*/preamble_offset_bits | ucd/profiles.rs | configuration_json.rs: source/conditional/enum fixtures |
| MULPI Table29 burst23.19, pp114–117; §6.4.3.4 p121 | Conditional per IUC; IR power optional | u16 positive even Nir; PHY max64/128 at50/25kHz; IUC3 only | /presets/*/channel/bursts/*/initial_ranging_subcarriers | ucd/profiles.rs | configuration_json.rs: source/conditional/enum fixtures |
| MULPI Table29 burst23.20, pp114–117; §6.4.3.4 p121 | Conditional per IUC; IR power optional | u16 positive even Nfr; PHY max256/512; IUC4 only | /presets/*/channel/bursts/*/fine_ranging_subcarriers | ucd/profiles.rs | configuration_json.rs: source/conditional/enum fixtures |
| MULPI Table29 burst23.21, pp114–117; §6.4.3.4 p121 | Conditional per IUC; IR power optional | 1..126 two-byte groups, total <=237 slots; IUC5,6,9..13 only | /presets/*/channel/bursts/*/data_profile | ucd/profiles.rs | configuration_json.rs: source/conditional/enum fixtures |
| MULPI Table29 burst23.22, pp114–117; §6.4.3.4 p121 | Conditional per IUC; IR power optional | two unsigned quarter-dB bytes: starting dBmV/1.6MHz and retry increment; IUC3 only | /presets/*/channel/bursts/*/ir_power_control | ucd/profiles.rs | configuration_json.rs: source/conditional/enum fixtures |
| MULPI Table29.21 pp116–117: modulation_index | Required/conditional as cited | high nibble0 or2..12;1,13..15 reserved | /presets/*/channel/bursts/*/data_profile/*/modulation_index | ucd/profiles.rs | configuration_json.rs: all modulation/pilot indices, count limits; 25kHz mixed/zero fixture |
| MULPI Table29.21 pp116–117: pilot_pattern | Required/conditional as cited | low nibble0 iff zero loading; otherwise patterns1..7/8..14 and channel-wide family | /presets/*/channel/bursts/*/data_profile/*/pilot_pattern | ucd/profiles.rs | configuration_json.rs: all modulation/pilot indices, count limits; 25kHz mixed/zero fixture |
| MULPI Table29.21 pp116–117: additional_minislots | Required/conditional as cited | u8 count after first; groups cannot cross physical frame; exact slot coverage | /presets/*/channel/bursts/*/data_profile/*/additional_minislots | ucd/profiles.rs | configuration_json.rs: all modulation/pilot indices, count limits; 25kHz mixed/zero fixture |
| MULPI Table29.22 p117: starting_power_quarter_dbmv | Optional IUC3 | u8 quarter units; represented for inspection; no ranging power loop | /presets/*/channel/bursts/*/ir_power_control/starting_power_quarter_dbmv | ucd/profiles.rs | us-controls-inspection.json; roundtrip |
| MULPI Table29.22 p117: step_quarter_db | Optional IUC3 | u8 quarter units; represented for inspection; no ranging power loop | /presets/*/channel/bursts/*/ir_power_control/step_quarter_db | ucd/profiles.rs | us-controls-inspection.json; roundtrip |
| MULPI Table29 burst23.1, pp114–118 | Inapplicable/forbidden for OFDMA | non-OFDMA modulation type | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.2, pp114–118 | Inapplicable/forbidden for OFDMA | differential encoding | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.5, pp114–118 | Inapplicable/forbidden for OFDMA | RS correction T | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.6, pp114–118 | Inapplicable/forbidden for OFDMA | RS information bytes | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.7, pp114–118 | Inapplicable/forbidden for OFDMA | 15-bit scrambler seed | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.8, pp114–118 | Inapplicable/forbidden for OFDMA | maximum burst size | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.9, pp114–118 | Inapplicable/forbidden for OFDMA | guard-time TLV explicitly absent/ignored for OFDMA | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.10, pp114–118 | Inapplicable/forbidden for OFDMA | last codeword length | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.11, pp114–118 | Inapplicable/forbidden for OFDMA | scrambler on/off | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.12, pp114–118 | Inapplicable/forbidden for OFDMA | RS interleaver depth | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.13, pp114–118 | Inapplicable/forbidden for OFDMA | RS interleaver size | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.14, pp114–118 | Inapplicable/forbidden for OFDMA | preamble constellation | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.15, pp114–118 | Inapplicable/forbidden for OFDMA | S-CDMA spreader | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.16, pp114–118 | Inapplicable/forbidden for OFDMA | S-CDMA codes/subframe | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.17, pp114–118 | Inapplicable/forbidden for OFDMA | S-CDMA interleaver step | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Table29 burst23.18, pp114–118 | Inapplicable/forbidden for OFDMA | TCM encoding | No JSON property; source reasons retained here | Strict BurstDescriptor | configuration_json.rs: non-data attributes reject |
| MULPI Fig38/§6.4.4 pp122–123: version | Required when optional CAT0 object is present | management version5 | /presets/*/map/version | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: upstream_channel_id | Required when optional CAT0 object is present | u8 matching UCD | /presets/*/map/upstream_channel_id | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: ucd_count | Required when optional CAT0 object is present | u8 matching UCD | /presets/*/map/ucd_count | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: number_of_elements | Required when optional CAT0 object is present | 9-bit count, actual elements<=490 for CAT0 | /presets/*/map/number_of_elements | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: reserved | Required when optional CAT0 object is present | three bits0 | /presets/*/map/reserved | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: channel_allocation_type | Required when optional CAT0 object is present | four bits=0 | /presets/*/map/channel_allocation_type | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: allocation_start | Required when optional CAT0 object is present | u32 minislot count | /presets/*/map/allocation_start | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: acknowledgement_time | Required when optional CAT0 object is present | u32 latest processed minislot count | /presets/*/map/acknowledgement_time | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: ranging_backoff_start | Required when optional CAT0 object is present | u8 exponent0..15 | /presets/*/map/ranging_backoff_start | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: ranging_backoff_end | Required when optional CAT0 object is present | u8 exponent0..15 | /presets/*/map/ranging_backoff_end | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: data_backoff_start | Required when optional CAT0 object is present | u8 exponent0..15 | /presets/*/map/data_backoff_start | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig38/§6.4.4 pp122–123: data_backoff_end | Required when optional CAT0 object is present | u8 exponent0..15 | /presets/*/map/data_backoff_end | ucd/controls.rs | configuration_json.rs: all control/header boundaries; literal roundtrip |
| MULPI Fig39/Table32 pp124–125: sid | Conditional by IUC | 14 bits; request any,request2=0x3ff0,IR unicast/broadcast,station/data unicast,null0; expansion uses expanded IUC | /presets/*/map/elements/*/sid | ucd/controls.rs | configuration_json.rs: SID/IUC/offset/expansion/final-null cases |
| MULPI Fig39/Table32 pp124–125: iuc | Conditional by IUC | 1..7,9..13,15;0/8/14 reserved; data IUC and descriptor association required | /presets/*/map/elements/*/iuc | ucd/controls.rs | configuration_json.rs: SID/IUC/offset/expansion/final-null cases |
| MULPI Fig39/Table32 pp124–125: minislot_offset | Conditional by IUC | 14 bits, nondecreasing; IUC15 instead counts following words | /presets/*/map/elements/*/minislot_offset | ucd/controls.rs | configuration_json.rs: SID/IUC/offset/expansion/final-null cases |
| MULPI Fig39/Table32 pp124–125: expansion_words | Conditional by IUC | additional u32 words for IUC15 only, exact declared count; semantics unexecuted | /presets/*/map/elements/*/expansion_words | ucd/controls.rs | configuration_json.rs: SID/IUC/offset/expansion/final-null cases |
| MULPI Fig40 p126: version | Required when optional CAT1 object present | 5 | /presets/*/probe_map/version | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig40 p126: upstream_channel_id | Required when optional CAT1 object present | u8 matching UCD | /presets/*/probe_map/upstream_channel_id | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig40 p126: ucd_count | Required when optional CAT1 object present | u8 matching UCD | /presets/*/probe_map/ucd_count | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig40 p126: number_of_elements | Required when optional CAT1 object present | 9-bit count, <=128 and <=K for this one CM | /presets/*/probe_map/number_of_elements | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig40 p126: reserved | Required when optional CAT1 object present | three bits0 | /presets/*/probe_map/reserved | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig40 p126: channel_allocation_type | Required when optional CAT1 object present | four bits=1 | /presets/*/probe_map/channel_allocation_type | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig40 p126: allocation_start | Required when optional CAT1 object present | u32 first probe-frame minislot count | /presets/*/probe_map/allocation_start | ucd/controls.rs | configuration_json.rs: probe header boundaries |
| MULPI Fig41/Table33 pp127–128: sid | Required/conditional as cited | 14-bit ranging SID; matches this CM assignment | /presets/*/probe_map/elements/*/sid | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: measure_rxmer | Required/conditional as cited | MER bit: false do not measure,true measure; CM ignores | /presets/*/probe_map/elements/*/measure_rxmer | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: alternate_power | Required/conditional as cited | PW bit selects normal versus source-defined alternative | /presets/*/probe_map/elements/*/alternate_power | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: equalization_disabled | Required/conditional as cited | EQ bit: false enabled,true disabled (source polarity retained) | /presets/*/probe_map/elements/*/equalization_disabled | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: stagger | Required/conditional as cited | St bit repeat skip+1 shifted symbols, may cross frames | /presets/*/probe_map/elements/*/stagger | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: frame | Required/conditional as cited | 2 bits0..3 relative probe frame | /presets/*/probe_map/elements/*/frame | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: symbol | Required/conditional as cited | 6 bits restricted0..K-1 | /presets/*/probe_map/elements/*/symbol | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: start_subcarrier | Required/conditional as cited | 3 bits0..7; PW0 requires start<=skip;PW1 actual start=start%(skip+1),power=-2-start dB | /presets/*/probe_map/elements/*/start_subcarrier | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI Fig41/Table33 pp127–128: subcarrier_skip | Required/conditional as cited | 3 bits0..7; defines spacing and stagger duration | /presets/*/probe_map/elements/*/subcarrier_skip | ucd/controls.rs | configuration_json.rs: all powers/PW/order/boundaries and literal bool roundtrip |
| MULPI §10.5.1 pp454–456; C.1.5.1.11/12 p658; AnnexA: registered | Assignment rules required in their stated registration context | explicit context: before registration only IUC13; afterwards assigned IUCs | /presets/*/assignment/registered | ucd/controls.rs | configuration_json.rs: IUC/SID/context fixtures |
| MULPI §10.5.1 pp454–456; C.1.5.1.11/12 p658; AnnexA: assigned_iucs | Assignment rules required in their stated registration context | one or two distinct5/6/9..13; source46.11 values1..4/7..8/others reserved; empty before registration | /presets/*/assignment/assigned_iucs | ucd/controls.rs | configuration_json.rs: IUC/SID/context fixtures |
| MULPI §10.5.1 pp454–456; C.1.5.1.11/12 p658; AnnexA: testing_sid | Assignment rules required in their stated registration context | optional lower14-bit unicast SID, distinct from SID cluster; no testing-burst execution | /presets/*/assignment/testing_sid | ucd/controls.rs | configuration_json.rs: IUC/SID/context fixtures |
| MULPI §10.5.1 pp454–456; C.1.5.1.11/12 p658; AnnexA: ranging_sid | Assignment rules required in their stated registration context | explicit existing ranging SID, unicast1..0x3dff | /presets/*/assignment/ranging_sid | ucd/controls.rs | configuration_json.rs: IUC/SID/context fixtures |
| MULPI §10.5.1 pp454–456; C.1.5.1.11/12 p658; AnnexA: sid_cluster | Assignment rules required in their stated registration context | explicit existing one-CM association; bounded to256 entries as application policy | /presets/*/assignment/sid_cluster | ucd/controls.rs | configuration_json.rs: IUC/SID/context fixtures |
| MULPI §10.5.1 pp454–456; C.1.5.1.11/12 p658; AnnexA: cm_mac | Assignment rules required in their stated registration context | exact six-byte one-CM association, never filename-derived | /presets/*/assignment/cm_mac | ucd/controls.rs | configuration_json.rs: IUC/SID/context fixtures |

## Additional model and execution boundaries

- Schema/header: `morgana.us-configuration`, version1, named preset inventory and
  explicit selection. Vendor template IDs (including224) are optional provenance.
- PHY bandwidth is an explicit geometry input, not an invented MULPI TLV.
  Encompassed span, all CP/RP enums, numerology-dependent frame limits and
  consistent channel-wide pilot family are validated in shared Rust.
- Preamble options are retained for IUC3/4; scheduled data does not acquire a
  generic preamble. Initial/fine-ranging waveforms and PHY probing remain deferred.
- OFDMA Table29 guard-time TLV9 is absent. Separate synthetic zero-sample
  observation guards are engineering settings outside the complete tapered burst.
- Type23 descriptor IUC1/2 contains no attributes. Data IUC13 remains required
  regardless of the selected data profile. No universal robust QAM order is inferred.
- MAP expansion payload words are retained as explicit source words. No expanded
  IUC meaning is invented; raw MAP/P-MAP execution is unavailable.
- Testing SID is not a license to label PRBS payload as the specified OUDP testing
  burst: its segment header, Ethernet counting pattern, CRC and CCF are unimplemented.
- Ranging holdoff/class bits0..4 and16..31 are retained; reserved5..15 reject per
  Annex C.1.3.1.16 p634. No live class-eligibility/registration timer is executed.
- C.1.5.1.9/10 RF power-window/level control, C.1.5.1.13 error transactions and
  unrelated full TCC actions are outside the bounded assignment/probe configuration
  scope. Source registration/DBC transport is not simulated.

## Structural schema and runtime acceptance

The [schema](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/schemas/us-configuration-v1.schema.json) rejects unknown
structural fields and records types/enums/counts. Source conditional rules,
reserved masks, band mapping, assignment, budgets and executable support remain
authoritative in Rust. Fixtures exercise every represented control in a roundtrip,
with independent boundary/enum mutations and real Rust/native load/run parity.
See [examples](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/examples/configuration/README.md).

The generic source configuration is distinct from binary UCD/MAP capture support,
PNM observations and result schemas. Capture-format tasks remain separately pending.
