<!--
  title: Survey — Sonden-Flotte (Stand 2026-09-16)
  class: survey
  date: 2026-09-16
  sha256: c24490e8c951edf9c14dadb033b512a82181a2b1b676036995cc260aecea8ad5
  status: live
  see-also: phi/sources.φ phi/blocked_sources.φ docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md
-->
# Survey — Sonden-Flotte (Stand 2026-09-16)

Anlass: welche **Sonden-Daten** das Haus auf dem CDN trägt. Gezählt aus
`phi/sources.φ` + den `github.com/omegaflow/sources/releases`-Assets, 2026-09-16 —
wiederholbar wie die astroquery-Schnittmenge (Methode = Herkunft). Kein Verdikt,
die Messung.

## A — Radio-Science / Doppler (ODF/ODR/ATDF): 13 Missionen, 14 Raumfahrzeuge

| Sonde | Asset auf dem CDN | Quellhost | Workflow |
|---|---|---|---|
| Voyager 1/2 | `voyager_odr.bin` | pds-ppi.igpp.ucla.edu | `voyager-odr-cdn.yml` |
| Voyager (Saturn) | `voyager_saturn.bin` | spdf.gsfc.nasa.gov | `voyager-saturn-cdn.yml` |
| Pioneer 10 | `pioneer10_skyfreq.bin` (`atdf`) | spdf.gsfc.nasa.gov | `pioneer-atdf-cdn.yml` |
| Galileo | `galileo_odf.bin`, `galileo_odr.bin` | pds-ppi.igpp.ucla.edu | `galileo-trk-noise.yml`, `galileo-odr-cdn.yml` |
| Juno | `juno_odf.bin` | atmos.nmsu.edu | `planetary-odf-cdn.yml` |
| Mars Express | `mars_express_odf.bin` | archives.esac.esa.int | `planetary-odf-cdn.yml` |
| Rosetta | `rosetta_odf.bin` | archives.esac.esa.int | `planetary-odf-cdn.yml` |
| Venus Express | `vex_odf.bin` | atmos.nmsu.edu | `vex-cdn.yml` |
| MESSENGER | `messenger_odf.bin` | pds-ppi.igpp.ucla.edu | `planetary-odf-cdn.yml` |
| Mars Global Surveyor | `mgs_odf.bin` | pds-geosciences.wustl.edu | `planetary-odf-cdn.yml` |
| Mars Reconnaissance Orbiter | `mro_odf.bin` | pds-geosciences.wustl.edu | `planetary-odf-cdn.yml` |
| Odyssey | `odyssey_odf.bin` | pds-geosciences.wustl.edu | `planetary-odf-cdn.yml` |
| Magellan | `magellan_odf.bin` | pds-geosciences.wustl.edu | `planetary-odf-cdn.yml` |
| Dawn | `dawn_odf.bin` | sbnarchive.psi.edu | `dawn-cdn.yml` |

## B — Ephemeriden (Sonden-Bahnen)

| Sonde | Asset |
|---|---|
| JUICE | `ephemeris_juice.bin`, `ephemeris_juice_cog.bin` |
| Juno | `ephemeris_juno.bin` |
| New Horizons | `ephemeris_new_horizons.bin` |
| Parker Solar Probe | `ephemeris_parker_solar_probe.bin` |
| Solar Orbiter | `ephemeris_solar_orbiter.bin` |
| Voyager 1/2 | `ephemeris_voyager1.bin`, `ephemeris_voyager2.bin` |
| Pioneer 10/11 | `ephemeris_pioneer10_daily.bin`, `ephemeris_pioneer11_daily.bin` |
| JWST | `ephemeris_jwst.bin` |
| ISS | `ephemeris_iss.bin` |

Dazu ~90 **Himmelskörper**-Ephemeriden (Planeten/Monde/Asteroiden,
`ssd.jpl.nasa.gov`) und die Planetentheorien **EPM** (`ftp.iaaras.ru`), **INPOP**
(`ftp.imcce.fr`), **NOE4** — keine Sonden.

## Die offenen Türen (Lücken mit Grund)

| Sonde | Befund | Grund |
|---|---|---|
| Ulysses | 0 | kein Register-Eintrag |
| BepiColombo | 0 | kein Register-Eintrag |
| LRO | 0 | kein Register-Eintrag |

