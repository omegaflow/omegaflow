<!--
  title: Handover — Mountain-Folge 161 (2026-09-25)
  session: Mountain-Folge 161
  class: handover
  date: 2026-09-25
  sha256: dba3c6450feb40556c9b838e9a1bd968130a0950b0a83bb66ec69971376b458b
  status: live
-->
# Handover — Mountain-Folge 161 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet; `blockiert`/`wartend` werden benannt, nie dispatcht. Sortierung: erst
Akteur (Linie | Rat | Operator | Dritter), dann chronologisch nach `Lage`-Datum. Jeder
Punkt aufgeschlüsselt: Trigger / Lage / Blockade / Braucht.

Diese Session konsumierte `docs/handover/archiv/handover-2026-09-25-mountain-folge160.md`.

Gebaut in diesem Atom: der reverify-Sweep klassifiziert die Format-Diagnose getrennt
(`format-void`), die Format-Arme gzip/TSV/VOTable-Status/columnar-envelope(HAPI) stehen,
`void_class` liest den Schema-Kontext, der `key-void`-Text nennt die CI-Umgebung, die 21
`refused`-URLs sind einzeln gemessen und als Zugangs-/Query-Zustände im Register, der
AEC-`gap` steht, die ariel_j-Astrometrie-Serie und die 400-Ursachen sind registriert. Der
Rat: keine Änderung am CI-Fail-Gate (`format-void` bleibt Artefakt; der Register-Home
trägt die Zustände).

## Klassen-Träger (Register-Ledger)

Gemessen 2026-09-25 via `sgrep -c "gap <token>" phi/blocked_sources.φ`:

- `phi/blocked_sources.φ::gap:unit-auto-detect ×168`
- `phi/blocked_sources.φ::gap:force-undetermined ×16`
- `phi/blocked_sources.φ::gap:konverter ×4`
- `phi/blocked_sources.φ::gap:astrometry-reader ×6`
- `phi/blocked_sources.φ::gap:curation ×13`
- `phi/blocked_sources.φ::gap:votable-reader ×2`

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### `votable-reader`/`curation` — Token arm-genau fassen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `curl` Body-Triage) die 2 `parser-def votable` +
  `gap votable-reader` (`ws-uv.canfar.net/youcat` VLASS_Component/VLASSQL_Source)
  antworten HTTP 400 `unknown format: json — service serves VOTable only`; der
  VOTable-Reader liest die Quelle — die Ursache ist die Anfrage (`FORMAT=json`), kein
  fehlender Arm. Die 13 `gap curation` fassen stale/ill-formed ADQL/HAPI-Queries. Der
  Token-Satz (`curation`/`votable-reader`) nennt den Arm noch nicht.
- **Blockade:** keine.
- **Braucht:** die 2 Einträge auf den arm-genauen Token setzen; die Registerkopf-Legende
  der `gap`-Token dokumentieren (`unit-auto-detect`/`force-undetermined`/`konverter`/
  `astrometry-reader`/`curation`; der AGENTS-Kanon nennt `votable-reader`/`html-parser-arm`).

#### BINARY2/BINARY-VOTable-`<STREAM>`-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25) `extract.rs` liest TABLEDATA (VOTable) und asu-tsv;
  BINARY2/BINARY-`<STREAM>`-Rows sind ungelesen. Keine lebende BINARY2-Quelle messbar —
  die 3 VOTable-TAP-Quellen (AKARI, IRSF, DES DR2) liefern TABLEDATA (gemessen
  2026-09-25); der Arm ist defensiv, kein Register-Eintrag ohne gemessene Quelle.
- **Blockade:** keine.
- **Braucht:** den BINARY2-`<STREAM>`-Arm in `src/archivar/extract.rs` bauen (Struktur-
  Vorlage `votable_to_json`); Fixture im selben Atom.

#### ariel_j-/Uranus-Satelliten-Astrometrie — Serien-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25) 6 VizieR `J/A+A/582/A8` asu-tsv-Quellen (ariel_j,
  miran_j, obero_j, titan_j=Titania, umbri_j, uranu_j) tragen Positions-Serien
  `JD/RAJ2000/DEJ2000 (+mas err)` eines bewegten Körpers; kein dist, kein Skalarfeld, RA/Dec
  sexagesimal (A12) → kein `extract` darstellbar; `gap astrometry-reader ×6`.
- **Blockade:** keine.
- **Braucht:** einen Astrometrie-Serien-Arm bauen (bewegter Körper über Zeit; `Surface`/
  `Barycenter`-Bindung) ODER die 6 als Positions-Serie ohne 9-Kraft-Fit mit Messung
  `descoped` stellen.

#### `archive_search` — `--cc` Common-Crawl-Index (keyless)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25, Future-Folge 119 via `--verdict`) der Common Crawl
  Index ist keyless erreichbar —
  `index.commoncrawl.org/CC-MAIN-2024-51-index?url=…&output=json` → HTTP 200, 46794 B;
  `net.rs` verdrahtet 6 Web-Engines, `--cc` fehlt.
- **Blockade:** keine
- **Braucht:** `cc_lines()` in `tools/utils/src/bin/archive_search/net.rs` nach dem
  Muster `wayback_lines` (JSON-Lines: url/timestamp/status/mime), Eintrag in die
  Modus-Listen (`server.rs`, `web.rs`-FALLBACK) + Test; keyless. Träger: Future-Folge 119
  (Suchschnittstellen-Inventur).

#### `free_models.tsv` — Disposition fehlt, blockierte/gestrichene Modelle bleiben in der Auswahl
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25, Future-Folge 119) `tools/measure/free_models.tsv`
  (Auswahl für `free_model_bench.rs:7`, `text_review.rs:7`, `text_probe.rs:8`,
  `free_model_agent_bench.rs:8`) trägt fünf Spalten `provider|model|base|env|channel` —
  **keine** `blocked`/Disposition-Spalte. Kein Tool entfernt eine Zeile; fehlt der Key,
  emittiert es nur `pending_no_key` (`free_model_bench.rs:632`). Gestrichene/blockierte
  Modelle stehen darum weiter drin: `kilo cohere/north-mini-code:free` (Z. 27 — das
  Modell, das am 2026-09-13 `git checkout -- .` lief, AGENTS.md:563),
  `zai glm-5.3-flash`/`glm-5.3-flashx` (Z. 70–71, `1113 kein Guthaben`),
  alle `google gemini-*` (Z. 12–22, kein `GOOGLE_API_KEY`).
- **Blockade:** keine
- **Braucht:** Entscheidung — (a) Spalte `disposition` (`eligible|blocked|struck`) in das
  TSV + Filter in den vier Konsumenten, oder (b) die gestrichenen Zeilen entfernen
  (Fixtures `free_model_bench.rs:687–718`, `text_review.rs:631–653`,
  `free_model_agent_bench.rs` nutzen `zai glm-5.3-flash` mitziehen). Träger: Future-Folge 119
  (LLM-Provider-Routen).

### Wartend

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede
  ungecachte Query; gecachte Queries 200. Retry für 406 entfernt.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage bei Trigger; kein Code.

Stufe 3 blockiert: keiner. termin: keiner. LOCK: keiner.

Hinweis (nicht hier getragen): der `blocked account`-Eintrag purpleair
(paid plan/quota, HTTP 402 mit Key) ist operator-gebunden und gehört in die
private Future-Queue (`state/funding/handover/`), nicht in dieses Register.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
