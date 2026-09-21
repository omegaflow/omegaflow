<!--
  title: Handover — Mountain-Folge 130 (Stand 2026-09-21)
  session: Mountain-Folge 130
  class: handover
  date: 2026-09-21
  sha256: 642eb2f641f22e805461c9d947c65823d03bb68d7a9d34ddfc807ec1e26af6b8
  status: live
-->
# Handover — Mountain-Folge 130 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `3b42cb72` == `origin/main` beim Start; `git_safety --snapshot`
  `refs/safety/1790022301`. Der Baum trägt Fremdarbeit (ernte: Move
  `handover-…-ernte-folge135` → `archiv/`, neues `handover-…-ernte-folge136.md`,
  `phi/blocked_sources.φ`, `phi/pipeline/{index,ledger,catalog/korpora_heim}.φ`,
  `phi/sources.φ`) — nicht angetastet.
- **Postfach** — letzter `mail_ledger.φ`-Eingang `1790013970` (Cloudflare-Login-Code,
  Maschine, informativ; laut `external-state.md`); kein handlungsbedürftiger
  mountain-Fall. **`post.md`**: keine `An mountain:`-Zeile.
- **CI** — Watchdog-Snapshot: `hdf5-real-granule` `35649134961` @`3b42cb72`
  **failure** — `flash_lat layout: Some(Chunked { btree: 181836, chunk_dims:
  [256], elem_size: 4 })`; der folge129-Assert `!Chunked` war selbst ungemessen
  (in diesem Atom auf die gemessene Wahrheit korrigiert); `ci-check`
  `35649256512` in_progress. Kein Poll.
- **`register_lookup --open`** — ein mountain-getaggter Register-Punkt:
  DEMETER parser-gap (`phi/pipeline/ledger.φ:22`) → gefaltet und in diesem Atom
  gebaut. **`open_points_check`** gegen folge129: 0 absent.

## Offen (aufgeschlüsselt)

### 1. HDF5 v2-Chunk-Index — kein `BTHD` Typ-10-Zeuge
- **Status:** offen | **Bindung:** eigen
- **Lage:** der v1-Multilevel-Pfad hat jetzt einen real-granule-Zeugen: Test
  `real_granule_atl03_v1_chunk_index_materializes_multilevel`
  (`src/archivar/hdf5.rs`, `#[ignore]`, im `hdf5-real-granule`-Workflow mit
  EDL-Token + `ATL03_GRANULE_URL`), Range-Reads, 4-MiB-Präfix; assertet TREE
  (nicht BTHD), depth ≥ 2, 682 TREE-Knoten (38 level-1 / 640 level-0),
  Materialisierung + Element-Decode. Kein Produkt mit v2 `BTHD` Typ 10 gemessen.
- **Blockade:** kein v2-Chunk-Index-Granule.
- **Braucht:** ein Granule eines neueren HDF5-Writers mit `BTHD` Typ 10, depth ≥ 2
  → die depth-2-Kandidaten `(nsz, tsz)` in `btree_records` gegen die echte
  Checksum prüfen; nur bei Fehlschlag Fix.

### 2. `hdf5-real-granule` — grüner Lauf
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** `flash_lat`-Assert auf die gemessene Wahrheit korrigiert:
  `Chunked { btree: 181836, chunk_dims: [256], elem_size: 4 }` (CI
  `35649134961`); der neue ATL03-Test ist im selben Workflow.
- **Blockade:** Lauf auf dem neuen Commit steht aus.
- **Braucht:** `hdf5-real-granule`-Lauf nach Push (paths-Trigger auf `hdf5.rs`).

### 3. `flush_port_block` — CI-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** Fix committet (`b2f3fa51`/`da1213f4`); `ci-check` `35649256512`
  in_progress.
- **Blockade:** kein abgeschlossener grüner `ci-check`.
- **Braucht:** `ci-check`-Lauf lesen (`ci_manage log <id> --all`).

### 4. Rote `te.rs`-lib-Tests
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** `coherent_phase_null_absorbs_linear_cross_coupling` +
  `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200` @`8d6553fe` rot;
  am neuen HEAD ungemessen.
- **Blockade:** `ci-check`-Lauf ausstehend.
- **Braucht:** `ci-check`-Lauf lesen; grün → erledigt, sonst Wurzel + Fix/Gate.

