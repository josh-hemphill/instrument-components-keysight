# Member C — implementation v2 round1

Independent review by **gpt-6.1-sol / high**, member **C**. Framing: consider model, module and execution distinctions. Approved rubric **v2**, stage **IMPLEMENTATION**, round **implementation-v2-r1**. Frozen manifest SHA256: `43d87036abe94ce53c2d98db9e8c6ae4a74c149cb2f65c01f9ba973766794dd6`.

**Ballot: PASS for all54 item/category cells**, each independently graded against the current implementation at the exact **>=9.2** threshold. No mandatory defect, required change, or rubric amendment found. This is my individual ballot; it does not substitute for the full three-member gate. No prior numerical grades carried forward and no peer current implementation ballots read.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY | Votes |
|---|---:|---:|---:|---:|---:|---:|---|
| 34401A | 9.4 | 9.4 | 9.4 | 9.5 | 9.5 | 9.4 | 6 PASS |
| 33120A | 9.4 | 9.4 | 9.4 | 9.5 | 9.5 | 9.4 | 6 PASS |
| 53132A | 9.4 | 9.4 | 9.4 | 9.5 | 9.5 | 9.4 | 6 PASS |
| 3585B | 9.3 | 9.4 | 9.4 | 9.5 | 9.5 | 9.4 | 6 PASS |
| 34980A | 9.4 | 9.5 | 9.4 | 9.5 | 9.4 | 9.4 | 6 PASS |
| 34937A | 9.4 | 9.4 | 9.4 | 9.5 | 9.4 | 9.4 | 6 PASS |
| 34922A | 9.4 | 9.4 | 9.4 | 9.5 | 9.4 | 9.4 | 6 PASS |
| 34950A | 9.4 | 9.5 | 9.4 | 9.5 | 9.5 | 9.4 | 6 PASS |
| TDS1002 | 9.4 | 9.4 | 9.4 | 9.5 | 9.5 | 9.4 | 6 PASS |

## Artifact and execution evidence

Verified the manifest identity and all **1,316 bound input file hashes**, with zero mismatches. Copied tracked files join the exact recorded revisions: core `1243881c6b91c46ff40cf0cb3cfd4598b8352409` (575 files), Keysight `ac1d71a9fba87be2a1cd2bcc16c6b1888c0f0f13` (365), Tektronix `f71ca340406370cc2467f78b604cc435752cadd8` (137), with no tracked-source mismatch. All14 bound primary PDFs are accessible and hash-verified. The normative specs and own-model source traceability are read alongside actual code and independent acceptance traffic, not treated as implementation credit.

Inspected the **20 original ZIPs** (12 SDK and8 vendor driver proofs), including actual archive members, manifests, phase logs, binary headers, execution records and package/feed contents. Executed both artifact-checking helpers against those original ZIPs in read-only mode with report writes suppressed. GitHub ZIP digests, per-log digests/exit codes, binary execution joins and exact package/feed producer joins all passed. Positive native binaries are x64 ELF/PE; Windows binaries have no CLR header and Linux dependency records show no managed runtime. Intentional warning gates actually fail with IL2026 and IL3050; positive NativeAOT publish records are clean.

All four platform cells per vendor contain **source and packed** proof for Rust default/async and C# NativeAOT on Linux/Windows x64 with .NET8/10. Actual logs show1,859 Agilent cases and452 Tek cases, each C# corpus through four frontends, plus compiled examples and the new admission/lifetime/pending/deadline/serialization procedures. SDK native records exercise all11 added neutral typed operations, byte zero/oversize and existing count constraints. The matching core/vendor workflows have11/9/9 successful jobs, respectively; evidence is assessed from original content rather than conclusion fields alone.

Freshly reproduced both vendors’ active Rust/C# case outputs and profile outputs exactly using the frozen external generator. Packaged catalog semantics match the operation source after excluding authoring-only source references; all12 SDK ZIP wheels carry that catalog. C# core contract outputs also reproduce exactly. Local Rust contract formatting was not rerun because rustfmt is absent in the resumed environment; the immutable producer generation gate and packed source-byte joins cover the accepted artifact.

Inspected concrete Rust/C# factory and backend code, typed core contracts/generator, shared runtime lease paths, procedural hosts and runner wiring. Deliberately distinguished local model configuration, module topology/parent ownership, and execution frontend guarantees. Actual pending hosts fail if sync IO is used on the true-native path, suspend real writes/queries, and prove active access has ended before cancellation/abandonment completion. Queued operations cannot interleave with the full current transaction.

## Per-item reasons

### 34401A

- **EVIDENCE 9.4 / PASS:** Own-model user manual and frozen 34401A specification distinguish seven manual-qualified DMM reads from the retired generic profile; current active registration uses the qualified factory.
- **API 9.4 / PASS:** QualifiedScpi immutable exact-endpoint commissioning and seven typed DMM operations; unsupported reset/shutdown and unbound endpoint requirements fail before acquisition.
- **PROTOCOL 9.4 / PASS:** Exact MEAS commands, per-measurement units, strict decimal exponent grammar, overload handling and bounded complete-message contract; failures after IO quarantine without retry.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.5 / PASS:** 69 independent qualified cases replace 69 retired cases; new actual-factory tests add nonfinite rejection with healthy follow-up, ownership, admission, pending native read/write, deadline and serialization checks.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 33120A

