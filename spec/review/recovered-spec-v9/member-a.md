# Council member A — recovered-spec-v9 full independent specification review

Member: `/root/hp_council_a_recover`, availability replacement for original member A after execution environment loss destroyed its context and scratch records. Actual model: `gpt-6.1-sol`; reasoning effort: `high`. Stable framing: **examine how a new user would interpret the declared supported behavior**. Rubric: frozen approved v1, exact target `>=9.2` in each category, precision 0.1.

Artifact: `recovered-spec-v9/manifest.json`; repository checkpoint `1e2868902b48725ce71910224107feafde83be02` (verified HEAD). Stage: **specification only**. I reread all four specifications and their current acceptance evidence and independently reconsidered every category. No earlier grade was carried forward. No peer Council ballot/review was inspected. No production file was edited.

All 16 manifest artifact SHA256 values and 11 primary PDF SHA256 values were recalculated and matched. The individual specification identities are:

| Model | SHA256 |
|---|---|
| HP3497A | `66870a951b15188d39b1126d272311860d97bd8e61122c210e5067a1d9df6fef` |
| HP8920A | `c3e32d3d577ea53e7b7a62fb6da55adb0d09b959cbbdd66b69d7db58f870be59` |
| HP8920B | `76391946421a0667707cb22d7132c63205f832b1dfd03789e1b90490c9e0d583` |
| HP8921A | `c0db97000cee047396776372aab0aab8143013c26562c6a123fba2d1a67e433c` |

I also inspected the supplemental `procedural-acceptance-matrix.md` (SHA256 `473cec9b855aa54bb3a67d640a735402823fc3885a789f46b3dc4f41a01eae27`) and acceptance `README.md` (SHA256 `650f0aa72941754d596e15ccfdb8b6ee9c1cfd7c553fd6bedb2a1495c25ccad7`), confirming those match the hash-bound independent r4 evidence record. The independent disposition was inspected as supporting evidence, not accepted as a substitute for my own judgment.

## All 24 scores and threshold-derived votes

| Category | HP3497A | HP8920A | HP8920B | HP8921A |
|---|---:|---:|---:|---:|
| EVIDENCE | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.4 PASS |
| API | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.4 PASS |
| PROTOCOL | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.4 PASS |
| PARITY | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| VALIDATION | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.4 PASS |
| DELIVERY | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |

Member verdict: **PASS each of these four specifications in every category on this exact revision.** This is not an implementation, package-release or hardware-validation result.

## HP3497A

**EVIDENCE — 9.4 PASS.** The 03497-99000 April 20, 1981 primary manual, exact publisher/mirror identities and printed pages make commands, topology, output and scope traceable. Primary printed p77 establishes ASCII output; pp132–133 establish SO1 controller pacing; p157 establishes VC0; pp167–168 establish ranges, overflow and maximum displayed magnitudes; pp172–173 distinguish VT4 and bus GET; pp184–186 and 288 establish analog input routing/triggering; pp191–194 distinguish software compensation from hardware-added voltage and reserve B9. The declared scope clearly excludes uncommissioned topology, direct rear-terminal DVM, other modules, temperature conversion and global output safety. Fresh direct PDF checks confirmed p167's ranges/overflow. Nominal VR1/VR2 values now fit the primary ranges. The newly explicit ±120 V interpretation accurately separates a parser policy from physical device output.

**API — 9.4 PASS.** `DataAcquisitionMeasureVoltageDc(channel,range?)->Quantity(Volt)` accurately captures a single channel-aware acquisition. Positive public IDs map to immutable supplied physical topology, with explicit Option001 and normal routing requirements. The slot/local/native arithmetic and 020 software-compensated reference exclusion are testable. Frozen exact endpoint binding, no fallback inventory, no hidden probes, and selected-route capability checks before obtaining IO keep one chassis from authorizing another. Proprietary behavior remains entirely external to core.

**PROTOCOL — 9.4 PASS.** The exact `VT4AE0VC0VS0VF1VN1VW0SO1VRnAIc` transaction specifies control normalization, one requested acquisition and controller-paced reading under one lease. It avoids GET's channel increment. The 11-byte grammar, positive-only overflow sentinel, global plausibility policy, preallocation bound and finite deadline define deterministic decoder outcomes. Commissioning explicitly establishes a clean boundary and exclusive trigger/controller access, without claiming a framed stale value is detectable. Typed IO failures, active cancellation, quarantine, no automatic recovery and resource-only close are unambiguous. The v9 fixture correction intentionally does not create a new undocumented range-dependent parser rule.

