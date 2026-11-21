MERLIN Basic Python API
=======================

This searchable reference is generated from the actual native ``merlin`` extension.
Signatures, properties and documentation come from the Rust PyO3 adapter. No mock
module, CLI subprocess or Python simulation is used. Importing for documentation
loads type definitions only; it does not load captures or run a simulation.

Use ``merlin.downstream.Simulation`` or ``merlin.upstream.Simulation`` for the
directional sessions and ``merlin.common`` for shared contracts. Root
``merlin.Simulation`` retains its downstream behavior. Alias pages link to the
same native class and its canonical member documentation.

Start with :doc:`workflow`, or select a class below. The module version and native
artifact used for this build are recorded in ``../inventory.json``. The shared
landing page at ``../index.html`` links to the matching Rust API and user guides.

.. toctree::
   :maxdepth: 1

   workflow
   classes
   downstream/index
   upstream/index
   common/index

* :ref:`genindex`
* :ref:`search`
