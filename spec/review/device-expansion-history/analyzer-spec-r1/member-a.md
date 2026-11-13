# Council A — independent 3585B specification review

Member **A**; actual model **gpt-6.1-sol**, reasoning **high**. Stable neutral framing: consider a new user's interpretation. Stage **SPEC**, round **analyzer-spec-r1**, rubric **v1**. Frozen manifest SHA256 **8b853b3d5a22481e98558032a9997a9ba9ec30a41789ecc1eed5e4768411bec5**. I verified this identity and all nine bound-file hashes. All six categories were independently reviewed against the current packet, with no peer scores/ballots inspected or grade carryforward.

**All six 3585B specification categories PASS.** This is a judgment of the declared design and executable acceptance plan. It does not establish backend implementation, physical instrument behavior, native/package execution or nine-item completion.

| Item | Category | Score | Target | Derived vote |
|---|---|---:|---|---|
| 3585B | EVIDENCE | 9.4 | >=9.2 | PASS |
| 3585B | API | 9.3 | >=9.2 | PASS |
| 3585B | PROTOCOL | 9.4 | >=9.2 | PASS |
| 3585B | PARITY | 9.3 | >=9.2 | PASS |
| 3585B | VALIDATION | 9.3 | >=9.2 | PASS |
| 3585B | DELIVERY | 9.3 | >=9.2 | PASS |

## Direct inspection and checks

I read the frozen analyzer specification,123-case independent authoring helper/oracle, source metadata, OCR/render manifest, independent clearance and supporting external tooling/platform plan. The analyzer-specific public constructor and native-language admission govern; supporting first-three constructors do not apply to this model.

I verified all150 retained OCR/render rows against their recorded text/image hashes, plus the separate format-detail render hash. I also independently rendered **PDF268 directly from the actual bound primary PDF** with scale2200; its PNG hash exactly matched the retained primary-page image. The source is03585-90017, October1990, serial3008A00860 and greater; source/edition/serial applicability are explicit, rather than inferred from a neighboring analyzer.

Direct visual primary-page inspection included:

- **PDF268 and its high-resolution detail:**11-character signed amplitude field,12-character signed frequency field, leading-zero spaces, fixed dB exponentE-0,0.1Hz frequency output resolution and trigger requirement.
- **PDF271:**D2 frequency/comma/amplitude/CRLF order and final-byte EOI.
- **PDF277:**T6 behaves as delayed T5 without SRQ; marker validity depends on visibility, rearming/settling and selected marker. It does not prove a fresh complete sweep.
- **PDF221:**display-line deactivation exposes the marker; trace priority, absolute reference units and the warning that stored traces under changed settings can display misleading physical coordinates.
- **PDF272/274:**CF/FS/RL message forms and suffixes, CL/SP commands, range/trace-related commands.
- **PDF176:**center/span hardware limits including zero, reference-level envelope relative to input range, and SWEEP SPAN LIMITED behavior.
- **PDF251:**stored-trace largest point and lower-frequency tie behavior for peak search.
- **PDF278:**R12 is+30dBm with50Ohm termination.
- **PDF292:**fixed-range and valid-marker guidance, including delayed T6 after setup changes. I also read the retained coupling, bandwidth-list and command-EOS text and checked key literals against the command-page scans.

I executed the independent authoring helper in a fresh temporary directory. Its123-case output reproduced the frozen bytes exactly, with123 unique IDs and one expected result/error per case. This validates golden authoring reproducibility, not backend conformance or observed hardware replies.

## Category evidence

**EVIDENCE9.4 — PASS; high confidence.** The own-model scanned manual is inspectable through a byte-bound PDF plus explicit OCR/render provenance. Critical glyphs and field widths were verified visually, with a direct independent source-page render. Commands, range/envelope assumptions, native interface, marker visibility and units have exact primary evidence. The specification repeatedly and accurately labels examples as synthetic diagram instantiations. Hardware0Hz/fractional capability is distinguished from the declared software1Hz/integer floor; no false physical limitation is introduced.

**API9.3 — PASS; high confidence.** Eight existing neutral SpectrumAnalyzer methods form a useful scalar subset. A concrete external factory takes copied per-exact-endpoint commissioning bindings, bounds them1–4096, rejects unknown/incompatible admission before acquisition and gates absent genuine-native mode. No SCPI fallback or undocumented reset/sweep/trace capability appears. Stored-trace MarkerPeak and selected-marker MarkerX/Y semantics are clearly disclosed to a new user, including their differing freshness guarantees. Commissioning is an external assertion, not covert setup or hardware detection.

**PROTOCOL9.4 — PASS; high confidence.** Setter commands, integer/tenths formatting, bandwidth sets and exact CL/SP or CL/D2,T6 ordering are specified. The24-character parser preserves spaces, validates both fields even when returning one, requires the documented dB exponent and handles range/negative-zero policies explicitly. A numeric D2 record does not invent overload detection. LF command EOS, CRLF/EOI reply framing, allocation bounds and independent host deadlines address native pending IO. Whole operations share one lease, uncertainty quarantines, and ownership/close/recovery never introduce hidden device traffic.

**PARITY9.3 — PASS; high confidence at SPEC stage.** Existing neutral methods, exact common oracle and integer-derived numeric formatting give an implementable equivalent Rust/C# contract. Source/packed sync/native/worker and Linux/Windows x64 .NET8/10 NativeAOT requirements reuse established external tooling rather than claiming existing executions qualify this backend. The analyzer needs no new core operation; the entire expansion's final compatible immutable SDK producer must still include any new switch/digital schema. Native support remains dependent on a genuine connector and does not imply OpenTAP host AOT compatibility.

**VALIDATION9.3 — PASS; high confidence at SPEC stage.** The123 independent cases cover all eight operations, every admitted RBW/VBW choice, limits and off-grid values, both marker units, fixed-space syntax, negative/zero/boundary values, malformed discarded-field rejection and errors at every transaction position. B-PROC01–07 add concrete admission, immutable-boundary/capability, actual pending CL/query timeout/cancellation, healthy-invalid follow-up, subsequent no-traffic quarantine and ownership checks. They require actual model integration in the source/packed platform matrix and preserve the distinction between synthetic software conformance and physical evidence.

**DELIVERY9.3 — PASS; high confidence at SPEC stage.** Static registration and the concrete external hp3585 factory keep proprietary native commands outside core. Supported/deferred operations, serial applicability, commissioning conditions, stored-global-peak versus valid-selected-marker behavior and synthetic-record provenance must remain in user-facing documentation/examples. The generator/TestKit/checksum-joined package/native plan is explicit, publicalpha.1 lacks the addition, and future coordinated version/pin changes require separate qualification. No publication, merge or hardware session is implied.

## Required changes and limitations

**Mandatory defects: none demonstrated. Required specification changes: none. Rubric amendments: none.** All numeric scores use one-decimal precision and the unchanged >=9.2 threshold; final Council aggregation still requires all three members.

Implementation review must verify actual code, actual pending cancellation/deadlines and the complete source/packed native matrix. The current case reproduction is not that implementation proof. No observed raw device transcript is supplied or claimed. Physical serial applicability, truthful fixed50Ohm/R12/dbm/liveA commissioning, exclusive control and suitable non-overloaded input remain application responsibilities. Neither SP nor T6 establishes a completely fresh global physical peak, and no D2 overload flag establishes calibrated/unoverloaded measurement status.

Primary scans were sampled for each supported command/range/parser family, not reread cover to cover. Only my ballot artifacts were written. **Exact TDS1002A remains pending**, with no model substitution or overall nine-item completion claim.
