# Final independent recovered-specification review r3

**APPROVE all four exact specifications below for Council specification review.** This supersedes earlier independent dispositions. This review is separate from Council and makes no implementation or hardware-validation claim. Existing HP5334A/B work was excluded from this review.

| Model | Final SHA256 |
|---|---|
| HP3497A | `300bc77fbfdf0af243db44f913250cf1b4ad179ec3bdf55abf33050ad454e7e3` |
| HP8920A | `c3e32d3d577ea53e7b7a62fb6da55adb0d09b959cbbdd66b69d7db58f870be59` |
| HP8920B | `43c1ad235ce6a805473995f9c9c90886b9db7e20328d056d2b27bb3c3700cab6` |
| HP8921A | `c0db97000cee047396776372aab0aab8143013c26562c6a123fba2d1a67e433c` |

The technical checks and primary-page evidence recorded in `independent-review-r1.md`, `independent-review-r2.md` and `8921a-1998-primary-check.md` form this review's evidence trail. r1's exact-baseline firmware suggestion was superseded by the root's documented-minimum/native-family requirement; final revision parsing is deterministic. The final 3497A document includes explicit endpoint binding, deep-frozen inventory and no fallback. Numeric grammar, readback plausibility, inclusive decimal tolerance comparison and shared binary64 power constants are explicit software policies, independently checked against the physical/source matrices and high-precision conversion.

HP8921A is qualified for the full first-release route only on A.15.00 and later well-formed revisions in its native family. Its actual own-model08921-90031 December1998 RevA complete PDF was freshly recovered and independently checked, including numeric-OFF preservation p43, SDC-before-termination p61, RFG UNIT expansion p81 and the exact CONFIG/RFG/RFAN/MEAS/trigger syntax and freshness pages. Older A.12.01 evidence is historical and does not grant an A.12 route. All three RF specifications now correctly classify V/DBUV as known wrong RFG UNIT replies→Device and unknown TX-power UNIT replies→Parse. The new8921A manual's discovery after environment replacement is accurately identified rather than attributed to lost pre-replacement work.

The manual-specific model/option matrices remain distinct: A007/008/016, B006/007/016, CStandard; A/B/C source and receiver minima are not conflated, B006 power has30MHz minimum, and8921A power retains30MHz even though electronic receiver frequency starts10MHz. The3497A scope remains001DVM with010None local0–19 or020Software local0–18, excluding B9/reference19 and Hardware/Unknown compensation. Sources use dBm. All RX calls require new FULL/SING trigger transactions. No globalShutdown, fabricated overload, inferred topology, unsupported automatic recovery or cleanup is introduced.

Fresh response framing/commissioning, finite operation deadlines and actual stalled IO checks, cancellation/quarantine, lease ownership and owned/borrowed lifecycle, true asynchronous host IO, source/generated and packed Linux/Windows consumers and actual NativeAOT binaries remain mandatory implementation acceptance obligations. They must be demonstrated by new-backend artifacts; earlier counter CI does not satisfy them. Manuals and simulators are not bench-validation evidence.

No unresolved specification blockers remain for these exact hashes. Any later substantive edit requires a new independent hash-bound disposition. No production source was edited or reviewed by this reviewer. Machine-readable dispositions are in `final-dispositions.json`.
