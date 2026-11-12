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

Published `0.2.0-alpha.1` does **not** contain these new HP backends. This branch implements the manual-derived HP 5334A and 5334B counter subset; 3497A and 8920A/B/8921A specifications are under staged implementation/review. All remain hardware-unvalidated. No new repository is needed: HP, Agilent and Keysight implementations share this external package while core contains only neutral contracts and standard tooling.

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
