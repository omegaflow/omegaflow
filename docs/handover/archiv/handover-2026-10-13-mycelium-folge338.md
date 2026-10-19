<!--
  title: Handover — Mycelium-Folge 338 (2026-10-13)
  session: Mycelium-Linie in einem Pass — die 4 ableitbaren Ersatzquellen-Compiler in phi/sources.φ registriert (osc/chime_frb_cat2/lamost_dr10/cfa_co, url/format/origin/compiler, Mycelium-Klasse); Insertion zurückgezogen und neu gesetzt, um den konkurrierenden Mountain-Schreiber (Commit e48703f94) nicht zu kreuzen; mountain-Token .1/.2 gefaltet (8-Ziele-Liste, xfer migrate); Nachtrag: LHAASO/VizieR J/ApJS/271/25 als Nachzügler registriert + lhaaso_compiler auf die ASU-TSV angepasst, OCNS2019 (geoazur-Ersatz) gemessen — keine offene Lizenz (Riss); Stehender Pass am HEAD
  class: handover
  date: 2026-10-13
  sha256: a189b67840e9c8d01e385f1ecada451173dfcc5ff75c492ab397453332319142
  status: live
-->
# Handover — Mycelium-Folge 338 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge337.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.1266 — gemessen `opencode.db` Session `ses_ecd958500ffeeGumcGExqQNmPS` beim Handover-Schreiben; deepseek-flash, kein pro/max.

`session_burn` aggregiert 85 Sessions (total `$3.64`); der Per-Session-Wert steht in `opencode.db`,
nicht im Aggregat — offen benannt, kein erfundener Deckel.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „mache weiter" (weitergetragener folge337-Bericht: Welle 3 durch/gepusht; ~15 Registerfälle + HEALPix-Kern offen) | 2026-10-13 | Operator (Session 338) · `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | Operator (Session 337) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 337) · session-weiter Delegations-Consent |

## Offen — eigen

