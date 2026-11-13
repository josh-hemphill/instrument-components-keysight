# Council A — complete implementation review under v2

Member **A**; actual model **gpt-6.1-sol**, reasoning **high**. Stable framing: consider a new user's interpretation. Rubric **v2**, stage **IMPLEMENTATION**, round **implementation-v2-r1**. Frozen manifest SHA256 **43d87036abe94ce53c2d98db9e8c6ae4a74c149cb2f65c01f9ba973766794dd6**.

**All54 current item/category implementation votes PASS at9.3–9.5.** No mandatory defect, required change or UNVERIFIED category remains in my assessment of the declared software subsets. This is an independent member ballot; Council acceptance still requires the other complete ballots and aggregation. No SPEC/prior numeric grade was carried forward, and no peer current implementation ballot or score was read.

Exact current code: core **1243881c6b91c46ff40cf0cb3cfd4598b8352409**, Keysight **ac1d71a9fba87be2a1cd2bcc16c6b1888c0f0f13**, Tektronix **f71ca340406370cc2467f78b604cc435752cadd8**. All scores use one decimal and the unchanged **>=9.2/10** comparison; every displayed vote is derived directly from its score.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34401A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 33120A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 53132A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 3585B | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 34980A | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 34937A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 34922A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |
| 34950A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.3 PASS |
| TDS1002 | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.4 PASS |

## Independent evidence inspection and execution

I verified the manifest hash and all **1316** bound-file hashes, read the approved rubric/README, and inspected current copied backends, common transaction engines, neutral schema/generated contracts, static fixtures/registrations, actual procedural tests, compiled examples and delivery documentation against the approved manual/oracle subset. Relevant own-model primary PDFs and scanned3585B provenance remain bound and readable; manual-derived/synthetic evidence is distinguished from hardware throughout. I did not read peer ballots even where earlier SPEC records were included in the manifest.

To verify that current copies were the tested code, I compared **1077 copied tracked files** directly against their exact immutable repository commits:575core,365Keysight and137Tektronix, with no drift. The active corpora have **1859** and **452** unique IDs; all504Agilent and334Tek approved new cases retain their normative logical JSON content. The retired unqualified34401 profile remains outside active production registration; unaffected baseline corpora are preserved.

I independently wrote and ran a read-only ZIP inspection rather than accepting the facilitator's audit output. Across all **20 original ZIPs**—four standalone SDK, eight vendor-run SDK and eight vendor driver-proof artifacts—I verified:

- GitHub artifact ZIP SHA256 digests and **712 phase exit/log-hash joins**, including expected negative gates with actual IL2026 and IL3050 errors.
- **28 actual native binaries**:12SDK and16vendor generated/source-or-packed consumers. I inspected x64ELF/PE headers, PE CLR-directory absence, binary/checksum joins, Linux runtime dependency logs and native publication warnings. Normal publications have no unexpected IL warnings.
- Every vendor cell's **twelve exact consumer/feed package hashes** match the corresponding fourteen-entry SDK producer after excluding its two test-only synthetic providers.
- Actual release-form Cargo/NuGet hashes, member lists and production dependencies. **120 SDK production-package instances** have exactly the allowed neutral dependency boundaries; vendor archives depend only on the intended SDK API/SCPI libraries, contain their own implementation DLL/source and exclude validation/spec payload. Core artifacts contain no external driver implementation.
- **220 packed Rust source-file comparisons** against the current frozen sources, including generated profile data and the new concrete backends. Native execution logs contain compiled examples, actual admission/pending/ownership/probe/module procedures and the1859/452case counts through four frontends in both generated/source and packed phases.

I additionally extracted and independently executed the **exact packed Linux native binaries** from each vendor's net8.0 and net10.0 ZIP. All four exited **0**, running their full corpora and current procedures/examples. Keysight each reported1859independent cases ×four frontends; Tek each reported452 ×four frontends, including fresh capture/status/probe and pending HEAD/RUN/OPC/VALUE proofs. I inspected Windows hosted execution artifacts/logs without claiming a local Windows rerun.

