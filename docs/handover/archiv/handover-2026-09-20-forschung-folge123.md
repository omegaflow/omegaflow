<!--
  title: Handover — Forschung-Folge 123 (Stand 2026-09-20)
  session: Forschung-Folge 123
  class: handover
  date: 2026-09-20
  sha256: 8021cc41102591e10eaeacc94de627c20099ec6e94abd81da9064ca198b6d643
  status: live
-->
# Handover — Forschung-Folge 123 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein
Auswahlpunkt — sie nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 123)

- **HEAD** `e6b6eec8` (== `origin/main`, Ernte-Folge 122). Geteilter Baum trägt
  fremde uncommittete Arbeit (entscheid: `handover-2026-09-20-entscheid-folge66.md`,
  Move `entscheid-folge65.md` → `archiv/`).
  Sicherheitsnetz `refs/safety/1789930840`.
- **Postfach** — `state/mail/mail_ledger.φ`: 114 Zeilen, letzter Eingang
  `1789930255` (`info@pine64.org`) — **kein neuer Eingang**. Pine64 beantwortet:
  Versanddaten + Telefon an `info@pine64.org` gesendet (`sent_ledger` `1789931195`,
  Resend `01a0c036-8aea-70aa-8ade-29b1dfbd2d1c`).
- **CI** — `tools-build` `35530751626` = **success** @`cd742454` (Binär trägt
  `--features browser_relay` + `archive_search`-cjs `OMEGAFLOW_COOKIES`).
  `ci-check` `35530750046` @`58629476` **cancelled** (kein Verdikt); neu
  `ci-check` `35530850386` @`db9f489c` **pending**. `te-gate` `35513982359`
  @`7d0a1272` hing 7 h ohne Update (eigener 360-min-Timeout überschritten) →
  `ci_manage cancel` + neu dispatcht `te-gate` `35531196101` @`e6b6eec8` **pending**.

## Punkt 1 — CI-Verdikte @`db9f489c` / `e6b6eec8`

- `ci-check` `35530850386` @`db9f489c` — `path_reference_scan`-Health-Gate +
  Relay-Routentest `top_level_static_js_routes_reach_the_asset`. (Schritt:
  `ci_manage view 35530850386`, Trigger Lauf-Abschluss)
- `te-gate` `35531196101` @`e6b6eec8` — Verdikt der vier
  `fpr_rise_sigma_test`-Zeilen. (Schritt: `ci_manage view 35531196101`, Trigger
  Lauf-Abschluss)

## Punkt 2 — Cookie-Editor-Export (wartend, operator-gebunden)

Kein Ziel-Host genannt → bei Bedarf. Die Empfangsmechanik trägt
(`OMEGAFLOW_COOKIES`, `playwright_fetch.cjs:12`); der Export bleibt Operator-Akt.
(Schritt: Operator nennt Host → Ziel-Site aktiv → Cookie-Editor → Export → JSON
nach `state/cookies/<host>.json`; dann
`OMEGAFLOW_COOKIES=state/cookies/<host>.json archive_search --playwright <url>`)

## Punkt 4 — Flyby-Path-2-Kette (termin:2026-09-28)

Zellen ab Perigäum füllen. (Schritt: ab Datum)

## Punkt 5 — NSE/Haug (wartend, dritter)

Trigger Dateieingang. (Schritt: Trigger Dateieingang)

## Punkt 6 — BepiColombo MORE (termin:2027-04)

Freigabe Wissenschaftsphase. (Schritt: ab Datum)

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. `ci-check`-Verdikt `35530850386` @`db9f489c` | wartend | eigen | `ci_manage view 35530850386` (Trigger Lauf-Abschluss) |
| 2. `te-gate`-Verdikt `35531196101` @`e6b6eec8` | wartend | eigen | `ci_manage view 35531196101` (Trigger Lauf-Abschluss) |
| 3. Cookie-Editor-Export | wartend | operator | Operator nennt Host → Export → `state/cookies/<host>.json` |
| 5. Flyby-Path-2-Kette | termin:2026-09-28 | termin | Zellen ab Perigäum füllen |
| 6. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 7. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |

Kein session-abarbeitbarer undatierter Punkt bleibt: 1–2 sind `wartend` (Trigger
Lauf-Abschluss), 3 `operator-gebunden`/`wartend` (Host fehlt), 6 `wartend`
(dritter), 5/7 datierte Wiedervorlagen.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-forschung-folge123.md` (neu)
- Move `handover-2026-09-20-forschung-folge122.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Status + Postfach + TE-Gate-Zeile)
- `docs/handover/post.md` (Pine64-Zeile auf erledigt)
- `state/mail/pine64-ox64-reply.body.txt` + `state/mail/sent_ledger.φ` (gitignored,
  nicht committet)

## Benchmark

- Stehender Pass + CI-Verdikte direkt (kein Agent nötig). Kein pro/max-Doppellauf
  in diesem Atom; die Routine (CI-Lesen, Ledger, Mail) trägt der `build`-Kontext
  (flash-Tier) selbst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
