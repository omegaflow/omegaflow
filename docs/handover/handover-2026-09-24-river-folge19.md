<!--
  title: Handover — River-Folge 19 (Stand 2026-09-24)
  session: river
  class: handover
  date: 2026-09-24
  sha256: 04cd40ab734125b4f6dae01b0cbee8665017412f275b4ae14d66be16235c1a47
  status: live
-->
# Handover — River-Folge 19 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Anlass: folge18 wird gefaltet. Die Session misst am HEAD `dbe254cd` (Sensory
folge156) und hält jeden Punkt gegen den Baum. Der folge18-`ci-check`
`35924695548` @`0c7682916` ist **failure** — aber **fremd**: das `clippy`-Job ist
grün (rivers Heilung hält), rot sind `dropped-gate` (delta 199, Baseline mountain)
und `test` (10× `archivar::ble`, sensory — durch `fdb4f9f27` @HEAD geheilt). Die
folge18-Claims im echten Code verifiziert: `record_in_enclosure` 6-Arg-Struct-Form
(`fetch.rs:480`), `as_chunks` + Positiv-Gate (`dispersion.rs:71/74/181`),
`passband_at` ohne Borrow (`spectral.rs:673`). **Gebaut:** die Beat-Quellen-
Arbitrierung (Rat-Verdikt 2026-09-24) — ein Punkt in `main_flow.rs`, totale
Ordnung `serial > BLE > FIT`, Battery orthogonal; `cargo check --all-targets
--features browser_relay` 0/0.

## Stehender Pass (gemessen 2026-09-24)

- **Postfach** — `state/mail/`/`mail_ledger.φ` in diesem Baum absent, `mail_digest`
  pending (CI-Bau); keine Mail. Die Postfach-Zeile in `external-state.md` bleibt.
- **CI** — HEAD `dbe254cd` (Sensory folge156). folge18-`ci-check` `35924695548`
  @`0c7682916` **failure** (clippy grün; `dropped-gate` delta 199 + `test` 10×
  `archivar::ble`, beide fremd — ble durch `fdb4f9f27` geheilt). mycelium
  `50f2bdee` rot: `clippy bayestar.rs:352` (`chunks_exact`) + `test` 11 Fehler
  (u. a. `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel`).
  Jüngste Läufe: `ci-check 35971097226` in_progress @`fdb4f9f2`, `35971147236`
  pending @`dbe254cd`; `tools-build 35971097502` success; CDN jüngst success
  (`allwise`/`quake-feeds`/`ned`/`ps1`/`swpc-mirror`).
- **`git_safety --snapshot`** → Arbeitsbaum = HEAD.

## Offen (aufgeschlüsselt)

### CI-Verifikation: Beat-Arbitrierung folge19
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check` des folge19-Commits läuft nach `/commit`+Push aus.
- **Lage (gemessen 2026-09-24):** `select_beat_source`/`BeatSource` in
  `main_flow.rs`, Test `beat_source_precedence_is_total` (`tests.rs`);
  `cargo check --all-targets --features browser_relay` 0/0; nur die CI-Messung
  fehlt.
- **Blockade:** keine.
- **Braucht:** `ci_manage view <id>` bzw. Watchdog-Snapshot im nächsten Pass; grün
  = functional verifiziert.

### Beat-Arbitrierung: funktionale Verifikation im Betrieb
- **Status:** wartend | **Bindung:** eigen (hardware/versteckter Lauf)
- **Trigger:** ein Beat-Quellen-Satz ist am Host gesetzt (Serial-Gerät +
  `OMEGAFLOW_BLE_HR`/`FIT_DIR`) und ein sichtbarer/hidden Lauf steht.
- **Lage (gemessen 2026-09-24):** die Spawn-Arbitrierung steht
  (`main_flow.rs`), aber kein Lauf hat die Verdict-Zeile (`beat source: …`)
  gemessen erzeugt; `test` deckt nur die reine Funktion.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — eine Live-Messung braucht
  CI oder das Operator-Wort.
- **Braucht:** `cargo test` in CI (Funktion) bzw. ein versteckter Lauf mit zwei
  gesetzten Quellen, der genau eine `beat source:`-Zeile zeigt.

### LAN-Sensorik: adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Host-Relay läuft auf `:1618` + Pixel öffnet `http://127.0.0.1:1618`.
- **Lage (gemessen 2026-09-23):** `adb devices` listet `67151JEA305427 device`;
  `adb reverse tcp:1618 tcp:1618` gesetzt (nach adb-Neustart erneut zu setzen);
  Host-Relay läuft nicht; `OMEGAFLOW_HIDDEN=1` erzeugt keinen `TcpRadiator`
  (`main_flow.rs:715` `if !hidden` → `:720`, gemessen 2026-09-24).
- **Blockade:** Heavy compute (CI, nie lokal) — der Relay braucht dennoch einen
  sichtbaren Lauf.
- **Braucht:** der **Operator** startet `bin/omegaflow` in eigenem Ermessen; dann
  am Pixel die URL, `/consent`=1; messen `sensor: N samples` >0 + Vibration.

