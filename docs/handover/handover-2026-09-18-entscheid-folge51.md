<!--
  title: Handover — Entscheid-Folge 51 (Stand 2026-09-18)
  session: Entscheid-Folge 51
  class: handover
  date: 2026-09-18
  sha256: 0199bb3d0f950e731ddbdca242f8e9ee58c359d6490ef8f250730ffa38b6744d
  status: live
-->
# Handover — Entscheid-Folge 51 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, Entscheid-Folge 51)

- **HEAD** `642ff12c` (Session-Start; im Verlauf auf `14ccc152` fortgeschritten —
  bau/forschung pushten, `origin/main` == HEAD); `git_safety` Snapshot
  `refs/safety/1789725855`.
- **CI** — Watchdog-Snapshot 15:12Z + `ci_manage list` (~10:40Z): die
  `fugin-cdn`-Welle (09:20–09:23Z) ist durchweg **success**; **failure**:
  `ci-check` `35329963293`/`35332746196`/`35338058140`, `harvest`
  `35330766271`/`35330768755`/`35338094778`; in_progress: `harvest`
  `35338097066`/`35338099137`, `ci-check` `35348641363`, `allwise-cdn`
  `35341349072`, `health-check` `35335138589`; cancelled: `harvest-long`
  `35330771831`/`35332922040`. Wert in `external-state` fortgeschrieben.
- **Post** — `post.md`: Zeile an ernte (bc_mpo_mag) unverändert; die
  BepiColombo-MORE-Zeile an forschung auf die Iess-Antwort aktualisiert.
- **Postfach** — Iess-Antwort `1789729151` (kein Zwischenzugang; Freigabe zur
  Wissenschaftsphase); eigene Antwort gesendet `1789737560`; kein weiterer
  Agenten-Eingang. `external-state` fortgeschrieben.
- **Arbeitsbaum** — fremd uncommittet (nicht angefasst): `src/mathematikerin/te.rs`
  (M), `forschung-folge81`→`archiv/` (staged), drei `handover-2026-09-16-*`-Renames
  (staged), `forschung-folge82` (neu), `tools/measure/src/bin/{betti0,silence_map}_probe.rs`
  (neu). Die früheren ernte-`bc_mpo_mag`- und bau-`static/*`-Pfade sind inzwischen
  committet (HEAD `14ccc152`).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** operator-gebunden/blockiert/
wartend sind keine Auswahlpunkte; die operator-gebundenen liegen dem Operator vor.

- `termin` — **Lasair-LSST**: API 502, Wiedervorlage nach dem at-risk-Fenster.
  (Schritt: `archive_search --verdict https://api.lasair.lsst.ac.uk/api/query` —
  Trigger 2026-09-19 in `external-state`.)
- `operator-gebunden` — **Pipeline-Port force-Gate**: kein sanktionierter Ort für
  den `--port`-Lauf. Weg A lokaler Release-Lauf auf den gitignorierten Korpora |
  Weg B CI-Workflow mit Korpus + `--port`/`--probe`. (Schritt: Operator-Wort A/B.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `wartend` — **SuperDARN**: Zugangsanfrage 2026-09-18 gesendet (Ledger
  `1789718159`); Globus-Einladung ausstehend. (Auslöser: Operator-Postfach.)
- `termin` — **BepiColombo MORE** (`bc_mpo_more`): PI Luciano Iess
  (`1789729151`) — Cruise-Daten werden erst zum Beginn der Wissenschaftsphase
  (~April) freigegeben, kein Zwischenzugang; eigene Antwort gesendet `1789737560`.
  (Schritt: Wiedervorlage ~April; `archive_search --verdict` am MORE-URN.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten.
- `wartend` — **GitHub PII-Exposition**: 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt). (Auslöser: GitHub-GC-Antwort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, Privacy-Löschung (Ref `01a0b032`), NSE/Haug
  (Keller 17.09.: „in einigen Tagen"), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou (neue Prozedur nach
  CSES-02-Umstellung). (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: der Dispositions-Digest lebt im
  Quellcode, aber das PATH-Binary ist der alte Build. (Auslöser: Release-Build;
  Schritt: `ci_manage list` auf `release-build`.)
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
- ~2027-04 — BepiColombo MORE: öffentliche Freigabe (Wissenschaftsphase).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-18-entscheid-folge51.md` (neu)
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge50.md` (Move)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile; Header-sha)
- `docs/handover/post.md` (BepiColombo-MORE-Zeile an forschung aktualisiert)
- Fremd uncommittet (nicht angefasst): `src/mathematikerin/te.rs`,
  `forschung-folge81`→`archiv/` (staged), drei `handover-2026-09-16-*`-Renames
  (staged), `forschung-folge82`, `tools/measure/src/bin/{betti0,silence_map}_probe.rs`.

## Benchmark

- Kein flash/pro-Doppellauf: die Routine-Klasse (Reachability/Register) ist
  geschlossen (`grind-flash` $0.0008, 2026-09-16). Der stehende Pass lief lokal
  (`ci_manage list`, Watchdog-Snapshot, Ledger-Read) — kein Agenten-Dispatch nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
