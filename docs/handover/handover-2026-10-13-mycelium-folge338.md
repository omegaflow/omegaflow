<!--
  title: Handover — Mycelium-Folge 338 (2026-10-13)
  session: Mycelium-Linie in einem Pass — die 4 ableitbaren Ersatzquellen-Compiler in phi/sources.φ registriert (osc/chime_frb_cat2/lamost_dr10/cfa_co, url/format/origin/compiler, Mycelium-Klasse); Insertion zurückgezogen und neu gesetzt, um den konkurrierenden Mountain-Schreiber (Commit e48703f94) nicht zu kreuzen; mountain-Token .1/.2 gefaltet (8-Ziele-Liste, xfer migrate); Stehender Pass am HEAD
  class: handover
  date: 2026-10-13
  sha256: 66a2fe468f716a30916c9aefe5697008f410e20aec5c6bf6aab8ab7a11448c37
  status: live
-->
# Handover — Mycelium-Folge 338 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge337.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.0758 — gemessen `opencode.db` Session `ses_ecd958500ffeeGumcGExqQNmPS` beim Handover-Schreiben; deepseek-flash, kein pro/max.

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
- **Trigger:** die `terms`/`at`/`field`-Zeilen Mountains je Quelle
- **Lage:** (gemessen 2026-10-13/338) **4 Blöcke registriert** in `phi/sources.φ` (Mycelium-Klasse `url`/`format`/`origin`/`compiler`±`ttl`): `osc` (`sne.space/osc_transients.bin`), `chime_frb_cat2` (`www.canfar.net/chime_frb_cat2.bin`), `lamost_dr10` (`www.lamost.org/lamost_dr10.bin`), `cfa_co` (`zenodo.org/h2_dens_mean_beg03.volume.bin`). **4 Compiler printen keinen vollständigen Block** (kein `FORMAT`/`COMPILER`-const, `--out` Pflicht): `heasarc_w3browse` (`heasarc.gsfc.nasa.gov`), `jpl_sbdb` (`ssd-api.jpl.nasa.gov`), `gesdisc_gnssro` (`data.gesdisc.earthdata.nasa.gov`), `opentopography_raster` (`topex.ucsd.edu`, `format srtm`, `url`-Asset = `--out`-Basename; daneben steht bereits `SRTM15_V2.6` in `phi:29896` → Riss).
- **Blockade:** die `terms`/`at`/`field`-Zeilen sind Mountains Recht; die 4 Restcompiler brauchen eine Format-Entscheidung (Parser-Arm/Format-Name).
- **Braucht:** Mountain: `terms`/`at`/`field` je Quelle, dann die 4 Restcompiler typisieren + registrieren. Token `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.1~9ff1374e`.

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

- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.1~9ff1374e` **terms/at/field für die 4 registrierten Ersatzquellen + Format-Design der 4 Restcompiler** — in `phi/sources.φ` registriert (url/format/origin/compiler): `osc` (sne.space), `chime_frb_cat2` (www.canfar.net), `lamost_dr10` (www.lamost.org), `cfa_co` (zenodo.org). Bitte je Quelle `terms`/`at`/`field` setzen. Die 4 Compiler `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` printen keinen vollständigen Block → Format-Name nötig; `opentopography_raster` kollidiert mit dem vorhandenen `SRTM15_V2.6`-Block (`phi:29896`).
[manifest n=1 root=9ff1374e]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
