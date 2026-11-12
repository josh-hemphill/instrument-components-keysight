# Procedural acceptance matrix accompanying recovered CatalogCase drafts

These checks require runtime/factory/lifecycle/cancellation control beyond CatalogCase's scripted calls and single final outcome. They are independent acceptance requirements, not claims of completed testing or hardware validation. This v9 correction batch requires a new all-four Council review; unchanged A/C v8 bytes retain their identities without substituting prior grades. Shared golden suites carry synthetic manual-derived transcripts and resource-only owned close. No production source was inspected to derive expected bytes.

## DAQ immutable inventory and opening

All invalid inventory scenarios must raise InvalidArgument before connector open (zero connector calls):

| Scenario | Required observation | Primary evidence / policy |
|---|---|---|
| Option001 absent/unknown | Reject; no voltage capability | 03497-99000 p19,150; explicit inventory policy |
| Empty or unknown inventory | Reject; no inferred modules | p135–136 SR code7 analog OR empty |
| Normal DVM routing false/unknown | Reject | p184–185 normal JM1–JM6 routing |
| Duplicate publicID | Reject | Immutable mapping uniqueness policy |
| Duplicate native address | Reject | Immutable mapping uniqueness policy |
| Slot−1/5;local−1/20;native−1/100 | Reject | p45–46 mainframe5slots×20 |
| Native not20*slot+local | Reject | p45–46 allocation arithmetic |
| Same slot conflicting010/020 or compensation | Reject | Inventory topology consistency policy |
| Option010 with Software/Hardware/Unknown compensation | Reject | Scope policy;010 raw inputs |
| Option020 Hardware/Unknown compensation | Reject | p191–194 hardware adds compensation voltage |
| Option020 Software local19/referenceB9 | Reject user-input mapping | p191–193 reference junction reservation |
| Unsupported070/071,extender,unknown module | Reject voltage capability | p45;scope excludes conditioning/extenders |
| Caller mutates records/maps after factory construction | Deep-copied/frozen inventory remains authoritative | Immutable configuration policy |
| Wrong exact manufacturer/model;unsupported scheme | No matching/fallback/probe | Static matching policy |
| Application connector cannot commission clean boundary | Reject open before returning IO | No automatic stale-boundary proof policy |

Golden positive configuration is Option001+normalrouting+boundaryconfirmed,public7→slot0/local0/native0/010None;13→slot0/local19/native19/010None;29→slot1/local18/native38/020Software;61→slot4/local19/native99/010None. Public channel is u32 in catalog metadata: negative values are unrepresentable through typed CatalogRequest; do not emit a negative channel CatalogCase.

## RF endpoint inventory and opening

Every model requires exact immutable endpoint binding; all invalid scenarios reject before connector open. Identity is application supplied; no ID/option query/reset occurs at matching/open.

| Scenario | Required observation | Evidence / policy |
|---|---|---|
| Unbound endpoint or mismatched model route | Reject; no default source options | v8 immutable endpoint routing |
| Duplicate normalized scheme+exactaddress endpoint | Reject | Unique route policy |
| Routecount4097 | Reject;4096 maximum | Bound policy |
| Scheme HPIB/hpib | Same normalizedscheme | ASCIIcase policy |
| Address case/text differs | Exact address remains distinct;no guessedalias | Exact address policy |
| A Option006;B Option008;8921A any006/007/008/016 | Reject unsupported option | Model-specific datasheet source tables |
| Unknown attenuator/inputpower/sourceport/receiverinput enum | Reject | Frozen explicit finite configuration |
| 8920A Mechanical055 with Standard/007/008/016 | Accept; independently named combined configurations | 5968-5385E p2 and full option descriptions |
| 8920B Mechanical055 with Standard/016 | Accept | 5968-1376E p9 |
| 8920B Mechanical055+006 or+007 | InvalidArgument at registration/open before connector; zero connector calls in sync/worker/native modes and exact selected-endpoint gating even when another route is valid; no ordinary request attempted | 5968-1376E p9 explicit availability exclusion overrides conflicting numeric rows |
| 8921A Electronic or Mechanical055 with Standard only | Accept; do not inherit A/B power options | 5968-5820E |
| Below-minimum revision,unknown format or wrong native firmware family | Reject before connector open;accept documented minimum and later same-family native revisions | Manual applicability;no exact-baseline narrowing |
| 8921A firmware older than A.15.00 | Reject full source/receiver route before connector open | Own-model08921-90031 A.15.00+ p43 proves OFF preservation,p61 SDC ordering;p81 expanded RFG units |
| Any commissioning assertion false/unknown | Reject | Manual-switching-before-screen,frequencydisplay,independentRF,PTTinhibited,exclusiveaccess,cleanboundary/nativeGPIB |
| Selected ANT route requires RfReceiverMeasurePower | UnsupportedOperation before connector,even anotherroute supports power | ANT has no TX-power operation |
| Caller mutates supplied route list/records after registration | Selected frozenroute remains authoritative | Deep-copy/freeze policy |
| Connector sync/worker/native path absent | Do not advertise unsupportedmode | Execution-mode truthfulness |

