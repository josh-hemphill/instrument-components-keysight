# Council A — independent specification review, spec-r1

Member A; actual model **gpt-6.1-sol**, reasoning **high**. Stable framing: consider a new user's interpretation. Rubric **v1**, stage **SPEC**. Frozen packet manifest SHA256 **09da4923574f14e0f92078292287890beef5c9dacdc66d96e4c30176328b3d4c**. I verified that hash and every one of its 17 bound file hashes. I reviewed the copied packet specifications and independent cases, never evolving original specifications or peer ballots.

The manual-derived device transactions are strong. My ballot does **not yet pass the complete specification gate** because commissioning admission for 34401A and the new-device procedural/delivery integration plan remain insufficiently concrete. These are specification requirements; I am not demanding an implementation or execution results at SPEC stage.

Every numeric score uses one decimal. Votes below derive strictly from **>=9.2/10**, with no rounding or averaging across items/categories.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34401A | 9.4 PASS | 9.1 FAIL | 9.3 PASS | 9.1 FAIL | 9.0 FAIL | 9.0 FAIL |
| 33120A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.1 FAIL | 9.0 FAIL | 9.0 FAIL |
| 53132A | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.1 FAIL | 9.0 FAIL | 9.0 FAIL |

## Inspection and evidence

I directly inspected the frozen specs, API-GAPS.md, all three independent case shards and their generator, source metadata and pre-Council review/clearance. I checked actual primary PDF text for 34401A printed 112–118 and 159, 33120A printed 56–67 including the offset inequalities on 62, and 53132A measurement/configuration pages including 4-53–59, frequency/period descriptions and 4-87 gate source/time. I also inspected the rendered 33120A printed-57 frequency table: it reads **100 microhertz**, supporting 0.0001 Hz.

To distinguish reuse from missing plans, I read baseline Keysight README.md, driver-package.json, .github/workflows/ci.yml, spec/acceptance/compose.py, validation/composition/README.md and validation/hp-procedural-coverage.md through `git show c63cc025d283deb0832dbe2b9c721d8a33412160`. Those demonstrate an existing independently packaged driver, static registration, a reusable source/packed NativeAOT matrix and a pinned SDK workflow at `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`. The current baseline compose list contains hp3497a/hp8920a/hp8920b/hp8921a, and its procedural map covers those earlier HP additions. Neither is a first-three integration plan merely because the infrastructure exists.

## Item-specific findings

### 34401A

**EVIDENCE 9.4 — PASS, high confidence.** Own-model edition, publication hash and command/page references are inspectable. The seven neutral methods map to six real physical functions, with RES/2-wire equivalence explicit. MEAS defaults, DEF autoranging, expected-input range semantics and signed 15-character output grammar agree with inspected primary pages. Positive overload is distinguished from software plausibility policy. Native USB, temperature and speculative split-phase support are explicitly excluded.

**API 9.1 — FAIL, medium-high confidence.** Units, range policy, capability exclusions, canonical replacement and alias behavior are clear and useful. However, the spec requires affirmative SCPI language instead of persistent HP3478A/Fluke emulation without specifying the immutable admission input, unknown-value handling or pre-acquisition validation that establishes this requirement. A new user cannot determine the required factory construction contract from this frozen packet. A-R1 addresses this without asking software to infer physical wiring or prove the caller's statement.

**PROTOCOL 9.3 — PASS, high confidence.** Exact one-query transactions, parser grammar, overload/status/unit policy, finite bounded framing, quarantine, resource-only close and native-versus-worker semantics are specified. MEAS state changes and settling/deadline implications are candid. The admission API clarification is separately scored under API; I found no demonstrated wrong command or parser rule in the inspected subset.

**PARITY 9.1 — FAIL; VALIDATION 9.0 — FAIL; DELIVERY 9.0 — FAIL, medium-high confidence.** The 69 independent cases give meaningful positive, boundary, overload, parser and injected failure coverage. The lifecycle/concurrency/admission checks and platform deliverables are named, but no concrete new-device procedural matrix or explicit integration into the inherited external generator/TestKit/CI/package route is bound by the specification. Historical inherited 34401 cases must be removed from the active oracle as specified, yet the concrete replacement/composition and retained-regression plan is absent. A-R2/A-R3 address these gaps.

### 33120A

**EVIDENCE 9.4 — PASS, high confidence.** Own-model primary text and frequency-table image support the sine limits and voltage coupling rules. Always-on output and fixed source impedance are accurately exposed as operational facts. The design distinguishes load accounting from physical termination and does not invent output-enable support.

