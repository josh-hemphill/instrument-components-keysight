# Council B — module SPEC round 1

Member **B**; actual model **gpt-6.1-sol**, reasoning **high**. Stable framing: consider boundary guarantees and evidence. Approved rubric **v1**; stage **SPEC**. Frozen manifest SHA256 **`74fbadf2b5b43c093e2ec6f708b42993f9e00c95fb99387433d39e95e2adfb69`**. Target **>=9.2/10** in every item/category, one decimal. I independently checked the manifest and every bound file hash; all matched. No peer ballots/scores were read and no prior grades were carried forward.

## Scores and votes

These four items meet the SPEC threshold in this ballot. No demonstrated mandatory defect remains in their declared design. Implementation qualification remains separate.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34980A | 9.3 PASS | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| 34937A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| 34922A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| 34950A | 9.4 PASS | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |

## Inspection and primary evidence

I read the five normative module/construction specifications, supporting platform plan, independent clearance, fixture, all four case shards and authoring helper, source metadata and relevant primary manuals. The shards contain7 parent,32 relay,33 mux and123 digital cases: **195 total**, unique IDs and exactly one expected result/error per case. Their common `keysight.34980a` identity is appropriate: modules do not manufacture independently opened sessions. The fixture's separate switch/port namespaces and slot/model mappings agree with the construction contract.

Direct primary-source inspection included:

- **34980A:** mainframe interface discussion confirms actualGPIB/USB/LAN hardware; this subset honestly admitsGPIB only. Programmer's Reference printed422–424 and448–449 verifies ROUT:CLOS/OPEN, single-channel addressing, strict closed-query Boolean meaning, supported module applicability and scan-wide effects.
- **34937A:** own shared guide printed15 verifies28FormC/4FormA, armature latching and no analog buses; printed17 gives configurable power-fail behavior for5A relays. The specification correctly avoids inferring an initial de-energized state or fail-safe isolation from software open/power cycling. Its FormC NC consequence is explicit.
- **34922A:** own shared guide printed34–35 verifies two35-channel latching banks, local1–70, excluded analog-bus locals, scan-only4wire pairing, multiple simultaneous non-scanning closures and power-off channel-state retention. These are the34922A pages, not adjacent reed/FET models.
- **34950A:** Programmer's Reference printed160–161 direction;175–176 HIMP/OFF/ON;181–185 polarity and numeric1/2/4 width/grouping;568–570 SENSE input/output-latch distinction and DEC format;596–597 input memory;756–757 SOURCE's implicit output/width behavior;759 driveACT/OCOL, plus referenced threshold/level/output-memory sections. Actual command parameters and return tokens agree with the guards.

Primary PDFs are hash-bound, with official-source metadata. Source/manual-derived behavior, synthetic test policies and unavailable physical verification remain distinct. The supporting first-three platform plan provides existing tooling mechanics; module construction overrides it, and the module supplement explicitly invalidates reuse of the old SDK producer for these five new operations.

## Per-item rationale

### 34980A

**EVIDENCE9.3:** Mainframe and module manual identities support the admitted addressedGPIB and route/digital subset; source pages substantiate why scan/monitor/autonomous actions cannot be treated as isolated scalar work. The scope is honestly smaller than all physical interfaces/DMM functions and is not claimed bench-qualified.

**API9.4:** One explicit factory owns one physical connection and SDK lease, with module capabilities as class views. The typed external configuration freezes endpoint inventories and derives native addresses from validated slot/local values. Duplicate physical/logical mappings, cross-namespace slot conflicts, unknown enums, absent commissioning and endpoint/capability/mode mismatches fail before acquisition. Separate logical switch and port namespaces permit a shared numeric ID without inventing matrix pairs. Per-endpoint checks prevent descriptor-union capability leakage. Core requests contain no vendor slot/model vocabulary.

