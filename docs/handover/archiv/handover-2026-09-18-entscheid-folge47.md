<!--
  title: Handover — Entscheid-Folge 47 (Stand 2026-09-18)
  session: Entscheid-Folge 47
  class: handover
  date: 2026-09-18
  sha256: 86b2a7a157e9b9fb592ccc6ef604765a9ff746f7eda6c072a0745497e660c3d2
  status: live
-->
# Handover — Entscheid-Folge 47 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, Entscheid-Folge 47)

- **HEAD** `b67f5cae` == `origin/main`. Baum stark fremd bewegt (fremd
  uncommittet, nicht angefasst): `src/archivar/*` (u. a. `fugin.rs` neu),
  `opencode.json`, `phi/blocked_sources.φ`, drei `handover-2026-09-16-*`-Renames.
- **CI** — `ci_manage list` 2026-09-18: `ci-check` `35318815330` pending,
  `35318671509` cancelled; `harvest` `35317120978`/`35317118656`/`35317116366`
  failure (Ernte-Linie); `swpc-mirror-cdn` `35319011399` success. Watchdog-Snapshot
  09:06 oben. `external-state`-CI-Zeile auf `b67f5cae` fortgeschrieben.
- **Postfach** — kein neuer Agenten-Eingang (letzter Ledger `1789689115`,
  2026-09-18, Publika/Rubin, kein Agenten-Reply) → zitiert, kein Re-Mess.
- **Post** — `docs/handover/post.md` leer; keine Nachricht an die Entscheid-Linie.
- **Lasair-LSST (neu gemessen)** — `api.lasair.lsst.ac.uk` (Root + `/api/query`)
  → **502** (nginx upstream absent, 166 B) über **zwei** Exits (US
  `149.102.242.106`, SG `149.50.211.161`); Hauptseite `lasair.lsst.ac.uk/` → 200.
  Kein Geoblock/IP-Block — das API-Backend antwortet nicht; Exit-Rotation nutzlos.
  Status-Seite `lasair.lsst.ac.uk/`: „scheduled maintenance … fully offline Sept
  14–16, at-risk on Sept 17-18" (Stand 2026-09-10 14:17 UTC); meldet „Lasair
  LSST: Up" (status code 200, 2026-09-18 07:58 UTC) — das misst das Web-Frontend,
  nicht das API-Backend; der 502 fällt in das at-risk-Fenster.
- **Digest** `register_lookup --live`: 555 offene Zeilen / 110 Docs. Die
  owner-getaggten Pipeline-Register fehlen noch — PATH-Binary ist der alte Build.

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** Die operator-gebundenen Punkte
liegen dem Operator vor; blockiert/wartend sind keine Auswahlpunkte.

- `pending` — **Lasair-LSST**: API 502 (Backend absent) im Wartungsfenster
  (14.–16.09. offline, 17.–18.09. at-risk; Status-Seite meldet Frontend „Up"),
  gemessen 2026-09-18 über US + SG; `LASAIR_LSST_TOKEN` unverified. (Schritt:
  Re-Messung nach Wartungsende
  `ALL_PROXY=socks5h://127.0.0.1:25344 curl -o /dev/null -w '%{http_code}'
  https://api.lasair.lsst.ac.uk/api/query` — Trigger in `external-state`.)
- `wartend` — **SuperDARN**: Request 2026-09-18 gesendet (Operator, von
  `johannes.tyroller@proton.me` an `superdarn@usask.ca`; Formweg durch defektes
  reCAPTCHA — „Ungültige Domain für Websiteschlüssel" — blockiert). Globus ID
  verifiziert, GCP 3.3.1 + Endpoint `omegaflow` laufen. (Auslöser:
  Gruppeneinladung im Operator-Postfach → annehmen + Rules of the Road; dann
  Transfer.)
- `operator-gebunden` — **Pipeline-Port force-Gate**: kein sanktionierter Ort für
  den `--port`-Lauf. Weg A lokaler Release-Lauf auf den gitignorierten Korpora |
  Weg B CI-Workflow mit Korpus + `--port`/`--probe`. (Schritt: Operator-Wort A/B.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten. Kein
  Sendeschritt.
- `wartend` — **GitHub PII-Exposition**: Wert 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt). (Auslöser: GitHub-GC-Antwort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, Privacy-Löschung (Ref `01a0b032`), NSE/Haug
  I(q,t) (Daten zugesagt), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou. (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: der Dispositions-Digest lebt im
  Quellcode (`cargo check -p omegaflow-register` sauber), aber das PATH-Binary ist
  der alte Build. (Auslöser: Release-Build; Schritt: `ci_manage list` auf
  `release-build`.)
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

- `docs/handover/handover-2026-09-18-entscheid-folge47.md`
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge46.md` (Move)
- `docs/zustand/external-state.md` (CI-Zeile auf `b67f5cae`; Lasair-Zeile neu)
- Fremd uncommittet/rot (nicht angefasst): `src/archivar/*` (u. a. `fugin.rs` neu),
  `opencode.json`, `phi/blocked_sources.φ`, die drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Kein flash/pro-Doppellauf: die Routine-Klasse ist geschlossen (`grind-flash`
  $0.0008, 2026-09-16). Der Lasair-Route-Diver (`research-max`) war der benannte
  harte Recherche-Fall; Ergebnis: Route blockiert nicht — API-Backend absent.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