**API 9.3 — PASS, high confidence.** Four existing neutral methods form a useful typed subset. The commissioned sine/50 Ohm/VPP state is checked before each operation; requested versus rounded-wire voltage checks, settings-only readback semantics and post-query InvalidArgument/quarantine behavior are explicit. Output mutation and close behavior are clear to a new user.

**PROTOCOL 9.3 — PASS, high confidence.** Guard ordering, exact native command tree, state-token and numeric error classes, coupled constraints and write-only setter completion are concrete. The rounded-wire conflict fixture is valuable. Whole-operation serialization prevents in-library interleaving; exclusive ownership is correctly a commissioning condition.

**PARITY 9.1 — FAIL; VALIDATION 9.0 — FAIL; DELIVERY 9.0 — FAIL, medium-high confidence.** The 66 golden cases cover all four methods, important boundaries, guard failures, coupled conflicts and numeric rejection. They do not constitute the procedural integration plan for this model: injected failures cover the frequency-set path, while the counterpart-query/write stages of voltage operations and postfailure follow-up health need a specified matrix. The real pending-IO, ownership, selected-route, nonfinite and platform/package goals remain prose requirements without a bound plan extending the existing entry points. A-R2/A-R3 apply.

### 53132A

**EVIDENCE 9.3 — PASS, high confidence.** The shared primary publication explicitly covers 53132A; the spec does not use neighboring-model provenance alone. Gate ranges and resolution, frequency/period arming subtree, DEF substitution and fresh READ semantics are supported by directly inspected pages. Optional channel 3 remains deferred; no arbitrary expected-frequency range is substituted for the reply parser contract.

**API 9.3 — PASS, high confidence.** Two channels, two measurement units, initial software intent and no-IO setters are explicit. Integer-millisecond subset and binary64 grid equality are precise enough for equivalent implementations. Reapplying intent on each measurement avoids stale state assumptions.

**PROTOCOL 9.3 — PASS, high confidence.** Six-message ordering, format normalization, mandatory exponent/significant-digit grammar, invalid-result sentinel and positive-value policy are concrete. Configuration precedes gate application and READ requests fresh data. Deadlines remain independent of carrier progress; no automatic bus recovery is invented.

**PARITY 9.1 — FAIL; VALIDATION 9.0 — FAIL; DELIVERY 9.0 — FAIL, medium-high confidence.** The 50 independent cases cover channel/unit distinctions, grid boundaries, intent changes, NR3 variants, sentinel/error classes and failures at each traffic position. The packet still needs a concrete procedural plan for rejected-intent preservation/healthy follow-up, concurrent setters and complete transactions, genuine pending cancellation/deadlines and close ownership in every frontend, plus explicit composition and packed-consumer integration. A-R2/A-R3 apply.

## Required changes, in priority order

1. **A-R1 (API, 34401A):** Specify a per-exact-endpoint immutable commissioning/admission input for SCPI language, handling of Unknown/emulation, rejection before connector acquisition, and equivalent Rust/C# construction examples. Keep physical assertions as caller commissioning statements; do not add an implicit language probe or switch.
2. **A-R2 (VALIDATION, all three):** Add a concrete procedural acceptance matrix with stable IDs, setup, trigger and observable assertions. Cover pre-acquisition admission/capability isolation; NaN/infinities and healthy follow-up; error then no-traffic quarantine; serialized concurrent operations; owned/borrowed close; genuinely pending query/write cancellation and deadlines. Distinguish reusable SDK proofs from required new-device integration tests. Include 33120 counterpart-query/write failures and 53132 intent preservation.
3. **A-R3 (PARITY/DELIVERY, all three):** Bind the explicit external generator/TestKit/CI/package route for these additions: shard composition/replacement rules, static factory and proof entry points, exact SDK/tool pins or precise frozen-baseline reuse, generate/check/verify-package commands, and source/packed Linux/Windows x64 × .NET8/10 acceptance matrix. Preserve regressions and historical 34401 evidence while removing its invalid active oracle. Distinguish the required future coordinated package versions from existing public packages.

These can be satisfied by concrete specification additions and executable-plan descriptions; implementation code and actual new-device CI execution remain later-stage requirements.

## Mandatory defects, amendments and limitations

**Demonstrated mandatory model/protocol defects: none found in the inspected declared subset.** Required specification completeness changes are listed above. No rubric amendment is requested.

No implementation, physical validation, merge or publication is asserted. Primary pages were directly sampled for each supported protocol/range/parser family, not exhaustively reread cover to cover. No peer ballots were inspected. The six other requested items—3585B,34980A,34937A,34922A,34950A,TDS1002A—remain pending; this ballot cannot establish completion of the nine-item scope.
