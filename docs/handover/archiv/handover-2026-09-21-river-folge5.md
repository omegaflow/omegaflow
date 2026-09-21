<!--
  title: Handover — River-Folge 5 (Stand 2026-09-21)
  session: River-Folge 5
  class: handover
  date: 2026-09-21
  sha256: ab6075041b179997217e9db4524c77b8842527010b41b3bae7577be02d32b619
  status: live
-->
# Handover — River-Folge 5 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt: **Lage** / **Blockade** / **Braucht**.
`operator-gebunden`, `blockiert` und `wartend` werden benannt, nie dispatcht.

## Stehender Pass (gemessen 2026-09-21, River-Folge 5)

- **HEAD** Start `af25d752` == `origin/main` == `merge-base`, Arbeitsbaum leer.
  Während der Session arbeiten **fremde Linien uncommittet im geteilten Baum**
  (`src/mathematikerin/{shaders,te}.rs` = Mountain Riss 4 / KSG-Spiegel, plus
  `src/archivar/hdf5.rs`, `phi/pipeline/{index,ledger}.φ`,
  `.github/workflows/hdf5-real-granule.yml`) — nicht angetastet.
- **Postfach** — `state/mail/mail_ledger.φ` gelesen (120 Zeilen), kein
  `river`-Eingang; letzte Eingänge (Brave-Limit `1789978555`, Cloudflare-Notify)
  sind fremd. Keine neue Zeile an River.
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD beim Start, nichts zu sichern.
- **`register_lookup --open`** — kein `owner=river`; 14 zustand due (fremde
  Abhängigkeiten), 1 sources-offen (`noaa-goes16`, mycelium).
- **`open_points_check`** folge4 — 13 Pfad-Refs, 4 „absent" = Brace-Glob-Muster
  (`{a,b}.rs`), die der Check nicht expandiert; **keine** echten stalen Punkte.
- **CI** — Watchdog-Snapshot `2026-09-21T21:55` + `ci_manage list`: `ci-check`
  `35639506151` in_progress, `te-gate` `35628669014` in_progress,
  `hdf5-real-granule`/`babamul-cdn` failure. Kein Poll.

## Rat 2026-09-21 — halten-vor-reichen, Verhaltenshälfte (gebaut)

- **Blatt:** die Owner-Map-Frage **löst sich auf** — der sounde Check braucht
  keine Token→Owner-Map. `disposition_owner` und `state_class` in
  `register_lookup.rs` sind **zwei Karten für zwei Fragen**; eine Kopie in die
  Vocab oder ein neues `phi/*.φ` wäre eine zweite Wahrheit (Gate-Fixture
  `kernel_id_for_force`). Der Sender-Regex + Owner-Token-Check ist **nicht sound**:
  `linie:<x>` ist Bindung-Vokabular (Rat-eigene Form), `post.md` trägt den
  Empfänger, nicht den Sender.
- **Soundes Residuum:** der Act-in-falscher-Heimat — eine Zeile, deren getrimmter
  Kleinbuchstaben-Anfang `an <voice>:` ist (mountain/mycelium/sensory/future/river),
  in einem **lebenden** Handover = Hard `routing-act-home`. Exempt: `post.md`
  (Heimat), `docs/handover/archiv/**` (Historie), alles außerhalb
  `docs/handover/`; `_template.md` nicht exempt (darf die Form nie lehren).
- **Gebaut:** `src/gate/commit_gate.rs:1051` (`check_routing_act_home`,
  Integration `:876`), `src/gate/commit_gate_vocab.json` (`routing_act_home` +
  Feedback), Test `fn_tool_routing_act_home` (6 Fixtures: 5 Act-Formen im lebenden
  Handover = Hard; post.md/archiv/auftrag/`Post An future steht`/_template =
  pass). Das Nature-Urteil (ist der Punkt die eigene Natur?) bleibt Session-Pflicht
  — Empfänger-Fold-in, Planungs-Tafel.

## Offen (aufgeschlüsselt)

### 1. clippy `needless_range_loop` `src/mathematikerin/te.rs:1574`
- **Status:** blockiert | **Bindung:** `linie:mountain`
- **Lage:** `ci-check` `35608204623` Job `clippy` (Rust 1.98, `-D warnings`)
  meldet `te.rs:1574`; die Zeile steht am Baum (`endpoint_matched`,
  `for t in 0..n`). Die Post-Route wurde von future auf mountain korrigiert
  (Post `An mountain`, clippy-Zeile; gleiche Datei wie Riss 4).
- **Blockade:** fremde Datei — mountain baut aktiv an `te.rs`.
- **Braucht:** mountain behebt; Post `An mountain` steht.

### 2. Linien-Umbenennung — Namenshälfte
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** Rat-Blatt 2026-09-21 empfiehlt reine Stimmen-Namen; Namenshälfte
  gebaut (`8d6553fe`); das lebende Routing-Vokabular ist noch funktional benannt.
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort „umbenennen" → Atom (Slugs, `post.md`-Adressen, `linie:`-Tags,
  Palette, Tafel-Köpfe; Gate-Fixture im selben Commit).

### 3. vC-Permeabilität — Vollzug (hidden run)
- **Status:** blockiert | **Bindung:** operator (Maschine)
- **Lage:** `future-folge83:182-186`; versteckter sensor-getriebener Lauf wartet.
- **Blockade:** keine Sensoren angeschlossen (Operator-Wort 2026-09-21).
- **Braucht:** Sensoren angeschlossen (Trigger), dann `OMEGAFLOW_HIDDEN=1`.

### 4. format-Drift — Recurrence-Wurzel
- **Status:** operator-gebunden | **Bindung:** operator (Architektur)
- **Lage:** lokales `cargo fmt` ist Regel-verboten, jede Session schreibt Code
  ohne Format-Lauf → Drift sammelt sich bis zum Heil-Commit (70 Diffs in
  `af25d752` geheilt). Ein Toolchain-Pin behebt das nicht (CI = lokal = 1.98.1).
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort — CI-Auto-Format-Job (`cargo fmt` + Commit) **oder** lokales
  `cargo fmt` als erlaubter Syntax-Schritt.

## Geteilter Baum — eigener Pfad-Satz

- `src/gate/commit_gate.rs` (neuer Check + Integration + Test)
- `src/gate/commit_gate_vocab.json` (`routing_act_home` + Feedback)
- `docs/handover/handover-2026-09-21-river-folge5.md` (neu)
- `docs/handover/archiv/handover-2026-09-21-river-folge4.md` (verschoben)

Fremd, **nicht angetastet:** `src/mathematikerin/{shaders,te}.rs`,
`src/archivar/hdf5.rs`, `phi/pipeline/{index,ledger}.φ`,
`.github/workflows/hdf5-real-granule.yml` und alle nicht genannten Pfade.

## Benchmark

- `council` (Architektur, pro/max) — Verdict; die Arbeitsschicht hatte den
  Konfund (Owner-Map = eine Karte) vermutet, der Rat maß zwei Karten und ein
  sounderes Residuum.
- Bau-Atom `routing-act-home` via `grind-flash` (Design vom Rat vollspezifiziert
  → Routine, flash-first). Kein Doppel-Lauf: die Klasse „mechanischer Gate-Bau aus
  präziser Spezifikation" trägt den registrierten Sieger `grind-flash`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
