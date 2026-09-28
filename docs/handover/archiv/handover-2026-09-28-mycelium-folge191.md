<!--
  title: Handover — Mycelium-Folge 191 (2026-09-28)
  session: Mycelium-Folge 191
  class: handover
  date: 2026-09-28
  sha256: 38ff559a9ffead280b84011cedb875a75b968e12c46843af7cc72e9d0df9cf65
  status: live
-->
# Handover — Mycelium-Folge 191 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` |
`termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge190.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „warum broweranbindung store review haben wir das nicht schon längt geforkt und gepinnt a - d /consent" — der Store-Review-Pfad ist überholt (Fork `tools/browser-extension/` + MCP-Pin `opencode.json:158-163` stehen); Consent zur Ausführung des Plans A–D | Operator (Mycelium-Session 190).
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | Operator (Mycelium-Session 191).

## Offen (aufgeschlüsselt)

### ci-check — clippy geheilt; Bestätigungslauf steht
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** Ende des ci-check-Laufs `36385522146` (Push `87b200c02`).
- **Lage:** (gemessen 2026-09-28 via `ci_manage view 36379199936`) der Lauf `36379199936` = `cancelled`, 0 Jobs → clippy/dropped `unread`; `docs/zustand/dropped-baseline.md:16` = Baseline 1054, `:17` misst delta 2 an `36377277112` @`97474363b` (alt). Der clippy-Rot `src/archivar/odf.rs:209` ist im Atom `9f8debcc3` geheilt; Baseline 1052→1054 gebumpt.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36380327273` — clippy grün + delta 0 gegen Baseline 1054.

### gosat-cdn — Leer-Monate als benannter Skip gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des dispatchten `gosat-cdn`-Laufs `36385528038`.
- **Lage:** (gemessen 2026-09-28) `tools/harvest/src/bin/gosat_tanso3_compiler.rs:1180` — 0 Granules endet jetzt mit `exit(0)` + benannter Skip-Meldung statt `exit(1)`; `cargo check -p omegaflow-harvest --bin gosat_tanso3_compiler` 0/0. Die zwei Leer-Zustände (`SearchParse::Empty` vs. `Granules(vec![])`) kollabieren in `search_granules` auf `Some(Vec::new())`, am Aufrufort ununterscheidbar; `recs.is_empty()` (:1273) bleibt `exit(1)` (Download/Parse void — anderer Zustand). `--ci-mode`-Upload ist bei fehlender Ausgabe-Datei unerreichbar (Kontrollfluss).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neu>` — alle Monate grün. Will man die zwei Leer-Zustände unterscheiden, braucht `search_granules` einen reicheren Rückgabetyp (Mountain-Feder; heute nicht nötig).

### modis-year-split — Split läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des `split`-Jobs `36377301474`.
- **Lage:** (gemessen 2026-09-28 via `ci_manage view 36377301474`) `in_progress`, Schritt „Lift every family shard into its year tag, then delete the verified-moved slot".
- **Blockade:** keine.
- **Braucht:** `ci_manage jobs 36377301474`; danach `modis-cdn` (sonst 422 `file_count > 1000`).

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf ODER Hinet-Readiness.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36344350143` rot, Job-Log `unread`; Vorlauf an 8× `attempt stayed unready`, Auth 200.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim Trigger.

### Workflow-Klassen-Zensus Step 5 — Konsolidierungs-Plan steht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Bau je Netloc (`docs/surveys/survey-2026-09-03-orphan-verdicts.md` Step 5).
- **Lage:** (gemessen 2026-09-28) Plan für 13 Netlocs (survey Step 5, sha `bbf5512f5998599cd12b8f577000f385c5f6be53b498cbb5b47f153e27976318`); kanonisch fast überall `manifest`; einzige `*-cdn.yml` mit `sources.φ`-gebundener Release-Menge bleibt `planetary-odf-cdn.yml:37`; `zenodo.org`/`pds-rings.seti.org` als Messgrenze benannt.
- **Blockade:** Konsolidierung destruktiv.
- **Braucht:** je Netloc Ziel-Bindung `*-cdn.yml` → `phi/sources.φ` bauen; vorher zu prüfende Tags/Releases festhalten.

