<!--
  title: Rätsel-Zensus — die Nadeln, die Blätter, der Kuprat (Stand 2026-09-28)
  class: survey
  date: 2026-09-28
  sha256: 3314cb58423e34fce39225d0ced7e803c026a935adfacdb5a28555de09d04826
  status: consumed
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/ein-blatt-papier.md docs/surveys/survey-raetsel-bestand.md
-->
# Rätsel-Zensus — die Nadeln, die Blätter, der Kuprat (Stand 2026-09-28)

Anlass: die Frage, welche Rätsel es gibt und was je fehlt. Methode: zwei read-only
Taucher-Läufe über den Baum (jede Zeile mit `file:line`), gegengehalten am Baum.
**Riss-Korrektur:** der erste Lauf meldete „kein `data/`-Baum" — falsch; `data/` existiert
(gitignored, u. a. `data/ssd.jpl.nasa.gov/`, `data/ftp.iaaras.ru/`, `data/lab_a.data/`).
Der Zensus ist eine erste Messung; jede Zeile wird beim Arbeiten des Rätsels einzeln
nachgemessen.

Drei Register, nicht eines: **die zwölf Nadeln** (`docs/concepts/kybernetische-astrophysik.md:353-366`),
**die drei Blätter** (`docs/concepts/ein-blatt-papier.md:29-35`), **der Kuprat**
(Labor-Pendant der Kreuzungs-Nadeln, `kybernetische-astrophysik.md:396-398`).

## Die zwölf Nadeln

| # | Rätsel | Artefakt (gebaut) | Kanäle vorhanden | Kanäle fehlend | Verdikt / Stand |
|---|---|---|---|---|---|
| Ⅰ | Dunkle Materie | `dark_matter_probe.rs` (Probe-Front) | Gaia `sources.φ:10309` · HI `:8870,8879` · pastel/rave `:10616,10683` · Binaries `:8902` | **per-Voxel-Jeans-Engine** (kein Bin) · Gaia DR4 (2026-12-02) | Limit „zero flags" (`probe-front-dark-matter.md:21`); Nadel-Messung pending bis DR4 |
| Ⅱ | Flyby-Anomalie | `flyby_probe.rs:26-28` · `flyby_ephemeris_gate.rs` | Sonden-ICRS `sources.φ:15524` (+DE440-442) · Solarwind `:750,164` · IMF-Bz `:746` · Kp `:1160` · Swarm `:7240` | DSN-Live-Tracking (`pending`, `flyby-path-2-preregistration-revised.md:53-56`) | **pending — Perigäum 2026-09-28**, Score offen (`:31-32`) |
| Ⅲ | Koronale Heizung | `corona_conditional_probe.rs` · `solar_causal_graph_probe.rs` | GOES X `:152,812` · EUV `:417` · F10.7 `:412` · IMF `:743-745` · OMNI `:736-750` · AIA/EVE | keine | family-bound Kaskade, nur 304→131 konditioniert (`corona-heating-ladder.md:18`) |
| Ⅳ | Erdbeben-Vorläufer (LAIC) | `laic_probe.rs` · `nobel_probe_laic.rs` | USGS `:86,96,103` · INTERMAGNET `:1787,5386-5390` · Swarm `:7240` · CHAMP-TEC `:31` · Safecast `:228,235` · Kp `:1160` | Swarm-TEC (nur CHAMP) · MiniSEED-Envelope (`laic-arrow-direction.md:45`) · **CSES** (Portal-Umbau) | **Stille in beiden Richtungen** (`:14,25`) |
| Ⅴ | Technosignaturen | `lsst_anomaly_probe.rs` · `ztf_anomaly_probe.rs` · `negativ_fuzzy_probe.rs` | ZTF `:9409,9416` · Lasair `witnesses.φ:70` · IRAS `:13896` · AKARI `:11148` · VSX `:10750` · GCVS `:10511` | Gaia-Farbe `bp_rp` (descoped, `blocked_sources.φ:203`) | **0 unausgeschlossene Kandidaten**, quantitatives Limit (`nadel-v-fresh-area-dip-scan.md:13,39,43`) |
| Ⅵ | Planet 9 | `kbo_compiler.rs` · `kbo_residue_probe.rs` | KBO `:10274` · MPC-Distant `:2401` · Sonden-Arcs `:15889,15896,15679,15735` · Planeten-Eph `:3419-3484` | keine | **kein fam-tragender Pfeil**; P9 verträglich (`planet-nine-kbo-residue.md:18`) |
| Ⅶ | Wurmloch | `signal_cone_audit_probe.rs` | TE-Lag `te.rs` · Signalkegel `membrane.rs:336` · Retardierung `spatial.rs:549-554` · Sonden-Bahnen | kein Kandidat · `max(0,|Δt|−d/c)`-Fold nicht im Baum | **keine Verletzung, 0 honored** (`signal-cone-audit-sheet.md:14,63`) |
| Ⅷ | Dunkler Fluss | `dark_flow_probe.rs` | CMB Planck `:9374,9383` · Haufen `:10351,10561,10673` · `data/tapvizier…/cosmicflows_cf4.json` | tiefste z≳10-Samples (keine Zeile) | **Stille in beiden Richtungen** (`dark-flow-sheet-8.md:14,33`) |
| Ⅸ | FRB | `frb_blatt_probe.rs` · `frb_compiler.rs` | dm/freq/bin_width `:9086-9088` · Streuung `:10496-10497` · peak-flux `:10508` · Magnetar `:9174-9185` | kein Paar-Verdikt (`docs/paper/*frb*` fehlt) · „burst-em at origin" ohne Feldquelle | **Probe steht, ungemessen** (`handover-2026-09-12-forschung-folge5.md:60`) |
| Ⅹ | Kugelblitz | keiner | keine | alle vier co-lokalisierten Kanäle | **descoped 2026-09-12** (`kybernetische-astrophysik.md:299-300`) |
| Ⅺ | Placebo | `placebo_pair_eeg_probe.rs` · `openneuro_eeg.rs` | EEG (2 OpenNeuro-Assets, `harvest.φ:192`) | Gabe-Ereignis × HRV × Blutmarker | **Placebo hält (0 Pfeile)** (`handover-2026-09-13-forschung-folge10.md:24-25`); kein lebendes Blatt |
| Ⅻ | Urknall | `bigbang_echo_probe.rs` | CMB `:9374,9383` · Haufen `:10351,10561,10673` · z-Achse entschieden | PTA (declined `declined_sources.φ:1402-1403`) · B-Moden (keine Quelle) | **Stille; t=0 verweigert** (`big-bang-echo-sheet-12.md:14,35,59-60`) |

