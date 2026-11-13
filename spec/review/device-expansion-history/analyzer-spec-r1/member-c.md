# Council C — 3585B SPEC round 1

Member **C**; actual model **gpt-6.1-sol**, reasoning **high**. Stable framing: consider model, module and execution distinctions. Rubric **v1**, exact target **>=9.2/10**, one-decimal scoring. Frozen manifest SHA256: `8b853b3d5a22481e98558032a9997a9ba9ec30a41789ecc1eed5e4768411bec5`.

I independently assessed all six categories on the copied frozen packet. I verified the manifest and every listed file/PDF hash, read no peer ballot or score, and carried no earlier grades forward. This is a specification review, not a completed implementation or hardware qualification.

| Item | EVIDENCE | API | PROTOCOL | PARITY | VALIDATION | DELIVERY |
|---|---:|---:|---:|---:|---:|---:|
| 3585B | 9.3 PASS | 9.3 PASS | 9.3 PASS | 9.2 PASS | 9.3 PASS | 9.2 PASS |

All votes derive directly from the numeric scores and exact threshold. **No mandatory defect, required change or rubric amendment found for this SPEC gate.** This is one member's independent ballot; the facilitator must still obtain all three valid ballots and apply the approved aggregation rules.

## EVIDENCE — 9.3 PASS

The own-model primary is manufacturer manual 03585-90017, October 1990, for serial 3008A00860 and greater. Its bytes, source URL/applicability and scanned-page provenance are retained. I verified all **150 OCR/render page pairs** and the format-detail image against the hashes in the bound OCR manifest. OCR is used as an index, with important glyphs cross-checked visually.

I visually inspected actual rendered PDF pages **268, 277, 221, 272, 274 and 278**. Page 268 (printed 3-9-11) defines 12-character frequency and 11-character amplitude fields with leading-zero spaces and the dB `E-0` exponent. Page 277 (3-9-20) defines delayed T5/T6 trigger behavior and marker visibility conditions. Page 221 (3-8-4) establishes absolute marker units/trace priority and warns about stored data after scale/frequency changes. Command/range pages establish CF/FS/RL, CL/SP and R12 +30 dBm at 50 ohms. I also read the retained primary OCR for PDF 174–177, 211–215, 251, 267, 271–274 and 292 to check numeric limits, discrete bandwidths, coupling, SP behavior, native framing and selected-marker validity.

The specification clearly labels diagram-derived example replies as **synthetic**, rather than observed hardware transcripts. Serial applicability, fixed modes, electrical prerequisites and physical-validation absence are explicit. It does not substitute a related analyzer or a SCPI family template.

## API — 9.3 PASS

Eight existing neutral spectrum operations provide a coherent scalar subset. Public Rust/C# factory and commissioning/binding shapes are specified; the `hpib` endpoint admits addressed native HP-IB, not an inferred SCPI path. Copied 1–4096 endpoint bindings, normalized duplicates, invalid enums/addresses, exact selected-binding capabilities and genuine-native admission are defined before connector acquisition. Static registration/opening is passive.

The commissioning enum expressly affirms the applicable serial range, fixed R12/50-ohm/dBm state, live selected A trace, disabled conflicting modes, exclusive control and clean framing. These are affirmative application facts rather than hidden discovery. The 1 Hz integer center/span floor is honestly a software subset; hardware zero/fractional capability is not denied or misadvertised. Method results use Hertz/Dbm and excluded sweep/trace/reset/shutdown operations remain absent.

The API distinguishes **stored-trace peak selection** from a freshly acquired global physical peak, and selected-marker delayed validity from a complete fresh sweep. This prevents the native SP/D2/T6 behavior from being represented as stronger neutral functionality than the manual supports.

## PROTOCOL — 9.3 PASS

The exact native messages and admitted argument sets are inspectable: ordinary integer CF/FS/RB/VB fields, integer-tenths RL formatting, CL then SP, or CL then D2,T6. The conservative reference-level envelope and three-digit formatting agree with commissioned R12 and the remote table. Coupled setting changes and out-of-band trace blanking are disclosed rather than normalized silently.

