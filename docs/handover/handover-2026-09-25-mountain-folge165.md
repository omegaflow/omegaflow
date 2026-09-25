<!--
  title: Handover — Mountain-Folge 165 (2026-09-25)
  session: Mountain-Folge 165
  class: handover
  date: 2026-09-25
  sha256: e7414a1f42f7b7abdbdb73e4c9e0cc62c79a8bb4bb73481b5eb790f86bd7edea
  status: live
-->
# Handover — Mountain-Folge 165 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Stehender Pass (gemessen zu Session-Beginn)

- **Postfach:** (gemessen 2026-09-25 via `mail_digest` + glob `state/mail/*`)
  `state/mail/` leer, `state/mail/mail_ledger.φ` absent → Stand `pending`, kein
  Silent-Zero. Eintrag: `docs/zustand/external-state.md`.
- **CI-Status:** Watchdog-Snapshot gelesen; aus dem Atom dispatcht
  `galileo-ionocal-cdn.yml` + `ci-check.yml` (nach dem Push). Ergebnis beim
  nächsten Pass aus dem Snapshot, nie gepollt.
- **Sicherheitsnetz:** `git_safety --snapshot` → `refs/safety/1790363215`
  (recover: `git_safety --restore refs/safety/1790363215`).

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×166`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c`) der Arm steht
  (`unit_from_name_suffix`, `units.rs:169`; verdrahtet in `field_or_review`,
  `port.rs`); 166 Einträge tragen das Token.
- **Blockade:** keine
- **Braucht:** Re-Port der 166 Einträge über den stehenden Arm (grind); das Token
  fällt pro Eintrag. Trägerform `phi/blocked_sources.φ::gap:unit-auto-detect ×166`.

#### Tote pub-Fns verdikten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c <fn> .`) zwei tote pub-Fns:
  `find_timestamp` (`src/archivar/port.rs:1498`), `number_text`
  (`src/archivar/mpcorb.rs:33`).
- **Blockade:** keine
- **Braucht:** entfernen **oder** mit Verdikt begründen.

#### Klasse-5 offene Routen bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `archive_search`, siehe
  `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`) vier
  Stellen offen: S3-Scheme (200 mit Token), ODF TRK-2-34/TNF (offener Korpus,
  Parser-Arm fehlt), AMS-02 TDAT (HEASARC live, Reader fehlt), Parquet/GRIB-2/
  OPeNDAP (Reader fehlen).
- **Blockade:** keine
- **Braucht:** TRK-2-34/TNF-Parser (`odf.rs`), TDAT-Reader, Parquet-/GRIB-2-/
  DAP2-Reader; AMS-02 als `live` registrieren.

#### gll.rss PDS4-Bundle — ATDF/ODR/TRK-2-34/RSR ernten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `archive_search --sniff`) das PDS4-Bundle
  `pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw/` ist HTTP 200 und trägt
  ATDF/TRK-2-25 (240), ODR (676), TRK-2-34 (2), RSR (7); die Haus-Arme stehen
  (`atdf.rs`, `galileo_odr.rs`, `cassini_rsr.rs`, `odf.rs`-TNF). Der
  TRK-2-23-Arm wurde in diesem Atom gebaut (`ionocal.rs`, registriert als
  `galileo_ionocal`).
- **Blockade:** keine
- **Braucht:** ATDF/ODR/TRK-2-34/RSR über die stehenden Arme registrieren +
  ernten (Compiler/`*-cdn`-Workflow je Sammlung).

#### arXiv OAI-PMH-Bulk-Weg bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via curl, dokumentiert in
  `docs/concepts/arxiv-api.md`) die Query-API ist auf `start+max_results ≤ 2`
  gekappt (darüber HTTP 406, UA-unabhängig, reproduzierbar); `archive_search
  --arxiv` ist auf dieses Fenster geklemmt. **OAI-PMH**
  (`https://export.arxiv.org/oai2`, `ListRecords&metadataPrefix=arXiv`) → HTTP
  200, keine Kappung; `phi/pipeline/catalog/oai_arxiv.φ` (1300 Einträge) liegt
  bereits, ist aber in `index.φ` als `descoped 0` geführt (kein Feld/URL).
- **Blockade:** keine
- **Braucht:** den OAI-PMH-Leser/-Harvest für arXiv bauen (Reader in
  `tools/utils`/`archive_search` oder harvest-Compiler) **oder** die Route mit
  Messung `descoped` stellen.

### Operator handelt

#### Membran-Debug — Chrome DevTools MCP anbinden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am laufenden Chrome)
- **Lage:** (gemessen 2026-09-25 via `docs/concepts/tools-map.md`) Pfad (i) ist
  „noch nicht angebunden"; Telemetrie-Flags `--no-usage-statistics`
  `--no-performance-crux` sind Bedingung.
- **Blockade:** das Operator-Wort.
- **Braucht:** Operator-Wort; dann MCP mit den beiden Flags anbinden und in
  `docs/concepts/tools-map.md` registrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
