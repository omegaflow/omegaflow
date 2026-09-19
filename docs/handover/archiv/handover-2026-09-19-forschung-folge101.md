<!--
  title: Handover — Forschung-Folge 101 (Stand 2026-09-19)
  session: Forschung-Folge 101
  class: handover
  date: 2026-09-19
  sha256: 2f13e680aa17b6f651afa45cca73b4091cd536b4f2797ec91bca29fdea63816a
  status: live
-->
# Handover — Forschung-Folge 101 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 101)

- **HEAD** `2a8c8aaf` (bau folge96) == `origin/main` bei Session-Beginn; FF stand.
  `git_safety` Snapshot `refs/safety/1789853205`.
- **Postfach** — kein neuer Eingang seit `1789795811` (Rubin-Forum AGN DP2,
  informativ); `post.md` trägt nur `An bau`, **keine** `An forschung`-Zeile.
  Eintrag zitiert (`external-state.md`), nicht kopiert.
- **CI** — die `external-state.md`-CI-Zeile auf HEAD `2a8c8aaf` fortgeschrieben
  (`ci_manage list` ~21:56Z): **pending** `ci-check` `35471873589` @`2a8c8aaf`;
  **in_progress** `ci-check` `35470928166`, `silence-map-probe` `35468942740`,
  `allwise-cdn` `35471283884`, `placebo-ave-cdn` `35470938996`; **success**
  `measure-gates` `35468941451`, `harvest` `35471362190`, `openneuro-cdn`
  `35471360435`; **failure** `paper-check` `35470928180`, `openneuro-cdn`
  `35468606830` @`7aa5c23e` (ds007822 .set, parser-gap); **cancelled** die
  ci-check-Kette `35470773186`→`35470468101`→…→`35468939645`.

## ci-check-Gate — Bruch diagnostiziert, vier Fixes gebaut — `wartend` (CI-Verdikt)

Diagnose (research-max, gemessen): ci-check hat **0 Erfolge in den letzten 100
Läufen** — vier unabhängige Ursachen: (1) test-Job rot (3 Tests), (2) format-Job
rot (fmt-Drift im committed code; `fmt-apply.yml` nur workflow_dispatch), (3)
clippy-Job rot (`unreachable!` in `src/mathematikerin/te.rs:1216`), (4) kein
`timeout-minutes` → Watchdog hat keine Erfolgs-Median-Basis → kein
Selbstheilungspfad; die per-ref-Concurrency (`ci-check.yml:15-17`,
`cancel-in-progress: false`) verwirft jeden pending-Vorgänger (GitHub-Verhalten).

Gebaut (dieses Atom):
- `conditional_te_surrogates_n` — `unreachable!`-Zweig zu einem exhaustiven
  `match null` refactored (`te.rs` ~1188) → heilt clippy.
- `gate_mi_lag_stability` `#[ignore = "… runs in te-gate.yml"]` + Step in
  `te-gate.yml` (schwer, n bis 4096 × 200 Surrogate).
- `flare_envelope_conditional_keeps_true_coupling` `#[ignore = "… runs in
  te-gate.yml"]` + Step in `te-gate.yml` (Power-Problem, s. u.).
- Familien-Gates (`family_fn_gate`/`family_fp_gate`) an eigene Spur: Test-Step in
  `hyperscanning-te.yml` (eigene Gruppe, eigener Slot).
- **`timeout-minutes`** (build 60 / format 15 / test 120 / clippy 120) in
  `ci-check.yml` — **nicht committet**: der Index trägt fremde staged Hunks
  (bau folge97: openneuro/brainvision/snirf-Teststeps); die eigenen Timeout-Hunks
  liegen unstaged bereit. (Schritt: die eigenen `ci-check.yml`-Hunks committen,
  sobald der fremde staged Satz geräumt ist.)

Riss-Auflösung (grind-max, gemessen): die drei roten Tests aus Run `35465331299`
@`eb2e414e` sind **Pre-Swap-Messungen** — `eb2e414e` ist kein Nachfahre des
folge98-Swaps `4638d7f3`; kein post-Swap-`ci-check` lief je. Tests 1+2
(`coherent_phase_null_absorbs_linear_cross_coupling`,
`coherent_phase_null_detects_nonlinear_transfer`) sollten post-Swap grün sein
(Null-Bau verifiziert, Marge ≥20×/40×, Kovarianz-Algebra). (Schritt: das Verdikt
des `ci-check` auf dem neuen HEAD lesen — `ci_manage view <id>`; grün ⇒ Tests 1+2
halten, rot ⇒ der Swap war falsch.)

