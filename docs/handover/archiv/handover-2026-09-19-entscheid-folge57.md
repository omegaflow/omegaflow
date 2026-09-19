<!--
  title: Handover — Entscheid-Folge 57 (Stand 2026-09-19)
  session: Entscheid-Folge 57
  class: handover
  date: 2026-09-19
  sha256: 3aba57f69d3a0cef231926c8116f04787d54f01d49e049f54a42849176201d72
  status: live
-->
# Handover — Entscheid-Folge 57 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage. Der
Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl; Wartestellungen
(`wartend`) sind kein Auswahlpunkt, sondern nennen nur ihren Auslöser. Jeder Punkt
trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Entscheid-Folge 57)

- **HEAD** `1fd18b31` (== `origin/main`, „research folge101"); Safety-Net
  `refs/safety/1789847946` (Start). Seit Folge 56 (`8ce7caa0`) über den Merge
  `eb2e414e`, `9ef51d21` (ernte 102), `2a8c8aaf` (bau 96) auf `1fd18b31`.
- **CI** — `ci_manage list` (~22:18Z): **pending** `ci-check` `35472167436`,
  `te-gate` `35472169687`, `hyperscanning-te` `35472171277`; **in_progress**
  `fmt-apply` `35472907001` (dieser Session dispatcht), `allwise-cdn` `35471283884`;
  **success** `openneuro-cdn` `35471360435` (ds007822), `harvest` `35471362190`
  (gedi_l2a); **failure** `paper-check` `35470928180`; **cancelled** die
  ci-check-Kette (`35471873589`→…). Zustand-Ledger auf `1fd18b31` fortgeschrieben.
- **Postfach** — jüngster Eingang `1789853943` (Rubin-Forum-Summary, informativ);
  davor `1789795811` (Rubin AGN DP2). Kein entscheid-relevanter Eingang.
- **Post** — `post.md` war leer; diese Session hat 5 Zeilen an bau/forschung
  gelegt (Verdikte, Deadlock, fmt-apply, Riss 4).
- **Verdikte gelesen** — `openneuro-cdn` `35471360435` grün: ds007822 `.set`
  ergibt EEG `.bin` (sub-G08S03…G11S03) → **parser-gap geschlossen**;
  `harvest` `35471362190` grün: gedi_l2a. Beide an bau gepostet.
- **Riss 4 gemessen** — Ksg ist **nicht** off-path (TeEstimator::Ksg-Dispatch +
  ~7 Gates + `te-operating-point-sweep.yml --est ksg`); kein descope. An
  forschung gepostet.

## Offen

- **Fleet-Deadlock (härtester undatierter Punkt)** — bau folge97 hält seinen
  Commit-Satz **gestaged** (u. a. 3 Bin-Test-Zeilen in `.github/workflows/ci-check.yml`);
  forschung hält 4 `timeout-minutes`-Hunks in derselben Datei **unstaged** und
  committet sie nicht, weil ein pfad-begrenzter Commit baus gestagte Zeilen
  mitsweepen würde. Genau der `timeout`-Fix gibt dem Watchdog die Median-Basis
  (Selbstheilungspfad). **`operator-gebunden`** (Commit-Wort auf bau97). (Schritt:
  `/commit` auf der bau-Linie, dann forschung-Hunks committen — an forschung
  gepostet.)
- **F1 — Rat-Konvention** „keine gestagten Fremd-Hunks in geteilten Dateien am
  Sessionende" als AGENTS.md-Zeile. **`operator-gebunden`** (Rat-Beschluss).
  (Schritt: Ratssitz `council` oder Operator-Wort.)
- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}, dann n setzen
  oder Driver-Design ändern; Floor bleibt. An forschung gepostet. **`wartend`**.
- **Pipeline-Port force-Gate — binär A/B** (seit folge46). **`operator-gebunden`**.
- **vC-Permeabilität — Vollzug** (Operator-Maschine). **`operator-gebunden`**.
- **register_lookup-Symlink** — PATH-Eingriff. **`operator-gebunden`**.
- **opencode-Browser-Bridge** — kein Target (`browser_targets` leer). **`operator-gebunden`**.
- **Limadou PI-Freigabe** — per-act consent. **`operator-gebunden`**.
- **`tools/harvest/**` fehlt in den ci-check-Push-Pfaden** — baus benannter
  Konfound; Aufnahme ist ein Bau-Atom. An bau gepostet. **`wartend`**.
- `termin` — **Lasair-LSST** (API 502, Backend server-seitig; Trigger
  Banner-Wechsel). **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC, Rubin-Review, Sonden-Antworten.

## Geteilter Baum — eigener Pfad-Satz

- `docs/zustand/external-state.md` (CI-Zeile `2a8c8aaf` → `1fd18b31`)
- `docs/handover/post.md` (5 Zeilen an bau/forschung)
- `docs/handover/handover-2026-09-19-entscheid-folge57.md` (neu)
- Move `handover-2026-09-19-entscheid-folge56.md` → `archiv/`
- Fremd uncommittet (nicht angefasst): bau97s gestagter Satz, forschungs
  `ci-check.yml`-Timeout-Hunks, die drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Kein Dispatch — kein `task`-Werkzeug in dieser Laufzeit verfügbar; die
  autonomen A0-Aktionen (fmt-apply, Verdikt-Lesungen, Ksg-Messung) liefen in der
  Hauptsession. Routineklasse geschlossen (`grind-flash` $0.0008, 2026-09-16).
  Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
