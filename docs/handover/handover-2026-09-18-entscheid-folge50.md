<!--
  title: Handover — Entscheid-Folge 50 (Stand 2026-09-18)
  session: Entscheid-Folge 50
  class: handover
  date: 2026-09-18
  sha256: bf19c706d919f4f67eefeca633f8da351b67ccbe677a865e10d12c197c78e82e
  status: live
-->
# Handover — Entscheid-Folge 50 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, Entscheid-Folge 50)

- **HEAD** `21590a43` (Ernte: „drop the consumed handover 82"); `git_safety`
  Snapshot `refs/safety/1789724045`.
- **CI** — `ci_manage list --limit 60` (~09:35Z): 56 `fugin-cdn` **queued**
  (09:20–09:23Z, TAP-Dispatcher der Bau-Linie), `harvest-dispatch` `35329963320`
  queued, `auto-dispatch` `35329963246` queued, `ci-check` `35329963293` pending;
  `te-gate` `35324015019` @`fb6b62b4` **pending** (seit 08:22Z); keine Failures
  im 60er-Fenster. Wert in `external-state` fortgeschrieben.
- **Post** — `docs/handover/post.md` trug die ernte-Zeile (bc_mpo_mag); kein
  Entscheid-Eingang. Neue Zeile an forschung gesetzt (BepiColombo MORE); auf
  Operator-Wort die PI-Anfrage an `luciano.iess@uniroma1.it` gesendet
  (`sent_ledger` `1789725472`, Resend `01a0b3f3…`).
- **Postfach** — letzter Ledger `1789723653` (PSA: BepiColombo MORE-Cruise
  nicht öffentlich bis zur Wissenschaftsphase ~April, Zwischenzugang über PI
  Luciano Iess); davor `1789718159` (eigene SuperDARN-Canada-Globus-Anfrage
  gesendet), `1789715019` (Brave-API-Schwellenwert-Alert, informativ); kein
  Agenten-Eingang. `external-state` fortgeschrieben.
- **Arbeitsbaum** — die eigene folge49-Arbeit war uncommittet übernommen
  (`handover-...-entscheid-folge49.md` neu, Rename folge48→`archiv/` staged,
  `external-state.md` modified); fremd uncommittet: drei
  `handover-2026-09-16-*`-Renames (nicht angefasst).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** Die operator-gebundenen Punkte
liegen dem Operator vor; blockiert/wartend sind keine Auswahlpunkte.

- `termin` — **Lasair-LSST**: API 502, Backend weiter absent; Frontend 200.
  Wiedervorlage nach dem at-risk-Fenster. (Schritt:
  `archive_search --verdict https://api.lasair.lsst.ac.uk/api/query` — Trigger
  2026-09-19 in `external-state`.)
- `operator-gebunden` — **Pipeline-Port force-Gate**: kein sanktionierter Ort für
  den `--port`-Lauf. Weg A lokaler Release-Lauf auf den gitignorierten Korpora |
  Weg B CI-Workflow mit Korpus + `--port`/`--probe`. (Schritt: Operator-Wort A/B.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `wartend` — **SuperDARN**: Zugangsanfrage 2026-09-18 an SuperDARN Canada
  gesendet (Ledger `1789718159`); Globus-Einladung ausstehend. (Auslöser:
  Gruppeneinladung im Operator-Postfach.)
- `wartend` — **BepiColombo MORE** (Radio Science, `bc_mpo_more`): PSA-Antwort
  2026-09-18 (Cruise-Daten nicht öffentlich bis ~April); Zwischenzugang über den
  PI Luciano Iess angefragt — Mail gesendet 2026-09-18 (`sent_ledger` `1789725472`,
  Resend `01a0b3f3…`). (Auslöser: PI-Antwort; Post an forschung gesetzt.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten.
- `wartend` — **GitHub PII-Exposition**: Wert 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt). (Auslöser: GitHub-GC-Antwort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, Privacy-Löschung (Ref `01a0b032`), NSE/Haug
  (Keller 17.09.: „in einigen Tagen"), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou (neue Prozedur nach
  CSES-02-Umstellung). (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: der Dispositions-Digest lebt im
  Quellcode (`cargo check -p omegaflow-register` sauber), aber das PATH-Binary ist
  der alte Build. (Auslöser: Release-Build; Schritt: `ci_manage list` auf
  `release-build`.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-19 — Lasair-LSST Re-Messung (nach dem at-risk-Fenster).
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-18-entscheid-folge50.md` (neu)
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge49.md` (Move)
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge48.md` (Move aus folge49, staged)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile; Header-sha)
- `docs/handover/post.md` (eigene Zeile an forschung)
- Fremd uncommittet (nicht angefasst): drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Kein flash/pro-Doppellauf: die Routine-Klasse (Reachability) ist geschlossen
  (`grind-flash` $0.0008, 2026-09-16). Der stehende Pass lief lokal
  (`ci_manage list`/`view`, Ledger-Read) — kein Agenten-Dispatch nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