- **EVIDENCE 9.4 / PASS:** Own-model user manual and current 33120A-v2 specification, including load-specific 9.9e37 HighZ/Device precedence, continuous-mode prerequisites and always-live output limitation.
- **API 9.4 / PASS:** Typed frequency/amplitude/offset subset requires explicit continuous-sine/50ohm/Vpp commissioning; no fictitious output-off or shutdown operation is advertised.
- **PROTOCOL 9.4 / PASS:** Current code validates waveform/load/unit/modulation/sweep state before supported traffic; amplitude/offset counterpart checks validate original and transmitted rounded values. Literal HighZ handling and failure precedence match current oracle.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.5 / PASS:** 67 independent cases plus Transport/Timeout injection at every amplitude/offset position, including counterpart reads and final writes; native pending, healthy local rejection and shared quarantine procedures execute in source and packed consumers.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 53132A

- **EVIDENCE 9.4 / PASS:** Own-model programming manual and frozen specification bind channel1/2, gate grid and exact ASC response grammar; no broader counter-family equivalence is assumed.
- **API 9.4 / PASS:** Typed channel/gate intent is local to a handle, immutable factory bindings are selected per endpoint, and rejected setters preserve prior valid intent.
- **PROTOCOL 9.4 / PASS:** Every frequency/period read reissues CONF/ARM/FORM/READ with exact channel/gate formatting; parser precision, exponent, sign and finite/range checks retain distinct Parse/Device outcomes.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.5 / PASS:** 50 independent cases plus repeated reads, two-handle intent isolation, gate boundaries, invalid setters, nonfinite pre-IO rejection, actual pending IO and serialization/deadline/ownership proofs.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 3585B

- **EVIDENCE 9.3 / PASS:** Own scanned operating manual, OCR metadata and inspected format/T6/marker pages support the qualified serial/revision and native R12/dBm/live-A subset. OCR is a navigation aid, not replacement primary authority.
- **API 9.4 / PASS:** Eight typed analyzer operations are restricted to explicitly commissioned native mode; stored-peak selection is distinguished from a newly acquired global peak and from fresh selected-point T6 update.
- **PROTOCOL 9.4 / PASS:** CL-only prelude, native command units/ranges and both returned native fields are checked. Discarded-field parsing is strict; software1Hz floor and fixed commissioning are explicit. No silent mode repair or cleanup.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.5 / PASS:** 123 independent cases cover native numeric layout, both fields, bounds, stored/fresh marker distinctions and errors; new analyzer actual-factory admission, pending writes/reads, timers, ownership and fail-fast follow-up run in every delivered proof cell.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 34980A

- **EVIDENCE 9.4 / PASS:** Own mainframe programming/manual sources plus separate module manuals and frozen topology specification distinguish parent session, installed-model configuration and physical validation.
- **API 9.5 / PASS:** One parent handle owns frozen endpoint-local relay and digital inventories. Slots1..8, model-specific locals, logical/physical duplicates, module conflicts and direction are validated before acquisition; child typed views share parent lease.
- **PROTOCOL 9.4 / PASS:** Operation-specific capability gating uses the selected route, not union inventory. All module guard/body traffic remains under one operation lease, with no discovery or independent module connector.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.4 / PASS:** 7 parent golden cases supplemented by exhaustive slot/local construction, bad/duplicate/conflicting inventories, mutable caller data, selected relay/input/output capability tests in sync/native/worker paths, and digital/relay sibling serialization.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 34937A

- **EVIDENCE 9.4 / PASS:** Own module manual and mainframe programming source bind the32-relay topology and FormC semantics, including NC consequence of opening a FormC relay.
- **API 9.4 / PASS:** Typed Switch view maps frozen logical paths to declared parent slot/local routes; module identity and state are not presented as independent hardware ownership.
- **PROTOCOL 9.4 / PASS:** ROUT commands/query grammar and topology guards follow the parent protocol; open/close mutations are explicit, with no silent reset or shutdown.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.4 / PASS:** 32 independent module cases plus all supported locals across all slots, parent shared ownership and actual pending relay query/write/cancellation/deadline/serialization procedures.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 34922A

- **EVIDENCE 9.4 / PASS:** Own70-channel multiplexer manual and mainframe programming source bind its distinct channel count and nonexclusive switching behavior.
- **API 9.4 / PASS:** Typed Switch view uses exact frozen slot/local model mappings; no invented exclusive switch abstraction or independent child session.
- **PROTOCOL 9.4 / PASS:** Explicit route changes, strict state replies and parent topology guard ordering match the independent module acceptance oracle; no autonomous opening of unrelated routes.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.4 / PASS:** 33 independent cases plus exhaustive70-local ×8-slot construction, boundary/conflict rejection, pending mutation/read tests, shared parent health and whole-operation serialization.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### 34950A

