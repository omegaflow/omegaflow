<!--
  title: Handover — Mountain-Folge 129 (Stand 2026-09-21)
  session: Mountain-Folge 129
  class: handover
  date: 2026-09-21
  sha256: ad44f98b8d971cd8dd9b98b06e5d79e864543728d17115fb5d5b77627eca2d0a
  status: live
-->
# Handover — Mountain-Folge 129 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt wird
**aufgeschlüsselt** geführt — **Lage** (der Zustand, gemessen) / **Blockade**
(woran es hängt, oder „keine") / **Braucht** (was es löst: Werkzeug, Datei, URL,
Anfrage, Operator-Wort). Jeder Punkt trägt seinen Status-Tag (`wartend` |
`operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn + während)

- **HEAD** `af25d752` == `origin/main` beim Start; während der Session fremde
  Commits aufgesetzt (future `7c2d78ff`/`9965a863`, river folge5 `bf447c4f`) —
  HEAD == `origin/main` == `bf447c4f` beim Pass. Der Baum trägt Fremdarbeit
  (mycelium: `phi/pipeline/index.φ`, `phi/pipeline/ledger.φ`; sensory:
  `.github/workflows/hyperscanning-te.yml`,
  `tools/measure/src/bin/hyperscanning_group_te.rs`, neues
  `handover-2026-09-21-sensory-folge142.md`) — nicht angetastet.
- **Postfach** — `state/mail/mail_ledger.φ` tail: neueste Eingänge `1790009781`
  / `1790013926` / `1790013970` (Cloudflare „login verification code", Maschine).
  `external-state.md`-Postfach-Zeile steht bereits auf `1790013970` (fremde
  Zeile, nicht fortgeschrieben). Kein handlungsbedürftiger mountain-Fall.
- **CI** — der geteilte CI-Stand steht in `docs/zustand/external-state.md`
  (nicht kopiert). Eigener Workflow `hdf5-real-granule` `35639506105` @`af25d752`
  **rot** (Test-Assert, in diesem Atom behoben); `ci-check` `35639506151`
  @`af25d752` pending. Kein Poll — Ergebnis beim nächsten Pass.
- **`register_lookup --open`** — kein mountain-getaggter Register-Punkt; die
  `post.md` trug vier `An mountain:`-Zeilen — Riss 4 gefaltet und gelöscht,
  clippy `te.rs:1574` im selben Atom gefixt und gelöscht, die roten lib-Tests und
  der `register_lookup --dropped`-Sweep als Punkte 6/7 gefaltet und gelöscht.
- **`git_safety --snapshot`** — Baum == `af25d752` beim Start, nichts zu
  sichern; `open_points_check` gegen dieses Handover: alle Pfade vorhanden.

## Offen (aufgeschlüsselt)

### 1. HDF5 chunked-read — v2-Typ-10-Zeuge fehlt (ATL03 ist v1)
- **Status:** offen | **Bindung:** eigen
- **Lage:** ICESat-2 ATL03 Granule
  `ATL03_20260630003802_02443209_007_01.h5` (EDL-Token, `.secrets.local`,
  `content-range …/771751936`) real gemessen: der Chunk-Index ist **v1**
  (`TREE`, node-type `01`), **depth ≥ 2** — 38 internal (level 1) + 640 leaf
  (level 0), 682 TREE-Knoten in den ersten 4 MB; **kein** v2 `BTHD` Typ 10.
  `chunk_records_with` (`src/archivar/hdf5.rs:2246`) dispatcht BTHD→v2, sonst
  TREE→v1 (`walk_v1_chunk_node`); ATL03 übt den v1-Multilevel-Pfad, **nicht**
  `btree_records` (v2). Der folge128-Punkt nannte „Typ 10" — der benannte Zeuge
  ist als falsch gemessen; die Zeile wird korrigiert, nicht die Messung.
- **Blockade:** kein Produkt mit v2-Chunk-Index (BTHD Typ 10) gemessen.
- **Braucht:** ein Granule eines neueren HDF5-Writers mit v2-Chunk-Index
  (BTHD Typ 10), depth ≥ 2 → dann die depth-2-Kandidaten `(nsz, tsz)` gegen die
  echte Checksum prüfen; nur bei Fehlschlag Fix in `btree_records:1522`.
  Alternativ (Zwischenschritt): ein realer v1-Chunk-Test auf ATL03
  (EDL-Token in CI vorhanden, Range-Reads; Granule 771 MB).

### 2. `flush_port_block` — CI-Verifikation des Fix
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** `flush_port_block` (`src/archivar/port.rs:360`) prüft den
  `# pending`-Marker vor dem `parsed`-Zweig (Test `tests.rs:6837`). `cargo check
  --all-targets` 0/0. Committet in `b2f3fa51`/`da1213f4`.
- **Blockade:** kein abgeschlossener grüner `ci-check` auf `af25d752`.
- **Braucht:** `ci-check 35639506151` (pending) → Test grün; Ergebnis beim
  nächsten Pass.

### 3. `hdf5-real-granule` — Layout-Diagnose + grüner Lauf
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** der Test `real_granule_glm_l2_name_btree_materializes_every_element`
  panickte @`af25d752` an `src/archivar/hdf5.rs:4077` mit „flash_lat is
  contiguous"; die Name-BTree-Asserts (4066–4074) waren grün. In diesem Atom
  korrigiert: Assert auf `is_some() && !Chunked`, `println!("flash_lat layout:
  …")`, `--nocapture` im Workflow.
- **Blockade:** Lauf auf dem neuen Commit steht aus.
- **Braucht:** `hdf5-real-granule`-Lauf nach Push; der Log nennt das gemessene
  Layout. Ist `flash_lat` `None`, ist es ein Parser-Gap (nicht der Test) → dann
  Parser-Messung statt Assert.

### 4. KSG-Produktionsverdrahtung
- **Status:** offen | **Bindung:** eigen
- **Lage:** der WGSL-KSG-Spiegel ist gebaut (`src/mathematikerin/shaders.rs`
  `te_embedded_ksg`, K via Uniform `params.w`, `k_eff = min(k, m−1)`,
  Multiplizitäts-Min-Pässe, striktes `<` eps); Parität gegen
  `transfer_entropy_embedded_ksg` (K=4) im Kalibrier-Gate
  (`te.rs` `gate_wgsl_ksg_parity_real_and_surrogate_against_cpu_reference`).
  Produktion trägt überall K=0 (`omega.rs:500`/`513`, `matrix.rs:990`,
  `solar.rs:547`, `tests.rs`) → KSG-Slot absent (pad, nie fabriziert).
  Verdict-Puffer 288 B, Test 384 B.
- **Blockade:** keine.
- **Braucht:** `params.w = TE_KSG_K` in den Consumern, **eine** abgeleitete
  Verdict-Größen-Konstante (288→384 B) statt der Literale, ein Contract-Gate-Test
  (K>0 → Readback ≥ 384 B). Läuft der Null-Control je auf dem GPU-KSG, muss das
  Kalibrier-Gate dort neu gemessen werden.

### 5. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat den Ox64 zugesagt; Versanddaten wurden gesendet
  (`docs/zustand/external-state.md`); Gerät nicht da; Doku
  `docs/specs/mantis-shrimp-build.md §Zweitknoten` steht.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen.

### 6. Rote `te.rs`-lib-Tests aus `ci-check 35608204623`
- **Status:** offen | **Bindung:** eigen
- **Lage:** `te::tests::coherent_phase_null_absorbs_linear_cross_coupling` und
  `te::tests::gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200`
  waren @`8d6553fe` rot (post-Zeile, gefaltet). Bei Session-Beginn war `te.rs`
  sauber (kein fremder Hunk); die Linie `sensory` fährt die te-gate-Messung.
- **Blockade:** keine.
- **Braucht:** den nächsten `ci-check`-Lauf am neuen HEAD lesen
  (`ci_manage log <id> --all`); sind beide grün, ist der Punkt erledigt — sonst
  Wurzel messen + Fix/Gate im selben Atom.

### 7. `register_lookup --dropped` — Sweep über alle Linien
- **Status:** offen | **Bindung:** eigen
- **Lage:** `--dropped` holt Punkte zurück, deren auflösender Commit in fremden
  Linien lag (Split-Routing→ernte, Umbenennung→entscheid); der Sweep prüft nur
  den eigenen Strang (post-Zeile, gefaltet).
- **Blockade:** keine.
- **Braucht:** vor dem Zurückholen `git log --all --grep/-S` über alle Linien;
  Bau in `tools/register/src/bin/register_lookup.rs`.

## Benchmark

- Punkt 3 (hdf5-Assert) → `grind-flash`: mechanisch, korrekt; `cargo check
  --all-targets` 0/0. Klasse „mechanischer Fix": flash-Sieger, kein Doppel-Lauf.
- Punkt 4 (WGSL-KSG-Spiegel) → `grind-max`: hartes Atom (Urteil + Schreiben in
  einem Kontext); gebaut, `cargo check` 0/0; kein Doppel-Lauf (max per Design für
  harte Atome). Rat-Votum: Riss in eine gemessene Paritäts-Brücke geschlossen,
  nicht getragen; Verdrahtung als offener Punkt (Punkt 4).
- Punkt 1 (chunked-Granule) → `grind-pro`: Recherche-Messung; Ergebnis =
  korrigierter Zeuge (ATL03 = v1, depth ≥ 2), v2 Typ 10 bleibt offen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `src/archivar/hdf5.rs` (Assert + Layout-Diagnose),
  `.github/workflows/hdf5-real-granule.yml` (`--nocapture`),
  `src/mathematikerin/shaders.rs` (WGSL-KSG-Spiegel),
  `src/mathematikerin/te.rs` (Paritäts-Gate-Test + clippy-Fix
  `endpoint_matched`), `AGENTS.md` (Riss-Satz ersetzt), `docs/handover/post.md`
  (vier mountain-Zeilen entfernt), neues Handover
  `handover-2026-09-21-mountain-folge129.md`, Move
  `handover-2026-09-21-mountain-folge128.md` → `archiv/`.
- **Fremd (nicht angetastet):** `phi/pipeline/index.φ`, `phi/pipeline/ledger.φ`
  (mycelium), `.github/workflows/hyperscanning-te.yml`,
  `tools/measure/src/bin/hyperscanning_group_te.rs`, neues
  `handover-2026-09-21-sensory-folge142.md` (sensory),
  `docs/zustand/external-state.md` (fremde Zeilen).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt. Nach Push:
  `hdf5-real-granule` läuft durch den `paths`-Trigger auf `src/archivar/hdf5.rs`
  von selbst; `ci-check` deckt den Paritäts-Gate-Test.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
