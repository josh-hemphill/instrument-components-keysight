# TDS1002 static application composition examples v1

These examples specify the intended public construction and operation sequence for implementation and compile verification. The application supplies a real complete-message connector; these snippets do not contain a VISA installation, hardware discovery or physical bench result.

Before opening, fit a documented TDS2CM/CMA/CMAX GPIB module and commission addressed remote GPIB, main view, YT display, normal trigger, Trigger View off, matching connected voltage probes, DESE255 and clean event/status state. Use exclusive stable control. The supplied connector must enforce finite deadlines,256-byte response allocation, LF/EOI framing and plain-text bodies that reject binary headers. Standard .NET VISA composition uses `endpointScheme: "scpi"`, `terminator: "\n"`, `maxResponseBytes: 256`, `allowBinaryBlocks: false`, and a positive timeout suitable for the timebase/trigger wait.

Rust, with application-owned `connector: Arc<dyn instrument_scpi::Connector>`:

```rust,ignore
use instrument_api::{catalog::Oscilloscope, *};
use instrument_driver_tektronix::tds1002::{
    Binding, Commissioning, CommunicationModule, Configuration, Factory, Probe,
};
use instrument_runtime::{DriverRegistry, Selection};
use std::{collections::BTreeSet, sync::Arc};

let endpoint = Endpoint {
    scheme: "scpi".into(),
    address: "GPIB0::7::INSTR".into(),
};
let factory = Factory::new(vec![Binding {
    endpoint: endpoint.clone(),
    configuration: Configuration {
        communication_module: CommunicationModule::Tds2Cma,
        channel_1_probe: Probe::X10,
        channel_2_probe: Probe::X1,
        commissioning: Commissioning::QualifiedGpibNormalModeConfirmed,
    },
}], connector)?;
let registry = DriverRegistry::new(vec![Arc::new(factory) as Arc<dyn DriverFactory>])?;
let candidate = Candidate {
    endpoint,
    identity: Identity {
        manufacturer: Some("TEKTRONIX".into()),
        model: Some("TDS1002".into()),
        ..Identity::default()
    },
};
let handle = registry.open(&candidate, &Selection::default(), &BTreeSet::new(),
    ExecutionMode::Sync, ConnectionOwnership::Owned)?;
let scope = Oscilloscope(&handle);
scope.set_timebase_scale(1e-3)?;
scope.set_channel_scale(1, 1.0)?; // displayed V/div with commissioned10X probe
scope.set_channel_display(1, true)?;
scope.set_channel_coupling(1, "DC".into())?;
let vpp = scope.measure_vpp(1)?; // new SAMPLE/SEQUENCE, waits for normal trigger
handle.close(false)?; // resource release; no device cleanup/reset/stop commands
```

C#, with application-supplied `IScpiConnector connector`:

```csharp
using InstrumentComponents.Drivers;
using InstrumentComponents.Drivers.Catalog;
using InstrumentComponents.Drivers.Tektronix;
using InstrumentComponents.Runtime;

var endpoint = new Endpoint("scpi", "GPIB0::7::INSTR");
var configuration = new Tds1002Configuration(
    Tds1002CommunicationModule.Tds2Cma,
    Tds1002Probe.X10,
    Tds1002Probe.X1,
    Tds1002Commissioning.QualifiedGpibNormalModeConfirmed);
var factory = new Tds1002Factory([new(endpoint, configuration)], connector);
var registry = new DriverRegistry([factory]);
using var handle = registry.Open(new(endpoint,
    new InstrumentComponents.Drivers.DeviceIdentity("TEKTRONIX", "TDS1002")));
var scope = new Oscilloscope(handle);
scope.SetTimebaseScale(1e-3);
scope.SetChannelScale(1, 1.0);
scope.SetChannelDisplay(1, true);
scope.SetChannelCoupling(1, "DC");
var frequency = scope.MeasureFrequency(1); // fresh sample, unit/event checks
handle.Close(); // releases owned host only
```

Opening is passive. The first operation writes HEADER OFF and VERBOSE ON, verifies DESE255 and reads/clears ESR. Every completed operation checks ESR again. A fresh scalar method intentionally replaces a prior average/peak mode and leaves the instrument stopped in SAMPLE/SEQUENCE; it preserves externally configured trigger source/level/slope. Run and Single only start acquisitions. Close does not restore modes, event history, communication format or instrument outputs.

For genuine native async, attach the separately supplied native connector and select NativeAsync in the registry; do not wrap synchronous methods and call that native. Dedicated-worker frontends are available for blocking hosts. All paths share one serialized runtime handle and the same immutable requests. On uncertainty, timeout or cancellation, do not retry through the quarantined handle: externally resynchronize/recommission and reopen.
