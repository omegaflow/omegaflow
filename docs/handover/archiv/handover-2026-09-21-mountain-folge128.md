<!--
  title: Handover — Mountain-Folge 128 (Stand 2026-09-21)
  session: Mountain-Folge 128
  class: handover
  date: 2026-09-21
  sha256: 50eedd7e007f592a0516eca4c544ec50f8501e70aec124665beb5ab41e951ec8
  status: live
-->
# Handover — Mountain-Folge 128 (2026-09-21)

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

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `da1213f4` (mountain folge127) == `origin/main` beim Start; während
  der Session auf `84d09a0e` (future folge85) == `origin/main` fortgeschritten —
  fremde Commits (river folge3 `3e319087`, forschung folge140 `0b1a7d03`, future
  folge85 `84d09a0e`) auf `da1213f4`. Der Baum trägt Fremdarbeit (ernte/sensory/
  river/future: `phi/*.φ`, `docs/zustand/external-state.md`, staged Renames
  `handover-2026-09-21-{ernte-folge134,forschung-folge140}.md` → `archiv/`, neue
  `handover-2026-09-21-{ernte-folge135,sensory-folge141}.md`) — nicht angetastet.
- **Postfach** — neuester Ledger-Eingang `1790009781` (Cloudflare „Your login
  verification code", 2026-09-21 16:56:02 UTC — Login-/Sicherheitsereignis,
  informativ); davor `1789978555` (Brave Search API „usage limit reached",
  informativ). Kein handlungsbedürftiger mountain-Fall. Die
  `external-state.md`-Postfach-Zeile (Stand `1789978555`) ist damit fällig; die
  Datei trägt Fremdarbeit → von dieser Linie nicht fortgeschrieben, hier gemessen.
- **CI** — der geteilte CI-Stand steht in `docs/zustand/external-state.md` (nicht
  kopiert). Eigener Trigger `ci-check` auf `da1213f4`: die jüngsten
  `ci-check`-Läufe sind durch Konkurrenz gecancelt (`35628616878`, `35627916025`,
  `35627819169`, …); der neueste `ci-check` `35629979237` ist **pending**
  (17:07Z); kein abgeschlossener grüner Lauf auf einem `da1213f4`-Nachfahren
  gemessen. Kein Poll — Ergebnis beim nächsten Pass.
- **`register_lookup --open`** — 618 offen, 5 Post offen, kein mountain-getaggter
  Register-Punkt; keine `An mountain:`-Post-Zeile.
- **`git_safety --snapshot`** — `refs/safety/1790008495`.

## Offen (aufgeschlüsselt)

### 1. HDF5 chunked-read — realer Chunk-Baum (Typ 10, depth ≥ 2) unbewiesen
- **Status:** offen | **Bindung:** eigen
- **Lage:** am echten Granule gemessen — GLM-L2-LCFA G18
  `OR_GLM-L2-LCFA_G18_s20260010029200_e20260010029400_c20260010029411.nc`
  (208 700 B, sha256 `07caa4325bac5ea25ab35226156b05d98c6f94721eaf54d266d3f20016cffc4e`,
  token-frei via `noaa-goes18` S3, `--verdict` direct 200): die Datei trägt
  **keinen** Chunk-B-Tree (20 BTHDs, Typen nur 05/06/08/09; `flash_lat`
  contiguous); die realen BTIN-Knoten (Typ 05/06, depth 1) tragen
  `(nsz, tsz) = (1, 0)` — checksum-bewiesen (Jenkins über 35 B == `65aab5a4`),
  `btree_records` materialisiert 54 == `total_records`. Kein Fix nötig:
  GLM-L2-LCFA ist **Name-Index**-Zeuge, nicht Chunk-Baum-Zeuge. Dreifach gemessen
  (GLM-L2-LCFA, ABI-L1b-RadC, ABI-L2-ACMC) keine chunked Datasets. Die Messung
  ist jetzt permanent: Test
  `real_granule_glm_l2_name_btree_materializes_every_element`
  (`src/archivar/hdf5.rs:4024`, `#[ignore]`, lädt via `GLM_L2_GRANULE`) + neuer
  Workflow `.github/workflows/hdf5-real-granule.yml`.
- **Blockade:** keine chunked Quelle im offenen Bucket gemessen.
- **Braucht:** ein Granule einer chunked Quelle mit >~15 Chunks/Blatt (Kandidat:
  ICESat-2 ATL03 — `icesat2_atl03_compiler` nutzt `read_chunk` bereits) → dann
  die depth-2-Kandidaten `(nsz, tsz)` gegen die echte Checksum prüfen; nur bei
  Fehlschlag Fix in `btree_records:1522`.

### 2. `flush_port_block` — CI-Verifikation des Fix
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** `flush_port_block` (`src/archivar/port.rs:360`) prüft den
  `# pending`-Marker vor dem `parsed`-Zweig (Test `tests.rs:6837`, war in
  `ci-check 35608204623` rot). `cargo check --all-targets` 0/0. Committet in
  `b2f3fa51`/`da1213f4`.
- **Blockade:** kein abgeschlossener grüner `ci-check` auf einem
  `da1213f4`-Nachfahren (Konkurrenz-Cancels).
- **Braucht:** `ci-check 35629979237` (pending) → Test grün; Ergebnis beim
  nächsten Pass.

### 3. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat den Ox64 zugesagt; Versanddaten wurden gesendet
  (`docs/zustand/external-state.md`); Gerät nicht da; Doku
  `docs/specs/mantis-shrimp-build.md §Zweitknoten` steht.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen.

## Benchmark

- Punkt 1 → `grind-max` (hartes Parser-Atom: Urteil + Schreiben in einem
  Kontext): Ergebnis = **kein Fix**, die Messung selbst (Name-Index-Zeuge,
  `(nsz, tsz) = (1, 0)`, 54 == total), permanenter `#[ignore]`-Test + neuer
  Workflow. Kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `src/archivar/hdf5.rs` (real-granule Test),
  `.github/workflows/hdf5-real-granule.yml` (neu), neues Handover
  `handover-2026-09-21-mountain-folge128.md`, Move
  `handover-2026-09-21-mountain-folge127.md` → `archiv/`.
- **Fremd (nicht angetastet):** `phi/blocked_sources.φ`, `phi/footprints.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`, `phi/witnesses.φ`,
  `docs/zustand/external-state.md`, die staged Renames
  `handover-2026-09-21-{ernte-folge134,forschung-folge140}.md` → `archiv/`, die
  neuen `handover-2026-09-21-{ernte-folge135,sensory-folge141}.md`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt. Nach Push:
  `gh workflow run hdf5-real-granule.yml` dispatcht.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
