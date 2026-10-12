<!--
  title: Handover — Mycelium-Folge 327 (2026-10-12)
  session: Mycelium-Linie in einem Pass (INPOP/NOE4-Fix, adressierte Faltungen, Stehender Pass)
  class: handover
  date: 2026-10-12
  sha256: f441fd7b4d6e139c514e30ae6c6fba66ce16b76eea874625461a80e6065cf50f
  status: live
-->
# Handover — Mycelium-Folge 327 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-12-mycelium-folge326.md` (→
`archiv/`) und faltete die adressierten Blöcke `future-folge242` und `mountain-folge327`
**mit Token** (die folge326-Faltung war unvollständig: `xfer:mountain327-1`- und
`-2`-Einträge fehlten im Live-Handover).

## Burn: open 0.00 · close 0.13 — opencode.db Session-Cost 0.1267 (Live-Flush, läuft nach); Mycelium-Folge 327 in einem Atom (INPOP/NOE4-Fix + BC430-Register/Mirror + adressierte Faltungen + Stehender Pass), deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt … entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ist das wirklich alles Trigger oder was kannst du davon selbst bis zur Kante abarbeiten?" | 2026-10-12 | Operator (Session 326) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-12 | Operator (Start Mycelium-326/327) · session-weiter Delegations-Consent, nicht das Commit-Wort |

## Gefaltet (gemessen 2026-10-12)

**`future-folge242` (adressiert, ohne Token — Altblock):** vollständig gefaltet.
- USGS-Kombi-Mosaik `Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`: der `future-242`-Wert
  (`1a90a8ff…`/66 414 088 B) ist **ungültig**; gemessen `324c0e49…`/1 880 562 475 B. Der
  Registerblock `phi:31582-31586` trägt **keine `sha256`-Zeile** → als Ask an Mountain
  (`xfer:mycelium327-1` unten); der M10-Mosaik-Lauf `38237291082` ist **success** (8 Jobs grün).
- `archive_search --all`-Spill: **geschlossen** — `OMEGAFLOW_SPILL_DIR` wird bereits honoriert
  (`tools/utils/src/bin/archive_search/net.rs:3780`).
- NIST NCNR NSE: **geschlossen** (Lauf `38233919420` grün; folge326).
- Fetch-Provenienz Sidecar `<datei>.src`: **offen** → eigener Punkt unten.
- `de441-de442-sha`: → Ask an Mountain (`xfer:mycelium327-2` unten).
- `dropped-gate-move-disposition`: eigener Punkt unten (gemessen).

**`mountain-folge327` (adressiert):**
- `xfer:mountain327-1` 8-Ziele-Einkaufsliste (future-241) → eigener Punkt unten.
- `xfer:mountain327-2` Orphan-Doc `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` → Trägerpunkt unten.
- M10-Kamera/ISIS-Kette (ASU `url` `phi:31568`, `terms`/`format`, `camera_angles`-Parser-Arm),
  DSC 0119, JADES, BK18, FMI-NUR, NIST, USGS-Mosaik: registriert (folge326) bzw. Mountain-Ask.
- **Korrektur:** folge326 trug „Sonke/Robinson-Tabellen 1–8" als Mycelium-`blockiert` mit
  `Braucht: archive_search`. Gemessen (mountain-327:65-71): die ASU-`url`-Zeile `ser.im-ldi.com/M10/M10_ISIS/`
  ist **registriert** (`phi:31568`); die Tabellen 1–8 sind **paper-only** (voller cnet-Satz) —
  kein messbarer URL-Satz, letzte Meile Operator-Hand → `## An future`.

## Offen — eigen

