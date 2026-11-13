# TDS1002 research supplement — confirmed non-B model

The user corrected the requested model to **TDS1002**, without A or B. This supplement does not modify the previously frozen Agilent packets. Four actual official Tektronix PDFs and their SHA256 values, page counts, part revisions, applicability and source URLs are recorded in [TDS1002-SOURCES.json](TDS1002-SOURCES.json). They were downloaded from `download.tek.com`, checked for a PDF signature, and extracted locally. Primary sources are the own-applicable programmer manual **071-1075-04** and user manual **071-1064-00**. Later programmer manual 077-0444-01 and specification supplement 071-1858-01 provide corroboration; do not import B/C/TPS-only commands from them.

All page references below are **one-based PDF pages**. No hardware was connected. This is source research, not an approved specification or implementation.

## Identity and communications

User-manual page 20 identifies TDS1002 as a two-channel, 60 MHz, 1 GS/s monochrome scope. Pages 21 and 145 describe the optional TDS2CMA communications module. Programmer pages 17, 18 and 21 together establish:

| Installed module | TDS1002 applicability | Programmable interfaces |
|---|---|---|
| TDS2CM | Yes; cover and compatibility table include it | GPIB and RS-232 |
| TDS2CMA | Yes | GPIB and RS-232 |
| TDS2CMAX | Yes | GPIB and RS-232 |
| TDS2MEM | Yes for TDS1002; exceptions are TDS1001/TDS2004 | RS-232 only; no GPIB |
| TDS2MM | Not a supported TDS1000 module | Do not infer support from its TDS200 capabilities |
| None/unknown | No established PC control interface | Reject before connector acquisition |
| TEK-USB-488 | Not a non-B TDS1000 accessory | Do not claim B-model USB or this adapter for TDS1002 |

A GPIB-only initial factory can truthfully accept typed CM/CMA/CMAX commissioning and reject MEM, unknown and none before IO. Document MEM's legitimate RS-232 path as outside that initial subset. The programmer cover supports any CM/CMA version on TDS1000; its CMV1.04/FV1.09 restrictions apply to older TDS210/TDS220 combinations and must not be imposed on this model. User-manual cover qualifies firmware FV:v1.00 and above.

User pages 161–163 describe GPIB setup: unique address 0–30 and **Talk-Listen**, rather than Off-Bus. These are explicit application commissioning facts. No module probe or reset is needed during passive selection/open. User page 165 gives an actual own-model example identity containing `TDS 1002`; passive aliases `TDS1002` and `TDS 1002` can share one canonical factory. Do not accept TDS1002B/C/A by prefix.

Programmer page 28 says GPIB responses end in **LF plus EOI**. Use LF host framing, preserved payload whitespace and a bounded scalar response size such as 256 bytes. This differs from the native HP CRLF bodies. Page 29 describes RS-232's configurable terminators; serial support should not be claimed simply because the host interface accepts arbitrary endpoints.

## Useful existing neutral Oscilloscope surface

The existing neutral API can represent thirteen scalar/control operations without a new device-specific core class: set/read main timebase; set channel scale, display and coupling; set edge-trigger source, slope and level; run, stop and single; measure Vpp and frequency. Trace capture is a separate larger-format decision.

Verified own-model Tek roots and arguments:

| Behavior | Native command family | Primary programmer pages |
|---|---|---|
| Response headers off | `HEADer OFF` | 25–27 |
| Main timebase | `HORizontal:MAIn:SCAle` and query | 157–160 |
| Main display view | `HORizontal:VIEW MAIn` | 161 |
| Channel gain | `CH1:SCAle`, `CH2:SCAle` | 89–90 |
| Probe attenuation | `CHn:PRObe` and query | 88–89 |
| Channel coupling | `CHn:COUPling AC/DC/GND` | 84–85 |
| Channel display | `SELect:CHn ON/OFF` and query | 216–217 |
| Edge trigger | `TRIGger:MAIn:EDGE:SOUrce` and `SLOpe` | 244–246 |
| Trigger level | `TRIGger:MAIn:LEVel` | 248 |
| Acquisition control | `ACQuire:STATE`, `STOPAfter`, `MODe` | 66–71 |
| Immediate measurements | `MEASUrement:IMMed:SOUrce`, `TYPe`, `UNIts?`, `VALue?` | 177–183 |

Use `MEASU` as the documented minimum root, not a copied `MEAS` abbreviation from another vendor. The own-model user example on page 165 proves `MEASUREMENT:IMMED:SOURCE CH1`, `TYPE PK2` and `VALUE?`, returning a sample voltage `5.16E0` with a 10X probe. Programmer page 177's heading SOURCE1 has a B/TPS-specific syntax line; the old unsuffixed SOURCE form is independently grounded by that own-model example. `CHn:YUNIT` is B/TPS-only and should not be used to qualify a non-B channel.

## Limits, scaling and probe parity