**PARITY — 9.3 PASS.** Rust/C# must share argument validation, channels, units/status/errors, immutable endpoint selection and sync/worker/native behavior. Actual asynchronous IO, deadline/cancellation behavior, static registration and Linux/Windows x64 actual Native AOT execution are specified as acceptance obligations. Their absence as current execution proof is correctly identified at this design stage. Full confidence in implementation parity will require that separate gate.

**VALIDATION — 9.4 PASS.** All 53 unique cases join exactly to 53 provenance entries. I independently checked all Valid DAQ scalar fixtures against exact raw grammar and numeric expectations; selected fixed ranges remain within their overrange bounds. VR1 now returns `+1.25000E-2` = 0.0125 V; VR2 returns `+1.25000E-1` = 0.125 V. The ±120 cases have explicit parser-policy IDs, prose and provenance rather than a physical displayed-value claim. Ranges/null, mapping, strict malformed output, positive-only overload, invalid requests and typed faults form concrete executable cases. The procedural matrix supplies pre-open inventory/route rejections, stale boundary refusal, real stalled IO, cancellation, quarantine, concurrent transaction serialization, ownership and all execution modes. Fake timeout injection is expressly insufficient for actual deadline proof.

**DELIVERY — 9.3 PASS.** Independent external distribution, canonical factory/model selection, generated request/API/frontend checks, compatible immutable SDK/tool pins, source/generated and packed Linux/Windows consumers, actual warning-gated AOT binaries and archive/dependency inspection are concrete planned gates. Examples must state timeout, limits, inventory, commissioning and ownership. The document accurately labels proposed support and makes no existing-distribution or bench claim. Implementation-specific examples and exact execution artifacts remain future required evidence.

## HP8920A

**EVIDENCE — 9.4 PASS.** Its own A.18.00+ 08920-90220 April 2000 Rev B source and 5968-5385E datasheet are exact and page traced. CONFIG pp117–119, RFAN/RFG pp161–163, numeric/state p175, measurement attributes pp177–178 and trigger/freshness pp173/224–227 support the specified flow. Primary datasheet pp2–4 support the separate source/receiver minima and A-specific 007/008/016 rows. I freshly rechecked p2: it explicitly names 007,008,016 under Option055 and separately names those combined configurations in its spectral table. Therefore A's retained combinations have their own evidence and are not inferred from B. The contradictory p3 Option007 signal row is transparently resolved using the explicit p4 continuous-power section. No reverse-power/short-duration row is substituted for measurement scope.

**API — 9.4 PASS.** All five neutral typed operations have clear Hz/dBm/Boolean/Unit results, immutable source/receiver port binding and ANT power exclusion. Configure leaves output disabled and confirms digital settings while preserving modulation. Users are told that this does not establish unmodulated CW, physical settling, calibration or global shutdown. Exact A identity/firmware and supplied options are required without discovery. Frozen endpoint binding and required-operation checking before IO prevent cross-endpoint authorization.

**PROTOCOL — 9.4 PASS.** The common prefix verifies OFF before CONFIG; grouped writes/readbacks are explicitly deterministic software policy. Offset removal, ATT hold OFF, route port, units and digital readbacks remove hidden setting assumptions. Each receiver call activates the display and measurement, removes averaging/reference, uses Auto/FULL/SING and issues a fresh trigger. Strict token/unit/state domains distinguish Parse from a known wrong Device setting. Numeric envelopes and inclusive decimal halfstep policy are deterministic. Session quarantine, bounded response/deadline, external recovery order and no compensating commands are explicit. Commissioning before screen changes and inhibited physical PTT avoid overstating software control.

**PARITY — 9.3 PASS.** Typed outcomes and all advertised modes must match across Rust/C#, with genuine native async and static registration. I independently recomputed every frozen power constant using decimal precision 80 and matched both binary64 decimal and hex values. This provides a precise cross-language bound oracle, together with the decimal tolerance rule. Linux/Windows actual AOT and packed consumer execution remain concrete required implementation evidence.

