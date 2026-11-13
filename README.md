# Instrument Components Keysight drivers

Independently packaged Rust (`instrument-driver-keysight`) and .NET (`InstrumentComponents.Drivers.Keysight`)
source-derived profiles, version `0.2.0-alpha.1`. These are published previews;
no devices in this library have physical bench validation yet.

## Install and try the preview

```sh
cargo add instrument-driver-keysight@=0.2.0-alpha.1
dotnet add package InstrumentComponents.Drivers.Keysight --version 0.2.0-alpha.1
```

Use the [complete hardware-free Rust/C# guide](https://github.com/josh-hemphill/instrument-components/blob/72015932f9b168ac7d182fcb5ca7a367318505b8/docs-site/docs/getting-started.md) before connecting equipment. It uses exact public package versions and an in-memory development connector. Select this family's factories explicitly; adding a package never probes or opens hardware. The [published source tag](https://github.com/josh-hemphill/instrument-components-keysight/tree/driver-v0.2.0-alpha.1) contains this release; the default branch contains the merged SDK extraction. New HP additions described below are unreleased source work.


Models and aliases are passive hints carried forward from main commit
`5eb92b17f775d326e5a3542ee40855b156321745`: 33220A, 33522B, 33622A, 34401A, 34410A, 34461A, 34465A, 34970A, 34972A, 53131A, 53220A, 53230A, DSOX2024A, DSOX3024T, E36312A, E3631A, E3640A, N6705C, N9010B, N9020B, N9030B, U2000A, U2001A, U2002A, U2004A.
See `provenance/identities.json` for every manufacturer alias and the source
resolution, and `provenance/source.json` for frozen source hashes. A hint is not
a claim that every listed model implements these commands. Suspicious inherited
identities, including Keithley's E36312A hint, are preserved as unvalidated data.

Applications explicitly register `sync_factories` / `native_factories` in Rust,
or `Drivers.Factories` / `Drivers.NativeFactories` in C#. Connector ownership,
serialization, cancellation, recovery and class views use the public SDK. No
runtime assembly loading, reflection activation, or core registry installation
is required. Multiple class views share the same opened handle. Factories have
one canonical identity for equivalent aliases; the 2602B is one DMM+PSU factory.

Supported channel IDs are inherited hints, not complete physical inventories.
Supply and generator outputs are unknown by default: metadata channel count and
checked shutdown are unavailable. Pass immutable `InventoryBinding` values keyed
by driver ID before opening a factory to declare the actual complete inventory.
Supply bindings can use sparse channel IDs. The modeled generator has one
unchanneled output; it cannot safely represent multi-output shutdown. An unknown
or multi-output setup must not claim checked shutdown from this model.

Shutdown disables every declared output and executes class-specific stop,
open-all or continuous-off actions, attempting all actions despite failures.
It never implies reset. DMM/counter/power-meter output-off is a no-op for their
modeled outputs. Temperature results default to Unknown physical units, also
through Read/Fetch; no Celsius assumption or unit configuration command is sent.
Generic remote-sense enable/disable maps Internal to OFF and External to ON,
following the legacy source documentation. N6705 sense remains unsupported.
DMM6500 ranged measurements explicitly preserve the old generic wire commands;
this source-derived behavior still requires device evidence.

`profiles.json` holds the generated SCPI wire plans; `src/hp5334.rs` and `Hp5334.cs` implement native HP counter behavior separately. `cases.json` is a separately
specified golden traffic/value corpus; its generation never reads production
plans. Generated Rust/C# files contain static data and require no runtime JSON.
`provenance/operation-coverage.json` joins behavior to the shared case IDs.
`provenance/source/` is historical audit evidence, excluded from production
packages. The SDK TestKit and Tokio consumers are development-only dependencies.

The public 0.2.0-alpha.1 SDK and driver packages are available on crates.io and NuGet. The release workflow tests fresh public-package consumers on Linux/Windows x64 and .NET 8/10.

For pre-publication development, CI also uses checksummed release-form SDK archives from
a pinned main workflow. The consumer job checks out only this repository and
resolves exact versioned SDK dependencies from fresh verified development feeds.
This is explicitly bootstrap validation, not public registry availability.
`instrument-driver verify-package` builds generated-source and packed consumers,
runs Rust default and async modes, and publishes and executes .NET Native AOT
with all four frontends on Linux/Windows x64 and net8.0/net10.0. It also requires
the intentional trimming-warning probe to fail. No hardware access occurs.

Core applications that do not select this package need none of its artifacts.
Release tags are immutable. A subsequent release needs a new version, matching SDK pins, passing native gates and council review.

## Connection-specific inventories

Use a `ConnectionBinding` containing the canonical driver ID, exact endpoint and inventory. C# accepts `connections:` in `Drivers.Factories`/`NativeFactories`. Rust exports `sync_factories_for_connections`, `native_factories_for_connections` and `combined_factories_for_connections`. All bindings are frozen and validated before IO. Two endpoints may use the same driver with different inventories. Unknown output inventory never inherits another endpoint's complete output list. Model-wide defaults remain available separately. Unknown driver IDs and normalized duplicate endpoints fail explicitly.

The committed exact wire cases now execute through endpoint bindings, including sparse channel IDs, per-operation transport failures and closed-handle checks. Source evidence remains distinct from physical evidence. All model hints in this repository are in initial scope; physical validation is pending.


## Native HP additions (unreleased)

Published `0.2.0-alpha.1` does **not** contain these new HP backends or the added DAQ/RF SDK contracts. This branch implements manual-derived subsets for HP5334A/B, HP3497A, HP8920A/B and HP8921A; the preceding HP review record is retained under `spec/review`; this expansion requires its own final implementation and distribution review. All remain hardware-unvalidated. No new repository is needed: HP, Agilent and Keysight implementations share this external package while core contains only neutral contracts and standard tooling. A release requires a new coordinated SDK/driver version; do not mix these sources with the existing public SDK.

The counters support frequency on A/B, period on A, gate time 0.001–99.999 s, logical channel selection and explicit initialization. Optional C, totalize and the remaining modes are not advertised. Each model has a distinct factory ID. Do not register these through generic SCPI fallback or infer model identity from the ambiguous native ID response. See [5334A](spec/legacy/HP5334A.md) and [5334B](spec/legacy/HP5334B.md) for exact commands, state changes and limits.

Before using either factory, commission an exclusively owned addressed REMOTE connection in native language, ordinary internal gating, external arming and 100-gate averaging disabled, appropriate input conditioning, and a clean message boundary. In particular, Option 700 on the B may start in CIIL; select native mode explicitly outside the library. `Unknown`/`Ciil` commissioning is rejected before connector acquisition. Supply a complete-message connector with finite operation deadlines (for example 2 s, increased for long gates), bounded response allocation (recommended 256 bytes), and CRLF handling that preserves the native record body. A timeout shorter than the selected gate can fail normally; no retries/reset are inferred. A native connector must end host accesses before cancellation completes; use the dedicated worker for blocking adapters.

Rust static composition (the application supplies `connector: Arc<dyn instrument_scpi::Connector>`):

```rust,ignore
use instrument_api::{catalog::Counter, *};
use instrument_driver_keysight::hp5334::{Commissioning, Factory, Model};
use instrument_runtime::{DriverRegistry, Selection};
use std::{collections::BTreeSet, sync::Arc};
let factory = Factory::new(Model::Hp5334A,
    Commissioning::NativeInternalGatingConfirmed, connector)?;
let registry = DriverRegistry::new(vec![Arc::new(factory) as Arc<dyn DriverFactory>])?;
let candidate = Candidate {
    endpoint: Endpoint { scheme: "hpib".into(), address: "GPIB0::3::INSTR".into() },
    identity: Identity { manufacturer: Some("HP".into()), model: Some("5334A".into()), ..Identity::default() },
};
let handle = registry.open(&candidate, &Selection::default(), &BTreeSet::new(),
    ExecutionMode::Sync, ConnectionOwnership::Owned)?;
Counter(&handle).set_gate_time(0.01)?;
Counter(&handle).select_channel(2)?; // B frequency; period is A-only
let frequency = Counter(&handle).measure_frequency()?;
handle.close(false)?; // resource release; reference/clock outputs are not disabled
```

C# static composition (the application supplies a commissioned `IScpiConnector connector`; despite that historical interface name, Query sends the documented native command body and receives one message):

```csharp
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Keysight;
using InstrumentComponents.Runtime;
var factory = new Hp5334Factory(Hp5334Model.Hp5334B,
    Hp5334Commissioning.NativeInternalGatingConfirmed, connector);
var registry = new DriverRegistry([factory]);
using var handle = registry.Open(new(new("hpib", "GPIB0::3::INSTR"),
    new InstrumentComponents.Drivers.DeviceIdentity("HP", "5334B")));
var counter = new Counter(handle);
counter.SetGateTime(0.01);
counter.SelectChannel(1);
var period = counter.MeasurePeriod();
handle.Close(); // resource-only; SafeShutdown is deliberately unavailable
```

Attach a true native connector using Rust `with_native_connector` or C# `nativeConnector`; otherwise native async is not advertised. The same factory supports the SDK dedicated-worker path for sync connectors. Factory matching and opening send no ID/reset/enable commands. Whole operations are serialized, invalid inputs are rejected before IO, and timeout/transport/parser failures quarantine the session. Recovery is unsupported for a quarantined counter: explicitly resynchronize the hardware/bus outside this session, then reopen. Healthy-session recovery follows the SDK's no-op behavior. Close never emits IN/TE/GAL or bus-local commands and preserves borrowed connections.

`cases.json` contains separate A/B manual-derived wire/quantity/parser cases, generated with the same external SDK tool as the existing profiles. [Counter case provenance](provenance/hp-counter-cases.json) records manual pages; synthetic parser/API rejection cases are software policies, not observed hardware transcripts. Conformance runners exercise sync, native and both worker frontends; additional checks cover commissioning, passive selection, ownership, quarantine and cancellation. CI builds and executes generated/source and packed Rust/.NET consumers, including actual linux-x64/win-x64 Native AOT binaries; a local IL run alone is not AOT evidence. Physical bench evidence is still pending.

### HP3497A acquisition

Register `hp3497::Factory` / `Hp3497Factory` explicitly. Bind each endpoint to Option001 presence, confirmed normal DVM routing, and logical-channel mappings (`channel`, `slot`, `local`, module, compensation). Option010 accepts local0–19 without thermocouple compensation; Option020 accepts0–18 with software compensation, reserving19 for reference. Slots0–4 map to native20×slot+local. Unknown, conflicting or duplicate inventory rejects before acquisition. The factory copies/freezes the complete inventory. Only `DataAcquisition.MeasureVoltageDc(channel, range)` is exposed, with nominal ranges0.1/1/10/100V or auto. This is one scalar reading, not a scan or a generic DMM/switch implementation. [Specification](spec/legacy/HP3497A.md).

### HP8920A/B and HP8921A RF

Register one `hp_radio::Factory` / `HpRadioFactory` for each model. Provide a `Binding` / `HpRadioBinding` per exact endpoint with its model, firmware, attenuator, input-power option, source port, receiver input and affirmative commissioning. Minimum firmware is respectively `A.18.00`, `B.06.00`, `A.15.00`, with exact `A.dd.dd`/`B.dd.dd` syntax; later revisions in that family are accepted. Source/receiver ports are independent immutable enum choices, with no fallback for an unbound endpoint.

Commission native GPIB, initial Manual RX/TX switching, independent frequency display, inhibited external PTT/control, exclusive access, a clean bounded response stream and suitable input carrier before first screen change. Each model specification contains its option/port frequency and level tables and physical prerequisites: [8920A](spec/legacy/HP8920A.md), [8920B](spec/legacy/HP8920B.md), [8921A](spec/legacy/HP8921A.md). HP8920B Mechanical055 cannot combine with006/007; HP8921A accepts Standard input power only. Software ranges do not provide physical input protection.

`RfSignalGenerator.Configure(frequencyHz, levelDbm)` validates integerHz/decimal tenths, disables the managed RFG output, normalizes settings and verifies all readbacks. It leaves output disabled; `OutputEnable(true)` is a separate explicit operation permitted only after successful configure. Modulation is preserved, so configuration does not promise unmodulated CW. `OutputStateQuery()` reads the managed state. `RfReceiver.MeasureFrequency()` and RFInOut-only `MeasurePower()` disable that output and execute a fresh FULL/SING trigger transaction on every call. Receiver operations invalidate source readiness. ANT routes never gain power capability from another endpoint. No general Reset, Shutdown, identity probe, call-processing or AF support is advertised. Resource close is not RF shutdown.

Both families use sync, dedicated-worker and optional true native async connectors with identical transactions and fail-fast quarantine. Require finite positive host deadlines, a256-byte response allocation bound and complete ASCII bodies with framing removed but payload whitespace preserved. Binary blocks must be rejected before any header can be removed. Standard .NET VISA composition must select `endpointScheme: "hpib"`, `maxResponseBytes: 256`, `terminator: "\r\n"`, `allowBinaryBlocks: false`; its one-shot body API does not retry or drain on timeout. Custom Rust/.NET connectors must provide the same contract; Rust `query_body` is plain-text only. Bus resynchronization (including documented selected-device-clear before RF measurement abort), recommissioning and reopen are external responsibilities.

The [independent acceptance shards](spec/acceptance) add481 cases (53DAQ,1528920A,1488920B,1288921A) to the112 counter and831 previous cases. Run `python spec/acceptance/compose.py --check` before the SDK's `generate-tests --check`. Expected traffic is extracted from manuals and reviewed independently of backend code; provenance, synthetic policies, option exclusions and frozen cross-language power constants are retained beside the shards. [Specification Council record](spec/review/recovered-spec-v9/RESULT.md) is separate from the required final implementation review.


## Agilent scalar and module expansion (unreleased)

This branch adds manual-derived, hardware-unvalidated subsets for **34401A,33120A,53132A,3585B and34980A with34937A/34922A/34950A**. These implementations are absent from public0.2.0-alpha.1. A coordinated new SDK/device version and passing distribution review are required before release. Register the new factories explicitly; inherited passive hints and generic profiles do not qualify these models. The old generic34401A profile has been retired to provenance and replaced with the explicit factory.

| Device | Explicit factory | Supported subset / prerequisites |
|---|---|---|
| 34401A | Rust `qualified_scpi::Factory` / C# `QualifiedScpiFactory`, model `Agilent34401A` | Seven DC/AC voltage/current and2/4-wire resistance scalar methods; fresh MEAS, strict fixed response grammar, documented positive overload. GPIB SCPI commissioning; no temperature, Read/Fetch, USB hint or implicit reset. |
| 33120A | Same, model `Agilent33120A` | Set frequency/amplitude/offset and read frequency, continuous sine/50ohm/Vpp only. Guards verify mode and coupled voltage bounds before changes. **The instrument has no output-off relay; setting changes affect the live output.** Affirm explicit authorization in commissioning. No OutputEnable or safe-shutdown capability. |
| 53132A | Same, model `Agilent53132A` | Channel1/2 frequency/period; local channel and integer-millisecond gate intent0.001–1000s reapplied after each CONF. Every measurement is fresh READ. GPIB SCPI commissioning; no totalize/channel3. |
| 34980A parent | `hp34980::Factory` / `Hp34980Factory` | One exact-endpoint frozen inventory and one resource/lease for all modules. No scanning/monitoring/autonomous action, internal DMM or independent child factories. Commission exclusive stable control and no autonomous activity. |
| 34937A /34922A | Parent switch inventory | Explicit logical path ->slot1..8/local mapping, close/open/query. 34937 locals1..28 are FormC: **open selects NC, rather than isolating common**;29..32 FormA. 34922 supports1..70; multiple same-bank closures do not imply exclusive routing. No automatic open-all. |
| 34950A | Parent digital-port inventory | Static per-port input/output direction; byte0..255. Verify all four bank widths, direction, polarity, memory/handshake and fixed electrical policy before access. Input threshold2.5V; output active drive3.3V. Never silently change width, direction or electrical settings. No buffered transfer/pattern/interrupt API. |
| 3585B | `hp3585::Factory` / `Hp3585Factory` | Native hpib scalar center/span/RBW/VBW/reference and marker peak/X/Y; qualified serial3008A00860+, R12/+30dBm50ohm input, liveA/dBm. Peak selects stored trace; X/Y refresh only the selected point. No full fresh sweep/trace, implicit reset or mode normalization. |

Read the own-model [specifications](spec/legacy) before affirming commissioning. They define exact bounds, framing, persistent state changes and deferred operations. Logical channel/port IDs are application names, never inferred native addresses. Do not register overlapping unqualified factories for these models.

The compiled, hardware-free composition examples show explicit constructors, exact bindings, typed calls and one shared module parent: [Rust](validation/rust/src/qualified_examples.rs) and [C#](validation/dotnet/QualifiedExamples.cs). Applications replace TestKit with their commissioned connector. Attach a genuine native connector via Rust `with_native_connector` / C# `nativeConnector`; otherwise choose the SDK worker for blocking IO. Registration/open send no discovery/reset/device commands. A native provider cannot be a synchronous wrapper.

Require finite host deadlines,256-byte bounded scalar allocation and complete plain-text bodies. For these GPIB SCPI models select LF/EOI framing;3585B uses its native framing described in its specification. Reject binary wrappers before removing headers. Keep payload whitespace for strict parsing. Any uncertain IO/parse/Device failure or active cancellation quarantines the entire handle; externally resynchronize/recommission and reopen. Close is resource-only and preserves borrowed IO.

The [procedural coverage map](validation/agilent-procedural-coverage.md) joins the independent requirements to executable tests. New504 manual/policy-derived cases plus1355 unaffected cases give1859 active transcripts. Spec Council grades and eventual implementation grades are distinct; hardware evidence remains absent. Neither a passing simulator nor Native AOT proves measurement performance, physical contact state or calibrated electrical behavior.
