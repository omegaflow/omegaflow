<!--
  title: Handover — Mycelium-Folge 189 (2026-09-27)
  session: Mycelium-Folge 189
  class: handover
  date: 2026-09-27
  sha256: 4452a058d18d322719608b48eda1d5e922cdf5cb84ffd3c4cf915c426cb78c19
  status: live
-->
# Handover — Mycelium-Folge 189 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` |
`termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge188.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen." — session-weiter Consent (Delegation), **nicht** das Commit-Wort; „Commit und Push trägt `/commit`" | Operator (Mycelium-Session 189).
- Wort | 2026-09-27 | „ist das alles was du bis zur Kante abarbeiten kannst?" — der stale Pass ersetzt keine Live-Messung; gefeuerte Trigger werden im nennenden Atom gearbeitet | Operator (Mycelium-Session 189).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | Entscheidungen nie als Liste vorlegen — jede Entscheidung braucht eine aussagekräftige Erklärung | Operator (Future-Session).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).
- Wort | 2026-09-27 | „falte alle" — die genuin-offenen Punkte der trägerlosen Docs in die Übergaben ihrer Linien falten (Aufenthalt = Eigentum) | Operator (Mycelium-Session 184).
- Wort | 2026-09-27 | „den rest gebe ich future" — die Tafel trägt nur `eigen`; operator-gebundene Punkte → Future-Operator-Queue, Dritt-Waits → `state/zustand/wartend.φ`; die Queue wird nicht kopiert | Operator (Mycelium-Session 185).

## Offen (aufgeschlüsselt)

### dropped-gate — Baseline auf 1052 gebumpt; lokale `--count`-Lücke
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf am HEAD beendet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36347555576` @`a457ed8c9`) `dropped-gate: baseline 989 | current 1052 | delta 63`; die Baseline `docs/zustand/dropped-baseline.md:16` auf **1052** gebumpt (Messquelle im Note). Früher `36341839537` @`5780d939`: current 1039 | delta 50. Der lokale `register_lookup --dropped --count` brach bei >30 min ohne Ausgabe ab (Sweep: 700 pairs, 3105 dropped, 2112 commit-resolved, 993 `git:none`; Owner: sensory 1212 · mycelium 926 · mountain 618 · entscheid 167 · river 153).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <nächster-ci-check-id>` — bleibt delta > 0 gegen 1052, die neuen Netto-Drops je Owner auftragen.

### ci-check — 2 Test-Fehler am aktuellen HEAD
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** nächster `ci-check`-Lauf nach diesem Commit beendet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36347555576` @`a457ed8c9`) `test`: `1890 passed; 2 failed` — (1) `archivar::tests::test_cache_fresh_cdn_stamp_equality_and_release_branch` (`src/archivar/tests.rs:6673`): wird von der laufenden **Mountain-Session** gefixt (uncommittet, `set_cache_mtime` + stale-mtime); (2) `mathematikerin::tests::volume_probe_parity_masked_corner_and_plain` (`src/mathematikerin/tests.rs:1563`): **in diesem Atom gefixt** — Root-Cause `src/mathematikerin/omega.rs:708` `ensure_capacity` kehrte bei leerem Feld früh zurück (`field_cap 0 >= n 0`), die Probe-Bind-Group entstand nie, der Dispatch wurde übersprungen → GPU 0 vs. CPU 2.5; Fix `field_cap > 0 && field_cap >= n` (erster Aufruf alloziert die Floor-Kapazität 256), `cargo check` 0/0. `dropped-gate` rot nur durch die Baseline (989 < 1052, mit diesem Commit gebumpt); `build`/`format`/`clippy` grün.
- **Blockade:** keine.
- **Braucht:** (1) bleibt bei Mountain; (2) ist mit diesem Atom gebaut (kein Carry); nach dem Push `ci_manage log <neu>` einmal.

