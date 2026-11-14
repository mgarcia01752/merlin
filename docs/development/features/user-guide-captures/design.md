# UI-DOC-01: Current native user-guide captures

Audit every screenshot in the maintained user guides against the current built
Qt workbench. Replace obsolete native captures using the actual dark Ubuntu VM
application and curated example workbenches. Keep capture states explicit: ready
configuration, completed simulation, single/multiple measurements and local TX
preview. Do not present mockups or old layouts as the current interface.

Remove historical proposal links from user guides; preserve separately labeled
design evidence in development records. Consolidate duplicate overview captures
where one current screenshot is sufficient. Reconcile nearby stale instructions
with visible current controls, especially saved workbench support.

This task changes documentation and PNG assets only. The existing executable is
current; no compilation or signal-processing tests are needed. Verify image
integrity, documentation links, actual captured states and diff whitespace.

## Historical design material

These are proposals, not screenshots of the current application:

- [DPD mockup](../../../images/workbench/dpd-design-dark.png)
- [PRBS-TX proposal](../../../images/workbench/prbs-tx-design.html)
- [SA timing proposal](../../../images/workbench/sa-timing-proposal-dark.png)
- [Transmitter proposal](../../../images/workbench/transmitter-workbench-proposal.html)

## Completed review

Reviewed all 32 unique native screenshots embedded in the user guides.
Refreshed 28 using the running Ubuntu Qt application; retained the four current
TX SA captures showing the recently implemented radio controls, metadata overlay,
footer actions and cable grid. Removed six obsolete/duplicate PNGs and their
references. Historical mockups are linked only from development records.
Numerical signal-processing plots remain illustrations of their documented
analyses, not desktop screenshots.

Captured accepted single and multi-RxMER example inputs, staged flat response
without applying it, exact-byte file source without applying it, completed
single-example receiver results and TX instruments. The completed baseline run
compared 1,223,952 bits with zero errors; the average stress example produced
1,424 bit errors. Neither outcome is an assertion about arbitrary inputs.
Restored the accepted single example at handoff; no examples or user input files
were saved or modified. Native Open was captured at the curated examples root.

Corrected outdated Open/Save, profile-import and source run-integration wording.
The architecture's current ownership and lifecycle contracts remain unchanged.
All user-guide relative links and image integrity checks pass; deleted asset
names have no remaining maintained references, and diff whitespace passes.
No source changed and no build or automated simulation suite was run.

| Screenshot | Audit outcome |
| --- | --- |
| [about-dark.png](../../../images/workbench/about-dark.png) | Refreshed native capture |
| [channel-inputs-ready-dark.png](../../../images/workbench/channel-inputs-ready-dark.png) | Refreshed native capture |
| [channel-response-capture-dark.png](../../../images/workbench/channel-response-capture-dark.png) | Refreshed native capture |
| [channel-response-flat-dark.png](../../../images/workbench/channel-response-flat-dark.png) | Refreshed native capture |
| [dpd-input-dark.png](../../../images/workbench/dpd-input-dark.png) | Refreshed native capture |
| [measurement-mouse-zoom-dark.png](../../../images/workbench/measurement-mouse-zoom-dark.png) | Refreshed native capture |
| [ofdm-settings-dark.png](../../../images/workbench/ofdm-settings-dark.png) | Refreshed native capture |
| [open-examples-native-dark.png](../../../images/workbench/open-examples-native-dark.png) | Refreshed native capture |
| [prbs-tx-dark.png](../../../images/workbench/prbs-tx-dark.png) | Refreshed native capture |
| [prbs-tx-file-dark.png](../../../images/workbench/prbs-tx-file-dark.png) | Refreshed native capture |
| [receiver-configuration-dark.png](../../../images/workbench/receiver-configuration-dark.png) | Refreshed native capture |
| [receiver-fec-dark.png](../../../images/workbench/receiver-fec-dark.png) | Refreshed native capture |
| [receiver-qam-ncp-dark.png](../../../images/workbench/receiver-qam-ncp-dark.png) | Refreshed native capture |
| [receiver-response-dark.png](../../../images/workbench/receiver-response-dark.png) | Refreshed native capture |
| [receiver-slicer-dark.png](../../../images/workbench/receiver-slicer-dark.png) | Refreshed native capture |
| [receiver-spectrum-dark.png](../../../images/workbench/receiver-spectrum-dark.png) | Refreshed native capture |
| [receiver-workbench-dark.png](../../../images/workbench/receiver-workbench-dark.png) | Refreshed native capture |
| [rxmer-interpolation-dark.png](../../../images/workbench/rxmer-interpolation-dark.png) | Refreshed native capture |
| [rxmer-single-dark.png](../../../images/workbench/rxmer-single-dark.png) | Refreshed native capture |
| [sa-mouse-zoom-dark.png](../../../images/workbench/sa-mouse-zoom-dark.png) | Current native capture retained |
| [sa-tx-baseband-dark.png](../../../images/workbench/sa-tx-baseband-dark.png) | Current native capture retained |
| [sa-tx-mapped-dark.png](../../../images/workbench/sa-tx-mapped-dark.png) | Current native capture retained |
| [settings-global-dark.png](../../../images/workbench/settings-global-dark.png) | Refreshed native capture |
| [transmitter-message-bytes-dark.png](../../../images/workbench/transmitter-message-bytes-dark.png) | Refreshed native capture |
| [transmitter-messages-dark.png](../../../images/workbench/transmitter-messages-dark.png) | Refreshed native capture |
| [transmitter-output-sa-dark.png](../../../images/workbench/transmitter-output-sa-dark.png) | Current native capture retained |
| [receiver-qam-mer-dark.png](../../../images/workbench/receiver-qam-mer-dark.png) | Paired receiver view replaces the retired TX thumbnail |
| [transmitter-stage-dark.png](../../../images/workbench/transmitter-stage-dark.png) | Refreshed native capture |
| [transmitter-workbench-dark.png](../../../images/workbench/transmitter-workbench-dark.png) | Refreshed native capture |
| [verification-results-dark.png](../../../images/workbench/verification-results-dark.png) | Refreshed native capture |
| [workbench-header-dark.png](../../../images/workbench/workbench-header-dark.png) | Refreshed native capture |
| [workbench-multi-rxmer-dark.png](../../../images/workbench/workbench-multi-rxmer-dark.png) | Refreshed native capture |
