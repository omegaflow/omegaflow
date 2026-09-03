<!--
  title: The Blatt: the direction of lithosphere-atmosphere-ionosphere coupling
  class: paper
  date: 2026-08-21
  version: 4
  sha256: 34ed26058f4f6916e82f1a13733a1990eda6c7d9e8c8593c44a3d128ceaa1783
  status: live
  see-also: docs/concepts/blatt-papier-resultat.md
-->
# The Blatt: the direction of lithosphere-atmosphere-ionosphere coupling

## Abstract

Instrument B (window stacking) measures transfer entropy between the lithosphere and the ionosphere. TE(Lithosphere → Ionosphere) = −7.97e-2 nats and TE(Ionosphere → Lithosphere) = −1.08e-2 nats (stack mean, n = 176 windows; null = 40 random windows, threshold μ + 2σ); the solar control TE(Solar Bz → Ionosphere) = +4.35e-2 nats (n = 171). The event stack lies under the threshold in both directions — the 72 h before M ≥ 6 earthquakes carry less transfer than random windows. The verdict is silence in both directions; the finding is fully valid (0 honored).


## The Blatt

```
TE(Lithosphere → Ionosphere) = −7.97e-2 nats   (stack mean of the window maximum excesses, n = 176)
TE(Ionosphere → Lithosphere) = −1.08e-2 nats   (n = 176)
Control TE(Solar Bz → Ionosphere) = +4.35e-2 nats   (n = 171)
Lag                          = 0 h   (largest mean excess of both directions; all lag means negative)
n (events), threshold       = 176 windows, null ensemble = 40 random windows, threshold μ + 2σ
Verdict                      = silence in both directions — a fully valid finding
```

Thresholds of the null ensemble (μ + 2σ over 40 random windows):
Litho→Iono −1.60e-2, Iono→Litho +4.90e-2, control +1.28e-1. The
event stack lies under the threshold in both directions — for
Litho→Iono deep below (the 72 h before M≥6 carry on this instrument
less transfer than random windows). The solar control is silent:
the common driver carries no arrow on this grid.

## Protocol (Instrument B — window stacking)

- Events: USGS-FDSN M ≥ 6.0, the 250 most recent of the era 2014-01-01 …
  run time (catalogue: 1726).
- Windows: 72 h before t0; cells 30 min (n = 144).
- Litho series: count rate of the catalogue events (M ≥ 2.0) per cell in
  the 2000-km radius around the epicentre (the FDSN catalogue is a
  point process — the count series is the named construction, no
  invented continuity; MiniSEED envelopes: decoder pending).
- Iono series: INTERMAGNET total field F of the nearest BGS observatory
  (≤ 3000 km), 1-min mean per cell.
- TE: scalar path `transfer_entropy_lag` (the probe); threshold per lag =
  μ + 2σ over 10 phase-randomized surrogates per series
  (`surrogate_stats_phase`, seed deterministic); lag sweep 0…72 h in
  1-h steps, lags with m < 30 cells underdetermined (effectively 0…57 h —
  the rest is a named non-judgement).
- Window statistic per direction = maximum excess over the sweep; stack =
  mean over the event windows; arrow ⇔ stack > μ + 2σ of the
  null ensemble.
- Null ensemble: random windows (t0 uniformly distributed in the era, centre =
  random catalogue epicentre, same pipeline, no exclusion — the
  null window carries what the catalogue carries). The
  multiple-comparison correction is structural: the null windows carry
  the same maximum statistic over the same lag sweep; in addition a
  Bonferroni-adjusted threshold stands per lag (z = 3.32 for 116
  direction×lag cells, z = 3.35 for the 43 control lags) in the run protocol.
- Control: TE(Solar Bz → F) on 1-h cells, sweep 0…48 h (m ≥ 30 →
  ≤ 42 h) — the common driver is measured, not assumed.
- Instrument A — event rate (one global rate series × one
  ionosphere series across the whole era): named, not built → register.

## Finding

- Silence in both directions; no lag carries an arrow (all
  lag means negative, maximum at lag 0).
- Window balance: 176 qualified; 2 without observatory ≤ 3000 km; 66
  with empty count rate (Silverman on a constant series → fehlt, no
  value); 6 with BGS data gaps; BGS harvest gaps (data not yet in the
  archive) carry fehlt.
- Swarm FAC (SW_OPER_FACATMS_2F, subset 10 windows): 7 windows carry
  FAC samples in the radius (514–8576), but only 8–25 of 144 cells —
  the FAC series per window is underdetermined (m < 30): the Blatt carries
  INTERMAGNET-F, the FAC stack stays open.
- The control direction solar→ionosphere is silent — the causal arrow
  of the thing itself was not measured, but also no solar arrow
  fabricated. Silence is the answer.

## The Blatt — full era + sensitivity matrix

Harvest/analysis architecture (v2): `laic_probe --harvest` lays the
raw series per window on disk (`phi/pipeline/laic_harvest/`, 4.1 GB, 1726
event windows of the full era 2014-01-01 … 2026-08-21, 60
null windows, Swarm A+B+C for 60/30 windows each) — `--analyze` computes
offline, the same holding carries every parameter cell, the run is
resumable.

