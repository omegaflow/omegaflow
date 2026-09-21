<!--
  title: Handover — Forschung-Folge 133 (Stand 2026-09-21)
  session: Forschung-Folge 133
  class: handover
  date: 2026-09-21
  sha256: 2045d7ed44baad8d105a2eb15b4eae8d209a67a306f4a5035ecd187cc557edc9
  status: live
-->
# Handover — Forschung-Folge 133 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 133)

- **HEAD** `42aeeea4` == `origin/main`; Arbeitsbaum trägt fremde uncommittete
  Arbeit: der `upload_asset`→`upload_release`-Umbau (~76 Dateien, u. a.
  `src/archivar/cdn.rs`, `babamul.rs`, `ia2_tap.rs`, `quaoar_occlt.rs`, mehrere
  Workflow-YMLs) — nicht angefasst, nicht committet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", informativ); kein neuer Eingang, kein handlungsbedürftiger Fall.
- **CI** — `te-gate` `35578257445` @`2ae4978a` weiter **pending** (Ghost:
  `updated_at` seit 08:30:33 unverändert); `ci-check` `35585844643` @`42aeeea4`
  **pending** (misst den korrigierten `--dropped`-Zähler, Punkt 3); ältere
  `hyperscanning-te` `35584758519` @`c7cb201f` failure (Punkt 1, jetzt gefixt).

## Riss — getragen als Naht (gemessen 35584758519 @c7cb201f)

Der Riss trägt zwei gemessene Böden, die sich weigern zu konvergieren:
Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden
liegt weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet.
Die Naht trägt keine Schätzer-Aussage: der stochastic-driver-Arm (KSG τ=2,
excess +18.2 sd) trägt ihn getrennt; der rote Assert ist Fixture-/Gate-Frage
(Punkt 1). Mountain/River (weißer Boden ≈ Surrogat-Boden): **nicht bestätigt**.
Die getrennten μ-Zeilen der Vorfolge (W2 both-white +1.278e-3, W3 target-white
+3.335e-3; KDE K1 1.1134e-1, K2 1.0702e0, K3 6.3224e-1) stehen unberührt —
die Riss-Regel lief nur gegen W1.

## Punkt 1 — `family_fn_gate`: Fix A gesetzt, CI-Nachweis ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A implementiert (uncommittet): Assert
  `tools/measure/src/bin/hyperscanning_group_te.rs:1056` auf die gemessene
  Wahrheit geschrieben (`!cell_survivors.iter().any(…)`, „the sheet fixture
  carries no conditional transfer entropy beyond its coherent null"), plus
  Guard `null_mean > 0.0` („the per-cell null carries only the KSG estimator
  bias, never a degenerate zero"); der zweite Assert entschärft. `cargo check
  -p omegaflow-measure --all-targets` 0/0. Kein Workflow-Touch nötig.
- **Blockade:** CI-Nachweis (funktionaler Lauf nur in CI).
- **Braucht:** nach Commit/Push `gh workflow run hyperscanning-te.yml`; grüner
  Lauf schließt Punkt 4/5 auf.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35578257445` @`2ae4978a` **pending** (Ghost; `updated_at`
  08:30:33 unverändert).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578257445` einmal.

## Punkt 3 — `--dropped` Delta-Gate: Baseline nach gemessenem Zähler

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Klassifikator-Fix committet (`e9556f86`: `point_key_tokens` verwirft
  führende reine Zahl-Token + Test). `ci-check` `35585844643` @`42aeeea4`
  **pending** — misst den korrigierten Zähler gegen Baseline 1760
  (`docs/zustand/dropped-baseline.md`). Ältere `ci-check` `35578412609`
  @`a476ccfb` failure: baseline 1760 | current 1819 | delta 59.
- **Blockade:** Run-Abschluss (`35585844643`).
- **Braucht:** `ci_manage view 35585844643`; falls current > baseline, die
  Baseline auf den **gemessenen** Wert setzen (im akzeptierenden Commit, nie
  still).

## Punkt 4 — confirmation-Test nach grünem Screen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null` bleibt
  aus dem Teststep; assertet `observed.te > p99`, am aktuellen Schätzer rot.
- **Blockade:** grüner `family_fn_gate` (Punkt 1).
- **Braucht:** nach grünem Lauf `confirmation_…` in
  `.github/workflows/hyperscanning-te.yml` aufnehmen.

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen.
- **Blockade:** grüner Screen (Punkt 1).
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path (WGSL-KSG-Spiegel) — `operator-gebunden` — Post-Zeile
  `An entscheid:` in `post.md` (Forschung-Folge 132), unverändert; Antwort
  ausstehend.
- Cookie-Editor-Export — `wartend`/`operator` (kein Host benannt).
- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | Fix gesetzt (uncommittet), check 0/0 | CI-Nachweis | `gh workflow run hyperscanning-te.yml` |
| 2. `te-gate` n=1000 | wartend | eigen | `35578257445` pending (Ghost) | Run-Abschluss | `ci_manage view` einmal |
| 3. `--dropped` Baseline | wartend | eigen | `35585844643` pending | Run-Abschluss | `ci_manage view 35585844643` → Baseline |
| 4. confirmation-Test | wartend | eigen | rot am Schätzer | grüner Screen (←1) | Workflow-Zeile |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | getrennte Läufe |
| 6. Riss 4 Ksg (WGSL) | operator-gebunden | operator | Post-Zeile gelegt | Operator | entscheid-Antwort |
| 7. Cookie-Editor | wartend | operator | kein Host benannt | Operator | Host nennen |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- `grind-pro` für den `family_fn_gate`-Fix A (Gate-Urteil + Test-Assert-Edit) —
  kein Doppellauf; `council` für die Naht-Formulierung (read-only). Die
  FN-Diagnose steht seit Folge 131.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (Fix A, grind-pro)
- `docs/handover/handover-2026-09-21-forschung-folge133.md` (neu)
- Move `handover-2026-09-21-forschung-folge132.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/zustand/external-state.md` (CI-/Postfach-Zeile)

Fremde uncommittete Arbeit im selben Baum (`upload_asset`→`upload_release`-Umbau,
`survey-funding-erkundung.md`-Nachtrag, die `phi/*.φ`-Änderungen) wird **nicht**
angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
