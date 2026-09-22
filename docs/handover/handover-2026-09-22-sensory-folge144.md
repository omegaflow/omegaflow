<!--
  title: Handover — Sensory-Folge 144 (Stand 2026-09-22)
  session: Sensory-Folge 144
  class: handover
  date: 2026-09-22
  sha256: 668caeec8c34ef0b32045b91359ba35e0b3ab2997d66be1ff9f9c0727b23772e
  status: live
-->
# Handover — Sensory-Folge 144 (Stand 2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Sensory-Folge 144)

- **HEAD** — `26f3c1b9` == `origin/main` (future folge88). Die Sensory-Folge 143
  committete als `18d4c51c`; danach zogen ernte (`dbd399c3`/`0d73603d`) und future
  (`d71fc9f8`/`26f3c1b9`) den Branch weiter.
- **Postfach** — neuester Ledger-Eingang `1790023913` (Globus-Einladung `fitacf_30`,
  Maschine → mycelium), davor `1790023548` (GitHub-OAuth „Globus Auth",
  Sicherheitsereignis), davor `1790021001` (SuperDARN-Canada-Globus-Zugang, bereits
  `An mycelium:`); **keine** neue sensory-Zeile.
- **CI** (`ci_manage list`/`view`/`log`, kein Poll) — `ci-check 35649256512`
  @`3b42cb72` **failure** (clippy/format/test/dropped-gate); `ci-check 35656851467`
  @`0d73603d` **failure** (dieselben vier Jobs); `hyperscanning-te 35649230477`
  @`054e0c5e` **failure** im `confirm`-Job (stochastisch rot, Riss-Wächter grün);
  `te-gate 35628669014` @`3e319087` war **Ghost** (`ci_manage log` → „1 jobs, all
  green", `updated_at` 6 h eingefroren) → **cancelt** und neu dispatcht
  `35664286044`; Punkt-11-Sweep dispatcht `35664548297`.

## Offen (aufgeschlüsselt)

### Punkt 11b — frozen-tau Delay-Sweep (stochastischer Treiber)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirm`-Job von `hyperscanning-te 35649230477` @`054e0c5e` rot:
  `confirmation_stochastic_driver_pair_clears_its_own_null` FAILED
  (`TE 7.3470e-2 vs p99 2.8796e-1`); `riss_guard_deterministic_pair_measures_below_its_own_null`
  grün. Diagnose-Probe `frozen_tau_delay_sweep_stochastic_pair` gebaut
  (`tools/measure/src/bin/hyperscanning_group_te.rs:1681-1726`), Step
  `hyperscanning-te.yml:117-119` (`if: always()`), dispatcht `35664548297`.
- **Blockade:** Run-Landung.
- **Braucht:** die `frozen-tau sweep:`-Zeilen aus `35664548297` lesen. Hypothese
  (grind-max, Code-Lektüre): `find_mi_lag` (`te.rs:1832`) wählt für den
  Treiber (Periode 36) τ≈18 statt des Kopplungs-Delays 8; die Null friert die
  Original-MI-Lags ein.

### Punkt T — `endpoint_matched`-Regression (Rampe zurückgenommen)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `4e3ddb13` (2026-09-20) fügte `endpoint_matched`-Ramps in beide
  Surrogat-Generatoren (`phase_randomized_surrogate`, `coherent_phase_surrogates`);
  seither sind zwei **deterministische** Gates rot:
  `coherent_phase_null_absorbs_linear_cross_coupling` (`te.rs:4603`, obs 0.36200 >
  thr 0.35046) und `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200`
  (`te.rs:4371`, FPR 9.07 % > 8 % bei a=0.5 D_Z=4). Rat-Verdikt (d): Rampe zurück,
  Nominees/τ-Freeze/Fix A/Riss bleiben — umgesetzt (`te.rs`, beide Generatoren auf
  `fa44f315`-Stand). `cargo check --lib` 0/0.
- **Blockade:** CI-Lauf am Revert-Commit.
- **Braucht:** `ci-check`-Testjob — die zwei Gates müssen deterministisch grün sein
  (identischer Gate-Code war auf `fa44f315` grün).

### Punkt 3 — `dropped-gate` Baseline
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `dropped-gate` rot in beiden Läufen (`35649256512`: baseline 2286 |
  current 2541 | delta 255; `35656851467`: 2286 | 2597 | delta 311). Die Drops
  stammen aus Handover-Archivierungen (ernte/future) ohne Baseline-Nachzug;
  Baseline in diesem Atom von 2286 auf **2647** nachgezogen
  (`docs/zustand/dropped-baseline.md`).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci-check`-`dropped-gate`-Zeile (delta ≤ 0 erwartet).

### Punkt te-gate-Lesung (1/5/6/6b/F2 gebündelt — ein Run, ein Leseakt)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `te-gate 35628669014` war Ghost (cancelt); neu dispatcht `35664286044`.
  Trägt: Step 58 `te_fn_probe` (`te_scal`/`te_topo`, topologische FN-Spalte,
  Punkt 6/6b), Step 55 `flare_envelope_power_probe` (F2), `family_fn_gate`-Screen
  (Punkt 1), Takens-Wandzeit (Punkt 5).
- **Blockade:** Run-Landung.
- **Braucht:** `ci_manage log 35664286044` — screen-Log (1), `te_scal`/`te_topo`
  (6/6b), `flare power probe:` + Gate-Verdikt (F2), Wandzeit (5).

### Format-Job (repo-weit, blockiert `ci-check`)
- **Status:** wartend | **Bindung:** linie:<mehrere>
- **Lage:** `format`-Job rot in beiden Läufen — `Diff in` in
  `src/archivar/fai_kz.rs:47`, `src/archivar/hdf5.rs:3967/4007/4060/4069/4086/4163/4211/4249`,
  `src/archivar/ia2_tap.rs:96`,   `src/mathematikerin/machines/verdict.rs:46`,
  `src/mathematikerin/te.rs:5709/5727/5870/5878/5887/5913`,
  `src/mathematikerin/tests.rs:641`,
  `tools/measure/src/bin/hyperscanning_group_te.rs:1624/1645`. Nur eigene Hunks —
  jede Linie formatiert ihre eigenen Dateien.
- **Blockade:** kein `cargo fmt` lokal (strukturell CI-only).
- **Braucht:** die betroffenen Dateien je Linie formatieren (die sensory-Dateien
  `te.rs`/`tests.rs`/`hyperscanning_group_te.rs` sind eigene).

### Seam der Phasen-Null (benannt, nicht geparkt)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Seam-Heil (`4e3ddb13`) heilte sein Ziel nicht (folge126: -2.02 sd)
  und brach zwei Invarianten (zurückgenommen, Punkt T). Der KSG-Boden ist
  Eigenbias ~0.1 (folge128), nicht die Naht.
- **Blockade:** keine.
- **Braucht:** Blindband eines Produktions-Fixtures der Phasen-Null messen, bevor
  je ein Heal versucht wird; benannte Konstruktion: **n-Punkt-DFT-Rotation ohne
  Zero-Padding** (erhält Kreuzspektrum und Marginalien konstruktionsbedingt exakt).

### Wartend / operator-gebunden / termin
- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Benchmark

- **CI-Log-/Run-Extraktion (folge144):** die Run-Lage (list/view/log) über
  `grind-flash` — Routine-Klasse geschlossen (flash-Sieger 2026-09-16), zitiert.
- **TE-Null-Seam-Entscheidung (`council`):** Architektur, pro/max — kein
  flash-Gegenlauf (hartes Atom).
- **Punkt-11-Sweep-Bau (`grind-max`):** hartes TE-Atom (Embedding/Null), pro/max —
  kein flash-Gegenlauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/omega.rs` (clippy-Fix `Option::map`)
- `src/mathematikerin/te.rs` (`endpoint_matched`-Revert)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (frozen-tau-Sweep)
- `.github/workflows/hyperscanning-te.yml` (Sweep-Step)
- `docs/handover/handover-2026-09-22-sensory-folge144.md` (neu)
- `docs/handover/post.md` (`An ernte:` quaoar-Befund)
- `docs/zustand/external-state.md` (CI-Zeile)
- `docs/zustand/dropped-baseline.md` (Baseline nachgezogen)
- Move `handover-2026-09-21-sensory-folge143.md` → `archiv/` (eigene Linie, atomar)

Fremde uncommittete Arbeit im selben Baum
(`tools/utils/src/bin/archive_search/net.rs`, `server.rs`, `web.rs` — Marginalia-Bau
einer anderen Linie) wurde **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Der eigene Commit steht auf dem
aktuellen `origin/main` — der Push ist Fast-Forward. `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
