# Saved Designer fixture

`designer-flat.json` is the unchanged embedded Designer envelope from
`examples/workbench/cl-192mhz-8mhz-notch/wb-sim-1.merlin.json`. It is synthetic,
uses sanitized modem identity `00:ca:00:00:00:01`, and exercises seeded variation,
tilt, ripple and a notch on the exact profile grid. It is a MERLIN Designer
recipe, not a PNM binary capture or a CableLabs measurement.

Rust and Python regression tests embed this same payload into the existing
`cmce.merlin.json`, select MCE and shorten observation to 1 ms. They verify idle
loading, provenance, cancellation and actual FEC parity. The user's original
10 ms workbench remains unchanged and was run independently through both APIs.
