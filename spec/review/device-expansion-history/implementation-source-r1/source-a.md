# Council A — early implementation source inspection

Member **A**, actual model **gpt-6.1-sol**, reasoning **high**. Stable framing: consider a new user's interpretation. Rubric **v1**. Stage **EARLY_IMPLEMENTATION_SOURCE_INSPECTION**, not final grading. Frozen manifest SHA256 **6f4c9aea5ffcec1cf723237bb94e397a0b7e276421fdccec6b5c9cce747a173f**. I verified this identity and all **26** bound-file hashes and inspected the copied source files against the three adjacent approved specification packets. I did not inspect peer findings or evolving production worktrees.

## Required source correction

### A-SRC-01 — Rust/C# endpoint-scheme admission differs for Unicode surrounding whitespace

**Priority P2. Categories affected: API/PARITY. Confidence: high from direct source inspection.** Applies to the qualified first-three factory, the34980A parent/module factory and the3585B factory.

Rust validators accept a scheme such as `"\u00a0scpi\u00a0"` or `"\u00a0hpib\u00a0"` with a valid nonempty address and correct commissioning: Rust `str::trim()` removes U+00A0 before the ASCII-insensitive scheme comparison. `endpoint_key` also normalizes that scheme to the ordinary ASCII key. The C# corresponding validators explicitly inspect the **original** scheme and reject any non-ASCII character, so they raise InvalidArgument for the same otherwise-valid binding. The mismatch is observable at factory construction, before acquisition; it does not depend on later registry matching.

Frozen source locations:

- `instrument-components-keysight/src/qualified_scpi.rs:32` (`endpoint`, comparison at33).
- `instrument-components-keysight/src/hp34980.rs:58` (`endpoint`, comparison at59).
- `instrument-components-keysight/src/message_backend.rs:203` (`endpoint_key`) and209 (`validate_endpoint`, used by hp3585).
- `instrument-components-keysight/dotnet/InstrumentComponents.Drivers.Keysight/QualifiedScpi.cs:20` (original scheme non-ASCII rejection at23).
- `instrument-components-keysight/dotnet/InstrumentComponents.Drivers.Keysight/Hp34980.cs:22` (rejection at24).
- `instrument-components-keysight/dotnet/InstrumentComponents.Drivers.Keysight/Hp3585.cs:16` (rejection at18).

**Required fix:** choose the same admission policy in both languages, consistent with the declared ASCII scheme semantics. Matching the existing C# policy by rejecting non-ASCII Rust scheme strings before trimming is straightforward. Add paired constructor/open admission cases for NBSP-wrapped schemes and ordinary ASCII whitespace/case normalization, asserting the same error/result and zero connector acquisitions for rejected inputs. Include duplicate-key cases using admitted normalized spellings.

This is a demonstrated source parity defect, not a claim that malformed schemes produced device traffic. It was reported to the facilitator immediately. I did not execute a compiled Rust/.NET reproduction in this source-only inspection.

## Inspected behaviors that match the approved design

These are source observations, not executed backend/platform claims.

