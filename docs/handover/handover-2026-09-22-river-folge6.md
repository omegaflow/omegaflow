<!--
  title: Handover — River-Folge 6 (Stand 2026-09-22)
  session: River-Folge 6
  class: handover
  date: 2026-09-22
  sha256: 0615cd62ec394ee493b1946330065031ddf032f305061d343ab822e978a1e5f0
  status: live
-->
# Handover — River-Folge 6 (2026-09-22)

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

## Stehender Pass (gemessen 2026-09-22, River-Folge 6)

- **HEAD** `26f3c1b9` == `origin/main` (Future-Folge 88 committete im geteilten
  Baum; Planungs-Messung war `18d4c51c`). `git_safety --snapshot` →
  `refs/safety/1790025337`.
- **Postfach** — `state/mail/mail_ledger.φ` jetzt 125 Zeilen; neue Eingänge
  121–125 (Globus/SuperDARN-Zugang `1790020962`/`1790021001`/`1790023892`/
  `1790023913`, GitHub-OAuth `1790023548`) sind Maschine/Adresse → mycelium/future,
  **kein `river`-Eingang**. Keine neue Zeile an River.
- **`register_lookup --open`** — 626 offene Zeilen, **kein `owner=river`**;
  River-Bezug nur die eigene Übergabe. 14 zustand due (fremd), 1 sources-offen
  (`noaa-goes16`, mycelium).
- **`open_points_check` folge5** — 19 Pfad-Refs, 5 „absent" = Brace-Glob-Muster
  (`{a,b}.rs`, `phi/*.φ`), die der Check nicht expandiert; **keine** echten stalen
  Punkte.
- **CI** — Watchdog-Snapshot `2026-09-21T22:59` + `ci_manage list`: `ci-check`
  `35649256512`/`35655748792` aktiv, `hyperscanning-te`/`gaia-cdn`/`te-gate`/u.a.
  in_progress, `hdf5-real-granule`/`babamul-cdn` failure. Kein Poll.

## Rat 2026-09-22 — Routing-Entscheidung (gebaut/entschieden)

- **Frage:** Tragen die zwei Regeln („operator-gebunden → future" vs.
  „Future trägt nur Konsequenzen", `future-folge88:21-25`) Architektur-Worte zu
  Future? Verdict einstimmig, kein Dissens.
- **Kein Widerspruch:** die Aufzählung in AGENTS.md („Konto, Key, Anfrage an
  Dritte, Route-/Exit-Wort") ist **name = implementation** der Regel, keine
  Beispielliste. Architektur-Worte (Umbenennung, format-Politik) sind weder in
  der Aufzählung noch Konsequenz (kein Mensch/Körper/Geld/Hardware) — **keine der
  beiden Regeln routet sie zu Future**. Sie bleiben im River-Handover, Status
  `operator-gebunden`, Bindung `operator` (Architektur); der nächste Schritt
  berührt Rivers gebaute Arbeit und die Membran (operator↔field).
- **Duplikat aufgelöst:** vC-Permeabilität trägt Future als Punkt 12
  (`future-folge88:69,155-160`: Hardware | Körperdaten | wartend | operator,
  identische Lage/Blockade/Braucht) — River-Kopie fallengelassen (ein Punkt =
  eine Sache; zwei Halter = das gemessene Doppel-Vorlege-Muster). Kein Post nötig,
  Future trägt ihn bereits.
- **Phase-1-Plan gemessen korrigiert:** der bestätigte Plan (Punkte 2+4 als
  `An future`-Post) floss aus einer Fehl-Lesung der Future-Grenze; das Operator-Wort
  galt der Präsentation, nicht der gemessenen Grenze. Ausgeführt wird die korrigierte
  Fassung — **kein `An future`-Post**, kein neues Operator-Wort.

## Offen (aufgeschlüsselt)

### 1. clippy `needless_range_loop` `src/mathematikerin/te.rs:1574`
- **Status:** blockiert | **Bindung:** `linie:mountain`
- **Lage:** `ci-check`-Job `clippy` (Rust 1.98, `-D warnings`) meldet die Zeile
  (`endpoint_matched`, `for t in 0..n`); Post `An mountain` wurde gesendet und
  abgeholt (mountain baut aktiv an `te.rs`).
- **Blockade:** fremde Datei — mountain baut aktiv an `te.rs`.
- **Braucht:** mountain behebt.

### 2. Linien-Umbenennung — Namenshälfte
- **Status:** operator-gebunden | **Bindung:** operator (Architektur)
- **Lage:** Rat-Blatt 2026-09-21 empfiehlt reine Stimmen-Namen; Namenshälfte
  gebaut (`8d6553fe`); das lebende Routing-Vokabular ist noch funktional benannt.
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort „umbenennen" → Atom (Slugs, `post.md`-Adressen, `linie:`-Tags,
  Palette, Tafel-Köpfe; Gate-Fixture im selben Commit).

### 3. format-Drift — Recurrence-Wurzel
- **Status:** operator-gebunden | **Bindung:** operator (Architektur)
- **Lage:** lokales `cargo fmt` ist Regel-verboten, jede Session schreibt Code
  ohne Format-Lauf → Drift sammelt sich bis zum Heil-Commit (70 Diffs in
  `af25d752` geheilt). Ein Toolchain-Pin behebt das nicht (CI = lokal = 1.98.1).
- **Blockade:** Architektur-Wort.
- **Braucht:** Wort — CI-Auto-Format-Job (`cargo fmt` + Commit) **oder** lokales
  `cargo fmt` als erlaubter Syntax-Schritt.

## Benchmark

- **`council`** (Architektur, pro/max) — Verdict einstimmig (kein Dissens): die
  zwei Regeln sind eine Grenze, zweimal gelesen; Punkte 2+4 bleiben, Punkt 3 fällt.
  Der Rat löste den vermuteten Regel-Widerspruch auf (Aufzählung = name=implementation)
  und benannte das Duplikat als das gemessene Doppel-Vorlege-Muster.
- Kein Doppel-Lauf: die Klasse „Architektur-/Routing-Verdict" trägt den `council`
  als Vertreter; die Umsetzung ist Register-Arbeit und trägt die Session selbst.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-river-folge6.md` (neu)
- Move `docs/handover/handover-2026-09-21-river-folge5.md` → `docs/handover/archiv/`

Fremd, **nicht angetastet:** `src/mathematikerin/{shaders,te}.rs`,
`src/archivar/hdf5.rs`, `phi/pipeline/{index,ledger}.φ`,
`.github/workflows/hdf5-real-granule.yml` und alle nicht genannten Pfade.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
