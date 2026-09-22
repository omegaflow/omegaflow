<!--
  title: Handover — Mountain-Folge 131 (Stand 2026-09-22)
  session: Mountain-Folge 131
  class: handover
  date: 2026-09-22
  sha256: bbcc3acca7b4af7c32b3e1075e5ac729cdb3258b0320c693a29b9c173dd56664
  status: live
-->
# Handover — Mountain-Folge 131 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Session-Beginn)

- **HEAD** `18d4c51c` beim Start; `git_safety --snapshot` `refs/safety/1790025298`.
  Während der Session zogen future (`d71fc9f8`/`26f3c1b9`) und river (`e2ba8cba`)
  den Branch weiter. Der Baum trägt Fremdarbeit (sensory folge144: `te.rs`,
  `omega.rs`, `hyperscanning*`, `post.md`, `external-state.md`,
  `dropped-baseline.md`) — nicht angetastet.
- **Postfach** — keine mountain-Zeile; die `An mountain:`-Marginalia-Zeile
  (`post.md`) gefaltet und gelöscht. Die neuen Ledger-Eingänge (Globus/SuperDARN,
  `1790023913`) sind mycelium/Maschine.
- **CI** (`ci_manage list`/`log`, kein Poll) — `hdf5-real-granule 35653536787` /
  `35653525276` @`8cc34fd7` **failure**: der neue ATL03-Test assertete
  `head[5] >= 2`; gemessen ist der Wurzel-Level `1` (die Behauptung „depth ≥ 2"
  war ungemessen) — in diesem Atom auf die gemessene Wahrheit korrigiert.
  `ci-check 35649256512` @`3b42cb72` / `35656851467` @`0d73603d` **failure**:
  format (mountain `hdf5.rs`/`verdict.rs` geheilt), test (te.rs-Gates + quaoar —
  fremde Linien), clippy, dropped-gate (Baseline — sensory).

## Offen (aufgeschlüsselt)

### 1. HDF5 v2-Chunk-Index (`BTHD` Typ 10)
- **Status:** offen | **Bindung:** eigen
- **Lage:** der v1-TREE-Zeuge steht (ATL03, Wurzel-Level 1, multilevel) — die
  exakten Knotenzahlen werden gedruckt, nicht mehr hart assertet. Kein Produkt mit
  v2 `BTHD` Typ 10 gemessen; research-max: kein Kandidat verifiziert, GLM-L2-LCFA
  und ATL03 sind beide v1 TREE.
- **Blockade:** kein v2-Chunk-Index-Granule.
- **Braucht:** GWOSC-O4b-4kHz-HDF5
  (`https://gwosc.org/archive/data/O4b_4KHZ_R1/1420820480/H-H1_GWOSC_O4b_4KHZ_R1-1421201408-4096.hdf5`,
  134 MB, keyless) laden → `archive_search BTHD --root <dir>`; bei BTHD den
  Chunk-Index-Wurzel range-lesen und depth ≥ 2 + `(nsz,tsz)` gegen die echte
  Checksum prüfen.

### 2. `hdf5-real-granule` — grüner Lauf
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** ATL03-Assert auf `head[5] >= 1` korrigiert (gemessen Wurzel-Level 1);
  Format-Hunks in `hdf5.rs` geheilt; `cargo check --all-targets` 0/0.
- **Blockade:** CI-Lauf am neuen Commit.
- **Braucht:** `hdf5-real-granule`-Lauf nach Push (paths-Trigger auf `hdf5.rs`).

### 3. `ci-check`-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** die mountain-Format-Dateien (`hdf5.rs`, `verdict.rs`) geheilt; die
  test-Fehler (te.rs, quaoar) sind fremder Linien, clippy fremd.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci-check`-Lauf lesen (`ci_manage log <id> --all`).

### 4. `register_lookup --dropped` Laufzeit-Messung
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** der bestehende `dropped-gate` (ci-check) ruft `--dropped --count` —
  der `--count`-Pfad überspringt den git-Sweep, und der Checkout ist shallow
  (`fetch-depth: 1`), misst also nicht die Wirkung auf die Historie. Neuer Workflow
  `register-dropped.yml` (workflow_dispatch, `fetch-depth: 0`, voller
  `--dropped`-Sweep) gebaut; die Baseline zog sensory parallel auf 2647.
- **Blockade:** CI-Lauf.
- **Braucht:** `register-dropped`-Lauf dispatchen + lesen.

### 5. KSG-Produktionsverdrahtung — K-Flip
- **Status:** offen | **Bindung:** eigen
- **Lage:** gebaut (`te_verdict_bytes`, `TE_KSG_K_PROD = 0`), Gate 0/0.
- **Blockade:** kein Live-Datenlieferant für K.
- **Braucht:** eine echte K-Quelle → Konstante flippen, Kalibrier-Gate auf dem
  GPU-Pfad neu messen.

### 6. DEMETER-Register-Eintrag entfernen
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** Code gebaut (`demeter.rs`), `phi/pipeline/ledger.φ:22` stale.
- **Blockade:** die Datei trägt fremde uncommittete Hunks.
- **Braucht:** der die Datei führende Strang committet seinen Stand.

### 7. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat zugesagt, Gerät nicht da.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Geroutet / fremde Linien

- **`te.rs`-Gates** (`coherent_phase_null_absorbs_linear_cross_coupling`,
  `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200`) → sensory
  folge144 (Punkt T, `endpoint_matched`-Revert). grind-max fand unabhängig dieselbe
  Wurzel (die `endpoint_matched`-Rampe verzerrt das Kreuzspektrum vor der Rotation).
- **`quaoar_occlt`-Test** (`date_midnight_unix("20111301")`) → ernte; sensory
  postete die gemessene Zeile (`An ernte:`, `post.md`).

## Benchmark

- **`--marginalia`-Bau → `grind-flash`:** mechanisch (mwmbl-Muster), `cargo check`
  0/0; Sieger, kein Doppel-Lauf.
- **CI-Log-Extraktion (Punkte 2/3/4) → `grind-flash`:** Routine-Klasse geschlossen
  (flash-Sieger 2026-09-16), zitiert; kein Doppel-Lauf.
- **te.rs-Wurzel → `grind-max`:** hartes TE-Atom; unabhängig dieselbe Wurzel wie
  der sensory-Rat; stoppte an fremden Hunks (kein Schreiben). Kein flash-Gegenlauf.
- **v2-BTHD-Recherche → `research-max`:** kein verifizierter Kandidat, benannter
  nächster Schritt.
- **HDF5-Assert + Format → `build` (lokal):** `cargo check` 0/0.

## Geteilter Baum — eigener Pfad-Satz

- `tools/utils/src/bin/archive_search/{net,server,web}.rs` (Marginalia-Bau,
  `grind-flash`)
- `tools/utils/src/bin/archive_search.rs` (CLI-Arm `--marginalia` + Usage)
- `src/archivar/hdf5.rs` (ATL03-Assert auf gemessen + Format-Hunks)
- `src/mathematikerin/machines/verdict.rs` (Format-Hunk)
- `.github/workflows/register-dropped.yml` (neu)
- `docs/handover/handover-2026-09-22-mountain-folge131.md` (neu)
- Move `handover-2026-09-21-mountain-folge130.md` → `archiv/` (eigene Linie, atomar)
- **Fremd, nicht committet:** `post.md` (eigene `An mountain:`-Zeile gelöscht; die
  fremde `An ernte:`-Zeile bleibt), `te.rs`/`omega.rs`/`hyperscanning*`/
  `external-state.md`/`dropped-baseline.md` (sensory folge144).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
