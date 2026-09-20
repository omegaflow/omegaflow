<!--
  title: Handover — Bau-Folge 108 (Stand 2026-09-20)
  session: Bau-Folge 108
  class: handover
  date: 2026-09-20
  sha256: 2b2ad0c20d71f237f71f4d4e59c76c8be959ea9570ade5c07d09239e1241f798
  status: live
-->
# Handover — Bau-Folge 108 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `c29234d1` (== `origin/main`, Folge-107-Commit). Arbeitsbaum: fremd `M`
  `docs/auftrag/auftrag-flyby2-kette.md`,
  `docs/paper/flyby-path-2-falsification-metric-addendum.md`,
  `docs/surveys/survey-2026-09-20-browser-anbindung.md`, `opencode.json`,
  `phi/harvest.φ`, `phi/sources.φ`,
  `tools/utils/src/bin/archive_search/materialsproject.rs`; untracked
  `handover-2026-09-20-ernte-folge117.md` — nicht angefasst. Snapshot
  `refs/safety/1789923217`.
- **Postfach** — `post.md` leer (keine bau-Zeile); `external-state.md`-Postfach-Zeile
  zitiert (letzter Eingang `1789918147`, kein neuer Eingang).
- **CI** — `external-state.md`-CI-Zeile zitiert (Forschung-Folge 116): `ci-check`
  `35523145456` in_progress / `35524089715` pending; `failed` auf Branch
  `tools-latest`, nicht am Bau-HEAD. Kein grüner `ci-check` am Folge-107-HEAD.

## Offen

Kein abarbeitbarer undatierter Punkt in dieser Linie.

- **Queue-Korpora Re-Lauf — `operator-gebunden`** — die 7 `parser-gap`-Korpora
  (`phi/pipeline/ledger.φ`) brauchen den lokalen Release-Binär-Lauf (`--port` +
  `--probe`); der Session ist der lokale Funktionslauf verweigert, kein
  CI-`--port`-Workflow (Korpora gitignored). An `entscheid` geroutet (`post.md`).
  (Schritt: Operator-Wort für den Lauf-Ort, dann `--port` + `--probe`, dann die
  Ledger-Notes fortschreiben.) · `wartend`
- **`pos` ohne `body`-Direktiv — `blockiert`** — gemessen 3 Blöcke
  (`sources_new_untested_2k.φ:1806`, `sources_potential_pre-cdn_9k_richest.φ:199`,
  `:2344`); kein kanonisches `pos` (`docs/SOURCE_PORT.md:275`). Die Dateien liegen
  nicht im Baum (lokal/ignoriert). (Schritt: den 3 Blöcken ein `body <body>`-Direktiv
  geben bzw. `lat`/`lon` setzen, dann Konverter-Re-Lauf.) · `blockiert`
- **`ci-check`-Verdikt am Folge-107/108-HEAD — `wartend`** — (Schritt: den Run
  einmalig aus `/tmp/opencode/ci_status.md` lesen; bei Rot `ci_manage log <id>`.) · `wartend`

## Benchmark

- **Bau-Folge 108**: Atom „Register-Korrektur der 6 stale `parser-gap`-Einträge
  in `phi/pipeline/ledger.φ`". Messung (explore, flash): `tap_to_json`
  (`src/archivar/extract.rs:1554`) liest die EAS-Form bereits; die Euclid-Quelle
  steht (`phi/sources.φ:7725`); die übrigen fünf sind in
  `declined_sources.φ`/`dead_sources.φ` disponiert. Kein Reader-Bau nötig — ein Bau
  wäre Fabrikation. Die Klasse „Register-Disposition" hat den flash-Sieger
  (Routine-Messung, 2026-09-16 gemessen); kein Doppellauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (6 Einträge entfernt),
  `docs/handover/post.md` (An-entscheid-Zeile), neues
  `docs/handover/handover-2026-09-20-bau-folge108.md`, Move
  `handover-2026-09-20-bau-folge107.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/auftrag/auftrag-flyby2-kette.md`,
  `docs/paper/flyby-path-2-falsification-metric-addendum.md`,
  `docs/surveys/survey-2026-09-20-browser-anbindung.md`, `opencode.json`,
  `phi/harvest.φ`, `phi/sources.φ`,
  `tools/utils/src/bin/archive_search/materialsproject.rs`,
  `docs/handover/handover-2026-09-20-ernte-folge117.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` selbst; die Session pollt nicht. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
