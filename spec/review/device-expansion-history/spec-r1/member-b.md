# Council B — SPEC round 1

Member B; actual model `gpt-6.1-sol`, reasoning `high`. Stable framing: consider boundary guarantees and evidence. Rubric v1; target **>=9.2/10** in each category, one decimal. Stage **SPEC**, not implementation. Frozen manifest SHA256: `09da4923574f14e0f92078292287890beef5c9dacdc66d96e4c30176328b3d4c`. All manifest file hashes were independently checked and matched. I read no peer ballot.

## Decision and scores

This three-item specification round needs changes. Exact threshold-derived votes follow; no average is used.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34401A | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.1 FAIL | 9.1 FAIL | 9.0 FAIL |
| 33120A | 9.0 FAIL | 9.1 FAIL | 8.9 FAIL | 9.1 FAIL | 9.0 FAIL | 9.0 FAIL |
| 53132A | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.1 FAIL | 9.1 FAIL | 9.0 FAIL |

The other six requested items remain pending. These scores do not establish nine-item completion or authorize implementation of a failing item.

## Evidence inspected

I inspected the copied three specifications, API-GAPS, all 185 golden cases and their independently authored generator, source metadata, clearance record and prior source audit. Primary-manual checks included 34401A printed112–118/PDF120–126 and printed159/PDF167; 33120A printed56–67/PDF58–69, rendered printed57 frequency table, command/query descriptions around printed145–151 and command summaries; 53132A printed4-53–4-69/PDF221–237, format4-41/PDF209 and arming4-85–4-87/PDF253–255. The image confirms 100 microhertz, despite misleading text extraction.

I also inspected files at the manifest's pinned Git baselines, rather than treating current unpinned working files as normative: Keysight README, Cargo/project/package manifests, acceptance composer, C# consumer and three workflows at `c63cc025d283deb0832dbe2b9c721d8a33412160`; core package verification tool at `1b0704dbb2c4a9f4ffda9f6b1665de12654f672d`. These provide a real reusable source/packed and NativeAOT strategy and exact old package pins. The facilitator's factual pointer to these files was verified directly; it did not supply grades.

## Item findings

### 34401A

**EVIDENCE 9.3:** Own-model source identity, exact scalar format and MEAS commands are inspectable. The specification correctly removes inherited temperature/USB and split-phase claims. Range-as-expected-input, explicit DEF resolution, exact positive overload and software plausibility policy distinguish manual facts from deliberate software restrictions. No material command discrepancy was found in the checked pages.

**API 9.3:** Seven typed DMM methods map to six real physical functions, with the resistance alias explicit and correct physical units supplied per call. Passive exact-model registration, replacement of the old registration, commissioned GPIB/SCPI and deferred serial/split-phase operations avoid accidental support inflation.

**PROTOCOL 9.3:** Complete one-query transactions, exact 15-character grammar, 256-byte bound, LF/body preservation, finite deadlines, positive-only overload recognition, no hidden recovery and owned/borrowed lifetime are concrete. Invalid ranges stay local; uncertain IO/parse/cancellation quarantines.

**PARITY 9.1 / VALIDATION 9.1 / DELIVERY 9.0:** The shared plan deficiencies below prevent a complete, independently executable per-addition acceptance/distribution plan, despite strong existing reusable machinery and 69 useful golden cases.

### 33120A

**EVIDENCE 9.0:** The always-live output, fixed physical impedance versus load accounting, sine-only restrictions, real guard commands, microhertz minimum and coupled amplitude/offset restrictions are supported. However, a documented real load-query reply is omitted from the extracted guard contract's effective numeric policy: printed151/PDF153 explicitly says `OUTPut:LOAD?` returns `50` or `9.9E+37`.

**API 9.1 / PROTOCOL 8.9:** The guard table says a positive finite load other than 50 is Device. The numeric policy says absolute values >=1e30 are Parse. Therefore the actual documented HiZ value `9.9E+37` has contradictory classifications. This matters because known wrong commissioned state is intentionally distinguished from malformed traffic. Explicit precedence/sentinel handling is required; currently an implementer can reasonably select either error. The remaining mutation policy is unusually clear: structural checks precede IO; coupled conflicts occur after queries without a write and quarantine; requested and rounded wire values are checked; completion promises a write, not measured output. Guard serialization, always-on limitations and resource-only close are sound.