The current SDK hosted record has11successful jobs and each vendor9successful jobs at the recorded commits. I also inspected actual current core tooling/Rust/runtime/OpenTAP regression logs:23tool tests,46.NET runtime tests and30managed OpenTAP tests passed, alongside clippy and Rust regression results. SDK archives execute eleven added typed neutral operations, zero-byte/oversized-byte and retained positive-count behavior. The producer/consumer pin is now the actual compatible immutable1243881revision, not the obsolete first-three-only pin.

## Earlier findings and current disposition

**A-SRC-01 — resolved.** Current qualified_scpi/hp34980 Rust validators reject non-ASCII schemes before trimming; hp3585 uses its own ASCII admission wrapper without changing older HP semantics. Rust/C# paired NBSP/EM-space and normalized ASCII duplicate checks verify no connector acquisition. These procedures execute in the inspected source/packed native consumers and the independently rerun Linux binaries.

**A-TDS-SRC-01 — resolved.** Both C# quantity-return sites now construct the SDK's CatalogReply.Numeric, replacing the nonexistent Scalar type. Current successful builds and actual ReadTimebaseScale/Vpp/frequency cases—including post-value warning failure—exercise the corrected paths. The retained earlier failing snapshot/log remains historical evidence, not the current implementation.

## Evidence for every item/category

### 34401A

- **EVIDENCE 9.4 PASS:** Own-model/hash/page provenance, independent69-case scalar oracle and actual qualified_scpi Rust/C# decoders preserve the documented MEAS/default/range/positive-overload distinctions. Old generic34401 behavior is retired rather than called verified support.
- **API 9.4 PASS:** Current factories expose exactly seven neutral scalar methods with units, finite positive expected-input bounds, frozen exact endpoints and explicit SCPI commissioning. Unsupported temperature/split-phase/reset/USB hints are absent; admission now agrees on ASCII schemes.
- **PROTOCOL 9.4 PASS:** Actual code uses exact MEAS traffic, signed15-character parsing and overload status, pre-IO validation, shared operation lease and fail-fast quarantine. Owned/borrowed close is resource-only; actual pending read deadlines/cancellation complete without detached access.
- **PARITY 9.5 PASS:** Same69 golden cases and actual-factory procedures execute in Rust default/async and C# four frontends, generated/source and packed. All four hosted native cells were inspected; both packed Linux frameworks were independently re-executed.
- **VALIDATION 9.4 PASS:** QualifiedProof/qualified_proof instantiate the actual DMM, test every nonfinite numeric argument with usable follow-up, all golden errors with correct quarantine, ASCII/binding bounds and ownership. Pending proofs reach Active1 before cancellation and Active0 before completion.
- **DELIVERY 9.4 PASS:** Explicit external Keysight registration, compiled composition example, model prerequisites/deferred methods and source/packed package checks are present. Registry-only production dependencies and exact1243881 SDK feed join are verified; old public packages are explicitly excluded.

### 33120A

- **EVIDENCE 9.4 PASS:** Own-model publication/frequency glyph/load/coupling evidence joins67 independent normative cases and source. HighZ9.9e37 is a documented load state, distinct from generic parser magnitude policy and physical overload.
- **API 9.4 PASS:** Four typed generator methods enforce commissioned sine/50Ohm/VPP use. Always-live output, settings-only ReadFrequency and lack of enable/shutdown are accurately documented; structural and queried coupled argument errors are distinct.
- **PROTOCOL 9.4 PASS:** Both implementations issue all eight guards in order, check original and rounded E11 voltage pairs and classify load-specific HighZ before magnitude rejection. Post-query conflicts stop without a write and quarantine; close never disables output.
- **PARITY 9.5 PASS:** Rust/C# share wire formatting, guard/error/value semantics through67 golden cases and all four execution frontends. Verified source and packed NativeAOT Windows/Linux x64 net8/net10 logs/binaries include the actual generator factory.
- **VALIDATION 9.4 PASS:** Additional GeneratorFailurePositions covers every amplitude/offset query/write step and health follow-up; nonfinite/pre-IO versus coupled-post-IO behavior, real pending read/write cancellation/deadlines and whole-transaction ordering execute in native consumers.
- **DELIVERY 9.4 PASS:** External package/source bytes and dependency feeds are audited, compiled example and README disclose live output and physical commissioning, and future coordinated versions are separated from alpha.1/public availability.

