<!--
  title: Handover — River-Folge 23 (Stand 2026-09-25)
  session: river
  class: handover
  date: 2026-09-25
  sha256: 9dc928f51107d66c7a4144db29ee49529ec7989bdc760e09a907cd99904e450a
  status: live
-->
# Handover — River-Folge 23 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Anlass: folge22 wird gefaltet. In diesem Atom gebaut (git trägt): der `tcurve`-Job
des `te-ncurve`-Workflows ist shardbar geworden — `pcmci_class_benchmark` nimmt
`--t a:b` (inklusive Bereich, analog `--r a:b`), der T-Curve-Lauf filtert danach;
der Workflow fährt `tcurve` als Matrix aus vier Segmenten (400:450 / 500:550 /
600:650 / 700:700) mit Cap 180 min statt eines 360-min-Blocks. `cargo check -p
omegaflow-measure --bin pcmci_class_benchmark` 0/0; Syntax-Gate `cargo fmt` auf den
eigenen Pfaden. Beginnt am HEAD `0a0ce96d8`, `origin/main` == HEAD.

## Stehender Pass (gemessen 2026-09-25)

- **HEAD** zu Session-Beginn `0a0ce96d8` (Sensory folge160) == `origin/main`,
  Arbeitsbaum sauber (gemessen 2026-09-25 via `git rev-parse HEAD`); während der
  Session landete fremde Arbeit — HEAD jetzt `924577cdd`, u. a. eine sensory-
  Heilung an `src/archivar/ble.rs` (dem gemessenen roten Test). Eigene
  uncommittete Arbeit: `te-ncurve.yml`, `pcmci_class_benchmark.rs`, folge23.
- **Postfach** — `state/mail/` liegt im privaten Repo, lokal nicht gemountet;
  `mail_digest` meldet `ledger absent: state/mail/mail_ledger.φ` → Postfach für
  diese Session **pending** (keine River-relevante Mail messbar). Die externen
  Eingänge führt `docs/zustand/external-state.md`.
- **CI** — `ci-check 36065950583` @`0a0ce96d8` **failure** (gemessen 2026-09-25 via
  `ci_manage list`/`ci_manage log`): 1609 passed, **2 failed** —
  `archivar::ble::tests::gfdi_records_reads_a_5044_response_frame` +
  `…file_list_response_fields` (`src/archivar/ble.rs:1688/1701`) = **sensory
  (GFDI/BLE)**, nicht River; die River-Parity-Gates des Farbmodus liefen grün.
  `health-check 36089944258` in_progress; `te-ncurve 36052804297` @`57e2ba3c8`
  **completed/cancelled** (~6 h = 360-Cap) → tcurve sharden in diesem Atom gebaut;
  `tools-build 36065950598` success; CDN-Läufe success.
- **`git_safety --snapshot`** → Arbeitsbaum == HEAD, nichts zu sichern.
- **`open_points_check` folge22** → 15 Pfad-Refs, 0 absent, 0 format-gaps.
- **Fremde uncommittete Arbeit** (gemessen 2026-09-25 via `git status`): keine.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

keiner.

#### Stufe 2 — operator-gebunden

