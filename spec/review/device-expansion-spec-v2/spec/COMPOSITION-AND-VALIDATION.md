# First three models: construction and executable delivery plan

This normative supplement applies to 34401A,33120A and53132A specifications. It is a concrete implementation plan, not executed implementation or hardware evidence. No new core operations are needed for these three items. Later module contracts require their own reviewed neutral API and producer revision.

## Public construction contract

Rust exposes `qualified_scpi::Factory::new(Model, Vec<Binding>, Arc<dyn Connector>) -> Result<Factory>` and optional `with_native_connector(Arc<dyn AsyncConnector>)`. C# exposes `QualifiedScpiFactory(QualifiedScpiModel, IEnumerable<QualifiedScpiBinding>, IScpiConnector, IAsyncScpiConnector? nativeConnector = null)`.

Model values are `Agilent34401A`, `Agilent33120A`, `Agilent53132A`. Each binding contains an exact `Endpoint` and an explicit commissioning enum. `Unknown` is invalid. DMM/counter require `GpibScpiConfirmed`; the generator requires `ContinuousSine50OhmVppOutputChangesAuthorized`. The latter affirms physical load, exclusive use and authorization to change the always-connected output. These assertions are not hardware detection. Live guard queries still verify the generator settings before every operation as its specification requires.

Factories accept 1–4096 bindings, copy/freeze caller input and reject unknown enum values, incompatible model/commissioning, empty addresses, other endpoint schemes, and duplicate endpoints before acquisition. Endpoint scheme must be `scpi` after ASCII case-insensitive trim; address comparison is exact. An unbound endpoint fails before acquisition. Required operations are checked on the selected binding before acquisition; true native mode requires a genuine native connector. No metadata, registration or opening sends device traffic. Each opened handle has independent local channel/gate intent. All class views of that handle share the SDK serialization/lifetime boundary.

Canonical IDs and manufacturer rules are specified per model. Remove the old 34401A production profile and USB rule, retaining its 69 old cases and profile as historical provenance. It cannot coexist with its replacement under equal matching rules. The generic `Drivers.Factories` list therefore ceases to offer unqualified 34401A support; applications explicitly register its commissioned factory. This is an intentional allowed API change. The old public package remains immutable and does not gain new support.

Example construction in Rust, with an application-supplied finite, bounded, LF/plain-text GPIB connector:

```rust,ignore
let factory = qualified_scpi::Factory::new(
    qualified_scpi::Model::Agilent34401A,
    vec![qualified_scpi::Binding {
        endpoint: Endpoint { scheme: "scpi".into(), address: "GPIB0::22::INSTR".into() },
        commissioning: qualified_scpi::Commissioning::GpibScpiConfirmed,
    }], connector)?;
// Register Arc::new(factory) in DriverRegistry; open this exact endpoint;
// invoke catalog::Dmm(&handle).measure_voltage_dc(None).
```

The C# constructor uses the same model, endpoint and commissioning records. Both typed Dmm, FunctionGenerator and Counter views use the existing SDK. Close releases resources only; no output-disable promise is made for the generator.

```csharp
var factory = new QualifiedScpiFactory(QualifiedScpiModel.Agilent34401A,
    [new(new Endpoint("scpi", "GPIB0::22::INSTR"),
        QualifiedScpiCommissioning.GpibScpiConfirmed)], connector);
// Register factory in DriverRegistry, open the exact endpoint, then:
// new Dmm(handle).MeasureVoltageDc(null).
```

## Oracle integration and local proof

The independently authored three shards contain 69/66/50 cases before later corrections. Their authoring generator imports no production plans. Preserve their bytes and provenance; do not synthesize expected traffic from backend code. Extend `spec/acceptance/compose.py` with the three canonical IDs, remove the retired 34401A cases from the active corpus and append the reviewed replacement shards. Preserve all unaffected baseline cases. Extend the Rust/C# fixture selectors to construct these commissioned factories instead of generic profiles.

Use the external tool, setting `PYTHONPATH` to the qualified SDK tool source during development:

```sh
python spec/acceptance/compose.py --check
python -m instrument_driver_tools.cli generate --input profiles.json --output-dir generated --contract-major 1 --check
python -m instrument_driver_tools.cli generate-tests --input cases.json --output-dir generated --check
```

Regenerate before the drift checks when inputs change. Add a procedural coverage map naming both language tests for passive selection/open, no USB identity, immutable 1–4096 endpoint admission, selected-route requirements, invalid requests followed by a usable valid operation, post-IO conflict/quarantine, owned/borrowed close, absent-native pre-open gating, genuine pending read/write deadlines and active native cancellation. Nonfinite numeric arguments require procedural cases because the JSON oracle deliberately rejects NaN/Infinity. Read-only recovery must not invent bus commands. Extend all model loops to instantiate each actual new factory, rather than inferring model coverage from shared helpers.

The isolated source consumer prepared by `prepare-source-proof.py` uses explicit project/path references for iteration only. Execute Rust default and async, clippy with warnings denied, and the C# four-frontend runner. Keep its logs and source hashes, labeled source integration rather than public-package proof.

## Procedural acceptance matrix

Implement the following stable IDs in `validation/rust/src/qualified_proof.rs` and `validation/dotnet/QualifiedProof.cs`; invoke them from the existing proof entry points before the generated corpus. Each applicable scenario instantiates each actual model factory, with its own normal operation and manual-derived script. DMM's normal operation is auto DC voltage; generator's is a guarded frequency write or read; counter's is a fresh frequency query. No scenario passes solely because an older HP/SDK helper did.