**PROTOCOL9.3:** All sibling operations serialize whole transactions under the parent, including digital guards; uncertain failure quarantines every view. Finite deadlines,256-byte bounds, LF/body policy, binary-block rejection, explicit external recovery and owned/borrowed resource-only close are clear. Commissioning excludes autonomous actions for the whole lifetime, addressing the fact that a present-state guard cannot exclude an armed future interrupt/trigger.

**PARITY9.3:** Rust/C# construction fields, enums, five neutral operations and generated metadata must match. Native mode needs a genuine native connector; workers retain operation ordering and thread ownership. Source/generated and checksum-joined packed/native matrix requirements cover Windows/Linux x64 and .NET8/10. Old producer6524 cannot qualify modules; a compatible new immutable producer is explicitly required before final proof.

**VALIDATION9.3:** Parent transcript cases prove sparse cross-module mapping and invalid logical channels. M-PROC01–08 and11–15 add isolated constructor boundaries, deep freeze, endpoint capability isolation, one lease, sibling quarantine, lifetime, genuinely pending cancellation/deadlines and endpoint-count bounds. They are concrete acceptance obligations on actual module factory integration; old helper tests cannot substitute.

**DELIVERY9.2:** All implementations and vendor construction belong to the external Keysight package, while only five neutral operations/tooling change core. The supplement requires a coordinated compatible immutable SDK/tool producer, regenerated catalog/schema/views, exact checksum joins and source/packed validation. Current alpha.1 is explicitly insufficient. The actual future producer/version does not exist at SPEC stage; its creation and immutable qualification are mandatory implementation evidence, not a current compatibility claim.

### 34937A

**EVIDENCE9.4:** Exact own-model topology and common ROUT applicability are verified. Latching behavior, lack of analog bus and the distinction between FormC NC selection and circuit isolation are documented. No neighboring model topology or physical continuity promise is imported.

**API9.3:** Positive logical channels map to local1–32 in one parent's slot, with1–28FormC and29–32FormA. Three neutral single-path methods are useful and precise; paired routes, OpenAll and checked shutdown are absent. The query reports commanded state and all access remains part of the parent lifetime.

**PROTOCOL9.3:** Exactly one addressed close/open/query, no range/global mutation, strict0/1 grammar and parent-wide uncertainty handling define observable behavior. Close is not hardware cleanup; power-fail behavior is not inferred. The software path meaning preserves the FormC NC consequence.

**PARITY9.3:** Both language views use the same schema-generated single-channel operations and common parent; module-specific source/packed cases and procedural proofs run in every frontend. Genuine native gating and platform/native artifact requirements apply without claiming OpenTAP AOT compatibility.

**VALIDATION9.3:**32 independent cases include all three operations at1/28/29/32, true/false, malformed tokens and transport/timeout failures. Procedural coverage adds every valid local and slot boundary, sparse/duplicate/type mapping isolation, parent concurrency/quarantine and one resource lifetime. The NC/absence-of-cleanup semantics are explicit acceptance/documentation requirements.

**DELIVERY9.2:** A module is delivered as a documented capability of the external mainframe factory, with no separate connection/registration fiction. Prerequisites, deferred features and NC consequences are present; compatible new neutral SDK/tool and checksum-joined package/native proof are required before implementation completion or a future release.

### 34922A

**EVIDENCE9.4:** Two35-channel armature banks, local bounds, multiple same-bank direct closures, no automatic pairing for direct routing and power retention match actual34922A manual pages. The contract does not borrow FET break-before-make/exclusivity or reed power-off semantics.

**API9.3:** Three positive logical-channel operations address direct two-wire paths. Analog buses, DMM acquisition/scan,4wire, global OpenAll and cleanup are deferred. A partial binding cannot silently expand its scope, and the single mainframe identity/lease is retained.

**PROTOCOL9.3:** Native addressing and exact route traffic/0-or1 replies are unambiguous. Same-bank closures intentionally do not open other paths. Scan/monitor/autonomous exclusion and no global fallback address real cross-slot state effects. IO/parse/cancellation failures quarantine siblings, with finite bounded host transactions and no physical recovery invention.

