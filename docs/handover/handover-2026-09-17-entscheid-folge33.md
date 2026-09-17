<!--
  title: Handover — Entscheid-Folge 33 (Stand 2026-09-17)
  session: Entscheid-Folge 33
  class: handover
  date: 2026-09-17
  sha256: 6e2903b2397799e57f3b5380d91ee012333bc330db6e8f784d17d4518a4d19f2
  status: live
-->
# Handover — Entscheid-Folge 33 (2026-09-17)

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

- **Postfach** — `state/mail/mail_ledger.φ` (91 Zeilen) gemessen: neuester Eingang
  Rubin-LSST-Forum-Summary (`1789650823`, kein Antwortschreiben); die NSE/Haug-Antwort
  (Keller/TRISP, 2026-09-17) ist gefaltet (forschung folge59). Die fünf Sonden-Anfragen,
  GitHub-GC #4761801, Privacy und Rubin bleiben ohne Antwort.
- **CI-Status am HEAD `e197fbec`** — Watchdog-Snapshot (16:01Z): `pii-exposure`
  `35230489744` **in_progress @`5dcdda0d`** (in der vorigen Session dispatcht),
  `paper-check` `35230433910` queued @`5dcdda0d`; failures `paper-check`
  `35228278279`/`35224760511` (blatt-der-grat sha — forschung folge59 fixt ihn in
  `5dcdda0d`); demeter-cdn `35228716483`, ned-cdn `35228000854`, physionet-cdn
  `35222902368`/`35222570485`, planetary-odf `35218760142`, health-check
  `35212981417`, allwise `35210315065` in_progress; gaia-xp-full `35222746258`
  queued.
- **Zustand-Ledger** — `docs/zustand/external-state.md` trägt **fremde uncommittete
  Hunks** (Postfach/PII/CI-Zeilen; forschung folge59 nennt es als nicht anzufassen)
  → zitiert, nicht überschrieben; die PII-Zeile (measured @`bc9d6a0b`) wird nach
  Abschluss von `35230489744` nachgetragen.

## Operator-gebundene Punkte

- **adoption-Block** — `state/mail/adoption-mails.md`: drei sendfertige Entwürfe
  (Toth, Turyshev, Markwardt, je Quelle in der To-Zeile). **Zuordnung korrigiert
  (2026-09-17):** der unmittelbare Blocker ist **forschung**, nicht entscheid — die
  §4-Messung („which named stage of the reduction carries the 44–58-mHz complex",
  `docs/paper/twenty-second-band-ground-chain.md:188–190`) ist forschung-eigen;
  entscheid misst sie nicht. Der Amplituden-Anker (Reg 4) ist gemessen
  (`:68–78`, `pioneer-band-amplitude` 35119481936: A = 169/152/363 Hz), §4 bleibt
  offen → das Gate bleibt **zu**. Post `An forschung` in `post.md`. (Schritt:
  forschung benennt den §4-Abschluss [Post liegt]; danach holt entscheid das
  Sende-Wort beim Operator ein — diese Linie sendet nie ohne Wort.) **Nicht
  gesendet** (Operator-Wort 2026-09-17).
- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Warten auf Rückmeldung (extern)

- **NSE/Haug** — MPI-FKF/TRISP sendet die Rohdaten („a few days", Mail 2026-09-17);
  Route in forschung folge59. (Schritt: Postfach-Eingang, dann Datenannahme
  registrieren.)
- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **Rubin-Review** (Shaughnessy, high volume) — offen; einziger Eingang ist das
  Forum-Summary. (Schritt: Postfach/Forum-Thread `community.lsst.org` bei Eingang.)
- **GitHub GC #4761801** — Follow-up gesendet 2026-09-17 (Resend
  `01a0aded-75a3-7548-a994-682b9ef54843`), Antwort offen.
- **GitHub Privacy-Löschung** — gesendet 2026-09-17 (Resend
  `01a0aded-819e-75db-aaae-2ff654b6ec15`), Antwort offen; PII-Exposition
  `pii-exposure` `35230489744` in_progress @`5dcdda0d` — Ergebnis beim nächsten Pass
  lesen, dann PII-Zeile nachtragen (fremde Hunks zitieren, nie überschreiben).
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-entscheid-folge33.md`,
  `docs/handover/post.md`, `docs/handover/archiv/handover-2026-09-17-entscheid-folge32.md`
  (Move). Fremde uncommittete Arbeit (nicht anfassen): die gestagten Renames
  `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` →
  `archiv/`, `docs/handover/handover-2026-09-17-ernte-folge65.md`,
  `docs/zustand/external-state.md`, `phi/sources.φ`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