### INPOP/NOE4 — `--chain` fehlte; Workflow gefixt, Lauf dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der neue `ephemeris-inpop-noe4-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38236601893`) Lauf `38236601893` **failure**:
  `noe4: phobos (target 401) carries no granule — no SSB chain (--chain planetary kernel absent) — skipped`,
  `0 body line(s) compiled … exit 1`. NOE-4-Segmente messen `target 401/402 center 4` (Mars-Baryzentrum),
  **kein** 4→SSB-Segment. **Gefixt:** `.github/workflows/ephemeris-inpop-noe4-cdn.yml` lädt jetzt
  `kernels/de440s.bsp` und ruft `noe4_compiler kernels/NOE-4-2020.bsp --chain kernels/de440s.bsp --ci-mode`
  (kanonisches de440s-Muster aus `messenger-ephemeris-cdn.yml`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neue run-id>` — grün = CDN-Asset `ephemeris_noe4_phobos/deimos.bin`.

### GRDC-Caravan — Resume-Fix gebaut; Verifikationslauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf `38234294097` endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) Lauf `38234294097` (head `934c131f3`) **in_progress**;
  Resume-Fix (`src/archivar/fetch.rs`) steht.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 38234294097`.

### CI-Messzeilen — Werkzeug-Register gebaut; Pilot-Lauf queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `ci-measurements-fold`-Lauf endet
- **Lage:** (gemessen 2026-10-12) `tools/measure/data/ci_measurements.φ` (Skelett) +
  `.github/workflows/ci-measurements-fold.yml` (`workflow_run` auf `field-te-query`, `block_commit`
  privat-Index + CAS). Pilot `field-te-query` `38238626309` (head `43f9c83d0`) **queued**.
- **Blockade:** der erste Fold-Push über `OMEGAFLOW_TOKEN`/`block_commit` ist ungeprüft.
- **Braucht:** nach dem Lauf `ci_manage log <fold-run-id>` + `sread tools/measure/data/ci_measurements.φ`.

### 8-Ziele-Einkaufsliste — BC430 registriert + Mirror dispatcht; P10/P11/Venus offen `xfer:mountain327-1`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `zenodo-raw-cdn`-Lauf (BC430) endet; die Zenodo-`content`-Endpunkte für Pioneer 10/11 antworten wieder
- **Lage:** (gemessen 2026-10-12) **BC430** Zenodo `3558850`: sha256 `0b2da752…` über 59 541 013 B (via
  `curl -sSL` + `sha256sum`) → **registriert** (`phi:32389-32394`, kanonische Position nach `zcta_gazetteer`,
  vor `jades_early_galaxies`) + Workflow `.github/workflows/zenodo-raw-cdn.yml` (Matrix, `raw_mirror`,
  `# harvest: format=raw_zenodo`), Lauf durch den Push getriggert. **Pioneer 10** `13309156` / **11**
  `13309040`: der `…/api/records/<id>/files/<name>/content`-Endpunkt antwortet **504 Gateway Time-out**
  (gemessen via `curl`) → sha nicht messbar → nicht registrierbar. **Venus-Radar** `18176179`:
  `Venus_RadarBasemap_2025_Global_75m_v1.tif` 80 909 342 816 B — schwerer Mirror, eigener Schritt.
  Nicht-cc-by (PDS-Geosciences Venus-Radar, PDS-PPI `PVO-V-ORSE-1-ODR-OPENLOOP--V1.0`, JPL Spacecraft-VLBI):
  `terms` unbestimmt → Mountain.
- **Blockade:** P10/P11 an der Zenodo-Verfügbarkeit; Venus an der Dateigröße. Register-Ordnung: ein
  `register_sort --write` würde 175 fremde (inpop-)Zeilen sweepen → der BC430-Block wurde manuell an
  kanonischer `(ttl,url)`-Position gesetzt und per Sort-Dry-Run auf Relokationsfreiheit verifiziert.
- **Braucht:** `ci_manage log <zenodo-raw-cdn-run-id>` (BC430-Mirror); für P10/P11
  `curl -sSL --max-time 900 -o /tmp/opencode/<name> '<content-url>' && sha256sum /tmp/opencode/<name>`
  sobald Zenodo antwortet, dann je Block + Matrix-Zeile; Venus-Mirror als eigener Schritt.

### Orphan-Doc-Träger — `xfer:mountain327-2`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = **0**; Träger dieser Zeile ist
  `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2 offene Marker; see-also
  `docs/concepts/ui-seats.md`, `docs/SOURCE_PORT.md`). Die Survey misst Seats/Suchwege; ihr Fazit ist
  „kein neuer Science-Daten-Arm" (Chatbot-UIs + Agent-APIs; `archive_search` deckt breiter ab).
- **Blockade:** keine.
- **Braucht:** die zwei offenen Marker der Survey gegen den Baum messen (Seat-Kandidaten → `ui-seats.md`,
  Architektur → `SOURCE_PORT.md`) und schließen oder als eigenen Schritt führen.

### Fetch-Provenienz — Sidecar `<datei>.src` beim Fetch
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `archive_search`-Pass
- **Lage:** (gemessen 2026-10-12/327) die Taucher schreiben die Quell-URL nicht beim Fetch mit; Ableitung
  nur via `opencode.db` (`state/research/README.md` §Provenienz, 76 Dateien belegt).
- **Blockade:** keine.
- **Braucht:** `archive_search` (Sink-Pfad) schreibt je heruntergeladener Datei ein Sidecar `<datei>.src`
  mit der Quell-URL; `cargo check` + ein Unit-Test.

### dropped-gate-move-disposition — Punkt-ID/Hash-Baseline offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Baseline-Vergleich (`278ba2905`)
- **Lage:** (gemessen 2026-10-12) lokal `register_lookup --dropped --count` = **8** (unter der
  Baseline `docs/zustand/dropped-baseline.md` = 1300 → kein Gate-Trip); `--dropped-roster --count` = **52**;
  `--dropped-keys --count` läuft lokal in den Timeout (historie-schwer; die Gate-Zahl ist CI-only).
  Herkunft: `state/zustand/wartend-archiv-2026-10-11-r3.φ:25` (924 keys, `278ba2905`).
- **Blockade:** die lokale und die CI-Menge messen Verschiedenes (historisch belegt).
- **Braucht:** die 8 unaufgelösten `--dropped`-Punkte gegen den Baum prüfen (legitimer Archiv-Move →
  Baseline-Bump im annehmenden Commit, sonst echter Punkt).

### ui-seats — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui`
- **Lage:** (gemessen 2026-10-12) die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) ·
  Gemini Notebook (kein freier Composer) noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen und `docs/concepts/ui-seats.md`
  fortschreiben.

