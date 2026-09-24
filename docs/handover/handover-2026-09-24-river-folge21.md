<!--
  title: Handover — River-Folge 21 (Stand 2026-09-24)
  session: river
  class: handover
  date: 2026-09-24
  sha256: 59a2b1c612cfa063553e45aeab88eb3b5186a79f915c9c932d89e0658ce8f384
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

- **Postfach** — `state/mail/mail_ledger.φ` in diesem Baum absent, `mail_digest`
  pending (CI-Bau); keine Mail. `post.md` trägt nur fremde Zeilen (mountain,
  mycelium, sensory) — **kein `An river`** mehr.
- **CI** — HEAD beim Pass `0f7a6b0de`; beim Session-Ende `3a4e1d8ca`
  (`origin/main` == HEAD, fremde Atome zogen nach). folge19-Run `35973038431`
  @`54a8f8aef` = **failure**, Job-Ebene gelesen (`ci_manage log` 2026-09-24): rot
  ist allein `bayestar.rs:352` (`chunks_exact` → `as_chunks`, **mycelium**); der
  River-Job (clippy) ist **grün** — die Beat-Arbitrierung ist functional
  verifiziert. **Vom Operator beauftragt:** `register-dropped` `36030528745`
  @`3a4e1d8ca` dispatcht (2026-09-24T16:53Z) — der vollständige
  `register_lookup --dropped`-Sweep; Ergebnis `pending` (die Session pollt nicht,
  Abnahme via `ci_manage view 36030528745` / Watchdog-Snapshot im nächsten Pass).
- **`git_safety --snapshot`** → „the working tree equals HEAD — nothing to record".
- **`open_points_check` folge20** — 17 Pfad-Refs, 1 absent
  (`state/mail/mail_ledger.φ`, Postfach, erwartet); kein stale Punkt.

## Offen (aufgeschlüsselt)

### Stufe 1 — autonom

#### Vorbereitung: LAN-Sensorik adb-reverse-Route
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (sofort handlungsfähig; Vorbereitung des Aktes in Stufe 2).
- **Lage:** `adb devices` listet `67151JEA305427 device`; `adb reverse tcp:1618
  tcp:1618` gesetzt (nach adb-Neustart erneut zu setzen); Host-Relay läuft nicht;
  `OMEGAFLOW_HIDDEN=1` erzeugt keinen `TcpRadiator` (`main_flow.rs:715`).
  (gemessen 2026-09-24 via sgrep)
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereitlegen — exakter Operator-Befehl
  (`bin/omegaflow`, sichtbarer Lauf), die Pixel-URL `http://127.0.0.1:1618`, der
  Messschritt (`sensor: N samples` >0 + Vibration) und die `adb reverse`-Wiederholung
  nach adb-Neustart.

