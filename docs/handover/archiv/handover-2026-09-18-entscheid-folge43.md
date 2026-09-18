<!--
  title: Handover — Entscheid-Folge 43 (Stand 2026-09-18)
  session: Entscheid-Folge 43
  class: handover
  date: 2026-09-18
  sha256: 1aea15d0d1756c94007ebf72c05ae15deb85cd6e337cb49213d371baafbd95e5
  status: live
-->
# Handover — Entscheid-Folge 43 (2026-09-18)

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

- **HEAD** `985bee3c` == `origin/main` (Folge 42 nannte `e0c27635`; der Baum ist
  über den eigenen Folge-42-Commit weitergezogen).
- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Rubin-Forum-Migrations-Thread, Publika-Reply, **kein
  Agenten-Eingang**); davor `1789670592` (Rubin-Forum, kein Agenten-Eingang).
  `sent_ledger`: GitHub-Privacy-Löschantrag `1789662499` (privacy@github.com,
  Ref `01a0b032`) gesendet. Kein neuer Agenten-Eingang.
- **CI @`985bee3c`** — `ci_manage list`/`view`: `ci-check` `35310276385` pending
  @`985bee3c`; `release-build` `35302966400` failure @`e64cc21b` (→ installiertes
  `ci_manage` kennt `log` nicht, Release-Build rot); `ci-check` `35302986980`
  failure; `xp-pilot-cdn` `35305640254` failure; `pii-exposure` `35287195140`
  failure (exit 2 = Exposition bleibt, erwartet); `pioneer-odf-cdn` `35309762909`
  success; sonst fremde Linien.
- **Zustand-Ledger** — `docs/zustand/external-state.md`: PII-Zeile Trigger
  korrigiert (HEAD-Wechsel gestrichen — `pii_exposure` liest 15 feste
  Pre-Rewrite-SHAs + 10 feste Pfade, HEAD-invariant; Beleg `pii_exposure.rs`),
  CI-Zeile auf `985bee3c`, Postfach-Zeile measured-at auf Folge 43.

## Handlungsfähig — Auswahlpunkte

- `wartend` — **GitHub PII-Exposition**: Wert 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt; `commits reachable 15/15`, `PII retrievable: yes`, `exposed
  combinations 45`). Trigger jetzt nur GitHub-GC-Antwort (HEAD-Wechsel
  gestrichen, HEAD-invariant). GC offen (Ticket #4761801); Privacy-Löschantrag
  gesendet (Ref `01a0b032`). (Auslöser: GitHub-GC-Antwort.) (Schritt:
  `gh run download 35287195140 -n pii-exposure` bei neuem Lauf; Wert in
  `docs/zustand/external-state.md` fortschreiben.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande): §4 ist
  geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor; **Senden
  ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein Sendeschritt, keine
  Sende-Anfrage, keine Register-Zeile.
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, GitHub Privacy-Löschung (Ref `01a0b032`,
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
  `docs/handover/handover-2026-09-18-entscheid-folge43.md`,
  `docs/handover/archiv/handover-2026-09-18-entscheid-folge42.md` (Move).
- Fremd uncommittet (nicht angefasst): die drei `handover-2026-09-16-*`-Renames
  (`entscheid-folge24`, `forschung-folge44`, `forschung-folge51`), `opencode.json`.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-*.body.txt` und
  `state/mail/adoption-mails.md`.

## Benchmark

- Kein Doppellauf: der Stehende Pass + die PII-Trigger-Korrektur (Quell-Read
  `pii_exposure.rs`, 15 feste SHAs, 10 feste Pfade) sind Routine (flash-Klasse),
  keine gemessene Benchmark-Klasse; kein pro/max-Dispatch nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
