<!--
  title: Handover — Entscheid-Folge 37 (Stand 2026-09-17)
  session: Entscheid-Folge 37
  class: handover
  date: 2026-09-17
  sha256: 668e00706787cee7c8932bc5f4bf732cfa9e06532c2e4f927feda995ad211ac6
  status: live
-->
# Handover — Entscheid-Folge 37 (2026-09-17)

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

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — `state/mail/mail_ledger.φ`, neuester Eintrag `1789662457`
  (2026-09-17 ~18:27): `support@githubsupport.com`, Ticket #4761801
  aktualisiert. Der Körper ist der Zendesk-Update-Hinweis, der den eigenen
  Nachfass-Kommentar (omegaflow, 17.09. 16:27 UTC) echot — kein Agenten-Reply.
  Davor: Keller/TRISP `1789631158` („a few days"), Danke `1789650050`,
  Rubin-Summary `1789650823`. Kein neuer externer Eingang seit Folge 36.
- **CI-Status @HEAD** — Watchdog-Snapshot `/tmp/opencode/ci_status.md` (18:09):
  `pii-exposure` `35230489744` failure = `exit 2` (PII retrievable, by design);
  `paper-check` `35228278279`/`35224760511` failure (stale, sha-Fix in
  `5dcdda0d`); CDN-Läufe aktiv/queued (Concurrency-Wartende). Kein
  Handlungsbedarf.
- **Zustand-Ledger** — `docs/zustand/external-state.md` trägt fremde
  uncommittete Hunks (Postfach/PII/CI-Zeilen). PII-Zeile steht auf `bc9d6a0b`;
  @HEAD `5f9cd61b` nicht setzbar, solange die fremden Hunks uncommittet sind.

## Handlungsfähig — Auswahlpunkte

- `operator-gebunden` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande):
  §4 ist geschlossen (forschung `b4e70b1d`, Paper v10, §8 light-time erster
  Kandidat). Entwürfe `state/mail/adoption-mails.md` gegen v10 abgeglichen, Pin
  auf `b4e70b1d` gesetzt; drei sendfertige Body-Dateien
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt`; `smail --dry-run`
  für alle drei sauber (from `code@omegaflow.space`). Das Senden ist der einzige
  irreversible Akt — Wort des Operators ausstehend; **auf keinen Fall senden**
  (Operator-Wort 2026-09-17). Exakt je Empfänger:
  `smail --to <addr> --subject "<s>" --body state/mail/adoption-<name>.body.txt --send`.
  (Schritt: Operator-Sende-Wort; danach Register-Zeile pro Mail, Antwort `pending`.)
- `operator-gebunden` — ESP32-Modul, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — Zustand-PII-Zeile @HEAD setzen, blockiert solange fremde Hunks in
  `docs/zustand/external-state.md` uncommittet sind. (Schritt: fremde Arbeit
  committen lassen, dann PII-Zeile @`5f9cd61b` setzen.)

## Wartend (extern) — kein Auswahlpunkt, kein Handlungsschritt

- `wartend` — GitHub GC `#4761801`: neuester Eingang `1789662457` (Zendesk-Echo
  des eigenen Kommentars, kein Agenten-Reply). (Auslöser: Postfach-Eingang.)
- `wartend` — GitHub Privacy-Löschung: Resend `01a0b032` (gesendet
  `1789662499`), keine Antwort. (Auslöser: Postfach-Eingang.)
- `wartend` — NSE/Haug I(q,t): TRISP/MLZ (Keller) sendet die Rohdaten
  („a few days", `1789631158`). (Auslöser: Postfach-Eingang → Datenannahme +
  CDN-Manifestation.)
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

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-entscheid-folge37.md`,
  `docs/handover/archiv/handover-2026-09-17-entscheid-folge36.md` (Move).
- Fremde uncommittete Arbeit (nicht anfassen): `docs/zustand/external-state.md`,
  `tools/measure/src/bin/pioneer10_cell_census_probe.rs`; fremd gestagt: die drei
  `archiv/`-Renames. Nie ein nacktes `git commit`.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-mails.md` (Pin auf
  `b4e70b1d`, §4-Nachtrag) und die drei `adoption-*.body.txt`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
