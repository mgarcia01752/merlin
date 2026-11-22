# Configured channel preview

Rust `merlin::downstream::ChannelConfiguration` and installed Python
`merlin.downstream.ChannelConfiguration` validate and freeze the existing
additional-loss, power-conditioning and continuous-filter settings. Root/legacy
Python imports retain class identity. Construction and previews create no filter
stream, simulation job or source/noise RNG state.

| Rust operation | Python operation | Contract |
| --- | --- | --- |
| `ChannelConfiguration::new(ChannelConfigurationInput)` | `ChannelConfiguration()` for defaults; `from_json(data)` for explicit settings | Existing typed loss/power/filter validation; unity/relative/diagonal defaults. |
| `ChannelConfiguration::from_json(&[u8])` | `ChannelConfiguration.from_json(data: bytes)` | Strict UTF-8 settings with a 64 KiB application transport bound. |
| `settings() -> &ChannelConfigurationInput`; serde serialization | `.settings -> dict`; `.to_json() -> str` | Immutable admitted configuration; Python dictionaries are independent copies. |
| `power_reference() -> ChannelPowerReference` | `.power_reference -> dict` | Configured amplifier/CPA gain, amplitude, optional electrical scale and receiver noise. |
| `preview(&TransmissionConfiguration, &Dpd, Option<&ChannelEstimate>, AssociationTolerance)` | `.preview(transmission, profile, response=None, tolerance=None)` | Original profile and explicit H/unity admission followed by the actual channel planner. |
| `preview_capture(&TransmissionConfiguration, &ProfileCapture, raw_id, Option<&ChannelEstimate>, AssociationTolerance)` | `.preview_capture(transmission, capture, profile_id, response=None, tolerance=None)` | Same planning from an encoded source profile identity, preserving the existing legacy run-ID domain. |

Rust input fields are `additional_loss`, `power` and `filter`. Curated existing
configuration types (`AdditionalLoss`, `PowerConditioning`, `ChannelFilterSettings`,
`EchoPath`, `CableModel`, `LengthUnit`) remain with their scientific owners and
are reachable through the facade. Qt consumers use validated configuration
operations for scientific planning. Python JSON uses the same serde names and
validation, including the retained `power.attenuation_db` CPA key/alias.

`response=None` explicitly selects synthetic unity. Python `tolerance=None`
requires exact original profile/H epoch matching; a typed AssociationTolerance
selects a wider window. Original identity/grid/coverage, unsupported role,
active-null, imported-H/echo ambiguity and captured-FIR fit restrictions remain
with the existing scientific owners. Original capture bytes are preserved.

## Returned observation

`ChannelPreview` / the Python dictionary reports
`reference_plane="configured_transfer_before_receiver"`: the total configured
transfer after amplitude conditioning, original H, additional loss and selected
continuous filter, before receiver correction. Original measured H remains a
separate immutable input. A successful channel plan establishes preview/admission;
full-loop acceptance retains its own gate.

- Original key/source profile ID and actual source provenance.
- Full original-grid `effective_response` I/Q, with explicit excluded-carrier gaps.
- Active offsets/count and physical band endpoints.
- `gain`: at most 1024 min/max groups over the active band, using original grid coordinates.
- At most 128 actual active-carrier `additional_points`, including endpoints,
  with additional/effective gain and physical coordinates.
- Endpoint additional gains and configured power reference.
- Actual continuous-filter coefficients and diagnostics, or empty/null for diagonal mode.

Power references use the existing configured electrical model. An absent
`millivolts_squared_per_unit` retains relative units. `receiver_noise_power` is the
separately configured normalized receiver-input noise. No electrical reference,
captured H normalization or physical cable delay is inferred by this API.
Filter coefficients remain bounded by existing memory/path limits. Channel fitting
and echo composition use the same planner as execution and keep its diagnostics.

```python
from merlin.downstream import ChannelConfiguration

# transmission and profile are previously admitted inputs.
channel = ChannelConfiguration.from_json(
    b'{"additional_loss":{"mode":"manual","tilt_db":-6},'
    b'"power":{"amplifier_gain_db":6,"attenuation_db":2}}')
preview = channel.preview(transmission, profile)
print(preview["additional_low_gain_db"], preview["additional_high_gain_db"])
```

Results own their arrays and retain source provenance. Python releases the GIL
for scientific planning and serialization. Failed admission/preview leaves
accepted settings and original inputs unchanged. Saved-workbench envelope
versioning is separate; these operations preserve existing field semantics.

Traceability: [architecture](../../architecture.md#unified-qt-public-api-boundary),
[requirements](../../requirements.md#req-uiu-09),
[design](../../development/features/unified-qt-workbench/design.md#uiu-api-01-configured-channel-composition-preview-unit)
and [task](../../development/checklist.md#task-uiu-api-01).