### health-check — kein Codefehler; externer Runner-Shutdown
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** nächster `health-check`-Lauf.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log` + Code-Lesung `src/archivar/port.rs`/`main_flow.rs`) der `verify (9)`-failure ist der **externe Runner-Shutdown**, kein Codefehler — `ci_mode` führt einen Quellen-Timeout bereits als benannten Zustand (`API Unreachable`/`Malformed Data`, `exit 0`); keine Änderung nötig. Im Output zwei Format-Findings für Mountain: `vizier…asu-tsv…titan_j` (TSV als JSON geparst) und `ldeo…jan76_dec25.ndk` (.ndk als JSON).
- **Blockade:** keine.
- **Braucht:** Re-Run des Shards; die zwei Format-Findings an Mountain (Register-Disposition `format`).

### modis-year-split — Timeout 350 min; auf 720 + Hoist gefixt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `modis-year-split`-Dispatch.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36340759898`) der `split`-Loop über ~1000 Familien-Assets traf exakt `timeout-minutes: 350` (18:28→00:18); Fix gebaut: `.github/workflows/modis-year-split.yml` Timeout → **720** und die Jahres-Tag-Erzeugung aus dem inneren Loop vorgezogen (≈ **970 gesparte `gh`-Calls** pro Vollmigration).
- **Blockade:** keine.
- **Braucht:** `gh workflow run modis-year-split.yml` nach dem Push; erst danach `modis-cdn` (sonst 422 `file_count > 1000`).

### gosat-cdn — Jahres-Compile >720 min; auf Monats-Shards gefixt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `gosat-cdn`-Dispatch.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36350124170`) der `compile (2026)`-Lauf hing ~6 h im sequentiellen Download/Package der ~3400 Jahres-Granule (≈10 h geschätzt > `timeout-minutes`); Fix gebaut: `.github/workflows/gosat-cdn.yml` rechnet jetzt **monatsweise** (`gosat_tanso3_<year>-<month>.bin`, ~287 Granule/Shard) statt jahresweise; `phi/sources.φ:10794` `origin` nachgezogen.
- **Blockade:** keine.
- **Braucht:** `gh workflow run gosat-cdn.yml` nach dem Push; danach `ci_manage status`/`log` einmal.

### hinet-cdn — CONT-Readiness (Ersatzlauf rot)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf ODER Hinet-Readiness.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) Re-Dispatch `36344350143` **failure** (20:13); Job `hinet` scheitert, der Job-Log trägt keinen Grund (nur Checkout-Boilerplate, `unread`); Vorlauf `36323256126` scheiterte an 8× `attempt stayed unready`, Auth 200 (6659 B).
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim nächsten Trigger; bleibt der Grund `unread`, ist die Readiness-Warte der Befund, kein Maschinenfehler.

### browser-anbindung — MCP-Pin + MV3-Kaltstart (Träger)
- **Status:** wartend | **Bindung:** eigen (Träger)
- **Trigger:** Operator-Wort (MCP-Pin) / Store-Review (MV3).
- **Lage:** (gemessen 2026-09-27 via `register_lookup --orphan-docs`, gefaltet) Chrome DevTools MCP auf eine Version pinnen (Operator-Wort, Eintrag `state/zustand/wartend.φ::chrome-devtools-mcp`); MV3-Kaltstart über eine Store-Extension (Dritter, `state/zustand/wartend.φ::browser-mv3-kaltstart`); Pfad-1 0.16.1→0.17.0 geschlossen.
- **Blockade:** keine.
- **Braucht:** Operator-Wort für den MCP-Pin; Store-Review-Trigger für MV3.

### Workflow-Klassen-Zensus Step 5 (Orphan-Verdicts-Survey)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Konsolidierungs-Plan je Netloc; Artefakt `docs/surveys/survey-2026-09-03-orphan-verdicts.md`.
- **Lage:** (gemessen 2026-09-27 via `general`-Messung, Belege Datei:Zeile) 13 Netlocs aus ≥2 Klassen; kanonisch ist fast überall `manifest`. **Kernbefund: kein einziges `*-cdn.yml` liest seine Release-Menge aus `phi/sources.φ`** (eigener Tag-Satz; einzige Ausnahme `planetary-odf-cdn.yml:37`). Die ≥2-Klassen entstehen durch `probe`/`register`-Workflows, die ebenfalls Releases schreiben (`galileo-trk-noise.yml`, `harvest.yml`/`harvest-long.yml` via `phi/harvest.φ`, `tap_compiler`-Default `tapvizier.cds.unistra.fr`). Das Zensus-Artefakt `/tmp/opencode/workflow-klassen-zensus.md` existiert nicht mehr.
- **Blockade:** Konsolidierung ist destruktiv (Release-Vereinheitlichung) — braucht Plan je Netloc vor jedem Release-Verschwinden.
- **Braucht:** je Netloc kanonische Klasse + betroffene Releases festschreiben; `sources.φ`-gebundene Release-Menge als Ziel prüfen.

