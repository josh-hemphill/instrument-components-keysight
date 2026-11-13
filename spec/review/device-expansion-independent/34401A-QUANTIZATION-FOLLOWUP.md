# 34401A low-ohm ordinary fixture: quantization follow-up

Independent reviewer: expansion_spec_reviewer. Requested follow-up concerns three non-nominal RES/FRES golden cases: expected range0.025Ohm, default resolution, and synthetic reply +1.25000000E-03Ohm. No production or frozen specification/oracle files were changed.

## Decision

**No demonstrated impossible ordinary reply on the available primary evidence.** The preliminary inference "default5.5digits means a hard1mOhm remote-output grid on the100Ohm range" is not established by the actual manual. Do not promote that inference into a mandatory factual defect or change the parser to reject sub-display-step readings.

A more conservative ordinary synthetic reply of1.25Ohm could be chosen as optional fixture polish, preserving the same low expected-range input and range-selection test. That would not change API/parser support; it would need a new separately versioned oracle/helper/provenance and affected Council review. It must not be described as fixing a proved impossible hardware response unless new evidence supplies that proof.

## Exact primary evidence

Primary: Keysight34401A User's Guide,34401-90004,Edition10,August2014. ActualPDF SHA256 ea7ec922d14aafce23ec7058e37bcf0ccd192423f70d75ccdf6350382ae3b419. Official raw source https://www.keysight.com/content/dam/keysight/en/doc/gate/user-manuals/9018-01063.pdf.

| Printed / PDF page | Observation | Implication / limit |
|---|---|---|
|17 /25|Resistance nominal ranges begin at100Ohm.|An expected input0.025Ohm cannot select a lower listed physical range.|
|21 /29|Arrow keys may alter displayed digit count without changing integration time.|Display digit count alone does not establish remote conversion/serialization quantization.|
|54 /62|Default Slow5Digit and Fast6Digit entries both use10PLC.|The5.5-versus6.5 display/mode labels do not map one-to-one onto different integration precision.|
|55 /63|100Ohm example displays113.3256Ohm at6.5digits. DC/resistance digit selection can also affect integration.|This front-panel example illustrates display placement; it does not specify a hard remote-output rounding algorithm at defaultresolution.|
|57 /65|10NPLC measurement resolution is0.000001 timesfullscale.|On nominal100Ohm fullscale this is100microohm, not the claimed1milliohm. "Resolution" still does not explicitly define an exact allowed lattice for serialized calibration-corrected readings.|
|114 /122|The MEAS/CONF range parameter is expected input and selects the appropriate physical range.|Low non-nominal positive expected values remain a legitimate range-selection test under the chosen software policy.|
|117 /125|DEF resolution is5.5digits slow,10PLC.|Confirms default mode; does not override the10PLC resolution table or add a remote-output lattice.|
|159 /167|Single-reading output has a fixed signed scientific grammar with eight fraction digits.|Specifies representation width and terminators, but does not state that particular low-order mantissa digits must be zero for a given front-panel display selection.|

The fixture value0.00125Ohm is below a1milliohm display grid and is not an integer multiple of100microohm either. Neither fact alone proves impossibility because the manual does not state that wire values are quantized exactly to those steps. No calibrated bench trace or own-model primary statement resolving remote-output rounding was found in this follow-up. Existing packet records remain explicitly synthetic and unobserved on hardware.

## Review status

Keep the existing independent design clearance unless new direct evidence demonstrates a defect. If the facilitator chooses the conservative1.25Ohm fixture edit, preserve old frozen files, update only the three ordinary resistance examples in a new oracle and its independent authoring helper/provenance, and re-review that version. Deliberately synthetic parser-policy examples must remain separate and need not be electrically observable.
