<!--
  title: Handover — Mountain-Folge 126 (Stand 2026-09-21)
  session: Mountain-Folge 126
  class: handover
  date: 2026-09-21
  sha256: f6428eb45bb859a760de419426afb82f4f80bb71064f1f46104800fc18b44d4e
  status: live
-->
# Handover — Mountain-Folge 126 (2026-09-21)

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

- **HEAD** `90fdbed4` == `origin/main` beim Start; der Baum trug die eigenen
  folge125-Commits (`3186709b`, `d6538ebe`, `90fdbed4`).
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", informativ; Eintrag `docs/zustand/external-state.md`, Zeile
  „Postfach"); kein handlungsbedürftiger Fall.
- **CI** (Watchdog-Snapshot 15:31Z + `ci_manage list`): `ci-check 35603922594`
  failure (attempt 1), `ci-check 35611404040` pending, `ned-cdn 35611340540`
  in_progress; `tools-build`/`glm-l2-cdn` mehrfach success. Kein Poll.
- **`register_lookup --open`** — 601 offen, 0 Post, 14 `zustand` due; kein
  mountain-getaggter Register-Punkt offen (`blocked_sources.φ` ohne
  `parser-def`, ledger `parser-gap` disponiert, `harvest.φ` 0 offen).
- **`git_safety --snapshot`** — Arbeitsbaum gleich HEAD, nichts zu sichern.
- **`open_points_check`** (folge125) — 3 Pfad-Refs, 0 absent.

## Offen (aufgeschlüsselt)

### 1. HDF5 chunked-read — mehrstufiger v2-B-Tree (`BTIN`, depth > 0)
- **Status:** offen | **Bindung:** eigen
- **Lage:** der Kandidat „chunked Read materialisiert nur Element 0" ist für den
  einstufigen Fall **entwarnt** (gemessen): `src/archivar/hdf5.rs` Chunked-Zweig
  schreibt pro B-Tree-Record an `(scaled[d]+idx[d])*stride`; Regressionstest
  `chunked_v2_single_element_chunks_materialize_every_element` (Layout v3,
  `chunk_dims=[1]`, `ds.dims=[3]`, 3 `BTLF`-Records → 3 Elemente). Offen bleibt
  der **mehrstufige** v2-B-Tree (`BTIN`, depth > 0) in `btree_records`/
  `internal_node_try` (Z. 1477/1558) — ungemessen.
- **Blockade:** keine (Bau/Messung).
- **Braucht:** synthetischer `BTIN`-Test (depth > 0) **oder** ein Real-Granule-
  Read (`glm-l2-cdn.yml`, S3-Prefix `GLM-L2-LCFA/2026/001/00/`) — Element-Zahl
  nach Read.

### 2. DS18B20 1-Wire + Cutoff — CI-Verifikation + `unstable`-Feature
- **Status:** offen | **Bindung:** eigen
- **Lage:** gebaut — reine Logik `firmware/radiatorium-lib/src/ds18b20.rs`
  (CRC8 0x31, Scratchpad-Dekodierung, `Cutoff` mit `CUTOFF_C=80.0`/
  `RELEASE_C=70.0`, Start + Sensor-absent fail-safe), Bit-Bang
  `firmware/radiatorium/src/one_wire.rs` (GPIO7), Bindung in `main.rs`
  (M2 heater c1/GPIO38, M4/M5 peltier c3/c4/GPIO40/41 auf Duty 0 bei Cutoff).
  12 Host-Tests. Lokal nur Syntax-Gate möglich (`export-esp.sh` fehlt); die
  Host-Tests + der esp-Build laufen in `esp32-firmware.yml` (Trigger
  `firmware/**`).
- **Blockade:** CI-Verifikation steht aus, bis der Push den Lauf startet.
- **Braucht:** Push → `esp32-firmware.yml` (Host-Tests + esp-Build); Wort zur
  esp-hal-Feature-Erweiterung `unstable` (nötig für `gpio::Flex`, den
  bidirektionalen Open-Drain-Pin) — behalten oder auf Treibertausch umstellen.

### 3. Ox64-Zweitknoten — Hardware/Bring-up
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat den Ox64 zugesagt; Doku
  (`docs/specs/mantis-shrimp-build.md §Zweitknoten`) steht; Gerät nicht da.
- **Blockade:** Geräteankunft (Anfrage 2026-09-20 an PINE64).
- **Braucht:** Ankunft abwarten; dann Buildroot/OpenWrt-Bring-up + Kopplung
  messen (die `pending`-Punkte der Doku).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
