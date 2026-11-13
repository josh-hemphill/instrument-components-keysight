# Member A final independent ballot — implementation r6

Actual reviewer configuration: **gpt-6.1-sol / high**. Member A framing: **new-user interpretation**. Rubric **v1**. The recorded runtime failure required replacing unavailable A; no prior A numeric ballot existed. No peer ballot or scores were read. Earlier A factual audit records informed verification targets; every score below is fresh against the current common freeze.

Rubric ballot: **APPROVE**, recorded before grading in `member-a-rubric-ballot.md`. No amendments requested. Threshold is **>=9.2** independently in every category for every model, precision0.1; all36 votes below derive from that threshold. This is one member's ballot, not a three-member Council aggregate.

## Frozen identity and direct checks

- Main: `1b0704dbb2c4a9f4ffda9f6b1665de12654f672d`.
- External Keysight: `3d384a42a3dbb460314b77a4174abefdd4015190`.
- SDK producer: `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`.
- Source manifest SHA256: `13a78c6c6956e0fec18a3934a4b337d2eb3d9ea8ac2807acdb2e3e82c66bc530`.
- Final evidence manifest SHA256: `4ff9bffc3d4b198ac2924a471628163f5d74401ff7489e01d0cdde88952d9c25`.

Both source worktrees remain clean at the stated HEADs. I reran `member-a-artifact-check.py` after reading its assertions: all824 tracked-file hashes, both manifest hashes, all manifest-listed evidence hashes, eightZIP digests,264 phase-log hashes,12 native binaries and40 SDK/core package checksum joins passed. The script reads the already-downloaded ZIP bytes, not another member's score. Its output is `member-a-final-artifact-audit.json`. I separately reran acceptance composition and counted all1424 unique cases with exactly one final expected result/error:56 each5334,53DAQ,1528920A,1488920B,1288921A,831 existing regressions. The four acceptance-provenance spec hashes match actual current specifications. All11 primary PDF byte hashes match `provenance/hp-manual-sources.json`.

Direct source inspection covered all Rust and C# HP engines/factories, their common message engines, all six model specifications, acceptance/procedural matrices, fixture registration/runners, admission and actual pending-IO cancellation/deadline proofs; neutral generated requests, validation, typed sync/async classes and OpenTAP steps; current allocatingVISA connector and its ownership/framing tests; SDK neutral typed consumers and worker/partial-open proofs; package manifests, current CI workflow, and actual execution logs. Primary reference inspection included native counter3-80/84/85, DAQ native syntax/output/range pages, own-model RF command/trigger pages, the actual8920B datasheetp9 image, and8921A1998p43/61/81. PDF identities, page provenance and software policy are distinguishable.

## Numeric scores and threshold-derived votes

Every numeric cell is an independent category score; **P means PASS (score>=9.2)**.

| Model | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| HP5334A | 9.4 P | 9.4 P | 9.4 P | 9.5 P | 9.5 P | 9.3 P |
| HP5334B | 9.4 P | 9.4 P | 9.4 P | 9.5 P | 9.5 P | 9.3 P |
| HP3497A | 9.3 P | 9.3 P | 9.4 P | 9.5 P | 9.5 P | 9.3 P |
| HP8920A | 9.4 P | 9.3 P | 9.4 P | 9.5 P | 9.5 P | 9.3 P |
| HP8920B | 9.3 P | 9.3 P | 9.4 P | 9.5 P | 9.5 P | 9.3 P |
| HP8921A | 9.4 P | 9.3 P | 9.4 P | 9.5 P | 9.5 P | 9.3 P |

## Per-model reasons

### HP5334A

**EVIDENCE9.4:** The own-model05334-90047 December1993 source covers frequencyA/B, periodA, GA, IN and native output grammar; specification/provenance cites printed3-78–85. Actual manual3-80 confirms MR/MS are unavailable only onB, while those modes remain deferred here. The ambiguous shared identification response is explicitly excluded from supported identity probing. Synthetic parser-policy evidence is labeled.

**API9.4:** Distinct `keysight.hp5334a`, exact passive identity rules and explicit native/internal-gating commissioning support a small useful Counter surface. Logical channel1/2, Hertz/Second results, finite inclusive gate range and A-only period semantics are coherent. Reset is explicit; optionalC/totalize and unsafe global shutdown are not falsely advertised. Rust/C# examples make close/resource scope clear.