### Tooling-Lücke — `register_lookup --dropped --count` lokal zu langsam
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächste lokale Messung: `register_lookup --dropped --count`.
- **Lage:** (gemessen 2026-09-27) der lokale Aufruf läuft >30 min ohne Ausgabe (im CI ~10 min; ein `grind-flash`-Versuch brach bei 600 s und 1800 s ab); die Gate-Zahl ist damit CI-only.
- **Blockade:** keine.
- **Braucht:** Ursache messen (Debug-Binary? `git log -S` je Paar?) und den Wrapper/Bin beschleunigen — sonst bleibt jeder Baseline-Bump von einem CI-Lauf abhängig.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine `phi/`-Zeile, kein `*-cdn.yml`; generischer Weg steht (`src/archivar/cdn.rs` `upload_release`, `--ci-mode`-Tor). P2P in `ce1e231`/`576bcbb` entfernt.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

## An Mountain (gemessen, fremde Feder)

- **`archivar::tests::test_cache_fresh_cdn_stamp_equality_and_release_branch` rot** (gemessen 2026-09-28 via `ci_manage log 36347555576` @`a457ed8c9`): `src/archivar/tests.rs:6673` panicked. **In Bearbeitung durch die laufende Mountain-Session** (uncommittet: `set_cache_mtime` + stale-mtime) — kein Carry.
- **Zwei `format`-Findings aus dem `verify`-Output** (gemessen 2026-09-28 via `ci_manage log 36359297755`): `vizier.cfa.harvard.edu/viz-bin/asu-tsv?...titan_j` (TSV-Antwort, als JSON geparst) und `ldeo.columbia.edu/.../jan76_dec25.ndk` (.ndk, als JSON geparst) → `Malformed Data` im Verify; Disposition/`format`-Korrektur in `phi/` (Mountain-Feder).

## An Future (gemessen, fremde Feder)

- **`matrix-rotor` rot** (gemessen 2026-09-28 via `ci_manage log 36352357801` @`…`): Job `rotor` failure — der Rotor-Slice `./target/release/omegaflow "#station=41001"` (timeout 18000 s) wurde nach ~2 min durch einen **externen Runner-Shutdown** (`The runner has received a shutdown signal`) abgebrochen, kein Rotor-Defekt. Derselbe Shutdown traf die `health-check`-Läufe.

## Träger (Prosadokumente, eigene)
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang | nächster Schritt: Trigger Mail.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Zensus gemessen (13 Netlocs) | nächster Schritt: Konsolidierungs-Plan (Punkt oben).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | nächster Schritt: Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen §7 Roh-Korpora-Disposition | nächster Schritt: Disposition nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass.

## Orphan-Docs-Faltung (Operator-Wort „falte alle", ausgeführt)
- `register_lookup --orphan-docs` steht nach der Faltung auf **0**. Die Blöcke liegen in den Übergaben der Besitzerlinien bzw. in `state/zustand/wartend.φ`:
  - river-folge49: `survey-fortschritt`, `blatt-papier-beweis` (Membran-Bindung), `kybernetische-astrophysik`, `pfeiler-der-architektur`, `recherche-galileo-kadenz-reconciliation` (Träger), `auftrag-flyby2-kette` (Träger).
  - sensory-folge193: `survey-2026-09-17-sonden-request-only` (PDF→Bild-Konversion für den Vision-Leser).
  - mountain-folge190: `positive-maske` (Slab2/3D-Tomografie-Ernte), `auftrag-flyby2-kette` (`where source DSCOVR`), `survey-2026-09-17-sonden-request-only` (nativer ODF-Serien-Arm).
  - mycelium: `arxiv-api`, `exzellenz-konzept` (Träger).
  - `wartend.φ`: `chrome-devtools-mcp`, `browser-mv3-kaltstart`, `blatt-zuschnitt`.
- **Hinweis (One-writer):** die Falt-Hunks in den fremden Übergaben sind uncommittet und werden von der jeweiligen Besitzerlinie committet (nicht von dieser Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