**PARITY 9.1 / VALIDATION 9.0 / DELIVERY 9.0:** The 66-case corpus is useful but tests a fabricated positive wrong load `1e6`, rather than the documented HiZ reply that reveals the ambiguity. Add the real response, alongside the shared plan changes below. No hardware or calibrated-output claim is made.

### 53132A

**EVIDENCE 9.3:** The shared manual explicitly covers this exact model. CONFIG source syntax, period/frequency distinction, frequency arming applicability to period, millisecond subset, fresh READ and forced ASCII are supported by inspected pages. No neighboring-model aperture or totalize assumptions are imported.

**API 9.3:** Channels1/2, local selected intent, gate units/defaults and both result units are explicit. Optional channel3 and unrelated timing functions remain deferred. The canonical binary64 millisecond-grid rule avoids a common accidental rejection of valid inputs.

**PROTOCOL 9.3:** The separate CONFIG/arming/format/READ ordering is concrete. ASCII NR3, significant-digit rules, the real invalid-result sentinel and zero/negative policy are distinct; finite host deadlines remain independent of a missing carrier. Whole-operation serialization, no automatic bus recovery and quarantine are stated.

**PARITY 9.1 / VALIDATION 9.1 / DELIVERY 9.0:** Fifty golden cases cover both channels/units, grid edges, fresh reads, parser negatives and each command failure, but the shared acceptance/distribution planning gaps remain.

## Mandatory defect and required changes

1. **33120A known-state reply classification (EVIDENCE/API/PROTOCOL/VALIDATION, mandatory):** Resolve `9.9E+37` HiZ handling explicitly in `spec/33120A.md`, with precedence over the general plausibility cutoff if the intended result is Device. Add a literal own-model HiZ golden case and ensure malformed huge numeric bodies still have a defined classification. This is a demonstrated contradiction in a required command/reply/error rule, not a preference about parser style.
2. **Per-addition executable acceptance integration (PARITY/VALIDATION, all three):** Freeze a shared SPEC-stage reproduction plan identifying the precise external composer/generator/TestKit/consumer entry points, how the new shards replace/add active cases, and how new commissioned factories reach those runners in every mode. The pinned baseline composer only lists hp3497a/hp8920a/hp8920b/hp8921a; the baseline consumer still obtains ordinary non-HP cases via existing Vendor factories. A bare promise to run the same oracle does not specify the necessary new factory/oracle integration. Specify procedural scenarios and expected results for pre-acquisition unsupported commissioning/route checks, passive registration/open, invalid-input healthy follow-up, quarantine follow-up, owned/borrowed close, immutable endpoint bindings, concurrent operation serialization, and genuinely pending cancellation/deadline termination. The current lists identify valuable coverage, but are not yet a reproducible scenario-to-runner plan. Implementation execution is not demanded at SPEC stage.
3. **Explicit SDK/package rollout plan (DELIVERY, all three):** Name the chosen immutable SDK producer/tool pin and exact package dependency versions for this subset, distinguish current bootstrap archives from any future coordinated public versions, and identify the existing source/packed Linux/Windows x64 × .NET8/10 workflow invocation and expected evidence artifacts. Existing CI uses `6524d9...` while composition references `7dd04d...`; the packet should select the intended path rather than asking a reviewer to infer it. Explain which external package contains the new implementations, how the old34401A profile is removed from active generation without losing historical corpus, and where examples/prerequisites and supported/deferred metadata will be updated. Preserve current corpus/regression gates and core/external boundaries. This needs a concrete plan, not a release or publication.

## Confidence and limits

High confidence in the demonstrated33120A contradiction, verified primary command/gate/format evidence, hashes and baseline pipeline facts. Moderate-high confidence in overall SPEC scoring: baseline reuse is substantial, but the new-model integration and rollout are not frozen concretely enough for >=9.2 in the affected categories. No missing implementation, actual platform run or physical bench result was scored as an implementation failure; those are intentionally later-stage requirements. No physical, electrical-safety, calibration, transient-free output or public-release validation is claimed. No rubric amendment requested.
