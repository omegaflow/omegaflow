<!--
  title: Survey — Sonden-Flotte (Stand 2026-09-16)
  class: survey
  date: 2026-09-16
  sha256: 9d17f362bce9456d98b1aa4f20983d675a9d410afb290baf5872067122981b39
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

## Nachtrag 2026-09-28 — Asien, Russland/UdSSR, Welt-Zensus (gemessen)

Zwei Taucher (`research-max`, harte Bandagen: `--verdict`/`--sniff`/`--tavily`/`--exa`/
`--playwright` + curl-Fallback) haben die Flotte außerhalb NASA/ESA gemessen.
Register-Vorprüfung: keine dieser Missionen stand in `phi/sources.φ`/`blocked_sources.φ`/
`declined_sources.φ` — **alles Grünland**. Jede Zeile mit Beleg; Ungemessenes heißt so.

### Asien — anonym offen (200 gemessen, registrierbar)

| Mission | Agentur | Produkt | Route | Zustand |
|---|---|---|---|---|
| Akatsuki | JAXA/ISAS | **Radio-Science PDS4** `vco_rs` | `data.darts.isas.jaxa.jp/pub/pds4/data/vco/vco_rs/` | 200 anonym |
| Hayabusa | JAXA/ISAS | AMICA/LIDAR/NIRS PDS4 | `sbnarchive.psi.edu/pds4/hayabusa/` | 200 anonym |
| Hayabusa2 | JAXA/ISAS | SPICE | `naif.jpl.nasa.gov/pub/naif/pds/pds4/hyb2/hyb2_spice/` | 200 anonym |
| Kaguya/SELENE | JAXA/ISAS | LRS-Roh (PDS ODE) + SLN-Rstar | `ode.rsl.wustl.edu/moon/.../KAGUYA (SELENE)/LRS/Raw_Data.htm` | 200 anonym |
| SLIM | JAXA | DOI-Datensätze | `darts.isas.jaxa.jp/doi/slim/slim-rd-0002.html` | 200 anonym |
| Hisaki | JAXA/ISAS | EXCEED/EUV L2 | `darts.isas.jaxa.jp/en/datasets/darts:hisaki-exceed-euv-level2` | 200 anonym |
| Chandrayaan-1 | ISRO | M3/Mini-SAR/HySI PDS3 + SPICE | `pds-geosciences.wustl.edu/missions/chandrayaan1/`; `spiftp.esac.esa.int/data/SPICE/CHANDRAYAAN-1/` | 200 anonym |
| Danuri/KPLO | KARI/KASI | ShadowCam + KASI PDA | `shadowcam.im-ldi.com/`; `pda.kasi.re.kr/` | 200 anonym |
| BepiColombo-Mio | ESA/JAXA | PSA | `archives.esac.esa.int/psa/ftp/BepiColombo/` | 200 anonym |

### Asien — account-gated (Daten existieren, Konto nötig → `blocked account`, Future)

| Mission | Agentur | Portal | Zustand |
|---|---|---|---|
| Chang'e 1–6 | CNSA | `moon.bao.ac.cn` (GRAS) | Portal 200; Login-Gate (nicht end-to-end gemessen) |
| Tianwen-1/Zhurong | CNSA | `nssdc.ac.cn` | Portal 200; Antrag/Konto |
| Chandrayaan-2/3, MOM, Aditya-L1 | ISRO | `pradan.issdc.gov.in`, `mrbrowse.issdc.gov.in` | account nötig |
| Hope/Al-Amal | MBRSC (VAE) | `sdc.emiratesmarsmission.ae` | account (kostenlos) |
| Danuri KGRS/KMAG/LUTI | KARI | — | kein Portal gefunden |

### Russland/UdSSR — gemessen

| Mission/Korpus | Produkt | Route | Zustand |
|---|---|---|---|
| Venera 15/16 | Altimetrie (10,4 MB)/Radiometrie (5,9 MB) ASCII + Lander-Panoramen | `pds-geosciences.wustl.edu/venera/mpi-venus-alt.dat`, `…/mpi-radiometry.dat` | 200 anonym |
| Vega 1/2 (Halley) | 7 Instrument-Sets (TVS/DUCMA/SP-1/SP-2/PUMA/PM1/MISCHA) | `pds-smallbodies.astro.umd.edu/holdings/vega2-c-*` | 200 anonym |
| Vega 1/2 (Venus-Ballons) | Druck/Temp-Profile (PDS3 certified) | `atmos.nmsu.edu/PDS/data/vega_5001/` | 200 anonym |
| Phobos 2 | KRFM-Termoskan/VSK-FREGAT | `pds-smallbodies.astro.umd.edu/holdings/phb2-m-*` | 200 anonym (PWS via CDPP account) |
| ExoMars TGO | ACS/FREND/NOMAD/CaSSIS (russ. Instrumente) | `archives.esac.esa.int/psa/ftp/ExoMars2016/` | 200 anonym |
| Spektr-R/RadioAstron | VLBI-Roh | `asc.rssi.ru/radioastron/` | Portal 200; `opendata.*` tot |
| Venera-Doppler/Tracking | — | — | **kein öffentlicher Bestand** (request-only, IKI/NSSDC) |
| Luna 1–24, Mars 2–7, Zond 3/5–8 | — | — | kein öffentlicher Roh-Korpus gemessen; request-only |
| Mars-96 | — | — | **null-echt** (keine Daten existierten; Startfehler 1996) |

### Welt-Zensus (Portal | gemessen | im Haus?)

ESA/ESAC 200 anonym (23 Linien) · NASA PDS 200 anonym (ja) · NASA SPDF 200 (14) ·
NASA NSSDC 200 (**0 Linien — fehlt**) · CDAWeb 200 (11) · JAXA/ISAS 200 (**fehlt**) ·
CNSA/NAOC 200-Portal/Konto (**fehlt**) · ISRO 200-Portal/Registrierung (**fehlt**) ·
Roscosmos/IKI 403 geo-suspect (**Sonden-Roh fehlt**) · UKSA/UKSSDC 200 (**fehlt**) ·
CSA 200 n/a · ASI/SSDC 200 (**Sonden-Roh fehlt**) · CNES/CDPP 200-Portal/account (**fehlt**) ·
DLR `pds.dlr.de` **tot gemessen** · INPE pending · CONAE pending · ISA n/a ·
NSPO pending · KARI/KSDC pending · MBRSC 200 (**fehlt**) · KASI-PDA 200 (**fehlt**).

### Offene `unmeasured`-Punkte (nicht descoped)

CDSN/NAOC-VLBI/Changchun (kein Archiv gesucht) · Tianwen-2 · Tianwen-1-SPICE-Kernels
(NAIF 404) · DLR-PDS-Nachfolger-URL · `opendata.radioastron.*` (Wayback 429) ·
`rgcps.asu.edu/venera-15-16` (Mosaik-ZIP 356,7 MB, Host kein Response) ·
NSSDC-Einzel-Datasets Luna/Mars/Zond · ISRO-Subpfade (CH1-ISSDC, Aditya).

### Werkzeug-Befund

`archive_search --verdict` **hängt** an GitHub-Release-URLs (302-Redirect) — die
Reachability der CDN-Assets (`voyager_odr_s0.bin`, `juno_ocru_odf.bin`,
`mariner_occlt.bin`) wurde daher per `curl -sI` gemessen: alle **302** (signiert,
Asset vorhanden).
