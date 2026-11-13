# Council member A — implementation round 2 source inspection

Member `/root/hp_council_a_recover`; actual model `gpt-6.1-sol`, reasoning `high`. Availability replacement identity retained. Stable framing: **examine how a new user would interpret the declared supported behavior**. Rubric v1, exact threshold `>=9.2` per category per model. No old specification or implementation grade is carried forward.

**Source findings only. Final numeric ballot withheld pending the identical frozen final evidence manifest supplied to all members.** This record does not claim implementation acceptance, hosted execution, Native AOT execution, hardware validation or publication. No peer ballot was read and no production source was edited.

Frozen source inspected:

- Main `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`: all 595 listed files recalculated and matched the source manifest.
- Keysight `8729fea9161367816a82c816f4275b165d1b18b2`: all 206 listed files recalculated and matched the source manifest.

The manifest includes old review records; hashing them did not involve reading peer review contents.

## Concrete findings requiring correction or evidence

### A-SRC-01 — isolated DAQ inventory acceptance obligations are not exercised

Locations: `validation/rust/src/hp_legacy_proof.rs:210`, `validation/dotnet/HpLegacyProof.cs:53`, `validation/hp-procedural-coverage.md` HP-PROC-02; specification `spec/acceptance/procedural-acceptance-matrix.md` DAQ inventory table.

The source correctly implements the relevant admission checks, but the procedural tests do not independently exercise all explicitly required distinctions:

- Duplicate public ID with different valid native addresses.
- Duplicate native address with different valid public IDs.
- Same-slot conflicting module/compensation declarations using otherwise individually valid inputs.
- Empty inventory.
- Option020 Hardware and Unknown compensation.
- Local20 (and the defined local/module boundary).

The existing duplicate case repeats one identical input, so removing either uniqueness check alone could still leave that test passing. A same-slot conflict using duplicate coordinates would similarly fail for the wrong reason. These are meaningful independent inventory invariants in a device whose wiring is deliberately never probed. Add isolated configurations that violate only the targeted invariant, assert InvalidArgument before connector access and verify zero connector calls. Generic negative coordinates and inconsistent redundant native addresses need no impossible typed test because the public representation excludes them by construction.

The .NET caller-input mutation case clears the caller list, opens the factory, then requests unknown channel99. The negative result alone does not establish that the original valid mapping remained authoritative. After mutation, perform an exact successful channel7 transaction (using the frozen original native0) and verify its traffic/result. Rust transfers owned configuration values and stores only derived owned maps; its immutable-authority mechanism is directly inspectable, but endpoint-isolation tests remain useful.

Impact: VALIDATION for HP3497A. This is an executable-coverage defect against the required acceptance matrix, not a demonstrated defect in the actual inventory validator. Facilitator acknowledged the missing distinctions and intends to correct them in a new frozen revision.

### A-SRC-02 — RF nonfinite pre-IO requirements lack executable source tests

Locations: `spec/acceptance/procedural-acceptance-matrix.md` faults item11; `validation/rust/src/hp_legacy_proof.rs`; `validation/dotnet/HpLegacyProof.cs`.

The explicit requirement is to pass NaN and positive/negative infinity procedurally to each RF configure argument because JSON cases cannot represent these values. A search of all Rust/C# vendor validation sources finds nonfinite tests for counter gate setters only. The RF engines and generated neutral request validators do reject nonfinite inputs, but those checks are not exercised by the stated source evidence.

For HP8920A, HP8920B and HP8921A, independently pass each nonfinite value as frequency with a valid level, and as level with a valid frequency. Assert InvalidArgument, zero traffic, and successful subsequent valid operation so a validation failure cannot silently quarantine the session. These tests validate the public user boundary rather than mirroring the numerical implementation.

Impact: VALIDATION for all three RF models. No incorrect RF nonfinite behavior is established; the missing mandatory procedural proof is the issue.

### A-SRC-03 — explicit endpoint-count boundary lacks a matching procedural test

Locations: `src/hp3497.rs:51`, `src/hp_radio.rs:260`, .NET factory constructors, and procedural matrix RF `Routecount4097` row.

