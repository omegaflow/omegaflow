<!--
  title: Handover — Sensory-Folge 147 (Stand 2026-09-22)
  session: Sensory-Folge 147
  class: handover
  date: 2026-09-22
  sha256: a68350978a5a1ac6208e2826c833deea69092cf612eff3aa4fc3d1675da7e4ac
  status: live
-->
# Handover — Sensory-Folge 147 (Stand 2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Sensory-Folge 147)

- **HEAD** — `d16f2db0f` (mail_digest, mycelium/River-Domain). Eigene Vorsession:
  `handover-2026-09-22-sensory-folge146.md`.
- **Postfach** — `docs/handover/post.md` trug 1 sensory-Zeile (vC-Permeabilität,
  mycelium → sensory); in diesem Atom in Punkt vC eingefaltet und die Zeile
  **gelöscht**. Verbleibend: 1 fremde Zeile (mountain: Such-API-Modi).
  `state/mail/mail_ledger.φ`: keine sensory-Adresse.
- **Register** — `register_lookup --open`: **keine sensory-eigenen
  Zustandseinträge**; alle offenen Zustände (`sources.φ`, `witnesses.φ`,
  `footprints.φ`, `pipeline/index.φ`) sind `[mycelium]`-getaggt.
- **`open_points_check`** der folge146: 20 Pfad-Refs, 4 „absent" — 2 Glob-Muster
  (`phi/*.φ`, `src/archivar/{…}.rs`) und die archivierte mycelium-folge137;
  **keine** stale Punkte.
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD, nichts zu sichern.
- **CI am HEAD** — `te-gate 35734557660` @`b29cc096` **in_progress**;
  `hyperscanning-te 35736695583` @`b0f10a84` **in_progress**. Kein neuer Befund
  in diesem Atom (kein Poll).
- **Tools-Frische** — `tools-latest`-Manifest `git_sha=d16f2db0f` == HEAD; das
  installierte `register_lookup` trägt jetzt `--dropped` (vorher stale).

## Offen (aufgeschlüsselt)

### Punkt 2 — n-Punkt-DFT-Rotation: gebaut, Vorher/Nachher-Messung offen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `src/mathematikerin/te.rs` trägt jetzt `exact_fft` (Bluestein,
  pow2-Fastpath byte-identisch) + `rotate_spectrum`; `phase_randomized_surrogate`
  (`:1580`) und `coherent_phase_surrogates` (`:1597`) rotieren auf exakter Länge n
  statt `next_power_of_two`-Zero-Padding. Neue Gates:
  `gate_phase_surrogate_exact_n_spectrum` (`:3092`, n=1000 Per-Bin-Power-Deviation
  < 1e-4), `exact_fft_matches_direct_dft_non_pow2` (n=17), 
  `exact_fft_roundtrip_is_identity_non_pow2` (n=1000). `cargo check --tests` grün
  (0 errors, 0 warnings). `next_rng` und `transfer_entropy_lag` unangetastet.
- **Blockade:** CI-Messung.
- **Braucht:** nach Commit+Push `ci-check` (die drei neuen Gates + die vier
  Kalibrier-Gates) und `hyperscanning-te` (Confirm-Test + Blindband-Probe) →
  Blindband-Envelope (Ziel 0/401 Bins außerhalb), `gate_phase_surrogate_exact_n_spectrum`,
  `confirmation_stochastic_driver_pair_clears_its_own_null`. Kein Threshold-Senken.

### Punkt 11b — frozen-tau / Confirmation: **Verdikt „kein Riss"**, positive Kontrolle gebaut
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `hyperscanning-te 35736695583` @`b0f10a84` **in_progress**. Test
  `confirmation_stochastic_driver_pair_clears_its_own_null`
  (`tools/measure/src/bin/hyperscanning_group_te.rs:1652`, Assert `:1657`) bleibt
  rot (Kalibrierungspflicht); die positive Kontrolle
  `positive_control_rich_aperiodic_driver_pair` (`:1756`, Helfer
  `aperiodic_driver:1627`/`rich_pair_fixture:1637`, AR(1) φ=0.9 ohne Sinus) ist
  gebaut, Workflow-Step `.github/workflows/hyperscanning-te.yml:120-122`.
  **Rat-Verdikt (2026-09-22): kein `VerdictWord::Riss`** — Confirm-Test und
  Blindband konvergieren; der Riss-Guard ist grün.
