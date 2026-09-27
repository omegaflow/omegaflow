<!--
  title: Handover — Mycelium-Folge 186 (2026-09-27)
  session: Mycelium-Folge 186
  class: handover
  date: 2026-09-27
  sha256: bf16eef52d802a5a9cde908ffc31f0d67db63d25396a42e972ba3e6cc8da8f69
  status: live
-->
# Handover — Mycelium-Folge 186 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `eigen` | `wartend` |
`blockiert` | `termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits
in `state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge185.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „kümmer dich drum" — der verwaiste rustfmt-Fix `tools/utils/src/bin/omega_sh.rs` fällt Mycelium zu (Aufenthalt = Eigentum); Commit trägt `/commit`.
- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent (Delegation), **nicht** das Commit-Wort.
- Wort | 2026-09-27 | „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | Operator (Future-Session).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | Operator (Future-Session).
- Wort | 2026-09-27 | Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | Operator (Future-Session).
- Wort | 2026-09-27 | UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | Operator (Session, Mountain).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).
- Wort | 2026-09-27 | RX100 war nur ein Gedanke — Quelle zurückgezogen, Workflow-Zweige entfernen | Operator (Session, Mountain).
- Wort | 2026-09-27 | „falte alle" / „ja bitte falten" — die genuin-offenen Punkte der trägerlosen Docs in die Übergaben ihrer Linien falten (Aufenthalt = Eigentum) | Operator (Mycelium-Session 184).
- Wort | 2026-09-27 | „den rest gebe ich future" — die Tafel trägt nur `eigen`; operator-gebundene Punkte → Future-Operator-Queue (matrix-rotor-Design-Wort), Dritt-Waits → `state/zustand/wartend.φ` (voyager-nssdca, bepicolombo-more); die Queue wird nicht kopiert | Operator (Mycelium-Session 185).
- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent der Folge 186 (Delegation), nicht das Commit-Wort | Operator (Mycelium-Session 186).

## Offen (aufgeschlüsselt)

### modis-cdn — Einmal-Migration + Workflow-Dispatch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-year-split`-Lauf (Migration) erfolgreich.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36320557426`) `compile`/`series-manifest` scheitern `HTTP 422 … file_count limited to 1000 assets per release`; Familien-Release 1000/1000 (8day 887, monthly 113). Schema (b) gebaut: `cdn.rs` `MODIS_LST_CMG_FAMILY`+`modis_lst_cmg_tag_of`+3 Tests, `modis-cdn.yml` Jahr-Tag-Umstellung, `.github/workflows/modis-year-split.yml` (Einmal-Migration download→sha256→upload→delete), `cdn_reconcile`-classify-Arm, `sources.φ:11695/11712` origin ergänzt; `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** nach dem Push die Migration `gh workflow run modis-year-split.yml` dispatchen (in diesem Atom gestartet); dann `gh workflow run modis-cdn.yml` — erst nach migriertem Familien-Tag (sonst 422 auf der Serien-Manifest-Stufe). Lauf-Ergebnis im Stehenden Pass.

### ci-check — 30 Test-Fehler auf main
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** neuer `ci-check`-Lauf beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`/`log 36326102925`) `ci-check 36326102925` @ecce3b62 failure: `test result: FAILED. 1870 passed; 30 failed; 33 ignored` (5537 s); Beispiele `archivar::allwise::tests::bin_rejects_nonfinite_cell_behind_a_set_mask`, `archivar::bsp_reader::spk::tests::type1_single_record_taylor_expansion`, `mathematikerin::tests::volume_probe_parity_masked_corner_and_plain` (gpu 0 cpu 2.5); `36333267310` @88b00862 ebenfalls failure; `36336254499` in Arbeit.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36333267310` die 30 Fehler je Owner auszählen; Träger sind mountain (archivar::allwise/bsp_reader) und river (mathematikerin). Der Punkt reist in deren Übergaben, sobald die Owner-Aufschlüsselung gemessen ist (nicht raten).

### dropped-gate — Baseline 989 vs current 1023
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-`dropped-gate`-Rot.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36324206113`) `baseline 989 | current 1023 | delta 34`; `docs/zustand/dropped-baseline.md:16` = 989 @0a0ce96d; die 34 Namen nennt das Log nicht (unread); lokales `register_lookup --dropped --count` Timeout (>200 s, pending).
- **Blockade:** `register_lookup --dropped` lokal zu langsam.
- **Braucht:** die 34 je Owner aus `register_lookup --dropped` (register-fähiges Profil / längerer Lauf) auftragen; legitime Drops ins annehmende Handover, dann Baseline 989 → 1023 bumpen (nie still).

### gosat-cdn — Ghost-Lauf (2026-Shard)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch des `gosat-cdn`-Workflows.
- **Lage:** (gemessen 2026-09-27 via `ci_manage jobs 36322845122` + `general`-Report) Lauf `36322845122` API-`in_progress`, `updated_at` 13:34 (seither ~3 h 46 min ohne Step-Übergang), Zwilling `36322848137` `pending` 0 Jobs; `compile (2026)` hängt; `raster`/`release` success. Live/Ghost aus der API nicht endgültig trennbar.
- **Blockade:** Ghost-Run (runner-seitig).
- **Braucht:** Watchdog-Kandidat nach 2× Median: `ci_manage cancel 36322845122` + `gh workflow run gosat-cdn.yml`; der wartende Zwilling löst sich mit dem Ghost.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** periodischer Re-Dispatch (`gh workflow run hinet-cdn.yml`) / Hinet-Readiness.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36323256126`) `...963` failure: `cont status never read Available — the request stays unfetched`, 8× `attempt 0..7 stayed unready` (13:42→14:28); Auth 200 (6659 B).
- **Blockade:** quellenseitige Readiness.
- **Braucht:** `gh workflow run hinet-cdn.yml` beim periodischen Trigger; bleibt es so, wartend auf Hinet.

