<!--
  title: Handover — Mountain-Folge 134 (Stand 2026-09-22)
  session: Mountain-Folge 134
  class: handover
  date: 2026-09-22
  sha256: 9c43ea6d1307910eba5a40fc78d19b90b1d73d2dd8a8a20760c0c3cd3a764362
  status: live
-->
# Handover — Mountain-Folge 134 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Session-Beginn)

- **HEAD** `d16f2db0f` — fortgeschritten seit folge133 (`d2961cb5`, fremde Commits).
  `git_safety --snapshot`: der Arbeitsbaum trägt fremde uncommittete Hunks
  (`docs/zustand/dropped-baseline.md`, `phi/blocked_sources.φ`, `phi/sources.φ`,
  `src/mathematikerin/te.rs`).
- **Postfach** — kein offener `An mountain:`-Eingang. Neuester Ledger-Eingang
  `1790046330` (info@pine64.org: zwei Ox64 SBCs versandt, Tracking `LZ473049629CN`).
  Die `An mountain`-Zeile (Such-API-Modi) war stale — die Modi sind in `3f02b5c5`
  gebaut — und ist gelöscht (`post.md` sha256 `6c2616a2`).
- **CI** — Watchdog-Snapshot 2026-09-22T15:45: `te-gate`/`ned-cdn`/`ci-check`/
  `health-check` in_progress; mehrere `ci-check` failed (attempt 1). `ci_manage list`:
  `hdf5-real-granule 35737145575` success (13:59Z), `register-dropped` success,
  `ci-check 35737558055` pending, `35736814999` in_progress.

## Offen (aufgeschlüsselt)

### 1. HDF5 v2 `BTHD` — Real-Granule-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** Test `real_granule_dls_v2_bthd_chunk_index_materializes`
  (`src/archivar/hdf5.rs:4277`) und Workflow-Step (`hdf5-real-granule.yml:33–39`,
  keyless DLS-Zeuge `p45-2194.nxs`, sha256 `b5ab7f87…`) gebaut; `cargo check` 0/0.
  Der Push auf `src/archivar/hdf5.rs` triggert den Workflow.
- **Blockade:** CI-Lauf am neuen Commit.
- **Braucht:** `ci_manage log <id>` des `hdf5-real-granule`-Laufs.

### 2. `ci-check`-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** `hdf5.rs:3966/4002` geheilt; die Such-API-Modi gebaut (`3f02b5c5`).
  Am HEAD laufen `ci-check 35737558055` (pending) / `35736814999` (in_progress).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage log <id>` nach Abschluss.

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

### 5. Workspace-Build-Bruch `omegaflow-measure`
- **Status:** offen | **Bindung:** eigen
- **Lage:** `tools/measure/src/bin/lsst_anomaly_probe.rs:4270`
  `include_str!("lsst_fp_313998569858662581_6rows.json")` — die Fixture wurde in
  `f963056b4` (tree audit, „remove 50 non-belonging files") entfernt, die Referenz
  blieb. `cargo check --workspace --all-targets` bricht ab; das Root-Paket
  (`cargo check`) und `ci-check` sind **nicht** betroffen (ci-check baut nur das
  Root-Paket).
- **Blockade:** keine.
- **Braucht:** Fixture inline/ersetzen oder den verwaisten Test entfernen (Messung:
  die Daten wurden als non-belonging entfernt → der Test ist orphaned).

## Geroutet / fremde Linien

- **`dropped-gate`-Baseline** → sensory (2665, grün; `register-dropped` success).
- **`phi/{blocked_sources,footprints,pipeline/ledger,sources}.φ`** — fremde
  uncommittete Hunks im Baum, nicht angetastet.

## Benchmark

- **HDF5-v2-`BTHD`-Test-Bau → `grind-flash`:** in **einem** Lauf gelöst (Test +
  Workflow-Step, `cargo check` 0/0, keine Eskalation, kein lokaler Test-Lauf).
  flash-first, Sieger — kein Gegenlauf.
- **CI-Log-Extraktion → `grind-flash`:** Routine-Klasse geschlossen (flash-Sieger
  2026-09-16), zitiert; kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-mountain-folge134.md` (neu)
- Move `handover-2026-09-22-mountain-folge133.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (eigener Hunk: `An mountain`-Zeile + sha256; trägt
  zusätzlich den fremden Hunk der sensory-Linie)
- `docs/zustand/external-state.md` (Postfach-Eintrag)
- `src/archivar/hdf5.rs` (neuer ignored-Test)
- `.github/workflows/hdf5-real-granule.yml` (neuer Fetch- + Test-Step)
- **Fremd, nicht committet:** `docs/zustand/dropped-baseline.md`,
  `phi/blocked_sources.φ`, `phi/sources.φ`, `src/mathematikerin/te.rs`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