- User page 174: TDS1002 main timebase spans 5 ns/div through 50 s/div in a 1–2.5–5 sequence. Programmer page 158 says unsupported requests are forced to a nearest available setting. A reliable initial API should admit a frozen canonical grid and/or verify readback; it must not silently promise an arbitrary requested scale.
- User pages 170–171: supported probe factors are 1, 10, 100 and 1000. The input-BNC vertical range is 2 mV/div to 5 V/div. The displayed volts/div range scales with the configured attenuation. Immutable per-channel probe factors and readback guards preserve useful 10X operation and avoid silently changing physical interpretation.
- Programmer page 89 additionally lists 20X, 50X and 500X **only for B/TPS**. Reject these for TDS1002.
- User pages 130–131 describe coarse 1–2–5 and fine vertical gain. A conservative coarse subset is useful; arbitrary fine settings require a specified device-rounding/readback policy.
- Analog bandwidth is not a proof of an absolute frequency-parser ceiling. Do not turn 60 MHz into an unsupported command or measurement boundary simply because it is the model bandwidth rating.
- Probe compensation, attenuation truth, grounded inputs, signal limits, calibration and adequate displayed waveform cycles remain physical commissioning conditions. Reading a matching software probe factor does not prove the attached probe position.

## Fresh scalar measurement transaction

Immediate VALUE? computes a measurement when queried; it does **not** guarantee a newly acquired waveform. Programmer pages 296–303 explicitly show why reading too soon after RUN gives an incorrect old result, and establish the single-sequence/*OPC? barrier.

A grounded candidate transaction is:

1. Set HEAD OFF and establish the expected current channel display/probe, YT format, main view and trigger conditions through readback or explicit documented commissioning. Reject unsupported conditions before beginning acquisition where possible.
2. Stop acquisition; select SAMPLE; select STOPAFTER SEQUENCE; start RUN/ON.
3. Query *OPC? and require exactly the documented completion value 1. Page 190 explicitly associates it with busy single-sequence acquisitions; page 189 requires a host deadline longer than acquisition.
4. Check completion state and acquisition count if the specification chooses those guards. RUN resets NUMACQ; programmer page 68 says NUMACQ is always zero in Scan mode. SAMPLE/SEQUENCE should have one fresh acquisition, rather than a reused display cache.
5. Set immediate SOURCE CH1/CH2 and TYPE PK2PK or FREQ. Query UNITS? and require the quoted expected V or Hz token. Query VALUE? and parse a bounded ASCII scalar. Verify the final device status under enabled error reporting.
6. Leave the scope stopped after the completed sequence. This intentionally changes run state; do not silently restore continuous acquisition or retry a failed trigger. A separately requested Run operation can start it again.

Programmer pages 181–183 give quoted unit tokens and NR3 output. Its actual scalar frequency example is `28.75E6`. A missing trigger may leave *OPC? pending until the finite host timeout; this is expected, not a reason to inject FORCE or AUTO. An AUTO acquisition is not necessarily externally triggered; qualify any freshness claim accordingly.

## Error reporting cannot be assumed

Programmer page 65 says Trigger View ignores most set commands and generates event 221. This can invalidate a naive freshness proof: ignored RUN followed by an immediate *OPC?=1 and an old NUMACQ=1 is possible if no command acceptance/error guard is used.

Pages 114–115 and 290–294 establish that **DESE masks events before they reach *ESR?**. Consequently ESR=0 proves no error only under a verified mask that includes command, execution, device and query errors (bits 5–2, mask 60; DESE=255 is a simple full-mask policy). A preflight `DESE?` guard plus clean preflight status and final `*ESR?` guard is grounded. Do not automatically clear status with *CLS or drain EVENT?/ALLEv? to hide prior events. Reading *ESR? itself clears status; page 293 describes how repeated status reads affect event-queue visibility, so retain that disclosed diagnostic side effect.

Pages 182–183 document **9.9E37** for hidden source channels, Trigger View, Scan mode and XY. It is a measurement-invalid sentinel, not ordinary physical data and not uniquely overload. A Device error is more truthful than returning an Overload quantity. Recognize numerically equivalent valid ASCII spellings, while malformed grammar remains Parse. Known wrong units/readbacks should be Device; malformed tokens are Parse. Finite-looking values accompanied by device warnings need the specified status policy, rather than assuming numeric success.

The actual user manual puts AUTO Scan mode at **100 ms/div or slower** (page 110); do not import another series' 10 ms threshold. A normal-trigger sequence or explicit Scan guard avoids treating a rolling display as a fresh triggered capture.

## Deferred and external behavior

Trace transfer uses a 2,500-point record, DATa encoding/start/stop/source controls, CURVe and WFMPre scaling. It requires a larger body bound, exact binary/ASCII framing and atomic metadata/data validity. Deferring it while exposing useful scalar/control operations is honest; do not advertise the generic CaptureVoltageTrace capability until implemented and tested for this model. Likewise defer serial interfaces, alternate modules, math/reference measurements, FFT, hardcopy, global reset and checked shutdown unless separately specified.

Factory metadata and open remain passive. Owned/borrowed cleanup, sync/native/worker parity, finite pending IO deadlines, cancellation and quarantine follow the public SDK and external message engine. A quarantined incomplete GPIB sequence needs external bus resynchronization and reopen; resource close should not send device clear/reset or restart acquisitions. All stated results remain software/manual evidence pending physical bench validation.
