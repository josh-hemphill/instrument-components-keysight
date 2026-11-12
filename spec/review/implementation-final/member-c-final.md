# Council member C — final implementation-r6 ballot

Member C, replacement for an actual unavailable execution context, as recorded in `MEMBERSHIP-RECOVERY-2.md`. Actual model: **gpt-6.1-sol**; reasoning: **high**. Neutral framing: consider model, option and execution boundaries. Rubric **v1**. Before grading I submitted **APPROVE**, with reasons and no amendments, in `member-c-rubric-ballot.md`. No peer numeric ballot or assessment was read; no earlier C grade exists or is carried forward.

**Vote: PASS for every category of every model for the declared manual/simulator-validated software scope.** No mandatory acceptance defect demonstrated. This is one independent member ballot; the facilitator must still obtain and aggregate all three valid ballots. It is not hardware qualification, publication, merge, or release approval.

## Frozen identities and inspected evidence

- Main `1b0704dbb2c4a9f4ffda9f6b1665de12654f672d`; external Keysight `3d384a42a3dbb460314b77a4174abefdd4015190`. Both working trees were clean. I independently rehashed **all824 tracked files**, with zero mismatches.
- Source manifest SHA256 `13a78c6c6956e0fec18a3934a4b337d2eb3d9ea8ac2807acdb2e3e82c66bc530`; final evidence manifest SHA256 `4ff9bffc3d4b198ac2924a471628163f5d74401ff7489e01d0cdde88952d9c25`. Manifest identities verified directly. Peer audit files named within the manifest were not opened; their findings are unnecessary to my conclusions.
- Read all six `spec/legacy/HP*.md` model specifications, the actual Rust `hp5334.rs`, `hp3497.rs`, `hp_radio.rs`, `message_backend.rs`, and corresponding C# `Hp5334.cs`, `Hp3497.cs`, `HpRadio.cs`, `MessageBackend.cs`. Inspected neutral API metadata/generated wrappers, OpenTAP steps, host/session framing diffs, standard VISA ownership guard and its tests, procedural tests/coverage map, consumers, packaging manifests, README and pinned workflows.
- Independently checked all12 primary research PDF hashes against `provenance/hp-manual-sources.json`. Read retained scanned-page OCR and extracted actual PDF pages afresh for model-specific RF command syntax, 8920A combined option rows, 8920B page9 availability exclusion, and 8921A pages43/61/81 and datasheet pages3–4. Counter pages3-80/84 and DAQ pages77/172/186 independently support the key grammar/trigger distinctions. Manual identity/page references and synthetic policy are distinct from hardware observations.
- Rechecked corpus totals: HP5334A56, HP5334B56, HP3497A53, HP8920A152, HP8920B148, HP8921A128: **593 HP +831 existing regression =1424**. Verified complete unique provenance ID joins for all HP cases and spec SHA joins for all four reconstructed DAQ/RF specifications. Ran `compose.py --check` and the actual SDK `generate-tests ... --check`; both exited0.
- Actual vendor run **43322642590**, exact vendor HEAD, has all9 jobs completed/success: acceptance; four SDK platform/framework cells; four driver cells. Each driver cell covers generated-source and packed Rust default/async and actual .NET NativeAOT, Linux/Windows x64, net8/net10. Inspected downloaded JSON/job records and retained phase logs, not just a proposed workflow.
- Re-executed `member-c-audit.py` against the eight downloaded ZIPs: **264 phase log hashes/expected exits, 12 native binary hashes/formats, 40 checksummed core-package joins**, plus actual packed Rust HP source equality, all passed. Windows binaries are AMD64 PE32+ with absent CLR directory; Linux binaries are x64 ELF. Core archive dependency/member inventories remain independent of HP/Keysight implementation.
- ZIP IDs audited:13992515568(driver Windows/net10),13991486481(SDK Linux/net8),13991458163(driver Linux/net8),13991278021(SDK Windows/net10),13990928706(driver Linux/net10),13990689310(driver Windows/net8),13990598373(SDK Windows/net8),13990303376(SDK Linux/net10). Full binary/archive hashes and phase observations are in `member-c-artifact-audit.json`.
- I additionally extracted and **executed the downloaded Linux/net8 generated and packed NativeAOT DriverProof binaries myself**, each exit0 with1424cases×4frontends and all six-model procedural checks; also executed the downloaded SDK AotConsumer, exit0, including six added neutral typed operations and185 shared SCPI cases×4frontends. Record: `member-c-replacement-native-execution.json`, SHA256 `83daa83bf9fa77a7e667daf89ac5d673f3af793c3a3580d20067c5bf5c9d9363`. Windows execution is established by inspected hosted records, not a claim of local Windows execution.
- Independently checked current-main CI evidence hashes and revision: full CI **43314190052** and .NET **43314190048**, completed/success. SDK producer is immutable **6524d9a3c1a1627f3fed8e4fbdca0f577220434c**. The current-main delta is the allocating VISA connector's Owned-only admission guard and its separate tests/docs/evidence, not a changed driver/API hidden behind that producer pin.

