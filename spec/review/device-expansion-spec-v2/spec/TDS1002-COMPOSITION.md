# TDS1002 composition, procedural acceptance and distribution plan v1

This supplement accompanies `TDS1002.md`. The root architecture remains unchanged: an explicitly registered external Tektronix factory uses neutral SDK requests and application-supplied standard complete-message connectors. No model exception, optional communications-module implementation or waveform parser is added to shared core classes.

## Explicit composition and fixture identity

The application supplies the connector; no factory probes hardware or installs transport providers. Register one `tds1002::Factory` / `Tds1002Factory` with the SDK `DriverRegistry`. Supply exact endpoint bindings and immutable configuration of module type, both probe factors and affirmative qualified commissioning. Multiple bindings may differ in modules/probes, but share no implicit model-wide fallback. A metadata-only match never sends `*IDN?`.

The independent software fixture uses endpoint scheme `scpi`, address `fixture`, module Tds2Cma, channel1 probe X10, channel2 probe X1 and QualifiedGpibNormalModeConfirmed. Its passive manufacturer is TEKTRONIX and model TDS1002. Case IDs begin `tektronix.tds1002|E|`. The fixture means a scripted host, not a claim these modules/probes were observed on a physical bench.

No existing profile or inherited MDO3024 oracle is changed to satisfy this model. Preserve all existing Tek cases and add this separately authored shard to the active corpus. The new factory has ten exact neutral operations; unsupported trigger setters/trace/reset/identity/shutdown are not inferred from the generic class view.

## Procedural acceptance matrix

These requirements complement exact transcript cases; non-JSON values and concurrent/pending behavior require real procedural tests.

| ID | Required proof |
|---|---|
| T-PROC-01 | Exact passive identities TDS1002 and TDS 1002 match, wrong A/B/non1002 models do not; no query/reset/open traffic from metadata or registration. Communication-module Unknown, MEM, undefined C# enums and unknown probes/commissioning reject before acquisition. CM/CMA/CMAX are accepted for GPIB; no direct USB fallback. |
| T-PROC-02 | Empty/blank/wrong-scheme/unbound and normalized duplicate endpoints reject before acquisition; constructor bounds1/4096 valid and4097 invalid. Deep-copy caller collections/configurations and verify a later valid call still uses original probe/endpoint settings. |
| T-PROC-03 | Per-endpoint probe factors remain isolated. Test all1/10/100/1000 factors on both channels, all11 canonical vertical scales and all31 canonical main timebase scales; reject near-neighbor binary64 values before IO. Integer-derived millivolt/probe division and exact literal timebase grids agree in both languages. |
| T-PROC-04 | Zero/negative/NaN/Infinity and off-grid numeric requests, channel0/3 and unknown coupling symbols cause no HEAD/VERBOSE/status/action IO and preserve health. A subsequent valid operation succeeds. |
| T-PROC-05 | Every operation emits the exact HEAD OFF/VERB ON/DESE255/pre-ESR0 prefix and a final ESR0 check. Valid nonzero ESR, wrong DESE, malformed status bodies and a finite measurement followed by warning ESR16 fail explicitly and quarantine. Never emit *CLS or a DESE setter/drain. |
| T-PROC-06 | Fresh Vpp/frequency use the requested channel/probe, MAIN/YT/NORMAL/visible guards, STOP/SAMPLE/SEQUENCE/RUN, OPC completion, stopped state and acquisition-count checks. Repeat calls each issue RUN and completion checks; no cached VALUE or previous Single acquisition is substituted. |
| T-PROC-07 | Trigger View ignored-SET event221, AUTO/Scan/XY/hidden inputs, wrong full tokens/probes/units, invalid-result9.9e37 and finite warning outcomes cannot produce a Valid measurement. Exercise both first failure and a follow-up SessionNeedsRecovery with zero extra traffic. |
| T-PROC-08 | Genuine native-only hosts reject synchronous connector/open/write/query use. Test independent deadlines and cancellation during a pending prefix write, sequence RUN write, OPC wait and VALUE query, and ensure no host accesses continue after cancellation completes. Cancellation may leave the device acquiring; no implicit STOP/clear cleanup is emitted. |
| T-PROC-09 | SDK worker modes use one serialized owner thread and preserve cancellation/cleanup contracts. Whole multi-message operations from two class views cannot interleave. Owned IO closes once, including partial-open/failure/drop; borrowed IO stays open. Close emits no device traffic or inferred safe shutdown. |
| T-PROC-10 | Required native/unsupported operation checks occur on the selected endpoint before acquisition. Ten advertised operations correspond exactly to backend support; trace/trigger/reset/identity/shutdown are absent. Run/Single are start-only; fresh scalar calls end stopped in SAMPLE/SEQUENCE and preserve trigger source/level/slope. |
| T-PROC-11 | Unit/string/numeric parsers are invariant culture and ASCII-only. Sample Vpp zero is valid, negative Vpp and nonpositive frequency are Device, positive9.9e37 is Device rather than Overload, other giant values/NaN/headers/blocks/CSV are Parse. Status read-clear behavior and no physical-calibration guarantee are documented in executable examples. |

## Executable validation and evidence

Use the same external generator/TestKit as the existing Tek package. No production assembly/package should embed the JSON oracle or load factories through reflection. Run all unaffected old cases plus the new independent TDS1002 shard, using Rust default and async features and C# sync/native/worker paths.

The final gate builds and executes generated-source and packed release-form consumers on Linux x64 and Windows x64. Publish and run .NET Native AOT for net8.0 and net10.0, reject unexpected IL2026/IL3050/trimming warnings and verify the intentional warning probe fails as designed. Preserve native binary formats/hashes, case counts, procedural outcomes, phase exits/log hashes and SDK producer/consumer checksum joins. An IL run alone is not Native AOT evidence.

Pin the compatible SDK/tool producer to an immutable commit. The current expansion's94-operation SDK is suitable if its reviewed implementation/API remains unchanged; an old public package cannot be called compatible merely because its version string matches a development feed. Public alpha.1 lacks this factory. A coordinated new package version and release qualification are separate from prepublication development-feed proof.

The Council independently reviews this specification/oracle and the eventual implementation/evidence. Physical model/module/firmware/probe configuration, trigger timing, measurement performance and raw bench traces remain future hardware evidence. No merge, publication, live instrument access or repository creation is included.
