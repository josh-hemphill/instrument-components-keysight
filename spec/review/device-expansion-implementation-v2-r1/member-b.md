# Council B — full implementation review, v2 round1

Member **B**, actual **gpt-6.1-sol / high**, stable framing **consider boundary guarantees and evidence**. Rubric **v2**, stage **IMPLEMENTATION**, round **implementation-v2-r1**. Target **>=9.2/10** per category/item, one decimal. Frozen manifest SHA256 **`43d87036abe94ce53c2d98db9e8c6ae4a74c149cb2f65c01f9ba973766794dd6`**.

**Individual ballot: all54 item/category votes PASS.** No demonstrated mandatory defect, required change or UNVERIFIED required evidence remains in the current declared software subset. All three Council ballots are still required for an aggregate result. No SPEC/previous numeric grade was carried forward.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34401A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 33120A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 53132A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 3585B | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 34980A | 9.3 PASS | 9.5 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 34937A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 34922A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| 34950A | 9.4 PASS | 9.5 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |
| TDS1002 | 9.4 PASS | 9.4 PASS | 9.5 PASS | 9.5 PASS | 9.5 PASS | 9.4 PASS |

## Frozen identity and independent checks

All **1316** bound files matched hash and byte count. Current code identities are core`1243881c6b91c46ff40cf0cb3cfd4598b8352409`, Keysight`ac1d71a9fba87be2a1cd2bcc16c6b1888c0f0f13`, Tektronix`f71ca340406370cc2467f78b604cc435752cadd8`. I matched701 copied tracked source/tool/test/doc inputs to these exact Git revisions; no differences. Current vendor backends were inspected against the approved complete v2 specifications/oracles and own-model primary evidence.

I independently opened **all20 original retained ZIPs**: four standaloneSDK artifacts, each vendor’s fourSDK artifacts and each vendor’s fourdriver artifacts. Verification did not depend on facilitator audit conclusions: ZIP hashes matched recorded GitHub digests; every phase log hash/exit matched; intentional negative gates had nonzero exits and bothIL2026/IL3050 errors; successful publish logs were warning-clean; native binaries matched hashes and x64ELF/PE structure, with noCLR directory onPE; Linux dependency logs excluded shared.NET hosting. SDK crates/driver crates/NuGet manifests and payload boundaries were inspected;168 SDK source members and every packaged vendor Rust source/generated member matched frozen input (normalizing platform line endings). Each vendor cell’s exact12-entry feed map joined its corresponding14-entry producer package map after excluding only the two synthetic providers.

The source/packed logs actually execute examples, procedures and **1859 Agilent /452 Tek cases**, including C# fourfrontends and Rustdefault/async, on Linux/Windowsx64 ×net8/net10. SDK native logs execute eleven added typedneutral operations. I inspected actual procedural host suspension/cancellation, active-access counters, timers, queued transactions, admission/immutability/grid/error-followup code and runner wiring. As an extra check I extracted the Linuxnet10 **packed** NativeAOT binary from each vendor ZIP into temporary storage and executed both locally: exit0, examples/procedures and1859×4/452×4 corpus checks all passed. This is additional verification of audited binaries, not a replacement for retained Windows/platform evidence.

| Family | Native cells | SDK phases per cell | Driver phases per cell | Corpus per frontend |
|---|---|---|---|---:|
| Standalone SDK | Linux/Windows ×net8/net10 | 47 Linux /45 Windows | — | Eleven typed additions |
| Keysight | Linux/Windows ×net8/net10 | 47 Linux /45 Windows | 21 Linux /19 Windows | 1859 |
| Tektronix | Linux/Windows ×net8/net10 | 47 Linux /45 Windows | 21 Linux /19 Windows | 452 |

Both generated/source and packed driver native images are present/executed in every driver cell. Core CI job records, current .NET managed regression logs (30 OpenTAP /46 runtime tests) and immutable pins were checked separately; they are not presented as OpenTAP-hostAOT proof. New independent504Agilent/334Tek case payloads remain unchanged, alongside1355/118 unaffected cases respectively.

## Earlier source findings

- **B-SOURCE-01 resolved:** the two newSCPI Rust validators and the3585-localHP-IB validator explicitly requireASCII schemes before trimming. Paired Rust/C# NBSP/EMSPACE constructor rejection and ASCII case/trim tests are present, assert no acquisition and execute in the native proof. Old unrelatedHP admission behavior was preserved.
- **B-TDS-SOURCE-01 resolved:** both C# quantity sites use the actualSDK`CatalogReply.Numeric`. Current source and packed native execution cover timebase/Vpp/frequency returns. Earlier wrongScalar snapshots and failing logs remain history, not current-source defects.