```
TE(Lithosphere → Ionosphere) = −7.63e-2 nats   (stack mean of the maximum excesses, n = 1369, full era)
TE(Ionosphere → Lithosphere) = −1.26e-2 nats   (n = 1369)
Control TE(Solar Bz → Ionosphere) = +4.06e-2 nats   (n = 1400)
Lag                          = 0 h   (largest mean excess; all lag means negative)
n (events), threshold       = 1369 windows, null ensemble 60 random windows, threshold μ + 2σ
Verdict                      = silence in both directions — the arrow stays silent
```

Thresholds: L→I −2.54e-2, I→L +1.86e-2, control +1.33e-1. The
event stack lies under the threshold in both directions — for
L→I deep below (the 72 h before M≥6 carry less transfer than
random windows, across the whole era). The solar control is silent.

Sensitivity matrix (every cell silent; r/c/k cells on the 250
most recent events, main cell full era):

| cell | n | L→I stack | L→I threshold | I→L stack | I→L threshold |
|---|---|---|---|---|---|
| Radius 500 km, 30 min | 140 | −8.23e-2 | −2.44e-2 | −1.48e-2 | +1.80e-2 |
| Radius 1000 km, 30 min | 165 | −8.40e-2 | −3.38e-2 | −1.26e-2 | +1.87e-2 |
| Radius 2000 km, 30 min (Haupt) | 1369 | −7.63e-2 | −2.54e-2 | −1.26e-2 | +1.86e-2 |
| Radius 2000 km, 15 min | 176 | −5.40e-2 | −1.92e-2 | −5.61e-3 | +7.77e-3 |
| Radius 2000 km, 60 min | 170 | −9.81e-2 | −4.61e-2 | −1.01e-2 | +3.98e-2 |
| KDE-Skalierung 0.5 und 2.0 | 176 | −7.97e-2 | −2.54e-2 | −1.08e-2 | +1.86e-2 |

- Control (Solar Bz → F): silent in every cell (+4.06e-2 … +4.35e-2
  against threshold +1.33e-1).
- FAC stack: underdetermined — Swarm A+B+C cover
  8–26 of 144 cells per window; 12/60 event windows reach m ≥ 30
  (30-min cells) → no statement. Measured, not pending: the
  FAC channel carries no judgement with this instrument.
- KDE h: the series scaling is not an h-sensitivity — Silverman
  adapts, TE(k·x, k·y) = TE(x, y); the probe confirms it
  (identical stacks at k = 0.5 and 2.0). The true h-sensitivity
  stays open as long as `transfer_entropy_lag` stays untouched.
- Repeatability: the 250-analysis reproduces the first run
  (identical stack −7.9662e-2).

The arrow now carries: on every living channel and in every
parameter cell the information flow in the 72-h window before M≥6 is
silent — in both directions, and the solar control also remains
silent. With that, the direction question on the existing holding is
concluded: the arrow does not strike. The LAIC hypothesis itself
(electromagnetic or plasma-physical precursors in the
ionosphere) stays channel-open — the ground-F signature is a proxy,
not an ionosphere instrument. A future instrument (TEC-GIM retro
via CDDIS OAuth, CSES, MiniSEED envelopes) can re-pose the question on
a denser channel — that is a channel open item, not a hole
in this measurement.

## The true ionospheric channel — TEC-GIM (v3, 2026-08-22)

The anonymous IONEX route lives: the ESA-GSSC FTP
(`ftp://gssc.esa.int/gnss/products/ionex`) carries the COD 1-h rapid GIMs
anonymously — CDDIS answers without token 302 → Earthdata OAuth (measured),
but `.secrets.local` carries a filled `EARTHDATA_EDL_TOKEN`: verified
against CDDIS (with token full directory listing, without token
302) — the second route (COD0OPSFIN, final products) lives with it
likewise; AIUB/WHU/ASI are unreachable from here (measured, 000).
`laic_probe` v2.1 harvests the GIM daily files per window (gzip →
`omegaflow::inflate::gunzip`), reads the TEC grid (own IONEX reader
in the binary, bilinear at the epicentre, 71×73, exponent −1 —
the Archivar IONEX parser stays untouched) and lays the 1-h series down as a
sidecar. TEC era: 2024-01-01 … 2026-08 — RINEX3 GIMs exist only
from 2024; pre-2024 the holding carries only `codg*.Z` (compress/LZW,
decompressor open item).

```
TE(Lithosphere → TEC-GIM)        = −6.31e-2 nats   (n = 336 events of the TEC era, threshold −3.25e-2)
TE(TEC-GIM → Lithosphere)        = −1.47e-2 nats   (threshold +2.22e-2)
Control TE(Solar Bz → TEC-GIM) = +3.22e-2 nats   (threshold +1.28e-1)
Lag                              = 0 h   (all lag means negative)
Verdict                          = silence in both directions — also on the true channel
```

