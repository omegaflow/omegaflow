<!--
  title: Handover — Mountain-Folge 127 (Stand 2026-09-21)
  session: Mountain-Folge 127
  class: handover
  date: 2026-09-21
  sha256: 363e4c057a1c1289740cd3c9f90f7f634ad9d30743f37f27b0a29c71e2ed779a
  status: live
-->
# Handover — Mountain-Folge 127 (2026-09-21)

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

- **HEAD** `b3ec7683` beim Start; während der Session auf `b2c966c0` (river
  folge2) == `origin/main` fortgeschritten. Der Baum trägt Fremdarbeit (river:
  `phi/blocked_sources.φ`, `phi/pipeline/ledger.φ`, `phi/sources.φ`, `post.md`;
  future: `src/mathematikerin/te.rs`) — nicht angetastet, nicht mitcommittet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", informativ); kein handlungsbedürftiger Fall.
- **CI** — der geteilte CI-Stand steht in `docs/zustand/external-state.md`
  (Zeile „CI-Status", von river folge2 fortgeschrieben) — hier nicht kopiert.
  Der eigene Trigger: `esp32-firmware 35615957619` **failure** (Wurzel in Punkt 2).
- **`register_lookup --open`** — 601 offen, 0 Post, kein mountain-getaggter
  Register-Punkt offen.
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD beim Start, nichts zu sichern.

## Offen (aufgeschlüsselt)

### 1. HDF5 chunked-read — mehrstufiger v2-B-Tree (`BTIN`, depth > 0)
- **Status:** offen | **Bindung:** eigen
- **Lage:** synthetisch gemessen — zwei Tests
  `chunked_v2_multilevel_internal_root_materializes_every_element`
  (`src/archivar/hdf5.rs:3945`, depth 1: `BTIN`-Wurzel → 3 `BTLF`-Blätter, 5
  Chunks) und `chunked_v2_multilevel_depth2_materializes_every_element`
  (`src/archivar/hdf5.rs:3985`, depth 2 mit Subtree-Totals + `UNDEF`-Skip, 6
  Chunks): jeder Knotenpfad materialisiert **alle** Elemente, `read_chunk(&[n])`
  ist `None`. Kein Produktions-Fix nötig (`btree_records`/`internal_node_try`,
  Z. 1477/1558, wandern korrekt). `cargo check --all-targets` 0/0.
- **Blockade:** keine — der synthetische Baum ist von der Hand gebaut; das echte
  Writer-Layout (Probe-Kandidaten `(nsz,tsz)`) ist damit noch nicht bewiesen.
- **Braucht:** Real-Granule-Read GLM-L2-LCFA S3
  (`GLM-L2-LCFA/2026/001/00/`) → Element-Zahl nach Read; falls die echte Datei
  eine nicht gelistete `nsz`/`tsz`-Kombination trägt, Fix.

### 2. DS18B20 1-Wire + Cutoff — CI-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** Host-Tests **65 pass** im Lauf `35615957619`; der esp-Build scheiterte
  am **transienten** `espup`-Toolchain-Download (HTTP 504 Gateway Timeout) — kein
  Code-Fehler. Neulauf `35625255981` auf HEAD dispatcht. Entscheid `unstable`:
  **behalten** — gemessen (grind-pro): nur `gpio::Flex` ist `#[instability::unstable]`
  (`one_wire.rs:16–62`, `main.rs:143`); kein stabiler Einzelpin-Bidirektional-
  Treiber in esp-hal 1.2.1, `ds18b20`/`one-wire-bus` sind embedded-hal-0.2 (2020).
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35625255981`; bei success Punkt zu.

### 3. `flush_port_block` — CI-Verifikation des Fix
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** `flush_port_block` (`src/archivar/port.rs:360`) prüft den
  `# pending`-Marker jetzt **vor** dem `parsed`-Zweig. Wurzel: der `map`-Extract
  ließ einen force-losen Block als `parsed` zählen (Test
  `test_flush_port_block_carries_pending_review_marker`, `tests.rs:6837`, war in
  `ci-check 35608204623` rot). `cargo check --all-targets` 0/0.
- **Blockade:** Lauf (ci-check nach Push).
- **Braucht:** nach Push `ci-check` → Test grün.

### 4. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat den Ox64 zugesagt; Versanddaten wurden gesendet
  (`docs/zustand/external-state.md`, Postfach-Zeile); Gerät nicht da; Doku
  `docs/specs/mantis-shrimp-build.md §Zweitknoten` steht.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen (die `pending`-Punkte der Doku).

## Benchmark

- Punkt 1 → `grind-max` (hartes Parser-Atom: Urteil + Schreiben in einem
  Kontext); Punkt 2 (Entscheid `unstable`) → `grind-pro`; Punkt 3 im
  `line`-Kontext. Kein Doppel-Lauf auf derselben Aufgabe, kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `src/archivar/hdf5.rs`, `src/archivar/port.rs`, neues
  Handover `handover-2026-09-21-mountain-folge127.md`, Move
  `handover-2026-09-21-mountain-folge126.md` → `archiv/`.
- **Fremd (nicht angetastet):** river (`phi/blocked_sources.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`), future (`src/mathematikerin/te.rs`),
  und die `post.md`-Fremdzeilen (river/sensory) — die zwei eigenen Post-Zeilen
  (`An river`, `An future`) stehen im Arbeitsbaum, ohne `post.md` mit dem fremden
  Inhalt zu committen.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
