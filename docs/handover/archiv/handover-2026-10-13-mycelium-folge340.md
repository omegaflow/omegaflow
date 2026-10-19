<!--
  title: Handover — Mycelium-Folge 340 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: 2fadc505797a74a014eb634e5c5135bfd1219ee4517121a151f484eebacc632b
  status: live
-->
# Handover — Mycelium-Folge 340 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-13-mycelium-folge339.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.0350 — gemessen `opencode.db` Session `ses_ecbe9e817ffefb1Wu55Ub2Ib8X` beim Handover-Schreiben (offen; wächst bis Session-Schluss); deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „hast du die ttls gewürfelt?" | 2026-10-13 | Operator (Session 339) · `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „mache weiter" (weitergetragener folge337-Bericht) | 2026-10-13 | Operator (Session 338) · `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | Operator (Session 337) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater … /commit (Push), danach den frischen Pass" | 2026-10-12/13 | Operator · session-weiter Delegations-Consent |
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok — für mich ist Fachjargon Vertuschung und unnütz" | 2026-10-13 | Operator (Session 340) · `state/operator-gespraeche/2026-10-13-mycelium.md`; gefaltet ins `AGENTS.md` (Language doctrine) |

## Offen — eigen

### Ersatzquellen-Registerblöcke — Format der 4 Restcompiler (Klassen-Träger)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** ein Format-Name je Restcompiler von Mountain
- **Lage:** (gemessen 2026-10-13/339) die 4 Compiler `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` printen keinen vollständigen Block (kein `FORMAT`/`COMPILER`-const, `--out` Pflicht); `opentopography_raster` kollidiert mit dem vorhandenen `SRTM15_V2.6`-Block (`phi:29896`). An Mountain getragen: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.
- **Blockade:** Format/Format-Name ist Mountains Recht (Quellen-Identität).
- **Braucht:** Mountain typisiert die 4 Restcompiler + registriert sie.

### LHAASO/VizieR `J/ApJS/271/25` (Ersatz für `casdc`)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `at`/`field` + das CI-Fetch-Wiring
- **Lage:** (gemessen 2026-10-13/338) Block `phi/sources.φ:10799-10802` (`url .../vizier.cds.unistra.fr/lhaaso_sky1.sky1`, `format sky1`, `origin` = ASU-TSV `J/ApJS/271/25`, `compiler lhaaso_compiler.rs`). Der Compiler lädt nach `vizier.cds.unistra.fr` und liest die ASU-TSV (`#`-Preamble, `RAJ2000`/`DEJ2000`/`N0`); `cargo build -p omegaflow-harvest --bin lhaaso_compiler` grün. An Mountain: `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`.
- **Blockade:** `at`/`field` = Mountains Recht; **kein CI-Workflow** ruft `lhaaso_compiler` (Query→`--input`-Brücke fehlt).
- **Braucht:** Mountain `at sun`/`field`; danach das Fetch-Wiring.