- **EVIDENCE 9.4 / PASS:** Own module manual plus mainframe programming source bind four8-bit banks, anchor/width/direction guards and deferred counter/handshake/electrical features.
- **API 9.5 / PASS:** Neutral ReadByte/WriteByte exposes byte0..255 with positive port identifiers; selected endpoint and commissioned direction restrict operations. Bank/channel encoding remains vendor-local; caller inventories are frozen.
- **PROTOCOL 9.4 / PASS:** Every byte operation checks the specified bank width/anchor configuration before traffic; exact query/write grammar, guards and shared lease prevent sibling interleaving. No silent electrical reconfiguration.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.5 / PASS:** 123 independent cases plus zero/255 success,256 rejection, invalid directions/undeclared paths, mutable configuration, both bank scenarios and actual cross-view pending/cancel/deadline/ownership tests. Core typed proofs retain positive existing count constraints.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

### TDS1002

- **EVIDENCE 9.4 / PASS:** Own TDS1002 programmer/user/module sources bind the confirmed exact non-A/non-B model, GPIB-capable CM/CMA/CMAX modules and MEM exclusion, probe factors and scalar range tables.
- **API 9.4 / PASS:** Ten typed scalar scope operations with frozen per-endpoint module/probe/commissioning configuration; no trace, trigger setters, reset/shutdown or substituted MDO/A/B model support.
- **PROTOCOL 9.4 / PASS:** Strict NR1/NR3/status/quoted-unit parsing, exact scale grids and fresh STOP/SAMPLE/SEQUENCE/RUN/OPC/state/count/source/type/unit/value/finalESR sequence. Value withheld until poststatus success; no automatic STOP after cancellation. Current C# returns CatalogReply.Numeric.
- **PARITY 9.5 / PASS:** Paired Rust/C# source and all four frontends have exact outcome/traffic corpus checks. Original source and packed proof ZIPs contain successful Rust default/async and genuine Windows/Linux x64 .NET8/10 NativeAOT executions, with pending-native procedural tests; no managed OpenTAP AOT inference.
- **VALIDATION 9.5 / PASS:** 334 new independent cases plus118 retained cases, all31 timebase grid points and adjacent binary64 rejection,4×2×11 probe/channel scales, endpoint isolation, fr-FR formatting, every negative health follow-up and actual suspended HEAD/RUN/OPC/VALUE with deadline/cancellation/serialization proof.
- **DELIVERY 9.4 / PASS:** Vendor-local source/package boundary, compiled static typed examples and documented commissioning/deferred behaviors; exact SDK producer1243881 pin and checked release-form feeds/packages. Existing public0.2.0-alpha.1 explicitly lacks additions; future coordinated version required. No new public release, merge or hardware claim.

## Finding history and current disposition

- **C-SOURCE-01 resolved:** current Rust ASCII-before-trim admission matches C# for all three new Agilent factory families. Paired NBSP/EM-space malformed schemes are rejected before acquisition in actual source and packed executions. The earlier defective snapshot remains historical evidence.
- **TDS-C-01 resolved:** both current TDS C# scalar returns use `CatalogReply.Numeric`, matching the pinned SDK. Actual source/packed native corpus and typed examples execute those read and measurement paths. The earlier compile-failure log remains historical evidence.

No new proved mandatory defect or unverified required category remains in this packet. No source changes were made by this reviewer.

## Confidence and limits

High confidence in the declared software subset, static/package separation and recorded native execution. Scores stop short of exceptional completeness because the acceptance evidence is synthetic and depends on declared commissioning and host transport contracts; it cannot establish physical equipment truth. These are explicit scope limits, not missing required software evidence.

- Software conformance only: no physical instrument/module presence, calibration, timing performance, probe/electrical state, protection or output safety was physically validated.
- Declared manual-derived subsets only. Endpoint commissioning, complete plain-text message framing, binary rejection,256-byte allocation bounds and genuine native connector deadline/cancellation termination remain explicit host contracts; synthetic proof verifies those interactions, not arbitrary application connectors.
- Hosted source/packed native executions are inspected from original bound artifacts and execution logs; I did not rerun Windows/Linux .NET8/10 NativeAOT builds locally. Local fresh Python generation was reproduced; rustfmt is absent after environment transition.
- SDK/driver NativeAOT proof does not establish OpenTAP host NativeAOT compatibility. OpenTAP evidence is managed execution.
- Development release-form package/feed evidence is not new public availability. Public0.2.0-alpha.1 lacks additions; no merge, publication, release or repository creation is authorized.
- Keysight proof fixture alone uses opt-level0 and a32MiB stack to accommodate its large generated constructor; production dependencies retain normal release optimization. This limits fixture performance conclusions, not tested driver semantics.