Both implementations visibly reject too many bindings, but no 4097-route test exists in the inspected validation sources. Add explicit 4097 unique valid bindings, require pre-connector InvalidArgument, and retain a small valid-route positive control. A duplicate-address failure must not substitute for exercising the count bound. DAQ adopts the same documented 4096 limit; cover its boundary consistently.

Impact: VALIDATION for DAQ/RF, narrower than A-SRC-01/02. This is a missing planned admission-boundary proof, not a located production bug.

### A-SRC-04 — final current-source execution evidence remains pending

The source context says current hosted source/packed Linux/Windows/AOT proof is running. Final PARITY, VALIDATION and DELIVERY judgments require its final-revision producer SDK test outputs plus all four host-native source/packed proof ZIPs, commands/exit codes, package/content/dependency records and actual binary execution/AOT warning gates. Old counter evidence, prior-revision source runs and IL execution cannot replace this. No final numeric grade is issued while that required evidence is unavailable.

## Actual source inspection by model

### HP5334A and HP5334B — separately considered

Read `spec/legacy/HP5334A.md`, `HP5334B.md`, Rust `src/hp5334.rs`, C# `Hp5334.cs`, both counter procedural sources, fixture registration and shared conformance runner. The two exact canonical model IDs and passive identity rules are separate; shared engine behavior is justified by the A/B manual and their declared identical operation subset. A-only MR/MS is absent; B Option700 CIIL/unknown commissioning is rejected and GAL is not sent.

Actual engines implement logical A/B selection, A-only period, exact FN/HS/MD/WA/GA/RE messages, explicit IN reset, successful-write-only gate caching/reset, strict record width/type/ASCII/exponent/zero grammar and exact positive overflow. Ordinary and native code quarantine before uncertain IO; abandoning a Rust native future does not restore confidence. Native .NET IO receives the cancellation token. Owned close is idempotent and borrowed close is preserved. These are actual source observations, not inferred from the spec score.

The Rust/C# gate formatter differences are deliberately reconciled through standard E11/.11E formatting and explicit exponent normalization. Counter tests include distinct model loops, passive matching without IO, missing native connector, commissioning failure, nonfinite gate arguments, ownership, parse/timeout/transport quarantine, recovery refusal, failed-close exactly once, pre/during native cancellation, and no detached traffic. Generated wire cases still require current execution evidence. No counter-specific production logic blocker was identified during this inspection.

### HP3497A

Read the approved spec, both actual engines and common transaction code. The factory owns/freeze-copies endpoint inventory before connector access. Positive logical channels, slot/local bounds, Option001, normal routing, 010 None and 020 Software local0–18 restrictions, separate logical/native uniqueness and same-slot consistency are visibly implemented; native addresses are derived rather than supplied redundantly. There is no fallback chassis.

Preparation validates request and selected logical channel before IO, uses exact range mappings and the single native transaction, and decodes exact eleven-byte ASCII/positive overload with declared global120V parser policy. Common transaction engines bound ASCII bodies and quarantine before each host access. The implementation does not claim to discover or establish a clean hardware boundary; that remains an explicit connector/commissioning contract. Source behavior appears aligned, but A-SRC-01 and03 leave required validation coverage incomplete on this revision.

### HP8920A

Read both full actual RF engines and A spec. Source/model/firmware/option validation is performed at factory construction and route selection before IO. A retains its explicit 007/008/016 options with Electronic/055 and its own mechanical maxima; source electronic30MHz/mechanical250kHz and receiver electronic30MHz/mechanical400kHz are distinct. Frozen power endpoints match the specification constants. The full output-OFF/CONFIG/source readback and fresh receiver transactions are present exactly, with explicit opt-in enable gated by successful configure. Receiver calls invalidate source readiness. A-SRC-02/03 are validation evidence gaps; no A-specific production behavior defect was located.

### HP8920B

Own-model B firmware and option gates are actual code, including rejection of055+006/007 before connector open. Its electronic006/007/016 choices, supported mechanical Standard/016, B-specific level maxima and receiver minima remain separate from A. Configuration cannot be supplied by a different model or an unbound endpoint. Source/receiver preparation uses the same inspected deterministic engine while consuming B's validated immutable configuration. A-SRC-02/03 apply; no confirmed B-specific production logic defect was located.

