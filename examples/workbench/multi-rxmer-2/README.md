# Channel 193 RxMER history

Open `workbench.merlin.json` to load 132 pre-collected RxMER captures and their
companion modulation-profile PNM. The WB embeds every input and defaults to
average linear residual power, Profile A/ID 0, PRBS15 and a 1 ms pilot-estimated
receiver. No run starts automatically.

The exact common grid has 7,600 positions at 25 kHz spacing, from 835.000 to
1024.975 MHz (subcarrier zero 827.600 MHz, first index 296). Channel ID is 193.
A two-hour DPD association tolerance explicitly covers the complete collection.
No matching DS channel estimate was found: supplied response is flat/unity;
receiver correction is estimated from pilots. The channel 34 estimate from the
other examples is not substituted.

Original PNM header capture times are:

| Input | Capture time (UTC, December 4, 2025) |
| --- | --- |
| First RxMER | 03:57:56 |
| Last RxMER | 04:58:55 |
| Modulation profile | 04:59:32 |

The profile follows the last RxMER by 37 seconds. Filename timestamps are
slightly later than the original header capture times; both are recorded in
provenance. The series has a roughly 46-minute gap between two capture bursts.
Hold/interpolate modes use an explicitly configured fixed OFDM-symbol interval,
not the irregular measured cadence. Average mode includes all 132 captures.
A 1 ms default run uses this aggregate; it does not simulate an hour of playback.

Sources are pinned to PyPNM commit
`82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9`. Its demo filenames use one alias,
but the profile header retained a different MAC. Bundled copies consistently use
`00:ca:00:00:00:01`; only the six-byte MAC field changes. Every measurement,
header timestamp and other byte is preserved. This normalization does not prove
that the original sanitized records came from one physical modem.

See [provenance.json](provenance.json) for source/current hashes, Git blob IDs,
identity transformation and original filenames, and retain [LICENSE](LICENSE)
and [NOTICE](NOTICE). These are pre-collected demo observations, not MERLIN's
synthetic RxMER stress variants. Decoder outcomes are not guaranteed error-free.
