# Executable HP coverage map

This maps independent requirements in `spec/acceptance/procedural-acceptance-matrix.md` to source tests. It does not replace their execution logs or establish physical hardware evidence. Source/generated and packed consumers run the same entry points on Linux/Windows x64. Current pinned SDK: `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`.

| Stable ID | Scope and observation | Executable evidence |
|---|---|---|
| HP-PROC-01 | Six distinct static identities; passive matching/opening; no probing; commissioned configuration required | `hp_counter_proof::synchronous`, `HpCounterProof.Run`; `hp_legacy_proof::synchronous`, `HpLegacyProof.Run`; all1424 generated cases open without extra messages |
| HP-PROC-02 | DAQ Option001/normal-routing, module/compensation, reference channel, slot/logical channel and duplicate inventory rejection before acquisition | `hp_legacy_proof::synchronous`, `HpLegacyProof.Run`; `hp_admission_proof::synchronous` / `HpAdmissionProof.Run` isolate duplicate-public vs duplicate-native, empty inventory, individually valid conflicting same-slot modules, Option020 Hardware/Unknown, local20 and4096/4097 bounds;53 independent DAQ cases |
| HP-PROC-03 | RF own-model firmware minimum/syntax, later same-family revision, model/option/unknown commissioning rejection | Same legacy procedural functions;152/148/128 model-specific wire cases |
| HP-PROC-04 | B055+006/007 unavailable at construction, no IO; A combined options and C Standard-only remain distinct | Same legacy procedural functions; independent option-boundary cases in each RF shard |
| HP-PROC-05 | Exact immutable endpoint routes; selected ANT cannot gain power from another RFInOut route; unbound endpoints rejected; mutable caller lists frozen | Legacy procedural functions plus `hp_admission_proof::asynchronous` / `HpAdmissionProof.Run` check selected ANT/unbound/exact address negatives before native/worker acquisition; Sync is covered in legacy proof. Admission proofs show valid logical7 still reads the original native address after caller-list mutation; typed API succeeds with exact script |
| HP-PROC-06 | Exact command order, immediate verification/fail-fast, scalar grammar, units/status/error differences, source settings and frozen power bounds | `generated_cases::cases()` / `GeneratedCases.Create()`:593 HP cases +831 regressions; all four frontends in both languages |
| HP-PROC-07 | RF fresh trigger each receiver call, no extra power frequency query, receiver invalidates source readiness, configure leaves output disabled; explicit enable | Independent RF golden lifecycle/repeated-receiver cases; all four frontends |
| HP-PROC-08 | Timeout/Transport/Parse and RF Device/readback mismatch follow-up quarantine; no implicit retry/recovery/reset/cleanup; owned resource close once, borrowed preserved | Counter/legacy procedural functions (RF known0/1 wrong state yields Device then SessionNeedsRecovery without later IO); `hp_async_proof::run` / `HpAsyncProof.Run`; ordinary corpus ownership/closed-handle checks |
| HP-PROC-09 | Native future abandonment/cancellation before and during IO, active host access ends, quarantine, owned/borrowed release | `hp_async_proof::run` / `HpAsyncProof.Run`, all six models; additionally all three RF models cancel pending write; counter-specific proof retained |
| HP-PROC-10 | Genuine pending read expires independent finite deadline; all RF models also pending write; no retry, active access ends | `hp_async_proof::run` / `HpAsyncProof.Run`, sync/native; channel wait/WaitHandle and timeout(pending)/CancelAfter, not delayed injected Timeout |
| HP-PROC-11 | Complete-message adapter preserves payload, bounds allocation, rejects block wrappers/wrong terminator/extra buffered frame; pending empty/incomplete response expires independent deadline | Main `crates/instrument-core/tests/message_body.rs`; .NET `MessageBodyTests.cs`; `OpenTapVisaCompanionTests.cs`. Byte/session tests separate from opaque vendor host tests. Standard connector must select plain text for these devices |
| HP-PROC-12 | Acquired resource on failed/canceled/undelivered open is released; invalid ownership rejected | Pinned SDK `AsyncDriverTests.NativeCanceledOpenReleasesPartialAcquisition`, `CompletedUndeliveredWorkerOpenIsFinalizedByRequestingOwnerCleanup`, `OwnershipRegressionTests`; Rust SDK `native_open_abandonment_and_completed_undelivered_handle_release`, `worker_abandoned_open_and_completed_undelivered_open_are_released_on_worker`. Connector owns cleanup during partial acquisition. Vendor factories perform all fallible validation before IO transfer and no fallible setup after it |
| HP-PROC-13 | Queued cancellation emits no IO; running worker cancellation drains on owner thread; close policy and affinity shared across frontends | Pinned SDK .NET `WorkerRunningCancellationDrainsOnSameThreadBeforeClose`, `WorkerQueuedCancellationSyncFrontendAndReentry`, `PreCanceledRequestsDoNotOpenOrDispatchAndWorkerFrontendsShareLifecycle`; Rust `worker_running_abandonment_retains_affine_owner_through_close`, `worker_queued_cancel_sync_frontend_and_reentry`, `worker_sync_and_async_share_owner_and_first_close_policy_is_not_escalated`. Vendor corpus additionally asserts actual worker thread affinity for every case |
| HP-PROC-14 | Typed neutral DAQ/RF methods and static NativeAOT rooting, Rust/C# sync/async parity | Pinned SDK `validation/driver-sdk/rust-consumer/src/neutral_classes.rs`, `dotnet-consumer/NeutralCapabilityProof.cs`; all six concrete factories rooted by vendor consumer fixture registration. No reflection, runtime JSON or external implementation in SDK |
| HP-PROC-15 | Release-form archive dependencies/content and actual native executable, generated/source and packed consumers; trim/AOT warnings fail intentionally | `instrument-driver verify-package` hosted matrix: linux-x64/win-x64 ×net8.0/net10.0; actual binary format/dependencies/execute records. A successful source IL run alone is insufficient; attach actual final-revision CI artifacts |
| HP-PROC-16 | All three RF models reject NaN/±Infinity in each configure argument before IO; a succeeding typed output-disable proves no quarantine | `hp_admission_proof::synchronous` / `HpAdmissionProof.Run`; exact empty events after each rejected argument followed by complete valid3message transaction |
| HP-PROC-17 | Exact constructor route count bound, empty routes and absent native connector | Admission proofs construct4096 unique endpoints successfully and reject4097 and empty bindings with zero acquisition; each new family advertises no NativeAsync without native connector |