### HP8921A

Actual code requires own-model A.15+ firmware, Standard input power only, separate source/receiver ports, electronic10MHz/mechanical400kHz frequency envelopes and the declared power endpoints. All sources use the qualified expanded RFG unit domain DBM/W/V/DBUV while TX-power uses only DBM/W. Known wrong tokens become Device and unknown/malformed tokens Parse. The engine does not infer A.12 qualification or inherit A/B power options. Source and receiver state handling is inspected as for A/B with this C-specific gate and data. A-SRC-02/03 apply; no confirmed C-specific production logic defect was located.

## Shared API, parser, host and delivery observations

- Actual neutral generated DAQ/RF requests, operation metadata and typed sync/async convenience classes expose channel/Hz/dBm/Boolean/Quantity semantics; vendor names/commands are absent from core production Rust/C# source searches. Static external factories own all native grammar, options, topology and finite tokens. No reflection/dynamic device discovery was introduced.
- Both RF parsers have exact bounded decimal grammar, finite binary64 conversion and normalized exact decimal comparisons for inclusive source-setting halfsteps. Frozen dBm constants are literal data; no platform log recomputation is used. Bound and token policies are externally documented. No numerical discrepancy was located in the inspected Rust/C# algorithms.
- Common message engines set unhealthy before every write/query and only restore it after the full transaction and commit. Pre-IO request preparation errors do not start IO or reset readiness. IO/parser/readback failures stop immediately, with no cleanup/reset/retry. Runtime leases cover the complete backend invocation rather than scheduling separate RF steps.
- Inspected actual main sync/native `query_body`/`QueryBody` and strict framing extraction. Plain-text mode rejects a leading `#` before a binary header can be removed, rejects alternate terminators and multiple buffered messages, and preserves payload spaces. Standard SCPI framing remains available separately. The standard .NET connector defaults to ordinary SCPI compatibility; Keysight README explicitly requires `allowBinaryBlocks:false`, CRLF,256-byte bound and hpib scheme for HP. Native body paths use actual native transport awaits, not detached synchronous host IO.
- Inspected byte-level source tests for full/split binary wrapper rejection, exact framing, payload preservation, no retry/drain, and actual pending empty/incomplete reply deadlines in sync/native paths. Vendor async proof loops through all six models with real pending read deadline/cancellation and owned/borrowed cleanup; it additionally stalls writes for all three RF models. The fake host uses channel/WaitHandle or cancellation timeout of a pending future, not an immediate injected Timeout. Execution still needs the final manifest.
- `validation/hp-procedural-coverage.md` usefully separates generic SDK worker/open ownership proofs from model-specific engines. Those SDK references must be substantiated by the final pinned-producer execution artifacts. A coverage map is not itself executed proof and currently overstates DAQ inventory distinction coverage.
- Actual conformance runners statically instantiate all six factories and all previous profile regressions. They execute each generated case through sync/native and both worker frontends, check exact messages, final values/unit/status/errors, resource lifetime, closed-handle behavior and worker affinity. Source contains1424 total cases (593 HP and831 previous regressions); observed execution counts await final current artifacts.
- External Cargo/.NET package definitions contain proprietary code and depend on neutral SDK/SCPI contracts. README expressly states published0.2.0-alpha.1 does not contain these additions, coordinated new versions are needed, and no hardware validation exists. Current dev source/packed archive behavior, dependency contents and actual AOT execution require A-SRC-04 evidence. Release is not authorized or claimed by this record.

## Disposition at source-inspection stage

Report A-SRC-01/02/03 before the next freeze and rerun affected proof on the corrected revision; retain exact new hashes and provide the same artifact/evidence manifests to every member. I found no confirmed production logic defect in the six inspected backends, but incomplete mandatory procedural coverage prevents treating the current source proof map as complete. Once the revised source and final hosted evidence are frozen, review every category anew for each of six models and issue 36 numeric scores/votes. No final scores, pass votes, rubric amendment or implementation approval are issued here.
