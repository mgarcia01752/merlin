# Open a workbench example

Native **File → Open** and individual input-file pickers start in the curated
examples folder. Open a scenario directory and choose `workbench.merlin.json`.
You can also select that directory's original PNM files in the owning blocks.
Save/Save As keep the operating system's normal destination selection.

| Scenario | Files beside the WB | Default behavior |
| --- | --- | --- |
| `single` | One modulation profile, one RxMER, one DS channel estimate | Single RxMER input |
| `multi-rxmer` | One modulation profile, five RxMER files, one DS channel estimate | Average linear residual power |
| `multi-rxmer-dschan` | Reserved documentation only | Varying channel-estimate playback is deferred |

Both runnable WBs use Profile A/ID 0, a PRBS15 source, independent comparison and
a 1 ms pilot-estimated receiver. Example validation uses the same Rust import and
full request preparation as the UI. This does not promise zero errors: the stress
RxMER inputs deliberately increase residual error power. No simulation starts
automatically when a WB is opened.

All captures share modem `00:ca:00:00:00:01`, channel 34, zero frequency
631.100 MHz, first index 356 and 25 kHz spacing. The 7,480 measurement positions
span 640.000–826.975 MHz and match the selected DPD. Capture epochs differ; the
90,000-second association tolerance is explicit. Synthetic RxMER alternatives
retain the baseline epoch and are not successive measured events. Do not infer
measured time evolution from their filenames or reorder them as capture history.

The WBs embed exact input bytes with hashes, so copying a WB alone preserves
its configuration. Each bundle also carries original files, provenance and
upstream LICENSE/NOTICE. See the [source inventory](../../examples/workbench/README.md).

![Five compatible RxMER inputs restored from the multi-rxmer WB](../images/workbench/workbench-multi-rxmer-dark.png)

## Development and installed paths

Development builds find `examples/workbench/` relative to the executable's
marked source checkout, independently of the shell's working directory.
The former top-level `examples/workbench.merlin.json` moved into `single/`.

Future desktop installers must include these same curated bundles. The native
Open resolver already recognizes these layouts:

| Application layout | Examples root |
| --- | --- |
| Linux shared resources | `../share/merlin/examples/workbench` relative to the executable directory |
| Portable Windows/Linux | `files/examples/workbench` beside the executable |
| macOS bundle | `Contents/Resources/examples/workbench` |

These are packaging contracts and tested path layouts, not a claim that desktop
installers or non-Ubuntu native pickers have shipped. Installed-OS acceptance
must verify file hashes, offline restore and default Open location after moving
the application and launching it from an unrelated directory. When example
resources are absent, native Open remains usable with the OS default location.

![Native Open starts in the curated examples folder](../images/workbench/open-examples-native-dark.png)
