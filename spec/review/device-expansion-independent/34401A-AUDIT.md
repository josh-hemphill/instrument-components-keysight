# Independent 34401A source/manual audit

Reviewer: expansion_spec_reviewer. No production edits or hardware access. Source baseline core1b0704d / Keysightc63cc02. This is an evidence review, not a Council ballot.

Primary actual manual: Keysight34401A User's Guide,34401-90004,Edition10,August2014; research/34401A-user.pdf SHA256 ea7ec922d14aafce23ec7058e37bcf0ccd192423f70d75ccdf6350382ae3b419. PDF page=printed page+8 for cited operational pages. URL obtained by research: https://www.keysight.com/content/dam/keysight/en/doc/gate/user-manuals/9018-01063.pdf.

## Demonstrated inherited-profile defects

1. Current keysight.34401a advertises DmmMeasureTemperature=>:MEAS:TEMP?. The own-model command summarypp104–105 and supported function listp121 contain no temperature operation. Do not retain that capability or infer34410A commands.
2. Inherited VID2391/PID1543 is not evidence of a34401A USB interface. Own-model interface selectionpp92/162 permits GPIB/RS232, notUSB. Remove the directUSB match rule for this factory; external USB-GPIB/serial adapters do not establish a manufacturer/product match for the instrument.
3. Actual identity responsep89 is HEWLETT-PACKARD,34401A,0,XX-XX-XX. Existing identity rules listAgilent/Keysightonly; add verifiedHP identity spelling for the new qualifiedfactory and do not register twoequalpriorityfactories for the samekeysight.34401aID.
4. Generic profile decoder returns every finite numeric replyValid. Manualp61 uses9.90000000E+37 foroverload; a qualifiedbackend must returnOverload in the requestedunit, notValid9.9e37. Negative sentinels/magnitude-onlyheuristics lackmanualevidence.
5. GenericRead/Fetch can useunknownphysicalunit orstale splitphasestate. A newqualifiedscalarMEAS-onlysubsetavoidsclaimingunverifiedsplitphaseconfiguration; if configure/read/fetchretained, explicitlymodel configuredfunction, INITreadiness andsingle sample/count, do notguessunitsfromkind.

## Verified useful scalar subset

Own-modelMEAS:VOLT:DC?,MEAS:VOLT:AC?,MEAS:CURR:DC?,MEAS:CURR:AC?,MEAS:RES?,MEAS:FRES? support6physicalfunctions; DmmMeasureResistance andResistance2Wire are equivalentRESaliases. All7neutralmethods canbeoffered. Manualpp117–118(PDF125–126) suppliesexactgrammar. Explicitnominalrangepolicy is acceptablewhen labeledsoftwareAPIrestriction:DCV.1/1/10/100/1000V;ACV.1/1/10/100/750V;DCI.01/.1/1/3A;ACI1/3A;R100/1k/10k/100k/1M/10M/100Mohm. Omittedrange=>DEF(auto); omittedresolution=>DEF. Use explicitDEF,DEF toavoid relyingon omittedparameterbehavior, orcitecommandexamples foromission.

Themanualp114describesrangeasexpectedinputandautoselectsthecorrectphysicalrange; acceptingonlynominalvalues wouldbeintentionalnarrowing, not a claim thatotherpositivevaluesareinvalidSCPI. Explicitrangevalidationmustthereforebe documented consistently.

MEAS? automaticallynormalizesmathOFF,immediatetrigger,onesample,onetrigger,automaticdelay,20HzACfilter and10MohmDCinput(defaults tablep112/PDF120). Nosystemresetneeded. Eachmeasurementisfresh; don'treuseFETChasMEAS.

Strict scalarframing: manualp159/PDF167 defines signed scientific body SD.DDDDDDDDESDD, newlineonGPIB, CRLFonRS232. HereD=ASCII digit,S=+/-; oneintegerdigit/eightfractiondigits/E/signedtwo-digitexponent. Amanualsentinel9.90000000E+37alsoappearswithoutleadingpositive sign; supporttheexactdocumentedpositiveoverloadrepresentation(s), notanylargefinitevalue. Rejectembeddedcomma/multiplevalues,whitespace,binarywrappers,trailingextra,lowercase/non-ASCII/NaN/Inf perdeclaredgrammar. Result status/unit mustmatchrequest.

Physical/software plausibilitylimits must distinguishoverrangefromsafetylimits: specspp216–219permit20%overrangeexcept1000VDC,750VACand3Aranges. Do not mechanicallyapply1.2 to thosehighestvoltage/currentlimits. Anyparserplausibilityfilteris softwarepolicy, not inputprotection oraccuracyvalidation. Resistancemayinclude smallnegativeoffset/readings; donot assumeallresistancevaluesmustbe>=0withoutseparatemanualevidence.

## Admission and host requirements

TheinstrumentselectsSCPI,HP3478AorFluke8840A language persistentlypp94/165–166. Require affirmativeSCPIcommissioning before acquisition; do notsendL1silently. GPIB/RS232framing andproperremote state mustbeestablished bytheapplication. RS232 additionallyneedsSYST:REMproperremotemode,matchingbaud/parityandDTR/DSR(7or8databits,1start/2stopbits)pp148–153; thefactorydoesnotconfigure serialportorpretendUSBnative support. A firstsubsetrestrictedtoexplicitGPIBcommissioning ishonestif documented.

Noimplicitidentityprobe/reset/calibration/selftest/deviceclearorlocalmodecommand. p160definesdeviceclear/serialCtrl-Cforabort, butautomaticrecoveryrequiresbus/hostsemanticsoutsidethecurrentmessageinterface. Failfast quarantineandexternalrecommission/reopenareappropriate. ResourcecloseisnotshutdownandpreservesborrowedIO.

## Necessary software coverage

Separatepositivefixtureforeachof7neutraloperationsandallnominalranges/auto,positive/negative/zero,exactoverload,wronggrammar/extraresponse; unsupportedTEMP/configure/read/fetch rejectionasdeclared;invalidrange/nonfinitepreIOandthenvalidoperation. Verify identityaliasesandabsenceofUSBrule,staticcatalogmetadata,borrowed/ownedclose,activecancel/pendingdeadline/quarantine andsync/native/workerRust+C#parity. Packedsource/artifactAOTmatrixremainsrequired; simulationcannotdemonstratephysicalinputbehavior.
