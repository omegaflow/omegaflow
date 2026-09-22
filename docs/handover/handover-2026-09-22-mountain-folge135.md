<!--
  title: Handover — Mountain-Folge 135 (Stand 2026-09-22)
  session: Mountain-Folge 135
  class: handover
  date: 2026-09-22
  sha256: b0d2f12ecc140c1b136690b7f89578504e74a6eb5fa4587a90487bc075a937ca
  status: live
-->
# Handover — Mountain-Folge 135 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Session-Beginn)

- **HEAD** `e12219b759154e2279f01de7acbeca91d90f81a0` — fortgeschritten seit folge134
  (`d16f2db0f`, fremde Commits).
- **`git_safety --snapshot`:** Arbeitsbaum != HEAD (eigene + fremde uncommittete Hunks);
  Snapshot gesetzt.
- **Postfach** — kein offener `An mountain:`-Eingang. Neuester Ledger-Eingang
  `1790086963` (Tavily „Welcome to Tavily" — Maschine, keine Aktion). Davor
  `1790069422` (GitHub Support: unreferenzierte Objekte getilgt, Ticket 4761801).
- **CI** — `ci_manage list` 2026-09-22: `ci-check 35746049660` failure (15:15Z),
  `hdf5-real-granule 35743680043`/`35743679859` failure (14:55Z),
  `superdarn-rawacf-cdn 35746075039` failure (fremd), `health-check 35752730232`/
  `allwise-cdn 35745349130` in_progress, `hyperscanning-te 35743984631` pending.

## Offen (aufgeschlüsselt)

### 1. HDF5 v2 `BTHD` — Real-Granule-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** Der Fehllauf war ein **über-breiter Test-Assert**: `hdf5.rs:4310`
  verlangte `BTHD` für **jedes** `Chunked`-Objekt, der DLS-Zeuge `p45-2194.nxs`
  trägt aber v1-`TREE`- **und** v2-`BTHD`-Chunk-Indizes (gemessen: 6 × `BTHD`
  Typ 10, `TREE` zuerst erreicht). Fix: `UNDEF`-Index und Nicht-`BTHD`-Kopf
  überspringen statt asserten; die Zahl der v2-Wurzeln bleibt als
  `assert_eq!(v2_roots, 6)` erhalten (`cargo check` 0/0). Workflow-URL/sha256
  waren korrekt.
- **Blockade:** CI-Lauf am neuen Commit.
- **Braucht:** `ci_manage log <id>` des nächsten `hdf5-real-granule`-Laufs (Push auf
  `src/archivar/hdf5.rs` triggert ihn).

### 2. `ci-check`-Verifikation
- **Status:** linie:sensory | **Bindung:** linie:sensory
- **Lage:** `ci-check 35746049660` failed in drei Jobs: (a) clippy
  `needless_range_loop` `te.rs:3350` (Test `exact_fft_matches_direct_dft_non_pow2`,
  sensory folge147) — der Fix liegt **bereits uncommitted** im Baum; (b) FPR-Gate
  `te.rs:4470` `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200`
  8.52 % > 8 % (statistisches Kalibrier-Gate, flaky); (c) dropped-gate delta 69
  (Baseline 2517, `docs/zustand/dropped-baseline.md`). Alle drei sind
  sensory-Eigentum; `src/mathematikerin/te.rs` ist im Baum fremd uncommittet.
  Der ältere Lauf `35743679830` scheiterte an `path_reference_scan` — am HEAD
  geheilt.
- **Blockade:** fremde uncommittete `te.rs`-Arbeit; Baseline-Bump.
- **Braucht:** sensory committet den `te.rs`-Clippy-Fix + Baseline-Bump;
  FPR-Gate-Bewertung durch sensory. (post.md `An sensory:`)

### 3. KSG-Verdrahtung — K-Flip
- **Status:** wartend | **Bindung:** eigen
- **Lage:** gebaut — `TE_KSG_K_PROD = 0` (`machines/verdict.rs:12`),
  `te_verdict_bytes` (`:8`), Gate `gate_te_verdict_bytes_follows_k` (`:48`);
  Konsumenten real (`omega.rs`, `matrix.rs`, `solar.rs`).
- **Blockade:** kein Live-Datenlieferant für K.
- **Braucht:** echte K-Quelle → Konstante flippen, Kalibrier-Gate GPU-neu messen.

### 4. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat **zwei** Ox64 SBCs versandt (Ledger `1790046330`, Tracking
  `LZ473049629CN`); physisch noch nicht angekommen.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Geroutet / fremde Linien

- **`src/mathematikerin/te.rs`** (clippy-Fix) + **`phi/{sources,witnesses}.φ`**
  + **`tools/harvest/src/bin/solar_system_open_data_compiler.rs`** (untracked) —
  fremde uncommittete Arbeit im Baum, nicht angetastet.
- **`docs/zustand/external-state.md:35`** (Such-API-Kandidaten): der Schritt
  „Bau `--tavily`/`--exa`/`--linkup` → `An mountain:`" ist stale — die Modi sind
  gebaut (`archive_search.rs:280–282`, `net.rs:1183/1309/1594`). → mycelium
  (post.md `An mycelium:`).
- **`dropped-gate`-Baseline** → sensory (siehe Punkt 2).

## Benchmark

- **HDF5-Real-Granule-Diagnose → `grind-flash`:** in **einem** Lauf gelöst
  (Log gelesen, über-breiten Assert gefunden, Fix + `cargo check` 0/0). flash-first,
  Sieger — kein Gegenlauf.
- **`ci-check`-Diagnose → `grind-flash`:** in einem Lauf gelöst (drei Failures
  isoliert, Eigentümerschaft gemessen). flash-first, Sieger — kein Gegenlauf.
- **Workspace-Bruch `omegaflow-measure` → `grind-flash`:** in einem Lauf gelöst
  (zwei verwaiste Referenzen gefunden statt einer, entfernt, `cargo check 0/0`).
  flash-first, Sieger — kein Gegenlauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-mountain-folge135.md` (neu)
- Move `handover-2026-09-22-mountain-folge134.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (eigener Hunk: `An sensory:` + `An mycelium:`)
- `docs/zustand/external-state.md` (Postfach-Eintrag)
- `src/archivar/hdf5.rs` (Test-Assert-Fix)
- `tools/measure/src/bin/lsst_anomaly_probe.rs` (verwaisten Test entfernt)
- `tools/measure/src/bin/disappearance_probe.rs` (verwaisten Test + Helfer entfernt)
- **Fremd, nicht committet:** `src/mathematikerin/te.rs`, `phi/sources.φ`,
  `phi/witnesses.φ`, `tools/harvest/src/bin/solar_system_open_data_compiler.rs`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
