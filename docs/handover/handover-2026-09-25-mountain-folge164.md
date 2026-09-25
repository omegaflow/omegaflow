<!--
  title: Handover — Mountain-Folge 164 (2026-09-25)
  session: Mountain-Folge 164
  class: handover
  date: 2026-09-25
  sha256: b5864c850b141f136f9830b04d876656416e3e0bd996332079e4391bd55b3da2
  status: live
-->
# Handover — Mountain-Folge 164 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Stehender Pass (gemessen zu Session-Beginn)

- **Postfach:** (gemessen 2026-09-25 via `mail_digest --last 6` + glob `state/mail/*`)
  `state/mail/` ist leer, `state/mail/mail_ledger.φ` absent → der Stand ist
  `pending`, kein Silent-Zero. Der Ledger liegt im privaten Repo; hier nicht
  lesbar. Eintrag: `docs/zustand/external-state.md`.
- **CI-Status:** Watchdog-Snapshot `/tmp/opencode/ci_status.md` (2026-09-25T20:00:31)
  gelesen. Aus diesem Atom dispatcht: `measure-gates 36172029908`,
  `service-build 36172034181` (beide in_progress), `tools-build 36173666018`
  (queued); davor `ci-check 36171288869` @ `f02171e5` pending. Rot am HEAD sind
  fremde Linien (u. a. `ci-check`, `paper-check`, `corpus-te`). Ergebnis beim
  nächsten Pass aus dem Watchdog-Snapshot, nie gepollt.
- **Sicherheitsnetz:** `git_safety --snapshot` → `refs/safety/1790359764`
  (recover: `git_safety --restore refs/safety/1790359764`).

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×166`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap unit-auto-detect" phi/blocked_sources.φ`)
  der Arm steht: `unit_from_name_suffix` (`src/archivar/units.rs:169`) um die
  Suffixe der Feldnamen erweitert und in den Block-Force-Pfad `field_or_review`
  (`src/archivar/port.rs`) verdrahtet (Suffix-Einheit + τ=ttl statt pauschal
  „unit absent"); 2 Stale-Zwillinge (CMA now 54511, JMA quake list) `descoped`
  (URL integriert → `phi/sources.φ:372/:423`). 166 Einträge tragen das Token.
- **Blockade:** keine
- **Braucht:** Re-Port der 166 Einträge über den stehenden Arm (grind); das Token
  fällt pro Eintrag beim Port. Trägerform `phi/blocked_sources.φ::gap:unit-auto-detect ×166`
  (N = live count, Scanner meldet Drift).

#### Tote pub-Fns verdikten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c <fn> .`) genau zwei tote pub-Fns im
  getrackten Baum: `find_timestamp` (`src/archivar/port.rs:1498`) und `number_text`
  (`src/archivar/mpcorb.rs:33`) — keine Call-Site; alle übrigen pub-Fns lebendig.
- **Blockade:** keine
- **Braucht:** entfernen **oder** mit Verdikt begründen (warum pub ohne Konsument
  bleibt) — kein stiller toter Pfad.

#### Klasse-5 offene Routen bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `archive_search` Re-Messung, siehe
  `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`) vier
  Werkzeuglücken sind **offen**: S3-Scheme (jetzt 200 mit Token), ODF
  TRK-2-34/TNF (offener Korpus, Parser-Arm fehlt), AMS-02 TDAT (HEASARC live,
  Reader fehlt), Parquet/GRIB-2/OPeNDAP (offene Endpunkte, Reader fehlen).
- **Blockade:** keine
- **Braucht:** die vier Arme bzw. die Registerpflicht — TRK-2-34/TNF-Parser
  (`odf.rs`), TDAT-Reader, Parquet-/GRIB-2-/DAP2-Reader; AMS-02 als `live`
  registrieren.

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv öffnet den ungecachten Query-Pfad **oder** OAI-PMH-Zweitkanal
  wird als Route freigegeben
- **Lage:** (gemessen 2026-09-25 via Browser: `info.arxiv.org/help/api/{index,basics}.html`
  + `help/bulk_data.html`) 406 mit leerem Body, UA-unabhängig, nur ungecacht;
  die Doku nennt **keine** API-Migration — `export.arxiv.org/api/query` bleibt
  dokumentiert, **OAI-PMH** ist der ausdrücklich bevorzugte Bulk-Weg (täglich,
  dediziert), Rate 4 req/s mit 1 s Schlaf je Burst.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage beim Trigger; kein Code. Der 406 ist Edge-Verhalten,
  keine dokumentierte Deprecation.

### Operator handelt

#### Membran-Debug — Chrome DevTools MCP anbinden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am laufenden Chrome)
- **Lage:** (gemessen 2026-09-25 via `docs/concepts/tools-map.md`) Pfad (i) ist
  „noch nicht angebunden"; Telemetrie-Flags `--no-usage-statistics`
  `--no-performance-crux` sind Bedingung.
- **Blockade:** das Operator-Wort.
- **Braucht:** Operator-Wort; dann Chrome DevTools MCP mit den beiden
  Telemetrie-Flags anbinden und als Pfad in `docs/concepts/tools-map.md`
  registrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