## Scores and threshold-derived votes

Every displayed score is out of10, precision0.1. Every cell's vote is derived from the exact **>=9.2** threshold; no averaging, rounding up or category carryforward applies.

| Model | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| HP5334A |9.4 PASS|9.4 PASS|9.4 PASS|9.5 PASS|9.4 PASS|9.3 PASS|
| HP5334B |9.4 PASS|9.4 PASS|9.4 PASS|9.5 PASS|9.4 PASS|9.3 PASS|
| HP3497A |9.3 PASS|9.4 PASS|9.3 PASS|9.5 PASS|9.4 PASS|9.3 PASS|
| HP8920A |9.4 PASS|9.4 PASS|9.3 PASS|9.5 PASS|9.4 PASS|9.3 PASS|
| HP8920B |9.3 PASS|9.4 PASS|9.3 PASS|9.5 PASS|9.4 PASS|9.3 PASS|
| HP8921A |9.4 PASS|9.4 PASS|9.3 PASS|9.5 PASS|9.4 PASS|9.3 PASS|

## Evidence supporting every model/category score

### HP5334A

**EVIDENCE9.4:** Own model specification identifies05334-90047/December1993 with PDF hash and printed3-78–89 references. Inspection confirms ASCII width/alpha units, documented special zero and exact overflow, math/wait/restart commands and the ambiguous shared ID response. MR/MS remains A-specific and deferred; optionalC and other modes are explicitly outside the first subset. The56 separately identified cases have complete provenance, with synthetic negative cases labeled as policy.

**API9.4:** `hp5334::Factory`/`Hp5334Factory` statically identify `keysight.hp5334a`, require affirmative native/internal-gating commissioning and a nonempty hpib endpoint, and expose neutral Counter operations/typed units. Channel1/2 is logical selection, period requires A, and no identity/reset probe or hidden generic fallback runs on open. Explicit reset is a supported operation; safe global shutdown is deliberately unavailable. Unsupported native mode/required operations reject before acquisition.

**PROTOCOL9.4:** Actual source constructs FN1/FN2/FN4 before HS0,MD1,WA1 and RE, reapplies a successfully cached GA gate, uses matching12-significant-digit E11 wire formatting, and resets cached channel/gate only after successful IN. Strict19-character native parsing plus the separate zero/overflow cases rejects controls, wrong units and malformed widths. Healthy state is cleared before uncertain IO/await and restored only on successful parse; no retry or recovery traffic; owned release is idempotent and borrowed resources preserved. Finite/bounded complete IO is a documented connector contract with tested standard framing support.

