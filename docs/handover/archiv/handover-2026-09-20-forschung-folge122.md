<!--
  title: Handover — Forschung-Folge 122 (Stand 2026-09-20)
  session: Forschung-Folge 122
  class: handover
  date: 2026-09-20
  sha256: 4ba76162d14bd41242e6162a9c505a5f2fe1e26df458e33cf4bed7f8911a5d62
  status: live
-->
# Handover — Forschung-Folge 122 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 122)

- **HEAD** `58629476` (== `origin/main`, ernte folge121); Arbeitsbaum clean
  (`git_safety --snapshot` → `refs/safety/1789930439`).
- **Postfach** — **neuer Eingang `1789930255`** (`info@pine64.org`): Ox64 SBC wird
  zugesagt, bittet um Versanddaten + Telefonnummer. Davor `1789922257`
  (`sales@pine64.org`, Verweis). → operator-gebunden, siehe Punkt 3.
- **CI** — die `f75e3245`-Läufe endeten **cancelled** (kein Verdikt):
  `ci-check` `35530480032`, `tools-build` `35530482159`; `release-build`
  `35530483756` = **success**. `te-gate` `35513982359` @`7d0a1272` weiter
  `in_progress` seit 13:36Z (kein Update). Neu dispatcht @`58629476`:
  `ci-check` `35530750046`, `tools-build` `35530751626` (beide `pending`).

## Punkt 1 — CI-Verdikte @`58629476` (neu dispatcht)

- `ci-check` `35530750046` — `path_reference_scan`-Health-Gate +
  Relay-Routentest `top_level_static_js_routes_reach_the_asset`.
  (Schritt: `ci_manage view 35530750046`, Trigger Lauf-Abschluss)
- `tools-build` `35530751626` — Binär trägt `--features browser_relay` + neues
  `archive_search`-cjs (`OMEGAFLOW_COOKIES`). (Schritt: `ci_manage view 35530751626`)
- `te-gate` `35513982359` @`7d0a1272` — hängt seit 13:36Z; Verdikt der vier
  `fpr_rise_sigma_test`-Zeilen offen. (Schritt: `ci_manage view 35513982359`;
  past 2× Median → Watchdog cancelt, sonst `ci_manage cancel` + neu dispatch)

## Punkt 2 — Cookie-Editor-Export (Bridge-Grenze gemessen)

Der Export ist **Operator-Akt**, session-seitig nicht führbar (gemessen 2026-09-20):
die opencode-browser-Extension liest eine fremde `chrome-extension://`-Seite nicht
(„Cannot access a chrome-extension:// URL of different extension"); der
chrome-devtools-MCP hat einen eigenen Browser (`about:blank`); kein CDP-Endpunkt
`9222`. Die Empfangsmechanik ist gebaut (`OMEGAFLOW_COOKIES`, `playwright_fetch.cjs:12`).

(Schritt: Operator — Ziel-Site im Tab aktiv → Cookie-Editor → Export → JSON nach
`state/cookies/<host>.json` (untracked); dann `OMEGAFLOW_COOKIES=state/cookies/<host>.json
archive_search --playwright <url>` headless verifizieren)

## Punkt 3 — Pine64 Hardware-Sponsoring (zugesagt, operator-gebunden)

`info@pine64.org` `1789930255`: Ox64 SBC zugesagt, bittet um Versanddaten +
Telefonnummer. Dritt-Akt (Mail) + PII → per-Akt-Consent des Operators.

(Schritt: `entscheid`-Linie legt dem Operator vor; Antwortentwurf in `state/mail/`
(gitignored), PII nie getrackt)

## Punkt 4 — Hardware-Sponsoring Framework/Tuxedo (wartend, dritter)

Trigger Antwort (Framework `NG2HWBZM`, Tuxedo `#991311279`).
(Schritt: Trigger Antwort)

## Punkt 5 — Flyby-Path-2-Kette (termin:2026-09-28)

Zellen ab Perigäum füllen. (Schritt: ab Datum)

## Punkt 6 — NSE/Haug (wartend, dritter)

Trigger Dateieingang. (Schritt: Trigger Dateieingang)

## Punkt 7 — BepiColombo MORE (termin:2027-04)

Freigabe Wissenschaftsphase. (Schritt: ab Datum)

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. `ci-check`-Verdikt `35530750046` @`58629476` | wartend | eigen | `ci_manage view 35530750046` (Trigger Lauf-Abschluss) |
| 2. `tools-build`-Verdikt `35530751626` @`58629476` | wartend | eigen | `ci_manage view 35530751626` (Trigger Lauf-Abschluss) |
| 3. TE-Gate-Verdikt `35513982359` @`7d0a1272` | wartend | eigen | `ci_manage view 35513982359` (success → vier `fpr_rise_sigma_test`-Zeilen) |
| 4. Cookie-Editor-Export | operator-gebunden | operator | Ziel-Site aktiv → Cookie-Editor → Export → `state/cookies/<host>.json` |
| 5. Pine64 Versanddaten + Telefon | operator-gebunden | operator | `entscheid` legt vor; Antwortentwurf `state/mail/` |
| 6. Hardware-Sponsoring Framework/Tuxedo | wartend | dritter | Trigger Antwort |
| 7. Flyby-Path-2-Kette | termin:2026-09-28 | termin | Zellen ab Perigäum füllen |
| 8. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 9. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |

Kein session-abarbeitbarer undatierter Punkt bleibt: 1–3 sind `wartend` (Trigger
Lauf-Abschluss), 4–5 `operator-gebunden`, 6/8 `wartend` (dritter), 7/9 datierte
Wiedervorlagen.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-forschung-folge122.md` (neu)
- Move `handover-2026-09-20-forschung-folge121.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Status + Postfach-Zeile)
- `docs/handover/post.md` (`An entscheid`: Pine64 + Cookie-Bridge-Grenze)

## Benchmark

- Stehender Pass + Verdikt-Lesen direkt (kein Agent nötig); Cookie-Bridge-Grenze
  in drei Messungen belegt (`browser_snapshot`, `chrome-devtools_list_pages`, CDP
  `9222`). Kein pro/max-Doppellauf in diesem Atom.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