**PARITY9.3:** Identical three neutral operations and external mapping rules apply to Rust/C# sync/native/worker consumers. The common qualified SDK/new-producer and native matrix plan is concrete for the actual module integration.

**VALIDATION9.3:**33 cases exercise1/35/36/70, both states, exact route errors and sequential same-bank closure without an intervening open. M-PROC01/02 independently isolate all local/type/slot/duplicate boundaries; parent lifecycle/pending-IO/immutability and native package procedures remain required in addition to transcripts.

**DELIVERY9.2:** The external parent factory is the only distributed runtime entry point for this module. Direct-routing prerequisites and electrical commoning caveat are documented; unsupported DMM/scan/analog-bus features are explicit. Future compatible producer and package/native checks are mandatory, with no old-public-version support claim.

### 34950A

**EVIDENCE9.4:** Own-model/manual command evidence supports numeric width counts, per-channel direction/polarity, bank-first HIMP/memory queries, SENSE output-latch behavior and SOURCE implicit output reconfiguration. Fixed2.5V/3.3VActive settings are admitted commissioning policy, not detected electrical protection or calibration.

**API9.4:** A neutral positive logical port plusu32 byte value/Count result keeps vendor topology outside core. Explicit0–255 admission and zero-valid results avoid inappropriate reuse of positive-count validation. Input reads and output writes are separately gated on the selected endpoint and port before IO. No hidden direction/width/electrical setters or fictitious output-latch-as-input operation is exposed.

**PROTOCOL9.3:** All four selected-bank width queries prevent non-anchor grouping bypass; direction/polarity, bank-first HIMP and both memory flags precede the selected electrical guard and final read/write. HIMP is correctly different fromOFF. The driver never uses MEAS to normalize state. Numeric and byte grammars, wrong-state versus malformed errors, stop-at-first-failure, no failed-guard write, parent quarantine and no physical shutdown guarantee are specified.

**PARITY9.3:** New neutral byte requests/results and zero semantics are generated consistently for Rust/C#. Actual source/packed class/module views must execute identical guarded transactions in all frontends. The required new producer/catalog qualification prevents mixing new operations with old runtime binaries; native/AOT claims have a concrete platform/evidence boundary.

**VALIDATION9.3:**123 cases cover both banks, non-anchor output addresses, zero/255 and256 rejection, direction isolation, selected/bank-first derivation, every guard mismatch and every transaction failure position, strict sampled-byte parsing and no hidden configuration. M-PROC01/10 cover every local101–104/201–204 and grouping; M-PROC06–08/11–13 cover shared parent concurrency/quarantine, real pending host IO and healthy invalid-argument follow-up. These distinguish software conformance from physical pin validation.

**DELIVERY9.2:** Static-byte input/output with fixed electrical commissioning and explicit autonomous-action exclusion is a documented external module subset. Buffering/wide IO/counters/direction/electrical configuration remain deferred. Only vendor-neutral generated byte contracts enter core; independently packaged implementation and source/packed consumer validation require the new qualified producer before completion.

## Required changes and limits

**Mandatory defects:** none demonstrated. **Required SPEC changes:** none. **Rubric amendments:** none.

Confidence is high in hashes, manual-backed topology/command/token checks, zero/direction/group boundaries and parent-session design; moderate-high in implementation feasibility of the specified procedural/platform plan. The new immutable SDK producer, actual generated APIs/OpenTAP regressions, implementation, tests and source/packed native execution remain future evidence. Their absence is not represented as successful implementation. Hardware installation/commissioning is application-asserted; current guards cannot prove future external stability, electrical compatibility, continuity/isolation or calibrated pin levels.

3585B and exactTDS1002A remain pending in this expansion; earlier three-item results are separate records. This ballot is neither an overall nine-item result nor authorization to merge, publish or open hardware.
