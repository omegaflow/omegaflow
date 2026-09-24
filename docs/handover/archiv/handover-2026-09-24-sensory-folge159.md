<!--
  title: Handover — Sensory-Folge 159 (Stand 2026-09-24)
  session: Sensory-Folge 159
  class: handover
  date: 2026-09-24
  sha256: f87c0ae66b93dfc904face42a969bef54db0de9ffca1b3340d04f1b0c4660f8a
  status: live
-->
# Handover — Sensory-Folge 159 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

Jeder offene Punkt wird aufgeschlüsselt geführt: **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Keine Rangfolge — die parallel
abarbeitbaren Punkte wurden in diesem Atom dispatcht (Operator-Wort 2026-09-21).

## Stehender Pass (gemessen 2026-09-24, Sensory-Folge 159)

- **HEAD** `f417cdd69` == `origin/main`; Arbeitsbaum zu Session-Beginn sauber
  (gemessen 2026-09-24 via `git rev-parse HEAD`/`git status --short`), danach
  absichtlich schmutzig (eigene + fremde uncommittete Arbeit).
- **Postfach** — keine Mail an sensory; die sechs jüngsten `mail_ledger.φ`-Eingänge
  sind Maschinen-News (Rubin LSST forum, STScI News, GitHub-Support)
  (gemessen 2026-09-24 via `mail_digest --last 6`).
- **CI** — Watchdog-Snapshot 22:54: aktiv `te-ncurve 36052804297`,
  `health-check 36027534328`; **failed** `ci-check 36030755250` @`e05f8a419`
  (trug den roten `ble`-Accessor-Test, siehe unten). Neuere `ci-check 36059450659`
  in_progress / `36060479549` pending (fremder Dispatch), `tools-build` success
  (gemessen 2026-09-24 via Watchdog-Snapshot + `ci_manage list`).
- **Zustand** — `docs/zustand/external-state.md:35` (BLE-HR-Reader, sensory)
  fortgeschrieben (lokal, seit `7cd9aedd1` gitignored, `.gitignore:134`):
  GFDI-Schema gemessen, Live-Bring-up bleibt hardware-gebunden `pending`.

## Operator-Queue

Keine offene Frage. **Hardware-Bestellung ist LOCK** (Operator-Wort 2026-09-23, für
alle drei Beschaffungs-Punkte) — die Punkte werden nicht erneut vorgelegt.
Gekoppeltes FR945 ist vorhanden (Preflight gemessen), ein Live-Lauf braucht nur das
Wort.

## Offen (aufgeschlüsselt)

**Autonom**

### GFDI-Frame-Decode bauen (Schema gemessen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (jetzt handlungsfähig)
- **Lage:** GFDI-Schema ist **nicht mehr `kein Spec`**: Frame `size u16 LE | type u16 LE | payload | crc16 u16 LE` (CRC-16/ARC, poly `0x8005` reflected, init 0, Vektor `"123456789"`→`0xBB3D`), Protobuf auf msg `5043`/`5044` (`request_id u16 | data_offset u32 | total_size u32 | payload_len u32 | proto_bytes`); Smart-Wrapper → field 43 FileSyncService (fields 1/2/9/10/12/15/17); Garmin-Epoch 631 065 600 s; Quellen `github.com/gwerneckp/garmin-ble` (`.proto`) und `github.com/braycarlson/gfdi` (Zig, deckt jede Feldnummer), Gadgetbridge-Doku als Lineage (gemessen 2026-09-24 via `research-max`/`archive_search`).
- **Blockade:** keine
- **Braucht:** `grind-pro`/`grind-max` — in `src/archivar/ble.rs` den Frame-Splitter (crc-guard, `skip` bei Mismatch) + std-only Protobuf-Walk (varint/fixed64/len-delimited) + FileSync-Records in den `tx`-Kanal bauen; Fallthrough `ble gfdi <uuid> <hex>` bleibt. Numerische Felder durch die `v > 0.0`-Plausibilität; `type_name` (String) ist eine 26×f64-Slot-Vertragsfrage (Rat).

### FIT-Verifikation FR945 — Parser (eigener Schritt)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (erste Messung)
- **Lage:** `emit_nn` (`src/archivar/fit.rs:243-253`) differenziert nur **innerhalb eines Feldpuffers**; ein echtes File mit einem `event_timestamp` je HR-Message liefert kein `nn` — synthetic Fixtures maskieren das. Developer-Fields-Codepfad (`fit.rs:175-190`) auf echtem File unverifiziert (gemessen 2026-09-24 via `grind-flash`/`sread`).
- **Blockade:** keine (Parserfrage; eine echte Datei entscheidet sie sauber)
- **Braucht:** `grind-flash` — `emit_nn` über Message-Grenzen prüfen und ggf. korrigieren; kein Fabrikat ohne echtes File, dann `pending`.

**Operator-gebunden**

