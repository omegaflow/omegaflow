<!--
  title: Handover — River-Folge 20 (Stand 2026-09-24)
  session: river
  class: handover
  date: 2026-09-24
  sha256: c51cc58edca54c1b93e7573c3362cb0030229559e0eae6bb0e5b1590dfada7d6
  status: live
-->
# Handover — River-Folge 20 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Anlass: folge19 wird gefaltet. Die Session misst am HEAD `15ca40c46` und hält
jeden Punkt gegen den Baum. Neu aufgenommen: die Post-Zeile „An river" (Sonnenfarbe,
`post.md:11`) — sie war in folge19 nicht gefaltet. Der Arbeitsbaum trägt fremde
Änderungen (`opencode.json`, `src/archivar/port.rs`), nicht angefasst.

## Stehender Pass (gemessen 2026-09-24)

- **Postfach** — `state/mail/`/`mail_ledger.φ` in diesem Baum absent, `mail_digest`
  pending (CI-Bau); keine Mail. Die Postfach-Zeile in `external-state.md` bleibt.
- **CI** — HEAD `15ca40c46`, `origin/main` == HEAD. River folge19-Commit
  `54a8f8aef` (@10:03) ist Vorfahr; sein `ci-check` `35973038431` = **failure**
  (attempt 1, `head_sha 54a8f8aef`, `ci_manage view` 2026-09-24), Job-Ebene offen.
  Jüngste `ci-check`-Läufe am HEAD noch in_progress/pending (`35998482967`,
  `36000037458`); `tools-build`/CDN jüngst success.
- **`git_safety --snapshot`** → `refs/safety/1790253450`; Arbeitsbaum = HEAD + 2
  fremde `M` (opencode.json, port.rs).
- **`open_points_check` folge19** — 14 Pfad-Refs, 1 absent
  (`state/mail/mail_ledger.φ`, Postfach, erwartet); kein stale Punkt.

## Offen (aufgeschlüsselt)

### Stufe 1 — autonom

#### Sonnenfarbe = gemessene Farbe
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (sofort handlungsfähig).
- **Lage:** der Membran-Render färbt jeden Punkt nach dem Feldwert `val`
  (`static/index.html:209-216`); `color_for_ci` (`src/archivar/spectral.rs:421`,
  `meta[10]`) liegt auf dem Draht, im Render ungenutzt — gelesen nur im Aktuator
  (`src/mathematikerin/actuators.rs:210`). (gemessen 2026-09-24 via sread/sgrep)
- **Blockade:** keine.
- **Braucht:** Render-Shader liest `props[id*4+2].z` (meta[10]) und mappt über
  eine WGSL-Color-LUT (die Rust-LUT `color_lut_rgba` portieren); die Farbsemantik
  (gemessene Farbe vs Feldwert = Operator-Gaze) vorab an den `council`.

### Stufe 2 — operator-gebunden

#### LAN-Sensorik: adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Host-Relay läuft auf `:1618` + Pixel öffnet `http://127.0.0.1:1618`.
- **Lage:** `adb devices` listet `67151JEA305427 device`; `adb reverse tcp:1618
  tcp:1618` gesetzt (nach adb-Neustart erneut zu setzen); Host-Relay läuft nicht;
  `OMEGAFLOW_HIDDEN=1` erzeugt keinen `TcpRadiator` (`main_flow.rs:715` `if
  !hidden` → `:720`, gemessen 2026-09-24 via sgrep).
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch
  einen sichtbaren Lauf.
- **Braucht:** der **Operator** startet `bin/omegaflow` in eigenem Ermessen; dann
  am Pixel die URL, `/consent`=1; messen `sensor: N samples` >0 + Vibration.

#### Sensor-Hardware: Mantis-Shrimp Node / ESP32-S3 + HRV-Teile (BOM)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft/an schließt die Sensor-Hardware an.
- **Lage:** `esp32-firmware`-Workflow existiert (CI); der Mantis-Shrimp-Node und
  die HRV-Teile (BOM) sind nicht beschafft/gemessen; das Inventar steht in
  `survey-2026-09-23-geraete-anbindung-radiatoren.md`. (gemessen 2026-09-23)
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session baut den Node bei gemessenem
  Vorhandensein.

### Stufe 3 — blockiert

keiner.

### Stufe 4 — wartend

#### CI-Verifikation: Beat-Arbitrierung folge19
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check` des folge19-Commits ist gelaufen (`54a8f8aef`).
- **Lage:** `select_beat_source`/`BeatSource` (`main_flow.rs:4/11/504`), Test
  `beat_source_precedence_is_total` (`tests.rs:9`); folge19-`ci-check`
  `35973038431` @`54a8f8aef` = **failure** (attempt 1), Job-Ebene noch nicht
  gelesen (gemessen 2026-09-24 via `ci_manage view`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 35973038431` — ist der River-Job (clippy) grün und
  der Rest fremd? grün = functional verifiziert.

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
- **`post.md`-Zeilen** an future und mountain — fremde Hunks, nicht angefasst.

## Benchmark

- **Planungs-Pass folge19/20:** `ci_manage view` (CI-Verdikt des River-Commits) +
  `sread`/`sgrep` (Post-Zeile `An river` gegen `index.html`/`spectral.rs` gemessen)
  — flash-Tier, kein pro/max.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-24-river-folge20.md` (neu; folge19 → `archiv/`)
- `docs/handover/post.md` (eigener Hunk: die Zeile „An river" gefaltet und gelöscht)
- **lokal, gitignored (nicht im Commit):** `docs/zustand/external-state.md` — CI-Zeile

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
