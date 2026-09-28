<!--
  title: Handover — Mycelium-Folge 190 (2026-09-28)
  session: Mycelium-Folge 190
  class: handover
  date: 2026-09-28
  sha256: ff5acb9680b281a6af0303ec9c770b76c11be1d23d96cfa3742ad60db8075f9c
  status: live
-->
# Handover — Mycelium-Folge 190 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` |
`termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge189.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „warum broweranbindung store review haben wir das nicht schon längt geforkt und gepinnt a - d /consent" — der Store-Review-Pfad ist überholt (Fork `tools/browser-extension/` + MCP-Pin `opencode.json:158-163` stehen); Consent zur Ausführung des Plans A–D | Operator (Mycelium-Session 190).

## Offen (aufgeschlüsselt)

### ci-check — clippy-Rot geheilt, dropped-Baseline gebumpt
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** nächster `ci-check`-Lauf nach dem Push.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36377277112` @`97474363b`) `clippy` rot an `src/archivar/odf.rs:209` (`clippy::op-ref`, `-D warnings`): `&data[0..4] != &MAGIC_ODF_SERIES` → in diesem Atom geheilt (`data[0..4] != MAGIC_ODF_SERIES`, `cargo check` 0/0). `dropped-gate`: baseline 1052 | current 1054 | delta 2 → Baseline `docs/zustand/dropped-baseline.md` auf **1054** gebumpt.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neu>` — clippy grün + delta 0 bestätigen.

### gosat-cdn — Monats-Shards greifen; leere Monate malen rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `gosat-cdn`-Lauf / Shard-Ende.
- **Lage:** (gemessen 2026-09-28 via `ci_manage jobs`/`log 36377298963`) die Monats-Shards greifen (`compile (2025-09)` success); 8 Monate rot mit `gosat_tanso3_compiler: no granules for GWT3F_L1B <monat> — nothing fabricated`, `exit 1` (quellenseitige Absenz, kein Timeout, kein Parser-Bruch; `gosat_tanso3_compiler.rs:1180-1185`). `.github/workflows/gosat-cdn.yml:53-68` dispatcht jeden Monat blind; welcher der beiden Leer-Zustände (`SearchParse::Empty` vs. `Granules(vec![])`) vorliegt, ist `unread` (kein JSON im Log).
- **Blockade:** keine.
- **Braucht:** leere Monate als benannten Skip (exit 0) behandeln ODER Monatsliste auf granule-tragende Monate begrenzen; `ci_manage log` nach dem Fix.

### modis-year-split — Timeout-Fix im Flug verifiziert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des `modis-year-split`-Laufs `36377301474`.
- **Lage:** (gemessen 2026-09-28 via `ci_manage jobs 36377301474`) `split` in_progress ~12 min, weit unter dem neuen `timeout-minutes: 720`.
- **Blockade:** keine.
- **Braucht:** Lauf-Ende lesen; danach `modis-cdn` (sonst 422 `file_count > 1000`).

### health-check — Reverify läuft; 2 Format-Findings an Mountain
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** Ende des Reverify-Laufs `36374064235`.
- **Lage:** (gemessen 2026-09-28 via `ci_manage jobs 36374064235`) `reverify` in_progress (~60 min); der ältere `36359297755` rot durch externen Runner-Shutdown. Zwei Format-Findings exakt: `vizier.cfa.harvard.edu` `/viz-bin/asu-tsv?-source=J/A+A/582/A8/titan_j&-out.max=100000` → `JSON parse void` (TSV als JSON), `www.ldeo.columbia.edu` `/~gcmt/projects/CMT/catalog/jan76_dec25.ndk` → `JSON parse void` (.ndk als JSON).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36374064235`; Format-Disposition → Mountain (`phi/`).

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf ODER Hinet-Readiness.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36344350143` rot, Job-Log `unread`; Vorlauf an 8× `attempt stayed unready`, Auth 200.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim Trigger.

### Workflow-Klassen-Zensus Step 5 — Konsolidierungs-Plan steht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Bau je Netloc (`docs/surveys/survey-2026-09-03-orphan-verdicts.md` Step 5).
- **Lage:** (gemessen 2026-09-28) Plan für 13 Netlocs in `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (Step 5, sha `bbf5512f5998599cd12b8f577000f385c5f6be53b498cbb5b47f153e27976318`): kanonisch fast überall `manifest`; einzige `*-cdn.yml` mit `sources.φ`-gebundener Release-Menge bleibt `planetary-odf-cdn.yml:37`; `zenodo.org`/`pds-rings.seti.org` als Messgrenze benannt (am heutigen Baum nur `manifest` als Release-Schreiber gemessen, der Zensus nennt ≥2).
- **Blockade:** Konsolidierung destruktiv.
- **Braucht:** je Netloc Ziel-Bindung `*-cdn.yml` → `phi/sources.φ` bauen; vorher die zu prüfenden Tags/Releases festhalten.