**PROTOCOL9.4:** Both source implementations issue FN before HS0/MD1/WA1 and reapply explicit gate before RE; GA cache/reset updates occur only after successful IO. The strict19-character parser, shorter spaced-zero exception and exact letterO overload match the declared grammar. IO marks the session unhealthy before execution/await; parse/transport/timeout/abandonment leaves quarantine and no automatic TE/IN/retry. Owned close is idempotent and borrowed IO preserved.

**PARITY9.5:** Rust/C# action/state/parser code agrees, including E11 gate formatting and invariant exponents. Sync, native and two worker frontends execute the same separate56 cases in source and packed consumers on all four hosted platform/framework cells. Native connectors are optional and honestly gated; true native cancellation is separately proved. Actual NativeAOT ELF/PE binaries and SDK typed rooting proof support the distribution claim.

**VALIDATION9.5:** Its56 independently specified cases exercise channels,period rejection,gate/formatting/reset,zero/overload/malformed replies and typed failures. Counter procedural proof adds passive/open/commissioning/nonfinite/ownership/quarantine/native cancellation. Shared all-six pending-read proof establishes actual finite timeout and abandoned-future cleanup. All1424 cases execute with exact traffic/value/unit/status/error/lifetime checks; affected main regressions pass.

**DELIVERY9.3:** Its implementation is present in the independent Keysight crate/DLL, statically selected and absent from inspected core source archives. Source/packed native matrix and checksum-joined pinnedSDK feeds are concrete. README explicitly states publicalpha.1 lacks this addition, commissioning/deadline/framing needs and no hardware validation. A new coordinated public version remains future work, correctly disclosed.

### HP5334B

**EVIDENCE9.4:** Shared05334-90047 is explicitly anA/B manual, with separateB specification and provenance. Manual3-80 excludes MR/MS onB and the specification separately documents Option700/CIIL uncertainty and address setup; noA-only store behavior is inherited.

**API9.4:** `keysight.hp5334b` and exact5334B passive matching remain distinct despite verified wire equivalence. Unknown/CIIL commissioning is rejected before connector acquisition. Same useful typedCounter units/channel/gate/reset surface asA, with optionalC and remaining functions honestly deferred.

**PROTOCOL9.4:** The inspected shared engine uses the declared exact native commands, gate-after-function policy, strict record/overflow parsing and fail-fast quarantine. No automaticGAL language change occurs. Borrowed/owned,close/recovery and native cancellation semantics agree with theB specification and are executed on actualB factory instances.

**PARITY9.5:** Separate56B objects exercise both languages and all four frontends in generated/source and packed modes, including B selection and gate formatting. NativeAOT and normal optimized driver/SDK dependency evidence applies to actual statically rootedB factory code, not an assumed family alias.

**VALIDATION9.5:** SeparateB56 cases plus counter commissioning/lifecycle and all-six stalled-read/cancellation procedures cover model binding and shared engine contracts. Expected wire literals are independent golden data; runners assert whole exact scripts and resource lifetimes. Main and package generation/regression gates have current successful execution evidence.

**DELIVERY9.3:** Independent external package and explicitB registration require no core vendor registry. README includes aB C# construction example and warns Option700 may start inCIIL. Packed artifacts are verified; publishedalpha.1 is accurately excluded, and hardware/release status remains limited.

### HP3497A

**EVIDENCE9.3:** Own03497-99000 identity is corroborated by official and mirror PDF hashes; pages connect Option001,20-address slots,010/020/reference/compensation restrictions, native sequence and eleven-byte voltage grammar. The v9 positiveVR1/VR2 examples fit range; ±120V literals are explicitly synthetic parser-policy boundaries, not physically displayed reading oracles. Scanned/diagram interpretation and restricted first surface justify restraint below exceptional scores.

**API9.3:** One neutral channel-aware DC acquisition returnsVolt without misrepresenting the chassis as aDMM/Switch. Derived native20×slot+local removes redundant inconsistent coordinates. Validated copied inventory rejects duplicate public/native mappings, conflicting slots, unknown options/compensation and reference19 before open. Exact endpoint binding has no fallback. README/spec prerequisites are useful; a complete standalone DAQ application example would further ease first use.

**PROTOCOL9.4:** Both engines send exactly VT4AE0VC0VS0VF1VN1VW0SO1VRnAIc under one lease and neverGET. Supported ranges are exact, strict eleven-byte signed ASCII parsing recognizes only positive overload, and magnitude bound is explicit software policy. Standard connector clean-boundary/deadline/allocation obligations are documented. Failure/cancellation quarantines without another command, drain or implicit reset.