The SDK runtime supplies the whole-operation serialization lease used by every transaction. Device engines never split a transaction into separately scheduled calls. Generic worker/open ownership algorithms are proved in the SDK rather than reimplemented in each proprietary driver. Host deadline enforcement and partial-acquisition cleanup remain connector responsibilities; the vendor cannot impose a deadline by abandoning a still-running blocking host thread.

DAQ representation refinement: native address is derived from validated slot/local, so inconsistent redundant native addresses and negative byte coordinates are unrepresentable. Logical channel uses u32/uint. The constructor also bounds endpoint bindings to4096, matching RF. Hardware/input limits and commissioning remain application responsibilities.

Evidence scope: public0.2.0-alpha.1 packages lack these additions. CI feeds are checksummed development packages from the immutable producer; successful bootstrap consumption does not establish new public package availability. No hardware validation, merge or release has occurred as part of this work.

The Rust validation manifest sets only the `keysight-package-proof` consumer to
release opt-level0, avoiding expensive optimization of its1424-object generated
fixture constructor. Actual driver and SDK dependency crates retain their normal
release optimization; no production manifest or global RUSTFLAGS override is
used. Downstream generic wrapper instantiations in this fixture consumer can also
use opt0. The independently packed SDK producer consumers remain normal release
builds and exercise the typed wrappers. All source/packed/default/async cases and
procedural assertions remain enabled; C# NativeAOT settings are unchanged. This
is an explicit validation-consumer profile, not a claim that its entire executable
uses opt3. The fixture consumer is excluded from distributed driver archives.

The complete Rust proof runs on one explicitly bounded32MiB consumer thread.
This accommodates the large unoptimized fixture constructor on Windows, whose
default main stack is smaller. Sync/native checks stay on this consumer thread;
SDK worker ownership and affinity checks continue using their own worker threads.
Spawn/join errors and panics fail the process; every assertion remains enabled.
Production application/driver stack settings are unaffected.