Dazu **fünf DSN-Briefe in Flug** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno;
Entscheid-Handover §Warten).

## Gegenprobe zur Warteliste — Cassini/MAVEN/DART (gemessen 2026-09-16)

Die eigene Warteliste misst: **nur Cassini/Maven/DART tragen TRK-2-34-Bundles**
(PDS, öffentlich). Diese Liste zeigt Cassini/MAVEN (0) und DART (fehlt). Die
Taucher-Runde (flash + pro + max + vision, Playwright) hat die Türen gefunden —
**alle drei sind erntbar**:

| Mission | TRK-2-34-Route (gemessen) | Umfang |
|---|---|---|
| **MAVEN** | `pds-ppi.igpp.ucla.edu/data/maven-rose-raw/data/tnf/` (PDS4 `data.tnf` v1.33) | **1 167** TNF-Produkte, 2016-02-19 → 2025-12-05, anonym |
| **DART** | `pdssbn.astro.umd.edu/holdings/pds4-dart:data_trk234-v1.0/` (PDS4) | **401** TNF-Produkte, 2021-11-24 → 2022-09-26, anonym |
| **Cassini** | `atmos.nmsu.edu/pdsd/archive/data/co-s-rss-1-*/…/tnf/` (PDS3 RSS-Volumes) | TNF+ODF je Volume (sroc/enoc/hygr/tocc/tbis/iagr/rhgr/gwe), Beispiel `NJPL2I00C124`, HTTP 200 |

Cassini ist **nicht** request-only: das galt für die PDS4-Registry, die die
PDS3-TNF/ODF **nicht** katalogisiert — die Dateien liegen in den RSS-Volumes
(`…/tnf/`, `…/odf/`), anonym ladbar. Die Flotte wächst damit um **drei**.

Nachzug 2026-09-17: Cassini und DART sind geerntet und in `phi/sources.φ`
registriert (`cassini_tnf.bin`:9202, `dart_tnf.bin`:9194); MAVEN bleibt offen
(kein Eintrag in `sources.φ`).

**Parser-Lücke (gemessen 2026-09-16, `src/archivar/odf.rs`):** das SFDU-Framing steht
(`read_tnf_sfdu`/`scan_tnf_sfdus`, `sfdu_length` @12–19, Zeit-Tag @48–60) und
dekodierte damals **format_code 0** (`tnf_dt0`, DT0/Uplink-Carrier-Phase). Nachzug
2026-09-17: alle **18 Format-Codes** sind dekodiert (`odf.rs:1639` match über UL/DL
Carrier-, Seq-/PN-Ranging-Phase, Doppler, Range, Angle, Ramp, VLBI, DRVID, Smoothed
Noise, Allan Deviation, PN-/Tone-Range, Carrier-/Total-Phase Observable); die
17-Code-Lücke ist geschlossen. Offen bleibt der native Serien-Arm (der `tnf_compiler`
schreibt CSV, `format csv`). Referenz-Parser: `github.com/NASA-PDS/PyTrk234`; SIS:
`pds-geosciences.wustl.edu/radiosciencedocs/…/dsn_trk-2-34.2021-06-03.pdf`.

## Zähl-Konvention

**13 Missionen, 14 Raumfahrzeuge.** Voyager 1 und 2 zählen einzeln, obwohl sie ein
gemeinsames Asset (`voyager_odr.bin`) tragen; Pioneer 10 trägt das
Radio-Science-Asset, Pioneer 11 nur eine Ephemeride. Ohne diese Zeile rechnet eine
spätere Session die Tabelle anders nach.

## Kontext

- **91 % außerhalb jeder Fremd-SDK** — kein astroquery/SunPy/Airbyte liest ODF,
  ODR, ATDF oder UNIVAC-Uhren (siehe
  `survey-2026-09-16-fremde-parser-sammlungen.md`).
- **JUICE fliegt** — Erd-Vorbeiflug 28./29.09.2026; die Ephemeride liegt schon auf
  dem CDN. Alle anderen Zeilen sind Archäologie; hier könnte das Haus einem
  Vorbeiflug zum ersten Mal **live** zusehen.