## Per-item findings

### 34401A

- **EVIDENCE 9.4 PASS:** Own34401-90004 manual/spec/provenance and69 independent cases are bound. Actual qualified_scpi Rust/C# replacement implements only manual-derived seven scalars, documented units/ranges/positive overload and GPIB commissioning; inherited USB/temperature/split-phase claims are removed.
- **API 9.4 PASS:** Actual explicit model factory exposes exactly seven DMM operations; validated copied endpoint bindings, matching commissioning and selected requirements gate acquisition. Passive alias selection and absence of native connector are exercised; old generic profile no longer coexists.
- **PROTOCOL 9.4 PASS:** Both implementations emit one MEAS query with explicitDEF, parse exact15-character scientific bodies, distinguish exact positive overload, reject giant/malformed responses and quarantine uncertainty. Finite connector deadline/body/framing prerequisites and resource-only close are documented and verified by host fixtures.
- **PARITY 9.5 PASS:** All69 model cases are part of1859 unchanged independent cases executed in both languages and all frontends, generated-source and packed, in four Linux/Windowsx64 net8/net10 cells. Actual native binaries/log hashes and package/feed joins were independently inspected; Linuxnet10 packed binary also passed locally.
- **VALIDATION 9.5 PASS:** Actual QualifiedProof/qualified_proof test every nonfinite range with healthy follow-up, all independent error quarantine outcomes, passive/admission/ASCII/binding bounds, copied bindings and owned/borrowed cleanup. Pending query native cancellation/deadline and whole-operation serialization run in source and packed proofs.
- **DELIVERY 9.4 PASS:** External Keysight source/crate/NuGet contains qualified factory, with TestKit development-only and core implementations separate. Compiled examples and README explain prerequisites/support limits, old generic retirement and no publicalpha.1 additions. Immutable1243881 tool/SDK feed and actual source/packed proof are joined.

### 33120A

- **EVIDENCE 9.4 PASS:** Own33120-90006 pages establish always-live output,50ohm accounting,100microhertz floor, coupled amplitude/offset envelope and real9.9E37HighZ. Current code and67 independent cases preserve these facts without output-disable or physical-calibration claims.
- **API 9.4 PASS:** Four operations and explicit continuous-sine/50ohm/VPP/output-change commissioning are implemented in both languages. Frozen endpoints and passive selection are tested. No OutputEnable/shutdown is advertised; write-only setters and read-setting semantics are documented.
- **PROTOCOL 9.4 PASS:** Exact guards precede action; HighZ numeric exception precedes generic magnitudeParse. Both original and rounded E11 pairs are checked. Post-query conflict produces InvalidArgument with no write and quarantines, while structural rejection preserves health. Current wrapper never emits retry/reset/output cleanup.
- **PARITY 9.5 PASS:** The67-case subset, actual commissioned-factory selectors, C# four frontends and Rust default/async run in every source/packed native platform cell. Package sources match frozen Rust implementation; native images/logs and12-entry feed joins were checked independently and local packed execution passed.
- **VALIDATION 9.5 PASS:** GeneratorFailurePositions explicitly injects Transport/Timeout at every amplitude/offset guard/counterpart/final-write position beyond the golden frequency failures. Nonfinite healthy follow-up, rounded conflicts, real pending query/write cancellation/deadline, owned/borrowed lifetime and transaction ordering execute in native proofs.
- **DELIVERY 9.4 PASS:** External package/documents require truthful load and authorization for live output changes; compiled typed example exercises actual factory. Old public package exclusion, exact current SDK workflow/tool pin and separate future coordinated version are explicit; packed archives contain no TestKit/runtime oracle.

### 53132A

