# C3 worker transport source preparation

Base: `00c6e3c9f1268e5e2a444a18e2f28e2e51adaf3b`.
Branch: `feature/qt-worker-transport-admission`.
This prepares C3 / REQ-UIU-07 under
[the frozen closure record](../session-closure.md#fixed-closure-list).
The lead owns canonical records and runtime/SESSION acceptance.

## Existing-owner reuse

Reuse `RetentionBudget`, `RetentionReservation` and `Retained`; keep std bounded
queues. `RetainedResources` adds `channel_count` and `slot_count` to the same
checked atomic reservation/release. Named desktop ceilings are 64 simultaneous
channel owners and 256 configured slots, independent application populations.
Scientific footprint contracts and algorithms are unchanged.

`bounded_channel<T>(&budget, capacity)` returns `DeliverySender<T>` and
`DeliveryReceiver<T>`. One actual `Arc<Retained<()>>` pointee/root carries the
channel and configured slots; endpoint handles belong to their containing
admitted Model/Work/State/Job/task. Zero capacity remains a rendezvous channel.
`sender.reserve()` returns a consuming `DeliveryPermit<T>` before backend work.
`permit.send(value)` publishes `Retained<T>`; receiver `recv`, `try_recv` and
`recv_timeout` preserve that frame. Every independently held delivery owns its
actual fixed layout, without a capacity-plus-one estimate. Endpoint populations
can disappear while detached delivery bytes remain charged. Disconnection
returns raw transient execution data; independently guarded children stay owned.
Private std synchronization/bookkeeping is excluded by the approved policy.

`admit_task(&budget, actual_closure)` returns concrete `AdmittedTask`, reserving
`size_of_val(closure) + size_of::<AdmittedTask>()` before boxing its body.
`run_task(task)` retains the final guard through actual return, unwind or discard.
Already admitted inline roots must be Box-moved before whole closure/frame
measurement. No scientific clone or duplicate child visitor is introduced.

## Common worker and concrete callers

`prepare_worker<Input,T,E>(name, capacity, message_count, &budget, operation)`
prepares the actual measured task before any lease. Its exact guarded recovery
slot is `Mutex<Option<(RunLease,Input)>>` plus one Arc root. Checked permit-buffer
bytes are reserved before the finite builder; final storage is an exact Box slice.
Each DATA permit and the separate terminal permit owns its frame charge.
`PreparedWorker::spawn(lease,input)` supplies genuine admission and late input.
Early stop/panic drops unused permits; overpublication returns original data.
Known terminal ownership acknowledges actual completion even when its raw queued
frame is discarded, without polling or releasing an active lease early.

`spawn_worker(name,capacity,message_count,&budget,lease,operation)` is the explicit
already-leased convenience. Resource rejection returns the original lease in the
existing `WorkerSpawnError` I/O context. `WorkerSender::send` takes `&mut self`.
`WorkerReceiver` retains existing receive methods. `WorkerEvent::Message.value`
is now `Retained<T>`; `WorkerTerminal` methods stay unchanged with a final guard.

DS `Worker::prepare` reserves actual profile-ID capacity and exactly one event per
selected profile plus Finished. Model calls it before begin/performance/status
mutation. Its late timing Arc starts at the original construction point.
Preview admits channel, delivery and actual task before clearing status.
US retains its one report inventory before backend claim and returns original
admission on rejection. Preparation Box-moves its already admitted source bytes.

Preparation `Prepared::Configuration` now declares `expected,candidate,delivery`;
`Prepared::Run` declares `controller,target,delivery`. `delivery` is
`Option<RetentionReservation>`: Some for received frames, None only for the static
disconnected notice. The lead-owned workbench match must bind that guard before
candidate/target locals and retain it through adoption or disposal. Direct DS,
preview and US receive branches likewise declare guards before payload locals.

## Source checks and remaining integration

Eleven new cases prepare exact/below independent channel dimensions, checked
population overflow, independently held frames, rendezvous/disconnection, actual
closure size and data-before-guard Drop for return/unwind/discard, genuine terminal
and lease lifetimes, finite inventory/overpublication, preallocation rollback,
real DS and preview old-result preservation, US old-report preservation and
original-controller return from preparation rejection.

Only pinned Rust formatting/parse, source/old-body/drop-order/allowlist/NUL/SPDX
and whitespace review ran here. **Compilation, Cargo, Clippy and every runtime
test/build are UNEXECUTED.** No shared caches or artifacts were written.
Source receipt: `docs/development/local/unified-qt-workbench/worker-transport-source-00c6e3c/`.

The lead owns five-dimension literal/test compatibility, Designer/export/loader
controls and fixed complements, Box moves and callsites, workbench guard forwarding,
canonical evidence and affected runtime validation. C4 loader products/paths and
C5 persistent QML remain their existing separate closure conditions. No SESSION
acceptance or aggregate application closure is claimed by this source preparation.
