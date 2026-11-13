# TDS1002 independent final specification review

**Clear for Council specification review.** No demonstrated mandatory design defect remains in the declared ten-operation TDS1002 subset. This is an independent pre-Council review, not a numeric Council ballot or implementation approval. The user corrected the model to TDS1002, without A/B; the earlier identity blocker is therefore resolved by new own-model evidence. Earlier frozen packets and scores were not edited or reused as this assessment.

Exact reviewed hashes are in [TDS1002-clearance.json](TDS1002-clearance.json): specifications, composition, examples, 334-case oracle, independent authoring helper, fixture mapping, provenance, source manifest/research and this review's audit/check inputs. Four actual primary-manufacturer PDF signatures and byte hashes were independently verified. Relevant pages and design findings are retained in [TDS1002-PRIMARY-AUDIT.md](TDS1002-PRIMARY-AUDIT.md).

## Interfaces and construction

Actual qualifying manuals 071-1075-04 and 071-1064-00 support non-B TDS1002 with CM/CMA/CMAX GPIB. MEM's legitimate serial capability is distinguished from GPIB; no B-model USB path, TDS1002A/B alias or old TDS210 firmware restriction is imported. Both exact TDS1002 and documented TDS 1002 passive spellings use one canonical identity. Later 077-0444-01's own-applicable page 140 explicitly clarifies optional SOURCE[1] query grammar; the older own-model user example establishes the unsuffixed setter.

The typed factory/configuration supplies exact frozen endpoints, communication module, both probe factors and affirmative commissioning. Admission rejects unknown/unsupported configuration before connector acquisition, requires 1–4096 copied bindings and checks selected endpoint capabilities/modes. All requests remain existing neutral Oscilloscope operations; no vendor rules enter core. Static Rust/C# registry examples provide the corresponding construction/use sequence.

## Protocol and state

HEAD OFF and VERB ON address distinct header and discrete-argument behavior. DESE 255 must be observed before relying on clean status. Pre/post ESR 0 checks preserve a fail-fast boundary without silently clearing or draining errors. The documentation discloses persistent communication-format changes and read-clear/event-history consequences. Power-on or historical events must be handled explicitly during external commissioning.

The fresh scalar transaction stops an existing acquisition, selects SAMPLE/SEQUENCE, starts RUN, waits through OPC, verifies stopped state/count 1, selects/verifies source/type/quoted unit, reads VALUE and checks final status. It cannot treat an old cached VALUE, ignored RUN, masked error or finite warning as successful fresh acquisition under the declared conditions. Scope remains stopped afterward; mode restoration and implicit retry/abort are absent. Run and Single are separately documented start-only operations.

The source verifies only non-B voltage-probe factors 1/10/100/1000; B-only YUNIT/current/math features are excluded. Thirty-one canonical main-timebase settings and integer-derived probe-scaled voltage settings avoid silent hardware-rounding claims. Strict bounded ASCII grammars distinguish malformed responses, wrong valid states, zero Vpp, nonpositive frequency and the invalid 9.9e37 sentinel. A 60 MHz analog-bandwidth rating does not become an invented numeric parser ceiling.

## Independent oracle and executable plan

I inspected the exact 334-case oracle and authoring helper, then copied the helper into an isolated review directory and reproduced the oracle and fixture byte for byte. It imports no production plan or backend. Checks independently confirmed unique IDs, ten positively exercised operations, no traffic on structural rejection and final status checks on successful scripts. Per-case provenance joins all 334 IDs one-to-one, binds the primary PDFs and source manifest, and marks every case as unobserved on hardware. Published scalar examples are not misrepresented as an observed full scripted transaction.

The cases cover all 31 timebases, both channels' 22 fixture scale settings, all ten operations, repeated fresh acquisitions, register/unit/state/grammar/sentinel failures and typed IO failures at each full-measurement position. The separate T-PROC01..11 acceptance requirements cover runtime behaviors that a JSON oracle cannot establish: immutable/scoped configuration, nonfinite inputs, usable pre-IO rejection, parent quarantine, ownership, concurrent whole-operation serialization and actual pending native deadlines/cancellation. Their implementation and execution remain required; these plans are not execution results.

The delivery plan uses the external generator/TestKit and compatible immutable SDK development feeds, with actual generated-source and packed Rust/C# consumers, four frontends and Linux/Windows x64 Native AOT .NET 8/.NET 10. It requires binary/checksum/log evidence, warning gates and compiled composition examples before completed implementation claims. Existing published alpha.1 is explicitly excluded from new-factory availability.

## Remaining limits

Hardware has not been opened or measured. Installed module/probe truth, suitable inputs/trigger, physical limits and calibration remain external facts. A missing trigger can time out; cancellation may leave the instrument acquiring. The quarantined session does not issue STOP, device clear, reset or cleanup, and reopening requires external resynchronization/recommissioning. Readbacks and clean status do not constitute electrical protection or calibration certification.

This subset excludes waveform transfer, serial/USB host support, trigger setters, identity/reset/shutdown and B/TPS features. Wider support needs its own contract and evidence. The full nine-item Council review must use the current rubric v2 frozen packet and obtain fresh complete grades; this clearance does not carry older v1 grades forward.