Power endpoints are frozen authoritative nearest-binary64 constants derived with80decimaldigits;test immediate next-representable outside endpoints as Parse;do not recompute platform log10. Native firmware format is exactly A.dd.dd or B.dd.dd permodel,ASCIIdecimaldigits,uppercasefamilyletter,no whitespace/sign/extra fields;compare integer revision components. Test equalbaseline,oneincrementlatermajor/minor,belowbaseline,wrongfamily and malformed/unknown strings. Firmware baselines in golden route provenance are8920A A.18.00;8920B B.06.00;8921A A.15.00. Each route records explicit commissioning confirmation. Source/receiver envelopes are model/option-specific and are not physical input protection.

## Faults, deadlines, cancellation, quarantine and ownership

Run these for DAQ and all3RF models in Rust/C# sync,worker,nativeasync where applicable:

1. Constructor/connector deadlines must be positivefinite;rejectzero,negative,NaN,infinity. Bound complete response before allocation;golden host limit256bytes.
2. Actual stalled write/read/incomplete-response adapter must expire finite operationdeadline and return Timeout,stop hostaccess,and quarantine without retry/deviceclear/reset/abort. Immediate injected Timeout cases do not satisfy this stalled test.
3. After injected Timeout,Transport,partialresponse,Parse,Device/readback failure,or activecancellation,any follow-up operation must raise SessionNeedsRecovery before moreIO. Verify firstfailure exacttyped classification separately;CatalogCase cannot express an earlierfailure followed by a secondoutcome.
4. Queued/pre-IO cancellation causes noIO and leaves session usable. Active cancellation stops hostaccess and quarantines. Preserve native cancellation contract;do not fabricate an errorenum absent from catalog.
5. Unsupported/pre-IO InvalidArgument leaves session usable and sends noIO. Validate all configure arguments before even firstscreenwrite.
6. Recovery unsupported;draining/unchangedbuffer/readingoneoldresult must not silently restore state. Application recovery/reopen must recommission cleanresponseboundary and RF sourceconfiguration.
7. Ownedclose exactlyonce under repeateddispose;borrowedclose zerotimes. Failedopen after acquiring ownedIO releasesonce;borrowedresource preserved. Close sends nodevicecommands and makes noglobaloutputclaim.
8. Genuine nativeasync adapter must fail if syncIO invoked. Cancellation and deadline operate on nativeasync calls;dedicatedworker preserves typedoutcomes.
9. Concurrent source/receiver/DAQ operations serialize complete transactions under onelease;no interleavedmessages.
10. RF power-only transactions contain no extra frequencymeasurement;carrierfrequency/powerband is commissioningprecondition. RF repeatedreceivergoldens require twofulltransactions and twoTRIG:IMM. Sourceconfigure→receiver→enable must block enable until reconfigure.
11. RF hostnumeric nonfinite arguments cannot be encoded as legalJSON CatalogCases;procedurally pass NaN/±Infinity to sourceconfigure and require InvalidArgument beforeIO. Negativezero/noncanonical source argument formatting must not introduce scientificnotation onwire;integerHz and canonicaldecimal0.1dB writes are fixed goldenpolicy.

## Delivery evidence still required

Run generator/API/runtime/SCPI/vendor regression suites. Execute independently composed source/generated and packed consumer binaries on Linuxx64/Windowsx64 for Rust and .NET NativeAOT with trim/AOT warnings aserrors. Retain commands,exitcodes,source/artifacthashes,SDKpins,archive/dependency inventories and staticregistration evidence. Do not infer anotherdevice's acceptance from counterCI;do not claim hardwarevalidation from these fixtures.

## V9 correction audit

Frozen v8 had485 CatalogCases. V9 has481: DAQ53,A152,B148,C128. Four B execution cases using unavailable Mechanical055+006/007 routes were removed and replaced with the explicit zero-connector factory cases above. Those factory failures cannot be expressed by the CatalogCase last-call outcome schema. A/C case/spec/provenance bytes remain unchanged. DAQ VR1/VR2 positive literals now fit the selected range;±120V cases explicitly test synthetic parser policy rather than a physical displayed-value oracle.