### 53132A

- **EVIDENCE 9.4 PASS:** Own-applicable shared primary guide,50 literal cases and implemented CONF/arming/READ/NR3 grammar establish two-channel frequency/period support without borrowing generic53131 operations.
- **API 9.4 PASS:** Typed local channel/gate setters preserve prior intent on rejection, use exact millisecond-grid validation and fresh per-open state. Deferred channel3/ratio/totalize/reset remain unavailable; endpoint/capability admission is passive.
- **PROTOCOL 9.4 PASS:** Actual six-message transaction applies gate after CONF, forces ASCII and fresh READ. Integer-derived gate text, significant-digit/sentinel/positive-result policy and per-operation physical units agree with the approved contract.
- **PARITY 9.5 PASS:** Both language/frontends run50 independent cases and independent-handle/intent proofs in generated/source and packed consumers. Exact native binaries/logs for all four cells and local packed Linux8/10 reruns support parity.
- **VALIDATION 9.4 PASS:** Counter-intent proof rejects invalid setters, retains intended2/.010, opens a separate default1/.100 handle and executes repeated fresh frequency/period. Golden min/max/off-grid/parser cases plus real pending write/query and quarantine proofs execute.
- **DELIVERY 9.4 PASS:** External explicit factory, compiled typed counter example, own-model prerequisites and unsupported capabilities are documented. Matching source/package/producer identities and compatible1243881 feeds are verified without claiming new public release.

### 3585B

- **EVIDENCE 9.4 PASS:** Bound own-model scanned manual/OCR/render provenance joins123 synthetic diagram-derived cases and actual fixed-field decoder. Native ranges, serial applicability, stored peak and selected-marker validity are accurately separated from physical observation.
- **API 9.4 PASS:** Eight useful neutral methods have integer/discrete/tenths bounds and frozen native qualification. Stored-trace MarkerPeak and selected-point MarkerX/Y semantics remain explicit; no SCPI fallback or sweep/trace/reset capability is invented.
- **PROTOCOL 9.4 PASS:** Actual CL/SP or CL/D2,T6 sequences, exact24-character ASCII record, both-field validation, E-0 and frequency bounds match the manual-derived contract. Native framing/deadlines and no-clear quarantine/close behavior are specified and tested.
- **PARITY 9.5 PASS:** Both Rust/C# decoders/formatters consume123 cases through sync/native/two-worker frontends in source and packed native cells. Both packed Linux frameworks were independently executed; Windows proof binaries/logs were structurally/hash inspected.
- **VALIDATION 9.4 PASS:** Golden malformed-discarded-field cases, entry/range negatives and all error follow-ups execute. Actual pending native CL/query and independent sync/native deadline tests enforce cleanup, ownership and no transaction interleaving.
- **DELIVERY 9.4 PASS:** External hp3585 factory, compiled analyzer construction and README/spec document qualification, stored peak/selected refresh and synthetic evidence. Actual package source/dependency boundaries, feeds and future release limitations are retained.

### 34980A

- **EVIDENCE 9.4 PASS:** Mainframe/module primary hashes,195 module cases and exact inventory/address source distinguish real topology from caller commissioning and simulator claims. Partial inventory is not falsely described as complete hardware discovery.
- **API 9.5 PASS:** One parent factory/connector/handle supplies Switch/DigitalIo under one lease and shared health. Actual frozen maps validate physical/type conflicts across namespaces and selected-endpoint capability requirements before acquisition; no independent child session is opened.
- **PROTOCOL 9.4 PASS:** Transaction engine serializes complete guard/action operations; parse/Device/IO/cancellation uncertainty quarantines sibling views. No implicit DMM enable, scan abort, reset, globalOpenAll or direction cleanup occurs. Owned parent closes once, borrowed survives.
- **PARITY 9.5 PASS:** Five new neutral operations and typed/OpenTAP generation use compatible immutable1243881 SDK. Producer native consumers execute eleven added operations, including byte zero; external source/packed module consumers execute all frontends/platform cells.
- **VALIDATION 9.4 PASS:** Inventory/SelectedRouteAdmission exercise every module local and slot, isolated conflicts,4096/4097 bounds and mutated collections. Pending proofs execute digital-to-relay shared-parent noninterleaving/quarantine with genuine suspension and ownership checks.
- **DELIVERY 9.4 PASS:** Core packages contain neutral contracts and exact allowed dependencies only; module implementations stay in Keysight. Static examples, commissioning/deferred features, compiled native consumers and exact package/feed hashes establish independent distribution without publication.