**VALIDATION — 9.4 PASS.** The 152 unique CatalogCase IDs join all 152 provenance entries. The executable matrix includes configure/enable/state/disable sequencing, source/receiver port/attenuator/option endpoints, resolution and bounds, known-wrong versus unknown units/tokens, inclusive halfsteps, fresh repeated triggers and receiver readiness invalidation. Primary-backed A mechanical combined routes remain present. Procedural obligations cover route mutation/isolation, source argument nonfinites, real stalled IO, deadlines, quarantine reuse, ownership, native async and serialized leases. Synthetic replies are labeled and do not claim hardware results.

**DELIVERY — 9.3 PASS.** External-only commands/options, exact static factory binding and capability freeze, compatible immutable generation tools, regression preservation, actual Linux/Windows source/generated/packed and AOT consumers, and archive/dependency evidence form a concrete planned gate. Proposed support and physical/commissioning limits are explicit. Current execution/package/hardware proof is not falsely substituted with another model's CI.

## HP8920B

**EVIDENCE — 9.4 PASS.** Own-model B.06.00+ 08920-90222 April 2000 Rev D identifies the grammar and firmware boundary; source/receiver pp179–181, CONFIG pp113–115, scalar/state p193 and measurement pp195–196 support the API flow. Datasheet p3 source and p5 receiver/continuous-power tables retain B's numeric differences. I directly extracted primary p9: the complete Option055 description states it can be ordered with all options **except 006 and 007**. V9 explicitly acknowledges conflicting numeric rows and does not use them to grant a supported exception. This conservative declared boundary is appropriately evidence based; it need not assert that every historical modification is physically impossible. Mechanical055 Standard/016 remain supported and B's unsupported combinations do not remove any of the six requested instrument models or neutral operations.

**API — 9.4 PASS.** Five typed operations, route-specific ANT power gating and exact B/firmware/option binding remain coherent. V9 now clearly accepts Electronic Standard/006/007/016 and Mechanical055 Standard/016 only. Invalid combinations fail before connector open, including selected endpoint checks when another endpoint is valid. That boundary is understandable to a new user and cannot be bypassed by claiming the excluded option is installed. Neutral core remains vendor independent, and source setting confirmation and resource-only close are appropriately bounded claims.

**PROTOCOL — 9.4 PASS.** Complete OFF-before-CONFIG, source readback and fresh receiver transactions preserve the exact grammar/error/state semantics. Numeric limits apply only after route admission; invalid 055+006/007 routes have no authorized source or power continuation. Electronic Option006 retains a 30 MHz power band; mechanical Standard/016 use 400 kHz. Fixed source/receiver minima remain distinct. Decoder domains, canonical formatting, decimal tolerances, fixed binary64 bounds, input bounds, timeouts, cancellation/quarantine and no automatic recovery are specific and testable. External physical/commissioning preconditions remain explicit.

**PARITY — 9.3 PASS.** Both languages and every supplied connector mode must reject the same invalid option combinations before IO, use the same selected frozen route, and expose equivalent operation support/error classes. Native async, cancellation and actual warning-gated AOT/source/packed execution requirements are explicit rather than a reflection-based discovery plan. Power constants were independently recomputed and matched as above. Final executable parity still requires implementation review.

**VALIDATION — 9.4 PASS.** The 148 unique case IDs join exactly 148 provenance entries. I checked that no `R:M:Option006:` or `R:M:Option007:` execution case remains and no matching invalid route is present in the factory provenance. Four removed ordinary execution cases are correctly replaced by procedural registration/open assertions: InvalidArgument, zero connector calls, sync/worker/native coverage, and exact selected-endpoint gating with another valid route present. Those are testable pre-open conditions that the post-admission CatalogCase schema cannot express. The remaining matrix still covers supported boundaries, unit/token distinctions, fresh measurements/readiness, exact power endpoints, faults and lifecycle. The spec independently requires unsupported combinations before connector access, so this is not merely deleting difficult tests.

**DELIVERY — 9.3 PASS.** The option conflict and first-release boundary are explicitly documented in the model spec and acceptance README. Canonical static selection, deep-frozen route capabilities, external distribution, compatible tool pins, regressions, actual generated/packed Linux/Windows x64 and Native AOT consumers, and source/archive provenance are required. No excluded route, old package contents or hardware support is implied. Runnable implementation examples must retain the new option restriction.

## HP8921A

