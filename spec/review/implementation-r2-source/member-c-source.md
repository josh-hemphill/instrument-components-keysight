# C-resume — implementation r2 source findings

**Source inspection complete; final numeric ballot withheld pending the identical frozen final evidence manifest for all members.** I found concrete procedural acceptance coverage gaps, but no demonstrated backend command/parser/state/ownership defect in this source inspection. Final current-revision source/generated, packed, producer-regression and two-OS Native AOT evidence is still required. This is not an implementation approval or a numeric grade.

Member C-resume; configured `gpt-6.1-sol`, reasoning effort `high`. Stable framing: examine behavior at model, option, and execution-mode boundaries. Rubric v1 **APPROVE**, unchanged exact `>=9.2` per category/model. Fresh actual implementation inspection; no specification or previous implementation grades carried forward. No peer ballots read; no production source edits.

Frozen source identity: SDK `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`; Keysight `8729fea9161367816a82c816f4275b165d1b18b2`. Recomputed all **801 source-manifest hashes**:595 SDK files and206 vendor files; all match, including a second check after inspection. Independently counted1424 unique shared cases:56 HP5334A,56 HP5334B,53 HP3497A,152 HP8920A,148 HP8920B,128 HP8921A, plus831 prior cases. Source audit retained in `member-c-source-audit.json`.

## Actionable findings before final evidence freeze

### C-SRC-01 — DAQ procedural map overstates executed inventory distinctions

Category: VALIDATION; model HP3497A. Locations: `validation/rust/src/hp_legacy_proof.rs::synchronous`, `validation/dotnet/HpLegacyProof.cs::Run`, coverage map HP-PROC-02/05, independent procedural matrix DAQ opening rows.

Current helpers test one repeated identical record, which simultaneously violates public and native uniqueness; they do not isolate the two independent conditions. They also omit same-slot module/compensation conflict, empty inventory, local20, and Option020 Hardware/Unknown compensation. Slot5, missing Option001, unconfirmed routing, reference19, unknown module and selected other invalid cases are exercised. The implementation's actual validation predicates appear correct; the deficiency is the promised distinguishing acceptance evidence.

The C# caller-list mutation test clears the input list, then only invokes invalid channel99. That passes both when channel7 was preserved and when the factory incorrectly retained the emptied list. Add a successful strict channel7 acquisition after mutation, with its original expected native address, as well as an unknown-channel rejection. For independent duplicate checks, use same public ID/different native and different public IDs/same native. Exercise same-slot010/020 conflict with otherwise valid unique channels, empty input list, local20,020 Hardware and020 Unknown and require InvalidArgument with zero connector calls.

### C-SRC-02 — RF nonfinite configure and Device-quarantine follow-ups need concrete evidence

Category: VALIDATION; HP8920A, HP8920B, HP8921A. Same legacy helper files; independent procedural matrix faults items3/5/11 and coverage map HP-PROC-03/08.

The literal CatalogCase format cannot encode NaN/±Infinity. Counter nonfinite gate values have explicit helper tests, but I did not find procedural RF configure tests passing nonfinite frequency/level arguments to these actual backends. Add both argument positions for each model, require InvalidArgument and no messages, and perform a subsequent permitted operation to establish that pre-IO invalid arguments leave the session usable/readiness intact. The neutral generated request validators and vendor prepare logic appear correct in source; this is a missing specified integration check.

Legacy follow-up quarantine loops currently exercise Timeout, Transport and Parse. RF golden cases exercise Device/readback mismatch as their final error but do not follow it with another operation. Add one strict known-valid wrong readback yielding Device, followed by SessionNeedsRecovery on another valid operation and recover refusal, with no additional traffic beyond resource close. This demonstrates that a semantic verification failure follows the same uncertainty policy as malformed/transport results rather than relying on source similarity.

### C-SRC-03 — complete or explicitly map remaining constructor/mode matrix rows

Category: VALIDATION/API; HP3497A and all three RF models. Coverage map HP-PROC-03/05 and procedural opening matrix.

