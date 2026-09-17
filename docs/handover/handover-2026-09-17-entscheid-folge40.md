<!--
  title: Handover — Entscheid-Folge 40 (Stand 2026-09-17)
  session: Entscheid-Folge 40
  class: handover
  date: 2026-09-17
  sha256: 2d52dc940f13b097e68a49fcef6865a0d34998cee8a3a65fa0ab0906ea8f72b9
  status: live
-->
# Handover — Entscheid-Folge 40 (2026-09-17)

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
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-17)

- **HEAD** `15463762` (fremde ernte-Commits `4e27f339`/`15463762` landeten
  während der Session); `origin/main` `5a670e09` (cdn-reconcile-Bot-Commit) —
  **divergiert**: beide Zweige sind Kinder von `964d18b9`. Ein Fast-Forward-Push
  ist derzeit nicht möglich; der cdn-Bot-Zweig (`5a670e09`) und die lokalen
  ernte-Commits müssen erst integriert werden. Die ernte-eigenen Änderungen an
  `phi/sources.φ` + `phi/pipeline/ledger.φ` sind committet (nicht angefasst).
- **Postfach** — kein neuer externer Agenten-Eingang: neuester Ledger-Eintrag
  `1789670592` (2026-09-17, Rubin-Forum „News" — kein Agenten-Reply),
  unverändert seit Folge 39. `docs/zustand/external-state.md` trägt den Wert.
- **CI-Status @`964d18b9`** — Watchdog-Snapshot 22:26 + `ci_manage list`:
  `ci-check` `35273949693` pending @`964d18b9`; CDN-/Harvest-Runde @`ab45b3f3`
  abgeschlossen (trmm-lis/lis-otd/kcdc/iss-lis/ionex/igets success, auto-dispatch
  `35273075509` success, cdn-reconcile `35273461901` success); failure:
  pii-exposure `35273689697` (exit 2 = Exposition bleibt, kein CI-Defekt),
  harvest-dispatch `35273075476`, harvest `35270996845`; in_progress: gaia-sso-cdn,
  goes-cdn, harvest, pioneer-cell-census, health-check. Fremde Linien — kein
  Entscheid-Handlungsbedarf.
- **Zustand-Ledger** — PII-Zeile auf die fällige Messung @`ab45b3f3` gesetzt
  (Trigger gefeuert), CI-Zeile auf `964d18b9`, Postfach measured-at Folge 40;
  `docs/zustand/external-state.md` aktualisiert (Commit mit `/commit`).

## Handlungsfähig — Auswahlpunkte

- `wartend` — **GitHub PII-Exposition** @`ab45b3f3`: 45 Kombinationen, PII
  retrievable: yes, GC offen (Ticket #4761801). Wert steht in
  `docs/zustand/external-state.md`. (Auslöser: HEAD-Wechsel oder GitHub-GC-Antwort.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande): §4 ist
  geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor, aber
  **Senden ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein
  Sendeschritt, keine Sende-Anfrage, keine Register-Zeile. (Schritt: keiner; das
  Verbot steht.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — **Push-Divergenz**: `origin/main` `5a670e09` (cdn-reconcile-Bot)
  und HEAD `15463762` (ernte) divergieren, beide Kinder von `964d18b9` —
  Fast-Forward unmöglich. (Schritt: `git pull --no-rebase` — Merge des
  Bot-Commits — vor dem Push; die lokalen ernte-Commits reisen mit.)

## Wartend (extern) — kein Auswahlpunkt, kein Handlungsschritt

- `wartend` — GitHub GC `#4761801`: neuester Eingang `1789662457`
  (Zendesk-Echo des eigenen Kommentars, kein Agenten-Reply). (Auslöser:
  Postfach-Eingang.)
- `wartend` — GitHub Privacy-Löschung: Resend `01a0b032` (gesendet
  `1789662499`), keine Antwort. (Auslöser: Postfach-Eingang.)
- `wartend` — NSE/Haug I(q,t): TRISP/MLZ (Keller) sendet die Rohdaten
  („in einigen Tagen", `1789650050`). (Auslöser: Postfach-Eingang → Datenannahme
  + CDN-Manifestation.)
- `wartend` — fünf Sonden-Anfragen (NSSDC Voyager/Mariner 10/Viking, Cassini,
  Juno): gesendet 2026-09-16. (Auslöser: Postfach-Eingang.)
- `wartend` — Rubin-Review (Shaughnessy): Forum zieht 2026-09-24 auf
  `rubin.community`. (Auslöser: Postfach/Forum bei Eingang.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/zustand/external-state.md` (PII @`ab45b3f3`, CI
  @`964d18b9`, Postfach Folge 40), `docs/handover/handover-2026-09-17-entscheid-
  folge40.md`, `docs/handover/archiv/handover-2026-09-17-entscheid-folge39.md`
  (Move).
- Fremde uncommittete Arbeit (nicht anfassen): die gestagten `archiv/`-Renames
  (`handover-2026-09-16-entscheid-folge24`, `-forschung-folge44`, `-forschung-
  folge51`). Nie ein nacktes `git commit` — der Index trägt fremde Stufen.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-mails.md` und die
  drei `adoption-*.body.txt`.

## Benchmark

- Kein Doppellauf: der Stehende Pass + die PII-Lauf-Auswertung sind Routine
  (flash-Klasse), keine gemessene Benchmark-Klasse; kein pro/max-Dispatch nötig.
  Die Auswertung lief im Hauptkontext (`gh run view --log-failed`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
