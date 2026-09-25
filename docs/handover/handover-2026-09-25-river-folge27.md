<!--
  title: Handover — River-Folge 27 (2026-09-25)
  session: River-Folge 27
  class: handover
  date: 2026-09-25
  sha256: 7037d12cc0584fdd9cdf33710aec649986bc2ce5750e37cfb5ee46951826a337
  status: live
-->
# Handover — River-Folge 27 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch
(Messdatum)**. **Vorbereitung ≠ Akt:** ein operator-gebundener Punkt wird
getrennt geführt — Vorbereitung (autonom, dispatcht) und Akt (operator-gebunden).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic-causal-driver — Neu-Messung nach Härtung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gh workflow run bz-retro-probe.yml` (Dispatch nach dem Push dieses
  Commits).
- **Lage:** (gemessen 2026-09-25 via `git diff`/`cargo check`/`sgrep`) die 7
  Lücken der folge26 sind im Baum geschlossen: Lag-Sweep `0..=LAG_MAX_H` (`=6`)
  (`tools/measure/src/bin/bz_retro_probe.rs`), `const N_SURR = 100` in beiden Bins
  (`bz_retro_probe.rs`, `bz_blatt_probe.rs`), KDE-h-Tabelle `KDE_FACTORS`
  `[0.5..3.0]` über `te_h_null` auf Bz→dB/dt, Median- + Newell-Treiber
  (`bin_cells_median`, `newell_from_cells`, `COMP_BY` in `omni2.rs:7`), Titel/Claims
  auf „directional" gehärtet (`docs/paper/gic-causal-driver.md`, sha256
  `e4cceb3c…`), neuer `minute`-Job in `.github/workflows/bz-retro-probe.yml`;
  `cargo check -p omegaflow-measure --bin bz_retro_probe`/`--bin bz_blatt_probe`
  **0/0**. **Ungemessen:** der Lauf selbst.
- **Blockade:** keine — der schwere Lauf gehört nach CI.
- **Braucht:** `gh workflow run bz-retro-probe.yml`, dann `ci_manage view <id>`;
  Artefakt `bz-blatt-minute.txt` / der hourly-Lauf.

#### GIC-Paarung — co-lokiertes Mäntsälä-dB/dt fehlt
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `sgrep`/`sread`) `fmi_gic.bin` (GIC1,
  `phi/sources.φ:8590`, `src/archivar/geo.rs:7` `MAGIC_GIC`, `:56` `COMP_GIC_A`) ist
  **nicht** mit dB/dt gepaart; die Probe nutzt ABK (68.4 N) / SOD (67.4 N), Mäntsälä
  liegt bei 60.6 N — keine co-lokierte Magnetogramm-Reihe im Stack (GIC-Zeile in
  `bz_blatt_probe.rs`, Paper §6).
- **Blockade:** keine.
- **Braucht:** Mäntsälä ist IMAGE-Station `MAS` — erster Schritt
  `archive_search --supermag MAS` / IMAGE-HAPI `MAS` `PT1M/xyzf`, daraus dB/dt bei
  Mäntsälä, dann Paarung mit `fmi_gic_a`; existiert keine co-lokierte Reihe, den
  Punkt als gemessene Lücke (`descoped`) schließen.

#### Minuten-Archiv — Storm-Ensemble
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Minuten-Retro-Archiv ist verfügbar.
- **Lage:** (gemessen 2026-09-25 via `sread`) `.github/workflows/swpc-mirror-cdn.yml:30`
  spiegelt nur die rollierenden RTSW-Dateien (1 d/7 d); Minuten-Retro fehlt, das
  Minuten-Ensemble ist auf ein 22-h-Fenster begrenzt (Paper §6).
- **Blockade:** Asset fehlt.
- **Braucht:** Harvest eines Minuten-Sonnenwind-Retro-Archivs (SOURCE_PORT,
  Träger Harvest-Linie) — dann Minuten-Ensemble.

#### health-check — voller grüner Shard-Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene health-check-Lauf (`36132614446`
  in_progress @ `93c4bb77`).
- **Lage:** (gemessen 2026-09-25 via `ci_manage view`/`log`) Lauf `36120693487`:
  **6/8 Shards success**, Shards 3+4 extern gecancelt (`The runner has received a
  shutdown signal` — kein Assertions-/Exit-Fehler), Total **2h16m < 3h** → das
  8er-Shard-Ziel (`health-check.yml:31-33`, `timeout-minutes: 240`) ist erreicht. Die
  health-check-Läufe stauen sich (ein neuer Lauf ersetzt den laufenden; `36127486237`
  mit 0 Jobs gecancelt) — es gibt keine Median-Basis, der Watchdog greift nicht.
  Der volle 8/8-Verdikt ist ausstehend.
- **Blockade:** wartet auf das Laufende.
- **Braucht:** `ci_manage view 36132614446`; bei erneutem externen Cancel die
  `concurrency`-Gruppe in `.github/workflows/health-check.yml` prüfen
  (`cancel-in-progress`) — der Stau ist ein eigener Befund.

#### register_lookup — CI-Test des Heading-Keys
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der ci-check-Lauf auf dem Push dieses Commits.
- **Lage:** der Fix (`from_heading`, `register_lookup.rs:1686`/`:1909`, Test
  `:2877`) ist in `522facc53` (River-eigener Commit) **committet**; die folge26
  behauptete „nicht committet / fremde rustfmt-Hunks", was der saubere Baum
  widerlegt (gemessen 2026-09-25 via `git show --stat`/`git status`).
  `cargo test --release -p omegaflow-register` läuft in
  `.github/workflows/ci-check.yml:56`.
- **Blockade:** keine.
- **Braucht:** `ci_manage view <ci-check-run>` — der `test`-Job grün → Punkt löschen;
  rot → den Test `extract_open_points_keys_shared_status_points_by_their_headings`
  lesen.

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kabelloser Sensor wird gebraucht.
- **Lage:** (gemessen 2026-09-25 via `sgrep`/`grind-pro`) kabellos braucht HTTPS
  secure context (`static/sensorium.js:14/88/137`); rustls ist pure Rust → kein
  Kern-Umbau. Der externe Terminator ist gebaut:
  `docs/specs/relay-tls-terminator.md` (Wahl **stunnel** — `0.0.0.0:1619` →
  `127.0.0.1:1618`, Caddy verworfen: ACME braucht eine öffentliche Domain) +
  `bin/relay-tls.stunnel.conf`.
- **Blockade:** keine — ungebaut-gewesen; die geräteseitige CA ist operator-gebunden.
- **Braucht:** Operator: lokale CA + Leaf-Cert (SAN = LAN-IP, `pending`),
  `stunnel bin/relay-tls.stunnel.conf`, dann `bin/omegaflow` ohne
  `OMEGAFLOW_HIDDEN`, `https://<lan-ip>:1619` — Kern bleibt `std+curl+serialport`.