The legacy helpers do not presently demonstrate empty bindings and the4097-route rejection boundary, or absent-native mode gating for these four factories. Some enum/identity/scheme/selected-route checks are covered elsewhere or by the shared SDK; map those exact existing tests where appropriate instead of implying one helper proves every row. Current code clearly bounds routes and truthfully advertises native only when provided, and existing counter helpers prove analogous mode absence. The final evidence should distinguish vendor-specific configuration checks from generic SDK algorithms rather than claim full matrix coverage from the1424 ordinary transcript cases.

These findings do not require duplicating generic runtime ownership/cancellation algorithms per vendor backend. Existing SDK tests can satisfy shared contracts when named, executed against the pinned producer and linked accurately. In particular, source ownership transfer has no fallible vendor setup after connector return; the connector remains responsible for cleanup during partial acquisition.

## Inspected six-model behavior by rubric category

The observations below are intentionally unscored until final source/evidence identity is frozen.

### HP5334A

- **EVIDENCE:** Read actual A specification and current native backend/parser plus independent case/provenance structure. Declared functions and deferred A-only memory commands remain distinct, with no invented hardware/release evidence. Current backend hashes match the reviewed formatter implementation, but prior reviews are not substituted for current evidence.
- **API:** Distinct A enum/ID and exact passive hpib identity; commissioned native/internal-gating factory; typed Counter frequency A/B, period A, gate/reset subset. No QueryIdentity or global-safe-shutdown claim; optional C remains unavailable.
- **PROTOCOL:** Rechecked prepare/finish and both sync/native adapters. Gate cache/reset update only after successful IO; function precedes normalization; twelve-significant-digit gate literal is reapplied; strict F/S/width/exponent/zero/letter-O parser and pre-await uncertainty flag agree in both languages. Resource close is idempotent, owned-only and command-free. No source defect observed.
- **PARITY:** Rust `.11E` and C#standard E11 normalize only exponent spelling; current literal midpoint vectors remain in the56 A cases. Actual native IO is awaited; worker/native mode exposure follows connector/feature availability. Final current packed/AOT execution still pending.
- **VALIDATION:** Counter helper covers commissioning, passive manufacturer/scheme selection, unsupported requirement before open, nonfinite gate, owned/borrowed and close failure, quarantine/recovery refusal and native cancellation. Six-model async helper additionally exercises A pending read deadline/cancellation with active-access/ownership assertions. Generic SDK open/worker tests are mapped. Current final evidence not yet received.
- **DELIVERY:** Actual crate/NuGet source inclusion, static explicit registration, source examples, commissioning requirements and unreleased status are inspectable. Current package inventories/consumer evidence remain to inspect at final freeze; old counter CI is not counted.

### HP5334B

- **EVIDENCE:** Read B's separate specification and current source/cases; native subset equality does not import A MR/MS or qualify Option700 CIIL. Unknown/Ciil commissioning remains rejected, not silently language-switched.
- **API:** Distinct B identity/factory and same typed common counter subset. Explicit logical A/B selection does not infer optional C from model naming. No hidden identity/reset/language probe at open.
- **PROTOCOL:** The common implementation has no B branch that changes the frozen native grammar or enables A-only commands. Parser/type/overload/reset/cache/quarantine/recovery and resource ownership behavior were inspected for B, with genuine native calls and no automatic command cleanup.
- **PARITY:** Both languages expose identical B operations, units/status/errors and actual sync/worker/native paths; formatter correction and56 separate B literals remain rooted. New final current-source packed two-OS AOT proof is required.
- **VALIDATION:** Helper loops run B separately, including Ciil/Unknown rejection, native cancellation/quarantine and borrowed/owned close. Six-model deadline tests include B independently. Generic failed-open and worker proofs are mapped rather than reimplemented. Final producer/vendor logs and archives remain uninspected.
- **DELIVERY:** B remains an explicitly selected external implementation with documented language prerequisites and no public alpha.1 support claim. No publication or bench proof asserted; final archive/consumer provenance still pending.

### HP3497A

