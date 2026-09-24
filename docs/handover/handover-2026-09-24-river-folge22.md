<!--
  title: Handover — River-Folge 22 (Stand 2026-09-24)
  session: river
  class: handover
  date: 2026-09-24
  sha256: f84b6b7303fa4d5d9d7a60dd3510e53bf8b58aa7d94e3b9a0206ff4c8734205b
  status: live
-->
# Handover — River-Folge 22 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Anlass: folge21 wird gefaltet. In diesem Atom erledigt (git trägt): der
`spectral.rs:752`-clippy (`needless_range_loop` → index-freies `zip`), die
`1N4007`-Freilaufdiode als BOM-Position (Preis `pending`), die Bind-Adress-Korrektur
der Radiator-Survey (`0.0.0.0` per `relay_bind_addr`, nicht Loopback) und die
verifizierte `adb reverse`-Kantenzeile (in Stufe 2 gefaltet). Beginnt am HEAD
`f417cdd69`, `origin/main` == HEAD.

## Stehender Pass (gemessen 2026-09-24T21:14Z)

- **Postfach** — `state/mail/mail_ledger.φ` existiert; `mail_digest --last 6` zeigt
  6 Eingänge (Rubin/LSST, STScI, GitHub-Support-Purge-Antwort ×3); keine neue
  River-relevante Mail.
- **CI** — HEAD `f417cdd69` (`origin/main` == HEAD). `te-ncurve` `36052804297`
  **in_progress** @`57e2ba3c8`; `health-check` `36027534328` in_progress; letzte
  `ci-check` (`36030755250` @`e05f8a419`) **failure** — clippy `bayestar.rs:352` +
  `spectral.rs:752` (`needless_range_loop`), Tests `bayestar.rs:494` +
  `ble.rs:1116`; **fremd** (bayestar/ble = mycelium), nur `spectral.rs:752` ist
  River (in diesem Atom gebaut).
- **`git_safety --snapshot`** → „the working tree equals HEAD — nothing to record".
- **`open_points_check` folge21** — 18 Pfad-Refs, 0 absent, 0 format-gaps.
- **Fremde uncommittete Arbeit im geteilten Baum** (gemessen `git status`):
  `bayestar.rs`, `fit.rs`, `commit_gate.rs`, `smail.rs`, `open_points_check.rs`,
  `bayestar-cdn.yml`, `phi/sources.φ` — nicht River, unberührt.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

keiner.

#### Stufe 2 — operator-gebunden

### Akt: LAN-Sensorik adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (gemessen 2026-09-24 via sgrep/read am HEAD
  `f417cdd69`): der Relay ist nur unter Feature
  `browser_relay` kompiliert (`Cargo.toml:25`, Gate `main_flow.rs:713`);
  `TcpRadiator` wird bei `main_flow.rs:720` gebaut, **nur** wenn nicht hidden;
  `hidden = env("OMEGAFLOW_HIDDEN").is_ok()` (`main_flow.rs:612`) — **jeder** Wert
  (auch `0`/leer) unterdrückt den Relay, nur *unset* lässt ihn laufen; Bind
  `relay_bind_addr()` = `RELAY_BIND_DEFAULT "0.0.0.0"` (`relay.rs:11/21/73`),
  nicht Loopback; Startzeile `serving on http://{bind}:{port}` (`relay.rs:75`);
  Diagnose-Literal `sensor: {} samples` (`main_flow.rs:60`); die adb-Brücke lebt nur
  in den Handover-Docs, nicht im Code.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch
  einen sichtbaren Lauf.
- **Braucht:** der **Operator** führt aus:
  ```sh
  adb devices && adb reverse tcp:1618 tcp:1618   # nach jedem adb-Neustart erneut
  bin/omegaflow                                   # OHNE OMEGAFLOW_HIDDEN (sichtbar)
  #   stderr:  serving on http://0.0.0.0:1618
  ```
  dann am Pixel `http://127.0.0.1:1618` öffnen, Consent bestätigen (`/consent?ja`,
  `relay.rs:396`), messen: Statuszeile `sensor: N samples` mit N > 0 **und**
  Vibration (`navigator.vibrate`, `static/radiator.js:105`).

### Akt: Sensor-Hardware beschaffen/anschließen
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft und schließt die Sensor-Hardware an.
- **Lage:** Node und HRV-Teile (BOM) sind nicht beschafft (gemessen 2026-09-23 via
  sread); die `1N4007`-Freilaufdiode (M8, Pflicht) steht jetzt als BOM-Position,
  Preis `pending` (gemessen 2026-09-24 via `archive_search --all`); Artefakt
  `docs/specs/mantis-shrimp-bom.md` + `…-build.md §5`.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session baut den Node bei gemessenem
  Vorhandensein.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

