# Council A — independent module specification review

Member **A**; actual model **gpt-6.1-sol**, reasoning **high**. Stable neutral framing: consider a new user's interpretation. Stage **SPEC**, round **module-spec-r1**, rubric **v1**. Frozen manifest SHA256: **74fbadf2b5b43c093e2ec6f708b42993f9e00c95fb99387433d39e95e2adfb69**. I verified its identity and all **20** bound-file hashes and assessed all twenty-four categories independently. No peer ballot or score was inspected; no earlier scores were carried forward.

**All twenty-four item/category specification votes PASS.** This approves the declared software design and executable acceptance plan for this stage. It does not demonstrate an implementation, hardware behavior or package/native execution.

All votes derive from **>=9.2/10**, using one-decimal numeric precision.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34980A | 9.3 PASS | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| 34937A | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| 34922A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| 34950A | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |

## Inspected evidence

I read all five frozen normative specifications, the module fixture and independent authoring helper/oracles, source metadata, independent clearance, rubric and supporting external-tool/platform plan. The module-specific public constructor and SDK requirements govern; the supporting first-three constructors and old compatible producer do not override them.

Primary PDF checks included:

- 34937A own-module guide15–20:32 relays, FormC/FormA distinctions, latching behavior, lack of analog-bus connection, power-fail jumper caveat and explicit ROUTe switching/query examples.
- 34922A own-model pages within the shared guide21–35:34922 topology is specifically two35-channel banks, direct non-scanning multi-close behavior, internal-DMM four-wire pairing, channel versus analog-bus power-off behavior and schematic.
- 34950A own-module guide19–23:bank/local numbering, byte/word/lword grouping, SENSE preservation of direction versus output-latch readback, SOURCE implicit output selection and per-byte electrical behavior within grouped channels.
- 34980A programmer reference160–184:direction, normal/inverted polarity, HIMP/OFF/ON handshake distinction, bank-first addressing and actual numeric1/2/4 width return format. Pages568–570,596–597,756–757,759,769–770 and773–774:input data format/side effects, input/output memory restrictions, output data direction/width effects, active/open-collector drive, logic-level limits and memory-trigger behavior.
- Own-model mainframe guide for physical interface and slot/channel addressing context. GPIB admission is a declared subset; other physical interfaces are not falsely denied.

I executed the independent oracle helper in a fresh temporary directory. All four shards and the fixture reproduced the frozen bytes exactly. Counts are **7 mainframe +32 relay +33 mux +123 digital =195**, with195 globally unique case IDs and exactly one expected result/error per case. Every shard intentionally targets the single parent `keysight.34980a` factory; module file names are test grouping, not independent device sessions. This check establishes authoring reproducibility only.

## 34980A mainframe

**EVIDENCE9.3 — PASS; high confidence.** Own-model primary sources, hashes and relevant module references are inspectable. Mainframe and module responsibilities, GPIB-only admission, address derivation and deferred internal-DMM/scan scope are explicit. Physical inventory and exclusive, nonautonomous operating conditions are caller assertions, not falsely presented as discovery or hardware validation.

**API9.4 — PASS; high confidence.** One factory/handle/connector supplies neutral Switch and DigitalIo views under a common runtime lease. Concrete equivalent Rust/C# construction freezes exact-endpoint inventories, separates logical switch/port namespaces, rejects physical/type conflicts across namespaces and gates selected-endpoint capabilities before acquisition. The five new operations are neutral; module identities, slots and SCPI remain external. No fictitious independent module factory or matrix endpoint is introduced.

**PROTOCOL9.3 — PASS; high confidence.** Whole-operation locking covers digital guards and final access, with shared parent uncertainty/quarantine and resource lifetime. Deadline, framing, allocation, no-retry and no-device-cleanup contracts are explicit. No scan abort, DMM enable, discovery or global OpenAll becomes a hidden side effect. Lifetime no-autonomous-actions commissioning correctly addresses external-trigger changes that present-state guards cannot exclude.

