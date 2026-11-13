# Council B — full nine-item SPEC review under rubric v2

Member **B**, actual model **gpt-6.1-sol**, reasoning **high**. Stable framing: consider boundary guarantees and evidence. Stage **SPEC**, round **full-spec-v2-r1**, approved rubric **v2**. Target **>=9.2/10** per item/category, one decimal. Frozen manifest SHA256 **`c52c37b13a460ea7e5c86916d1d51c9377fc8e9f11848347a3727581f5208a76`**.

I independently verified the manifest and all61 bound file hashes. All nine normative specifications and four supplements, normative oracles/fixtures/provenance/helpers, independent clearances and relevant own-model primary evidence were inspected. The old33120A66-case oracle is authoring input only. There are **838 normative independent cases**; every shard has unique IDs and exclusive expected-result/error outcomes. The334-caseTDS oracle and fixture reproduced byte-for-byte from the independent helper in an isolated temporary directory; provenance joins its334 IDs one-to-one.

This ballot independently reassesses all54 cells from the current fullv2 packet. No v1 scores were carried forward or peer ballots read. All54 threshold-derived votes are PASS in this individual ballot; no demonstrated mandatory SPEC defect remains. Council aggregation still requires all three independent ballots.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 34401A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| 33120A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| 53132A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS |
| 3585B | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS | 9.2 PASS |
| 34980A | 9.3 PASS | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| 34937A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| 34922A | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| 34950A | 9.4 PASS | 9.4 PASS | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS |
| TDS1002 | 9.4 PASS | 9.4 PASS | 9.4 PASS | 9.3 PASS | 9.4 PASS | 9.3 PASS |

## Primary evidence inspected

For34401A, checked own manual MEAS/default/range and fixed output-format sections (printed112–118/159/61);33120A actual microhertz table, coupled bounds and printed151 load reply;53132A CONFIG/frequency/period/arming/ASCII including printed4-87 gate limits. Module evidence includes own34937A topology/NC/power-fail sections,34922A printed34–35 topology/nonexclusive direct routing and34980A route/width/direction/HIMP/memory/SENSE/SOURCE sections. Scanned3585B evidence includes actual renderedPDF268format,277trigger,221marker units/visibility and272/274native commands, with bound OCR/render provenance.

ForTDS1002, directly inspected qualifying programmer tables1-1/1-2/2-1 for exact non-B/module/GPIB boundaries; acquisitionNUMACQ/STATE/STOPAfter; probeNR1 and non-Bfactor restrictions; DESE/ESR read-clear; HEADER andVERBOSE argument behavior; immediate source/type/quoted-unit/value/warning; andOPC single-sequence completion pages. Own-model user printed147/PDF165 explicitly demonstratesTDS1002 identity spelling, unsuffixedSOURCECH1 and5.16E0with10Xprobe; own timebase/vertical limits and later own-applicable optionalSOURCE[1] syntax corroborate the design. No B/TPS-only USB/current/math features are used.

## Per-item category reasons

### 34401A

- **EVIDENCE 9.4 PASS:** 34401-90004 Edition10 is own-model evidence; MEAS/default/range semantics, fixed scalar grammar and positive overload are traceable to printed112–118,159,61. Temperature/USB inheritance is excluded.
- **API 9.3 PASS:** Seven typed DMM methods map to six physical functions with explicit two-wire alias and per-call units. Frozen commissioned endpoint binding and required-capability/native checks precede acquisition; old generic registration is replaced.
- **PROTOCOL 9.3 PASS:** One explicit-DEF query, E11 formatting, strict15-character grammar, positive-only overload,256-byte/LF host contract, finite deadline and whole-operation lease/quarantine/owned-borrowed close are concrete.
- **PARITY 9.3 PASS:** Corresponding Rust/C# factory and method/error contracts use actual-model selectors, real native admission and SDK workers. Shared plan specifies source/packed Linux/Windowsx64 net8/net10 NativeAOT and warning gates.
- **VALIDATION 9.3 PASS:** 69 independent cases cover every exposed scalar method, auto/explicit/non-nominal/max ranges, units/status, invalid boundaries and parser/IO failures. Q-PROC01–11 add actual-factory admission, immutable bindings, health/lifetime/concurrency and pending IO.
- **DELIVERY 9.3 PASS:** External Keysight qualified factory, historical retention of69 retired cases/profile, explicit removal from active registration, generator/TestKit commands, checksum-joined package proof and future public-version boundary are specified.

### 33120A

