<!--
  title: Handover — Entscheid-Folge 36 (Stand 2026-09-17)
  session: Entscheid-Folge 36
  class: handover
  date: 2026-09-17
  sha256: dcb76e1aa9ce9f5f6398371796e39f0465377518d86b8f2524664caea398ac50
  status: live
-->
# Handover — Entscheid-Folge 36 (2026-09-17)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — `state/mail/mail_ledger.φ`, neuester Eintrag `1789662457`
  (2026-09-17 ~18:27): `support@githubsupport.com`, Ticket #4761801 aktualisiert.
  Der Körper ist der Zendesk-Update-Hinweis, der den eigenen Nachfass-Kommentar
  echot — kein Agenten-Reply. Letzte echte Eingänge: Rubin-Summary `1789650823`,
  Keller/TRISP `1789631158` (Daten „a few days", Danke `1789650050`).
- **CI-Status @HEAD** — Watchdog-Snapshot `/tmp/opencode/ci_status.md`:
  `pii-exposure` `35230489744` failure = `exit 2` (PII retrievable — by design);
  `paper-check` `35228278279`/`35224760511` failure (stale, sha-Fix in `5dcdda0d`);
  CDN-Läufe aktiv/queued (Concurrency-Wartende). Kein Handlungsbedarf.
- **Zustand-Ledger** — `docs/zustand/external-state.md` trägt fremde uncommittete
  Hunks. PII-Zeile steht auf `bc9d6a0b`; Messung @`5dcdda0d` (45 Kombinationen,
  unverändert) wird zitiert, nicht überschrieben.

## Handlungsfähig — Auswahlpunkte

Kein undatierter handlungsfähiger Punkt offen. Die Linie wartet; die drei
folgenden Punkte sind nicht durch die Session allein abarbeitbar.

- `operator-gebunden` — adoption-Block §4, forschung-eigen
  (`docs/paper/twenty-second-band-ground-chain.md:188–190`). (Schritt: forschung
  benennt §4-Abschluss; danach holt entscheid das Sende-Wort ein.)
- `operator-gebunden` — ESP32-Modul, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — Zustand-PII-Zeile @HEAD setzen, blockiert solange fremde Hunks in
  `docs/zustand/external-state.md` uncommittet sind. (Schritt: fremde Arbeit
  committen lassen, dann PII-Zeile @`5dcdda0d` setzen.)

## Wartend (extern) — kein Auswahlpunkt, kein Handlungsschritt

Reine Wartestellungen; ihr einziger Schritt ist der Postfach-Eingang. Nicht als
Auftrag lesen.

- `wartend` — GitHub GC `#4761801`: Ticket gemessen 2026-09-17, Status `Open`,
  2 Kommentare (beide `omegaflow`), kein Support-Reply. (Auslöser:
  Postfach-Eingang.)
- `wartend` — GitHub Privacy-Löschung: Resend `01a0b032`, keine Antwort.
  (Auslöser: Postfach-Eingang.)
- `wartend` — NSE/Haug I(q,t): MPI-FKF/TRISP sendet die Rohdaten („a few days").
  (Auslöser: Postfach-Eingang → Datenannahme + CDN-Manifestation.)
- `wartend` — fünf Sonden-Anfragen (NSSDC Voyager/Mariner 10/Viking, Cassini,
  Juno): gesendet 2026-09-16. (Auslöser: Postfach-Eingang.)
- `wartend` — Rubin-Review (Shaughnessy): Forum zieht 2026-09-24 auf
  `rubin.community`. (Auslöser: Postfach/Forum bei Eingang.)
- Offene Alternativen zu den Wartepunkten:
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

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

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-entscheid-folge36.md`,
  `docs/handover/archiv/handover-2026-09-17-entscheid-folge35.md` (Move).
- Fremde uncommittete Arbeit (nicht anfassen):
  `.opencode/command/{bau,entscheid,ernte,forschung}.md`,
  `docs/zustand/external-state.md`, `phi/sources.φ`,
  `src/archivar/{extract,main_flow,tests}.rs`; fremd gestagt: die `archiv/`-Renames.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
