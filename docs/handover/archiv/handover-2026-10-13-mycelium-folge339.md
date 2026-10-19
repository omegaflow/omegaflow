<!--
  title: Handover — Mycelium-Folge 339 (2026-10-13)
  session: Mycelium-Linie in einem Pass — CI-Zensus via ci_manage reds (44 Nicht-success, 4 auf HEAD deefc4c92); register_sort-Grund gemessen (osc-Block ohne ttl, exit 2), gewürfelte ttl zurückgenommen (Riss, an Mountain); clippy/format/dropped-gate/matrix-rotor-Gründe je Träger benannt; XFER-MISSING (untokener An-mountain-Eintrag) mit Token 73d3d0af geschlossen, ttl-Token accd94dd, 2 fertige Token CLOSED; future-248 adressierten Block gefaltet
  class: handover
  date: 2026-10-13
  sha256: b77999f2041612f545c9bddbb17bcc1504c06e8075a92e5229fa55f6718f6b22
  status: live
-->
# Handover — Mycelium-Folge 339 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-13-mycelium-folge338.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.0707 — gemessen `opencode.db` Session `ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph` beim Handover-Schreiben (offen; wächst bis Session-Schluss); deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „hast du die ttls gewürfelt?" | 2026-10-13 | Operator (Session 339) · `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „mache weiter" (weitergetragener folge337-Bericht) | 2026-10-13 | Operator (Session 338) · `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | Operator (Session 337) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 337) · session-weiter Delegations-Consent |

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

