# Independent HP acceptance fixtures

These native-message expectations were extracted from primary manuals independently of production backend source. Replies are synthetic. Schema generation does not establish backend, hardware, calibration or physical protection evidence.

| Suite | Cases | Qualified firmware |
|---|---:|---|
| HP3497A | 53 | Explicit Option001/module inventory and commissioning |
| HP8920A | 152 | A.18.00 and later same native family |
| HP8920B | 148 | B.06.00 and later same native family |
| HP8921A | 128 | A.15.00 and later same native family; manual08921-90031 |

The 481 cases use strict SDK schemaVersion1/contractMajor1. IDs join separate provenance JSON records. IDs contain canonical factory then route marker: DAQ D; RF R:E/M:input-power-option:source-port:receiver-input. Every owned case closes once with no device commands. Only the final value/error is compared exactly; preceding calls must succeed. Prior-error/quarantine, lifecycle and factory scenarios appear in `procedural-acceptance-matrix.md`.

RF transcripts group CONFIG writes before CONFIG queries after the first output-OFF verification. Source frequency writes use integer Hz; levels use invariant decimal tenths. Receiver calls repeat all preparation and a fresh trigger. Bounds remain model/option specific. `power-bound-constants.json` freezes inclusive binary64 dBm limits derived with 80 decimal digits, with hexadecimal and derivation provenance; immediately adjacent outside values are rejected.

The qualified8921A A.15 manual p81 expands the RFG UNIT domain to W/DBM/V/DBUV. Thus V/DBUV are Device when DBM expected on this source; TX-power V/DBUV remain Parse. The older A.12 manual has a W/DBM-only table and does not qualify an A.12 source route.

This v9 correction batch updates the3497A and8920B specifications;8920A/8921A spec,case and provenance bytes remain identical to the frozen v8 batch. The batch requires a new all-four Council review and never inherits old frozen review grades. Independent reviews in `review/` bind approval to exact new hashes. `validation-evidence.json` records successful neutral metadata checks, fixture/provenance/spec joins and SDK Rust/C# generation. Backend execution and procedural checks remain separate implementation requirements. Use the SDK generate-tests tool with the canonical catalog-case JSON inputs; generated language files are not authoritative source expectations.

V9 uses physically plausible positive VR1/VR2 replies. The ±120V fixtures are explicitly synthetic parser-policy boundaries, not physical output oracles. B Mechanical055 supports Standard/016 only: datasheet5968-1376E p9 explicitly excludes006/007. Conflicting numeric rows do not qualify an exception. Invalid configurations are procedural pre-open InvalidArgument cases with zero connector calls, not ordinary CatalogCases. A Mechanical055+007/008/016 remains separately qualified; C remains Standard only. Frozen485-case v8 references remain unchanged for audit.