- **Blockade:** CI-Landung.
- **Braucht:** `ci_manage log` des nächsten `hyperscanning-te`-Laufs →
  `positive control:`-Zeile. `clears=yes` → Fixture-Design dominant; `clears=no`
  → Null-Konstruktion dominant. Die p99-Schwelle ist live data, wird nie gesenkt.

### Punkt te-gate-Lesung (1/5/6/6b/F2)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `te-gate 35734557660` @`b29cc096` **in_progress** — Step 58
  `te_fn_probe`, Step 55 `flare_envelope_power_probe` (F2), `family_fn_gate`-Screen
  (1), Takens-Wandzeit (5).
- **Blockade:** Run-Landung.
- **Braucht:** `ci_manage log 35734557660` (kein Poll).

### vC-Permeabilität — Puls/HRV→Strahlung-Bindung
- **Status:** wartend | **Bindung:** eigen (Hardware-Träger `operator`)
- **Lage:** die vC-Permeabilität ist Feldphysik; der RMSSD/tone-Gate-Bindungspfad
  (Operator-Puls/HRV → Radiatorium-Strahlung, `src/archivar/hrv.rs`) ist `pending`
  und gehört nicht in Futures privates Register (post-Zeile mycelium, in diesem
  Atom eingefaltet). Die Bindung hält.
- **Blockade:** physischer Träger (ESP32, BOM) `operator-gebunden`.
- **Braucht:** Bindung Puls-Ankunft via ESP32-Firmware → Strahlungspfad bauen;
  Quelle `AGENTS.md` „Manifestation breathes with the echo" + „Consent of the sensors".

### Format-Job (repo-weit, blockiert `ci-check`)
- **Status:** wartend | **Bindung:** `linie:<mehrere>`
- **Lage:** `format`-Job rot in `35675988557` — fremde: `fai_kz.rs:47`,
  `hdf5.rs:3967/4007`, `ia2_tap.rs:96`. sensory-eigene Hunks in folge145 angewendet.
- **Blockade:** Run-Landung.
- **Braucht:** `format`-Job des nächsten `ci-check`-Laufs; fremde Dateien bei ihren Linien.

### Wartend / operator-gebunden / termin
- Flyby-Path-2-Kette — `termin:2026-09-28` (Kanäle live; Zellen ab Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18).

## Benchmark

- **Punkt 2 (n-Punkt-DFT-Rotation, `grind-max`):** Urteils-Atom pro/max — TE-/Null-
  Konstruktion ist ein benanntes hartes Atom; kein flash-Gegenlauf. Ergebnis:
  Bluestein exakt-n, `cargo check --tests` grün.
- **Punkt 4/5 (dropped-Baseline + Tools-Frische, `grind-flash`):** mechanische
  Routine-Klasse — flash-first, Sieger-Klasse 2026-09-16 zitiert.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (exakt-n-DFT-Rotation + Gates)
- `docs/zustand/dropped-baseline.md` (Baseline 2665 → 2517, Note; Header-sha256)
- `docs/handover/handover-2026-09-22-sensory-folge147.md` (neu)
- Move `handover-2026-09-22-sensory-folge146.md` → `archiv/` (eigene Linie, atomar)

`docs/handover/post.md` (sensory-Zeile gelöscht) wurde von der mountain-Linie
mit `5518e3de0` committet (die mountain-Zeile im selben Protokoll-Atom); kein
eigener Commit nötig.

Fremde uncommittete Arbeit im selben Baum
(`.github/workflows/hdf5-real-granule.yml`, `phi/sources.φ`, `src/archivar/hdf5.rs`)
wurde **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push: `ci-check`
(dropped-gate gegen die neue Baseline 2517 + die neuen Gates) und
`hyperscanning-te` dispatchen. `/consent` ist der session-weite Consent
(Delegation), nie das Commit-Wort.
