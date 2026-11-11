# Instrument Components Keysight drivers

Independently packaged Rust (`instrument-driver-keysight`) and .NET (`InstrumentComponents.Drivers.Keysight`)
source-derived profiles, version `0.2.0-alpha.1`. These are preview candidates;
no devices in this library have physical bench validation yet.

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

`profiles.json` is the production wire plan. `cases.json` is a separately
specified golden traffic/value corpus; its generation never reads production
plans. Generated Rust/C# files contain static data and require no runtime JSON.
`provenance/operation-coverage.json` joins behavior to the shared case IDs.
`provenance/source/` is historical audit evidence, excluded from production
packages. The SDK TestKit and Tokio consumers are development-only dependencies.

Before public SDK publication, CI uses checksummed release-form SDK archives from
a pinned main workflow. The consumer job checks out only this repository and
resolves exact versioned SDK dependencies from fresh verified development feeds.
This is explicitly bootstrap validation, not public registry availability.
`instrument-driver verify-package` builds generated-source and packed consumers,
runs Rust default and async modes, and publishes and executes .NET Native AOT
with all four frontends on Linux/Windows x64 and net8.0/net10.0. It also requires
the intentional trimming-warning probe to fail. No hardware access occurs.

Core applications that do not select this package need none of its artifacts.
Do not publish tags until native gates and the cross-repository council review
are complete and the trusted publishers/registry ownership are configured.