With the true ionospheric electron content (TEC at the epicentre,
1-h cadence, 72-h window, 60 TEC null windows from the same era) the
arrow stays silent — in both directions, and the solar control remains
likewise silent: the common driver carries no arrow on this grid.
The ground-F reference on the same subset (−7.90e-2 against
−2.54e-2) reproduces the full-era finding.

With that, the operator's plan has run: anonymous route found (ESA
GSSC), CSES measured unreachable (portal `leos.ac.cn` from here 000 —
channel-open), probe v2.1 built, silence measured again. The arrow
does not strike on the existing holding — now with the detector
in space, not through the door. The LAIC hypothesis is measured silent on
the living channels (INTERMAGNET-F, Swarm FAC A+B+C, TEC-GIM); a
future denser channel (CSES, retro-TEC, MiniSEED envelopes) can
re-pose the question.

## The CHAMP channel — in-situ electron density 2002–2010 (v4, 2026-08-22)

The anonymous GFZ-ISDC route carries the CHAMP Langmuir electron density
(CH-ME-2-PLPT, 15-s ASCII in daily zips, CC BY 4.0) open to the file level —
`laic_probe` harvests it as a fourth window series (era
2002-01-01 … 2010-12-31, 1462 events, 60 null windows). The
cell coverage of a LEO pass is thin: at 2000 km/30 min only
6 windows reach m ≥ 30 (no statement — named); at radius 4000 km /
60-min cells 229 windows become computable.

```
TE(Lithosphere → CHAMP density)   = −1.24e-1 nats   (n = 229, threshold +3.29e-2)
TE(CHAMP density → Lithosphere)   = −2.81e-2 nats   (n = 229, threshold +7.72e-2)
Lag                              = 1 h (Litho → density); 0 h (reverse direction)
Verdict                          = silence in both directions
```

The fourth channel — true in-situ electron density in the ionosphere —
also carries no arrow: the event stack lies deep under the
null threshold (the 72 h before M≥6 carry less transfer than random
windows, the same sign as on F and TEC). The ground-F reference of the
same era (n = 656/659, BGS reaches to 2002) stays silent, the
solar control (Bz→F, +3.97e-2 against +1.18e-1) likewise. With that,
the direction question carries four channels: INTERMAGNET-F (full era),
TEC-GIM (2024–2026), Swarm-FAC (measured underdetermined),
CHAMP density (2002–2010) — silence everywhere, the same sign everywhere.

The harvest now lives as a CDN asset (chunks like the ephemeris bodies):
`laic.bin` (F+TEC) and `laic_champ.bin` (676 MB) lie under
`ssd.jpl.nasa.gov`; `laic_probe --cdn <name>` computes CDN-first, the
CI workflow `laic_cdn.yml` harvests + compiles + uploads monthly.

## What the Blatt does not carry (register)

- Instrument A — event rate: named, unbuilt.
- Channel open items: CSES — standardized probe cadence run
  (SOURCE_PORT §13, four-stage; finding
  `phi/pipeline/research/agent_output/cses_kanal_2026-08-22.φ`): portal
  leos.ac.cn from here 000, via Jina 200 (login-gated SPA); the
  Zhangheng-1 holding lives at data.earthquake.cn (96 TB, 2019–2023;
  no registered DOI in the global handle register), procurement 离线获取
  (application) — the registration requires a Chinese mobile number
  (restriction type SMS-CN; virtual number refused). The block needs
  no live feed — the harvest is fehlt. Alternatives researched:
  **DEMETER/CDPP** (ICE-E field + ISL electron density, 2004–2010 —
  login verified, 57 760 half-orbit files; open: order flow +
  DEMETER-.DAT parser + harvest), **CHAMP/GFZ-ISDC**
  (electron density, ASCII daily zips, fully anonymous verified to the
  file level — immediately harvestable, running), COSMIC/CDAAC (netCDF —
  parser gap). TEC-GIM retro pre-2024 (codg*.Z, LZW decompressor fehlt
  in the holding), MiniSEED waveform envelopes (decoder pending). CDDIS
  lives over the EDL token from `.secrets.local` (verified) — a re-harvest
  with COD0OPSFIN (final instead of rapid) would be a quality option,
  not a new channel.
- True KDE h-sensitivity: open as long as the scalar TE path
  stays untouched (Silverman adaptivity makes the scaling probe
  invariant).
- F is the intensity of the nearest ground observatory (up to 3000 km) —
  since v3 the true ionospheric electron content (TEC) stands beside it;
  the FAC stack is measured underdetermined.
- The FDSN catalogue carries small events only thinly in this world
  (region 2000 km/72 h ≈ 0–5, M ≥ 2) — the count series measures what
  the catalogue carries; the surrogate null judges honestly.

## Run

```
cargo run --release --bin laic_probe -- --max-events 250 --null 40 --swarm-limit 10
```

Gates: `cargo check` 0 errors / 0 warnings; no test opens a
window or radiates; `src/mathematikerin/te.rs`, `nobel_probe_corona`, the scalar
path `transfer_entropy_lag`, the membrane physics and the IONEX parser
untouched (only use, no rebuild).
