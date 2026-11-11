# MERLIN FAQ

Known status warnings offer suggestions on hover or keyboard focus. Click the
message, or press Enter, Space or F1 while it is focused, to open its FAQ entry.
About → FAQ opens this guide without a network connection.

## RxMER playback needs a longer run

A sequence switches between captured RxMER inputs at the configured
**OFDM symbols between captures** interval. Its last switch occurs at
`(capture count − 1) × interval` symbols. Single capture and Average modes
have no timed transitions. The source observation must include a complete symbol
after the last switch. Acquisition and interleaver drain are separate from this
observation interval.

Increase **Configure WorkBench → Run → Duration per Profile**, or reduce
**RxMER → OFDM symbols between captures** / the capture count. At 25 kHz spacing
and CP512, two captures 1,000 symbols apart switch at 42.5 ms and require a
whole-millisecond UI duration of at least 43 ms. The status help calculates the
minimum for the actual accepted settings; other numerologies can differ.

The UI supports 1–1000 ms per profile. If the calculated minimum exceeds this
limit, reduce the sequence span, or use Single capture / Average for a static
channel model. Suggested changes are never applied automatically.

**Total Active Run Budget** limits computer processing time across the selected
profiles. Raising it does not extend the source observation or resolve this
playback warning. Change Duration per Profile instead. Each selected profile
uses the same observation duration independently.

## No suggestion is available for a message

Only known diagnostic codes have contextual help. An unfamiliar message remains
visible in its original form; MERLIN does not guess a fix from its wording.
Include the message and the affected settings when reporting an issue.
