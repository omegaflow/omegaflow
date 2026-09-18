<!--
  title: Handover — Entscheid-Folge 42 (Stand 2026-09-18)
  session: Entscheid-Folge 42
  class: handover
  date: 2026-09-18
  sha256: 2a93d7dbcb5efc82ab8bbe4b1ed56d816b9a62f53ffa0ea244dd4aa0e9a13d4b
  status: live
-->
# Handover — Entscheid-Folge 42 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `e0c27635` == `origin/main` — Folge 41 nannte `da2cf8f9`; der Baum ist
  über `28eb8959`/`e64cc21b` weitergezogen. Der frühere `blockiert`-Punkt
  „Push-Divergenz" bleibt am Baum erledigt.
- **Postfach** — neuester `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Rubin-Forum-Migrations-Thread, Publika-Reply, **kein
  Agenten-Eingang**); davor `1789670592` (Rubin-Forum, kein Agenten-Eingang).
  `sent_ledger`: GitHub-Privacy-Löschantrag `1789662499` (privacy@github.com,
  Ref `01a0b032`) gesendet. `docs/zustand/external-state.md` trägt den Wert.
- **CI @`e0c27635`** — Watchdog-Snapshot 06:58 + `ci_manage list`: `ci-check`
  `35309762475` pending @`e0c27635`; `release-build` `35302966400` failure
  @`e64cc21b`; `ci-check` `35302986980` failure; `xp-pilot-cdn` `35305640254`
  failure; `pii-exposure` `35287195140` failure (exit 2 = Exposition bleibt,
  erwartet); `pioneer-odf-cdn` `35309762909` success; sonst fremde Linien.
- **Zustand-Ledger** — `docs/zustand/external-state.md`: Postfach-Zeile auf
  `1789689115`, PII-Zeile auf `35287195140` @`3b7aa5a1` (Zählwert 45 über das
  Artefakt `pii-exposure.txt` gelesen; Register-Fehler `@da2cf8f9` korrigiert),
  CI-Zeile auf `e0c27635`.

## Handlungsfähig — Auswahlpunkte

- `wartend` — **GitHub PII-Exposition**: Wert 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt; `commits reachable 15/15`, `PII retrievable: yes`, `exposed
  combinations 45` — identisch zu 45 @`ab45b3f3`). GC offen (Ticket #4761801);
  Privacy-Löschantrag gesendet (Ref `01a0b032`). (Auslöser: GitHub-GC-Antwort
  oder HEAD-Wechsel.) (Schritt: `gh run download <id> -n pii-exposure` beim
  nächsten Pass; Wert in `docs/zustand/external-state.md` fortschreiben.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande): §4 ist
  geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor; **Senden
  ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein Sendeschritt, keine
  Sende-Anfrage, keine Register-Zeile.
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, GitHub Privacy-Löschung (Resend `01a0b032`,
  gesendet 2026-09-17), NSE/Haug I(q,t) (TRISP/MLZ Keller; Daten zugesagt), fünf
  Sonden-Anfragen (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno), Rubin-Review
  (Umzug `rubin.community` 2026-09-24). (Auslöser: Postfach-Eingang.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-18-entscheid-folge42.md`,
  `docs/handover/archiv/handover-2026-09-18-entscheid-folge41.md` (Move),
  `docs/handover/post.md` (bau-Zeile + erledigte „An alle"-Zeilen),
  `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (zwei Zeilen
  „geschlossen mit Beleg").
- Fremd uncommittet (nicht angefasst): die drei gestagten/verschobenen
  `handover-2026-09-16-*`-Renames (`entscheid-folge24`, `forschung-folge44`,
  `forschung-folge51`), `opencode.json`.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-*.body.txt` und
  `state/mail/adoption-mails.md`.

## Benchmark

- Kein Doppellauf: der Stehende Pass + der PII-Artefakt-Read + die Register-/
  Survey-Pflege sind Routine (flash-Klasse), keine gemessene Benchmark-Klasse;
  kein pro/max-Dispatch nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
