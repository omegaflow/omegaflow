<!--
  title: The 20-s band — the evidence (sources, not interpretation)
  class: paper
  date: 2026-08-24
  sha256: f56092d73a1878c13caee91700425735c9aac03adedecdaf73ee66ff9a482cf3
  status: live
  see-also: docs/paper/twenty-second-band-ground-chain.md, docs/paper/text-as-data-pioneer.md, docs/paper/probe-front-dark-matter.md, docs/TODO.md (Pioneer-Front)
-->

# The 20-s band — the evidence (sources, not interpretation)

## Abstract

The documentation (DSN 810-005, TRK-2-18/24/25, TDA PR 42-67/69/72/82/120, FTS Mark IV-85, CRG 42-64, LRR-2010-4) proves a ground calibration path in the receiver-exciter chain and names the Doppler time grids precisely (0.1-s phase counting, 5/10/60/600/660/1000/1980-s integration) — but it proves no 20-second cycle. A 20-s keep-alive/ping stands in none of these documents. The measured 20-s shape with decay peaks at 40 s and 60 s is a measurement; the envelope is the harmonic windowing of this periodicity against the documented 60-s count grid, not a documented ground pulse. The sources support the calibration path and the time grid, not that clock.


## 1. The search surface

Searched (text + OCR, 2026-08-24): DSN 810-005 module 202 (Doppler, Rev. A 2002
and Rev. E 2023), the 1978 810-005 (TCI-10/TCI-20, scanned), TRK-2-18
(ODF interface 1988), TRK-2-24, TRK-2-25 (ATDF 1988), TDA PR 42-67 (SETI
downconverter), 42-69 (receiver-exciter controller), 42-72 (syntonization), 42-82
(FTS Mark IV-85), 42-120 (radio science, Morabito & Asmar), FTS Mark IV-85, CRG
42-64, and the review text LRR-2010-4. The calibration hypothesis was run against
candidate queries ("ground receiver calibration injection",
"receiver test signal injection", "closed-loop calibration cycle").

## 2. Evidence A — the ground calibration path exists

TDA PR 42-69 "Receiver-Exciter Controller Design" (P. A. Jansma, June 15, 1982,
Mark IV-A, Block III/IV). Table 2 "Tasks assigned to the I/O processor" (OCR,
page 121):

> **Calibrate** — "*Coordinate test signal and instrumentation controller to
> perform calibration*"

With this it is documented: the receiver-exciter subsystem controller has a
calibration task that coordinates a **test signal**. The environment is
instrumented (DVM for AGC/phase measurement, frequency counter for VCO/Doppler/
synthesizer, simulation (Doppler) synthesizer, MSA). **Limit:** no
period specification. The path is proven, a 20-s clock is not.

## 3. Evidence B — the time grid, and 20 s is missing

- 810-005 module 202 (Doppler, Rev. A): "*The downlink phase counts are available
  at 0.1-second intervals*"; carrier power "*reported once per second*";
  solar curves for integration times **5 / 60 / 1000 s**.
- LRR-2010-4 (Turyshev & Toth 2010): "*60 s, 600 s, 660 s, or 1980 s*"
  (l. 2982); "*10 s, 60 s, 600 s, 660 s, and 1980 s*" (l. 3040); "*50 s count
  time*"; the data situation is said to have been "*already been compressed to 60 s*" (l. 3044).

**The completeness of the list is the finding:** 0.1 / 1 / 5 / 10 / 60 / 600 /
660 / 1000 / 1980 s. **No 20-s interval.** The measured peaks at ~19.7 s,
~39 s, ~59 s fill exactly the gap between 1 and 60 s — and 59–60 s »is«
the documented grid.

## 4. Evidence C — TRK-2-24 is the weather interface

TRK-2-24, DSN-820-13 Rev. A ("DSN TRACKING SYSTEM INTERFACES — WEATHER DATA
INTERFACE", eff. Jan. 15, 1986), via the PDS radio-science archive. Purpose:
**weather files** of the TSAC group (dew point, air temperature, pressure, water-vapor
partial pressure, relative humidity; DSS-tagged; 30-minute grid, max. 60 s;
UTC). The suspected receiver-calibration location is thereby the wrong one:
TRK-2-24 proves the weather calibration (atmosphere), not a receiver pulse.

## 5. Evidence D — cal tones exist, in the open-loop branch

Morabito & Asmar, TDA PR 42-120 (radio-science performance, 1995): noise
injection into the feed (known noise temperature) and the **exciter-translator
test signal** that gets stepped across the bandwidth (phase/frequency/amplitude
calibration of the open-loop recording); internal memo "A Calibration Tone"
(Voyager RSST-88-016, PLLDEC analysis). SETI downconverter (TDA PR 42-67):
28 MHz test generator, stand-alone. → These are cal tones, but of the **open-loop
branch** (radio science / SETI), not of the closed Doppler channel.

## 6. What is not proven

**A 20-s keep-alive/ping from the ground station.** In none of the searched
documents does it reveal itself as a cycle. The two honest interpretation
paths that remain: (a) the documented calibration path (evidence A) with
station-internal component frequency response, but without claiming a 20-s clock;
(b) the measurement property itself — the 20-s periodicity is measured, its
interpretation as a "ping" is a hypothesis. 0 honored: where a mechanism is not
documented, it stays `open`, not `proven`.

## 7. The point for S01

The negative index on the residuum needs the **shape** (measured: 20-s periodicity,
decay envelope against the 60-s grid), not the verdict about the
mechanism. What is subtractable gets subtracted at the measurement; that it
is "ground" says the station fixedness (Deduction 26/30/33–35) — that the
**concrete 20-s source mechanism** is documented says no source. Both
sentences side by side are the complete, honest evidence.

## References

1. PDS Radio-Science Documentation (WUSTL), DSN-810-005 module 202 Doppler,
   2002-12-15 — `pds-geosciences.wustl.edu/radiosciencedocs/.../dsn_810-005/`.
2. DSN 810-005 module 202E, 2023 — `deepspace.jpl.nasa.gov/dsndocs/810-005/202/`.
3. P. A. Jansma, "Receiver-Exciter Controller Design," TDA Progress Report 42-69,
   pp. 117–125, June 15, 1982 (OCR table 2) —
   `ipnpr.jpl.nasa.gov/progress_report/42-69/69M.PDF`.
4. DSN 820-13 Rev. A, TRK-2-24, Weather Data Interface, Jan. 15, 1986 —
   `pds-geosciences.wustl.edu/radiosciencedocs/.../dsn_trk-2-24/`.
5. J. H. Yuen / P. W. Kinman, 810-005 module 202, section 2 (phase counting
   0.1 s; carrier power 1 s; integration times 5/60/1000 s).
6. Morabito D. D., Asmar S. W., 1995, Radio-Science Performance Analysis
   Software, TDA PR 42-120 (noise injection, exciter-translator test signal;
   memo RSST-88-016 "A Calibration Tone").
7. SETI Downconverter, TDA PR 42-67 (28 MHz test signal).
8. Turyshev S. G., Toth V. T., 2010, Living Rev. Relativity 13, 4 (Doppler
   count times 10/60/600/660/1980 s; review text).
