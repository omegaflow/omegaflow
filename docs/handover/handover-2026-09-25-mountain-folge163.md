<!--
  title: Handover — Mountain-Folge 163 (2026-09-25)
  session: Mountain-Folge 163
  class: handover
  date: 2026-09-25
  sha256: ddde79b61cd3800cf92e3e9149a0d184fabce6676984fc2709a03070d4198892
  status: live
-->
# Handover — Mountain-Folge 163 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×168`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap unit-auto-detect" phi/blocked_sources.φ`)
  168 Einträge tragen die Arm-Direktive; die Legend-Zeilen stehen.
- **Blockade:** keine
- **Braucht:** die Einheiten-Erkennung aus Wert/Header bauen (Arm) **oder** die
  Einträge mit Messung `descoped` stellen; Trägerform
  `phi/blocked_sources.φ::gap:unit-auto-detect ×168` (N = live count, Scanner
  meldet Drift).

#### `phi/blocked_sources.φ::gap:force-undetermined ×16`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap force-undetermined" phi/blocked_sources.φ`)
  16 Einträge; das Feld passt in kein 9-Kraft-Medium (Astrometrie/Farbindex
  ohne Kraft-Fit).
- **Blockade:** keine
- **Braucht:** Kraft-Kanal-Zuordnung (Force-Gate, `grind-pro`) **oder** `descoped`
  mit Messung; Trägerform `…::gap:force-undetermined ×16`.

#### `phi/blocked_sources.φ::gap:curation ×17`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap curation" phi/blocked_sources.φ`)
  17 Einträge; ADQL-/HAPI-Anfrage stale oder ill-formed (FORMAT/Fenster/Identifier).
  Hinweis: der Träger in folge162 nannte `curation ×15` und `konverter ×4` —
  live gemessen ist `curation ×17` und `konverter ×0` (kein Eintrag trägt den
  Token, nur die Legend-Zeile).
- **Blockade:** keine
- **Braucht:** den Query-Kurations-Arm bauen **oder** `descoped` mit Messung;
  Trägerform `…::gap:curation ×17`.

#### gzip-Body ohne `.gz`-Suffix — kein Erkennungs-/Entpack-Pfad
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `read`) `fetch_raw_with` (`src/archivar/fetch.rs:77`)
  wandelt die rohen curl-Bytes via `String::from_utf8_lossy` um, bevor ein Aufrufer
  sie sieht; ein gzip-`Content-Encoding`-Body an einer URL ohne `.gz` wird nicht
  entpackt und fällt als `data-present (non-JSON body …)` durch. Die toten
  `[0x1f,0x8b]`-Byte-Checks (`fetch.rs:590`, `:1061`) sind in diesem Atom entfernt.
- **Blockade:** keine
- **Braucht:** in `fetch_raw_with` vor der lossy-Umwandlung
  `output.stdout.starts_with(&[0x1f, 0x8b])` prüfen und `gunzip(&output.stdout)`
  (`src/archivar/inflate.rs:179`) anwenden; danach den separaten
  `fetch_raw_bytes`-Zweig bei `fetch.rs:1058` prüfen/entfernen.

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede
  ungecachte Query; gecachte Queries 200. Retry für 406 entfernt.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage bei Trigger; kein Code.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