### D5-Orphan-Residuum — Quellen-Registrierung/Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort „falte alle" (2026-09-27) — die Messung ist gelaufen.
- **Lage:** (gemessen 2026-09-27 via `grind-flash` `--verdict`/`--sniff`) Route 1 ESA-CCI SST: Host `esa-sst-cci-browser.ceda.ac.uk` tot (`dead_sources.φ:411`), Nachfolger `climate.esa.int` 200, Datenarm `data.ceda.ac.uk/neodc/esacci/sst/data/` 200 — beide ohne `sources.φ`-Zeile; Route 2 ZTF: bereits registriert (`sources.φ:532/:736/:10008`, `witnesses.φ:70`) → geschlossen; Route 3 Telescope-Array-Vollkatalog: keine URL (extern not-published) → kein Register; Route 4 Occultation-DB = Orphan 3 (siehe Träger); Route 5 3D-Tomografie: bereits registriert (`sources.φ:7521/:11950`) → geschlossen; Route 6 GW/Neutrino/CR: bereits registriert (`sources.φ:11942/:11896`, `witnesses.φ:37/43/55/79/91`) → geschlossen.
- **Blockade:** keine.
- **Braucht:** ESA-CCI-SST-Nachfolger als eigene Quelle ernten/registrieren (`archive_search --verdict` ist gemessen; nächster Schritt Harvest+`sources.φ`-Zeile via `docs/SOURCE_PORT.md`). Übrige Routen sind gemessen geschlossen.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht.
- **Lage:** (gemessen 2026-09-27 via `explore`) Das „Röhren-Asset" ist das Ausgabe-Asset der ZEUGNIS-Identitäts-Röhre (`docs/concepts/zeugnis.md:2/:114-130/:288`); es existiert **kein** Producer-Bin (`sgrep zeugnis/roehre` in `tools/` leer), keine `phi/`-Zeile, kein `*-cdn.yml`; der generische Weg steht (`src/archivar/cdn.rs:41` `upload_release`, `--ci-mode`-Tor, `docs/specs/cdn-ziel-schema.md:22-40`). P2P: Nostr-P2P in `ce1e231`/`576bcbb` entfernt, Zukunftsform `future-concepts.md:33-38` §4 (Global Station Web), kein Code; Schreibpfad consent-pflichtig.
- **Blockade:** Röhren-Asset nicht gebaut (Mountain/River-Baustelle).
- **Braucht:** Manifestations-Weg erst nach laufendem Producer (url/origin/compiler + Workflow); P2P bleibt downstream, Schreibpfad nur mit Consent. Kein Deferral (`zeugnis.md:385-387`).

### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

## Register-Träger (Dispositions-Einträge, Aufenthalt = Eigentum)

- `phi/blocked_sources.φ:357` `https://github.com/vtsuperdarn/hdw.dat` | pending: SuperDARN-Radar-Positionen 200 gemessen 2026-09-27 (raw 200); FITACF/RAWACF stehen (`sources.φ:11487/:9814`) | nächster Schritt: entscheiden ob statische Radar-Metadaten eine eigene `sources.φ`-Zeile (Producer) brauchen oder der `released`-Eintrag `:353` (superdarn.ca/radar-info) den Fall abschließt.
- `phi/blocked_sources.φ:361` `https://planet.physics.uiowa.edu/das/das2Server/hapi` | pending: DAS2 Iowa (HAPI 1.1, CSV) 200 gemessen (2067 B); HAPI-Klasse steht (`sources.φ:584/:720/:7784`); native das2.2 `jupiter.physics.uiowa.edu/das/server` 200 (3879 B), 551 Datasets, Coverage-Frage offen | nächster Schritt: Coverage je Dataset messen, dann `sources.φ`-Zeile mit url/origin/compiler.
- `phi/blocked_sources.φ:365` `http://occultations.ct.utfpr.edu.br/` | pending: Occultation-DB UTFPR (SOSB/Lucky Star) 200; JSON-API `/api/objects` 200 (86855 B), `/api/events`; keine Stations-lat/lon | nächster Schritt: JSON-Compiler + `sources.φ`-Zeile (`format occultation`), Reader-Arm Mountain.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen; offen: native ODF-Serien-Arm + vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte zu, danach kein Eingang (gemessen 2026-09-27 via mail_ledger) | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv); der Akt liegt in Future's Operator-Queue | nächster Schritt (eigen): Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; das Layout-Wort liegt in Future's Operator-Queue | nächster Schritt: Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora-Disposition; der Akt liegt in Future's Operator-Queue | nächster Schritt: Disposition nach Wort.
- `docs/concepts/tools-map.md` | CI-Werkzeug-Detail (`ci_manage status`/`jobs`) ergänzt; übrige offene Marker unverändert | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass; Owner-Klärung offen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