### 5. KSG-Produktionsverdrahtung — K-Flip
- **Status:** offen | **Bindung:** eigen
- **Lage:** gebaut — `te_verdict_bytes(k)` (`src/mathematikerin/machines/verdict.rs`:
  0 → 288 B, k > 0 → 384 B), `TE_KSG_K_PROD = 0`; die Consumer (`omega.rs`,
  `machines/matrix.rs`, `machines/solar.rs`, `tests.rs`) setzen
  `params.w = TE_KSG_K_PROD` und dimensionieren über `te_verdict_bytes`;
  Contract-Gate `gate_te_verdict_bytes_follows_k`; `cargo check --all-targets` 0/0.
- **Blockade:** kein Live-Datenlieferant für K.
- **Braucht:** eine echte K-Quelle → `TE_KSG_K_PROD` flippen (nur die Konstante),
  dann das Kalibrier-Gate auf dem GPU-KSG-Pfad neu messen (falls der Null-Control
  dort läuft).

### 6. DEMETER-Register-Eintrag entfernen
- **Status:** blockiert | **Bindung:** eigen
- **Lage:** der Code ist gebaut (`src/archivar/demeter.rs`: das Marker-Gate
  akzeptiert `ISL SURVEY` oder die 9-Byte-`ISL BURST`-Präfix, Test
  `parses_isl_burst_block`); der Register-Eintrag `phi/pipeline/ledger.φ:22`
  (parser-gap) ist damit stale.
- **Blockade:** die Datei trägt fremde uncommittete Hunks — ein pfad-begrenzter
  Commit würde fremde Arbeit mitschreiben.
- **Braucht:** der die Datei führende Strang committet seinen Stand, dann die stale
  Zeile entfernen (oder ein späteres Atom bei sauberer Datei).

### 7. `register_lookup --dropped`-Sweep — Laufzeit-Messung
- **Status:** offen | **Bindung:** eigen
- **Lage:** gebaut (`tools/register/src/bin/register_lookup.rs`: `git log --all
  --grep`/`-S`-Fallback, `resolution_status`, Test); `cargo check --all-targets`
  0/0. Die Wirkung auf die reale Historie ist ungemessen (Binary nicht gelaufen).
- **Blockade:** lokaler Lauf verboten (schwere Last → CI).
- **Braucht:** ein `register_lookup --dropped`-Lauf, der den Effekt misst.

### 8. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat zugesagt, Gerät nicht da; Doku
  `docs/specs/mantis-shrimp-build.md §Zweitknoten`.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten; dann Bring-up + Kopplung messen.

## Benchmark

- Punkt KSG-Verdrahtung → `grind-max`: hartes Atom (Urteil + Schreiben), gebaut,
  `cargo check` 0/0; kein Doppel-Lauf.
- Punkt DEMETER → `grind-flash`: mechanisch, korrekt, 0/0; Sieger, kein Doppel-Lauf.
- Punkt `register_lookup --dropped` → `grind-flash`: gebaut, 0/0; Laufzeit offen.
- Punkt HDF5-v1-Test → `grind-pro`: gebaut, 0/0.
- Rat (AGENTS.md-Satz + Abschluss): alle fünf Stimmen „ja, in diesem Atom"; die
  Satz-Korrektur ist umgesetzt. Kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `AGENTS.md` (KSG-Verdrahtungs-Satz, Ratswort),
  `docs/handover/post.md` (Route Punkt 9 → sensory), `src/archivar/demeter.rs`,
  `src/archivar/hdf5.rs` (flash_lat-Assert + v1-Chunk-Test),
  `.github/workflows/hdf5-real-granule.yml` (ATL03-Step),
  `src/mathematikerin/{machines/verdict.rs, machines/matrix.rs, machines/solar.rs,
  mod.rs, omega.rs, te.rs, tests.rs}`,
  `tools/register/src/bin/register_lookup.rs`, neues Handover
  `handover-2026-09-21-mountain-folge130.md`, Move
  `handover-2026-09-21-mountain-folge129.md` → `archiv/`.
- **Fremd (nicht angetastet):** der ernte-Move, `handover-…-ernte-folge136.md`,
  `phi/blocked_sources.φ`, `phi/pipeline/{index,ledger,catalog/korpora_heim}.φ`,
  `phi/sources.φ`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt. Nach Push:
  `hdf5-real-granule` läuft durch den `paths`-Trigger auf `src/archivar/hdf5.rs`
  von selbst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