**PARITY9.5:** Both source implementations use equivalent requests, wire/state updates, quantity units/status and errors. Separate56case A set executes through sync/native/worker-async/worker-blocking in both languages on both operating systems; actual NativeAOT artifacts and checksummed SDK joins establish static consumption. Native code awaits actual native IO; the native-only fake fails if any sync acquisition/IO is used.

**VALIDATION9.4:** The56wire cases cover channels, period rejection, gate boundaries/formatting, reset and strict parser negatives; procedural counter/async proofs add passive matching, nonfinite rejection, ownership, cancellation/quarantine and genuinely pending reads under independent deadlines. Exact scripts, values, errors, complete consumption, close count, affinity and closed handles are asserted. Current regression/generation and actual source/packed execution evidence apply to this model, not inferred from another counter.

**DELIVERY9.3:** External production archives contain the HP counter implementation and only neutral runtime dependencies, exclude the validation fixture harness, and are consumed in the hosted matrix. README has usable Rust/C# static registration examples, native-versus-worker guidance, Option700 caveat scoped to B, response/deadline/ownership prerequisites and unreleased/public-version distinction. No new release is asserted. Minor practical limit: real hardware commissioning still requires application/connector work as documented.

### HP5334B

**EVIDENCE9.4:** Separate own-model spec/cases use the shared A/B manual only where verified equivalent. Page3-80 explicitly excludes A MR/MS on B, and page3-89 Option700 CIIL startup is called out. The implementation supports the common proven subset and rejects unknown/CIIL commissioning instead of guessing language or sending GAL. Complete56case B provenance remains separate from A.

**API9.4:** Distinct `keysight.hp5334b` passive identity and explicit model enum preserve model binding. Identical typed Counter subset/units/channel semantics are useful and justified; optionalC, MR/MS and unmodeled modes are not advertised. Open validates endpoint and required operations before connector access; native mode requires a native connector. Shared SDK contains no B exception.

**PROTOCOL9.4:** Independently checked B's actual common engine paths against its spec: same justified FN/HS0/MD1/WA1/GA/RE grammar, strict unit-aware ASCII parser, explicit IN state update and no inferred ID/language changes. Unknown language rejects at construction. Sync/native failure and abandonment leave quarantine, no recovery/reset/TE/GAL cleanup and resource-only exactly-once owned close. Connector bounds/deadlines remain explicit.

**PARITY9.5:** Its56 B-specific cases run through all four frontends in Rust/C#, source and packed, Linux/Windows/net8/net10; B is statically rooted in AOT consumers. Source comparison confirms same error, cache, units and framing behavior across languages without dynamic discovery.

**VALIDATION9.4:** B identity/language/Option700 boundaries, period-on-B rejection and distinct B traffic/parser cases coexist with per-model cancellation/deadline/ownership loops. Golden success/negative cases and procedural assertions execute in the audited artifacts and directly re-executed Linux consumers. No A-only case stands in for B.

**DELIVERY9.3:** Separately named factory ships in the independent Keysight archives, with its own spec and explicit C# B example, no automatic language switch and a coordinated future-version requirement. Same actual package/native provenance applies because both counter model implementations are in the verified archives and independently selected in consumers.

### HP3497A

**EVIDENCE9.3:**03497-99000 primary mirror plus publisher corroboration have verified hashes/page mapping. Manual pages support Option001, five20-address slots, SR analog-or-empty ambiguity, SO1 pacing, VC0 and AI-trigger semantics. Model spec/provenance explicitly distinguish physical in-range literals from synthetic±120V parser-policy boundaries. Scanned/OCR source and the lack of bench evidence keep this below the unusually-comprehensive anchor.

**API9.4:** Neutral `DataAcquisition.MeasureVoltageDc(channel,range)` uses positive public channel IDs, optional exact voltage range and Volt result. Immutable exact endpoint routes validate explicit001/normal routing, module/compensation/slot/local inventory, duplicate IDs/native addresses and conflicting slot modules. Option020 reference19 is excluded; native address is derived so inconsistent redundant coordinates are unrepresentable. Required operations and unbound endpoints fail before connector open in every path; no topology probe or scalar-DMM misrepresentation.

