<!--
  title: Handover — Bau-Folge 107 (Stand 2026-09-20)
  session: Bau-Folge 107
  class: handover
  date: 2026-09-20
  sha256: ad4ec4ecbb585a3108b27ab908002754bed98a189f40d4fc57bba4d01186ef2c
  status: live
-->
# Handover — Bau-Folge 107 (2026-09-20)

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

- **HEAD** `cb847ba2` (== `origin/main`, „archivar: dist_scale/rv_scale are
  measured-or-absent (Option), never a factor 1") — der Folge-106-Commit.
  Arbeitsbaum: fremd-`M` `docs/handover/post.md`, `docs/zustand/external-state.md`;
  fremd-`R` entscheid-folge62-Archiv-Move; untracked
  `handover-2026-09-20-entscheid-folge63.md` — nicht angefasst. Snapshot
  `refs/safety/1789922800`.
- **Postfach** — `post.md` leer (18 Zeilen), keine bau-Zeile; kein neuer Eingang.
- **CI** — Watchdog-Snapshot `2026-09-20T18:10:36+02:00`: `ci-check`
  `35520538766` in_progress (der Folge-106-Fix-HEAD-Run, `paths: src/**`); die 3
  `failed` (`35517957987`/`35517136467`/`35516232478`) liegen am Branch
  `tools-latest`, nicht am Bau-HEAD. Kein grüner `ci-check` am Folge-106-HEAD.

## Offen

Kein abarbeitbarer undatierter Punkt in dieser Linie: die zwei offenen Punkte der
Folge 106 (`rv_scale` für die 17 `radvel`-Quellen, `dist_scale`-Einheiten) sind in
diesem Atom gemessen und geschlossen — die verbleibenden Punkte sind
`operator-gebunden`/`blockiert`/`wartend` und damit kein Auswahlpunkt.

- **Queue-Korpora Re-Lauf nach Konverter-Fix — `operator-gebunden`** — der
  `--port`-Konverter trägt jetzt ra/dec/plx/z, dist/pmra/pmdec/radvel, `rv_scale`
  und `dist_scale`; die 7 `parser-gap`-Korpora (`phi/pipeline/ledger.φ:94-120`)
  brauchen den `--port`-Re-Lauf. Die 61 `dist` ohne `dist_scale` liegen in diesen
  gitignorierten Korpora; die 17 registrierten `radvel`-Quellen sind **alle km/s**
  (`rv_scale 1000.0` gesetzt) — die Korpora ebenso beim Re-Lauf setzen. (Schritt:
  Operator-Wort für den lokalen Release-Binär-Lauf auf den 7 Korpora, dann
  `--port` + `--probe`, dann die 7 Ledger-Notes fortschreiben.) · `operator-gebunden`
- **`pos` ohne `body`-Direktiv — `blockiert`** — gemessen 3 Blöcke:
  `sources_new_untested_2k.φ:1806`, `sources_potential_pre-cdn_9k_richest.φ:199`,
  `:2344`; kein kanonisches `pos` (`docs/SOURCE_PORT.md:275`). Die Dateien liegen
  nicht im Baum (lokal/ignoriert). (Schritt: den 3 Blöcken ein `body <body>`-Direktiv
  geben bzw. `lat`/`lon` setzen, dann Konverter-Re-Lauf.) · `blockiert`
- **`ci-check`-Verdikt am Folge-107-HEAD — `wartend`** — der Push dieses Atoms
  triggert `ci-check` neu (`paths: src/**`; dieses Atom ändert nur `phi/sources.φ`
  — kein Code-Pfad, der Run kann ausbleiben). (Schritt: den neuen Run einmalig aus
  `/tmp/opencode/ci_status.md` lesen; bei Rot `ci_manage log <id>`.) · `wartend`

## Benchmark

- **Bau-Folge 107**: Atom „`rv_scale` für die 17 `radvel`-Quellen gesetzt (alle
  km/s → `1000.0`); die 26 registrierten `dist_scale` gegen die Katalog-Doku
  verifiziert (alle match, keine Korrektur)". Delegation 2× `grind-flash`
  (Routine-Messung, je ein Kontext). Die Klasse „Routine-Messung/Suche" ist
  geschlossen (gemessen 2026-09-16: flash $0.0008–0.0017 vs. pro/max
  $0.0041–0.0090) — kein Doppellauf, flash zitiert.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/sources.φ` (17 `rv_scale`-Zeilen); neues
  `docs/handover/handover-2026-09-20-bau-folge107.md`; Move
  `handover-2026-09-20-bau-folge106.md` → `archiv/`.
- **Fremd (nicht anfassen, gemessen 2026-09-20 Commit-Vorbereitung):**
  `docs/surveys/survey-2026-09-20-browser-anbindung.md` (`M`), `opencode.json`
  (`M`), `tools/utils/src/bin/archive_search/materialsproject.rs` (`M`). Die im
  Planungs-Pass noch fremden `post.md`/`external-state.md` und die
  entscheid-folge62/63-Dateien hat ihre Linie inzwischen committet. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` selbst; die Session pollt nicht. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