### Tooling-Lücke — `register_lookup --dropped --count` lokal zu langsam
- **Status:** wartend | **Bindung:** eigen (Werkzeug)
- **Trigger:** Operator-Wort zur Baseline-Änderung.
- **Lage:** (gemessen 2026-09-28 via Code-Lesung + Instrumentation) 100 % der >30 min sind Git-Subprozesse (~6 100 Spawns; `git log --all -S`-Pickaxe 16,1 s einzeln wegen 973 `refs/safety/*`-Refs → ~3×); `--count` spart nur die Ausgabe. Instrumentation zurückgerollt, `cargo check -p omegaflow-register --bin register_lookup` 0/0.
- **Blockade:** kein Fix ohne Semantik-/Baseline-Änderung — resolved/dropped-Zahl ändert sich.
- **Braucht:** Operator-Wort für den `--exclude='refs/safety/*'`-Weg (oder den Zwei-Pass-Umbau) + Baseline-Bump; die Frage gehört in Futures Operator-Queue.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine `phi/`-Zeile, kein `*-cdn.yml`; generischer Weg steht (`src/archivar/cdn.rs` `upload_release`, `--ci-mode`-Tor).
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### browser-anbindung — MCP-Pin + Fork; Store-Review descoped
- **Status:** wartend | **Bindung:** eigen (Träger)
- **Trigger:** Ende des dispatchten `browser-extension`-Laufs `36385530990`; Operator lädt `.output/chrome-mv3/` unpacked.
- **Lage:** (gemessen 2026-09-28) MCP-Pin `opencode.json:158-163` (`chrome-devtools-mcp@1.9.0`, `--no-usage-statistics --no-performance-crux --autoConnect`, `enabled: true`); Fork `tools/browser-extension/` (chrome.alarms-Keepalive) + `.github/workflows/browser-extension.yml` committet; `ci_manage list` (100 Fenster) trägt **keinen** Lauf des Workflows. Survey-Marker auf Stand 2026-09-28 gezogen (`docs/surveys/survey-2026-09-20-browser-anbindung.md`, Messnachtrag, sha `5ce1dc1e513c4043961e9831d69b99c8e8f11eb2fbebc78cf5f993f209048555`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view <neu>` für Fork-Build/Test; Operator lädt unpacked (Operator-Akt).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## An Mountain (gemessen, fremde Feder)

- **Trägerloses Doc** (gemessen 2026-09-28 via `register_lookup --orphan-docs`): `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` (3 offene Marker, kein Träger in einer live Übergabe) → Träger in Mountains Übergabe falten; Ziel-Übergabe: Mountain, nächste Folge.
- **Zwei `format`-Findings** (gemessen 2026-09-28 via `ci_manage log 36359297755`): `vizier.cfa.harvard.edu` `/viz-bin/asu-tsv?-source=J/A+A/582/A8/titan_j&-out.max=100000` (TSV als JSON geparst) und `www.ldeo.columbia.edu` `/~gcmt/projects/CMT/catalog/jan76_dec25.ndk` (.ndk als JSON geparst) → `JSON parse void`; Disposition/`format` in `phi/` (Mountain-Feder). Ziel-Übergabe: Mountain, nächste Folge.

## An Future (gemessen, fremde Feder)

- **`matrix-rotor` rot** (gemessen 2026-09-28 via `ci_manage log 36352357801`): Job `rotor` rot — externer Runner-Shutdown nach ~2 min, kein Rotor-Defekt. Ziel: Futures Linie/Queue.

## Träger (Prosadokumente, eigene)
- `docs/concepts/arxiv-api.md` | offene Marker (2) | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/concepts/exzellenz-konzept.md` | offene Marker (3) | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Plan (sha `bbf5512f…`) | Bau je Netloc (Punkt oben).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang | Trigger Mail.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | §7 Roh-Korpora-Disposition | Disposition nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker | `register_lookup --orphan-docs` beim nächsten Pass.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