**PROTOCOL9.3:** Actual source emits one exact `VT4AE0VC0VS0VF1VN1VW0SO1VRnAIc` query with the mapped channel and exact range/null policy, no GET/increment or question mark. Parser accepts only eleven-byte finite scalar grammar, exact positive overflow and absolute120V policy. Common engine validates before IO, fails immediately, and quarantines uncertain IO/parse/cancellation without retry or false recovery. Fresh boundary, exclusive control and normal routing must be commissioned externally; software cannot prove them from one response, and documentation does not claim otherwise.

**PARITY9.5:** Rust-owned map and C# frozen dictionaries implement the same admitted inventories, derivation, ranges, parser/status/error and close policy.53 distinct wire cases and DAQ-specific procedural loops execute in the four-front-end source/packed NativeAOT matrix and the downloaded binaries I executed. Native-only checks exclude blocking shims.

**VALIDATION9.4:**53 golden cases plus isolated inventory tests distinguish duplicate-public from duplicate-native, independently valid conflicting modules,020compensation/reference, slot/local boundaries,4096/4097routes and caller mutation. Pending reads/deadlines, active cancellation, owned/borrowed lifetime and follow-up quarantine are exercised. Actual host byte-session tests cover incomplete framed replies and allocation/framing rejection separately from the opaque vendor host.

**DELIVERY9.3:** Factory/spec/immutable inventory instructions and exact prerequisites are in the external package README/spec. Actual crate includes the implementation and core archives do not; checksummed source/packed consumers execute it. Limited scalar scope, unknown other output state, resource-only close and public-alpha incompatibility are accurately documented. A fuller copy/paste DAQ registration example would be a useful nonblocking usability improvement.

### HP8920A

**EVIDENCE9.4:** Own08920-90220 RevB/A.18.00+ manual and5968-5385E datasheet have verified hashes and page references. Direct PDF inspection confirms A055combined007/008/016 source rows, source/receiver frequency distinction, own-model RF syntax and power-domain provenance. Continuous power uses frozen mathematically derived binary64 constants rather than rounded48dBm or reverse-power rows. Accepted/deferred operations and simulation claims are explicit.

**API9.4:** Neutral typed source configure/output/state and receiver frequency/power methods do not introduce vendor ports in core. Exact immutable route configuration binds A-specific firmware/options/source/receiver. Selected ANT cannot inherit RFInOut power from another route, unbound routes reject, and capability/mode checks precede acquisition. Configure leaves output disabled; enabling requires successful configuration, and receiver success invalidates that readiness.

**PROTOCOL9.3:** Actual RF engine verifies OFF before CONFIG screens/writes, verifies Manual/offset-off/ATT/port/UNIT/readback, preserves modulation and uses integerHz/tenth-dB formatting. Each receiver call normalizes units/state/averaging/reference, FULL/SING and fresh TRIG:IMM; power does not silently query carrier. Exact decimal tolerance comparisons distinguish valid wrong settings(Device) from malformed/implausible(Parse). Native failures/abandonment quarantine before further IO. Physical settling, suitable carrier, PTT inhibition and external bus recovery remain explicitly unproved commissioning conditions.

**PARITY9.5:** A-specific envelopes/allowed options and transaction state machine match in Rust/C#;152 distinct A cases execute through all four frontends and source/packed native consumers on Linux/Windows/net8/net10. Actual producer/consumer archives, binary formats and phase logs establish AOT/static registration and unchanged neutral SDK compatibility.

**VALIDATION9.4:**152cases cover source/receiver ports, A combined-option endpoints, digit/resolution and exact tolerance boundaries, grammar/known-wrong-unit distinctions, repeated freshness and configure→receive→enable invalidation. Procedural loops cover A firmware admission, caller freeze, ANT route pre-open checks, NaN/±Infinity pre-IO usable follow-up, pending writes/reads, cancellation and resource lifetime. Regression/generation checks and real executed consumers complement synthetic fixtures.

