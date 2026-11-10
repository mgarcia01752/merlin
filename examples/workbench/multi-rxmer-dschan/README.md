# Multiple RxMER and channel-estimate inputs — planned

Reserved for one modulation profile, multiple compatible RxMER files and varying
DS channel estimates. Current MERLIN accepts one supplied channel response (or
flat/unity); it does not play a collection of response captures over time.

No runnable WB or capture set is provided here. Channel-estimate ordering,
alignment, complex interpolation, cadence, receiver interaction and WB schema
must be defined and validated first. The installer runnable inventory excludes
this directory. See the [design](../../../docs/development/features/workbench-examples/design.md#ch-est-playback-01-multiple-channel-estimate-inputs-deferred).