#### Vorbereitung: Sensor-Hardware Mantis-Shrimp Node / ESP32-S3 + HRV-Teile
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (sofort handlungsfähig; Vorbereitung des Aktes in Stufe 2).
- **Lage:** die BOM ist gemessen — `docs/specs/mantis-shrimp-bom.md` mit der
  Kern-Teilmenge in `docs/specs/mantis-shrimp-build.md §5` (16 Positionen,
  AliExpress-IDs + Preise 2026-09-13); die Survey
  `survey-2026-09-23-geraete-anbindung-radiatoren.md` ist **Inventar**, nicht BOM.
  HRV-Kanal: MAX30102 (I2C, mux way 0, SpO₂ 100 Hz); die HRV-Bindung selbst bleibt
  `pending` (Survey:303, Trigger „Puls liegt an"). `1N4007`-Freilaufdiode (M8)
  MANDATORY, im BOM-Table **nicht** als Position → `pending` (Item/Kosten
  ungemessen). (gemessen 2026-09-24 via sread/sgrep)
- **Blockade:** keine.
- **Braucht:** Kantenzeile steht — Artefakt `docs/specs/mantis-shrimp-bom.md`
  (Bau §5) | Beschaffung durch den Operator | Wort erwartet.

### Stufe 2 — operator-gebunden

#### Akt: LAN-Sensorik adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Vorbereitung (Stufe 1) steht + Operator-Wort zum sichtbaren Lauf.
- **Lage:** der Host-Relay läuft nicht; ein sichtbarer Lauf ist der Akt. (gemessen
  2026-09-24 via sgrep)
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch
  einen sichtbaren Lauf.
- **Braucht:** der **Operator** startet `bin/omegaflow` in eigenem Ermessen; dann
  am Pixel die URL, `/consent`=1; messen `sensor: N samples` >0 + Vibration.

#### Akt: Sensor-Hardware beschaffen/anschließen
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft und schließt die Sensor-Hardware an.
- **Lage:** der Mantis-Shrimp-Node und die HRV-Teile (BOM) sind nicht beschafft;
  das Inventar steht in `survey-2026-09-23-geraete-anbindung-radiatoren.md`.
  (gemessen 2026-09-23)
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session baut den Node bei gemessenem
  Vorhandensein.

### Stufe 3 — blockiert

keiner.

### Stufe 4 — wartend

#### Sonnenfarbe: GPU-Live-Parity + CI am neuen Commit (Bau steht)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check` des neuen Commits ist gelaufen (nach `/commit`+Push).
- **Lage:** der Farbmodus ist gebaut (Rat 2026-09-24): Kanaltrennung (Helligkeit
  immer `t2(|val|)`, nur der Farbton wechselt), Operator-Toggle `c`, Default
  `field`; die Rust-LUT über den Relay-Endpunkt `/color_lut` aus
  `spectral::color_lut_wire()` (`spectral.rs:445`), Parser `static/constants.js`,
  Shader-Binding(3) + Modus in `vp.expose.y` (`static/index.html`); Statuszeile;
  Parity-Gate `gpu_lut_index_mirror_matches_color_for_ci` (`spectral.rs:779`) +
  `static/color_lut.test.mjs`; `cargo check --all-targets --features browser_relay`
  0/0. **Ungemessen:** die WGSL-Ausführung (Shader-Validierung + LUT-Lesen) — kein
  GPU-Lauf lokal. (gemessen 2026-09-24)
- **Blockade:** Heavy compute (GPU-Lauf) — Regel: CI/Operator-Wort.
- **Braucht:** `/commit`+Push → `ci-check` (die neuen Rust-/JS-Tests) und
  `gh workflow run ci-check.yml`; dann der erste sichtbare/hidden Lauf, der
  `color: measured` rendert.

#### Beat-Arbitrierung: funktionale Verifikation im Betrieb
- **Status:** wartend | **Bindung:** eigen (hardware/versteckter Lauf)
- **Trigger:** ein Beat-Quellen-Satz ist am Host gesetzt (Serial-Gerät +
  `OMEGAFLOW_BLE_HR`/`FIT_DIR`) und ein sichtbarer/hidden Lauf steht.
- **Lage:** die Spawn-Arbitrierung steht (`main_flow.rs:504`), aber kein Lauf hat
  die Verdict-Zeile (`beat source: …`) gemessen erzeugt; `tests.rs` deckt nur die
  reine Funktion. (gemessen 2026-09-24)
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — eine Live-Messung braucht
  CI oder das Operator-Wort.
- **Braucht:** `cargo test` in CI (Funktion) bzw. ein versteckter Lauf mit zwei
  gesetzten Quellen, der genau eine `beat source:`-Zeile zeigt.

#### 945-FIT-Datei (Onboard nur FIT/CIQ)
- **Status:** wartend | **Bindung:** eigen (Ein-Quellen-Regel)
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad (direkt vom FR945-Speicher /
  CIQ-App) wird gemessen nötig.
- **Lage:** der Host-Reader ist gemessen — **sensory** hält `parse_fit`
  (`src/archivar/fit.rs:78`), verdrahtet in `main_flow.rs`; CIQ hat **keinen**
  Reader (`sgrep -i ciq src tools` = leer); ein Onboard-Reader ist nicht gebaut.
  River arbitriert die Beat-Quellen zentral (`main_flow.rs`); `OMEGAFLOW_SERIAL_IN`
  steht (`src/archivar/ingress.rs:4`, `ble.rs:931`). (gemessen 2026-09-24 via
  sgrep)
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** erste Messung bleibt: wird ein Onboard/CIQ-Pfad gebraucht? Sonst
  bleibt `FIT_DIR` der FIT-Kanal; BLE (`OMEGAFLOW_BLE_HR`) und Serial sind über die
  Arbitrierung ausgeschlossen, solange FIT läuft.

#### Deredden wartet auf das E(B−V)-Asset (Bayestar19)
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** das Bayestar19-Asset ist im CDN manifestiert und `bayestar.rs` grün.
- **Lage:** die Kette ist **im Code gebaut** (mycelium folge150 `50f2bdee`):
  `Buffer.bayestar` (`spatial.rs:38`), Loader `main_flow.rs:2698` (`format ==
  "bayestar"`, `load_map` `:2745`), `sightline_ebv` (`membrane.rs:133`); registriert
  in `phi/sources.φ:10000` (`format bayestar` `:10001`, `compiler
  bayestar_compiler` `:10003`). **Aber:** das CDN-Asset `…/dataverse.harvard.edu/
  bayestar2019.be19` ist **404** (gemessen 2026-09-24, `archive_search --verdict`,
  alle 3 Stufen), und `bayestar.rs:352` trägt `chunks_exact` (clippy) + ein Test rot.
  River-seitig ist nichts offen.
- **Blockade:** das Asset fehlt und das Modul ist rot.
- **Braucht:** mycelium manifestiert das Asset (`bayestar_compiler` →
  `phi/sources.φ` → CI-CDN) und heilt `bayestar.rs:352` (`as_chunks`) + den
  Leaf-Test.

#### TLS im Relay für kabellose Sensorik
- **Status:** wartend | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage:** für jetzt verworfen (Rat 2026-09-23): `rustls`/`native-tls` = neue
  C-Dependency im Kern (folge13-Lehre). (gemessen 2026-09-23)
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

### Stufe 5 — termin

#### Sensor-Bindung vC-Permeabilität (Smartwatch + Mantis-Shrimp)
- **Status:** termin | **Bindung:** operator (Hardware/versteckter Lauf)
- **Trigger:** Smartwatch + Mantis-Shrimp-Sensoren sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage:** der Permeabilitäts-Pfad steht (vC → `field_permeability` in
  `src/mathematikerin/omega.rs:200/349`); der HRV-Reader ist gebaut
  (`src/archivar/ble.rs:715/926`, external-state Zeile BLE-HR-Reader);
  `handover-2026-09-20-operator-entscheidungen.md:26` hält den `termin`.
  (gemessen 2026-09-24 via sgrep)
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo
  run --release` aus, dann `perm_target_probe --live <pfad>`.

### Stufe 6 — LOCK

keiner.

## Geroutet / fremd (nicht river)

- **Bayestar19-Asset 404 + `bayestar.rs`-Red** — `post.md` an **mycelium** (der
  Deredden-Input; mycelium folge150 `50f2bdee`).
- **dropped-gate** (delta 199 @`0c7682916`) — `post.md` an **mountain**
  (Zähler-Wurzel) hält; Baseline in `docs/zustand/dropped-baseline.md`.
- **`archivar::ble`-Test-Red** (sensory) — durch `fdb4f9f27` geheilt.
- **`post.md`-Zeilen** an mountain, mycelium und sensory — fremde Hunks, nicht
  angefasst.

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
- `docs/handover/post.md` (eigener Hunk: die Zeile „An river" gefaltet und gelöscht,
  Header-sha256 nachgezogen)
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