- **EVIDENCE:** Actual v9 specification and53 independent cases retain physically plausible VR1/VR2 examples and explicitly synthetic ±120 parser limits. Native module/compensation/routing facts remain distinct from software inventory policy. No derived scalar DMM/switch topology is introduced.
- **API:** Rust owns its supplied typed records; C#freezes channels into immutable dictionaries. Native address is derived from validated byte slot/local, so redundant inconsistent/negative coordinates are unrepresentable. Inputs require Option001, normal routing, positive public IDs, supported compensation/local bounds, unique native/public maps and consistent slots. Exact immutable endpoint routes and4096 bound have no fallback inventory.
- **PROTOCOL:** Checked both current engines and shared message adapters. Exact concatenated query, strict11-byte grammar, positive-only overload,120V policy, finite request validation and fail-fast uncertain state align. Any failed step/parse leaves unhealthy; default recovery refuses. Connector clean-boundary/deadline/allocation duties are explicit. No demonstrated parser/transaction bug; source-test coverage gaps C-SRC-01/03 remain.
- **PARITY:** Both languages derive the same native address, range digit and Quantity/Volt/status/errors. True async awaits host calls; sync/worker/native use the same selected core. Shared generic leases serialize entire engine invocation; no transaction is split into separately scheduled calls. Final actual packed/AOT parity evidence remains pending.
- **VALIDATION:**53 literal cases cover operation arguments/wire/parser/value/units and generic lifetime across all four frontends. Legacy/async helpers cover selected inventory negatives, borrowed resources, timeout/transport/parse quarantine, pending read deadlines and native abandonment/cancellation. C-SRC-01 identifies material missing distinctions and the ineffective mutation assertion; final logs cannot compensate until the relevant tests are meaningful.
- **DELIVERY:** Actual external production files and static factories are present, SDK remains neutral, README/spec explain mappings, limits and connector commissioning. CI composes the independent shards and pins producer6524d9a. Final generated/source and packed consumer artifacts/archives are still required.

### HP8920A

- **EVIDENCE:** Actual A specification,152 A cases and implementation preserve A007/008/016 and qualified A.18.00 native family; A's explicit mechanical combinations are retained separately from B's exclusions. Physical power/carrier prerequisites remain commissioning duties rather than protection guarantees.
- **API:** Verified exact model/firmware/port enums, owned immutable endpoint configurations, deep-frozen C#record map, distinct source/receiver ports and selected-route required-operation rejection. Factory descriptor union does not grant an ANT endpoint power; selected operations are independently checked before connector open. Configure/enable/state/receiver readiness is explicit.
- **PROTOCOL:** Reviewed all Rust and C#RF prepare/decode/begin/commit paths. OFF/config/ATT/port/UNIT/settings sequence agrees; all request validation precedes begin/IO. Begin clears readiness, successful configure commits it, receiver success clears it, successful state/enable operations preserve prior readiness. Exact decimal parser/comparator handles inclusive setting halfsteps without rerounding binary64; finite measurement envelopes use fixed constants. Fresh FULL/SING/TRIG per receiver call, no extra power-frequency measurement, typed mismatch/parse/quarantine and command-free close align.
- **PARITY:** Canonical integer-Hz/tenths, exact normalized decimal comparison, identical token domains/Boolean parsing and power constants are implemented in both languages. Native connector awaits are real and optional. Final current packed/two-OS AOT results still pending.
- **VALIDATION:**152 model-specific cases include boundary/readback/unit/state/freshness tests. Procedural helpers cover firmware/options/selected routes and actual async pending read/write cancellation/deadlines; generic worker/open tests map to the shared SDK. C-SRC-02/03 identify remaining matrix evidence gaps, not source defects.
- **DELIVERY:** Distinct static external A factory and explicit source-limited configuration are usable; no generic fallback, probes, reset/shutdown or physical validation claim. Actual packaged source/producer pins and final archives/consumers must be verified against frozen final evidence.

### HP8920B

