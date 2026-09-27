<!--
  title: Handover — Mycelium-Folge 187 (2026-09-27)
  session: Mycelium-Folge 187
  class: handover
  date: 2026-09-27
  sha256: 3f3a2236548a370e6a762d2e98685b934b4e3bac578d34176dd3ee208d73fe2c
  status: live
-->
# Handover — Mycelium-Folge 187 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `eigen` | `wartend` |
`blockiert` | `termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits
in `state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge186.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent (Delegation), **nicht** das Commit-Wort | Operator (Mycelium-Session 187).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | Entscheidungen nie als Liste vorlegen — jede Entscheidung braucht eine aussagekräftige Erklärung | Operator (Future-Session).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).
- Wort | 2026-09-27 | „falte alle" — die genuin-offenen Punkte der trägerlosen Docs in die Übergaben ihrer Linien falten (Aufenthalt = Eigentum) | Operator (Mycelium-Session 184).
- Wort | 2026-09-27 | „den rest gebe ich future" — die Tafel trägt nur `eigen`; operator-gebundene Punkte → Future-Operator-Queue, Dritt-Waits → `state/zustand/wartend.φ`; die Queue wird nicht kopiert | Operator (Mycelium-Session 185).

## Offen (aufgeschlüsselt)

### register-dropped — Timeout schneidet den Full-History-Sweep ab
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch `gh workflow run register-dropped.yml` nach diesem Commit.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log`/`jobs 36340753083`) der Job `sweep` startete 18:28:08, wurde 18:58:25 `cancelled` (genau 30 min) und terminierte den laufenden `register_lookup`-Prozess; `.github/workflows/register-dropped.yml` trug `timeout-minutes: 30`. Die lokale Messung (`grind-flash`, 2026-09-27) ergab ≈5,4 s/Eintrag × 1023 ≈ 1,5 h — der Sweep passt nicht in 30 min.
- **Blockade:** keine (Fix in diesem Atom: `timeout-minutes: 30 → 180`).
- **Braucht:** nach dem Push `gh workflow run register-dropped.yml`; Ergebnis **einmal** lesen (`ci_manage log <id>`), nie pollen.

### dropped-gate — Baseline 989 vs current 1032
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `register-dropped`-Lauf erfolgreich (nach dem Timeout-Fix).
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36333267310`) `dropped-gate: baseline 989 | current 1032 | delta 43`; `docs/zustand/dropped-baseline.md:16` = 989 @0a0ce96d. Die 43 Namen nennt das Log nicht (unread); lokal ist `register_lookup --dropped` nicht fahrbar (≈1,5 h; nur `future` lief durch).
- **Blockade:** keine (Sweep über den Timeout-Fix erreichbar).
- **Braucht:** die 43 je Owner aus `ci_manage log <register-dropped-id>` auftragen; legitime Drops ins annehmende Handover, dann Baseline 989 → 1032 bumpen (nie still).

### CI-check — Zählung + Transfer (Mountain bereits getragen)
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** neuer `ci-check`-Lauf am aktuellen HEAD.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36333267310`) der rote `ci-check` trug im `test`-Job **29** Fehler (1855 passed; 29 failed; 33 ignored; 3239,88 s) — 25 `archivar::*` (Mountain), 4 `mathematikerin::*` (River). Die zitierten `1870 passed; 30 failed; 5537 s` liegen **nicht** in diesem Lauf (nur `33 ignored` stimmt); die restlichen Test-Schritte waren `skipped`. `mountain 187` hat die 25 archivar-Fälle begrünt und trägt die mathematikerin-Fälle in `handover-2026-09-27-mountain-folge187.md` `## An River`.
- **Blockade:** keine.
- **Braucht:** nach dem Push `gh workflow run ci-check`; Ergebnis einmal lesen. Der Transfer an River steht (Mountain) — kein Doppel-Träger.

### modis-cdn — Einmal-Migration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36340759898` (`modis-year-split`) beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage status`/`jobs`) `36340759898` steht seit 18:28 auf dem Step `split` (`in_progress`); zuvor scheiterten `compile`/`series-manifest` an `HTTP 422 … file_count limited to 1000 assets per release`. Schema (b) ist gebaut (`dfb405a1b`).
- **Blockade:** Lauf läuft.
- **Braucht:** Ergebnis einmal lesen; erst nach migriertem Familien-Tag `gh workflow run modis-cdn.yml` (sonst 422).