- **EVIDENCE 9.4 PASS:** Exact53132A applicability of53131-90044, CONFIG/arming/ASCII/freshREAD and gate resolution is preserved in code/spec and50 independent cases. No53131-only aperture, channel3 or totalize behavior is imported.
- **API 9.4 PASS:** Current Core has per-open channel1/gate100ms intent; validated setters commit locally with no IO. Channels1/2 and frequency/period units are exact. Tests verify invalid setters preserve prior intent and a second handle receives independent defaults.
- **PROTOCOL 9.4 PASS:** Actual transactions reapply CONFIG before timed arming/gate/ASCII/READ. Canonical binary64 integer-millisecond equality and three-decimal wire format match; NR3 exponent/significant-digit, invalid9.91E37 and nonpositive-domain errors quarantine without bus cleanup.
- **PARITY 9.5 PASS:** Both languages execute all50 cases plus actual counter factory procedures in source/packed Windows/Linuxx64 net8/net10 native cells. E11/state/error semantics are equivalent; genuine native and worker paths are gated honestly and actual package/native identities inspected.
- **VALIDATION 9.5 PASS:** FrozenInventoryAndCounterIntent plus corpus checks repeated fresh frequency/period, grid boundaries, state preservation/default isolation and no staleFETCh. Pending native read/write, independent timers, abandonment/cancellation, ownership and whole-operation queuing execute; every error has quarantine/healthy follow-up.
- **DELIVERY 9.4 PASS:** External factory and compiled typed Counter example use version-pinned SDK packages without proprietary core rules. Own-model prerequisites/deferred channel3/totalize and missing-carrier timeout are documented; exact checksum-joined package proof and future-version boundary are present.

### 3585B

- **EVIDENCE 9.4 PASS:** Bound scanned03585-90017 source, OCR/render identities and native fixed-field diagrams support actualhp3585 code and123 independent synthetic cases. Serial applicability, R12/50ohm/liveA/dBm commissioning and software1Hz floor versus hardware0Hz/fractional capability are explicit.
- **API 9.4 PASS:** Eight native SpectrumAnalyzer operations are exposed through exacthpib/model bindings, not genericSCPI. Copied bounded endpoint admission and passive aliases are tested. Unsupported sweep/trace/reset/shutdown are absent; peak/read semantics are documented separately.
- **PROTOCOL 9.4 PASS:** Actual CL/SP andCL/D2,T6 match approved native protocol. Fixed24-byte record validates both fields even for one requested axis and preserves spaces; E-0 is required for dBm. Stored-traceSP is not a fresh global sweep andT6 waits only for selected marker validity. Uncertainty quarantines with resource-only close.
- **PARITY 9.5 PASS:** Rust/C# exact integer/tenths formatting, parser/error contracts and all123 cases execute in source/packed four-cell native matrix, including both worker paths. ASCII scheme fix is present at constructor/selection and paired rejection procedures execute; binaries/log/package sources verified.
- **VALIDATION 9.5 PASS:** Actual analyzer constructor bound/alias/nonfinite/error-health tests and pendingCLwrite/D2query cancellation/deadlines run. Cases include malformed discarded fields, widths/spaces/exponent/ranges; serialized multi-message marker calls are exercised. Local Linuxnet10 packed execution passed together with all other Agilent items.
- **DELIVERY 9.4 PASS:** Native vendor implementation/manual-derived corpus stays in Keysight package; compiled example and README disclose stored-peak/selected-point behavior and serial/mode prerequisites. Synthetic traffic and absent physical proof are explicit; compatible immutable feed/package proof is actual, without public-release claims.

### 34980A

- **EVIDENCE 9.3 PASS:** Own mainframe/programmer/module sources and independent fixture define addressedGPIB, slot/native addressing and scan-wide consequences. Current code/spec/README honestly scope internalDMM/scan/LAN/USB admission and require lifetime exclusion of autonomous actions.
- **API 9.5 PASS:** One parent factory/handle/lease owns all module views. Current validated frozen maps enforce slot/local bounds, namespace uniqueness, physical uniqueness, same-slot/cross-namespace types and selected-endpoint capabilities before acquisition. Core contains only five neutral operations; all module rules remain external.
- **PROTOCOL 9.4 PASS:** Actual MessageDevice health/lifetime is parent-wide, so guard failures quarantine relay/digital siblings. Native queuing test pauses digital transaction while relay view waits. Ownedclose is once, borrowed remains open, and no device/globalOpenAll/reset is emitted; finite connector contract is explicit.
- **PARITY 9.5 PASS:** New neutral schema/catalog/generated typed APIs and OpenTAP managed views are synchronized and exercised. SDK native proof executes eleven typed additions including bytezero and old-positive-count rejection. Source/packed vendor proofs use exact1243881 compatible feeds in all four native cells with no core/vendor leakage.
- **VALIDATION 9.5 PASS:** Inventory checks all supported locals at every slot, isolated invalid/duplicate/conflicting maps, empty inventory and deep mutation; SelectedRouteAdmission proves relay/input/output capability isolation before sync/native/worker opens. Pending sibling serialization/quarantine/ownership and1859-case corpus execute in both languages.
- **DELIVERY 9.4 PASS:** Actual external parent package has no fictitious module binaries/sessions; compiled shared-parent examples and prerequisites are delivered. Crate/NuGet dependencies and payloads,12-of14 SDK checksum joins and immutable producer pins were inspected. No TestKit/oracle/vendor source enters core production dependencies.

