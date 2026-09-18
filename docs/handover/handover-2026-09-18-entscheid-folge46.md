<!--
  title: Handover — Entscheid-Folge 46 (Stand 2026-09-18)
  session: Entscheid-Folge 46
  class: handover
  date: 2026-09-18
  sha256: e9b10d0750a4ab4dad7ee80c09930962807e88fa0d018bc52c3bfcc76b0076b2
  status: live
-->
# Handover — Entscheid-Folge 46 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, Entscheid-Folge 46)

- **HEAD** `6a8ecade` bei Messung (Bau-Folge 76 Run-ID-Commit). Der Baum ist stark
  bewegt — bau (`7b67b10d`, `6a8ecade`) und forschung (`45190022`) committeten
  während dieser Session parallel.
- **CI** — Zeile in `docs/zustand/external-state.md` von Bau-Folge 76 auf
  `7b67b10d` fortgeschrieben (`ci-check` `35314106560` + `te-gate` `35314110831`
  @7b67b10d pending); am selben Tag gemessen → zitiert, kein eigener Re-Mess
  (derselbe HEAD-Trigger wurde von bau gefeuert).
- **Postfach** — `state/mail/` im Baum absent; `smail` ist sende-only,
  `smail_recv` ein Empfangs-Daemon (`bind 127.0.0.1:1619`, bereits belegt). Kein
  lokaler Lese-Pfad in dieser Umgebung → Eintrag zitiert (Folge 45, kein neuer
  Eingang). `state/reports/mail_watchdog.φ`: health=200, recv=true, tunnel=true.
- **Baum rot (fremd)** — `cargo check` bricht in `src/archivar/fugin.rs:28`
  (untracked, fremde Linie, in Arbeit) — nicht der eigene Pfad-Satz; der eigene
  `register_lookup`-Code war davor `cargo check`-sauber (0/0).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** Die operator-gebundenen Punkte
liegen dem Operator vor; blockiert/wartend sind keine Auswahlpunkte.

**Neu surfaced (fremde Linien):** der Digest trägt jetzt die Pipeline-Register
owner-getaggt — `ledger.φ` (30 offen: 18 `ausstehend`/6 `kompiliert` → ernte,
6 `parser-gap` → bau), `sources.φ`/`witnesses.φ`/`footprints.φ`/`harvest.φ`/
`nrs_stations.φ`, die `probe_*`-Entwürfe, die Katalog-Pools als Zählzeile. Diese
Punkte gehören ernte/bau; ihr Pass liest sie aus dem einen Call — kein Post nötig.

- `operator-gebunden` — **SuperDARN** (`phi/blocked_sources.φ`, `blocked account`):
  Datenroute über Globus + PI-Vereinbarung (`superdarn.ca/piagreement`);
  Drittpartei-Akt, per-act consent. (Schritt: Operator-Wort für die
  Registrierung/Vereinbarung.)
- `operator-gebunden` — **Lasair-LSST** (gefaltet aus Post):
  `api.lasair.lsst.ac.uk` aus eigenem Netz absent (direct 0, Proton 502),
  Statusseite 200, `LASAIR_LSST_TOKEN` vorhanden/unverified. (Schritt:
  Operator-Wort für die Exit-Rotation
  `bin/proton-wg.sh suggest api.lasair.lsst.ac.uk`, dann Query-Messung mit Token.)
- `operator-gebunden` — **Pipeline-Port force-Gate** (gefaltet aus Post): kein
  sanktionierter Ort für den `--port`-Lauf. (Schritt: Operator-Wort — lokaler
  Release-Lauf auf den gitignorierten Korpora ODER ein CI-Workflow, der den Korpus
  trägt und `--port`+`--probe` fährt.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten. Kein
  Sendeschritt.
- `wartend` — **GitHub PII-Exposition**: Wert 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt). (Auslöser: GitHub-GC-Antwort.) (Schritt: `gh run download 35287195140
  -n pii-exposure` bei neuem Lauf; Wert in `docs/zustand/external-state.md`
  fortschreiben.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, Privacy-Löschung (Ref `01a0b032`), NSE/Haug
  I(q,t) (Daten zugesagt), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou. (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: der Dispositions-Digest lebt im
  Quellcode (`cargo check -p omegaflow-register` sauber), aber das PATH-Binary ist
  der alte Build — der Digest wird erst mit dem nächsten `release-build` sichtbar.
  (Auslöser: Release-Build; Schritt: `ci_manage list` auf `release-build`.)
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

- `tools/register/src/bin/register_lookup.rs` (Register-Digest + Gate-Tests)
- `AGENTS.md` (Work-Surface + Start-Pass + Register-Faltung)
- `docs/SOURCE_PORT.md` (§11 + Work-Surface-Zeile)
- `docs/handover/post.md` (zwei `An entscheid`-Zeilen gefaltet)
- `phi/declined_sources.φ` (AMS-02 `open no-consumer` → `decline aggregated-index`)
- `docs/handover/handover-2026-09-18-entscheid-folge46.md`
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge45.md` (Move)
- `phi/pipeline/{index,probe_wave,probe_hapi_proposed,probe_batch_skymap}.φ` — neu
  getrackt (`.gitignore`-Negation + `phi/canon.φ`-Deklaration, Operator-Wort);
  index.φ stale queue-Verweise → descoped bereinigt.
- `.gitignore`, `phi/canon.φ` (Kanon-Deklaration der vier Pipeline-Register).
- Fremd uncommittet/rot (nicht angefasst): `src/archivar/{fugin.rs (neu),extract,
  fetch,main_flow,mod,port}.rs` (bricht `cargo check` in `fugin.rs:28`),
  `.github/workflows/harvest.yml`, `tools/harvest/src/bin/{gedi_l2a,icesat2_atl03}_compiler.rs`,
  `opencode.json`, die drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Zwei Rat-Sitzungen (pro/max): Besitzer-Matrix + Register-Menge; Umsetzung an
  `grind-pro` delegiert (heterogene Register-Formate + Gate-Tests, ein Atom). Kein
  flash/pro-Doppellauf: die Routine-Klasse ist geschlossen (`grind-flash` $0.0008,
  2026-09-16) — der Digest ist ein Einzelbau, kein Benchmark-Atom.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
