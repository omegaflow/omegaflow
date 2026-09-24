<!--
  title: Handover — River-Folge 21 (Stand 2026-09-24)
  session: river
  class: handover
  date: 2026-09-24
  sha256: ce4aeb3d79ccbc32896c257126a0b6f7306d2c22d25a9e8a5ea8ee10df35ee10
  status: live
-->
# Handover — River-Folge 21 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Anlass: folge20 wird gefaltet. Die Post-Zeile „An river" (`post.md:24`,
**Vorbereitung ≠ Akt**) ist gefaltet und aus `post.md` gelöscht: die
`operator-gebundenen` Punkte stehen jetzt in **zwei** Zeilen — die Vorbereitung
autonom in Stufe 1 (dispatcht), allein der Akt `operator-gebunden` in Stufe 2.
Am HEAD `0f7a6b0de` gemessen, `origin/main` == HEAD, Arbeitsbaum clean.

## Stehender Pass (gemessen 2026-09-24)

- **Postfach** — `state/mail/mail_ledger.φ` existiert (gemessen via `sread`); keine
  neue Mail; `mail_digest` pending (CI-Bau).
- **CI** — HEAD `57e2ba3c8` (`origin/main` == HEAD). folge19-Run `35973038431`
  @`54a8f8aef` = **failure** (Job-Ebene, `ci_manage log` 2026-09-24): clippy rot
  `bayestar.rs:352` (`chunks_exact` → `as_chunks`, **mycelium**) **und** Test-Job
  rot — 2 Fails (`bayestar leaf` + `ble managed_objects`); der River-Job (clippy)
  ist **nicht** grün — die Beat-Arbitrierung ist nicht im CI verifiziert.
  Aktueller `ci-check` `36030755250` @`e05f8a419` = **failure**: Tests
  `bayestar.rs:494` + `ble.rs:1116`; clippy `bayestar.rs:352` +
  `spectral.rs:752` (`needless_range_loop`). **dropped-baseline** = **960**
  @`5179b438b`, gebumpt `df2b23320` (stale: delta 199 @`0c7682916`).
- **`git_safety --snapshot`** → „the working tree equals HEAD — nothing to record".
- **`open_points_check` folge20** — 17 Pfad-Refs; `state/mail/mail_ledger.φ`
  existiert jetzt (früher absent gemeldet); kein stale Punkt.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### spectral.rs:752 clippy
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (sofort handlungsfähig)
- **Lage:** `needless_range_loop` an `src/archivar/spectral.rs:752` (gemessen
  2026-09-24 via ci_manage log); `for c in 0..3` über die LUT-Texel, im
  `ci-check` `36030755250` @`e05f8a419`.
- **Blockade:** keine.
- **Braucht:** die Schleife index-frei umbauen, `cargo check` 0/0; der nächste
  `ci-check` bestätigt.

### Vorbereitung: LAN-Sensorik adb-reverse-Route
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (sofort handlungsfähig; Vorbereitung des Aktes in Stufe 2).
- **Lage:** `adb devices` listet `67151JEA305427 device` (gemessen 2026-09-24 via
  sgrep); `adb reverse tcp:1618 tcp:1618` gesetzt (nach adb-Neustart erneut zu
  setzen); Host-Relay läuft nicht; `OMEGAFLOW_HIDDEN=1` erzeugt keinen
  `TcpRadiator` (`main_flow.rs:715`).
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereitlegen — exakter Operator-Befehl
  (`bin/omegaflow`, sichtbarer Lauf), die Pixel-URL `http://127.0.0.1:1618`, der
  Messschritt (`sensor: N samples` >0 + Vibration) und die `adb reverse`-Wiederholung
  nach adb-Neustart.

