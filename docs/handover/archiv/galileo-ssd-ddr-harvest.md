# Harvest report — PDS PPI GO-J-SSD-5-DDR-STAR-SENSOR-V1.0 (Galileo Star Scanner DDR)

Date: 2026-09-05. Agent: omegaflow harvest/measure research subagent.
Repo read-only honored; external data fetched under /tmp/opencode/ssd. No repo writes, no git.

## Task

Anchor the Galileo spacecraft spin rate for the EGA-1 window 1990-12-07..12-10
(the "Station-42 tone" at 52.39 mHz / 19.10 s, nominally the dual-spin rotor at
3.15 rpm / 0.3300 rad/s ±0.0015) via the Galileo Star Sensor DDR at PDS PPI.

## 1. Reachability + archive structure (measured)

- PDS PPI dataset record found via the metadex collection index
  (`/metadex/collection/select/?q=GO-J-SSD-5-DDR-STAR-SENSOR-V1.0`).
- Record: `id = GO-J-SSD-5-DDR-STAR-SENSOR-V1.0`, `bundle_id` same,
  `archive_type = 3` (PDS3 volume set), volume `GOSSD_9001`, size 123 713 227 bytes,
  `curator = PPI`, `slot = /data/GO-J-SSD-5-DDR-STAR-SENSOR-V1.0`,
  DOI `10.17189/1519686`, citation Fieseler, P.D., GO JUP SSD DERIVED ELECTRON
  FLUX V1.0, NASA PDS, 2000. `start_date_time = stop_date_time = 1995-12-07T00:01:00Z`.
- Physical file access: `https://pds-ppi.igpp.ucla.edu/data/GO-J-SSD-5-DDR-STAR-SENSOR-V1.0/<file>`.
  Directory autoindex 404s (site returns its own 404 page with HTTP 200 on
  `/data/<id>/`); individual files serve normally. AAREADME.TXT (38 KB), ERRATA.TXT,
  INDEX/INDEX.TAB (12 KB), /DATA/ORB_XX_STAR_SCANNER.{LBL,TAB} all fetched 200.

## 2. What the archive IS (the measured content, not the assumed content)

AAREADME.TXT (volume GOSSD_9001): "Galileo Orbiter at Jupiter SSD Derived Electron
Flux Data". The star scanner is used as an energetic-electron detector:
"instantaneous flux of 1.5 to 30 MeV electrons in the Jovian environment",
usable inside ~12 RJ. Points spaced about 400 or 80 seconds apart; time accurate
to within 20 seconds without special processing.

Per-file label (DATA/ORB_00_STAR_SCANNER.LBL), SPREADSHEET object columns:
Time (SCET UTC), Spacecraft Clock, Star Code (instrument status), Day of Year,
Twist (rotor twist angle, DEGREE), Raw Star Intensity (COUNT), Raw Background
(COUNT), Filtered Data, Compensated Data, Error Low, Error High, flux
(PARTICLES CM**-2 SEC**-1), R (PLANETARY RADII), further fields.

Measured sample rows (DATA/ORB_00_STAR_SCANNER.TAB, first rows), cadence 100 s,
columns 5 = Twist deg (modulo 360):

    1995-12-07T00:01:27.811 ... 342.310 twist, flux 18.930
    1995-12-07T00:03:07.811 ...  70.200 twist, flux 18.912
    1995-12-07T00:04:47.811 ... 158.100 twist, flux 18.893
    (Twist advances ~ +87.9 deg per 100 s, wrapping 0-360; flux ~19 e cm-2 s-1)

Finding: this DDR is NOT a spin-rate/spin-period telemetry product. It carries
electron flux; the only rotor state is a modulo-360 deg Twist angle sampled at
100/400/80 s cadence — an attitude/pointing reconstruction quantity with
whole-revolution ambiguity at this cadence, time-stamped to ±20 s. No spin
period column, no revolution counter, no per-revolution spin measurement.

## 3. Time coverage (measured, 0 honored)

INDEX/INDEX.TAB lists the complete data set: files `ORB_00_STAR_SCANNER` ..
`ORB_35_STAR_SCANNER`, one per Jupiter perijove pass (J5 absent per AAREADME —
solar conjunction).

- Earliest (J0 / Jupiter orbit insertion): ORB_00, 1995-12-07T00:01:27.811 .. 1995-12-08T23:59:25.034.
- Latest: ORB_35, 2003-09-18T12:50:00 .. 2003-09-21T18:40:00.
- All records target JUPITER and its moons (IO, EUROPA, GANYMEDE, CALLISTO, AMALTHEA).

December 1990 (EGA-1 cruise) is NOT covered. There is no interplanetary/cruise
segment in the volume. Gap from the requested window to the first file:
1990-12-10 .. 1995-12-07 ≈ 1819 days (~5.0 years).

Closest covered epoch to the requested window by calendar date:
J0 perijove 1995-12-07 .. 1995-12-08 — five years later, at Jupiter, not the
1990 Earth-return cruise.

## 4. The comparison to the 19.10 s tone

Not possible from this archive: there is no star-sensor spin rate/period value
for 1990-12-07..12-10 because the archive does not extend to December 1990, and
its content is electron flux with a coarse modulo-360 Twist column — not a
revolution-accurate spin measurement even where it exists (J0 onward).

## 5. Verdict

The Galileo Star Sensor DDR (GO-J-SSD-5-DDR-STAR-SENSOR-V1.0) does NOT confirm,
and cannot test, the 19.10 s rotor-spin identification for the EGA-1 window.
The premise that this DDR carries the revolution-accurate spin telemetry of the
Dec-1990 cruise is not supported by the archive's measured content and coverage.
0 honored: no fabrication of a spin value.

## 6. Registered gap for a full source-port (per SOURCE_PORT.md)

The Dec-1990 spin anchor is `pending`, not found. A true anchor would come from:
- Galileo SPICE attitude kernels (CK) covering the 1990-12-07..10 EGA-1 cruise
  (NAIF archived Galileo s/c attitude), and/or
- the as-run Spacecraft Event Files (SEF) mentioned in AAREADME
  (/DOCUMENT/PROJECT/AR_SEF, per-orbit) or Galileo engineering telemetry spin
  tables — cruise-era, pre-HGA-deployment spin state.
These were NOT harvested here (scope: this DDR); they are named as the next
lookup. What a full omegaflow port of THIS volume would require (not done):
probe/extract pass, Force-Gate classification, τ decision, register disposition
in phi/sources.phi or dead/blocked registers per SOURCE_PORT.md state machine;
as a force the data is electron flux in the Jovian magnetosphere (radiation
environment), not a spacecraft-state measurement — its physical force channel
and ω value would need the review step.

## Files fetched (all under /tmp/opencode/ssd)

AAREADME.TXT, ERRATA.TXT, INDEX.TAB, ORB_00_STAR_SCANNER.LBL,
ORB_00_STAR_SCANNER.TAB (first 1.6 KB), coll.json (metadex record).
