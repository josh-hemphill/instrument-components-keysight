# Independent specification review — expansion 2026-11-13

Reviewer: expansion_spec_reviewer. This is an independent source/manual review before Council grading; it is not a Council ballot or hardware validation. Baselines and reviewed hashes are in reviewed-inputs-preliminary.json. Production source was not edited.

## Scope and method

Requested:34401A,33120A,53132A,3585B,34980A and34937A/34922A/34950A modules,TDS1002A. I inspected current neutral request/class types, actual34401profile and common external message engines, actual downloaded own-model34401/33120/53132/34980/34922/34950 manuals, first proposed34401/33120/53132 specifications, and primary Tek model applicability. Direct feedback was sent to extractor and facilitator while drafts evolved. Independent preliminary review is not a substitute for each-device Council review on frozen inputs.

## Confirmed material corrections

- 34401 existing generic profile falsely inherited TEMP, directUSBidentity, missingHP identity alias and ordinaryValid overload decoding. The proposed replacement explicitly qualifies its smaller real scalar surface, defers split-phase guessing and removes those claims. Own-manual details and required tests:34401A-AUDIT.md.
- 33120 has always-live output, noOUTP ON/OFF, noPulse,50ohm/HighZ accounting only. The proposed factory limits initial mutation to continuousSine/50ohm/VPP with verified function/load/unit and allfive modulation/burst/sweepflags. This is appropriate narrower support; do not claim outputdisable or transient-free switching.
- 53132 must reapply gate afterCONF and use its FREQ:ARM:STOP:TIM subtree, not adjacent53131genericAPERture guesses. Timegrid specification now usescanonicalbinary64 millisecond fractions andintegerm wire formatting, avoiding spurious rejecting validms values. Own source-list and period differences are evidenced; optional3rdchannel staysdeferred.
- 34922 is70channelstwo35banks, not anotherfamilymodule's40/80topology;1..35 and36..70 arelatching and retainpoweroffstate. Independent physicalchannel operations are required; existingmatrixroute(ch1,ch2) mustnotbe filled with inventedendpoints.
- 34937 has1..28FormC and29..32FormA; Open describesNOpathopen butdoesnotpromiseFormCNCdisconnected. No globalOpenAll onpartialinventory.
- 34950 read-SENSEonOUT returnslastoutputlatch, notpininput; writingSOURCEautomaticallyswitchesOUT. Width changes altergrouping andclearmemory. An initialBYTE/input-or-output/unbufferedbinding mustnot silentlychange direction/width/modes. Verify selected width/direction/polarity andbankmemory/handshakestate as specified, thenreadorwrite. Handshakequery isCONF:DIG:HAND:STAT? onbankfirstchannel, returningHIMP/OFF/ON, notNONE. Byte addresses101..104/201..204.
- 34980moduleviews mustshareonefactory/onephysicalhandle/oneSDKserializationlease, so aqueryfailure quarantinesallsiblingviews. Do notopenpermoduleconnectionssimultaneouslyagainstsamebusendpoint andpretendoperationserializationcoversmainframeglobalstate.
- ExactTDS1002A identity remainsunverified. OfficialTek model applicabilitylistsTDS1002 andTDS1002B, notA; nonB interfaces needoptionalCM/CMA/CMAX orMEM, BhasUSB. A cannotinheritB'sinterfacebyname. HoldaTDS1002A supportclaimuntiluser/model-specificprimaryevidenceresolvestheidentity; workonverifiedmodelscontinues.

## Remaining required concreteness in first packet

At the time of preliminary inspection,33120 still needed an unambiguous write-only setter contract, exactReadFrequencysettingbounds/errorclass, and distinction between structurallyinvalidarguments rejectedpreIO versuscoupledconstraints checkedafterquery. E11formattermustchecktheformattedwireparameteragainstcoupledconstraints, notonlytheoriginalbinary64argument. These were sent as concrete corrections; update this record after the extractor finalizes the packet.

34401 range policy mustbeoneconsistentchoice; currentrevisedversionchoosesfinitepositiveexpectedinput<=functionmaximum, ratherthanclaimingonlynominalvaluesarelegalSCPI. ExplicitDEF parameters andfixedscalar grammarare suitable. Its giantreplycutoff1e30 isclearlysoftwareplausibility, notphysicalinputprotection. 53132 zero/negativequantityDevice versusmalformedParse is nowexplicit.