### 34937A

- **EVIDENCE 9.4 PASS:** Own-model FormC/FormA/latching/no-analog-bus evidence joins32 independent cases and actual direct-path implementation. Power-failure and physical continuity are not promoted into unsupported safety guarantees.
- **API 9.4 PASS:** Three neutral logical-path operations and derived addresses preserve FormC common-to-NO semantics; opening selects NC rather than isolation. Sparse mapping and parent ownership are explicit, with no matrix/globalOpenAll behavior.
- **PROTOCOL 9.4 PASS:** Exact selected ROUTe command/query and strict0/1 decoding are implemented identically; failures affect parent health. Close emits no route changes. Actual pending relay reads/writes verify deadline/cancellation and borrowed/owned behavior.
- **PARITY 9.5 PASS:** New neutral methods, Rust/C# backends and32 cases execute through each source/packed frontend across four native cells using the qualified SDK. Actual archives and logs, not status summaries alone, were checked.
- **VALIDATION 9.4 PASS:** Full local1..32/slot1..8 admission, FormC/FormA boundary cases, malformed state/error follow-ups and parent/sibling lifecycle/concurrency procedures execute. Unaffected baseline cases remain in the1859-case corpus.
- **DELIVERY 9.4 PASS:** One external parent distributes the module view; compiled shared-parent example and README preserve NC/latching caveats and defer physical verification. Package source/dependency boundaries and matching SDK feeds were inspected.

### 34922A

- **EVIDENCE 9.4 PASS:** Own-model two35-channel armature banks, direct non-scanning multi-close and DMM-only pairing evidence joins33 cases and actual mapping. Neighboring FET/other mux topology is not inherited.
- **API 9.4 PASS:** Logical1..70 direct paths do not invent a paired endpoint or exclusive closure. Analog-bus/scanning/acquisition remain deferred; same-bank connection consequences and shared parent are documented.
- **PROTOCOL 9.4 PASS:** Both implementations emit only the selected ROUTe traffic, preserve earlier same-bank closures and decode strict Boolean state. Error/cancellation quarantine is parent-wide and no implicit bus/route cleanup occurs.
- **PARITY 9.5 PASS:** Same33 cases and typed neutral Switch operations execute in both languages/frontends and all source/packed native cells. Archived binaries/logs and both local Linux framework executions support implemented parity.
- **VALIDATION 9.4 PASS:** 1/35/36/70 and explicit nonexclusive two-close oracle execute, alongside exhaustive valid locals/slots, otherwise-valid conflict negatives, health follow-ups and genuine pending relay read/write lifecycle checks.
- **DELIVERY 9.4 PASS:** External static shared-parent factory, exact topology/deferred scope docs and compiled example are present. Compatible SDK pins, registry-only production dependencies, package source identity and old-public-version limits are verified.

### 34950A

- **EVIDENCE 9.4 PASS:** Own-module/programmer evidence for bank numbering, grouped width, numeric1/2/4, HIMP, SENSE latch behavior and SOURCE implicit output joins123 independent cases and implemented guards. Electrical truth remains external commissioning.
- **API 9.4 PASS:** Actual neutral byte methods enforce positive logical ports, frozen direction and inclusive0–255 values; zero reads/writes work while existing positive-count constraints stay intact. No proprietary topology/policy enters core.
- **PROTOCOL 9.4 PASS:** All four bank widths, selected direction/polarity, bank-first handshake/memory and electrical guards precede access; no hidden configuration write occurs. Strict sampled-byte parser, fail-fast Device/Parse classes and parent quarantine match the approved subset.
- **PARITY 9.5 PASS:** Producer typed proofs execute byte0,256 rejection and retained positive-count rejection. Both backends/frontends execute123 exact cases and procedures in source/packed Windows/Linux native cells with matching new SDK feeds.
- **VALIDATION 9.4 PASS:** Both banks, non-anchor guards,0/255/256, wrong directions/tokens/electrical state and every traffic failure execute. Deep-freeze mutation, selected-route isolation, actual pending writes/reads and cross-view serialization/quarantine are verified by real procedure code/logs.
- **DELIVERY 9.3 PASS:** Byte-only fixed-policy limitations and external parent composition are documented and packaged correctly; typed SDK proof and full corpus execute the digital calls. The compact composition example shows WriteByte as an explanatory call comment, a minor presentation limitation rather than missing conformance proof.