### gosat-cdn — Ghost-Lauf (2026-Shard)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch des `gosat-cdn`-Workflows.
- **Lage:** (gemessen 2026-09-27 via `ci_manage jobs 36322845122`) Lauf `36322845122` API-`in_progress`, `updated_at` 13:34, seither kein Step-Übergang; Zwilling `36322848137` `pending` 0 Jobs; `compile (2026)` hängt; `raster`/`release` success. Live/Ghost aus der API nicht endgültig trennbar.
- **Blockade:** Ghost-Run (runner-seitig).
- **Braucht:** Watchdog-Kandidat nach 2× Median: `ci_manage cancel 36322845122` + `gh workflow run gosat-cdn.yml`; der wartende Zwilling löst sich mit dem Ghost.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** periodischer Re-Dispatch (`gh workflow run hinet-cdn.yml`).
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36323256126`) `...963` failure: `cont status never read Available — the request stays unfetched`, 8× `attempt 0..7 stayed unready`; Auth 200 (6659 B).
- **Blockade:** quellenseitige Readiness.
- **Braucht:** `gh workflow run hinet-cdn.yml` beim periodischen Trigger; bleibt es so, wartend auf Hinet.

### ESA-CCI-SST — Nachfolger ernten + registrieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Compiler steht (`tools/harvest/src/bin/esacci_sst_compiler.rs` gebaut).
- **Lage:** (gemessen 2026-09-27 via `grind-pro` `--verdict`/`--sniff` + CEDA-`?json`) der tote Host `esa-sst-cci-browser.ceda.ac.uk` (`dead_sources.φ:411`) hat einen lebenden Nachfolger: CDR v3.0.1 unter `data.ceda.ac.uk/neodc/eocis/…/CDR_v3/Analysis/L4/v3.0.1/` (HTTP 200; Tagesdatei NetCDF-4, ~16 MB, `analysed_sst` K, `lat`/`lon`; Verzeichnis antwortet `?json`). Die fünf übrigen D5-Routen sind gemessen geschlossen (ZTF/Tomografie/GW bereits registriert; Telescope-Array not-published; Occultation = Mountain-Träger). Ein Compiler `tools/harvest/src/bin/esacci_sst_compiler.rs` ist **nicht** gebaut: der Dispatch in diesem Atom wurde vom opencode-Absturz abgebrochen (Datei ABSENT).
- **Blockade:** Compiler-Bau offen (Session-Absturz = gesprochene Grenze).
- **Braucht:** `esacci_sst_compiler.rs` bauen (Modell `nohrsc_snowfall_compiler`/`swot_l2_lr_ssh_compiler`; `cargo check` + `cargo build -p omegaflow-harvest --bin esacci_sst_compiler`), dann die `sources.φ`-Zeile (url/origin/compiler, `format esacci_sst_l4_cdr3`, thermal τ=86400) + `esacci-sst-cdn.yml` dispatchen.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad; `sgrep zeugnis/roehre` in `tools/` leer).
- **Lage:** (gemessen 2026-09-27 via `explore`) kein Producer-Bin (`sgrep zeugnis/roehre` in `tools/` leer), keine `phi/`-Zeile, kein `*-cdn.yml`; generischer Weg steht (`src/archivar/cdn.rs:41` `upload_release`, `--ci-mode`-Tor). P2P: Nostr-P2P in `ce1e231`/`576bcbb` entfernt; Zukunftsform `future-concepts.md:33-38` §4; Schreibpfad consent-pflichtig.
- **Blockade:** Producer fehlt.
- **Braucht:** Manifestations-Weg erst nach laufendem Producer; P2P downstream, nur mit Consent.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

## An Mountain (gemessen, fremde Feder)

Drei Dispositions-Träger — Verdikt/Reader-Arm Mountain (Register-Schreiber):
- `phi/blocked_sources.φ:357` `vtsuperdarn/hdw.dat` → **`released`** (fold in `:353`): die Radar-Geometrie ist bereits compile-time in allen drei SuperDARN-Compilern verdrahtet (`superdarn_{,fitacf,rawacf}_compiler.rs:11/21/15`, `RST_HDW_RAW`); kein separater Producer nötig; die Compiler ziehen `hdw.dat` aus `SuperDARN/rst/main/…`, nicht aus dem Community-Mirror.
- `phi/blocked_sources.φ:361` DAS2 Iowa → Coverage gemessen (HAPI `/hapi/catalog` = 19 Cassini-Datasets; `Cassini/MAG/Magnitude` 1999-08-16→2017-09-15, nT). Blockade: der Runtime-HAPI-Reader ist JSON-only (`port.rs:521`/`:1558`), der Server CSV-only (`outputFormats:["csv"]`); das native das2.2-Server (551 Datasets) braucht einen eigenen Stream-Parser. Schritt: CSV-Reader-Arm ODER das2.2-Parser.
- `phi/blocked_sources.φ:365` Occultation-DB UTFPR → JSON-API gemessen (`/api/objects` 86855 B; `/api/events` `{rows,total:568}`; Events tragen sexagesimale RA/Dec + `delta_au`; **Stations ohne lat/lon**). Vorschlag: `occultation_compiler.rs` (`format occultation`, `cmap`, `dist_scale 1.495978707e11`, leere `ra_position` skippen) + `sources.φ`-Zeile; Reader-Arm Mountain. Der Manifestations-Teil (`url`/`origin`/`compiler`) ist Myceliums Feder, sobald der Compiler steht.

## Träger (Prosadokumente, eigene)
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang (kein Nachfassen) | nächster Schritt: Trigger Mail.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv); Akt in Future-Queue | nächster Schritt (eigen): Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | nächster Schritt: Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen §7 Roh-Korpora-Disposition; Akt in Future-Queue | nächster Schritt: Disposition nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02 | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker unverändert | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
