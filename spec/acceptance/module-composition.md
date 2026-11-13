# 34980A module composition and qualification supplement v1

This supplement is normative alongside `34980A.md` and the three module specs. The first-three-device composition/platform plan prepared by the facilitator also applies to these factories; this document adds module-specific requirements.

## Build and distribution boundary

Add only five vendor-neutral operations to the main SDK: SwitchCloseChannel, SwitchOpenChannel, SwitchChannelIsClosed, DigitalIoReadByte and DigitalIoWriteByte. Switch parameters are positive logical channels. Digital ports are positive; write values permit 0 through255, and Count results permit zero. Do not reuse a generator's unconditional positive-count check for byte values. Existing positive-count operations retain their old constraints.

Generate typed Rust/C# sync/async classes and operation metadata from the same neutral schema. Generate corresponding OpenTAP views/steps only through the existing neutral tooling, preserving the old migration ledger and adding fresh execution evidence rather than rewriting historical results. Core source/build/artifacts depend on no external module implementation. External native code and manual-derived cases remain in the Keysight package; acceptance fixtures/specs are not runtime dependencies or production package payload.

The old immutable SDK producer pin does not contain these five operations. Before a package proof claims success, commit the neutral changes and pin a compatible immutable SDK source/workflow revision, or use an explicit verified local development feed for interim tests. Final qualification must include actual source/generated and packed consumers from checksum-joined compatible SDK feeds, not a newer source tree combined with older public alpha.1 binaries. Public versions still require coordinated new SDK/driver versions before release. No publication or merge is implied.

## Procedural acceptance inventory

These IDs supplement the independent transcript cases. They are acceptance requirements, not assertions that implementation already exists.

| ID | Required proof |
|---|---|
| M-PROC-01 | Constructors validate all slot/local boundaries: each valid34937local1..32,34922local1..70,34950local101..104/201..204; slot1..8; reject0/9, unsupported locals and all unknown enum values. |
| M-PROC-02 | Isolate duplicate logical IDs, duplicate physical paths, same-slot conflicting switch types and cross-switch/digital slot conflicts. Test each with otherwise-valid inventory. |
| M-PROC-03 | Deep-freeze caller lists; mutation after construction cannot alter admission, native addressing, directions or a successful valid operation. |
| M-PROC-04 | Multiple endpoints have independent inventory/capability sets; unbound endpoint and wrong per-endpoint native/worker requirements reject before connector open. |
| M-PROC-05 | Required read/write/switch capabilities and wrong selected port direction reject before acquisition or traffic as appropriate; sparse logical IDs never become native addresses. |
| M-PROC-06 | Two class/module views from one handle share one parent session; an operation with many guard queries cannot interleave with relay/digital operations on another view. |
| M-PROC-07 | Every successful or failed handle closes owned parent IO exactly once; borrowed IO stays open; child views never open/dispose an extra connection. |
| M-PROC-08 | Any uncertain IO/parse/Device/cancellation failure quarantines all module views. A follow-up on another module produces SessionNeedsRecovery with zero more traffic. |
| M-PROC-09 | 34937FormC open is documented NC selection; no automatic open/reset/cleanup occurs on creation or close. 34922same-bank closures do not silently open another path. |
| M-PROC-10 | All four width queries precede selected34950byte access; non-anchor selected ports cannot bypass a WORD/LWOR anchor. Bank-scoped guards address101/201, never the selected102/104/202/204. |
| M-PROC-11 | Native async fixtures reject synchronous open/read/write calls. Active pending reads and writes exercise independent host timeout and cancellation, not only injected Timeout errors. No host access continues after native cancellation completes. |
| M-PROC-12 | Dedicated-worker frontends retain SDK serialization, owner-thread cleanup and cancellation behavior; source/packed proof executes the same immutable cases with no reflection-based registration. |
| M-PROC-13 | Invalid byte256 and structural inputs produce no guard traffic and preserve health; zero writes/reads succeed. NaN/Infinity in any numeric construction policy is rejected if a future configurable policy is added. |
| M-PROC-14 | Unknown commissioning rejects before acquisition. The affirmative condition explicitly covers no scan/monitor/route sequence, no autonomous external-trigger/interrupt/pattern/buffer action, fixed normal byte electrical policy and exclusive stable control for the entire handle lifetime. |
| M-PROC-15 | Construction limits are tested at1/4096 endpoints and4097 rejection; normalized duplicate endpoints fail; empty usable inventory rejects. Large valid inventories remain bounded by physical slot/local uniqueness. |

## Executable matrix and evidence

Run the complete unaffected driver corpus plus new module cases and all applicable procedures in both Rust/C#, sync/native/worker frontends. Execute generated-source and packed release-form consumers on Linux x64 and Windows x64; publish and run .NET Native AOT for net8.0 and net10.0 with trimming/AOT warnings as errors, plus the intentional warning-probe failure gate. Preserve native binary hashes/formats, phase exits/log hashes and SDK producer/consumer checksums. A successful managed test is not Native AOT evidence. Any fixture-only build-profile adjustments remain explicit and cannot remove cases/assertions or replace production optimization claims.

The exact manufacturer/model/firmware/installed hardware and physical bench traces remain future evidence. Software guard success does not establish electrical protection, contact continuity, isolation or calibrated pin levels.