### `ttl`/Cadence — Rücknahme einer Schätzung (Riss)
- **Status:** eigen (Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `ttl`/`no-cadence`-Verdikt je Block
- **Lage:** (gemessen 2026-10-13/339) ich hatte `ttl 3600` (osc) / `ttl 604800` (lhaaso, chime_frb_cat2) gesetzt — **aus den Nachbarblöcken übernommen, nicht gemessen** (Operator-Wort „hast du die ttls gewürfelt?"); zurückgenommen, `phi/sources.φ` hinterlässt keinen eigenen Hunk. Die `ttl 31536000` auf `lamost_dr10`/`cfa_co` (folge338) sind ebenso ungemessen (Riss). `register_sort` bricht ohne ttl ab (exit 2, `block has no ttl line (opening osc_transients.bin)`). An Mountain: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd`.
- **Blockade:** `ttl` ist Mountains Register-Recht.
- **Braucht:** Mountain setzt `ttl`/`no-cadence` für `osc` (live API), `chime_frb_cat2`/`lhaaso` (statische Kataloge) und prüft `lamost_dr10`/`cfa_co`.

### `register_sort` — Kanonisierung fehlt (`--write`) [neu gemessen]
- **Status:** eigen (Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Lauf `register_sort --write phi/sources.φ`
- **Lage:** (gemessen 2026-10-13/339, lokal `./target/release/register_sort phi/sources.φ`) ohne die fehlende ttl: exit 2 (Parse-Abbruch). Auf der ttl-ergänzten Fassung: exit 1 — **4 ttl-order + 22 url-order Verletzungen** über 2591 Blöcke (u. a. `ceein` nach `chime_frb_cat2`, `jvo`-akari nach `lamost_dr10`, mehrere no-cadence-Blöcke). Die Ordnung ist durch viele Linien gewachsen, nicht durch eine.
- **Blockade:** `phi/sources.φ` ist Mountains Register; eine `--write`-Kanonisierung ist ein Ganz-Datei-Eingriff (Kollision mit laufenden Schreibern).
- **Braucht:** Mountain fährt `register_sort --write` in einem ruhigen Atom.

### OCNS2019 (geoazur/APDB-Ersatz) — Riss (keine lizenzklare Quelle)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein Wort zur Lizenz (oder NAOJ/Géoazur-Erlaubnis)
- **Lage:** (gemessen 2026-10-13/338) `geoazur` (APDB) ist entfernt. Ersatz **OCNS2019** („Observational Catalog of Neptunian Satellites 2019"; 24 996 Positionen, 1847–2019, ICRS/TT; DOI `10.12149/101048`) trägt **keine offene Lizenz** (NADC: copyright belongs to the original data authors; kein CC-BY); Download-Host `paperdata.china-vo.org` antwortet pending (nur Wayback `7b7ad66b…`); kein Compiler. Future-248 brachte denselben Punkt als „publicly available" (Feld-Mapping) — der Riss steht: die Lizenz-Premisse hält nicht. Gefaltet: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.10~7d5c9163`.
- **Blockade:** die Lizenz-Premisse („lizenzklar") hält nicht; Host pending.
- **Braucht:** Entscheidung (Mountain/Future) — Géoazur/NAOJ um Erlaubnis, as-is führen, oder fallenlassen.

### 8-Ziele-Einkaufsliste (future-241) / lizenzklare Alternativen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Ziel ein Compiler gebaut
- **Lage:** (gemessen 2026-10-13/339) die 4 direkt downloadbaren cc-by-4.0 (BC430 `3558850`, Pioneer 10/11 Doppler `13309156`/`13309040`, Venus-Radar `18176179`) + PDS-Geosciences Venus-Radar + PVO-ORSE + JPL SSD: je `url`/`origin`/`compiler`-Zeile. Future-248 meldet die lizenzklaren Alternativen als Bau/Ersatz-Auftrag: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362`. Mountains 8-Ziele-Token: echo `xfer:mountain.ses_ece7fc36bffeSJbFG4SLeLvRrR.1~0d43166a`; FUGIN+CfA-CO (FUGIN offline durch Future, CfA-CO-Block `cfa_co` registriert): echo `xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.1~1ea1cb21`; `xfer migrate` (PATH trägt `migrate` jetzt): echo `xfer:mountain.ses_ece7fc36bffeSJbFG4SLeLvRrR.2~1003fb06`.
- **Blockade:** je Ziel fehlt der Compiler; `url`/`origin`/`compiler` ist Mycelium, `ttl`/`field` Mountain.
- **Braucht:** Compiler je Ziel (je ein bounded step), dann die Registerzeile.

### CDN-Assets >2 GiB (`m10-mdis-raw-cdn`) — Split im Release
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-build`-Release trägt den Split (`fetch_release_or_parts`)
- **Lage:** (gemessen 2026-10-13/339) der Split (`cdn.rs`/`fetch.rs`/`raw_mirror.rs`, Mountain) steht; Betroffener Lauf `38275761488` scheiterte an `Mercury_MESSENGER_MDIS_…_166m.tif` (3,96 GiB, HTTP 422 = 2-GiB-Asset-Limit). Echo `xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.2~b2908226`. `fetch_release_or_parts` auf PATH prüfen.
- **Blockade:** der Split liegt noch uncommitted im Baum → nicht im Release.
- **Braucht:** nach Mountain-Commit + Release `gh workflow run m10-mdis-raw-cdn.yml` neu dispatchen und messen.

### Stehender Pass — `ci_manage reds` jetzt im Release [erledigt, Echo bleibt]
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-13/339) `ci_manage reds [--pages N]` antwortet jetzt (`--help` listet `reds`); der Pass zitiert es statt eines Fensters. Echo `xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.3~00ba30f9`.
- **Blockade:** keine.
- **Braucht:** nichts; Echo trägt den Mountain-Token.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/339) auf den `archive_search --help`-Stand gesynct; `--orphan-docs` = 14 (keine mycelium-Datei). Ruht.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

## An mountain

Origin: mycelium-339 (2026-10-13):

- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` **Format-Name für die 4 Restcompiler** `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` (printen keinen vollständigen Block; letzterer kollidiert mit `SRTM15_V2.6`, `phi:29896`).
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd` **`ttl`/`no-cadence` für `osc`/`chime_frb_cat2`/`lhaaso`** (ich hatte gewürfelt — zurückgenommen); `register_sort` exit 2; die `31536000` auf `lamost_dr10`/`cfa_co` sind ebenso ungemessen.
- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **`at`/`field` für den LHAASO/VizieR-Block + CI-Fetch-Wiring** (`phi/sources.φ:10799-10802`); kein CI-Workflow ruft `lhaaso_compiler`.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + Risse** (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 2 Schranken** — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig, 4LAC als eigene Quelle oder gestrichen?
- **`register_sort` Kanonisierung:** `register_sort --write phi/sources.φ` (4 ttl-order + 22 url-order Verletzungen auf der ttl-ergänzten Fassung). `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.3~8eff6a4e`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