### tools-map.md — Träger (Operator-Wort Debugger-Rechte)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort (Debugger-Rechte am live Chrome)
- **Lage:** (gemessen 2026-10-12) `docs/concepts/tools-map.md:349`: „offen bleibt allein das
  Operator-Wort (Debugger-Rechte am live Chrome)"; die Route ist gebunden (`opencode.json:172-178`,
  Commit `7e79c194d`). Träger-Zeile für `--orphan-docs` (die folge326-Zeile ist nach `archiv/` gewandert).
- **Blockade:** der Akt ist operator-gebunden.
- **Braucht:** das Operator-Wort — liegt in Futures Operator-Queue (`## An future`).

## An mountain

Origin: mycelium-folge327 (2026-10-12). Der `future-242`-Block war bei folge326 nur als Kopfzeile
„gefaltet" (Token-Pflicht verletzt); hier einzeln gefaltet.

- **M10↔MDIS-USGS-Mosaik — `sha256`-Zeile fehlt.** Der Lauf `38237291082` ist **success** (8 Jobs grün);
  der Block `phi:31582-31586` (`url …/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`, `terms PD`, `origin`,
  `compiler raw_mirror.rs`, `no-cadence`) trägt **keine `sha256`-Zeile**. Gemessen (Mountain-326):
  `324c0e49…` über 1 880 562 475 B. `xfer:mycelium327-1`
- **`de441-de442-sha`** — die NAIF-Blöcke (`phi:32008`, recursive SPK/PCK harvest) stehen; `sha256`
  erst nach dem Merge-Lauf (Mountain-Territorium). `xfer:mycelium327-2`

## An future

Origin: mycelium-folge327 (2026-10-12). Operator-gebundener Rest (No-referral → Futures Operator-Queue).

- **Sonke/Robinson (M10) Tabellen 1–8 — paper-only, letzte Meile Operator-Hand.** Gemessen (mountain-327):
  Sonke & Robinson 2026, PSJ 7:35, `10.3847/PSJ/ae32f5` — **OA gold, CC-BY** (`--unpaywall`, host publisher);
  der volle cnet-Satz ist nicht maschinell erreichbar (`ser.im-ldi.com/M10/M10_ISIS/` = 403, kein Listing;
  die ASU-`camera_angles.txt`-`url` ist registriert `phi:31568`). Wird gebraucht für das M10-MDIS-Pose-Wiring
  (Mountain/River). Möglichkeit: die Autoren anschreiben (Operator-Hand, kein Maschinen-Send) **oder**
  mit ISIS nachrechnen. `xfer:mycelium327-3`
- **Debugger-Rechte am live Chrome** — Operator-Wort nötig (`docs/concepts/tools-map.md:349`); ohne es
  bleibt die DevTools-MCP-Bindung (`opencode.json:172-178`, Commit `7e79c194d`) ungenutzt. `xfer:mycelium327-4`

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