### 34937A

- **EVIDENCE 9.4 PASS:** Actual topology agrees with own guide:1–28FormC and29–32FormA, latching/no analog bus, with NC meaning of Open explicit.32 independent cases and documentation avoid continuity, fail-safe or power-cycle isolation claims.
- **API 9.4 PASS:** Current module is a switch capability of one34980A parent, with positive logical paths mapped to validated slot/local addresses. Three single-channel operations avoid fake matrix pairs/independent sessions/OpenAll. Exact descriptor/selected requirements and sparse mapping are exercised.
- **PROTOCOL 9.4 PASS:** Actual writes/query are only selectedROUT:CLOS/OPEN/CLOS?, with strict0/1 parser. Failures quarantine parent without cleanup/reset; close does not change latching states. Concurrent-view parent lock and shared lifetime are directly tested.
- **PARITY 9.5 PASS:** Both implementations map the same frozen module inventory and execute32 cases in all modes within source/packed four-platform vendor proofs. New neutral operations are rooted in actual SDK typed/AOT execution; actual release-form packages/native images inspected.
- **VALIDATION 9.5 PASS:** All FormC/FormA boundary operations, states, parser/IO failures and healthy/quarantined follow-ups run. Inventory tests cover every local/slot and type conflicts; module-specific pending relay query/write, cancellation/deadlines, oneparentowned/borrowedclose and transaction queuing execute.
- **DELIVERY 9.4 PASS:** External shared-parent examples and README explain FormC NC connection and partial inventory, no automatic isolation/shutdown. Production packages exclude manual/oracle/TestKit payloads; compatible new SDK feed and actual package/native proof are retained without physical-contact or release claims.

### 34922A

- **EVIDENCE 9.4 PASS:** Own printed34–35 evidence, actual native mapping and33 independent cases retain two35-channel latching banks, scan-only4wire pairing and nonexclusive direct routing. No neighboring reed/FET topology or power-off behavior is substituted.
- **API 9.4 PASS:** Current parent binding admits direct local1–70 positive logical paths only; analog-bus/DMM/scan/four-wire/OpenAll remain absent. Sparse IDs and per-endpoint selected capability checks are actual, with no hidden sibling expansion or separate connector.
- **PROTOCOL 9.4 PASS:** ROUT:CLOS/OPEN/CLOS? messages and0/1 decoding match independent cases. Two same-bank close writes occur without an implicit open. Uncertain failure quarantines sharedparent and close emits no hardware cleanup; lifetime commissioning excludes scans/monitor/autonomous actions.
- **PARITY 9.5 PASS:** Identical generated neutral operations and external Rust/C# mapping/parser run in every source/packed native cell. SDK/package sources and full corpus/phase/native checks independently match current immutable revisions, including actual worker/native distinctions.
- **VALIDATION 9.5 PASS:** 33 cases include1/35/36/70, nonexclusive samebank closure and malformed/typed IO failures. Procedural inventory tests cover all70locals×8slots and isolated conflicts; actual pending muxquery/write, sibling health/ownership/concurrency and immutable route/capability tests execute.
- **DELIVERY 9.4 PASS:** External factory, typed shared-parent example and README document external signal commoning, latching retention and deferred acquisition. Newneutralcore/package separation and exact producer/feed/artifact joins are proven; simulator/native results remain separate from physical bus/contact evidence.

### 34950A

- **EVIDENCE 9.4 PASS:** Current code/spec and123 independently extracted cases use own-model numeric1/2/4 width tokens, HIMP versusOFF, bank-first memory addressing, SENSE input versus latch and SOURCE implicit-direction behavior; fixed2.5V/Active3.3V is truthful commissioned policy.
- **API 9.5 PASS:** Neutral byteu32/zero-validCount and positive logicalports are generated without weakening existing positive counts. Frozen per-port input/output and selected-endpoint capability checks occur beforeIO/acquisition as appropriate. Exact0–255 request checks and copied direction maps are tested.
- **PROTOCOL 9.4 PASS:** Actual read/write prefix checks all four bank widths, selected direction/polarity, bank-first HIMP/memory and electrical guards in order; failures stop before action. SENSE explicitDEC and SOURCE decimalbyte keep admitted width/direction. Strict sampled-byte parser/domain errors quarantine all siblings; no implicitconfiguration/cleanup.
- **PARITY 9.5 PASS:** New typed SDK/OpenTAP byte operations preserve zero and old count semantics. Both languages execute123 cases/all frontends and static sharedmodule views through actual source/packed native cells. Package feed/source/native binaries and no-reflection boundaries inspected.
- **VALIDATION 9.5 PASS:** Boundary0/255/256, direction/unknownport, allguardfailure/eachtrafficposition, strict decimal bodies and quarantine follow-ups run. Constructor tests cover all8byteports×8slots and both directions; mutable direction replacement, all-bank grouping, selectedcapability isolation, pendingwrite/read and relay-sibling serialization are executed.
- **DELIVERY 9.4 PASS:** External module policy/docs/compiled examples disclose fixed electrical state and lifetime autonomous-action exclusion, excluding buffer/wide/direction-control APIs. Production core is neutral; independent packages contain no runtimefixtures/TestKit and all matching newSDK/archive identities are retained.