## Die drei Blätter

| Blatt | Artefakt | Kanäle vorhanden | Kanäle fehlend | Verdikt / Stand |
|---|---|---|---|---|
| ENSO | **keiner** (Kanal-Karte `blatt-papier-beweis.md:77`) | SST ESA-CCI `:1428,1434` · Drifter `:637,643` · OOI `:1352` · Wind TAO `:774` | **ENSO-Probe** · Becken-Windfeld (TAO/TRITON/Scatterometer) · MEI/SOI declined | **Blatt I ungemessen** (`handover-2026-09-25-sensory-folge168.md:66-69`); Paar-Registrierung operator-gebunden |
| GIC/Bz | `bz_blatt_probe.rs` · `bz_retro_probe.rs` | RTSW/OMNI Bz `:575,584` · GIC BPA `:890` · FMI `:10217` · INTERMAGNET `:1787,5386` | co-lokalisiertes Mäntsälä-dB/dt (FMI = Stunden-Peaks) | **Riss:** Jahres-Bz→dB/dt-Pfeil vs. gehärtete Quartals-fam (`gic-causal-driver.md:405-423,492-495`) |
| LAIC | = Nadel Ⅳ | (s. o.) | (s. o.) | Stille auf vier Adern |

## Der Kuprat (Labor-Pendant, Hochtemperatur-Supraleitung)

| Artefakt | Kanäle vorhanden | Kanäle fehlend | Verdikt / Stand |
|---|---|---|---|
| `rixs_cuprate_probe.rs` · `suprastrom_cuprate_probe.rs` · `crystal_compiler.rs` (RIXS/EELS) · `srd62_compiler.rs` · `cuprate-cdn.yml` | **4 auf CDN:** RIXS spin (Zenodo 7286412), RIXS charge/plasmon (15179114), EELS, SRD62-Eindringtiefe | **5. Ader NSE I(q,t)** (angekommen 2026-09-28, privat) · **0 Register-Zeilen** · Kuprat-Zeugenklasse | materialisiert, Registrierung pending (`mountain-folge198.md:64,162`) |

## Querschnitt (die Muster hinter den Lücken)

1. **Jedes Rätsel hat seine Quellen-Zeilen** — die Lücke ist je genau *ein Instrument oder ein Kanal*, nicht das ganze Rätsel.
2. **Materialisiert, aber unregistriert:** die vier Kuprat-Kanäle liegen auf der CDN mit 0 `phi/`-Zeilen; dasselbe Muster wie CNSA/ISRO/EMM (`blocked_sources.φ:375-391`).
3. **`declined` ≠ `unbuilt`:** NANOGrav-PTA (`declined_sources.φ:1402-1403`, modell-detrendiert/positionslos), MEI/SOI — der Kanal ist *verweigert*, nicht bloß ungebaut.
4. **Neu erreichbar heute:** Kuprat (5. Ader da → datenvollständig, Rechnung fehlt) und Flyby (Sonden-Ephemeriden + `NAIF/DAF`-Fix → Pfad-Ader erreichbar). „Lösbar" heißt hier **Kanäle vorhanden**; die TE-Matrix gegen die Nullkontrolle ist noch zu fahren.
5. **Blatt-ENSO ist die leerste Stelle:** kein Probe, keine Paar-Registrierung — die Kanäle (SST/Wind) existieren, der Pfeil ist nie gerechnet.