### TDS1002

- **EVIDENCE 9.4 PASS:** Exact non-B model, module/probe applicability and own-manual acquisition/status/units evidence join334 independent cases, source and retained provenance. No A/B alias, direct USB or B/TPS-only operation is advertised.
- **API 9.4 PASS:** Ten typed operations use frozen per-endpoint module/probe/commissioning records, ASCII schemes and exact identities. Probe grids/settings, explicit acquisition/status side effects and unsupported operations are enforced before acquisition/IO as appropriate.
- **PROTOCOL 9.4 PASS:** Actual HEAD/VERB/DESE/pre-ESR prefix and complete guarded single-sample/OPC/count/source/type/quoted-unit/value/post-ESR transaction withhold quantities until final status success. Underflow/sentinel/domain policies and no-STOP cancellation quarantine are implemented.
- **PARITY 9.5 PASS:** Both C# reply paths now use SDK Numeric; ReadTimebaseScale/Vpp/frequency and all334 cases execute successfully in four source/packed NativeAOT cells. Both exact packed Linux framework binaries were independently re-executed with452 total cases and procedures.
- **VALIDATION 9.4 PASS:** Actual tests cover all four probe factors/two channels/nominal scales and adjacent doubles,31 timebases, invariant fr-FR parsing, endpoint isolation, every golden error health follow-up and pending HEAD/RUN/OPC/VALUE cancellation/deadlines with whole-capture serialization.
- **DELIVERY 9.4 PASS:** Dedicated external Tek factory, preserved118 earlier cases, compiled static application example and explicit immutable1243881 feeds are verified. Package boundaries, source identity/native logs and unreleased/public/hardware/OpenTAP distinctions are candid.

## Required changes, confidence and practical limits

**Required changes: none. Mandatory defects: none demonstrated. Rubric amendments: none.** Confidence is high for this current declared software implementation and its source/package/native conformance evidence. The34950A example's compact commented WriteByte illustration is a minor presentation limit; actual byte operations and zero semantics are executed by the independent corpus and typed SDK proofs, so it is not a required change.

- The reviewed result is manual-derived software conformance for declared subsets, not physical bench/model/module/probe truth, input protection, contact continuity, calibration, timing or measurement performance.
- Only original GitHub development-feed artifacts are qualified. Public0.2.0-alpha.1 lacks these additions; new coordinated versions and a separately authorized release remain necessary. No merge/publication is approved by this ballot.
- Windows binaries were inspected with their executed hosted logs; they were not rerun locally. Both exact packed Linux frameworks for both vendors were independently rerun and exited0.
- Owned/borrowed/worker/native cancellation proofs use scripted and genuinely pending synthetic hosts. Real connector/provider behavior must honor the documented framing/allocation/deadline/cancellation contract; no detached blocking thread is treated as cancelled.
- Keysight fixture-only Rust opt-level0 and32MiB consumer thread are disclosed; production driver/SDK dependencies retain normal release settings. SDK/driver NativeAOT evidence does not qualify the managed OpenTAP host.
- 3585B SP selects stored trace data and T6 refreshes only the selected marker. TDS fresh scalar calls intentionally consume status/replace acquisition mode; cancellation may leave acquisition running and close does not restore hardware state.
- Primary pages and code paths were reviewed for the declared command/parser/topology families, not every page of every manual. Current peer implementation ballots and prior numeric grades were not used.

Only my two current ballot artifacts were written in the packet. Independent inspection helpers/extracted binaries used temporary directories; no production, frozen source or evidence was edited. This ballot makes no hardware, new-public-package, merge, release or OpenTAP-AOT claim.