## Acceptance conditions for subsequent implementation review

Static registration has no implicitprobe/reset/configuration. Exactendpoint commissioning/module/options mustbefrozenbeforeIO. Contextuallyunsupportedroute/mode rejects before acquisition. Invalidlocalintent leaves priorintentunchanged andhandleusable. Wholetransactionsserialize; activeuncertainIO/parse/knownwrongreadback/cancellationfailfast andquarantine acrossclassviews. Ownedcloseonce/borrowedpreserved; noautomaticdevicecleanup. Actualpendingdeadline/cancelproofs mustnotbesubstitutedbyinjectedTimeout-onlycases. Independently authored golden cases joinmanualpages andeachmodelidentity. Rust/C# andsync/native/worker behavior mustagree. Packedconsumers andactualNativeAOTLinux/Windowsx64/net8/net10 executions remainrequiredforcompletedimplementationclaims. No hardware,calibration,physicalprotection orpublicreleaseclaim withoutseparateevidence.

## Status

Research/extractionreview inprogress; firstpacket canfreeze after the concrete33120items areclosed. 3585B/34980module specifications and exactTek model evidence awaitseparatereview. No requesteddevice is silentlyremoved fromscope; TDS1002A is explicitlyblocked byidentityevidence ratherthaninvented support.

## First packet follow-up

The extractor resolved all listed first-packet concreteness issues, including the global33120offset4V bound, bothrequestedandroundedwirecoupledchecks,write-onlycompletion,readfrequencyenvelope, andpost-queryInvalidArgument/quarantinepolicy. I independently inspected the finalizedthreeMarkdownspecsandall185goldencases/generators(69DMM/50counter/66generator), checkedtheiruniqueIDs/exclusiveexpectedoutcomes andtheliteralprotocol/units/parser/errorbindings. No demonstrated mandatory defect remains in this declaredfirst-packet softwaredesign. They are clear for independent Council specification review, not implementationapproval. Exact reviewedhashes andlimitations are in first-three-clearance.json. The othermodels/modulesremaininreview; Tekidentityisstillpending.

## First packet v2 and normative construction follow-up

Independently checked33120v2context-specificHighZ9.9e37Deviceguardagainstactualmanualprinted151/PDF153,theninspectedtheadded67thcaseandunchanged66cases. ReviewedthecurrentCOMPOSITION-AND-VALIDATION.mdconstructor,bindingandQ-PROC01..12softwareacceptancecontractsandactualCLI/source-proofhelper. No demonstratedmandatorydesignfinding;clearforCouncilunderspecifiedsoftwaredesignscope. Exacthashesareinfirst-three-v2-clearance.json. Publication,packed/nativeexecutionandbenchqualificationremainfutureimplementationevidence;theseplansdonotclaimtheyhaveexecuted.

## Module packet follow-up

Independently inspectedthefourmoduledrafts,theconcretehp34980publicconstructionandMODULE-COMPOSITION.md,the195goldencases/generatorandfixturemapping. EarlierwidthquerytokenassumptionwascorrectedbynumericReturnFormat1/2/4;allbankwidthguardsnowexcludeoverlap. LifetimeNoAutonomousActionscommissioningcoversmonitor,route-sequencesandexternallyarmedmemory/interruptactions,currentguardscannotprovefuturetriggerabsence. No demonstratedmandatorydesignfindingremainsinthestatedswitching/staticbyte subset. It isclearforCouncil spec review;exacthashes/primarychecks/limitationsareinmodule-clearance.json. DMMacquisition/scanremainexplicitlydeferredandphysicalstateunverified.

## 3585B packet follow-up

The independentownmanual/renderedpagecheckand123goldencasesconfirmthenative8operationdeclaredsubset. Positiveintegercenter/spanadmissionmatchesexistingneutralcontracts;zero/fractionalhardwarefeaturesaredeferred. CLvisibility/T6selectedpointvalidityisdistinguishedfromSPstoredtracepeakandfull-sweepfreshness. Exact24charD2diagram-derivedrecordsareclearlysynthetic; noactualwireobservationswerefoundorclaimed. B-PROC01..07andconcretefactory/commissioningcloseearlieradmission/lifetimegaps. No demonstratedmandatorydesignfindingremains;clearforCouncil specreview. Exact3filehashes/limitationsarein3585b-clearance.json.