### Sensor-Bindung vC-Permeabilität (Smartwatch + Mantis-Shrimp)
- **Status:** termin | **Bindung:** operator (Hardware/versteckter Lauf)
- **Trigger:** Smartwatch + Mantis-Shrimp-Sensoren sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage (gemessen 2026-09-23):** der Permeabilitäts-Pfad steht (vC →
  `fieldPermeability` in `src/mathematikerin/omega.rs`); der HRV-Reader ist gebaut
  (`src/archivar/ble.rs`, external-state Zeile BLE-HR-Reader);
  `handover-2026-09-20-operator-entscheidungen.md:26` hält den `termin`.
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo
  run --release` aus, dann `perm_target_probe --live <pfad>`.

### Sensor-Hardware: Mantis-Shrimp Node / ESP32-S3 + HRV-Teile (BOM)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft/an schließt die Sensor-Hardware an.
- **Lage (gemessen 2026-09-23):** `esp32-firmware`-Workflow existiert (CI); der
  Mantis-Shrimp-Node und die HRV-Teile (BOM) sind nicht beschafft/gemessen; das
  Inventar steht in `survey-2026-09-23-geraete-anbindung-radiatoren.md`.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session baut den Node bei gemessenem
  Vorhandensein.

### 945-FIT-Datei (Onboard nur FIT/CIQ)
- **Status:** wartend | **Bindung:** eigen (Ein-Quellen-Regel)
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad (direkt vom FR945-Speicher / CIQ-App)
  wird gemessen nötig.
- **Lage (gemessen 2026-09-24):** der Pfad-Inhaber ist gemessen — **sensory** hält
  den Host-Reader `src/archivar/fit.rs:78` `parse_fit` / `:278` `fit_ingress`
  (`FIT_DIR`), verdrahtet `main_flow.rs`; CIQ hat **keinen** Reader
  (`sgrep -i ciq src tools` = leer); ein Onboard-Reader ist nicht gebaut. River
  arbitriert jetzt die Beat-Quellen zentral (`main_flow.rs`). Der Transporthaken
  `OMEGAFLOW_SERIAL_IN` steht (`ingress.rs:31`).
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** erste Messung bleibt: wird ein Onboard/CIQ-Pfad gebraucht? Sonst
  bleibt `FIT_DIR` der FIT-Kanal; BLE (`OMEGAFLOW_BLE_HR`) und Serial sind über die
  neue Arbitrierung ausgeschlossen, solange FIT läuft.

### TLS im Relay für kabellose Sensorik
- **Status:** pending | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage (gemessen 2026-09-23):** für jetzt verworfen (Rat 2026-09-23): `rustls`/
  `native-tls` = neue C-Dependency im Kern (folge13-Lehre).
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

### Deredden wartet auf das E(B−V)-Asset (Bayestar19)
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** das Bayestar19-Asset ist im CDN manifestiert und `bayestar.rs` grün.
- **Lage (gemessen 2026-09-24):** die Kette ist **im Code gebaut** (mycelium
  folge150 `50f2bdee`): `Buffer.bayestar: Option<Arc<BayestarMap>>`
  (`spatial.rs:38/155`), Loader `main_flow.rs:2698` (`format == "bayestar"`),
  `sightline_ebv` liest die Karte (`membrane.rs:133-147`), `sed_to_bp_rp` nimmt
  `Option<f64>` (`spectral.rs:375`); registriert in `phi/sources.φ:10001`
  (`format bayestar`). **Aber:** das CDN-Asset `…/dataverse.harvard.edu/bayestar2019.be19`
  ist **404** (gemessen 2026-09-24, `archive_search --verdict`, alle 3 Stufen), und
  `bayestar.rs:352` trägt `chunks_exact` (clippy) + ein Test rot. River-seitig ist
  nichts offen.
- **Blockade:** das Asset fehlt und das Modul ist rot.
- **Braucht:** mycelium manifestiert das Asset (`bayestar_compiler` →
  `phi/sources.φ` → CI-CDN) und heilt `bayestar.rs:352` (`as_chunks`) + den
  Leaf-Test.

## Geroutet / fremd (nicht river)

- **Bayestar19-Asset 404 + `bayestar.rs`-Red** — `post.md` an **mycelium** (der
  Deredden-Input; mycelium folge150 `50f2bdee`).
- **dropped-gate** (delta 199 @`0c7682916`, 267 @`50f2bdee`) — `post.md` an
  **mountain** (Zähler-Wurzel) hält; Baseline in `docs/zustand/dropped-baseline.md`.
- **`archivar::ble`-Test-Red** (sensory) — durch `fdb4f9f27` @HEAD geheilt.
- **`post.md`-Zeilen** an future (SuperDARN-Globus-Transfer, Sicherheits-Befund) und
  mountain (dropped-Zähler) — fremde Hunks, nicht angefasst.

## Benchmark

- **CI-Red-Diagnose folge18:** `general` (flash), ein Lauf — `ci_manage log/view`
  + Pfad-Eigentum via `git log`; Ergebnis: beide Reds fremd (dropped-gate
  mountain/mycelium, ble sensory), clippy grün.
- **945-FIT + Bayestar-Messung:** `general` (flash) ×2 + `explore` (flash) ×1, drei
  Läufe — Ergebnis: `fit.rs` Host-Reader (sensory), CIQ leer; Bayestar end-to-end
  gebaut von mycelium, Asset 404.
- **Beat-Quellen-Arbitrierung (Architektur):** `council` (pro/max), ein Lauf —
  Verdikt: ein Arbitrierungspunkt am Spawn-Ort, totale Ordnung `serial > BLE > FIT`,
  Battery orthogonal, FIT-Temperatur-Absenz null-echt.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/main_flow.rs` (`BeatSource`, `select_beat_source`, Spawn-Arbitrierung)
- `src/archivar/tests.rs` (`beat_source_precedence_is_total`)
- `docs/handover/post.md` (eigene Hunks: „An mycelium" Bayestar)
- `docs/handover/handover-2026-09-24-river-folge19.md` (neu; folge18 → `archiv/`)
- **lokal, gitignored (nicht im Commit):** `docs/zustand/external-state.md` — CI-Zeile

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