### BLE-Live-Bring-up FR945 — Akt
- **Status:** operator-gebunden | **Bindung:** operator + Hardware
- **Trigger:** Operator-Wort für den Live-Lauf
- **Lage:** Vorbereitung kantenfertig, Preflight gemessen (gemessen 2026-09-24 via `bluetoothctl`/`busctl`): FR945 gekoppelt/gebondet, Address/Connected/UUID ohne Körperdaten — `F0:99:19:4E:0B:BF`, `00002a37-…`, `notify`.
- **Blockade:** Operator-Kontext.
- **Braucht:** `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF OMEGAFLOW_HIDDEN=1 cargo run`
  (hidden = körperlesend, strahlungsstill; sichtbar nur auf ausdrückliches Wort).
  Kantenzeile: `omegaflow-Core-Bin BLE-Beat-Quelle (src/archivar/ble.rs:926; Auswahl src/archivar/main_flow.rs:11) | OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 cargo run | Wort erwartet`.

### FIT-Verifikation FR945 — Akt (echte Datei)
- **Status:** operator-gebunden | **Bindung:** operator (Datei)
- **Trigger:** Operator liefert den Pfad einer echten FR945-`.fit`-Datei
- **Lage:** env-gated Test `archivar::fit::tests::real_fit_sample_parses` gebaut (gemessen 2026-09-24 via `grind-flash`) — `src/archivar/fit.rs`, `#[ignore]`, prüft nur strukturelle Wahrheiten (Header+CRC, finite Werte, `nn` > 0).
- **Blockade:** kein `.fit`-File im Baum (`glob **/*.fit` leer).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/path/FR945.fit cargo test -p omegaflow
  --lib archivar::fit::tests::real_fit_sample_parses -- --ignored --nocapture`
  (der funktionale Lauf gehört in CI, nicht auf den Operator-Rechner).

**Wartend**

### `ble`-Accessor-Heil — CI-Bestätigung
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** CI-Lauf am Commit dieser Session
- **Lage:** Accessor-Shape-Mismatch geheilt (gemessen 2026-09-24 via `grind-max`/`ci_manage log 36030755250`) — `device_by_address` (`ble.rs:579`) und `characteristic_matches` (`ble.rs:641`) erwarteten `Array`-of-`Dict`, der Wire-Parser liefert `Dict`; Fixture `managed_characteristic` (`ble.rs:1181`) auf die wahre Wire-Form gezogen; `cargo check --tests` 0/0, 9/10 Tests waren grün, dieser war der rote.
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run ci-check.yml`, Ergebnis einmal lesen
  (`ci_manage log <id>`); bei Rot denselben Punkt erneut.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

**Termin**

keiner.

**LOCK**

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** `sgrep -i hrv|pulse|rmssd` in `firmware/` = 0 Treffer; HRV/Puls-Bindung `pending` per AGENTS.md; physische Teile fehlen (gemessen 2026-09-24 via `sgrep`).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; GNSS fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`) (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

## Benchmark

- **`ble`-Accessor-Test — Klasse DBus-Wire-Accessor geschlossen:** `grind-flash`
  heilte 9/10 (`fdb4f9f27`), der Accessor-Test blieb rot; **`grind-max`** fand in
  diesem Atom die Shape-Wurzel (`Dict` vs `Array`-of-`Dict`) und heilte ihn —
  `cargo check --tests` 0/0, hohe Konfidenz per Byte-Trace; der CI-Lauf entscheidet.
  Bestätigt die folge158-Regel: `cargo check --tests` 0/0 genügt nicht.
- **GFDI-Schema — `research-max`:** drei unabhängige öffentliche Quellen, deckungsgleich
  auf allen Feldnummern (43, 1, 2, 9, 10, 12, `0xA5A5`); `kein Spec` widerlegt.
- **FIT-Vorbereitung — `grind-flash`:** env-gated Test + Kommando kantenfertig; flash
  genügte (Routine-Extraktion).

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (Accessor-Shape geheilt)
- `src/archivar/fit.rs` (env-gated `real_fit_sample_parses`)
- `docs/handover/handover-2026-09-24-sensory-folge159.md` (neu)
- Move `handover-2026-09-24-sensory-folge158.md` → `archiv/` (eigene Linie, atomar)

`docs/zustand/external-state.md` (Zeile 35) wurde lokal fortgeschrieben, ist aber
seit `7cd9aedd1` gitignored (`.gitignore:134`) — kein Commit-Pfad.

Fremd uncommittet am Baum, nicht angefasst (gemessen 2026-09-24 via `git status --short`):
`.github/workflows/bayestar-cdn.yml`, `phi/sources.φ`, `src/archivar/cdn.rs`,
`src/gate/commit_gate.rs`, `tools/harvest/src/bin/ps1_coverage_compiler.rs`,
`tools/measure/src/bin/footprint_gate_probe.rs`, `tools/measure/src/bin/laic_probe.rs`,
`tools/measure/src/bin/silence_map_probe.rs`, `tools/measure/src/bin/topocentric_coupling_probe.rs`,
`tools/measure/src/bin/weberin_body_verdict.rs`, `tools/measure/src/bin/weberin_verdicts_compiler.rs`,
`tools/register/src/bin/open_points_check.rs`, `tools/service/src/bin/smail.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