- **EVIDENCE 9.4 PASS:** 33120-90006 Edition6 supports exact commands, always-live output, fixed50ohm source/load-accounting distinction and coupled voltage restrictions; rendered frequency table confirms100microhertz. Printed151 explicitly establishes9.9E37 HighZ.
- **API 9.3 PASS:** Four typed generator operations use a frozen authorized continuous-sine/50ohm/VPP endpoint. Guards preserve external mode truth. No output-enable/shutdown fiction; write completion and setting-read semantics are explicit.
- **PROTOCOL 9.3 PASS:** Guard order, load-specific HighZ Device precedence, strict numeric policy, original-and-rounded coupled checks and post-query InvalidArgument/quarantine are unambiguous. Configuration writes explicitly change the live output; close releases only resources.
- **PARITY 9.3 PASS:** Rust/C# constructors, guards, E11 and error/health semantics are shared. Actual commissioned-factory selectors join the same corpus to all frontends and source/packed platform/AOT plan.
- **VALIDATION 9.3 PASS:** 67 independent cases include real HighZ, endpoints, guard failures, rounded-wire conflicts and every traffic failure position; Q-PROC06/09–11 require no-write conflict, quarantine, pending native IO and serialized groups.
- **DELIVERY 9.3 PASS:** External package, physical-load/output-change prerequisites, supported/deferred modes and publicalpha.1 exclusion are documented. Shared exact producer/bootstrap tool strategy and coordinated future-version/native qualification are concrete.

### 53132A

- **EVIDENCE 9.4 PASS:** 53131-90044 explicitly includes53132A; inspected frequency/period CONFIG, arming, ASCII and gate sections support the exact subset, distinct from adjacent-model aperture/totalize guesses.
- **API 9.3 PASS:** Channel1/2, local gate/channel setters, defaults, fixed operation units and deferred optionalchannel3 are explicit. Exact endpoint commissioning and capability/mode checks are frozen before acquisition.
- **PROTOCOL 9.3 PASS:** CONFIG then immediate-start/timed-stop/gate/ASCII/freshREAD avoids stale fetch and overwritten gate settings. Canonical binary64 millisecond grid, NR3 significant digits and invalid-result/positive-domain rules are specified.
- **PARITY 9.3 PASS:** Both language contracts use equivalent state/error semantics and real native versus worker gating. Actual factory selectors and source/packed NativeAOT Linux/Windowsx64 .NET8/10 plan are explicit.
- **VALIDATION 9.3 PASS:** 50 golden cases exercise channels/units/grid/parser/each failure position. Q-PROC12 requires state-preserving invalid setters, fresh repeated reads and independent handle defaults; common pending/lifetime procedures apply.
- **DELIVERY 9.3 PASS:** External Keysight addition uses existing neutral Counter operations. Pinned compatible bootstrap path, generation/TestKit/package commands, checksum/artifact evidence and future coordinated release are specified without public availability claims.

### 3585B

- **EVIDENCE 9.4 PASS:** Own03585-90017 serial-applicable source is hash-bound with render/OCR provenance. Actual diagrams/commands support fixed fields, dBm, CL/SP and D2/T6. Software1Hz floor is distinguished from hardware0Hz/fractional ability.
- **API 9.3 PASS:** Eight neutral operations and exact nativehpib identity use bounded immutable commissioned endpoints. R12/50ohm/liveA/absolute-dBm policy and unsupported sweep/trace/reset/shutdown are explicit.
- **PROTOCOL 9.3 PASS:** Exact24-character D2 validates both fields including discarded axis. CL-only visibility change, stored-trace SP and selected-marker T6 have honest freshness boundaries. Significant spaces, CRLF/EOI and finite pending bus waits are covered.
- **PARITY 9.3 PASS:** Matching language construction, integer/tenths formatting and fixed-field errors use genuine native/worker modes; external source/packed matrix and compatible coordinated SDK qualification apply.
- **VALIDATION 9.2 PASS:** 123 independent synthetic cases cover all operations/discrete bandwidths, range/grid boundaries, both axes/malformed discarded fields and traffic failures. B-PROC adds actual admission, pendingCL/query, health/ownership and physical-claim documentation.
- **DELIVERY 9.2 PASS:** Native implementation/parser/cases stay in external Keysight, with synthetic fixtures clearly labeled. Source/packed external-tool/TestKit proof and coordinated compatible SDK/public-version distinction are specified; hardware remains future evidence.

### 34980A

