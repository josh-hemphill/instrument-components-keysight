# Council C — early implementation source inspection

Member C; actual model `gpt-6.1-sol`, reasoning **high**. Rubric **v1**; stable framing: consider model, module and execution distinctions. Stage: **EARLY_IMPLEMENTATION_SOURCE_INSPECTION**, not final numeric grading.

Frozen manifest SHA256: `6f4c9aea5ffcec1cf723237bb94e397a0b7e276421fdccec6b5c9cce747a173f`. The manifest and all listed copied-file hashes match. I read no peer reviews and made no production edits. Specifications inspected are the adjacent approved `spec-r2`, `module-spec-r1` and `analyzer-spec-r1` packets, not evolving original drafts or live production files.

## Required fix

### C-SOURCE-01 — scheme admission differs across languages

**Affected:** all three new factory families, hence all eight reviewed items. **Category implications:** API/PARITY/VALIDATION. This is an input-admission discrepancy, not a claim of incorrect ordinary device traffic.

Rust accepts a scheme containing only Unicode whitespace around an otherwise valid ASCII name. For example, `"\u{00a0}scpi\u{00a0}"` passes `scheme.trim().eq_ignore_ascii_case("scpi")`; the shared endpoint key strips that whitespace. The equivalent `hpib` example passes the analyzer helper. C# explicitly rejects any non-ASCII character anywhere in the original scheme before opening. Consequently, a binding accepted by Rust fails construction with InvalidArgument in C#; a direct Rust factory open can select its normalized bound key while the C# factory never exists.

Frozen locations:

- `instrument-components-keysight/src/qualified_scpi.rs:32`: scheme validation uses Unicode `trim()` without an ASCII-character check.
- `instrument-components-keysight/src/hp34980.rs:58`: same.
- `instrument-components-keysight/src/message_backend.rs:209`: native `hpib` helper has the same policy; `hp3585.rs` delegates to it.
- `instrument-components-keysight/dotnet/InstrumentComponents.Drivers.Keysight/QualifiedScpi.cs:20`, `Hp34980.cs:22`, `Hp3585.cs:16`: each new factory explicitly rejects non-ASCII scheme characters.
- Shared normalized key implementations are Rust `message_backend.rs:203` and C# `MessageBackend.cs:19`.

**Concrete fix:** choose one explicit scheme-character/whitespace policy and implement it identically. Adding `scheme.is_ascii()` before the existing Rust trim/compare checks matches the current C# behavior and preserves ordinary ASCII surrounding whitespace. Add mirrored construction/open rejection cases for NBSP-wrapped `scpi` and `hpib`, plus a valid ASCII-whitespace/case-normalization control and unchanged exact address-case behavior. Rejections must happen before acquisition. No review target or supported device scope needs changing.

Confidence: **high from direct source semantics**. I attempted a tiny isolated Rust expression check, but `rustc` was not on this execution shell's PATH; no compiled reproducer or runtime result is claimed. The discrepancy itself is visible in the explicit character checks and Rust Unicode-trim semantics.

## Source observations supporting the intended design

### 34401A / 33120A / 53132A

Rust `qualified_scpi.rs` and C# `QualifiedScpi.cs` agree on exact supported operations, manufacturer/model aliases, finite structural bounds and commissioned binding admission for ordinary ASCII endpoint schemes.

- The DMM uses explicit DEF parameters, operation-specific units and the fixed signed 15-character output grammar. Only exact positive `+9.90000000E+37` becomes Overload; negative/other giant values go through the declared Parse policy. Temperature and unverified split-phase capabilities are absent from the new factory.
- E11 formatting follows the specified standard scientific formatter and normalizes exponent padding. Generator amplitude/offset validate both the original and rounded wire values against the queried counterpart. Wrong current numeric settings, malformed replies and post-query coupled InvalidArgument use the intended paths. Load-specific positive `9.9e37` is Device before generic giant-value Parse rejection.
- Counter channel/gate setters validate before committing local intent. Gate acceptance compares the supplied binary64 value with integer milliseconds divided by 1000, rather than requiring exact integral multiplication. The fresh measurement sequence reapplies intent after CONFIG, forces ASCII and ends in READ. Both decoders enforce exponent presence/length, <=15 significant mantissa digits, invalid-result sentinel and nonpositive-result Device policy. The later decimal parser rejects malformed signs that a preliminary exponent trimming step alone would not reject.
- Factory selection creates a fresh mutable counter core per opened handle in both languages. DMM/generator do not rely on a stale configured-unit cache.

No other demonstrated protocol/parser/range defect was found in these copied implementations. This does not replace executing midpoint, nonfinite, pending-IO and state-preservation tests.

### Mainframe and modules

Rust `hp34980.rs` and C# `Hp34980.cs` derive physical addresses exclusively from validated slot/local entries. They reject duplicate logical/physical paths and incompatible module types within a slot, including switch/digital conflicts. The stored maps are immutable after construction. Selected endpoint capabilities are checked against that endpoint rather than the descriptor's union.

Relay views issue the specified single-path CLOS/OPEN/CLOS? messages and strict Boolean grammar; they do not synthesize matrix pairs, OpenAll, automatic pairing or sibling opens. Digital ports enforce frozen read/write direction before guards, admit 0–255 writes including zero, and reject 256+ structurally.

