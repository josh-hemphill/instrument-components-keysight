# Member B — implementation round 2 source findings

Member B, availability replacement (`/root/hp_council_b_recover`); actual model `gpt-6.1-sol`, reasoning `high`. Fixed framing: **Examine the boundaries between stated guarantees and their supporting evidence.** Rubric v1, exact target `>=9.2` in every category for each of HP5334A, HP5334B, HP3497A, HP8920A, HP8920B and HP8921A. This is a fresh implementation assessment; specification grades and earlier implementation evidence are not carried forward. No peer assessments were read. No production files were edited.

Source freeze: main `6524d9a3c1a1627f3fed8e4fbdca0f577220434c`; Keysight `8729fea9161367816a82c816f4275b165d1b18b2`. Independently checked all595 main and206 Keysight tracked-file hashes in `source-manifest.json`: no mismatch. Read `REVIEW-CONTEXT.md`, rubric and `validation/hp-procedural-coverage.md`. **Final numerical ballot is withheld** pending the identical final current-revision CI/evidence manifest requested by the facilitator. This record contains source findings, not a passing implementation claim.

## Required source-validation corrections

### B-IMP-1 — isolate the DAQ inventory invariants in executable proof

Locations: `validation/rust/src/hp_legacy_proof.rs:189` and `validation/dotnet/HpLegacyProof.cs:46`; acceptance matrix DAQ inventory/opening; coverage map HP-PROC-02.

The existing duplicate test supplies the identical record twice. It cannot independently demonstrate both unique public IDs and unique native inputs: either single check would reject it. The test suite also does not instantiate a conflicting Option010/Option020 slot, an empty input inventory, or local20. These are expressly required acceptance conditions in the specification/procedural matrix. A passing53-case DAQ operation shard does not establish registration-time inventory checks because those cases always start from the same valid commissioned inventory.

Add separate cases with (a) duplicate public ID/different valid native input; (b) distinct public IDs/same native input; (c) distinct inputs in the same slot with individually valid but conflicting010/020 declarations; (d) empty input list; (e) out-of-range local20, plus the explicit compensation/reference boundaries in the approved matrix. Assert `InvalidArgument` and zero connector calls. Native addresses are derived, so redundant-address mismatch and negative unsigned coordinates legitimately need no impossible typed request; that representation refinement is documented.

Observation: the current Rust `Core::new` and C# `Freeze` implementations appear to implement these restrictions correctly. This finding is missing mandatory evidence, not a demonstrated acceptance of invalid topology. It principally affects VALIDATION and the truthfulness of the coverage mapping.

### B-IMP-2 — make the .NET DAQ mutation test detect retained caller state

Location: `validation/dotnet/HpLegacyProof.cs:56`.

After construction, the current proof clears the caller list and requests unknown channel99. That request also fails with a hypothetical incorrectly retained, now-empty input list, so it does not demonstrate that the valid channel7 mapping survived. Use the exact documented channel7 query and successful scalar expectation after mutation; keep an unknown-channel rejection as a separate observation if useful. Verify resource ownership and no extra IO. The production `Freeze` does create a separate frozen dictionary, but the test's claimed observation must distinguish the defect it is meant to catch.

### B-IMP-3 — execute the RF nonfinite pre-IO cases promised by the matrix

Locations: both `hp_legacy_proof` / `HpLegacyProof` functions; procedural matrix Faults item11.

The matrix explicitly requires procedural NaN/±Infinity source-configure arguments, which cannot be represented in legal JSON CatalogCases. The vendor procedural functions do not issue these RF requests. The only nonfinite values in the current HP procedural sources are counter gate inputs. For each RF model, pass nonfinite frequency and level values independently and require `InvalidArgument`, no script events, and a usable follow-up. Neutral request validation and backend source contain the correct finite checks, but no ordinary JSON case proves their IO ordering for these inputs.

### B-IMP-4 — exercise selected-route pre-open rejection through all advertised paths

Locations: Rust legacy proof around the selected ANT/unbound endpoint opens and C# `HpLegacyProof.cs:40`; RF matrix and HP-PROC-05.

Current selected ANT-required-power and missing endpoint checks use only synchronous registry opening. Extend the exact selected-endpoint/capability checks to worker/native paths, including another valid endpoint in the same factory. Assert zero connector acquisitions. The source correctly funnels paths through `selected`/`Select`; the required executable evidence should verify that path guarantee rather than infer it from ordinary post-open case execution. The same endpoint-freeze requirement applies to DAQ routes.

The facilitator has acknowledged these findings and indicated a source correction/new freeze will follow. No corrected result is assumed here.

## Model-specific implementation observations