**PARITY9.5:** Rust/C# freeze to equivalent logical/native maps and return equivalent units/status/errors.53 independentDAQ cases and admission/pending-read proofs run through source/packed sync,worker and genuine native paths in Linux/Windows net8/net10 artifacts. The neutralDAQ typed wrapper is independently rooted/executed in optimizedSDK producer consumers and NativeAOT.

**VALIDATION9.5:**53 golden cases plus isolated duplicate invariants,slot conflict,local20,Hardware/Unknown compensation,4096/4097 bounds and valid acquisition after caller mutation provide meaningful negative/mutation evidence. Procedural proofs cover faults,borrowed/owned lifetime and actual pending deadline/native cancellation. Body-layer partial-response tests remain separate from opaque-host model tests; regression evidence covers the neutral additions.

**DELIVERY9.3:** The actual Rust source is hash-matched inside the external crate, .NET package contains only its vendorDLL with pinnedScpi dependency, and neutral core inventories lack backend code. Endpoint/module/response policy and excluded scans/outputs are documented. Reproducible pinned feeds and actual packed consumers establish prepublication delivery, while publicalpha.1 support is explicitly disclaimed.

### HP8920A

**EVIDENCE9.4:** Own08920-90220 RevB/A.18.00+ and5968-5385E establish the route/command/units/option envelopes; A's different receiver minimum and007/008/016 combinations are explicit. Specification distinguishes continuous-power table from contradictory headline rows and freezes exact nearest-binary64 power bounds with derivation provenance. No hardware/settling claim is inferred.

**API9.3:** Five useful neutralRF operations use typed frequencyHz,levelDbm,Boolean andQuantity results. Explicit immutable options/source/receiver ports and qualified firmware bind the exact endpoint. ANT cannot inherit another route's power capability. Configure keeps output disabled, enable is separate, receiver invalidates source readiness. A complete application sample would improve approachability of the commissioning-heavy binding surface.

**PROTOCOL9.4:** Inspected Rust/C# steps verifyOFF beforeCONFIG, normalize/read backCONFIG andATT, preserve modulation, compare decimal half-step readbacks exactly and triggerFULL/SING freshly on every receiver call. Power-only sends no extra frequency query. Finite domains distinguish known wrongDevice from malformedParse; both quarantine. Source/receiver and electronic/mechanical envelopes remainA-specific. No retry/cleanup/abort is hidden after failure.

**PARITY9.5:** Both actual source engines agree on validated route operations, canonical integerHz/tenths formatting, exact decimal comparisons, state transitions and error classes.152A cases run in both languages/all four frontends across source/packed hosted cells. Real native cancellation/pending-write/read deadlines, NativeAOT binaries and independentSDK typedRF consumers establish the relevant async/AOT surface.

**VALIDATION9.5:**152 independent cases include options/bounds/immediate outside floats,readback/token grammars,OFF/configure/enable and repeated receiver lifecycle. Procedural route admission,mutation,nonfinite arguments with successful usable follow-up,quarantine,ownership and pending nativeIO cover what final-outcome fixtures cannot express. Existing831 regressions and all generation/native-warning gates pass.

**DELIVERY9.3:** Statically registeredA backend remains wholly external, with versioned model specification and exact commissioning/physical limitations. All actual source/packed archives and dependencies were inspected; release-form feeds are checksummed to pinnedSDK. README accurately distinguishes unreleased new source from publicalpha.1. Additional completeRF startup examples would be useful, not mandatory for declared scope.

### HP8920B

**EVIDENCE9.3:** Own08920-90222 RevD/B.06.00+ and5968-1376E are retained/hash-verified. I viewed actual datasheetp9:055 explicitly excludes006/007. Current source/spec reject those combinations; p3 source limits and model-specific receiver/power bands remain separate from contradictory later numeric rows. This is conservatively documented evidence resolution, not an application-asserted exception.

**API9.3:** DistinctB factory validatesB firmware/options before acquisition, freezes exact endpoint routes and exposes only supported receiver power onRFInOut. Explicit0055 restrictions andB electronicDuplex3.5dBm bound prevent silentA reuse. Typed configure/enable/state/receiver surface is coherent; commissioning and sparse binding require careful setup as documented.

