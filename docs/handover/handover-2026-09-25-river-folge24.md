<!--
  title: Handover — River-Folge 24 (2026-09-25)
  session: River-Folge 24
  class: handover
  date: 2026-09-25
  sha256: 4d246da908aa09269e801cc1bd4c87d110523f93781510b03f8aabf1a5f3a9b7
  status: live
-->
# Handover — River-Folge 24 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte (Stehender-Pass-Ergebnis, geschlossen-Register,
Benchmark, Geteilter Baum): sie leben in git. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. **Vorbereitung ≠ Akt:** ein `operator-gebundener` Punkt steht in zwei
Zeilen — die Vorbereitung (Stufe 1, autonom, dispatcht) und der **Akt** allein
(operator-gebunden). Jede `Lage` trägt ihren Messstempel.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

keiner. Die Vorbereitung beider `operator-gebundener` Akte steht vollständig an der
Kante — kein eigener offener Schritt: der Relay liegt im CI-Release
(`tools-build.yml:26` baut `-p omegaflow --features browser_relay`), die BOM ist
bestellfertig (siehe Stufe 2). Kein dispatchbarer Punkt in diesem Atom.

#### Stufe 2 — operator-gebunden

### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
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

### Akt: Sensor-Hardware beschaffen (Bestellung)
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

#### Stufe 4 — wartend

### Sonnenfarbe: GPU-Live-Parity + erster Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein sichtbarer/hidden Lauf rendert `color: measured`.
- **Lage:** der Farbmodus ist gebaut (gemessen 2026-09-24 via sgrep; Rat 2026-09-24, Commit `2de359982`, Ancestor von HEAD):
  Kanaltrennung, Operator-Toggle `c`, Default `field`, Rust-LUT über `/color_lut`
  (`spectral.rs:445`), Parser `static/constants.js`, Shader-Binding(3) + Modus in
  `vp.expose.y` (`static/index.html`); Parity-Gate `gpu_lut_index_mirror_matches_color_for_ci`
  (`spectral.rs:780`) + `static/color_lut.test.mjs` grün. **Ungemessen:** die
  WGSL-Ausführung im Lauf (Shader-Validierung + LUT-Lesen) — kein GPU-Lauf lokal.
- **Blockade:** Heavy compute (GPU-Lauf) — Regel: CI/Operator-Wort.
- **Braucht:** der erste sichtbare/hidden Lauf, der `color: measured` rendert
  (Operator-Wort), zusammen mit der adb-reverse-Route oben.

### te-ncurve sharden — Verifikation des Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-ncurve`-Lauf am Commit `9e894f368` endet.
- **Lage:** das Sharden ist gebaut (gemessen 2026-09-25 via read/sgrep; `--t a:b` in
  `tools/measure/src/bin/pcmci_class_benchmark.rs`, Filter im `TCURVE_ONLY`-Block;
  `te-ncurve.yml` `tcurve`-Job als 4er-Matrix 400:450/500:550/600:650/700:700, Cap 180 min);
  der Lauf **läuft** (gemessen 2026-09-25 via `ci_manage view 36102727319`: `in_progress`,
  head_sha `9e894f368` = river folge23).
- **Blockade:** keine — wartet auf das Laufende (der Trigger).
- **Braucht:** `ci_manage view 36102727319` — jedes Matrix-Segment success, kein 180-Cap-Abbruch
  (der Workflow ist bereits dispatcht, kein zweiter Lauf).

### Beat-Arbitrierung: funktionale Verifikation im Betrieb
- **Status:** wartend | **Bindung:** eigen (hardware/versteckter Lauf)
- **Trigger:** ein Beat-Quellen-Satz ist am Host gesetzt (Serial-Gerät + `OMEGAFLOW_BLE_HR`/`FIT_DIR`)
  und ein sichtbarer/hidden Lauf steht.
- **Lage:** die Spawn-Arbitrierung steht (`main_flow.rs:504`) (gemessen 2026-09-24 via sgrep),
  aber kein Lauf hat die Verdict-Zeile (`beat source: …`) gemessen erzeugt; `tests.rs` deckt
  nur die reine Funktion.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — eine Live-Messung braucht CI oder Operator-Wort.
- **Braucht:** `cargo test` in CI (Funktion) bzw. ein versteckter Lauf mit zwei gesetzten Quellen,
  der genau eine `beat source:`-Zeile zeigt.

### 945-FIT-Datei (Onboard nur FIT/CIQ)
- **Status:** wartend | **Bindung:** eigen (Ein-Quellen-Regel)
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad wird gemessen nötig.
- **Lage:** der Host-Reader ist gemessen (gemessen 2026-09-24 via sgrep) — sensory hält
  `parse_fit` (`src/archivar/fit.rs:78`), verdrahtet in `main_flow.rs`; CIQ hat **keinen**
  Reader (`sgrep -i ciq src tools` = leer); River arbitriert die Beat-Quellen zentral;
  `OMEGAFLOW_SERIAL_IN` steht (`src/archivar/ingress.rs:4`, `ble.rs:931`).
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** erste Messung bleibt: wird ein Onboard/CIQ-Pfad gebraucht? Sonst bleibt
  `FIT_DIR` der FIT-Kanal; BLE (`OMEGAFLOW_BLE_HR`) und Serial sind über die Arbitrierung
  ausgeschlossen, solange FIT läuft.

### TLS im Relay für kabellose Sensorik
- **Status:** wartend | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage:** für jetzt verworfen (gemessen 2026-09-23 via sgrep; Rat 2026-09-23): `rustls`/`native-tls` = neue
  C-Dependency im Kern (folge13-Lehre) (gemessen 2026-09-23 via sgrep).
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

#### Stufe 5 — termin

### Sensor-Bindung vC-Permeabilität (Smartwatch + Mantis-Shrimp)
- **Status:** termin | **Bindung:** operator (Hardware/versteckter Lauf)
- **Trigger:** Smartwatch + Mantis-Shrimp-Sensoren sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage:** der Permeabilitäts-Pfad steht (vC → `field_permeability`) (gemessen 2026-09-24
  via sgrep); Fundstelle `src/mathematikerin/omega.rs:200/349`; der HRV-Reader ist gebaut
  (`src/archivar/ble.rs:715/926`); `handover-2026-09-20-operator-entscheidungen.md:26`
  hält den `termin`.
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`
  aus, dann `perm_target_probe --live <pfad>`.

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** termin:2026-09-28
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
