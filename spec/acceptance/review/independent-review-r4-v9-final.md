# Final independent specification review r4 — correction batch v9

**APPROVE the exact four-model v9 review batch below for a new Council specification review.** This is a fresh disposition on changed3497A/B artifacts; it does not inherit the prior v8 grade or claim any implementation/hardware proof. A/C byte identities were independently verified unchanged from the previously reviewed specifications. Existing5334A/B work remains outside this review, so the overall six-model semantic-API scope is preserved.

| Model | Specification SHA256 | CatalogCases |
|---|---|---:|
| HP3497A | `66870a951b15188d39b1126d272311860d97bd8e61122c210e5067a1d9df6fef` | 53 |
| HP8920A | `c3e32d3d577ea53e7b7a62fb6da55adb0d09b959cbbdd66b69d7db58f870be59` | 152 |
| HP8920B | `76391946421a0667707cb22d7132c63205f832b1dfd03789e1b90490c9e0d583` | 148 |
| HP8921A | `c0db97000cee047396776372aab0aab8143013c26562c6a123fba2d1a67e433c` | 128 |

The reviewed artifact root is `/workspace/scratch/instrument-components-hp/v9-drafts/spec/`. Exact specification/case/provenance hashes are recorded in `v9-final-dispositions.json`; all12 were independently checked against the revision manifest. The procedural matrix and README hashes are additionally bound in `v9-independent-check-evidence.json`. There are481 CatalogCases, not485:53 DAQ,152 A,148 B,128 C.

## Primary findings and corrections

The earlier independent review missed B's explicit complete-option exclusion and did not inspect every3497range positive fixture literal. Those defects are acknowledged here; the direct primary facts prevail over earlier approvals or majority grades.

I independently read and visually inspected actual8920B5968-1376E November2000p9. Its Option055 description expressly excludes006and007. V9 therefore allows Electronic Standard/006/007/016 and Mechanical055 Standard/016 only, rejecting055+006/007 before connector open. Same-document numeric rows under055 contradict that availability statement and do not establish an exception. I independently read the complete four-page May2002 configuration guide5966-1736E as well: it lists options and the mutually exclusive input-range choice, but nowhere explicitly permits the excluded combination or rescinds the earlier restriction. No qualifying exception was found. This is a conservative first-release support boundary, not a claim that arbitrary historical modifications are physically impossible.

The restriction remains model-specific. Actual8920Ap2 explicitly names its007with055,008with055,016with055 configurations; its complete option sectionsp6–7 and selection matrixp8 do not impose the B exclusion. V9 correctly retains those A combinations and keeps8921A Standard-only Electronic/055 scope. B's supported source numeric envelope continues to use its mainp3 table; unrelated copied numeric rows onp9 do not change the016 source maximum. Receiver/power bands exclude all unavailable B combinations.

I independently read and visually inspected3497A printed167/PDF174 and read168/PDF175. The documented max displayed magnitudes are0.119999/1.19999/11.9999/119.999V forVR1–VR4, with20percent overrange. The new VR1 literal+1.25000E-2=0.0125V andVR2 literal+1.25000E-1=0.125V fit their selected ranges. A1.25V successful positive afterVR1/VR2 is no longer present. The±120V cases have parser-policy IDs and explicit provenance disclaiming a physical displayed-value oracle; the documented positive overflow sentinel remains+9.00000E+9. Parser behavior itself was deliberately preserved as a separately labeled software policy.

## Independent artifact checks

All3497Valid scalar fixture values were checked against their raw11-byte grammar, expected numeric quantity and selected fixed-range maximum; only the two explicitly labeled±120 parser-policy cases are nonphysical boundary oracles. Case/provenance joins and reconstructed-spec hashes match for all four models; every owned case ends with exactly one resource close.

B no longer contains any Mechanical055+006/007 execution case or positive factory-route record. Four impossible-route execution cases were removed. Negative factory-construction/open outcomes belong in the procedural matrix because the CatalogCase schema describes operation outcomes after route admission. The matrix now expressly requires InvalidArgument before connector, zero connector calls across sync/worker/native modes, and exact selected-endpoint isolation even when another endpoint is valid. Those remain concrete implementation acceptance obligations; this review did not execute an absent backend.

The existing sourceOFF/config/readback ordering, fresh FULL/SING receiver transaction, exact native grammar, per-endpoint freeze/capability rules, numeric unit/overload/status rules, finite deadlines with real stalled IO, cancellation/quarantine, owned/borrowed lifecycle, genuine async IO, external-package boundary and Linux/Windows source/packed actualNativeAOT evidence requirements remain intact. No model or semantic operation was silently dropped. Hardware validation and new backend implementation evidence are still pending.

No unresolved specification blockers remain for these exact artifacts. Any further material edit requires a new hash-bound disposition. No backend/source implementation was edited or reviewed. Retained primary-inspection notes are in `v9-primary-precheck.md`, and independent artifact-check evidence is in `v9-independent-check-evidence.json`.