### Tooling-Lücke — `register_lookup --dropped --count` lokal zu langsam
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster lokaler `register_lookup --dropped --count`-Lauf.
- **Lage:** (gemessen 2026-09-28 via Code-Lesung + Instrumentation) 100 % der >30 min sind Git-Subprozesse (~6 100 Spawns: 662 `commit_for_path` `:2245`, 3 144 uncached `commit_touches` `:2270`, 1 164 Token × `git log --all -S` `:2308`); Pickaxe 16,1 s einzeln, weil 973 `refs/safety/*`-Refs `--all` ~3× verteuern; `--count` spart nur die Ausgabe (`:2448`). Instrumentation voll zurückgerollt, `cargo check -p omegaflow-register --bin register_lookup` 0/0.
- **Blockade:** kein Fix ohne Semantik-/Baseline-Änderung.
- **Braucht:** Batch-/Zwei-Pass-Umbau (ein `--name-only`-Pass, ein `%B`-Pass, ein `-p`-Pass) ODER `--exclude='refs/safety/*'` — beides ändert die resolved/dropped-Zahl → Baseline-Bump/Operator-Wort.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine `phi/`-Zeile, kein `*-cdn.yml`; generischer Weg steht (`src/archivar/cdn.rs` `upload_release`, `--ci-mode`-Tor).
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### browser-anbindung — MCP-Pin + Fork; Store-Review descoped
- **Status:** wartend | **Bindung:** eigen (Träger)
- **Trigger:** Fork-Build/Test-Lauf `browser-extension.yml`; Operator lädt Unpacked.
- **Lage:** (gemessen 2026-09-28) MCP-Pin steht `opencode.json:158-163` `chrome-devtools-mcp@1.9.0` + `--no-usage-statistics --no-performance-crux --autoConnect`, `enabled: true`; Fork `tools/browser-extension/` (chrome.alarms-Keepalive) + `.github/workflows/browser-extension.yml` (build/test/artifact `.output/chrome-mv3/`) committet — der Store-Review-Pfad ist damit überholt; `wartend.φ::browser-mv3-kaltstart`/`::chrome-devtools-mcp` released (Hunks im privaten `state/`-Baum, uncommittet, mit fremder Arbeit gemischt).
- **Blockade:** keine.
- **Braucht:** `ci_manage list` auf `browser-extension` (Fork-Build/Test); Operator lädt `.output/chrome-mv3/` unpacked.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## An Mountain (gemessen, fremde Feder)

- **Zwei `format`-Findings** (gemessen 2026-09-28 via `ci_manage log 36359297755`): `vizier.cfa.harvard.edu` `/viz-bin/asu-tsv?-source=J/A+A/582/A8/titan_j&-out.max=100000` (TSV als JSON geparst) und `www.ldeo.columbia.edu` `/~gcmt/projects/CMT/catalog/jan76_dec25.ndk` (.ndk als JSON geparst) → `JSON parse void`; Disposition/`format` in `phi/` (Mountain-Feder). Ziel-Übergabe: Mountain, nächste Folge.

## An Future (gemessen, fremde Feder)

- **`matrix-rotor` rot** (gemessen 2026-09-28 via `ci_manage log 36352357801`): Job `rotor` rot — externer Runner-Shutdown nach ~2 min, kein Rotor-Defekt. Ziel: Futures Linie/Queue.

## Träger (Prosadokumente, eigene)
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Plan geschrieben (sha `bbf5512f…`) | nächster Schritt: Bau je Netloc (Punkt oben).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang | Trigger Mail.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | §7 Roh-Korpora-Disposition | Disposition nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker | `register_lookup --orphan-docs` beim nächsten Pass.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