| Model | Actual source inspection and evidence boundary |
|---|---|
| HP5334A | Read full Rust `hp5334.rs`, C# `Hp5334.cs`, model spec and counter procedural proofs. The distinct5334A descriptor, A/B logical channels, A-only period, gate formatter/cache, explicit IN initialization, strict19-character/zero/overflow parser, health state and sync/native wrappers implement the stated subset. Local channel selection emits no traffic. No QueryIdentity, optionalC or global shutdown is advertised. Actual new-revision platform execution is still required. |
| HP5334B | Read the same shared engine against its own model spec/descriptor. Its identity is distinct, native/internal-gating commissioning rejects Unknown/CIIL before acquisition, no GAL or A-only MR/MS behavior is emitted. Source sharing is limited to documented equivalent counter operations and does not infer Option700 language. Separate56-case model shard is present. No source logic defect found in this pass. |
| HP3497A | Read full Rust/C# factory, inventory freeze, native AI acquisition and parser. Slot/local/native derivation and010None versus020Software/reference exclusion are explicit; no module probing or fallback endpoint exists. Request validation precedes native acquisition; strict scalar/positive overflow and120V software policy agree with current spec. Required evidence gaps are B-IMP-1/2/4, not an observed parser/topology logic error. |
| HP8920A | Read full Rust/C# RF model configuration, bounds, decimal comparison, transaction builders and decoders against own-model spec. A-specific007/008/016 and055 combined options,5dBm electronic duplex maximum,30MHz electronic receiver floor and frozen power constants remain distinct. Configure keeps output OFF and verifies readbacks; receiver invalidates readiness and repeats FULL/SING/new trigger. No core vendor command leakage found. B-IMP-3/4 apply. |
| HP8920B | Shared RF engine validates B.06.00+, rejects055+006/007 at registration, preserves3.5dBm electronic duplex maximum and10MHz/400kHz receiver floors, and uses B-specific supported option ranges. Electronic006 power carrier qualification remains a documented commissioning precondition, not a new hidden frequency measurement. B-IMP-3/4 apply. No ordinary invalid mechanical route remains in fixtures. |
| HP8921A | Shared RF engine validates A.15.00+ native syntax and Standard-only inventory; A/B options do not leak into it. RFG V/DBUV versus TX-power unit domains and30MHz power carrier condition remain distinct from10MHz/400kHz frequency measurement bounds. Own-model documentation supplies OFF-preservation/recovery/units evidence. B-IMP-3/4 apply. |

All RF models explicitly disable/verify the managed RFG output before CONFIG writes, normalize Manual/offset/attenuator settings, preserve modulation, require a separate successful configure before enable, and fail immediately on the first bad verification. The transaction engines mark health uncertain before host IO/await and restore it only after all decoding/commit succeeds. Pre-IO argument/unsupported rejection preserves usability; failed IO or decode leaves quarantine. Source state, token, numeric, selected-route envelope and decimal half-step rules are consistent across Rust and C# in the inspected code.

## Neutral API, host and consumer observations

- Read neutral `driver-operations.json` additions, generated Rust request validation/results and typed DAQ/RF class methods, C# neutral capability proof, and runtime operation/lease/quarantine handling. The six new operations carry typed channel/optional range, Hertz, Dbm and Boolean semantics. HP grammar/options stay external. Searched neutral API/runtime/SCPI implementation sources for HP model identifiers; none found.
- Read plain-text body path in Rust session/framing and corresponding .NET body API/standard VISA connector. Plain-text rejects binary wrappers before stripping, enforces selected terminator/one buffered frame and preserves payload spaces. Ordinary SCPI query remains separate. Source tests cover complete/split wrappers, alternate terminator, buffered extra frame, preservation and pending empty/incomplete response deadlines without retry/drain. This inspection supports the claimed binary-wrapper correction; final pinned SDK test execution must still be inspected.
- Standard .NET VISA composition defaults to ordinary SCPI binary framing for compatibility, while README explicitly requires `endpointScheme:"hpib"`,256-byte response bound,CRLF and `allowBinaryBlocks:false` for these native devices. The native drivers rely on the host complete-message contract for preallocation bounds, finite deadlines, clean initial framing and cancellation completion; they cannot prove arbitrary custom connectors satisfy it.
- Read both languages' `hp_async_proof`, counter/legacy procedural functions and fixture registration, plus the main consumer loops. Native tests actually wait on pending responses/deadlines and inspect active-access teardown; all three RF models also stall writes. Script cases execute through sync/native/worker-async/worker-sync paths, with ownership, closed-handle and worker-affinity checks. Shared runtime open-abandonment/worker lifecycle mechanisms are mapped to pinned SDK tests, not reimplemented in vendor code.
- Directly inspected corpus structure:1424 unique cases, comprising56 HP5334A +56 HP5334B +53 HP3497A +152 HP8920A +148 HP8920B +128 HP8921A =593 HP, plus831 regressions. Each model has its declared operation support and negative/error classes. Case presence is not execution proof.
- Read workflow pinning, Cargo/.NET package declarations, driver-package metadata and README. External code is statically compiled/rooted without reflection activation or runtime JSON. The source explicitly says existing publicalpha.1 packages lack these additions, current feeds are checksummed development bootstrap packages, hardware remains unvalidated, and coordinated future versions are necessary. Final archive contents/dependencies and source/packed native executable evidence remain pending.

## Outstanding evidence and disposition

No additional backend logic defect was demonstrated during this source pass. The concrete missing procedural observations above must be corrected and mapped truthfully. Confidence is high in these source findings and moderate in any unexecuted behavior.

PARITY/VALIDATION/DELIVERY cannot receive a final numerical ballot from this partial stage: hosted final-revision Linux/Windows source+packed consumer/AOT proof and producer SDK tests are pending. No hardware claims are made. Await the replacement frozen source revision and identical final evidence manifest; then reassess every category for every model rather than carrying these qualitative observations or prior scores as a final grade.
