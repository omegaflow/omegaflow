<!--
  title: Handover — Mountain-Folge 162 (2026-09-25)
  session: Mountain-Folge 162
  class: handover
  date: 2026-09-25
  sha256: 9437e5c959e129d585c4f8bb9ccf0a85073f8c83e18f1d6c3c19bb20aee8fc38
  status: live
-->
# Handover — Mountain-Folge 162 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung: erst Akteur (Linie | Rat | Operator | Dritter), dann
chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt: Trigger / Lage /
Blockade / Braucht.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### Klassen-Träger (Register-Ledger)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap <token>" phi/blocked_sources.φ`)
  `phi/blocked_sources.φ::gap:unit-auto-detect ×168`, `::gap:force-undetermined ×16`,
  `::gap:konverter ×4`, `::gap:curation ×15`; `::gap:votable-reader ×2` und
  `::gap:astrometry-reader ×6` sind aufgelöst (die 2 VLASS-Einträge tragen den arm-genauen
  Token `curation` — der Reader-Arm steht; die 6 VizieR-Astrometrie-Serien sind gemessen
  `descoped`). Die Registerkopf-Legende der `gap`-Token steht (Zeilen 2–8).
- **Blockade:** keine
- **Braucht:** beim nächsten Dispatch je Klasse den Arm bauen oder den Eintrag mit Messung
  `descoped` stellen; Klassen-Träger-Form `phi/blocked_sources.φ::gap:<token> ×N` mit
  N = live count (`sgrep -c`, Count-Drift meldet der Scanner).

### Wartend

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede ungecachte
  Query; gecachte Queries 200. Retry für 406 entfernt.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage bei Trigger; kein Code.