- **EVIDENCE:** Own B.06.00 manual/spec boundaries and148 B cases are retained. Implementation expressly rejects Mechanical055 with006/007 before opening while preserving Electronic006/007 and supported Standard/016 mechanical routes. It does not reuse A008 or A's mechanical option maxima.
- **API:** Model/config mismatch, unknown enums, firmware syntax/minimum, duplicate/unbound endpoint and selected ANT required power gating are checked before acquisition. B-specific electronic duplex3.5dBm, receiver10MHz and power-option enums are implemented; immutable route records prevent another endpoint's inventory authorizing this one.
- **PROTOCOL:** Inspected B matrix paths through shared engines: B016 mechanical source limit remains−22.5dBm; unavailable006/007 mechanical routes are unreachable. Native OFF/normalization/readbacks/freshness/parser/quarantine semantics agree across languages, including exact decimal comparisons and finite frozen power bounds. No demonstrated code defect.
- **PARITY:** Both language option/firmware/port matrices and errors match in actual code; genuine async and worker support are accurately gated. Source transcript parity is not substituted for pending current packed two-OS AOT evidence.
- **VALIDATION:**148 cases independently cover valid B source/receiver boundaries. Legacy helpers directly reject forbidden055 combinations and check no connector acquisition, endpoint power isolation, duplicate routes, firmware and borrowed close; async helper tests B pending read and write deadlines/cancellation. C-SRC-02/03 remain applicable for nonfinite/mismatch-follow-up and constructor/mode rows.
- **DELIVERY:** B's rejected combinations and unreleased scope are clearly documented; backend stays external with statically rooted concrete factory. Final release-form archives and actual current producer/source/packed/Native AOT proof remain to inspect.

### HP8921A

- **EVIDENCE:** Own A.15.00+ specification/manual identity and128 C cases are maintained. Historical A.12 does not qualify source behavior; Standard-only power and receiver frequency versus power-carrier bands remain distinct. V/DBUV known-RFG versus unknown-TX domain is correctly reflected in current code.
- **API:** Explicit C model/firmware/Standard inventory, immutable independent ports and exact endpoints do not inherit A/B power options. Selected ANT power rejection and no source readiness before configure are enforced; no unsupported mode/probing/global shutdown introduced.
- **PROTOCOL:** Inspected C route through the same actual prepare/decode engine, including source ranges,10MHz/400kHz frequency receiver minima and fixed Standard power bounds. Own-model-supported OFF preservation, normalization/UNIT and fresh trigger/readback sequence, exact decimal policy and uncertainty handling are implemented consistently. No demonstrated source defect.
- **PARITY:** Both languages use C's qualified unit domain/Standard-only matrix and identical typed errors/results/state, static roots and actual native connector awaits. Current source/packed AOT execution evidence remains pending.
- **VALIDATION:**128 independent cases retain C-specific port/range/unit/readback/freshness boundaries. Procedural helpers separately exercise C firmware, unsupported options, route isolation, cancellation and actual pending read/write deadlines. RF evidence gaps C-SRC-02/03 still apply.
- **DELIVERY:** Distinct external static factory and A.15 commissioning/deferred capabilities are documented; public alpha.1 is not represented as containing C. Final SDK/vendor archives, inventories and native execution artifacts must establish current support before any completion claim.

## Neutral SDK/host inspection and independent targeted verification

Read actual neutral catalog request validation/result units and sync/async typed wrappers, neutral packed-consumer capability proof sources, standard VISA connector/provider changes and core sync/native complete-body framing paths. No proprietary command grammar or HP selection exception was found in the inspected production SDK/host additions; it exposes neutral endpoint scheme, response bound, terminator and binary-block policy controls.

The corrected strict host path rejects block headers before removal, wrong configured terminator and extra already-buffered frames. It preserves payload spaces for vendor grammar checks and performs no retry/drain/recovery. Normal SCPI `query` behavior remains separately available. README requires HP standard .NET VISA composition to choose hpib,256 bytes,CRLF and `allowBinaryBlocks:false`; custom connectors must honor the same contract. Detection of arbitrary later stale messages is not proven by these framing checks; commissioning and exclusive access remain necessary.

Independently executed on pinned SDK source:

`cargo test -p instrument-core --test message_body --features async`

Result: **7 passed,0 failed**, exit0. This includes sync/native strict wrapper rejection, payload preservation, no retry/drain, and independently expiring stalled empty/incomplete-body tests. I inspected equivalent C#MessageBodyTests and standard VISA tests; I have not independently executed their test binaries in this source-stage review. This local Rust result is narrow targeted evidence, not four-host package/AOT evidence or hardware validation.

Final action: facilitator should resolve/map the source acceptance gaps, freeze any revised source and common current execution evidence, and supply the same final manifest to all members. I will then review the final revision/evidence and provide36 numeric category assessments with derived votes, confidence/limitations and mandatory-condition status. **No final ballot is cast in this record.**