**PARITY9.3 — PASS; high confidence at SPEC stage.** The new neutral schema generates both language/frontend surfaces and neutral OpenTAP views through the established tooling. The plan explicitly requires a new compatible immutable SDK producer/tool revision for these five operations. No old-SDK binary is claimed compatible. Source/generated and packed Linux/Windows x64 ×.NET8/10 execution is a concrete later gate.

**VALIDATION9.3 — PASS; high confidence at SPEC stage.** The seven parent cases join195 cross-module cases, fixture mapping and M-PROC01–15. Admission boundaries, cross-namespace conflicts, mutation freezing, selected-route capability isolation, one-parent noninterleaving/quarantine/close and real pending IO are all required. The proposed scenarios test actual module views rather than assuming an earlier SDK helper establishes their integration.

**DELIVERY9.3 — PASS; high confidence at SPEC stage.** All proprietary code and static factory composition remain in the external Keysight package; fixtures are development evidence. Existing external generator/TestKit/package workflow commands are reused under an expressly updated producer for new operations. Source-only iteration, checksum-joined packed proof and future coordinated public versions are distinguished. No merge/publication or hardware authorization is inferred.

## 34937A relay module

**EVIDENCE9.3 — PASS; high confidence.** Own-model topology and switching examples support local1–32, FormC1–28, FormA29–32, latching relays and no analog bus. The spec makes no universal power-off isolation promise; this is appropriate because the primary guide describes configurable power-fail behavior for the5A relays.

**API9.3 — PASS; high confidence.** Three neutral single-channel methods model the common-to-normally-open path, with sparse logical mapping and parent ownership. The FormC open-to-NC consequence is prominent. State queries report commanded state, not continuity. No global OpenAll or generic paired-route capability is advertised.

**PROTOCOL9.3 — PASS; high confidence.** Exact selected-path ROUT:CLOS/OPEN/CLOS? messages and strict0/1 parser are concrete. No list/range expansion, implicit reset or output-isolation promise appears. Errors quarantine all parent views; software close leaves commanded state untouched and preserves borrowed host ownership.

**PARITY9.3 — PASS; high confidence at SPEC stage.** The same new neutral switch operations and frozen mapping drive both language/frontends. Parent lease/lifetime rules and externally generated/packed proof apply to the actual relay view. Final source/native package evidence must use the compatible new SDK producer.

**VALIDATION9.3 — PASS; high confidence at SPEC stage.** The32 literal cases exercise the FormC/FormA boundaries, both Boolean replies, malformed tokens and all operation failure classes. M-PROC01/02/03/06–12 add slot/local admission, immutable sparse mapping, parent-wide failure/lifetime, NC documentation and concurrent/pending behavior. Physical NC/contact continuity is correctly outside simulator assertions.

**DELIVERY9.3 — PASS; high confidence at SPEC stage.** The relay is a documented module view from one external parent factory, with explicit supported/deferred behavior and wiring caveats. Static construction and shared package/native plan avoid fictitious module deployment or independent physical sessions. Hardware validation and future coordinated release remain separate.

## 34922A multiplexer module

**EVIDENCE9.4 — PASS; high confidence.** The inspected own-model pages distinguish the two35-channel armature-latching banks from neighboring modules. They support non-scanning multiple closures, internal-DMM-only four-wire pairing and channel-relay power retention. Analog-bus addresses and scan behavior are acknowledged and excluded from this subset.

**API9.3 — PASS; high confidence.** Single logical channels map to1–70 without matrix aliases, pair invention or implicit exclusivity. Closing a second same-bank path can connect multiple external signals, and this is disclosed as an application wiring responsibility. Scalar acquisition and scanning are deliberately unavailable under the current ownership contract.

**PROTOCOL9.3 — PASS; high confidence.** Exact parent ROUTe commands, strict Boolean replies and no implicit paired/open operations match the admitted direct-switching use. Mainframe-global scan effects are not hidden inside apparently local methods. Quarantine and close follow the common parent boundary.

**PARITY9.3 — PASS; high confidence at SPEC stage.** Generated neutral switch methods, shared address/mapping constraints and identical traffic cases give a concrete cross-language contract. Native/worker gating and the new compatible SDK/source/packed matrix explicitly cover this actual module view.