**EVIDENCE — 9.4 PASS.** Own-model 08921-90031 December 1998 Rev A, A.15.00+, is correctly the qualifying source; the older A.12 source is historical only. I freshly extracted p43 (numeric GPIB settings preserve an OFF field), p61 (SDC before terminating a stuck measurement), and p81 (RFG amplitude W/DBM/V/DBUV versus TX Power W/DBM). These support the formerly consequential own-model distinctions without inference from A/B. The source/receiver/configuration/state/trigger page index and datasheet pp3–4 identify the exact permitted Standard-only routes, 10 MHz electronic frequency minimum, mechanical400 kHz frequency minimum and 30 MHz power band even with055.

**API — 9.4 PASS.** Five explicit operations, exact model/firmware and endpoint binding, immutable source/receiver port choice, no inherited A/B input-power options and ANT power exclusion make the support surface coherent. Digital source verification is distinguished from settling/CW/calibration; output enable is deliberate and gated. No hardware identity/option probe, global reset/shutdown or unsupported call-processing feature is implied. External-only vendor behavior preserves neutral core design.

**PROTOCOL — 9.4 PASS.** Its own source backs OFF-preserving configure and documented recovery ordering. Each operation has exact prefix/continuation/readback and the receiver requires fresh FULL/SING triggers. V/DBUV are Device for qualified RFG UNIT readback but Parse for TX-power UNIT. Token/state/scalar grammar, inclusive decimal tolerance, fixed power bounds, commissioning boundary, finite deadlines and response limits, cancellation/quarantine and ownership define deterministic behavior. Reopening alone is not advertised as sufficient to repair an uncontrolled device response history.

**PARITY — 9.3 PASS.** Rust/C# must share the own-model firmware/option gate, selected route capabilities, canonical numeric formatting, unit/parser classifications and state/lifecycle behavior in every advertised mode. Fixed numeric bounds remove platform-log ambiguity. Genuine async IO and executed Linux/Windows native/AOT packed consumers are planned mandatory proof, appropriately separate from this specification result.

**VALIDATION — 9.4 PASS.** All 128 unique case IDs join all 128 provenance records. Own-model source/readback/state paths, A.15 baseline, mechanical/electronic limits, source versus TX-unit distinctions, inclusive halfsteps, exact power extrema and immediately adjacent rejects, repeated fresh receiver cycles and reconfigure-after-receiver are concrete executable cases. The procedural matrix adds all factory, deadline/stall, cancellation, quarantine, ownership and mode requirements. Neither manual/synthetic success nor counter CI is substituted for an implemented C backend or hardware measurement.

**DELIVERY — 9.3 PASS.** The discovery/qualification history is accurately labeled, exact own-model prerequisites and Standard-only options are visible, and no existing distribution or physical support claim appears. Independent external static distribution, compatible immutable tools, generated and packed Linux/Windows consumer/AOT execution, warning gates and retained provenance are specific future acceptance requirements. Their current absence is an implementation-stage limit, not a missing specification plan.

## Material conditions, confidence and actionable follow-through

No demonstrated unmet mandatory **specification-stage** condition remains on these exact four artifacts. No rubric amendment or blocking specification edit is requested. The 481 cases and matching provenance are acceptance definitions, not backend test results.

Priorities for implementation review are concrete and unchanged in scope:

1. Enforce the selected frozen endpoint inventory before connector access in every mode, including B Mechanical055+006/007 rejection with zero connector calls and no cross-endpoint authorization. Preserve those explicit cases when constructing the real factory tests (API/PROTOCOL/VALIDATION).
2. Execute the real stalled write/read/incomplete-reply, native async cancellation, quarantine reuse, lease serialization and owned/borrowed/open-failure lifecycle tests. Injected Timeout alone cannot satisfy these requirements (PROTOCOL/PARITY/VALIDATION).
3. Produce source/generated and packed consumer binaries on Linux x64 and Windows x64, including actual .NET Native AOT execution with trimming/AOT warnings gated. Retain commands, exit codes, SDK/tool pins and source/artifact/archive hashes, and inspect core dependencies/archive contents (PARITY/VALIDATION/DELIVERY).
4. Keep the commissioned clean-boundary, inventory/option/firmware/carrier/PTT limits and digital-confirmation-only source semantics visible in runnable examples. Preserve the distinction between synthetic parser-policy values and realizable device transcripts (EVIDENCE/DELIVERY).

Confidence is high for these specification judgments and directly inspected primary semantics. Limits: no physical instrument was exercised; no implementation, new-backend CI, package execution or actual AOT artifact for these four models was reviewed or claimed. Approval here does not satisfy those later mandatory gates.
