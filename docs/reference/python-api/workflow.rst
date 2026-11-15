Workflow and contracts
======================

The high-level API owns validated configurations and original captures. Loading
and configuring never starts simulation. Only :meth:`merlin.Merlin.run` or
:meth:`merlin.Workbench.run` executes the existing Rust engine.

Saved workbench
---------------

.. code-block:: python

   from merlin import Workbench

   wb = Workbench.from_file("examples/workbench/multi-rxmer-2/workbench.merlin.json")
   print([profile.value for profile in wb.profile_ids])  # Still idle.
   for batch in wb.run():  # Releases the interpreter lock during native work.
       for entry in batch:
           print(entry.status)
           if entry.report is not None and entry.report.fec is not None:
               print(entry.report.fec.uncorrectable_codewords.value)

Captured input workflow
-----------------------

Use :class:`merlin.DPD` to discover/load profiles, :class:`merlin.OCD` to derive
settings with optional :class:`merlin.OcdOverrides`, and :class:`merlin.Merlin`
to retain the accepted configuration. Add :class:`merlin.RxMer` and
:class:`merlin.ChannelEstimate` originals separately. Returned capture IDs belong
to that controller; IDs from another controller are invalid.

:class:`merlin.RxMerSelection` supports Each, Individual, AverageAll and
InterpolateAt. Rust performs averaging/interpolation in residual power, validates
exact grids, and preserves missing values. :class:`merlin.ChannelSelection`
supports explicit response, nearest-time matching within a seconds tolerance and
explicit unity. Response selection and :class:`merlin.ReceiverMode` are independent.

Defaults and units
------------------

:class:`merlin.EvaluationOptions` defaults to Each, SuppliedChannel, a 100 ms
source observation, noise seed 41, PRBS seed 0x7fff and zero seconds of profile
association tolerance. Channel selection is initially unset and must be explicit
before running. :class:`merlin.RunLimits` defaults to a 30,000 ms host runtime,
50 decoder iterations and 64 retained detailed words. Source observation excludes
acquisition/completion/drain and is separate from host elapsed time.

A profile derives bandwidth when its full span fits exactly one supported width.
An ambiguous narrower profile still needs an override. Supply that
through :class:`merlin.BandwidthHz` when needed. Omitted OCD timing uses reported
application defaults: CP 512 samples, roll-off 128 samples and interleaver depth 16.
Saved-workbench loading restores its saved settings, rather than these defaults.
Quantity classes validate values in Rust; ``.value`` reads the named primitive
units. Frequencies are Hz, durations are ms, epoch/tolerance values are seconds,
MER is dB and BER is a dimensionless ratio.

Ownership, cancellation and errors
----------------------------------

Accepted options are copied; later edits do not change a pending configuration.
Original capture bytes/provenance remain available through the input objects.
Reports own results independently of controller lifetime. Complete means execution
finished; it does not imply successful FEC. Missing measurements or denominators
remain unavailable rather than becoming zero. FEC/profile metrics expose scope.

Pass :class:`merlin.RunControl` to ``run_with_control`` for cooperative cancellation
from another thread. Completed results survive; unfinished/later jobs retain
partial or not-started status. Concurrent use can raise :class:`merlin.ApiError`
with :class:`merlin.ErrorCode` Busy. Validation, capability, input and preflight
failures also use structured ApiError codes; Python conversion errors remain
TypeError or other native Python exceptions. Check report status for run outcomes.
The native adapter releases the interpreter lock and handles Ctrl-C cooperatively.

Binary/JSON exports use Rust validation and never overwrite destinations. Synthetic
Designer PNM binaries do not encode their full recipe; preserve the exported
sidecar. Loading the binary does not automatically ingest that sidecar.

The shared landing page links complete runnable Rust/Python examples, including
all-profile collection and designed-RxMER workflows. Neither API depends on Qt.


Loaded profile collections
--------------------------

Use ``Merlin.from_profile_file(path, protocol)`` to retain all profiles and
resolve channel settings. Inspect ``sim.profile_ids``, use
``sim.profile_by_id(profile_id)`` or ``sim.select_profile(profile_id)``, and call
``sim.run_all_profiles()`` only when ready to simulate. Entries expose
``profile_id`` even when cancelled before starting. Explicit DPD/OCD loaders
and ``set_profile`` remain supported.