**VALIDATION9.3 — PASS; high confidence at SPEC stage.** The33 cases cover1/35/36/70, state/parser/failure behavior and an explicit same-bank sequential-close transcript without inserted opens. M-PROC01/02/06–12 cover otherwise-valid mapping negatives, slot boundaries, concurrency, parent quarantine and pending IO. Unadmitted analog-bus locals and scan/acquisition capabilities remain required admission negatives.

**DELIVERY9.3 — PASS; high confidence at SPEC stage.** External shared-parent distribution, module-specific limits and prerequisites, preserved corpus and actual packed/native proof requirements are explicit. No old public package or neighboring module is represented as containing/validating this support.

## 34950A digital module

**EVIDENCE9.4 — PASS; high confidence.** Own-module and programmer-reference pages substantiate the bank/local mapping, width grouping, direction/token formats, HIMP distinction, memory behavior and SENSE-versus-SOURCE semantics. Fixed2.5V input threshold and3.3V active output policy is within documented settings. Sampled data formatting is separate from general reading metadata.

**API9.4 — PASS; high confidence.** ReadByte and WriteByte expose neutral positive logical ports, frozen direction and inclusive0–255 values. Count zero is explicitly legal without loosening unrelated positive-count contracts. Input reads cannot silently return an output latch, and output mutation is explicit. Physical electrical policy is a truthful commissioning requirement, not implicit setup or autodetection.

**PROTOCOL9.4 — PASS; high confidence.** All four width queries precede selected-port direction/polarity and bank-first handshake/memory guards. Non-anchor ports cannot bypass an adjacent WORD/LWOR grouping. SENSE performs no hidden MEAS configuration; SOURCE's implicit direction/width effect is acknowledged and guarded by matching current state. Strict byte grammar, exact numeric guard equality, failfast stopping and parent quarantine are concrete. Persistent external triggers are handled by lifetime commissioning rather than a false promise that current memory-off readback predicts future state.

**PARITY9.3 — PASS; high confidence at SPEC stage.** The same neutral u32/Count contract, zero-inclusive schema constraints, strict guards and independent cases bind both implementations. Generated frontend/OpenTAP additions and actual source/packed NativeAOT matrix require compatible new SDK artifacts, so old positive-count generators cannot silently reject zero.

**VALIDATION9.3 — PASS; high confidence at SPEC stage.** The123 cases cover both banks, sparse input/output bindings,0/255 values and256 rejection, zero/leading-zero replies, grammar failures, every guard mismatch and every transaction failure position. M-PROC05/06/08/10/11/13/14 add wrong-direction pre-IO behavior, adjacent-group defense, cross-module quarantine/order, genuine pending writes/reads, zero-health checks and lifetime autonomous-action commissioning. Implementation must instantiate these actual scenarios in both languages.

**DELIVERY9.3 — PASS; high confidence at SPEC stage.** Static byte-only subset, fixed electrical policy, no direction setters/buffer/counter/handshake feature and unchanged-on-close behavior are documented. New SDK generation, package dependencies and qualified producer transition are explicit. Guard success is never presented as calibrated pin-level evidence or electrical protection.

## Required changes and limitations

**Mandatory defects: none demonstrated. Required specification changes: none. Rubric amendments: none.** This is a complete passing specification ballot for these four declared items under the frozen packet, subject to full Council aggregation.

Implementation review must inspect actual code/tests and source/generated plus packed/native executions using a newly qualified compatible immutable SDK producer and matching external tool catalog/schema. The old6524 producer cannot qualify the new operations. No module backend, package/native execution, physical inventory verification, contact continuity or calibrated electrical behavior has been demonstrated here.

Primary pages were sampled for each supported topology/command/parser family, not read cover to cover. Truthful inventory/electrical/lifetime commissioning remains essential; guard queries alone cannot prove future absence of external autonomous actions. Only my ballot artifacts were written. **3585B and TDS1002A remain pending**, and no nine-item completion claim follows from this review.