**PROTOCOL9.4:** Shared transaction engine is justified by inspected own-model syntax/trigger evidence, while configuration/bounds remainB-specific. Mechanical055+006/007 fail construction with zero IO. Strict scalar/token/state grammar, decimal tolerances,OFF verification, fresh trigger, readiness invalidation and quarantine match both implementations. Borrowed resources and fail-fast failure policy are preserved.

**PARITY9.5:**148B wire cases plus explicit rejected factory combinations run in actualB instances in Rust/C#, source/packed and four frontends. Linux/Windows NativeAOT, genuine native pendingIO cancellation/deadlines and staticSDK typedRF proof are executed evidence. No capability is inferred merely fromA success.

**VALIDATION9.5:** The removed unsupportedB route execution fixtures are replaced by pre-open zero-acquisition rejection procedures, fitting their actual lifecycle position.148 remaining exact cases cover valid model-specific options/bounds/transcripts. Route isolation,nonfinite usable follow-up,mutation,4096 bound,Device/Parse quarantine and stalledIO procedures are meaningful. All cases/regressions and package/native gates execute successfully.

**DELIVERY9.3:** Actual vendor-only archive content,staticB factory selection and pinned checksummed feeds are verified. README clearly identifies the055 restriction andB prerequisites and does not imply publicalpha.1 contains additions. The remaining evidence conflict is disclosed and narrowed rather than presented as supported hardware. Version publication and bench validation remain future work.

### HP8921A

**EVIDENCE9.4:** Current qualification uses its own08921-90031 December1998 RevA/A.15.00+ PDF, not olderA.12 orA/B analogy. Actualp43 confirms numeric writes preserveOFF,p61 requires selected-device-clear before abort,p81 expands RFG UNIT toW/DBM/V/DBUV. Own5968-5820E limitsStandard-only input power and model-specific carrier/receiver bands. Historical manual remains clearly segregated.

**API9.3:** Distinct8921A binding rejects allA/B input-power options and too-old/wrong-family/malformed firmware before IO. Typed five-operation surface,independent immutable ports,exact endpoint andANT gating are useful and conservative. NoA.12 partial route or broad call-processing support is invented; a full startup example would ease commissioning-heavy first use.

**PROTOCOL9.4:** Both source implementations apply qualified own-model UNIT domains, classifyV/DBUV asDevice onRFG butParse onTX-power, and preserve shared verifiedOFF/configuration/readback/trigger grammar. Electronic receiver10MHz and055400kHz differ from source minimum; power carrier requirement remains commissioned. Readiness and quarantine semantics match specification without implicit cleanup.

**PARITY9.5:**128C cases run independently in both languages/source/packed/all four frontends; all-six cancellation/read proofs and all-threeRF pending-write proofs instantiate this own factory. ActualNativeAOT ELF/PE execution,static roots and independent typedSDK consumer execution substantiate the declared platform behavior.

**VALIDATION9.5:**128 independent cases exercise the narrowerStandard routes,UNIT/state/token/decimal boundaries and lifecycle. Procedural tests reject unsupported options/firmware/model/commissioning,freeze configuration and enforce exact selected capability before acquisition. Native/worker ownership/cancellation and actual finite pendingIO deadlines supplement transcript fixtures. Current full matrix and affected regression/generator gates pass.

**DELIVERY9.3:** The own-model factory is statically available in inspected independent vendor packages with no proprietary core implementation. README/spec stateA.15 minimum,Standard-only options and hardware/physical/commissioning limits. Reproducible pinned-source and packed NativeAOT evidence is present; publicalpha.1 is correctly excluded and a new coordinated version remains required for public release.

## Execution and artifact identity

Vendor run **43322642590** succeeded in all9 jobs on frozen vendor3d384a4; four cells areLinux/Windowsx64×net8/net10. The source/generated and packed consumers run1424 cases (593HP+831 regressions) throughSync,NativeAsync,WorkerAsync,WorkerBlocking in each language where enabled. I inspected actual Windowsnet10 source/packed Rust andNativeAOT execution log contents, and rehashed/checked corresponding logs for every cell. Intentional trim/AOT warning probes fail withIL2026/IL3050 as expected; positive native publication logs contain no trimming warnings.

| Artifact | ID | Independently checked phases | Actual native binaries |
|---|---:|---:|---:|
| driver-proof-windows-latest-net10.0 |13992515568|19|2|
| driver-proof-ubuntu-latest-net8.0 |13991458163|21|2|
| driver-proof-ubuntu-latest-net10.0 |13990928706|21|2|
| driver-proof-windows-latest-net8.0 |13990689310|19|2|
| driver-sdk-ubuntu-latest-net8.0 |13991486481|47|1|
| driver-sdk-windows-latest-net10.0 |13991278021|45|1|
| driver-sdk-windows-latest-net8.0 |13990598373|45|1|
| driver-sdk-ubuntu-latest-net10.0 |13990303376|47|1|

