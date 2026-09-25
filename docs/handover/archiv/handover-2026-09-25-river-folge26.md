<!--
  title: Handover — River-Folge 26 (2026-09-25)
  session: River-Folge 26
  class: handover
  date: 2026-09-25
  sha256: f1957bb4776817f6226f7f2288c73222400651592b879595b10acbfdcae6ee45
  status: live
-->
# Handover — River-Folge 26 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch
(Messdatum)** (Operator-Wort 2026-09-25). **Vorbereitung ≠ Akt:** ein
operator-gebundener Punkt wird getrennt geführt — Vorbereitung (autonom,
dispatcht) und Akt (operator-gebunden).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic-causal-driver reifen — restauriert nach stillem Drop
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `research-max`/`sgrep`/`git log`) der Punkt
  lebte in `river-folge22:45`/`folge23:52–84` und wurde beim Falten folge23→folge24
  (Commit `8753fcbe7`) ohne auflösenden Commit fallengelassen; das Papier ist seit
  `544936dc3` (2026-09-04) unberührt. Der `register_lookup --dropped`-Lauf sieht den
  Drop **nicht** — er meldet `river …folge23.md:53 …folge24.md **Status:** autonom |
  **Bindung:** eigen persist 4 git: resolved` (CI-Lauf 36116592301), weil der Detektor
  Punkte über die generische `**Status:**`-Zeile statt über den Titel schlüsselt.
  Die 7 Lücken sind gegen den Baum gemessen: (1) Lag-Sweep offen —
  `tools/measure/src/bin/bz_retro_probe.rs:575` `lags = [0,1]`; (2) `fam` 10→≥100
  offen — hartkodiert `for _ in 0..10` (`bz_retro_probe.rs:359`,
  `bz_blatt_probe.rs:312`), Regler `src/mathematikerin/te.rs:1499`/`:78` ungenutzt;
  `te-bz-laic-nsurr100.yml` schließt nur die nobel/LAIC-Probes (AE/Dst/SYM-H), nicht
  die dB/dt-Kette; (3) KDE-h-Robustheit **partial** — `te.rs:142 transfer_entropy_lag_h`
  hat 4 Aufrufer (`te_null_limits_probe.rs:247/252`, `corona_ladder_probe.rs:135`,
  `laic_probe.rs:1352`, `lsst_color_coupling_probe.rs:397`) mit `FACTORS=[0.5..3.0]`,
  aber auf SWPC-Live-Kanälen, **nicht** auf Bz→dB/dt; (4) Treiber-Statistik offen (Bz
  wird per Mittel gebinnt, kein Median/Newell); (5) `--station` generisch
  (`bz_retro_probe.rs:669`), nur ABK/SOD; ein GIC-Asset existiert
  (`fmi_gic.bin`, `fmi-gic-cdn.yml`, `phi/sources.φ:8596 fmi_gic_a`), aber kein Probe
  paart es mit dB/dt (`bz_blatt_probe.rs:863`); (6) Minuten-Archiv offen
  (`swpc-mirror-cdn.yml:30` spiegelt nur rollierend); (7) Titel „causal" vs nur
  PCMCI-Kreuzcheck offen.
- **Blockade:** keine — schwere Läufe gehören nach CI.
- **Braucht:** `cargo run`-Wiring (Lag-Liste `[0..6]`, `n_surr ≥ 100`, KDE-h-Zeile,
  Treiber-Stat) → `gh workflow run bz-retro-probe.yml`; oder Titel/Claims
  abschwächen. Werkzeuge `archive_search`, `sgrep`, `sread`.
- **Benchmark:** flash (`general`) schlägt pro/max (`research-max`) — **korrekter**
  (`te.rs:142` hat 4 Aufrufer, `fmi_gic.bin` existiert) bei ~2,4× geringerem Burn
  (`session_burn`: `general` ~$0,027/Session, `research-max` ~$0,066/Session;
  gemessen 2026-09-25). Sieger: `general` (flash).

#### `register_lookup --dropped` schlüsselt über die Status-Zeile (Blindstelle)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die für `tools/register/src/bin/register_lookup.rs` verantwortliche
  Linie committet die Datei.
- **Lage:** (gemessen 2026-09-25 via CI-Lauf 36116592301) ein Punkt, dessen
  `**Status:**`-Text in der Folgeübergabe von einem **anderen** Punkt getragen wird,
  wird als `git: resolved` gemeldet, obwohl er fehlt und kein auflösender Commit
  existiert — so blieb der GIC-Drop unsichtbar. Der volle Sweep listet **711**
  `git: none` (echte, unaufgelöste Drops), aber diese Klasse entgeht ihm. Der Fix
  ist geschrieben: `extract_open_points` schlüsselt Punkt-Identität jetzt über die
  `### `/`#### `-Überschrift (`from_heading`), Test
  `extract_open_points_keys_shared_status_points_by_their_headings`;
  `cargo check -p omegaflow-register --bin register_lookup` 0/0 (gemessen 2026-09-25
  via `grind-pro`).
- **Blockade:** die geteilte Datei trägt **fremde uncommittete rustfmt-Hunks** — ein
  pfad-begrenzter Commit würde sie mit-stagen (Regelverstoß); nicht committet.
- **Braucht:** die Datei-Eigentümer-Linie committet `tools/register/src/bin/register_lookup.rs`
  (ihre Formathunks + diesen Fix) — oder der Fix wird hunk-genau isoliert. Dann
  `cargo test -p omegaflow-register` in CI.

#### health-check `verify` — Shard-Lauf verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Lauf `36120693487` endet.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`) der Lauf ist **pending**
  (`health-check 36120693487`, gestartet 09:50:35Z, @`2bc46bed0`); der `verify`-Sweep
  war chronisch 4–9 h (#111 9h25m, #112 6h55m, #113 6h22m), daher 8er-Shard-Matrix +
  `timeout-minutes: 240` (`health-check.yml`).
- **Blockade:** wartet auf das Laufende (der Trigger).
- **Braucht:** `ci_manage view 36120693487` — jede Shard-Leg success, Total < 3 h;
  dann Cron/Cadence nachziehen.

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kabelloser Sensor wird gebraucht.
- **Lage:** (gemessen 2026-09-25 via `sgrep`/`grind-pro`) kabellos braucht HTTPS
  secure context (`static/sensorium.js:14/88/137`); rustls ist pure Rust → kein
  Kern-Umbau. Der externe Terminator ist gebaut: `docs/specs/relay-tls-terminator.md`
  (Wahl **stunnel** — `0.0.0.0:1619` → `127.0.0.1:1618`, Caddy verworfen: ACME
  braucht eine öffentliche Domain) + `bin/relay-tls.stunnel.conf`.
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
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`,
  dann `perm_target_probe --live <pfad>`.

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (gemessen 2026-09-25 via `sread tools-build.yml`):
  Relay kompiliert im PATH-Bin, Bind `0.0.0.0:1618` (`relay.rs:11/73`),
  `hidden = env("OMEGAFLOW_HIDDEN").is_ok()` (`main_flow.rs:612`), Consent
  `/consent?ja` (`relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht einen
  sichtbaren Lauf.
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