| ID | Setup and trigger | Required assertions |
|---|---|---|
| Q-PROC-01 | Register each exact manufacturer/model rule; inspect metadata, open and close an empty-script owned connection. | Selection, descriptor and open emit no device commands; canonical ID/operations exact; one acquisition/close. No 34401A USB product rule or temperature capability; no generator OutputEnable/Shutdown. |
| Q-PROC-02 | Construct Unknown or wrong-model commissioning, empty/duplicate/unsupported-scheme bindings; open an unbound endpoint. | Typed InvalidArgument before connector acquisition; zero opens/writes/queries. |
| Q-PROC-03 | Construct with caller-owned binding array, mutate/clear it, then open the original exact endpoint and execute a valid operation. | Original frozen binding succeeds with exact traffic; a newly injected endpoint remains unbound. Address case remains exact, scheme ASCII case-insensitive. |
| Q-PROC-04 | Construct 4096 distinct valid endpoints; separately try 4097; require a capability unavailable on this model or request absent native mode. | 4096 accepted, 4097 rejected before IO; unsupported operation/mode fails before acquisition and never uses another route/factory's capability. |
| Q-PROC-05 | For every numeric setter/range, pass NaN, positive/negative infinity and declared finite invalid boundaries; then invoke a normal valid operation on the same handle. | Structural InvalidArgument emits no traffic, leaves prior state and health intact; the valid operation succeeds and consumes only its expected script. |
| Q-PROC-06 | For generator amplitude/offset, provide valid guard replies and a valid counterpart incompatible with the requested pair (including E11-rounded conflict). | InvalidArgument after IO, no final write; follow-up operation is SessionNeedsRecovery with no traffic. Wrong guard state is Device; malformed guard is Parse. Include exact HighZ load 9.9E37. |
| Q-PROC-07 | Inject transport/timeout at each transaction position; separately return malformed and valid-but-wrong replies. | First typed error is preserved, later messages are absent, follow-up is SessionNeedsRecovery, no retry/drain/reset/recovery traffic. Owned cleanup happens once. |
| Q-PROC-08 | Open owned and borrowed versions, close twice and invoke again. | Owned host closes once, borrowed host never closes, both handles reject later calls as SessionClosed; no output/relay/device cleanup command. |
| Q-PROC-09 | Through a true-native-only host, suspend the model's real query until a start signal is observed; cancel/drop the invocation after suspension. | No sync host method used; active host access ends before completion/drop; follow-up is SessionNeedsRecovery, no detached traffic and no implicit abort; resource cleanup respects ownership. |
| Q-PROC-10 | Keep a query pending with no device progress; host enforces an independent finite timer. For 33120A and53132A also suspend a real write. | Deadline completes with Timeout independently of any response, host access has ended, no later transaction step occurs and follow-up quarantines. Immediate injected Timeout is not this proof. |
| Q-PROC-11 | Start two whole operations on one native handle; pause the first query/guard while the second task waits, then release it. | The second operation's first IO occurs only after the first's complete transaction; guard/write groups cannot interleave. Worker path retains its strict thread-affinity assertion and same operation ordering. |
| Q-PROC-12 | Counter: select channel2 and gate0.001, attempt invalid channel/gate, then measure frequency/period repeatedly; separately open a second handle. | Invalid setters preserve channel/gate; first handle reapplies2/0.001 after every CONF; second starts1/0.100; repeated READ is fresh and units match. |

Run the ordinary corpus in all four frontends (sync, native async, worker async, worker blocking) in both languages. The targeted pending/cancellation tests specifically use genuine native paths and bounded blocking hosts where applicable. Existing SDK worker partial-open/thread ownership proofs remain regression evidence, while the new factory integration scenarios above are required in addition. Source and packed consumers execute these procedural entry points as well as the corpus; failures propagate and no tests are filtered from native publication.

## Packed/native CI gate

The existing Keysight CI at baseline `c63cc025d283deb0832dbe2b9c721d8a33412160` pins both reusable SDK production and package-bootstrap workflows to main `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`. That producer has all neutral operations needed for these three devices. Current main `1b0704dbb2c4a9f4ffda9f6b1665de12654f672d` adds only the separately tested standard VISA ownership guard relative to that SDK. Retain this compatible immutable pin for a first-three-only proof. If later neutral module additions are included, replace both workflow/source pins with the same new qualified immutable SDK commit, regenerate its tool catalog/schema and revalidate all consumers; an older catalog is not acceptable for new operations.

The existing workflows define four native cells: Ubuntu/Windows x64 × .NET8/.NET10. Each cell prepares checksum-verified fresh SDK feeds and invokes:

```sh
instrument-driver prepare-feeds --artifacts <downloaded-sdk-artifacts> --output <fresh-feeds>
instrument-driver verify-package --repository <vendor-checkout> --feeds <fresh-feeds> --output <empty-proof-dir> --framework <net8.0-or-net10.0>
```

The runner checks generation, creates release-form registry-only crates/NuGet packages, runs generated-source and packed Rust default/async consumers and all four C# frontends, publishes/executes actual Native AOT binaries and requires an intentional IL2026/IL3050 probe to fail. Record actual source revisions, package checksums, execution logs and binary hashes/formats; inspect actual downloaded artifacts and producer/consumer checksum joins. No source/path dependency may leak into packed proof. Core packages must not depend on or contain vendor implementations. The large fixture-only opt0/32MiB harness policy is retained and disclosed; driver/SDK dependency optimization remains unchanged. OpenTAP's managed host is not thereby AOT-qualified.

Public 0.2.0-alpha.1 lacks these additions and contains the old unqualified 34401A profile. Bootstrap feeds with that development version prove consumption of exact audited bytes, not public availability. Before a future authorized publication, choose new coordinated SDK/driver versions, update exact dependencies and release metadata, run the public-package/native matrix against those versions, and retain immutable release evidence. This task does not publish, merge, open hardware or claim bench qualification.