### Vorbereitung: Sensor-Hardware Mantis-Shrimp Node / ESP32-S3 + HRV-Teile
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (sofort handlungsfähig; Vorbereitung des Aktes in Stufe 2).
- **Lage:** die BOM ist gemessen (gemessen 2026-09-24 via sread/sgrep) —
  `docs/specs/mantis-shrimp-bom.md` mit der Kern-Teilmenge in
  `docs/specs/mantis-shrimp-build.md §5` (16 Positionen, AliExpress-IDs + Preise
  2026-09-13); die Survey `survey-2026-09-23-geraete-anbindung-radiatoren.md` ist
  **Inventar**, nicht BOM; HRV-Kanal: MAX30102 (I2C, mux way 0, SpO₂ 100 Hz); die
  HRV-Bindung selbst bleibt `pending` (Survey:303, Trigger „Puls liegt an");
  `1N4007`-Freilaufdiode (M8) MANDATORY, im BOM-Table **nicht** als Position →
  `pending` (Item/Kosten ungemessen).
- **Blockade:** keine.
- **Braucht:** Kantenzeile steht — Artefakt `docs/specs/mantis-shrimp-bom.md`
  (Bau §5) | Beschaffung durch den Operator | Wort erwartet.

#### Stufe 2 — operator-gebunden

### Akt: LAN-Sensorik adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Vorbereitung (Stufe 1) steht + Operator-Wort zum sichtbaren Lauf.
- **Lage:** der Host-Relay läuft nicht; ein sichtbarer Lauf ist der Akt (gemessen
  2026-09-24 via sgrep).
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch
  einen sichtbaren Lauf.
- **Braucht:** der **Operator** startet `bin/omegaflow` in eigenem Ermessen; dann
  am Pixel die URL, `/consent`=1; messen `sensor: N samples` >0 + Vibration.

### Akt: Sensor-Hardware beschaffen/anschließen
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft und schließt die Sensor-Hardware an.
- **Lage:** der Mantis-Shrimp-Node und die HRV-Teile (BOM) sind nicht beschafft (gemessen 2026-09-23 via sread); das Inventar steht in
  `survey-2026-09-23-geraete-anbindung-radiatoren.md`.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session baut den Node bei gemessenem
  Vorhandensein.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

### Sonnenfarbe: GPU-Live-Parity + CI am neuen Commit (Bau steht)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check` des neuen Commits ist gelaufen (nach `/commit`+Push).
- **Lage:** der Farbmodus ist gebaut (Rat 2026-09-24, Commit `2de359982`) (gemessen 2026-09-24 via sgrep): Kanaltrennung (Helligkeit immer `t2(|val|)`,
  nur der Farbton wechselt), Operator-Toggle `c`, Default `field`; die Rust-LUT
  über den Relay-Endpunkt `/color_lut` aus `spectral::color_lut_wire()`
  (`spectral.rs:445`), Parser `static/constants.js`, Shader-Binding(3) + Modus in
  `vp.expose.y` (`static/index.html`); Statuszeile; Parity-Gate
  `gpu_lut_index_mirror_matches_color_for_ci` (`spectral.rs:780`) +
  `static/color_lut.test.mjs`; `cargo check --all-targets --features browser_relay`
  0/0. **Ungemessen:** die WGSL-Ausführung (Shader-Validierung + LUT-Lesen) — kein
  GPU-Lauf lokal. Der jüngste `ci-check` `36030755250` ist **fremd** rot
  (bayestar/ble), nicht der Farbmodus.
- **Blockade:** Heavy compute (GPU-Lauf) — Regel: CI/Operator-Wort.
- **Braucht:** `/commit`+Push → `ci-check` (die neuen Rust-/JS-Tests) und
  `gh workflow run ci-check.yml`; dann der erste sichtbare/hidden Lauf, der
  `color: measured` rendert.

### Beat-Arbitrierung: funktionale Verifikation im Betrieb
- **Status:** wartend | **Bindung:** eigen (hardware/versteckter Lauf)
- **Trigger:** ein Beat-Quellen-Satz ist am Host gesetzt (Serial-Gerät +
  `OMEGAFLOW_BLE_HR`/`FIT_DIR`) und ein sichtbarer/hidden Lauf steht.
- **Lage:** die Spawn-Arbitrierung steht (`main_flow.rs:504`) (gemessen
  2026-09-24 via sgrep), aber kein Lauf hat die Verdict-Zeile (`beat source: …`)
  gemessen erzeugt; `tests.rs` deckt nur die reine Funktion.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — eine Live-Messung braucht
  CI oder das Operator-Wort.
- **Braucht:** `cargo test` in CI (Funktion) bzw. ein versteckter Lauf mit zwei
  gesetzten Quellen, der genau eine `beat source:`-Zeile zeigt.

### 945-FIT-Datei (Onboard nur FIT/CIQ)
- **Status:** wartend | **Bindung:** eigen (Ein-Quellen-Regel)
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad (direkt vom FR945-Speicher /
  CIQ-App) wird gemessen nötig.
- **Lage:** der Host-Reader ist gemessen (gemessen 2026-09-24 via sgrep) —
  **sensory** hält `parse_fit` (`src/archivar/fit.rs:78`), verdrahtet in
  `main_flow.rs`; CIQ hat **keinen** Reader (`sgrep -i ciq src tools` = leer); ein
  Onboard-Reader ist nicht gebaut; River arbitriert die Beat-Quellen zentral
  (`main_flow.rs`); `OMEGAFLOW_SERIAL_IN` steht (`src/archivar/ingress.rs:4`,
  `ble.rs:931`).
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** erste Messung bleibt: wird ein Onboard/CIQ-Pfad gebraucht? Sonst
  bleibt `FIT_DIR` der FIT-Kanal; BLE (`OMEGAFLOW_BLE_HR`) und Serial sind über die
  Arbitrierung ausgeschlossen, solange FIT läuft.

### TLS im Relay für kabellose Sensorik
- **Status:** wartend | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage:** für jetzt verworfen (Rat 2026-09-23) (gemessen 2026-09-23 via sgrep):
  `rustls`/`native-tls` = neue C-Dependency im Kern (folge13-Lehre).
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

### te-ncurve sharden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-ncurve`-Run endet.
- **Lage:** `te-ncurve` `36052804297` in_progress @`57e2ba3c8` (gemessen
  2026-09-24 via ci_manage view); Cap auf 360 min angehoben, Vorgänger brachen am
  300-Cap ab.
- **Blockade:** keine — wartet auf Run-Ende (der Trigger).
- **Braucht:** `ci_manage view 36052804297`; läuft er >360 min, den `tcurve`-Job
  sharden.

#### Stufe 5 — termin

### Sensor-Bindung vC-Permeabilität (Smartwatch + Mantis-Shrimp)
- **Status:** termin | **Bindung:** operator (Hardware/versteckter Lauf)
- **Trigger:** Smartwatch + Mantis-Shrimp-Sensoren sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage:** der Permeabilitäts-Pfad steht (vC → `field_permeability`) (gemessen
  2026-09-24 via sgrep); Fundstelle `src/mathematikerin/omega.rs:200/349`; der
  HRV-Reader ist gebaut (`src/archivar/ble.rs:715/926`, external-state Zeile
  BLE-HR-Reader); `handover-2026-09-20-operator-entscheidungen.md:26` hält den
  `termin`.
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo
  run --release` aus, dann `perm_target_probe --live <pfad>`.

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** die präregistrierte JUICE-Erdpassage-Kette wartet auf das Perigäum (gemessen 2026-09-23 via sread); `docs/auftrag/auftrag-flyby2-kette.md` (Owner
  Forschung-Linie); RTSW-Retention (1 m mag/wind) **~24 h** — harte Frist: der
  erste Fill-Run muss ≤24 h nach der ersten Perigäum-Zelle starten, sonst ist die
  1-m-Kette der frühen Stunden nicht mehr messbar (kein Snapshot-Spiegel); Siegel
  + σ-Metrik stehen.
- **Blockade:** keine — wartet auf das Perigäum (der Trigger).
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle (RTSW/ACE Minuten,
  Kp ≤3 h, Swarm ≤1 d; je Messwert `source`+`active`).

#### Stufe 6 — LOCK

keiner.

## Benchmark

- **Planungs-Pass folge21:** `ci_manage view` (CI-Verdikt des folge19-Runs) +
  `sread`/`sgrep` (Post-Zeile gegen `index.html`/`spectral.rs`) — flash-Tier, kein
  pro/max.
- **Farbsemantik (Architektur):** `council` (pro/max), ein Lauf — Verdikt:
  Kanaltrennung (Helligkeit `|val|`, nur Farbton-Quelle wechselt), Operator-Toggle
  `c`, Default `field`, absent CI = benanntes Weiß (kein val-Fallback). Die harte
  A=A-Gaze-/Grenz-Klasse hat keinen flash-Ersatz.
- **Farbmodus-Bau (Multi-Layer Rust→JS→WGSL + Parity-Gate):** `grind-flash`, ein
  Lauf — vollständig (`spectral.rs`, `relay.rs`, `constants.js`, `index.html`,
  `color_lut.test.mjs`, `ci-check.yml`), `cargo check --all-targets --features
  browser_relay` 0/0. Kein Doppel-Lauf; die Klasse „Routine-Mehrschicht-Bau" trägt
  flash.
- **BOM-Extraktion (Routine):** `grind-flash`, ein Lauf — BOM gegen
  `mantis-shrimp-bom.md`/`build.md §5` gemessen; kein pro/max.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-24-river-folge21.md` (neu; folge20 → `archiv/`)
- `src/archivar/spectral.rs` (`color_lut_bounds`/`color_lut_wire` + Parity-Tests)
- `src/archivar/relay.rs` (`/color_lut`-Endpunkt)
- `static/index.html` (Shader-Modus, LUT-Binding, `c`-Taste, Statuszeile)
- `static/constants.js` (`parseColorLut`/`colorLutIndex`)
- `static/color_lut.test.mjs` (neu)
- `.github/workflows/ci-check.yml` (node-Parity-Test registriert)
- **lokal, gitignored (nicht im Commit):** `docs/zustand/external-state.md` — CI-Zeile

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
