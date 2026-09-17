<!--
  title: Handover — Entscheid-Folge 31 (Stand 2026-09-17)
  session: Entscheid-Folge 31
  class: handover
  date: 2026-09-17
  sha256: 24580ed9cc73cdbfbdb8f124ac75160ec68d757bec02f315f4050ff66b2da8c7
  status: live
-->
# Handover — Entscheid-Folge 31 (2026-09-17)

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

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — zuerst den Watchdog-Snapshot
  `/tmp/opencode/ci_status.md` lesen (kein API-Aufruf); bei Lücke/Detail
  `ci_manage list` / `ci_manage view <id>`. **Nie** `gh run list`/`gh run view`;
  `gh` nur für `--log`/`--log-failed`/`workflow run`/`run download`.

## Werkzeuge (gebaut — nutzt sie)

- `archive_search` — Inhalt (`--root`)/Pfade (`--index`)/NTFS/16 Netz-Modi/`--playwright`/`--verdict`/`--sniff`/`--all`; ersetzt bash-`grep`, `curl`, webfetch.
- `sgrep [-i]` — Zeilensuche über `git ls-files`.
- `sfetch` — fetch; ersetzt `curl -s`.
- `omega_sh` — `reports|status|search|fetch|jwst`.
- `smail` — Mail (Resend), `--dry-run`; Inhalte nie getrackt.
- `register_lookup` — `--live`/`--open`/`--history`.
- `git_safety` — `--snapshot`/`--restore`/`--list`.
- `ci_manage` — `list`/`view`/`cancel`/`rerun`; statt `gh run list`/`gh run view`.
- `sread [--offset --limit]` — Datei lesen.
- `session_burn` — Burn je Session.
- `gh` — nur `--log`/`--log-failed`/`workflow run`/`run download`.

## Operator-gebundene Punkte

- **adoption-Block** — `state/mail/adoption-mails.md`: drei Entwürfe (Toth,
  Turyshev, Markwardt, je Quelle in der To-Zeile). **Gated (Operator-Wort
  2026-09-17):** Senden erst, wenn die Forschung (20-s-Bande) komplett durch ist.
  Operator-Sache — keine empfohlene Aufgabe. (Schritt: je cleanen Body
  extrahieren; Sende-Wort erst nach Forschungsabschluss.)
- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- **DEMETER-Runner** (aus `post.md` gefaltet) — der Harvest ist nur von einer
  Residential-IP möglich (F5-ASM-WAF blockt GitHub-Runner-IPs auf
  `POST /api/v1/rs-order/user/orders`, dieselbe Anfrage von der Operator-Maschine
  = `201`); CI `35146819646` war `success` bei `0 files on disk` (falsches Grün).
  Route (Rat): self-hosted Runner, Label `demeter-residential`, trägt
  `demeter-cdn.yml`; `demeter-aggregate-cdn.yml` bleibt `ubuntu-latest`. (Schritt:
  Operator installiert den Runner — repo-gebunden, Label nur `demeter-residential`,
  eigener Nutzer, systemd `svc.sh install`, nie in einem `pull_request`-Workflow;
  dann `gh workflow run demeter-cdn.yml`.)

## Warten auf Rückmeldung (extern)

- **NSE/Haug** — Thomas Keller (TRISP, MPI-FKF) antwortet 2026-09-17 09:29
  (gemessen im Ledger): er sendet die TRISP-NSE-Daten in einigen Tagen. (Schritt:
  Wiedervorlage ~2026-09-24 — Postfach prüfen, Datenannahme registrieren.)
- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **Rubin-Review** (Shaughnessy, high volume) — offen.
- **GitHub GC #4761801** — Follow-up gesendet 2026-09-17 (Resend
  `01a0aded-75a3-7548-a994-682b9ef54843`), Antwort offen.
- **GitHub Privacy-Löschung** — gesendet 2026-09-17 (Resend
  `01a0aded-819e-75db-aaae-2ff654b6ec15`), Antwort offen.
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
