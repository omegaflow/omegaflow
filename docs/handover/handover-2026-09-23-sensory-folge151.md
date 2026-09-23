<!--
  title: Handover — Sensory-Folge 151 (Stand 2026-09-23)
  session: Sensory-Folge 151
  class: handover
  date: 2026-09-23
  sha256: 2b2bfa793337364f7bb2a9640d12826a5e7c548dc15db39720e90a6537c25ad7
  status: live
-->
# Handover — Sensory-Folge 151 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Lage** (der
Zustand, gemessen) / **Blockade** (woran es hängt, oder „keine") / **Braucht**
(was es löst: Werkzeug, Datei, URL, Anfrage, Operator-Wort; „Schritt unbekannt —
erste Messung: X" ist ein vollständiger Schritt). Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt).

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 151)

- **HEAD** — Session-Start `629486b77` (mycelium 143) == `origin/main`; während der
  Session auf `9983f5b0` (river folge9) fortgeschritten — die eigene Arbeit liegt
  darauf. Der Baum trug zwischenzeitlich river folge9 uncommittet — **nicht
  angefasst**; river hat committet+gepusht.
- **Postfach** — `mail_digest` absent; `state/mail/` fehlt (keine sensory-Adresse).
  `post.md` trägt die `An mountain:` sfetch-Zeile + river's `An mycelium:`
  ci-check-Reds (folge9 `9983f5b0`) — **keine an sensory**, nicht angefasst.
- **Register** — `register_lookup --open`: keine sensory-eigenen Zustandseinträge.
- **`open_points_check`** der folge150: 11 Pfad-Refs, 2 „absent" — nur Glob-Muster
  (`phi/*.φ`, `src/archivar/{…}.rs`); **keine** stale Punkte.
- **CI** — `ci-check 35831089754 @629486b77` rot nur `dropped-gate` (delta 77);
  `ci-check 35833935838 @9983f5b0` pending; `te-gate 35806319936 @7e9ae7af1`
  **cancelled** am 6h-Hosted-Cap; `hyperscanning-te 35794690642 @eea5867e1`
  **completed success**. Kein Poll.

## Geschlossen in dieser Session (git trägt sie)

### Rat-Bedingung — FPR-Zellen-Re-Budget bestätigt
`ci-check 35794686502 @eea5867e1`: die geteilte Zellen-Batterie
`mathematikerin::te::tests::gate_fpr_autocorrelation_*_n_surr_200` **alle `ok`** —
`block` 22:57:54, `arx` 23:02:13, `restricted` 23:03:51, `coherent_phase` 23:04:15,
`shift` 23:05:09, `xshift` 23:06:14, `gate_fpr_rise_calibration` 23:06:15,
`block_ksg` 23:09:07, `shift_ksg` 23:12:56. Die D_Z=4-Zellen mit 21 Trials
(`te.rs:2936–2938`, `:2946–2948`) halten; keine Zelle ≥ 8.0. Die n=1000-Varianten
sind `ignored` → te-gate (neuer Split).

### Punkt 11b — frozen-tau/Confirmation grün
`hyperscanning-te 35794690642 @eea5867e1` **completed success**:
`confirmation_stochastic_driver_pair_clears_its_own_null` ok,
`riss_guard_deterministic_pair_measures_below_its_own_null` ok. Frozen-tau:
mi-path te=2.792949e-1 / p99=2.330514e-1 clears=yes; τ=2…12 clears=yes, τ=1 no.
Pos. Kontrolle: te=4.842021e-1 / p99=3.895387e-1 clears=yes. Blind-band: kein Bin
außerhalb der Surrogat-Hülle (phase + coherent-phase). Der AR(1)-Breitband-Treiber
(`ar1_noise(0.8, 1.0)`) trägt; der deterministische Sinus bleibt Riss-Guard.

## Offen (aufgeschlüsselt)

### te-gate-Split — gebaut, Dispatch + Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Beide te-gate-Läufe (`35767848399`, `35806319936`) enden `cancelled`
  am 6h-Hosted-Cap. Gemessen im Lauf `35806319936`: `shift_sweep_n1000` 2652.89 s,
  `gate_fpr_autocorrelation` (7 Tests) **17323.13 s** (4h49m), `block_sweep_n1000`
  272.43 s, `ksg_sweep_n1000` lief beim Cut — neun Schritte landeten nie. Der
  Workflow ist in **13 unabhängige Jobs + `issue`** gesplittet
  (`.github/workflows/te-gate.yml`); Rat-Verdikt: der 6h-Cap ist hart
  (GitHub-hosted), Split statt Timeout-Hoch; `needs:` nur auf `issue`, Timeouts
  = gemessen × 2–3, ≤360.
- **Blockade:** Commit+Push; danach Dispatch.
- **Braucht:** nach `/commit`: `gh workflow run te-gate.yml`, dann `ci_manage log`
  des gelandeten Laufs → `te_fn_probe`-Riss-4-Tabelle + die n=1000-Gates
  (`ksg_sweep`, `ksg_k_gate`, conditional, `mi_lag`). Der erste Split-Lauf ist die
  Messung der nie erreichten Schritte.

### HRV/Puls→Strahlung-Bindung (vC-Permeabilität)
- **Status:** operator-gebunden | **Bindung:** eigen (Hardware-Träger `operator`)
- **Lage:** die vC-Permeabilität ist Feldphysik; die TE-Apertur→Strahlung-Bindung
  ist gebaut (`omega.rs:347` `aperture = field_permeability * tone_scale`,
  `actuators.rs:29`, `tests.rs:570`, `356fa616`). `pending` ist der **Puls-Ankunfts**-
  Pfad: Operator-Puls/HRV → `tone_scale`; `src/archivar/hrv.rs` trägt das RMSSD/tone-Gate.
- **Blockade:** physischer Träger (ESP32, BOM).
- **Braucht:** Bindung Puls-Ankunft via ESP32-Firmware → Strahlungspfad bauen.

### Wartend / operator-gebunden / termin
- Flyby-Path-2-Kette — `termin:2026-09-28` (Kanäle live; Zellen ab Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18).

## Fremd-CI (geroutet, nicht sensory)

- `dropped-gate` delta 77 (Baseline 2680 | current 2757 @`629486b77`) — von river
  folge9 (`9983f5b0`) via `post.md` `An mycelium:` geroutet, zusammen mit dem
  `register_sort`-Red auf `phi/sources.φ` (ttl/url-order). Beide mycelium.
- `ci-check 35833935838 @9983f5b0` pending — Bestätigungslauf des river-Commits.
- `path_reference_scan` / die fünf `archive_search`-Reds sind geheilt (mountain138
  `576dddf98`, mountain139 `634437817`); im Code bestätigt
  (`archive_search.rs:1477–1484` lowercased das Nadel).

## Geteilter Baum — eigener Pfad-Satz

- `.github/workflows/te-gate.yml` (Split: 13 Jobs + `issue`)
- `docs/handover/handover-2026-09-23-sensory-folge151.md` (neu)
- Move `handover-2026-09-23-sensory-folge150.md` → `archiv/` (eigene Linie, atomar)

Fremd uncommittet hielt der Baum während der Session river folge9 — river hat
committet+gepusht (`9983f5b0`); der Baum trägt seither **nur noch die eigenen
Pfade**.

## Benchmark

- **Rat (pro/max, Architektur):** te-gate-Split-Verdikt — Option D (parallele Jobs),
  Option B (Timeout > 360) verworfen (6h-Hard-Cap). Kein Gegenlauf (Architektur).
- **`general` (flash):** Cap-Verifikation — GitHub-hosted Job-Limit 6h hart
  (`docs.github.com/en/actions/reference/limits`, HTTP 200 2026-09-23). flash
  vollständig.
- **`grind-flash` (flash):** mechanischer YAML-Split, 1 Datei, 127/25 Zeilen,
  self-verifiziert. flash vollständig, keine Eskalation.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push: `te-gate.yml`
dispatchen (`gh workflow run te-gate.yml`) und `ci-check 35833935838` einmal lesen.
`/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