### Operator handelt

#### Sonnenfarbe: erster Lauf `color: measured`
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren/hidden Lauf.
- **Lage:** der Farbmodus ist gebaut (gemessen 2026-09-24 via `sgrep`; Commit
  `2de359982`, Ancestor von HEAD); Parity-Gate `gpu_lut_index_mirror_matches_color_for_ci`
  (`spectral.rs:780`) grün. **Ungemessen:** die WGSL-Ausführung im Lauf.
- **Blockade:** GPU-Lauf = Heavy compute (Regel: CI/Operator-Wort).
- **Braucht:** der erste Lauf, der `color: measured` rendert — zusammen mit der
  adb-reverse-Route.

#### Sensor-Bindung vC-Permeabilität
- **Status:** operator-gebunden (wartet auf Hardware) | **Bindung:** operator
- **Trigger:** Smartwatch + Mantis-Shrimp sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage:** der Permeabilitäts-Pfad steht (`src/mathematikerin/omega.rs:200/349`),
  der HRV-Reader ist gebaut (`src/archivar/ble.rs:715/926`) (gemessen 2026-09-24 via
  `sgrep`).
- **Blockade:** Hardware fehlt.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`,
  dann `perm_target_probe --live <pfad>`.

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (gemessen 2026-09-25 via `sread tools-build.yml`):
  Relay kompiliert im PATH-Bin, Bind `0.0.0.0:1618` (`relay.rs:11/73`),
  `hidden = env("OMEGAFLOW_HIDDEN").is_ok()` (`main_flow.rs:612`), Consent
  `/consent?ja` (`relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal).
- **Braucht:** der **Operator** führt aus: `adb devices && adb reverse tcp:1618
  tcp:1618`, dann `bin/omegaflow` **ohne** `OMEGAFLOW_HIDDEN`; `http://127.0.0.1:1618`
  öffnen, Consent bestätigen, messen: `sensor: N` N>0 **und** Vibration
  (`static/radiator.js:105`).

#### Akt: Sensor-Hardware beschaffen (Bestellung)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator bestellt die BOM und schließt die Hardware an.
- **Lage:** die BOM ist bestellfertig (gemessen 2026-09-25 via
  `archive_search --playwright`, `docs/specs/mantis-shrimp-bom.md`): alle 9
  Outdoor-Positionen mit Item-ID; die EUR-Spalte ist EZB-abgeleitet, nicht direkt
  gemessen.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber); kein Node-Teil vorhanden.
- **Braucht:** Operator bestellt die BOM-Positionen (AliExpress-Login) und liest den
  EUR-Preis gegen; Session baut den Node bei gemessenem Vorhandensein.

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin (Owner Forschung-Linie)
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) die präregistrierte
  JUICE-Erdpassage-Kette wartet auf das Perigäum; RTSW-Retention (1 m mag/wind)
  **~24 h** (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum (der Trigger).
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle (RTSW/ACE Minuten,
  Kp ≤3 h, Swarm ≤1 d; je Messwert `source`+`active`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