### Akt: LAN-Sensorik adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (gemessen 2026-09-24 via sgrep/read am HEAD
  `f417cdd69`): der Relay ist nur unter Feature `browser_relay` kompiliert
  (`Cargo.toml:25`, Gate `main_flow.rs:713`); `TcpRadiator` wird bei
  `main_flow.rs:720` gebaut, **nur** wenn nicht hidden; `hidden =
  env("OMEGAFLOW_HIDDEN").is_ok()` (`main_flow.rs:612`) — **jeder** Wert (auch
  `0`/leer) unterdrückt den Relay, nur *unset* lässt ihn laufen; Bind
  `relay_bind_addr()` = `RELAY_BIND_DEFAULT "0.0.0.0"` (`relay.rs:11/21/73`), nicht
  Loopback; Startzeile `serving on http://{bind}:{port}` (`relay.rs:75`); Diagnose
  `sensor: {} samples` (`main_flow.rs:60`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch
  einen sichtbaren Lauf.
- **Braucht:** der **Operator** führt aus:
  ```sh
  adb devices && adb reverse tcp:1618 tcp:1618   # nach jedem adb-Neustart erneut
  bin/omegaflow                                   # OHNE OMEGAFLOW_HIDDEN (sichtbar)
  #   stderr:  serving on http://0.0.0.0:1618
  ```
  dann `http://127.0.0.1:1618` öffnen, Consent bestätigen (`/consent?ja`,
  `relay.rs:396`), messen: `sensor: N samples` mit N > 0 **und** Vibration
  (`navigator.vibrate`, `static/radiator.js:105`).

### Akt: Sensor-Hardware beschaffen/anschließen
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft und schließt die Sensor-Hardware an.
- **Lage:** Node und HRV-Teile (BOM) sind nicht beschafft (gemessen 2026-09-23 via
  sread); die `1N4007`-Freilaufdiode (M8, Pflicht) steht als BOM-Position, Preis
  `pending` (gemessen 2026-09-24 via `archive_search --all`); Artefakt
  `docs/specs/mantis-shrimp-bom.md` + `…-build.md §5`.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session baut den Node bei gemessenem
  Vorhandensein.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

### Sonnenfarbe: GPU-Live-Parity + erster Lauf (Bau steht)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein sichtbarer/hidden Lauf rendert `color: measured`.
- **Lage:** (gemessen 2026-09-24 via sgrep) der Farbmodus ist gebaut (Rat
  2026-09-24, Commit `2de359982`, Ancestor von HEAD): Kanaltrennung (Helligkeit
  `t2(|val|)`, nur der Farbton wechselt), Operator-Toggle `c`, Default `field`;
  die Rust-LUT über den Relay-Endpunkt `/color_lut` aus `spectral::color_lut_wire()`
  (`spectral.rs:445`); Parser `static/constants.js`, Shader-Binding(3) + Modus in
  `vp.expose.y` (`static/index.html`); Parity-Gate
  `gpu_lut_index_mirror_matches_color_for_ci` (`spectral.rs:780`) +
  `static/color_lut.test.mjs`. **Der zugehörige `ci-check` ist gelaufen** — die
  River-Parity-Gates grün; das rote `ci-check 36065950583` @HEAD trägt allein zwei
  **fremde** sensory-Tests (`ble.rs`). **Ungemessen:** die WGSL-Ausführung im
  sichtbaren Lauf (Shader-Validierung + LUT-Lesen) — kein GPU-Lauf lokal.
- **Blockade:** Heavy compute (GPU-Lauf) — Regel: CI/Operator-Wort.
- **Braucht:** der erste sichtbare/hidden Lauf, der `color: measured` rendert
  (Operator-Wort), zusammen mit der adb-reverse-Route oben.

### te-ncurve sharden — Verifikation des Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-ncurve`-Lauf am neuen Commit endet.
- **Lage:** das Sharden ist gebaut (gemessen 2026-09-25 via read/sgrep): `--t a:b`
  in `tools/measure/src/bin/pcmci_class_benchmark.rs`, Filter im `TCURVE_ONLY`-Block;
  `te-ncurve.yml` `tcurve`-Job als Matrix (400:450 / 500:550 / 600:650 / 700:700),
  Cap 180 min; `cargo check` 0/0. Der neue Lauf ist noch nicht gelaufen.
- **Blockade:** keine — wartet auf den Lauf (der Trigger).
- **Braucht:** nach Commit+Push `gh workflow run te-ncurve.yml`; dann
  `ci_manage view <id>` — jeder Segment-Job success, kein 180-Cap-Abbruch.

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
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad wird gemessen nötig.
- **Lage:** der Host-Reader ist gemessen (gemessen 2026-09-24 via sgrep) —
  sensory hält `parse_fit` (`src/archivar/fit.rs:78`), verdrahtet in `main_flow.rs`;
  CIQ hat **keinen** Reader (`sgrep -i ciq src tools` = leer); ein Onboard-Reader ist
  nicht gebaut; River arbitriert die Beat-Quellen zentral (`main_flow.rs`);
  `OMEGAFLOW_SERIAL_IN` steht (`src/archivar/ingress.rs:4`, `ble.rs:931`).
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
- **Lage:** (gemessen 2026-09-23 via sread) die präregistrierte JUICE-Erdpassage-Kette
  wartet auf das Perigäum; `docs/auftrag/auftrag-flyby2-kette.md` (Owner
  Forschung-Linie); RTSW-Retention (1 m mag/wind) **~24 h** — harte Frist: der
  erste Fill-Run muss ≤24 h nach der ersten Perigäum-Zelle starten, sonst ist die
  1-m-Kette der frühen Stunden nicht mehr messbar; Siegel + σ-Metrik stehen.
- **Blockade:** keine — wartet auf das Perigäum (der Trigger).
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle (RTSW/ACE Minuten,
  Kp ≤3 h, Swarm ≤1 d; je Messwert `source`+`active`).

#### Stufe 6 — LOCK

keiner.

## Benchmark

- **te-ncurve sharden:** `build` (line), ein Lauf — `--t a:b`-Range + Workflow-Matrix,
  `cargo check -p omegaflow-measure --bin pcmci_class_benchmark` 0/0. Routine-Bau;
  kein pro/max.
- **ci-check-Rot klassifizieren:** `build`, ein Lauf — `ci_manage log 36065950583`
  las Job-Ebene: 1609 passed, 2 failed (sensory `ble.rs`), River-Parity grün. Kein
  Doppel-Lauf.
- Alle Läufe flash-Tier; kein pro/max in diesem Atom.

## Geteilter Baum — eigener Pfad-Satz

- `.github/workflows/te-ncurve.yml` (`tcurve`-Matrix + Cap 180)
- `tools/measure/src/bin/pcmci_class_benchmark.rs` (`--t a:b`-Range)
- `docs/handover/handover-2026-09-25-river-folge23.md` (neu; folge22 → `archiv/`)
- **untracked, nicht im Commit:** `docs/zustand/external-state.md` (seit
  `7cd9aedd1` gitignored — die CI-Status-Zeile ist auf der Platte nachgezogen)
- **fremd, unberührt (nicht im Commit):** `src/archivar/ble.rs` (sensory GFDI, rot
  @HEAD `0a0ce96d8` — via `ci_manage log`; bereits durch den Health-Issue-Kanal
  getragen)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
