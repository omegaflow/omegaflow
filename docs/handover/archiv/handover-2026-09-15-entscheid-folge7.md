<!--
  title: Handover — Entscheid-Folge VII (Stand 2026-09-15)
  session: Entscheid-Folge VII
  class: handover
  date: 2026-09-15
  sha256: 769e176e3c4edcd1a9fea7de424daf1dec616e6317de00603badeaaf126a683a
  status: live
-->
# Handover — Entscheid-Folge VII (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind als Nachricht an ihre Linie
überführt (nie in ein fremdes Handover geschrieben).

## Sicherheitsnetz — Aktivierung offen (härtester undatierter Punkt)

- Das Guard-Atom ist gebaut und committet (`9c8a729`, `c87fa78`): `free`/`free-vision`
  aus der Global-Config entfernt; destruktive Git-Denies in **global und Repo**
  (`reset`/`checkout`/`clean`/`rebase`/`stash`/`restore`/`switch`/`push --force`/
  `worktree`); `snapshot: false` global (opencode-Revert kann keine Dateien mehr
  zurückschreiben); `git_safety --snapshot` (`tools/utils/src/bin/git_safety.rs`)
  sichert den ganzen Arbeitsbaum — tracked + untracked — unter `refs/safety/<epoch>`
  (nicht-destruktiv, temporärer Index). (Schritt: **opencode neu starten**, erst
  dann greifen `snapshot: false`, die Denies und die Agenten-Entfernung; danach
  `git_safety --snapshot` bei Start und Abschluss, optional `git_safety --watch <secs>`.)
- Der Anlass, gemessen 2026-09-15: ein DRS-FITS-Sub-Agent hatte 227 Zeilen `fits.rs`
  auf der Platte (14:06), um 15:24 weg — ohne Git-Kommando, der opencode-Revert der
  Bau-Session; und am 2026-09-13 verwarf ein `kilo/cohere`-Sub-Agent die ganze
  uncommittete Kopie mit `git checkout -- .`.

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; der Blocker
  Bande-Split ist geschlossen (Forschung 09-14/15: Registerzeilen stehen).
  (Schritt: senden — Operator.)
- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Antwort offen.
- NOIRLab Data Lab — abgelehnt; offen bleibt nur die Speisekammer-Frage
  (Wiedervorlage 2026-12-02).
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).

## Entscheidungen offen

- NIM-Spezialmodelle-Docking (`docs/handover/archiv/handover-2026-09-13-nim-spezialmodelle.md`):
  ob `nvidia/nemotron-parse-2.0` / `nvidia/riva-translate-4b-instruct-v1.1|-v2` über
  den curl-Pfad andocken; „kein Commit, kein Push — wartet auf das Consent-Wort".
  (Schritt: Rat-Entscheid.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