- **EVIDENCE 9.3 PASS:** Own mainframe/programmer and module manuals support addressedGPIB, topology and scan-wide effects. PhysicalLAN/USB exist but initial admission is explicitlyGPIB; no DMM/discovery expansion is inferred.
- **API 9.4 PASS:** One factory/physical handle/SDK lease owns all module views. Validated slot/local-derived addresses, namespace uniqueness and cross-slot-type conflict checks freeze per-endpoint capabilities before acquisition; proprietary rules stay external.
- **PROTOCOL 9.3 PASS:** Whole guard/action transactions share parent serialization, health/quarantine and one owned/borrowed lifetime. Lifetime no-autonomous-actions commissioning handles future external triggers that readback cannot exclude; no globalOpenAll/reset/cleanup.
- **PARITY 9.3 PASS:** Five neutral generated operations and corresponding Rust/C# module construction/modes are specified. A new compatible immutable SDK/tool/catalog producer is mandatory; old6524 cannot qualify these modules.
- **VALIDATION 9.3 PASS:** 7 parent cases plus independent module shards and M-PROC01–15 cover shared-view mapping, per-endpoint capability isolation, deep-freeze/slot conflicts, sibling quarantine, one close, pending native IO and noninterleaving.
- **DELIVERY 9.2 PASS:** Core gains only neutral operations/tooling; implementation/addresses/oracles stay external. New immutable producer/checksum-joined source/packed matrix and coordinated future versions are mandatory later evidence, not claimed complete now.

### 34937A

- **EVIDENCE 9.4 PASS:** Own guide printed15/17/20/22 and common ROUT descriptions support28FormC/4FormA, latching/no analog buses and commanded-state queries, with no invented power-fail isolation.
- **API 9.3 PASS:** Three typed single-path operations map local1–32 to the parent. FormC open means NC connection, not circuit isolation; no invented matrix pairs, module session, OpenAll or shutdown.
- **PROTOCOL 9.3 PASS:** Exact addressedROUT writes/query and strict0/1 replies affect only the declared path. Failures quarantine parent; close emits no hardware cleanup. NC consequences and latching state remain explicit.
- **PARITY 9.3 PASS:** Both languages share generated neutral single-channel operations, parent lifetime and native/worker admission. Actual module cases/procedures must run source/packed across native platform matrix.
- **VALIDATION 9.3 PASS:** 32 cases cover FormC/FormA boundaries and each operation, states/parser/IO failures. M-PROC adds every valid local/slot, invalid/duplicate/type conflicts, shared serialization/quarantine and NC/no-cleanup documentation.
- **DELIVERY 9.2 PASS:** Distributed as a documented capability of the existing external mainframe factory, with prerequisites/deferred behavior. New qualified neutral producer and checksum-joined package/native proof precede final implementation/public qualification.

### 34922A

- **EVIDENCE 9.4 PASS:** Own34922A printed34–35 supports two35-channel latching banks, non-scanning multiple closures, scan-only four-wire pairing and power retention; neighboring reed/FET behavior is excluded.
- **API 9.3 PASS:** Direct two-wire single-path operations map local1–70 through one parent. Analog bus/DMM/scan/four-wire/OpenAll remain deferred; partial logical inventories cannot silently expand scope.
- **PROTOCOL 9.3 PASS:** ExactROUT grammar and strict state tokens are specified. Same-bank closes intentionally do not open others; shared quarantine/ownership and no hidden route/bus recovery preserve declared effects.
- **PARITY 9.3 PASS:** Identical generated channel operations and external mappings apply to Rust/C# all frontends, with new compatible SDK producer and source/packed platform qualification.
- **VALIDATION 9.3 PASS:** 33 independent cases exercise1/35/36/70, both state values, malformed/IO responses and sequential same-bank closes without open. M-PROC supplies isolated topology/admission/parent lifetime/pending IO tests.
- **DELIVERY 9.2 PASS:** External parent capability documents signal commoning/nonexclusive closures, power retention and deferred acquisition. Core/vendor boundary and future compatible producer/package/native release plan are explicit.

### 34950A

