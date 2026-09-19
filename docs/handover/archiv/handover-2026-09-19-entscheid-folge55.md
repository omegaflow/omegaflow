<!--
  title: Handover — Entscheid-Folge 55 (Stand 2026-09-19)
  session: Entscheid-Folge 55
  class: handover
  date: 2026-09-19
  sha256: 7f14f345e0a7985896720c293fc99f9cec2cea289c96ba79a7a0eede187658f7
  status: live
-->
# Handover — Entscheid-Folge 55 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Entscheid-Folge 55)

- **HEAD** `a786e208` (== `origin/main`, „pipeline: register the four OpenNeuro
  hyperscanning datasets"); `git_safety` Snapshot `refs/safety/1789840870`. Der
  Baum ist seit Folge 54 (`49801083`) über `160bf350` (bau folge90), `b980/2c13f7da`
  (forschung folge93) auf `a786e208` gezogen — fremde Linien committen und pushen;
  Fast-Forward.
- **CI** — Watchdog-Snapshot (17:06Z) + `ci_manage list` (2026-09-19 ~18:00Z):
  **in_progress** `ps1-cdn` `35459788459`, `hyperscanning-te` `35457694014`,
  `ci-check` `35456865096`, `measure-gates` `35456056999`; **pending** `ci-check`
  `35457844175`, `measure-gates` `35457737916`; **success** `swpc-mirror-cdn`
  `35459629646`, `harvest` `35457743119`, `auto-dispatch` `35457724169`,
  `harvest-dispatch` `35457724140`, `hyperscanning-te` `35456288477`,
  `allwise-cdn` `35456069108`; **cancelled** (überschrieben) `ci-check`
  `35457764693`/`35457724141`/`35457588800`/`35457443212`/`35456571554`/`35456216825`/
  `35456079177`/`35456048116`. Die früheren failures (`measure-gates` `35454958335`,
  `ci-check` `35451506666`) liegen außerhalb der letzten 20 Läufe. Wert in
  `external-state` bei HEAD-Wechsel fortgeschrieben.
- **Postfach** — `mail_digest`: jüngster Eingang 2026-09-19 (Rubin-Forum
  `[Science] AGN DP2`, informativ) = Ledger-Eingang `1789795811`; kein neuer
  Eingang seit Folge 54 (`state/mail/mail_ledger.φ`; Empfänger-Dienst `smail_recv`
  läuft auf 127.0.0.1:1619). Eintrag in `external-state` fortgeschrieben.
- **Post** — `post.md` leer; kein `An entscheid`. Keine Faltung nötig.
- **Arbeitsbaum** — fremd uncommittet (nicht angefasst): drei
  `handover-2026-09-16-*`-Renames (staged: `entscheid-folge24`,
  `forschung-folge44`/`-folge51` → `archiv/`).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** operator-gebunden/blockiert/
termin/wartend sind keine Auswahlpunkte; die operator-gebundenen liegen dem
Operator vor. Register-Hygiene dieser Session: vC-Permeabilität und
Betti-0-Schwelle wurden **gestrichen** — der Operator hatte am 2026-09-19
entschieden („Messakt lokal, kein CDN" bzw. „muss gemessen werden"), beide wurden
an **bau** delegiert (`bau-folge90:95-104`), und bau folge90 hat Betti-0 bereits
abgearbeitet (`1b0c3303`, Kalibrierung gegen die gemessene Null-Verteilung). Sie
gehören der Bau-Linie, nicht diesem Register.

### Operator-Entscheidungen

- **Pipeline-Port force-Gate — binär A/B** (seit `entscheid-folge46` offen, neun
  Sessions; jetzt scharf gestellt). Korpus: 10 gitignorierte
  `phi/pipeline/queue/*.φ` (~3.683 Blöcke, ~1,8 MB).
  - **A — lokal:** `--port` (billig, kein Netz) + `--probe` mit tausenden
    Netz-Fetches lokal. Konsequenz: schnell, aber der force-Gate-Beleg bleibt
    lokal/ungemanifestiert — die Korpora bleiben gitignoriert.
  - **B — CI:** Upload der 10 Korpora + Netz-Proben in CI. Konsequenz: geteilter,
    reproduzierbarer Beleg, aber die Korpora müssten aus dem gitignore, und der
    Upload-Pfad + das Netz-Volumen sind ungebaut.
  - (Schritt: **Operator-Wort A/B** — dies ist die einzige offene Entscheidung
    dieser Linie.)
- **ESP32-Modul** — on hold (Operator-Wort 2026-09-19); BOM
  `docs/specs/mantis-shrimp-bom.md`.
- `termin` — **Lasair-LSST**: API 502 (Root + `/api/query` + Token-Query) direkt
  *und* über Proton-Exit; Re-Messung 2026-09-19 bestätigt (Wayback 200 ohne
  Snapshot). **Kein Zugangs-Block** — Token liegt in `.secrets.local:59`; der
  Lasair-Backend-Upstream ist absent (nginx). (Schritt: `archive_search --verdict
  https://api.lasair.lsst.ac.uk/api/query` — Trigger Banner-Wechsel.)
- `termin` — **BepiColombo MORE** (`bc_mpo_more`): Cruise-Daten erst zur
  Wissenschaftsphase (~April 2027); PI Iess-Antwort `1789729151`. (Schritt:
  Wiedervorlage ~April; `archive_search --verdict` am MORE-URN.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten.
- `wartend` — **SuperDARN**: Globus-Einladung ausstehend. (Auslöser:
  Operator-Postfach.)
- `wartend` — **GitHub PII-Exposition**: 45 Kombinationen @`3b7aa5a1` (exit 2 =
  Exposition bleibt). (Auslöser: GitHub-GC-Antwort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801` (Privacy-Löschung, Ref `01a0b032`), NSE/Haug
  (Keller 17.09.: „in einigen Tagen"), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou (neue Prozedur nach CSES-02-
  Umstellung). (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: PATH-Binary ist der alte Build.
  (Auslöser: `release-build`-Run; Schritt: `ci_manage list`.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).
- ~2027-04 — BepiColombo MORE: öffentliche Freigabe (Wissenschaftsphase).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-19-entscheid-folge55.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-entscheid-folge54.md` (Move)
- `docs/zustand/external-state.md` (CI-Zeile + Postfach; Header-sha)
- Fremd uncommittet (nicht angefasst): drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Stehender Pass: kein Dispatch (lokal, Routineklasse geschlossen — `grind-flash`
  $0.0008, 2026-09-16). Keine harte Atom-Klasse offen; kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
