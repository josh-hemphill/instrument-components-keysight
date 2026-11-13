# Source-audited API gaps and extraction constraints

Source baseline on 2026-11-13: core1b0704dbb2c4a9f4ffda9f6b1665de12654f672d, Keysightc63cc025d283deb0832dbe2b9c721d8a33412160, Tektronix787c4881636d18618f1f8a503a7be7273333064f. This report is preliminary until own-model primary manuals arrive. No physical validation exists.

## Existing versus verified support

- `keysight.34401a` already exists as a source-derived profile with DMM measurements/configuration, INIT/FETCH/READ/TRG, identity/reset/shutdown. It is **not** manual-qualified support: it advertises temperature, carries inherited USB VID2391/PID1543, and uses generic split-phase state. Own-model evidence must qualify/remove these behaviors before claiming this request satisfied. The source README explicitly says all inherited identities and commands are hints, not model validation.
- `keysight.53131a` is a neighboring model, not53132A evidence. Its generic `:SENSe:FREQuency:APERture` and totalize commands must not be copied without53132A manual verification.
- Tek `tektronix.mdo3024` currently has generic Agilent-style `:TIMebase`, `:CHANnel` and `:WAVeform` commands. It is not a TDS1002A protocol template. TDS1002A applicability and actual communication interface must be established independently from similarly named TDS1002/B devices.
- Other requested identities are absent from current production profile lists.

## Neutral API currently available

Core already defines DMM scalar and configure/read/fetch operations; FunctionGenerator settings, identity/reset/shutdown; Counter frequency/period/gate/select; SpectrumAnalyzer frequency/span/bandwidth/level/trace/marker/sweep; Oscilloscope settings/measurements/trace. Support only verified subsets.

Existing `Switch.CloseRoute(ch1,ch2)`, `OpenRoute(ch1,ch2)`, `IsClosed(ch1,ch2)` are relational pairs inherited from matrix routing. They must not encode module slots, vendor SCPI addresses or independent relay actuation through invented row/column values. `Switch.OpenAll` is global and would be unsafe when a driver binds only a partial inventory.

There is no neutral digital IO class, direction model, or port-width/value contract.

## Proposed additional neutral contracts (await own-model evidence)

A channel-based switch abstraction should expose `CloseChannel(channel)`, `OpenChannel(channel)`, `ChannelIsClosed(channel)` with a commissioned complete or partial frozen logical-channel binding. A channel denotes one logical normally-open/common path, not a physical contact state guarantee. Module-specific FormC NC/NO implications and mux interactions remain in the vendor descriptor/spec. Avoid `OpenAll` unless the binding proves full affected inventory and the implementation emits only declared paths.

Digital IO should expose finite-width port reads/writes as integer counts, with immutable application-declared direction and logical/native mapping. Do not expose a write operation for input-only bindings, infer width or silently truncate overflow. If direction is changeable, direction must be an explicit operation with an atomic state/verification contract, not a side effect of read/write. Wait for34950A manual to choose byte/word width, port count and exact input/output semantics.

## Shared device protocol contract

All factories are statically linked and explicitly selected. Passive metadata and opening send no identity/reset/enable/route commands. Freeze endpoint-specific options/module inventories before acquisition. Validate arguments and requested capabilities/modes on the selected exact endpoint before IO. Whole operations serialize under one SDK lease. Require finite deadlines, bounded complete message bodies, correct terminator handling with payload whitespace preserved, no hidden retry/drain or family fallback. Real native connectors only advertise native async; synchronous connectors use SDK workers. Preserve borrowed ownership; close owned resources once; close is resource release, not hardware shutdown. Uncertain IO, malformed replies or cancelled active native operations quarantine until explicit external resynchronization/reopen. No automatic physical recovery is invented.

## Required independent evidence/tests

Per model/module: exact primary pages and hashes, valid/invalid range boundaries, identity aliases/interface applicability, command casing/numeric formatting, strict reply/token/overflow grammar, state prerequisites and unsupported operations. Golden traffic must be authored from manual evidence independently of backend implementation. Exercise sync/native and worker paths in Rust/C#, selected route capability isolation, immutable inventory, ownership, actual pending deadlines/cancellation, quarantine and packed NativeAOT Linux/Windows x64 .NET8/10. Simulation is not bench evidence.