AllZIP SHA256 values are recorded in `member-a-final-artifact-audit.json`. Native LinuxELF64/x86-64 dependencies excludeCoreCLR/hostfxr; WindowsPE/AMD64 binaries have noCLR header. Driver Rust archives use registry dependencies only, exclude validation harness, and contain current hash-matched HP engine source (WindowsCRLF normalized toLF). NuGet DLL inventories and exactScpi dependency are checked. SDK/core archives do not contain the HP Rust implementations. Forty producer-to-consumer package checksum joins bind tenSDK/core packages in each matrix cell. Independent read-only artifact-audit run **43322968632** also succeeds; my ballot relies on direct local byte/log rechecks as well.

Current main CI **43314190052** (11jobs) and.NET **43314190048** (8jobs) succeed at1b0704d. I inspected the current-main vs pinned6524 delta: only standard allocatingVISA non-Owned guard, its tests/docs/local evidence changed. Actual guard precedes acquisition; Borrowed and invalid99 tests prove no opener/writes/disposal. This correction is separately covered by15VISA/30OpenTAP local tests plus current main hosted checks; unchangedSDK packages do not falsely claim to contain it.

I separately rehashed all local release-consumer logs and inspected verbose compiler invocations: actual driver/api/scpi/testkit/runtime dependency crates useopt3; only unpublished fixture consumer usesopt0. Its finite32MiB thread preserves the same run function and assertions, propagatesResult/spawn failures/panics and does not replaceSDK worker owner threads. The normal-release original local proof and normal optimizedSDK typed consumers remain available. This limited validation profile is accurately documented and is not hardware or production performance evidence.

## Confidence, limits and changes

Confidence is high in the declared manual/simulator software surface and checked execution/package identity, with moderate confidence in behavior on physical instruments pending bench work. There is no required category markedUNVERIFIED: the specified software acceptance evidence is available. Physical operation, calibrated performance and release availability remain explicitly outside the proved claims.

**Mandatory defects:** none found. **Required changes:** none for this frozen declared scope. **Rubric amendments:** none. Each36category/model vote isPASS; no overall average is used.

Nonblocking improvements, in priority order:

1. Add complete copyable DAQ andRF application composition examples beside the existing counter examples, including an explicit host,endpoint/module/options,timeout/framing and typed operation sequence. This would further improveAPI/DELIVERY first-use clarity without changing supported behavior.
2. When actual hardware is available, retain per-model/firmware/option bench transcripts for initialization,framing,readbacks,cancellation/recommissioning and physical settling. This would improveEVIDENCE confidence; the current ballot does not require unavailable hardware or equate simulation with it.
3. Before any public distribution, select new coordinatedSDK/driver versions and rerun the documented release matrix against those versioned artifacts. Current checksummed development feeds prove prepublication consumption, not publicalpha.1 support.

Limitations retained in machine-readable ballot:

- No physical hardware validation: manual-backed synthetic transcripts establish declared software behavior, not hardware timing, settling, calibration, input protection or commissioning truth.

- Public 0.2.0-alpha.1 lacks these additions; source/packed CI uses checksummed development feeds from immutable SDK6524d9a. A coordinated new SDK/driver version is required before publication; no merge or release is approved by this ballot.

- Custom connectors must enforce finite deadlines, response allocation bounds, clean initial message boundary and completion of host access before native cancellation returns. A later stale complete frame cannot be detected reliably by the scalar API.

- Fixture consumer alone is opt0 and runs on a bounded32MiB thread; driver and SDK dependencies remain opt3. Generic wrappers instantiated in that consumer can use opt0. Normal-release SDK typed consumers and supplementary original normal-release Rust proof remain separate evidence.

- OpenTAP.Visa allocation ownership correction is separately tested on current main1b0704d. Vendor NativeAOT evidence does not establish that OpenTAP itself is AOT-compatible.

- Fresh replacement A independently re-inspected source, primary evidence and retained artifacts; no hardware or Windows binary was executed on this Linux reviewer host. Hosted execution is evidenced by authenticated-run records and hash-joined downloaded artifacts/logs, not claimed local execution.