### `register_sort` — Kanonisierung fehlt (`--write`)
- **Status:** eigen (Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Lauf `register_sort --write phi/sources.φ`
- **Lage:** (gemessen 2026-10-13/340) die fehlende ttl ist von Mountain gesetzt — `544b136f3` („measured ttl for the 5 cadence blocks": osc live API 86400; chime_frb_cat2/lhaaso no-cadence; lamost_dr10/cfa_co no-cadence statt des willkürlichen 31536000) — der **ttl/Cadence-Riss ist geschlossen**. Es bleibt allein die Ordnung: auf der ttl-ergänzten Fassung exit 1 — **4 ttl-order + 22 url-order Verletzungen** über 2591 Blöcke (u. a. `ceein` nach `chime_frb_cat2`, `jvo`-akari nach `lamost_dr10`, mehrere no-cadence-Blöcke). Die Ordnung ist durch viele Linien gewachsen, nicht durch eine.
- **Blockade:** `phi/sources.φ` ist Mountains Register; eine `--write`-Kanonisierung ist ein Ganz-Datei-Eingriff (Kollision mit laufenden Schreibern).
- **Braucht:** Mountain fährt `register_sort --write` in einem ruhigen Atom.

### OCNS2019 — geoazur/APDB-Ersatz (Alternative gefunden; Lizenz-Term offen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms` (China-VO PaperData) + die Host-Route (Wayback)
- **Lage:** (gemessen 2026-10-13/340) **Die Alternative ist gefunden** — Future-247/248: OCNS2019 ersetzt das entfernte `geoazur` (APDB, all-rights-reserved, gelöscht). OCNS2019 („Observational Catalog of Neptunian Satellites 2019"; 24 996 Positionen, 11 Neptun-Monde, 1847–2019, ICRS/TT; DOI `10.12149/101048`; NADC `nadc.china-vo.org/res/r101048/`) ist der moderne Superset, gefunden erst mit `archive_search --all` (109/114 Modi). Open ist **nicht die Alternative, sondern**: (a) der **Lizenz-Term** — DataCite trägt kein `rightsList` (kein CC-BY), Authority China-VO/NADC → **Mountain** setzt `terms` (China-VO-PaperData-Token bzw. Lizenzklärung); (b) der **Host** `paperdata.china-vo.org` antwortet pending — erreichbar nur über den Wayback-Snapshot `web.archive.org/web/20220609074246/…` (sha `7b7ad66b…`); (c) der **Block + das Feld-Mapping** (Mycelium) + ein Compiler (`neptune_apdb_manifestor.rs` ist auf die alten APDB-Files hardcodiert). **Kein `rightsList` ≠ keine Alternative** — meine folge339-Formulierung „keine lizenzklare Quelle" war zu scharf. Gefaltet: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.10~7d5c9163`.
- **Blockade:** der Lizenz-Term (Mountain) und der Host (nur Wayback).
- **Braucht:** Mycelium baut den OCNS2019-Block + Feld-Mapping (bounded step); Mountain setzt `terms`.

### Lizenzklare Alternativen — Register-/Compiler-Swap (future-248, vier Wellen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** je Alternative ein Compiler + Block gebaut
- **Lage:** (gemessen 2026-10-13/340) Future hat **nicht nur 8 Ziele**, sondern die **ganze Alternativ-Landschaft** recherchiert — `state/future/licence-alternativen-all-2026-10-12.md` (116 Zeilen, **vier Tabellen-Wellen, ~70 Zuordnungen** Observable → Alternative → terms → Beleg): (1) Tier-A-Hosts ohne Register-Ersatz (28 Zuordnungen, Z. 15–42), (2) Nachlauf mit dem 111-Modus-Tool (19, Z. 64–82), (3) JWST-Riss + Einzel-Observablen (15, Z. 88–107), (4) ArcGIS-Reste (~12, Z. 109). Fast jede lizenzlose Quelle hat eine **freie institutionelle Alternative** (NASA PD · Zenodo CC-BY-4.0 · VizieR · CTBTO/BGR CC-BY-NC-SA); Fazit der Tabelle: der Operator muss nicht verklagt werden. Die 4 direkt-downloadbaren cc-by-4.0 (BC430 `3558850`, Pioneer 10/11 Doppler `13309156`/`13309040`, Venus-Radar `18176179`) + PDS + PVO-ORSE + JPL SSD sind **ein Teilausschnitt** davon. `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362`; Mountain-Tokens: 8-Ziele echo `xfer:mountain.ses_ece7fc36bffeSJbFG4SLeLvRrR.1~0d43166a`; FUGIN+CfA-CO echo `xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.1~1ea1cb21`; `xfer migrate` echo `xfer:mountain.ses_ece7fc36bffeSJbFG4SLeLvRrR.2~1003fb06`.
- **Blockade:** `url`/`origin`/`compiler` ist Mycelium, `terms`/`field`/`ttl` Mountain; je Block ein Feld-Mapping.
- **Braucht:** **konkrete Bau-Liste gemessen (2026-10-13/340): `state/mycelium/alternativen-endpunkte-2026-10-13.md`** — je Zeile der maschinenlesbare Endpunkt + Format: **~20 `block`** (direkt baubar: HEASARC-TAP, VizieR-TAP, Zenodo-File-API, USGS/NOAA/EPA-REST, JPL-SBDB/Horizons, CDAWeb-HAPI) · **~30 `compiler`** (Endpunkt steht, neuer Leser nötig) · **die fehlenden Adressen ausgelesen (2026-10-13/340, drei Taucher): fast alle aufgelöst** (KiDS 17086764, MAGIC PDR1 13898269, HI4PI 5500447, CfA-CO 5501196, DAWN 22734795, MIRAGE 21990891, specsr/miri_wfss/jwstComet 23192226/22803722/20610727, COSMIC-RO/IGETS/BGR/FIRMS/WorldClim/WMM/EDGAR/ECCC/Argo/gamma-cat/EMM/CLPDS/PALAOA/iris je Endpunkt); **die 5 „ohne Adresse" sind korrigiert (gemessen `state/` + `/tmp/opencode`): sie haben eine Adresse — sie stehen bereits im Register, offen ist nur die Lizenz:** LAMOST DR10 `phi:21857` **CC-BY-4.0** (schon offen), CHIME/FRB Cat 2 `phi:10806` **CC-BY-4.0** (schon offen), JADES `phi:11399` no-license (Zenodo-DR5 `18328404` als Ersatz), Sheffield-NO₂ `phi:21415` no-license (OpenAQ/EPA-Ersatz), KASI `phi:10956+` no-license (kein PDS-Ersatz). Je Alternative ein bounded step (Block bauen, dann Mountains `terms`/`field`). **Wellen 1+2 gebaut (2026-10-13/340, 8 parallel Taucher):** 8 Entwurfs-Dateien `state/mycelium/block-drafts/welle{1,2}-*.φ` (54 Blöcke, je `register_sort`-kanonisch), mit ehrlichen Verdikten — teils **neu**, teils **bereits registriert** (LAMOST, CHIME, BGR, CLPDS, COSMIC-RO, OpenAQ, iris, KARI, GOES-EUVS, HiPS — kein Duplikat), teils **Oszillator-Gate-declined** (GDPS-Forecast, CAMS-Reanalyse, EDGAR-Inventar = Modell/Reanalyse, gehören nicht ins Register). **Risse gemessen:** VizieR II/288 · III/279 · J/ApJS/257/59 → 400 (Subtabellen-ID nötig); ICGEM `getmodel` braucht konkreten Hash; gamma-cat hat keine Einzeldatei; PANGAEA 967167 trägt CC-BY-4.0 (nicht CC-BY-3.0). **Gebaut (2026-10-13/340):** **43 Blöcke blockweise eingefaltet** (`faa8575`, 10 Duplikate verworfen), `phi/sources.φ` bleibt **kanonisch** (exit 0, 2549 Blöcke — eine andere Linie hat zwischenzeitlich sortiert und ein `terms`-Gate ergänzt); **6 fehlende Leser gebaut** (`43f791384` ascii/parquet/zip · `7ccb8f55` ghcnm_tavg/open_meteo_archive/openaq), `cargo check --bin` 0/0; **`compiler`-Zeilen verdrahtet** (`a514596`, 15 Zeilen). Alle gepusht (HEAD `2a57f43fb` == `origin/main`). **Bleibt:** Mountain prüft `terms`/`field`/`ttl` der 43 Blöcke (Register-Recht) · die CDN-Manifestation (CI) · die 4 Risse (VizieR-Subtabellen-IDs II/288·III/279·J/ApJS/257/59, ICGEM-Hash, gamma-cat ohne Einzeldatei, OCNS2019-Lizenz-Term).

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/340) auf den `archive_search --help`-Stand gesynct; `--orphan-docs` = 15 (keine mycelium-Datei). Ruht.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

## An mountain

Origin: mycelium-340 (2026-10-13):

- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` **Format-Name für die 4 Restcompiler** `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` (printen keinen vollständigen Block; letzterer kollidiert mit `SRTM15_V2.6`, `phi:29896`).
- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **`at`/`field` für den LHAASO/VizieR-Block + CI-Fetch-Wiring** (`phi/sources.φ:10799-10802`); kein CI-Workflow ruft `lhaaso_compiler`.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + Risse** (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 2 Schranken** — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig, 4LAC als eigene Quelle oder gestrichen?
- **`register_sort` Kanonisierung:** `register_sort --write phi/sources.φ` (4 ttl-order + 22 url-order Verletzungen auf der ttl-ergänzten Fassung). `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.3~8eff6a4e`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
