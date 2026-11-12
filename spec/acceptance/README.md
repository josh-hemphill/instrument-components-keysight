# Independent HP acceptance fixtures

These native-message expectations were extracted from primary manuals independently of production backend source. Replies are synthetic. Schema generation does not establish backend, hardware, calibration or physical protection evidence.

| Suite | Cases | Qualified firmware |
|---|---:|---|
| HP3497A | 53 | Explicit Option001/module inventory and commissioning |
| HP8920A | 152 | A.18.00 and later same native family |
| HP8920B | 152 | B.06.00 and later same native family |
| HP8921A | 128 | A.15.00 and later same native family; manual08921-90031 |

The 485 cases use strict SDK schemaVersion1/contractMajor1. IDs join separate provenance JSON records. IDs contain canonical factory then route marker: DAQ D; RF R:E/M:input-power-option:source-port:receiver-input. Every owned case closes once with no device commands. Only the final value/error is compared exactly; preceding calls must succeed. Prior-error/quarantine, lifecycle and factory scenarios appear in `procedural-acceptance-matrix.md`.

RF transcripts group CONFIG writes before CONFIG queries after the first output-OFF verification. Source frequency writes use integer Hz; levels use invariant decimal tenths. Receiver calls repeat all preparation and a fresh trigger. Bounds remain model/option specific. `power-bound-constants.json` freezes inclusive binary64 dBm limits derived with 80 decimal digits, with hexadecimal and derivation provenance; immediately adjacent outside values are rejected.

The qualified8921A A.15 manual p81 expands the RFG UNIT domain to W/DBM/V/DBUV. Thus V/DBUV are Device when DBM expected on this source; TX-power V/DBUV remain Parse. The older A.12 manual has a W/DBM-only table and does not qualify an A.12 source route.

The specifications in `../legacy/` are reconstructed v8 documents and never inherit old frozen review grades. Independent reviews in `review/` bind approval to exact new hashes. `validation-evidence.json` records successful neutral metadata checks, fixture/provenance/spec joins and SDK Rust/C# generation. Backend execution and procedural checks remain separate implementation requirements. Use the SDK generate-tests tool with the canonical catalog-case JSON inputs; generated language files are not authoritative source expectations.
