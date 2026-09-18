<!--
  title: Handover — Bau-Folge 72 (Stand 2026-09-18)
  session: Bau-Folge 72
  class: handover
  date: 2026-09-18
  sha256: 0e1395129fccb29e46974e403da6bb8358d41020f3dd4dfbb93dc582a1fc75f3
  status: live
-->
# Handover — Bau-Folge 72 (2026-09-18)

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
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatiert); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18, HEAD 43ff25cd)

- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Mandrill-Bounce des Rubin-Forum-Migrations-Threads, **kein
  Agenten-Eingang**); keine `An bau`-Zeile in `docs/handover/post.md`.
- **CI am HEAD** — Watchdog-Snapshot 2026-09-18T04:50 + `ci_manage list`:
  `ci-check` `35296234750` in_progress @`43ff25cd`, `health-check` `35301898466`
  pending; `ci-check` `35296123717` failure @`1a129cd2` (forschung-Commit; Re-Run
  @HEAD läuft). Aktiv/failed im Snapshot sonst fremde Linien.
- **HEAD** `43ff25cd` == `origin/main` (Fast-Forward möglich). Fremd im Baum
  (nicht angefasst): drei gestagte `handover-2026-09-16-*`-Renames (D + `archiv/`??),
  `opencode.json` (M — fremder Hunk `logLevel`/`watcher.ignore`).

## TE-Gate — konditionales Arx-FP/FN-Gate rot (härtester undatiert, abarbeitbar)

- `te-gate` **35129638318** @`1337c6c1` **success** (AR(p)-Arx-Switch verifiziert);
  `te-gate` **35139361939** @`43af521f` **failure** (konditionales FP/FN-n=1000-Gate,
  bau-folge53: „the te-gate CI measurement is open"); danach kein Fix an
  `src/mathematikerin/te.rs` / `.github/workflows/te-gate.yml`. Der Ledger-Eintrag
  `docs/zustand/external-state.md:23` ist stale (`in_progress`). (Schritt:
  `ci_manage log 35139361939` einmal lesen, die rote Zelle benennen, code-seitig
  beheben, `gh workflow run te-gate.yml`, die Zustand-Zeile fortschreiben.)

## Werkzeug-Gap geschlossen — `ci_manage log` (dieser Atom)

- `ci_manage log <run-id> [--all]` gebaut (`tools/utils/src/bin/ci_manage.rs`):
  liest die Jobs des Laufs und druckt die Logs der roten Jobs (Default) bzw. aller
  (`--all`); leerer Befund → ehrliche „no failed job"-Zeile, kein erfundenes Log.
  Drei `#[cfg(test)]`-Tests (Job-Auswahl). `cargo check -p omegaflow-utils --bins`
  0/0. **Offen:** das Release-Binär ist noch nicht auf PATH — nach dem Push
  `gh workflow run release-build.yml`, dann `ci_manage log <id>` gegen einen roten
  Lauf einmal fahren (Verifikation). · `pending` (Build/Verifikation)
- **`opencode.json`** — die Plan-/Lese-Profile um `ci_manage log`/`--help`,
  `git merge-base`, `du`/`find`/`awk` erweitert (gemessene Verweigerungen:
  `line`/`plan` 7, `research-max` 4, `explore` 1 — Standard-Inspektoren außerhalb
  der Allow-List). Die Datei trägt zugleich einen **fremden uncommitteten Hunk**
  (`logLevel`/`watcher.ignore`) — beim Commit klären, ob er mitgeht oder fremd
  bleibt. · `operator-gebunden` (Commit-Entscheid)
- **AGENTS.md + `_template.md`** auf `ci_manage log` gezogen; der Template-Satz
  „`gh` nur für `--log`/`--log-failed`" (Widerspruch zu AGENTS) ist gestrichen —
  `gh` bleibt nur `workflow run`/`run download`.

## Offen (unverändert, kein Handlungsschritt)

- **Scanned-/bild-only-PDFs → `vision`-OCR — `pending`** (kein Konsument hat es
  angefordert; `archive_search --pdf-image` + `vision` steht). Kein Bau nötig.
- **`--pdf-text` bei Type0/Identity-H ohne ToUnicode — `pending`** (arXiv-Weg
  trägt). Keine Aktion.

## Benchmark

- Kein Doppellauf: die Werkzeug-Analyse (`opencode.db`-Zählung) und der
  `ci_manage log`-Bau sind Routine (flash-Klasse) — kein pro/max-Dispatch.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/utils/src/bin/ci_manage.rs`, `AGENTS.md`,
  `docs/handover/_template.md`, `docs/zustand/external-state.md` (TE-Gate-Zeile),
  `docs/handover/handover-2026-09-18-bau-folge72.md` (+ archiviertes
  `handover-2026-09-18-bau-folge71.md`), `opencode.json` (nur nach Operator-Wort).
- **Fremd (nicht anfassen):** `opencode.json` (fremder `logLevel`/`watcher`-Hunk),
  die drei gestagten `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