### Ersatzquellen-Registerblöcke (Klassen-Träger)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** die Format-Entscheidung der 4 Restcompiler
- **Lage:** (gemessen 2026-10-13/338) **4 Blöcke registriert** in `phi/sources.φ` (Mycelium-Klasse `url`/`format`/`origin`/`compiler`±`ttl`): `osc` (`sne.space/osc_transients.bin`), `chime_frb_cat2` (`www.canfar.net/chime_frb_cat2.bin`), `lamost_dr10` (`www.lamost.org/lamost_dr10.bin`), `cfa_co` (`zenodo.org/h2_dens_mean_beg03.volume.bin`) — Mountains `terms`/`at`/`field` folgten mit Commit `405e4d6a2` (Token `9ff1374e` gefaltet). **4 Compiler printen keinen vollständigen Block** (kein `FORMAT`/`COMPILER`-const, `--out` Pflicht): `heasarc_w3browse` (`heasarc.gsfc.nasa.gov`), `jpl_sbdb` (`ssd-api.jpl.nasa.gov`), `gesdisc_gnssro` (`data.gesdisc.earthdata.nasa.gov`), `opentopography_raster` (`topex.ucsd.edu`, `format srtm`, `url`-Asset = `--out`-Basename; daneben steht bereits `SRTM15_V2.6` in `phi:29896` → Riss).
- **Blockade:** die 4 Restcompiler brauchen eine Format-Entscheidung (Parser-Arm/Format-Name) — Mountain (Operator-Wort: „mountain kümmert sich drum").
- **Braucht:** Mountain typisiert die 4 Restcompiler + registriert sie.

### LHAASO/VizieR `J/ApJS/271/25` — Nachzügler (Ersatz für `casdc`)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `at`/`field` + CI-Fetch-Wiring
- **Lage:** (gemessen 2026-10-13/338) der alte `casdc.china-vo.org`-Host war declined; lizenzklarer Ersatz = VizieR `J/ApJS/271/25` („first LHAASO catalog", Cao+ 2024, `vizier-scientific-free`, HTTP 200). Block `phi/sources.φ:10799-10802` (`url .../vizier.cds.unistra.fr/lhaaso_sky1.sky1`, `format sky1`, `origin` = ASU-TSV `J/ApJS/271/25`, `compiler lhaaso_compiler.rs`). Der Compiler lädt jetzt nach `vizier.cds.unistra.fr` (statt casdc) und liest die ASU-TSV (`#`-Preamble übersprungen, `RAJ2000`/`DEJ2000`/`N0`, whitespace-Split): `cargo build -p omegaflow-harvest --bin lhaaso_compiler` grün.
- **Blockade:** `terms`/`at`/`field` = Mountains Recht; **kein CI-Workflow** ruft `lhaaso_compiler` (Query→`--input`-Datei-Brücke fehlt).
- **Braucht:** Mountain `terms vizier-scientific-free`/`at sun`/`field`; danach das Fetch-/Dispatch-Wiring. Token `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`.

### geoazur/APDB-Ersatz OCNS2019 — Riss (keine lizenzklare Quelle)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein Wort zur Lizenz (oder NAOJ/Géoazur-Erlaubnis)
- **Lage:** (gemessen 2026-10-13/338) `geoazur` (APDB, Neptun-Satelliten-Astrometrie) ist entfernt. Der vorgeschlagene Ersatz **OCNS2019** („Observational Catalog of Neptunian Satellites 2019"; 24 996 Positionen, 1847–2019, ICRS/TT; NADC China-VO `nadc.china-vo.org/res/r101048/`, DOI `10.12149/101048`) trägt **keine offene Lizenz** — NADC-Policy: copyright belongs to the original data authors; kein CC-BY. Der Download-Host `paperdata.china-vo.org` antwortet **pending** (kein Response; nur Wayback-Snapshot, sha `7b7ad66b…`). Kein Compiler (`neptune_apdb_manifestor.rs` ist auf die alten APDB-Files hardcodiert).
- **Blockade:** die Lizenz-Premisse („lizenzklar") hält **nicht**; dazu Host pending + fehlender Compiler.
- **Braucht:** Entscheidung (Mountain/Future) — Géoazur/NAOJ um Erlaubnis bitten, oder OCNS2019 als author-retained führen, oder die Observable fallenlassen.

### 8-Ziele-Einkaufsliste (future-241) — gefaltet aus mountain-folge335
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Ziel ein Compiler gebaut
- **Lage:** (gemessen 2026-10-13/338) die vier direkt downloadbaren cc-by-4.0 ohne Registerzeile (BC430 `3558850`, Pioneer 10/11 Doppler `13309156`/`13309040`, Venus-Radar `18176179`) + PDS-Geosciences Venus-Radar + PVO-ORSE + JPL SSD: je `url`/`origin`/`compiler`-Zeile.
- **Blockade:** je Ziel fehlt der Compiler; `url`/`origin`/`compiler` ist Mycelium, `terms`/`field` Mountain.
- **Braucht:** Compiler je Ziel (je ein bounded step), dann die Registerzeile. Echo `xfer:mountain.ses_ece7fc36bffeSJbFG4SLeLvRrR.1~0d43166a`.

### `xfer migrate` fehlt im PATH-Release — gefaltet aus mountain-folge335
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein frisches `tools-latest`, das `xfer migrate` trägt
- **Lage:** (gemessen 2026-10-13/338) Baum `tools/register/src/bin/xfer.rs:789` kennt `migrate` — das PATH-`xfer` zeigt es jetzt (`xfer --help` listet `migrate`); ältere Legacy-Token der Transportblöcke sind damit faltbar.
- **Blockade:** keine mehr gemessen (PATH-`xfer` trägt `migrate`); offen ist erst der Lauf.
- **Braucht:** `xfer migrate --line mycelium --handover docs/handover/handover-2026-10-13-mycelium-folge338.md` für die Legacy-Token. Echo `xfer:mountain.ses_ece7fc36bffeSJbFG4SLeLvRrR.2~1003fb06`.

### CDN-Assets >2 GiB (`m10-mdis-raw-cdn`) — Split im Release
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-build`-Release trägt den Split (`fetch_release_or_parts`)
- **Lage:** (gemessen 2026-10-12/337) Mountain erweiterte `src/archivar/{cdn,fetch}.rs` + `raw_mirror.rs` um den 2-GiB-Split; Lauf `38275761488` scheiterte an `Mercury_MESSENGER_MDIS_…_166m.tif` (3,96 GiB, HTTP 422). Token `…b2908226` gefaltet.
- **Blockade:** der Split liegt noch uncommitted im Baum → nicht im Release.
- **Braucht:** nach Mountain-Commit + Release `gh workflow run m10-mdis-raw-cdn.yml` neu dispatchen und messen.

### Stehender Pass — CI-Tafel vollständig (`ci_manage reds`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Release, das `ci_manage reds` trägt
- **Lage:** (gemessen 2026-10-13/338) `ci_manage reds` steht im Baum (`tools/utils/src/bin/ci_manage.rs`, Mountain), noch nicht im Release; der Pass zitiert bis dahin den `gh api`-Zensus. Token `…00ba30f9` gefaltet.
- **Blockade:** Release.
- **Braucht:** sobald das Release `reds` trägt, im Pass zitieren.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-12/337) auf den `archive_search --help`-Stand gesynct (90 fehlende Netz-Arme, Zählung 106, Header-sha erneuert); `--orphan-docs` = 14 (keine mycelium-Datei). Ruht.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

## An mountain

Origin: mycelium-338 (2026-10-13):

- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **terms/at/field für den LHAASO/VizieR-Block + CI-Fetch-Wiring** — Block `phi/sources.φ:10799-10802` (`url .../vizier.cds.unistra.fr/lhaaso_sky1.sky1`, `format sky1`, `origin` = VizieR ASU-TSV `J/ApJS/271/25`, `compiler lhaaso_compiler.rs`). Bitte `terms vizier-scientific-free` + `at sun` + `field`/`quantity` (N0→KIND_GAMMA) setzen; **kein CI-Workflow** ruft den Compiler (der `origin` ist eine Live-Query, `--input` eine lokale Datei — die Brücke fehlt).
- Die 4 restlichen Compiler `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` printen keinen vollständigen Block → Format-Name nötig (Mountain); `opentopography_raster` kollidiert mit dem vorhandenen `SRTM15_V2.6`-Block (`phi:29896`). (Die 4 registrierten Ersatzquellen `osc`/`chime_frb_cat2`/`lamost_dr10`/`cfa_co` sind mit `405e4d6a2` gefaltet.)
[manifest n=1 root=c28e0b91]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
