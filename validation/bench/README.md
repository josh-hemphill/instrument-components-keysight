# Explicit bench evidence application

This all-five console application uses versioned SDK, VISA and driver packages.
It is a bench host, separate from the Native AOT application proof in
`validation/composition`. No hardware has been validated. The preserved generic
commands for some model hints, including the 2602B, are source evidence only;
review device manuals and each driver's provenance before physical execution.

This host targets the exact coordinated 0.3.0-alpha.1 packages. Check the matching releases for public availability. Candidate composition runs use isolated checksummed feeds. The example and fake self-test use the preserved generic 34410A profile. Qualified HP/Agilent/TDS factories require explicit commissioning and are exercised by the separate package/composition fixtures; this bench host's generic factory list does not register them. A 34410A scripted result does not qualify physical 34410A behavior.

From the repository root, after that release:

```sh
dotnet restore validation/bench/Bench.csproj
dotnet run --project validation/bench/Bench.csproj -- --list
dotnet run --project validation/bench/Bench.csproj -- --self-test
dotnet run --project validation/bench/Bench.csproj -- --validate-config --config validation/bench/configs/example.json
```

These modes never open VISA. The self-test exercises exact fake identity and
measurement traffic, bounds, units and one connection lifetime. It also injects a
fake byte transport to verify the adapter's 1 MiB payload cap, early block-header
rejection and typed parse errors without opening VISA. Commit the
resolved `packages.lock.json` after public restore; subsequent bench execution
uses `dotnet restore --locked-mode`. Artifact-bootstrap software validation is
recorded separately and is not a public-registry or physical-evidence claim.

Create a bench-specific committed configuration from `configs/example.json`.
Specify the exact resource, canonical driver ID, expected physical manufacturer
and model (optional serial), operator, wiring, reference source and VISA runtime
version. Every scalar check needs finite bounds and a known physical unit;
switch queries need ExpectedBoolean. Review the literal source-derived commands
for each selected operation and confirm the actual device's safe input ranges.
The app only maps the explicit listed read/query operations, plus an explicit
`PowerMeterConfigureMeasurement` request when the plan chooses Watt/Dbm. A power
read requires that explicit configuration to establish units. No output-enable,
reset, discovery, probe, retry or reconnect is inserted. Closing releases IO;
this read-oriented plan does not claim checked physical output shutdown.

Only set `ApprovedForHardware: true` after reviewing wiring, the reference and
expected ranges. Then an operator can invoke:

```sh
dotnet restore validation/bench/Bench.csproj --locked-mode
dotnet run --project validation/bench/Bench.csproj -c Release --no-restore -- --hardware --config validation/bench/configs/approved-bench.json --record artifacts/bench-unique-record.json
```

The main repository's manual bench dispatcher also requires an immutable
application commit, an exact resource, and the reviewed committed config path.
It is not part of normal CI and has not been run. If INSTRUMENT_RESOURCE is set,
it must equal the committed resource. Physical identity is queried explicitly
and must match before measurements run. Existing evidence files are preserved.

Evidence records the config and package-lock hashes, application and selected
assembly hashes, firmware identity, platform/runtime, operator/setup, timestamp,
exact wire traffic, typed units/status/values, acceptance ranges, verdict,
duration and failures including close failures. Retain failed records as well
as successful records. Review evidence per model/firmware/transport combination;
a passing fixture or a related model does not establish physical support.