CL is correctly the only marker prelude: it clears the display line so the selected marker is visible. SP operates on the stored trace and selects the lower frequency on equal peaks. T6 is last in the string and waits for valid selected-marker data without SRQ. Its possible device-progress wait does not replace an independent host deadline.

The parser requires exact 24-character body structure, preserves spaces, checks both fields even when returning only one, recognizes the fixed dB exponent and bounds frequency independently. Malformed discarded fields cannot slip through an axis-specific decoder. LF command EOS and CRLF/EOI response framing match the primary. Allocation bounds, whole-operation serialization, quarantine after uncertainty, absence of silent recovery/retry/abort, owned/borrowed resource close and real native/worker behavior are explicit. No physical overload or calibration status is invented from numeric D2 data.

## PARITY — 9.2 PASS

Both languages use the same typed operations, static factory model, integer formatting, parser fields, result units and errors. Genuine native connectors are optional; a blocking connector is not advertised as native asynchronous IO. Worker paths retain SDK lifetime/affinity behavior.

The supporting composition plan supplies actual external generator/TestKit commands and source/generated versus packed consumer strategy for Linux/Windows x64, .NET 8/10 and Native AOT with the negative diagnostic gate. The analyzer adds no neutral operation itself, while the specification correctly requires any coordinated expansion proof to pin a compatible immutable SDK/tool producer containing the separately added switch/digital contracts. The first-three constructor does not override this model's factory. Actual execution and final revision/package joins are deferred to the implementation gate without being claimed here.

## VALIDATION — 9.3 PASS

I ran the bound independent oracle authoring helper in an isolated temporary directory. All **123 cases reproduced byte-identically**, with unique IDs and exactly one expected result or error each. The helper reads no backend production plans. Every exposed operation is represented, with all RBW/VBW choices, center/span extrema and invalid fractions, RL grid/boundary rejection, signed/zero marker values, fixed spaces, both-axis malformed-field rejection, and failures at each message position.

B-PROC-01 through -07 specify actual-factory passive identity/admission, copied bindings and capacity limits, exact capabilities, pending CL writes and D2/T6 queries, genuine-native-only hosts, independent deadlines/cancellation, no detached access, healthy followup after structural rejection, cross-operation quarantine and owned/borrowed close. Source/packed consumers must execute the same tests and retain the synthetic/manual-derived labels. Stored-peak and selected-point semantics are explicit documentation assertions rather than a fabricated simulator proof of physical freshness.

The acceptance plan is sufficient at SPEC stage. Implemented tests, actual native binaries and hardware measurements have not been inspected or awarded execution credit.

## DELIVERY — 9.2 PASS

The backend belongs in the external Keysight package with explicit static registration; native commands, commissioning and model details remain outside core. No new neutral spectrum contract is required. The proposed factory, prerequisites, deferred modes, fixed parsing, ownership and physical limitations are documented. Full runnable constructor/consumer examples and actual package payload checks remain implementation-stage deliverables.

External SDK generator/TestKit reuse, compatible immutable producer/tool pinning, checksum-joined fresh feeds, source/packed Rust/C# matrix, actual Native AOT execution and warning gates are concrete. Old public alpha.1 is explicitly stated to lack the backend; a later coordinated release needs new versions and its own verified evidence. No release, merge, hardware operation or physical qualification is asserted.

## Confidence and limits

Confidence is high in the inspected manual glyphs/commands, field widths, timing distinctions and reproduced oracle identity; moderate-high in the proposed API and executable delivery strategy. The primary is scanned and no observed raw device transcript is available. Its synthetic examples are suitable software acceptance inputs, not bench observations.

The fixed commissioned state is not autodetected, physical input protection is not established, and D2 cannot prove absence of overload or calibration error. SP can select data stored before a setup change; T6 does not qualify an entire trace as fresh. These limitations are explicit and compatible with the declared subset.

This ballot approves only the **3585B specification gate** at the stated frozen identity. It does not review implementation of any item. Exact **TDS1002A remains pending** without model substitution or a nine-item completion claim.

Mandatory defects: **none found**. Required changes: **none for this SPEC gate**. Rubric amendments: **none**.
