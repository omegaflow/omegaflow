<!--
  title: Handover — Entscheid-Folge 34 (Stand 2026-09-17)
  session: Entscheid-Folge 34
  class: handover
  date: 2026-09-17
  sha256: 427e377c369e02ba757033d04d44c7780a7c99e11414a1678ff7ce47a66c67f0
  status: live
-->
# Handover — Entscheid-Folge 34 (2026-09-17)

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

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `state/mail/mail_ledger.φ` gemessen (Session-Read): neuester
  Eingang Rubin-Forum-Summary (`1789650823`); davor die NSE/Haug-Kette
  (`1789631158`, `1789650050` — Keller/TRISP sendet die Daten, „a few days");
  die GitHub-Support-Ablehnung (`1789624439`, siehe operator-gebunden);
  Cloudflare-Login-Code-Mails (`1789628784`–`1789629095`, IP `185.51.184.2`) —
  gemessen, nicht bewertet. Die fünf Sonden, Rubin-Review, GitHub Privacy bleiben
  ohne Antwort.
- **CI-Status @HEAD `84489abb`** — Watchdog-Snapshot 17:05Z: `pii-exposure`
  `35230489744` **failure = `exit 2` (PII retrievable — by design)** @`5dcdda0d`
  (der Workflow endet `exit "$code"`: 0 absent, 2 retrievable, 1 incomplete; die
  rote Konklusion IST das Messsignal, kein Code-Fehler); `paper-check`
  `35228278279`/`35224760511` failure (stale — sha-Fix liegt in `5dcdda0d`);
  CDN-Runs aktiv (demeter `35228716483`, ps1 `35227075857`, physionet
  `35222902368`/`35222570485`, gaia-xp-full `35222746258` queued, planetary-odf
  `35218760142`). (Schritt: `ci_manage list`/`view`.)
- **Zustand-Ledger** — `docs/zustand/external-state.md` trägt weiter **fremde
  uncommittete Hunks** (Postfach/PII/CI-Zeilen; PII @`bc9d6a0b`). Die neue
  PII-Messung @`5dcdda0d` (45, unverändert) wird **zitiert, nicht überschrieben**;
  die PII-Zeile wird nachgetragen, sobald die fremden Hunks committet sind.
  (Schritt: fremde Arbeit committen lassen, dann PII-Zeile @`5dcdda0d` setzen.)

## Operator-gebundene Punkte

- **GitHub GC/Privacy — E-Mail-Kanal tot, neue Route** — der `#4761801`-Follow-up
  (Resend `01a0aded-75a3…`, 2026-09-17) wurde per Auto-Antwort „Support Ticket
  Declined" abgelehnt (`1789624439`, `[RRKJ24-2JN2V]`): gefordert ist eine
  **angemeldete Session auf `https://support.github.com`** (Ticket `#4761801`
  referenzieren). `#4761482` ist durch `#4761801` überholt. Die Privacy-Löschung
  (`privacy@github.com`, Resend `01a0aded-819e…`, 2026-09-17) blieb ohne Antwort →
  Route `https://github.com/contact/privacy`. Der GC hat nichts bewirkt: PII
  weiter abrufbar, 45 (Datei,Ref)-Kombinationen @`5dcdda0d`. Beide Neuanträge sind
  Third-Party-Writes. (Schritt: **Operator-Wort** → angemeldete Session
  `omegaflow`, Ticket `#4761801` referenzieren.)
- **Delegation (Benchmark)** — die GitHub-Korrespondenz-Verifikation lief an
  `general` (flash); die Klasse „Routine-Recherche" ist geschlossen
  (`docs/concepts/tools-map.md`: flash-Sieger) — flash-first, keine Doppelmessung.
- **adoption-Block** — §4 bleibt offen, gemessen am Baum
  (`docs/paper/twenty-second-band-ground-chain.md:188–190`: „which named stage of
  the reduction carries the 44–58-mHz complex is the open measurement") — die
  Messung ist forschung-eigen; entscheid misst sie nicht. Der Amplituden-Anker
  (Reg 4) ist gemessen (`:68–78`, `pioneer-band-amplitude` 35119481936:
  A = 169/152/363 Hz). Post `An forschung` liegt in `post.md`. (Schritt: forschung
  benennt den §4-Abschluss; danach holt entscheid das Sende-Wort beim Operator
  ein — diese Linie sendet nie ohne Wort.)
- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Warten auf Rückmeldung (extern)

- **NSE/Haug** — Keller (TRISP/MLZ) sendet die Rohdaten („a few days", Mail
  2026-09-17); die Danke-Antwort ist raus. (Schritt: Postfach-Eingang, dann
  Datenannahme registrieren.)
- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **Rubin-Review** (Shaughnessy, high volume) — offen; einziger Eingang ist das
  Forum-Summary. Achtung: das Forum zieht am **2026-09-24** auf `rubin.community`
  um (Alt-Links bleiben übergangsweise). (Schritt: Postfach/Forum bei Eingang.)
- **GitHub GC `#4761801`** — abgelehnt; Neuantrag via `support.github.com`
  (operator-gebunden, siehe oben).
- **GitHub Privacy-Löschung** — gesendet 2026-09-17, Antwort offen; Neuantrag via
  `github.com/contact/privacy` (operator-gebunden).
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

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-entscheid-folge34.md`,
  `docs/handover/archiv/handover-2026-09-17-entscheid-folge33.md` (Move),
  `bin/omega_sh` (Build-on-Demand-Wrapper nach `bin/sgrep`-Muster; Install-Weg
  gemessen, `~/.local/bin/omega_sh` → `bin/omega_sh` umgehängt), `docs/handover/post.md`
  (sha-Korrektur — der `An bau`-Text liegt bereits in `a130d88d`). Fremde
  uncommittete Arbeit (nicht anfassen): die vielen uncommitteten
  `.github/workflows/*.yml` (bau), die gestagten Renames
  `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` →
  `archiv/`, `docs/zustand/external-state.md`, `phi/sources.φ`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
