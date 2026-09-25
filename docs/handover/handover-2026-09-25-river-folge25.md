<!--
  title: Handover — River-Folge 25 (2026-09-25)
  session: River-Folge 25
  class: handover
  date: 2026-09-25
  sha256: 058bc360b4f312417757d5ac7b457cf45ee4aa1f4a16368949c04b078d3d191d
  status: live
-->
# Handover — River-Folge 25 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als Kopie
hier. Keine Rangfolge — die Punkte werden parallel von Agenten abgearbeitet. Die Tafel ist
nach **Verantwortlichem** getrennt (Linie | Rat | Operator | Extern), je Überschrift genau
eine Verantwortlichkeit; `blockiert`/`wartend`/`termin` werden benannt, nie dispatcht.
**Vorbereitung ≠ Akt.** Jede `Lage` trägt ihren Messstempel.

## Offen (aufgeschlüsselt)

### Linie — eigen

#### health-check `verify` — Shard-Lauf verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste scheduled `health-check`-Lauf am neuen HEAD endet.
- **Lage:** der `verify`-Sweep war chronisch 4–9 h (gemessen 2026-09-25 via Browser/
  `ci_manage`: #111 9h25m, #112 6h55m, #113 6h22m; cron 3h → Backlog #114 pending).
  Ursache: serielle Traversierung von 521 Quellen mit `-m = ceil(ttl/Φ²)` als Netz-Bound
  (`port.rs:2423`, `fetch.rs:63`). Rat-Verdikt: sharden, nicht die Menge schrumpfen.
  Gebaut (2026-09-25): `--verify phi --shard k/8`, `ci_mode(dir, shard)` mit
  kontiguousem Slice `shard_bounds` (`port.rs:2379`), Test
  `shard_tests::shards_contiguous_and_covering`; `health-check.yml` `verify`-Job als
  8er-Matrix, `timeout-minutes: 240`, Cache-Key je Shard.
- **Blockade:** kein Commit-Wort; die Verifikation lebt nur im CI-Lauf.
- **Braucht:** nach `/commit`: der 8er-Matrix-Lauf — jede Shard-Leg success, Gesamtdauer
  < 3 h; dann Cron/Cadence aus der gemessenen Leg-Dauer nachziehen.

#### TLS im Relay (wireless) — pending
- **Status:** pending | **Bindung:** eigen
- **Trigger:** ein kabelloser Sensor wird für die Sensorik gebraucht (der Relay-Zweck).
- **Lage:** kabellos braucht HTTPS (secure context) für `Accelerometer`/`Gyroscope`/
  `DeviceMotion`/`getUserMedia` (`static/sensorium.js:14/88/137`); adb-reverse deckt nur
  den tethered Fall (`127.0.0.1`). Die Notiz „verworfen (neue C-Dependency)" ist
  widerlegt (gemessen 2026-09-25): rustls ist pure Rust, native-tls linkt System-OpenSSL;
  ein externer Terminator braucht gar keinen Kern-Umbau.
- **Blockade:** keine — ungebaut und gebraucht.
- **Braucht:** externer TLS-Terminator (Caddy/stunnel) vor `relay.rs` + lokale CA am
  Gerät; der Kern bleibt `std+curl+serialport`.

### Rat
keiner — kein Punkt steht aktuell vor dem Rat.

### Operator

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (gemessen 2026-09-25 via sread/tools-build.yml):
  Relay kompiliert im veröffentlichten PATH-Bin (`tools-build.yml:26`
  `cargo build --release -p omegaflow --bin omegaflow --features browser_relay`,
  `omegaflow` in der TOOLS-Liste Z. 29); `bin/omegaflow` execs `target/release/omegaflow`;
  Bind `0.0.0.0:1618` (`relay.rs:11/73`), `hidden = env("OMEGAFLOW_HIDDEN").is_ok()`
  (`main_flow.rs:612`, jeder Wert unterdrückt den Relay), Startzeile
  `serving on http://{bind}:{port}`, Consent `/consent?ja` (`relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch einen sichtbaren Lauf.
- **Braucht:** der **Operator** führt aus:
  `adb devices && adb reverse tcp:1618 tcp:1618` (nach jedem adb-Neustart erneut),
  dann `bin/omegaflow` **ohne** `OMEGAFLOW_HIDDEN`; stderr `serving on http://0.0.0.0:1618`,
  `http://127.0.0.1:1618` öffnen, Consent bestätigen, messen: `sensor: N samples` N>0
  **und** Vibration (`navigator.vibrate`, `static/radiator.js:105`).

#### Akt: Sensor-Hardware beschaffen (Bestellung)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator bestellt die BOM und schließt die Hardware an.
- **Lage:** die BOM ist am 2026-09-25 bestellfertig gemacht (gemessen 2026-09-25 via
  `archive_search --playwright`, `docs/specs/mantis-shrimp-bom.md` sha256 `e56f395d…`):
  `1N4007` (1005006454795578) und `DS18B20` wasserdicht (1005012179635448) gefüllt,
  alle 9 Outdoor-Positionen mit Item-ID. Die alte Notiz „1N4007 kein AliExpress-Treffer"
  ist widerlegt (jene Suche lief über `--all`, das AliExpress nicht abfragt). Die
  2026-09-25-Session rendert **CHF**; die EUR-Spalte dieser Positionen ist EZB-abgeleitet
  (1 CHF = 1,0628 EUR, 2026-09-24, in der BOM mit `*` markiert), nicht direkt gemessen.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber); kein Node-Teil vorhanden.
- **Braucht:** Operator bestellt die BOM-Positionen (AliExpress-Login nötig) und liest
  den EUR-Preis in seiner Session gegen; Session baut den Node bei gemessenem Vorhandensein.
  Spezifikations-Korrektur mitgenommen: `TP4056` ist ein 4,2-V-Li-Ion-Lader und darf eine
  3,2-V-LiFePO4-Zelle **nicht** laden — der LiFePO4-Pfad braucht ein 3,2-V-BMS.

#### Sonnenfarbe: erster Lauf `color: measured`
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren/hidden Lauf.
- **Lage:** der Farbmodus ist gebaut (gemessen 2026-09-24 via sgrep; Rat 2026-09-24, Commit `2de359982`, Ancestor von HEAD):
  Kanaltrennung, Operator-Toggle `c`, Default `field`, Rust-LUT über `/color_lut`
  (`spectral.rs:445`), Parser `static/constants.js`, Shader-Binding(3) + Modus in
  `vp.expose.y` (`static/index.html`); Parity-Gate `gpu_lut_index_mirror_matches_color_for_ci`
  (`spectral.rs:780`) + `static/color_lut.test.mjs` grün. **Ungemessen:** die
  WGSL-Ausführung im Lauf (Shader-Validierung + LUT-Lesen) — kein GPU-Lauf lokal.
- **Blockade:** Heavy compute (GPU-Lauf) — Regel: CI/Operator-Wort.
- **Braucht:** der erste Lauf, der `color: measured` rendert (Operator-Wort), zusammen mit
  der adb-reverse-Route oben.

#### Sensor-Bindung vC-Permeabilität
- **Status:** operator-gebunden (wartet auf Hardware) | **Bindung:** operator
- **Trigger:** Smartwatch + Mantis-Shrimp-Sensoren sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage:** der Permeabilitäts-Pfad steht (vC → `field_permeability`) (gemessen 2026-09-24
  via sgrep); Fundstelle `src/mathematikerin/omega.rs:200/349`; der HRV-Reader ist gebaut
  (`src/archivar/ble.rs:715/926`); `handover-2026-09-20-operator-entscheidungen.md:26`
  hält den `termin`. Kein Datum — der Trigger ist die angeschlossene Hardware, daher
  `operator-gebunden`, nicht `termin`.
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`
  aus, dann `perm_target_probe --live <pfad>`.

### Extern

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin (Owner Forschung-Linie)
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via sread) die präregistrierte JUICE-Erdpassage-Kette
  wartet auf das Perigäum; `docs/auftrag/auftrag-flyby2-kette.md` (Owner Forschung-Linie);
  RTSW-Retention (1 m mag/wind) **~24 h** — harte Frist: der erste Fill-Run muss ≤24 h nach
  der ersten Perigäum-Zelle starten, sonst ist die 1-m-Kette der frühen Stunden nicht mehr
  messbar; Siegel + σ-Metrik stehen.
- **Blockade:** keine — wartet auf das Perigäum (der Trigger).
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle (RTSW/ACE Minuten, Kp ≤3 h,
  Swarm ≤1 d; je Messwert `source`+`active`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
