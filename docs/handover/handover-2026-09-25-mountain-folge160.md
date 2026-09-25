<!--
  title: Handover — Mountain-Folge 160 (2026-09-25)
  session: Mountain-Folge 160
  class: handover
  date: 2026-09-25
  sha256: bb25f16513e24e7d9ff1ef5ab7a3a22c97d20965626fc99d35331016489c2a69
  status: live
-->
# Handover — Mountain-Folge 160 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung: erst Akteur (Linie | Rat | Operator | Dritter), dann
chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt: Trigger / Lage /
Blockade / Braucht.

Diese Session konsumierte `docs/handover/archiv/handover-2026-09-25-mountain-folge159.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### reverify `refused`-Klasse → Zugangs-Signal, nicht Drift-Maske
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `ci_manage`, Rat 2026-09-25) `reverify.txt` #113
  trägt 21 `refused` (Host 4xx/5xx); die Legende `src/archivar/port.rs:801` sagt
  „alive, not dead". Der Rat: `refused` ist ein Zugangs-Zustand, Heimat
  `phi/blocked_sources.φ` (ip-blocked/pending), **nicht** die Drift-Maske.
- **Blockade:** keine
- **Braucht:** die 21 refused-URLs einzeln messen (401 *mit* Token? 403? 429?) und je
  als `blocked_sources.φ`-Eintrag tragen — kein CI-Issue pro Run.

#### reverify Drift-Bucket = parser-gap, nicht API-Drift
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `ci_manage log`, Rat 2026-09-25) von 141 rechecks
  sind 78 `drift-void`, überwiegend Formate, die der generische Sweep-`extract` nicht
  liest (`.txt.gz`-rows, TSV, VOTable, HAPI-JSON-Key-Pfade). Der Rat: Überspringen ist
  ein stilles Null (verboten); der Sweep soll die Format-Diagnose separat
  klassifizieren, damit die Drift-Maske wieder echte API-Drift meint.
- **Blockade:** keine
- **Braucht:** Format-Diagnose im Sweep (`src/archivar/fetch.rs`) trennen + die
  Format-Arme (Dekomprimieren/TSV/VOTable/HAPI) bauen.

#### „all containers empty" als Quiet mehrdeutig
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (Rat 2026-09-25) `diagnose_no_samples` (`src/archivar/fetch.rs:533`) prüft
  nur den Body, nicht den Schema-Kontext: „all containers empty" kann leere Region
  oder Key-Drift sein.
- **Blockade:** keine
- **Braucht:** messen/urteilen, ob der Schema-Kontext in `void_class()` einfließt.

#### key-void-Detailtext — Wortlaut-Drift
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25) der `key-void`-Detailtext sagt „absent in
  `.secrets.local`", obwohl die CI Secrets per `env` injiziert (kein
  `.secrets.local`-Pfad).
- **Blockade:** keine
- **Braucht:** den Text an die CI-Realität angleichen (`src/archivar/fetch.rs`).

Keine eigenen operator-gebundenen Punkte: der `tap_index_alma_*`-Riss und
`OPENALEX_MAILTO` liegen in der Future-Queue (privates Repo, Future-Folge 118).

### Wartend

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede
  ungecachte Query; gecachte Queries liefern 200. Retry für 406 entfernt.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage bei Trigger; kein Code.

Stufe 3 blockiert: keiner. termin: keiner. LOCK: keiner.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