**DELIVERY9.3:** Independent package source and release-form archives carry A's implementation; static factory registration, option/commissioning instructions and finite/bounded host setup are usable and explicit. New source is clearly absent from existing publicalpha.1. No general shutdown/hardware/release claim. A complete standalone RF composition example remains a nonblocking documentation opportunity.

### HP8920B

**EVIDENCE9.3:** Own08920-90222 RevD/B.06.00+ and5968-1376E have verified identities/page references. I extracted actual datasheet page9: explicit055excludes006/007 controls availability despite conflicting numeric rows. Current spec/provenance and both backends implement that restriction. B's3.5dBm electronic Duplex maximum, options006/007/016 and receiver limits remain distinct from A/C. The source conflict is retained rather than silently erased, warranting9.3 rather than a higher evidence score.

**API9.4:** Distinct B model/factory/firmware family with frozen exact routes and typed neutral methods. Invalid055+006/007 configuration fails at construction without acquisition, even though valid electronic006/007 remain supported. ANT/route isolation and native-mode truthfulness use the selected route across all frontend opens. No A008support leaks into B.

**PROTOCOL9.3:** Own-model command diagrams support the shared RF grammar; actual B configuration chooses correct port/option envelopes and frozen power limits. OFF/CONFIG verification precedes further changes; source readiness, explicit enable, repeated receiver trigger, readback error classes and strict bounded tokens behave as specified in both implementations. Failed verification stops without compensating IO and quarantines. External commissioning and actual RF stability remain documented limits.

**PARITY9.5:**148 valid B wire cases plus explicit construction rejection tests run in Rust and C# across source/packed four-front-end/platform/framework matrix. Identical allowed combinations, firmware grammar and Duplex/power constants are visible in actual source, not only matching generated fixtures. Static AOT artifacts include B's factory.

**VALIDATION9.4:**148cases test legal B combinations and B-specific wire/parser bounds; procedural pre-open tests cover the unavailable mechanical combinations, which ordinary last-call CatalogCases cannot represent. Genuine pending-write/read deadlines, cancellation, ownership and no-detached-access assertions execute for B separately. Independent provenance/case count joins and native execution records were checked.

**DELIVERY9.3:** Package/documentation expose exact B option exclusions and own-model requirements, independent distribution and checked SDK pins. The actual Windows/net8/net10 and Linux generated/packed artifacts passed rather than extrapolating a Linux-only source run. Current public version and resource-only close are accurately scoped; standalone RF example is an optional improvement.

### HP8921A

**EVIDENCE9.4:** Own08921-90031 December1998 RevA/A.15.00+ manual and5968-5820E datasheet are hash-bound. Direct actual PDF reads confirm page43 numeric writes preserve OFF, page61 selected-device-clear-before-abort and page81 expanded RFG W/DBM/V/DBUV versus TX-power W/DBM domains. OlderA.12manual remains historical corroboration, not a fabricated qualification. Own-model electronic10MHz receiver,250kHz source055 and30MHz power-carrier distinctions are explicit.

**API9.4:** Separate8921A factory/model exact identity and A.15minimum, Standard-only input power with electronic/mechanical055, immutable source/receiver ports and explicit commissioning. No A/B power options are accepted. Neutral capabilities and typed units remain coherent; selected ANT power and unbound routes reject before acquisition, native support is conditional.

**PROTOCOL9.3:** Shared engine is justified by actual own-model syntax and qualified manual: OFF-verified configure, exact readback/UNIT/tokens, explicit output enable, fresh FULL/SING receiver trigger and source-readiness invalidation. Known V/DBUV source units yield Device while unknown TX-power units yield Parse; finite physical envelopes and exact tolerances are enforced. The declared PTT/clean-boundary/carrier prerequisites and resource-only close avoid claiming physical protection or complete shutdown.