## Flare-Gate — Power-Studie — `pending`

`flare_envelope_conditional_keeps_true_coupling` bleibt rot: 43 % (13/30) gegen
den 50 %-Floor — ein **Power-Problem** des Designs bei n=240 (die
Driver-Innovation steckt unter der α=0.9-AR-Historie; Signal ≈1.8σ über dem
Null-Mittel), nicht Null/Schwelle (beide sauber kalibriert, gemessen; Rat folge87
streicht Tuning). (Schritt: print-only Power-Probe bei n∈{400,600,1000}
(found/meas je n, keine Assertion) in `te-gate.yml`; danach das Gate an das n
setzen, dessen gemessene Power den Floor mit Marge trägt — oder Rat-Wort über das
Driver-Design.)

## format-Job — fmt-Drift — `pending`

`cargo fmt --check` rot (committed code, baumweit). (Schritt:
`gh workflow run fmt-apply.yml` — CI-seitig, berührt den lokalen Arbeitsbaum
nicht.)

## CI-Verdikte am Gate-Fluss — `wartend` (nach dem ci-check-Neulauf)

- Riss 2 (FFT-Pad-Randartefakt) + Riss 3 (τ-Instabilität): Messgates gebaut
  (folge100); Verdikt aus dem `ci-check`/`te-gate` auf dem neuen HEAD.
- TE-Kalibrier-Gate CoherentPhase: Fix committed; Verdikt = Tests 1+2 oben.
- `silence-map-probe` `35468942740` in_progress (measure-gates-Budget abgetrennt;
  `measure-gates` `35468941451` success). (Schritt: Verdikt lesen, dann ist der
  Punkt geschlossen.)
- Takens-Screen Wandzeit `35465589119` in_progress; danach Bestätigungsstufe
  p99/1000 + Frontalkanäle F3/F4 (`gh workflow run hyperscanning-te -f …`).
- `te-gate` `35462518676` @`3d2e6adb` pending (Arme frei / `211A→193A`).
- Abgeleiteter Watchdog-Floor: nach der Takens-Wandzeit-Messung.

## operator-gebunden

- **Riss 4 — Ksg off-path**: verdrahten oder descopen mit gemessenem „nicht auf
  dem Pfad". (Schritt: Operator-Wort; an `entscheid` posten.)
- **vC-Permeabilitäts-Karte**: Messakt
  (`OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow`, dann
  `perm_target_probe --live <pfad>`) + Richtungssemantik. (Schritt: Operator-Wort
  + Release-Build.)

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09.).

## Benchmark (dieses Atom)

- **Rat** (Gate-Fluss-Architektur): council (pro/max) — Verdikt 4 Schritte;
  `per-ref` bleibt, `timeout` + Konvention + Familien-Spur.
- **Diagnose** (ci-check-Ursachen): research-max (pro/max) — vollständige,
  run-id-genaue Ursachenliste.
- **Harter Kern** (TE-Null): grind-max (pro/max) — Verdikt: kein Edit nötig
  (Pre-Swap), Power-Problem benannt.
- **Bau** (Workflow-/te.rs-Fixes): `build` (flash).
- Kein flash-Gegenspieler für die drei harten Atome — sie sind inhärent pro/max
  (Diagnose/Rat/TE-Null-Konstruktion). Kein Doppel-Lauf in dieser Runde.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (clippy-Refactor; `#[ignore]` `gate_mi_lag_stability`
  + `flare_envelope_conditional_keeps_true_coupling`)
- `.github/workflows/te-gate.yml` (zwei Steps)
- `.github/workflows/hyperscanning-te.yml` (Familien-Gates-Step)
- `docs/zustand/external-state.md` (CI-Zeile auf `2a8c8aaf` fortgeschrieben)
- `docs/handover/handover-2026-09-19-forschung-folge101.md` (neu) +
  `archiv/handover-2026-09-19-forschung-folge100.md` (Move)
- **Uncommittet gelassen:** `.github/workflows/ci-check.yml` (eigene
  Timeout-Hunks) — fremder staged Satz (bau folge97).
- **Fremd (nicht angefasst):** `docs/handover/handover-2026-09-19-bau-folge97.md`,
  `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `.github/workflows/openneuro-eeg-probe.yml` (staged), die drei Handover-Moves
  `entscheid-folge24`/`forschung-folge44`/`forschung-folge51` → `archiv/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