### Sonnenfarbe: GPU-Live-Parity + CI am neuen Commit (Bau steht)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check` des neuen Commits ist gelaufen (nach `/commit`+Push).
- **Lage:** der Farbmodus ist gebaut (Rat 2026-09-24, Commit `2de359982`) (gemessen
  2026-09-24 via sgrep): Kanaltrennung (Helligkeit `t2(|val|)`, nur der Farbton
  wechselt), Operator-Toggle `c`, Default `field`; die Rust-LUT über den
  Relay-Endpunkt `/color_lut` aus `spectral::color_lut_wire()` (`spectral.rs:445`),
  Parser `static/constants.js`, Shader-Binding(3) + Modus in `vp.expose.y`
  (`static/index.html`); Parity-Gate `gpu_lut_index_mirror_matches_color_for_ci`
  (`spectral.rs:780`) + `static/color_lut.test.mjs`; `cargo check --all-targets
  --features browser_relay` 0/0. **Ungemessen:** die WGSL-Ausführung
  (Shader-Validierung + LUT-Lesen) — kein GPU-Lauf lokal.
- **Blockade:** Heavy compute (GPU-Lauf) — Regel: CI/Operator-Wort.
- **Braucht:** `ci-check` des neuen Commits (die neuen Rust-/JS-Tests) + `gh
  workflow run ci-check.yml`; dann der erste sichtbare/hidden Lauf, der
  `color: measured` rendert.

### Beat-Arbitrierung: funktionale Verifikation im Betrieb
- **Status:** wartend | **Bindung:** eigen (hardware/versteckter Lauf)
- **Trigger:** ein Beat-Quellen-Satz ist am Host gesetzt (Serial-Gerät +
  `OMEGAFLOW_BLE_HR`/`FIT_DIR`) und ein sichtbarer/hidden Lauf steht.
- **Lage:** die Spawn-Arbitrierung steht (`main_flow.rs:504`) (gemessen 2026-09-24
  via sgrep), aber kein Lauf hat die Verdict-Zeile (`beat source: …`) gemessen
  erzeugt; `tests.rs` deckt nur die reine Funktion.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — eine Live-Messung braucht CI
  oder das Operator-Wort.
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
  2026-09-24T21:14Z via ci_manage view); Cap auf 360 min angehoben, Vorgänger
  brachen am 300-Cap ab.
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
  HRV-Reader ist gebaut (`src/archivar/ble.rs:715/926`);
  `handover-2026-09-20-operator-entscheidungen.md:26` hält den `termin`.
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo
  run --release` aus, dann `perm_target_probe --live <pfad>`.

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** die präregistrierte JUICE-Erdpassage-Kette wartet auf das Perigäum (gemessen 2026-09-23 via
  sread); `docs/auftrag/auftrag-flyby2-kette.md` (Owner
  Forschung-Linie); RTSW-Retention (1 m mag/wind) **~24 h** — harte Frist: der
  erste Fill-Run muss ≤24 h nach der ersten Perigäum-Zelle starten, sonst ist die
  1-m-Kette der frühen Stunden nicht mehr messbar; Siegel + σ-Metrik stehen.
- **Blockade:** keine — wartet auf das Perigäum (der Trigger).
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle (RTSW/ACE Minuten,
  Kp ≤3 h, Swarm ≤1 d; je Messwert `source`+`active`).

#### Stufe 6 — LOCK

keiner.

## Benchmark

- **spectral.rs:752 clippy:** `grind-flash`, ein Lauf — index-freies `zip`
  (`lut[i][..3].iter().zip(lut[i+1][..3].iter())`), `cargo check --all-targets
  --features browser_relay` 0/0. Routine-Fix; kein pro/max.
- **adb-Kantenzeile verifizieren:** `general` (flash), ein Lauf — alle Aussagen
  gegen `file:line` belegt; deckte die stale „nur Loopback"-Behauptung der Survey
  auf (Bind ist `0.0.0.0`) und den `OMEGAFLOW_HIDDEN`-Fußangel (jeder Wert
  unterdrückt den Relay).
- **BOM `1N4007`:** `grind-flash`, ein Lauf — Position im exakten Tabellenformat,
  Preis `pending` (kein AliExpress-Treffer gemessen), sha256-Header aktualisiert.
- **Survey-Korrektur:** `line`-Agent — Bind-Adresse `0.0.0.0` (§Code), sha256 neu.
- Alle Läufe flash-Tier; kein Doppel-Lauf, kein pro/max in diesem Atom.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-24-river-folge22.md` (neu; folge21 → `archiv/`)
- `src/archivar/spectral.rs` (`max_adjacent_lut_delta` index-frei)
- `docs/specs/mantis-shrimp-bom.md` (`1N4007`-Position + sha256)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (Bind-Adresse + sha256)
- **fremd, unberührt (nicht im Commit):** `bayestar.rs`, `fit.rs`, `commit_gate.rs`,
  `smail.rs`, `open_points_check.rs`, `bayestar-cdn.yml`, `phi/sources.φ`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