**PARITY9.5:**128 C-specific cases and each model procedural loop execute identically in Rust/C#, sync/native/two worker frontends, generated-source/packed and all four CI platform/framework cells. Downloaded NativeAOT artifacts statically root the C factory and exercised it locally on Linux; Windows execution is in hash-audited hosted records.

**VALIDATION9.4:**128 independent cases explicitly cover C firmware/domain distinctions, source/receiver limits, Standard-only options, repeated triggers and state lifecycle. Procedural loops independently cover cancellation before/during native IO, pending reads/writes, precise quarantine and ownership; mutation/route/nonfinite rejection proofs supplement scripts. Spec/provenance/case joins and full1424case regression execution passed.

**DELIVERY9.3:** External archives include8921A actual backend, no vendor dependencies were added to core, and README/spec explain A.15qualification and Standard-only configuration. Actual packed/source native consumers and checksum joins support delivery; existing publicalpha.1 is not misrepresented. Optional standalone RF composition example would improve first use without altering support claims.

## Shared boundary checks, confidence and limitations

Main additions are neutral DAQ/RF contracts, generated typed methods/operation metadata/OpenTAP views and standard byte-session tooling. Source search found no new HP command grammar or model implementation in `crates`/`dotnet/src`; the actual SDK archive inventories have no Keysight dependency. Standard host plain-text mode preserves payload whitespace, rejects binary wrappers/alternate terminators/extra buffered frames, bounds response allocation, disables retries and tests actual pending incomplete response deadlines. The allocating VISA connector rejects borrowed/invalid ownership before creating a new session; separately provided borrowed connectors remain available through the public abstraction.

The Rust fixture consumer alone uses opt0 and a bounded32MiB thread. I read the exact manifest/main change and actual compiler commands: all five named driver/SDK crates compile at normal opt3; no assertions/frontends/cases are omitted; `join` propagates the exact Result and resumes panics. Generic downstream wrapper monomorphizations in that consumer can use opt0, as explicitly documented. Normal-release SDK typed consumers and a supplemental original full O3 Rust source run remain evidence, without overstating the entire fixture binary as optimized. Hosted final Windows and Linux logs establish the amended harness actually executes.

Confidence is **high for source/package/software conformance in the declared scope**; lower for physical instrument behavior because no real device was exercised. The review inspected representative manual-backed transcripts and all production engines, and rechecked complete provenance/generation/artifact joins; it does not claim a fresh manual derivation of every one of1424fixtures. Windows was verified through actual hosted records and binaries, not locally rerun. Connector correctness, exclusive access, clean initial boundary, option/firmware truth and physical RF/DVM conditions remain application responsibilities. Software bounds are not input protection. The OpenTAP host is separately tested; driver NativeAOT proof does not establish OpenTAP itself is AOT-compatible. Public0.2.0-alpha.1 lacks these additions; future coordinated versions are required. These explicit scope limitations do not leave a required rubric category UNVERIFIED.

## Required changes, optional improvements and amendments

**Mandatory defects:** none demonstrated. **Required changes:** none. **Unverified required categories:** none. **Rubric amendments:** none.

Prioritized nonblocking follow-ups:

1. DELIVERY, `README.md`: add full small DAQ and RF Rust/C# registration examples showing immutable bindings and plain-text deadline/bound settings together. Existing APIs, specs and executable fixtures already establish correct use; this would reduce initial composition effort.
2. EVIDENCE/VALIDATION, future bench records: when equipment is available and separately authorized, retain exact model/firmware/options/host setup and real traces for each supported subset, especially RF OFF/readback/freshness and scanned-manual native counter/DAQ records. Do not rename the present software evidence as hardware evidence.

No source edits were made during this review. All36 scores are fresh on the same current frozen source/evidence and meet the exact threshold independently.