- **Shared transaction engine:** Rust and C# perform structural request validation and engine preparation before marking uncertainty. Each host write/query is preceded by unhealthy state; subsequent decode/IO failure preserves that quarantine, including the33120post-query InvalidArgument conflict. Success alone commits state and restores health. Native uncertainty is marked before awaiting, so future abandonment does not restore confidence. Empty/oversized/non-ASCII/control-character bodies are rejected. Device close is idempotent and invokes host close only for owned resources. Finite pending deadlines, framing/allocation enforcement and ending host access after cancellation remain connector/runtime obligations requiring execution evidence.
- **34401A:** seven exact scalar operations, DEF versus explicit range messages, minimum positive finite range and maxima, standardE11 formatting, exact15-character signed reply grammar, positive overload sentinel/status/units and giant-number rejection are represented in both languages. The active generic34401profile is absent from the frozen profiles.json. The replacement is explicit rather than appended to generic factory lists.
- **33120A:** all eight guard queries precede the operation tail. Structural bounds are pre-IO; amplitude/offset check both requested and rounded wire values against a queried counterpart. Known wrong states are Device, malformed states Parse; load-specific9.9e37 HighZ is recognized before generic magnitude rejection. No hidden output enable/disable, reset or post-write completion query appears.
- **53132A:** integer-millisecond canonical grid comparison and integer-derived wire formatting match the spec. Local setters commit only after successful validation, measurements reapply intent after CONF, and each READ is fresh. NR3 exponent/significant-digit checks, positive quantity/sentinel Device policy and Hertz/Second units are present in both implementations. C# uses fresh per-open Core; Rust creates fresh local intent in selected().
- **34980A:** routes freeze validated physical/logical maps. Slot conflicts include switch/digital cross-namespace conflicts; same logical ID may legitimately exist in separate switch/port namespaces. Descriptor operation union is narrowed by selected-endpoint required-capability checks, and returned backend operations reflect only that endpoint. One MessageDevice owns all class/module views, health and connector lifetime. The immutable C# Core can be shared between opens without sharing session health or mutable intent.
- **34937A/34922A:** native addresses derive from validated slot/local values. Exact single-path ROUTe commands and strict0/1 parsing are present; no automatic same-bank open, paired mux channel, matrix endpoint or globalOpenAll is emitted. NC/power/physical continuity implications still need user-facing documentation as already required by the spec.
- **34950A:** frozen direction/port and0–255 validation precede guards. All four bank width queries are emitted in order, followed by selected direction/polarity and bank-first handshake/input/output memory guards. Selected threshold or drive/level checks precede SENSE sampled input or explicit SOURCE write. Byte parser accepts decimal digits/leading zeros with0–255 bound; zero survives the backend path. No MEAS/configuration/width/direction mutation is hidden in setup.
- **3585B:** both parsers check24-byte/character ASCII framing, comma andE-0, then validate **both** fixed fields before choosing the requested axis. Integer bandwidth/frequency and canonical reference-tenths validation match the spec. Marker operations contain only CL/SP or CL/D2,T6; no sweep/reset/normalization command was added. Source comments retain stored-peak versus selected-point validity distinctions.

## Oracle and core boundary checks

The frozen active corpus contains **1859 unique cases**. I compared every new model case with the approved frozen oracle:69DMM,67generator,50counter,195module and123analyzer entries are present with identical logical JSON content. No approved new-model case was missing or changed. Module shards correctly use the single parent factory ID; the fixture mapping matches the approved sparse switch/port inventory. The retired unqualified34401profile is absent.

I invoked the frozen pure contract-generation function in memory against the frozen neutral operation manifest. Its generated validation branches preserve positive channels, allow a DigitalIoWriteByte value of zero, reject values greater than255, and retain the old nonzero constraint for FunctionGeneratorSetBurstCount. The included neutral Rust/C# consumer sources explicitly exercise zero read/write,256 rejection and existing zero-positive-count rejection. This establishes source/generation intent, not compiled execution.

The five new operation definitions contain logical channel/port/u32/Count semantics without slots, model names or vendor commands. The copied OpenTAP views/steps follow the neutral DigitalIo/Switch API; WriteByte UI validation permits zero and caps255. Existing external static factory lists do not silently instantiate the new commissioned factories. Within inspected files, proprietary command grammar remains in the external Keysight backends.

## Evidence still needed for the final implementation gate

No numeric scores or pass verdict are assigned. The packet intentionally lacks the final procedural test implementations/execution, complete construction/user examples, regenerated artifact/package audits and source/packed native platform evidence. Final review must inspect the actual Q-PROC/M-PROC/B-PROC integration tests, real pending cancellation/deadline and shared-module noninterleaving/quarantine proofs, parser/formatter/range negatives, compatible new immutable SDK producer/tool pins and actual Linux/Windows x64 .NET8/10 NativeAOT consumer execution. Generator intent, scripted Timeout fixtures and source inspection alone cannot replace those results.

No other demonstrated command/parser/range/ownership source defect was found in this inspection. This is not proof that no defect exists; external runtime/connector behavior and omitted final artifacts remain unverified. Exact TDS1002A remains outside the implemented eight-item subset and pending. Only this review artifact remains written; frozen source hashes are unchanged.