Digital operations query all four selected-bank widths, then selected direction/polarity, bank-first handshake and input/output memory flags. They check threshold or drive/level and issue the exact DEC SENSE read or SOURCE write. The bank-first address remains independent of the selected non-anchor port. Numeric/token/byte parsing matches the declared policy, with no implicit MEAS/direction/configuration write. Byte results are explicitly bounded to 255 in both backends.

One parent MessageDevice/AsyncMessageDevice wraps each opened mainframe. C# shares an immutable core safely across parent opens; Rust clones its immutable core. Health/ownership resides in the opened parent backend, so all class views through that handle share it. Actual runtime lease/noninterleaving and sibling-view execution must still be established by procedural tests on the full artifact.

### Native 3585B

Rust `hp3585.rs` and C# `Hp3585.cs` expose exactly the eight approved scalar operations and native `hpib` identity. Integer-Hz settings, discrete bandwidths and integer-tenths reference formatting agree with the specified subset. Marker operations use only CL then SP or CL then D2,T6; no fresh global-sweep operation, hidden reset or SCPI fallback is introduced.

The D2 decoder checks ASCII and exact length before slicing. It parses **both** frequency and amplitude fields before choosing X/Y, preserves significant leading spaces, checks fixed `E-0`, and bounds frequency. This prevents a malformed discarded field from being silently accepted. No overload status is invented. The implementation comments preserve the stored-peak/selected-point distinction; final user-facing examples must do so too.

### Shared health, lifetime and execution

Rust `message_backend.rs` and C# `MessageBackend.cs` validate requests and prepare plans before touching IO. They mark health uncertain before each host write/query/await and restore it only after successful whole-operation completion. Post-query decode failure, transport failure or abandoned/failed active operation therefore leaves quarantine; later calls reject before more traffic. Empty local intent plans commit without device IO.

Owned resources release once and borrowed resources skip host close. No device cleanup command is emitted. Native open delegates only to the native connector after selected-route validation; optional native capability is absent without that connector. Complete-body size/ASCII/control checks are present after the connector response; enforcing bounded allocation and ending all native host accesses on cancellation remain connector contracts that require actual procedural evidence.

No extra lock is added inside the vendor engine: the single whole-operation SDK lease is an intentional dependency and needs verification against the final bound runtime and interleaving fixtures.

## Independent corpus and neutral boundary checks

I compared all current approved addition cases directly against frozen active `cases.json`: **504/504 are semantically identical** (186 first-three + 195 modules + 123 analyzer). The complete active corpus has **1859 unique IDs**, each with exactly one expected result or error.

Using immutable baseline Keysight commit `c63cc025d283deb0832dbe2b9c721d8a33412160`, I also compared the old corpus: all **1355 unaffected cases are unchanged**. Its 69 old 34401A cases are the explicit retired replacement set; the reviewed new 69-case DMM oracle appears intact. The frozen `profiles.json` contains no 34401A profile. Fixture selectors create the exact commissioned new factories; all four module shards intentionally use the single parent canonical ID.

The main operation manifest and packaged tool catalog agree semantically after removing source-location metadata. The neutral additions contain no slot/model/SCPI grammar. I executed the copied contract generator in-memory: it emits a 0–255 check for byte write values while retaining positive checks for ports/channels and existing burst count. Both typed SDK proof sources assert zero read/write success, 256 rejection and old zero burst-count rejection. This is source/generation inspection, not an execution of those typed proofs.

The copied OpenTAP generation path derives byte constraints from the neutral schema; generated DigitalIoWriteByteStep admits zero and rejects >255. The generated read step publishes a scalar count, and new channel steps route through neutral Switch methods. The extra metadata validation reads the neutral operation catalog instead of adding vendor commands. These changes do not prove OpenTAP host AOT compatibility, and no such inference is made here.

## Required evidence before final grading

The following are known incomplete parts of this early packet, not additional demonstrated source defects:

1. Freeze generated vendor profile outputs and generated SDK contracts actually used by the build. They are referenced by the copied source but absent from this manifest, so current compiled removal of the old generic 34401A registration and final generated-contract identity are not yet inspectable here.
2. Freeze all Q-PROC/M-PROC/B-PROC implementations and their invocation entry points, actual per-model executions, regressions and complete runnable examples. Source alone cannot prove shared lease ordering, no detached native access, truthful async gating or ownership under all failure/drop paths.
3. Bind final core/runtime, package/tool producer revisions, matching SDK feeds and source/packed consumers. Include fresh Linux/Windows x64 .NET 8/10 Native AOT execution, binary identities and checksum/log joins, with the negative warning gate.
4. Preserve physical and synthetic evidence boundaries. Exact TDS1002A remains pending; this report does not establish bench behavior or completion of all nine requested items.

No numeric scores, category pass claims or final implementation approval are issued. Confidence is high in the concrete admission mismatch and the corpus/source comparisons; moderate-high in code-level protocol alignment; execution-dependent properties remain unverified pending the full frozen evidence packet.

## Facilitator disposition received after inspection

The facilitator reports adding ASCII checks to the new Rust qualified-SCPI/mainframe validators and an analyzer-local ASCII validator, while preserving existing older HP admission behavior. Paired Unicode-whitespace constructor/no-acquisition tests are being added. I have not reviewed those evolving changes; C-SOURCE-01 remains a finding against the frozen r1 copies and must be verified on the next frozen artifact.