- **EVIDENCE 9.4 PASS:** Own programmer sections160–185,568–570,596–597,756–774 support numeric width counts, bank-first memory/HIMP, SENSE latch distinction, SOURCE direction effects and fixed electrical policy.
- **API 9.4 PASS:** Neutral positive port/u32 byte requests and zero-valid Count keep topology outside core. Frozen per-port direction/read-write capability gating and0–255 validation are explicit; no hidden setters/group transition.
- **PROTOCOL 9.3 PASS:** All four bank width guards prevent non-anchor bypass, followed by exact direction/polarity/HIMP/memory/electrical guards. SENSE DEC input and SOURCE decimal output stop at first error; strict byte grammar and parent quarantine apply.
- **PARITY 9.3 PASS:** Zero semantics and five neutral operations are generated identically in both languages; real native/worker modes and source/packed AOT matrix require the newly qualified compatible SDK/tool producer.
- **VALIDATION 9.3 PASS:** 123 cases cover banks, selected-versus-first addresses, zero/255/256, direction/guard failures, strict sampled-byte parser and all IO positions. M-PROC adds all local boundaries, grouping/immutability/shared lease and pending IO.
- **DELIVERY 9.2 PASS:** Fixed-byte2.5V/3.3VActive commissioning and lifetime autonomous-action exclusion are documented external prerequisites. Wide/buffer/counter/direction configuration stays deferred; new neutral packages and external implementation require checksum-joined qualification.

### TDS1002

- **EVIDENCE 9.4 PASS:** Exact user-corrected non-BTDS1002 is named in own user manual071-1064-00; qualifying071-1075-04 tables explicitly supportCM/CMA/CMAXGPIB, notMEMGPIB/USB. Probe, state, unit, status and OPC pages directly support the declared subset; later SOURCE[1] clarification is corroboration only.
- **API 9.4 PASS:** Ten existing neutral scope operations use immutable module/probe/commissioning endpoint configuration with1–4096 bounds and selected capability/mode checks. Exact TDS1002/TDS 1002 aliases excludeA/B; probes and scaled nominal grids are model-specific external rules.
- **PROTOCOL 9.4 PASS:** HEAD/VERB global changes and ESR read-clear effects are disclosed;DESE255 prevents masked-error success. Fresh scalarsSTOP/SAMPLE/SEQUENCE/RUN/OPC, verify state/count/source/type/quoted units, parse value and require finalESR0. Sentinel/underflow/domain/error rules and stopped-afterward semantics avoid stale/warning Valid results.
- **PARITY 9.3 PASS:** Corresponding Rust/C# constructors, integer-derived probe scales, literal timebase grids, exact traffic/errors and genuine native/worker gates are specified. External generator/TestKit and compatible immutable-feed source/packed Linux/Windowsx64 .NET8/10 NativeAOT/warning matrix are concrete.
- **VALIDATION 9.4 PASS:** 334 independently authored cases reproduce byte-for-byte with fixture, cover31 timebases,22 fixture scales, all10 supported operations, repeated fresh captures and every measurement IO/guard position. T-PROC01–11 adds full probe/endpoint grids, nonfinite inputs, healthy rejection, real pending prefix/RUN/OPC/VALUE cancellation/deadlines, serialization and ownership.
- **DELIVERY 9.3 PASS:** Implementation/optional-module/probe grammar stays in external Tektronix and preserves oldMDO corpus. Rust/C# composition examples disclose persistent status/mode changes and external connector prerequisites; oldpublicalpha.1 is excluded. Compatible producer/checksum-joined source/packed/native and compiled examples remain required implementation evidence.

## Defects, required changes and limits

**Mandatory defects:** none demonstrated. **Required SPEC changes:** none. **Rubric amendments:** none.

Confidence is high for hashes, manual-backed command/range/token boundaries and oracle reproduction, and moderate-high for planned implementation/platform/package feasibility. This is one complete nine-item SPEC ballot, not a final implementation or physical-validation result.

- SPEC-only judgment of proposed design/manual evidence/executable acceptance plan; no implementation, actual procedural/platform/native/package execution or hardware validation is inferred.
- Installed interfaces/modules/probes, source serial/firmware applicability, electrical conditions and lifetime exclusive commissioning are application-asserted; guards do not prove physical truth/protection/calibration.
- Compatible new immutable SDK/tool producer and generated schema/catalog, actual source/packed consumers, NativeAOT binaries and checksum/log joins remain mandatory implementation evidence for the complete expansion.
- 3585B raw records and full control scripts are synthetic; SP is stored-trace selection and T6 is selected-marker validity, not fresh-global sweep.
- TDS1002 fresh scalar operations intentionally alter acquisition/communication/status state; missing triggers may time out and cancellation can leave acquisition running. No automatic cleanup/recovery is authorized.
- Implementation-source findings and any claimed fixes require their own updated frozen implementation review; they are not final implementation grades or waived by this SPEC ballot.
- No peer scores/ballots read; all54 cells independently reassessed from this fullv2 packet without v1 grade carryforward.