### TDS1002

- **EVIDENCE 9.4 PASS:** Exact corrected non-BTDS1002 manual and334-case provenance are preserved; CM/CMA/CMAXGPIB andMEM/USB exclusion,1/10/100/1000 probes, NR1/NR3/quoted units andDESE/OPC/warnings are own-model backed. All118 older cases remain unchanged rather than substituting MDO behavior.
- **API 9.4 PASS:** Actual immutable endpoint/module/probe factory exposes precisely ten operations and exactTDS1002/TDS 1002 aliases. Constructor/admission tests cover module/probe/enum/count/ASCII/duplicates/unbound/nativeabsence and endpoint-specific probes; complete31timebase and4×2×11vertical grids are exercised.
- **PROTOCOL 9.5 PASS:** Current code performs disclosedHEAD/VERB and pre/postESR withDESE255. FreshcaptureSTOP/SAMPLE/SEQ/RUN/OPC/state/count/source/type/quotedunits/value withholds reply until finalESR0. Underflow/sentinel/domain policies match; no hidden STOP/retry/mode restoration follows cancellation. Both C# returns now use correctNumeric SDK type.
- **PARITY 9.5 PASS:** Corrected C# source and packed native binaries actually execute all452cases×4frontends plus procedures/examples across Linux/Windowsx64 net8/net10. Rust default/async counterparts also run; archive phase/digest/feed/native/source joins independently verified and extracted Linuxnet10 packed binary passed locally.
- **VALIDATION 9.5 PASS:** All334 own-model cases and118 baseline cases execute unchanged. Actual procedural code covers healthy invalid follow-up, everyerrorquarantine, allprobe/timebase neighbors, fr-FR, immutable perendpoint probes, pendingHEAD/RUN/OPC/VALUE cancellation/deadlines, ownercleanup and capture/Stop noninterleaving. LateESR failures do not return finitevalue success.
- **DELIVERY 9.4 PASS:** External Tek implementation retains existingMDO profile/corpus and adds no model exception tocore. Compiled static examples/README disclose statusconsumption, stoppedSAMPLE/SEQ state, module/probe/trigger/deadline prerequisites and absent publicalpha.1. Exact1243881 newproducer/tool/package/native proof and futureversion boundary inspected.

## Required changes, confidence and limitations

**Mandatory defects:** none demonstrated. **Required changes:** none. **Rubric amendments:** none. No source edits, hardware operations, publication, merge or release were performed by this review.

Confidence is high in the inspected artifact/source identity, deterministic protocol and parser behavior, independently joined package/native evidence and actual executions, within these limits:

- Manual-derived software subsets only. No live device, installed option/module/probe truth, electrical protection, continuity/isolation, calibration or physical performance was validated.
- Native and pending-host evidence uses TestKit/scripted hosts. It proves software lifetime/serialization/parser behavior, not unspecified third-party connector quality; real connectors must enforce bounded allocation, framing and finite deadlines.
- Hosted Windows/Linux source/packed NativeAOT is independently inspected retained execution evidence; only the two Linuxnet10 packed binaries were additionally re-executed locally by this reviewer.
- Packages use explicitly audited development feeds with0.2.0-alpha.1 development labels. Existing public0.2.0-alpha.1 does not contain these additions; coordinated new public versions/release gates remain future authorized work.
- Keysight fixture-only Rust consumer uses documentedopt-level0/32MiB stack; production SDK/driver dependencies retain ordinary release settings. The result is not a claim that the large fixture harness uses default optimized stack behavior.
- Core OpenTAP managed regression evidence does not establish OpenTAP-host NativeAOT compatibility.
- All54 implementation cells are newly assessed under v2 on this immutable packet; no SPEC or prior numeric grades were carried forward and no peer current implementation ballots were read.
