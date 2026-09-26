<!--
  title: Handover — River-Folge 38 (2026-09-26)
  session: River-Folge 38
  class: handover
  date: 2026-09-26
  sha256: dab898b3a3f5d731abff5665a2a6c16beb1fe552ce9c47056276ab9d2371dad0
  status: live
-->
# Handover — River-Folge 38 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter**, innerhalb der Gruppe ohne Rang. Jeder Punkt
trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge37.md` (nach
`docs/handover/archiv/`).

**Wort | Datum | Quelle**
- Punkt 1 Chrome-Debugger aktivieren: **ja** | 2026-09-26 | Operator-Wort (diese Session).
- Funk-Sensor über HTTPS: **ja** | 2026-09-26 | Operator-Wort folge36.
- FIT-Brücke 945: **ja** — Garmin liefert .FIT-Dateien | 2026-09-26 | Operator-Wort folge36.
- Puls-Weg: **(b) Live-BLE nach dem Membran-Fix** | 2026-09-26 | Operator-Wort folge36.
- Geräte-Inventar: **Einzelbefehle liefern** | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp (BOM): **LOCK — zuletzt** | 2026-09-26 | Operator-Wort folge36.
- Harte Läufe: **LOCK — verboten**, bis die Membran sauber lädt | 2026-09-26 | Operator-Wort folge36.
- Chrome-Debugger andocken: **ja** | 2026-09-26 | Operator-Wort folge36 („6 ja du darfst").
- Browser-Extension: **forken statt Dritten fragen** | 2026-09-26 | Operator-Wort folge36.
- vC-Permeabilität: **945 und Mantis Shrimp getrennt führen** | 2026-09-26 | Operator-Wort folge36.
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: alles außer Harte Läufe; Commit trägt `/commit`.
- Sensory-Eigentum `src/archivar/llnl_g3d.rs`: **ja** — die untracked Datei gehört der Sensory-Linie | 2026-09-26 | Operator-Wort.

## Offen

### Linie handelt (eigen)

#### star-dmax-probe dispatchten + Artefakt falten (RISS-Zeuge)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** sensory committet `src/archivar/llnl_g3d.rs` (baut `main`).
- **Lage:** (gemessen 2026-09-26 via git/SHA/`cargo check`) Probe-Bin `star_dmax_probe.rs` + Workflow `star-dmax-probe.yml` auf `main` (`f9ea3284`); Run `36240676551` = failure. Ursache gemessen: `HEAD:src/archivar/mod.rs:96` = `pub mod llnl_g3d;`, aber `git ls-tree HEAD src/archivar/llnl_g3d.rs` **leer** → CI-Checkout ohne Datei, `cargo check` rot (`error[E0583]`); lokal grün (Datei untracked auf Platte). Dieselbe Wurzel färbt `ci-check` rot (Job `build`, z. B. `36241316497`, head `fb8017ace`). Angelegt, aber noch nicht dispatcht: `.github/workflows/browser-extension.yml`. Lokaler `--span`-Pass steht: `dr3_stars.bin` → COUNT 1.704.587, SPAN_M 1.798012e21, EPOCH_MIN=EPOCH_MAX=0.0; `cell_size_star ≈ 2.57e19 m`, `rho_star ≈ 8.2 pc`.
- **Blockade:** sensory — committet die Datei noch nicht (aktive Fremdarbeit; neue untracked Compiler `isc_ehb_compiler.rs`, `llnl_g3d_jps_compiler.rs`).
- **Braucht:** sensory committet `src/archivar/llnl_g3d.rs`; dann `gh workflow run star-dmax-probe.yml`, `ci_manage view <id>`, Artefakt falten (RISS-Zeile per Schwelle schließen). Der Harte-Läufe-LOCK fällt erst, wenn der Folgelauf die drei Umgehungen (`main_flow.rs:191-241`, `:1227-1258`, `:1299-1364/1758-1833`) auf derselben Hülle verifiziert hat.

#### health-check — Verdikt am Lauf 36237216821
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36237216821` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`) `in_progress`, head `07d4b899`, `updated_at 11:58:15Z` — seither kein Fortschritt.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36237216821`, bei Rot `ci_manage log 36237216821`.

#### Browser-Extension — CI-Workflow gebaut; Dispatch + Artefakt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Commit+Push dieses Atoms (`browser-extension.yml` auf `main`).
- **Lage:** (gemessen 2026-09-26 via git/write) Fork committet (`f9ea3284`, 60 Pfade, `chrome.alarms` in `wxt.config.ts:33` + `bridge-client.ts`); kein Lockfile → `.github/workflows/browser-extension.yml` gebaut (npm install → `npm run build` → `npm test` → Artefakt `chrome-mv3/`). Die Testsuite ist **nicht gelaufen** — unverified.
- **Blockade:** keine (JS, unabhängig von der cargo-Blockade).
- **Braucht:** nach Push `gh workflow run browser-extension.yml`, `ci_manage view <id>`; bei Rot die Tests fixen; Artefakt für den Operator-Akt.

#### Sonnenfarbe color:measured — compute-only Renderpfad
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** `main` kompiliert (llnl-Fix) + lavapipe-ICD in CI.
- **Lage:** (gemessen 2026-09-26 via `general`) Route (A) gescopt: Extraktor `browser_field_shader()` in `src/mathematikerin/tests.rs` herausziehen, neuer `#[ignore]`-Test, Pipeline `static/index.html:285-313` offscreen rekonstruieren (`Rgba8Unorm`, `compatible_surface: None`, Bind-Layout `:285-289`, Blend `one/one`+`add` `:299-313`).
- **Blockade:** llnl_g3d (cargo-CI rot) + lavapipe-ICD fehlt auf ubuntu-latest (Crosscheck-Tests skippen, `tests.rs:110-113`).
- **Braucht:** nach llnl-Fix `mesa-vulkan-drivers`/lavapipe apt-Zeile + Test in `tests.rs` bauen; Verifikation, dass der measured-Zweig (LUT-Sampling) ausgeführt wird.

#### Ruhe-Ort als Hüllen-Zentrum — Lese-Kante bauen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `main` kompiliert (CI-Verifikation).
- **Lage:** (gemessen 2026-09-26 via `council`) Rat einmütig **Ja** — der Ruhe-Ort (SSB-Origin) zählt als Hüllen-Zentrum; die ruhende Presence ist ein voll realisierter Zustand, keine Abwesenheit. Der Wert muss aus dem stehenden Slot gelesen werden (`presence_slot`, `main_flow.rs:693-696`), nie hartkodiert `[0,0,0]`.
- **Blockade:** llnl_g3d (CI-Verifikation).
- **Braucht:** Code-Kante bauen — Hüllen-Zentrum aus dem stehenden Slot statt aus `archive.presence`, so dass der Hidden-Lauf ohne Browser die Sterne trägt; CI-Test.

### Rat handelt

(kein offener Punkt — das Ruhe-Ort-Verdikt ist gefallen)

### Operator handelt

#### Chrome DevTools MCP — Remote-Debugging aktivieren
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator aktiviert `chrome://inspect/#remote-debugging` und startet die opencode-Session neu.
- **Lage:** (gemessen 2026-09-26 via git diff) `opencode.json` MCP `--autoConnect` (`chrome-devtools-mcp@1.9.0`), `jq empty` grün.
- **Blockade:** keine.
- **Braucht:** in Chrome `chrome://inspect/#remote-debugging` einschalten, Session neu starten.
- **Wort:** Punkt 1 Chrome-Debugger aktivieren | 2026-09-26 | Operator-Wort („Ja").

#### Browser-Extension unpacked laden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Build-Artefakt `.output/chrome-mv3/` liegt vor.
- **Lage:** (gemessen 2026-09-26 via git) Fork committet; Workflow gebaut, Build noch nicht gelaufen.
- **Blockade:** Build (Linie).
- **Braucht:** `chrome://extensions` → Developer mode → „Load unpacked" → Fork-Unpacked-Verzeichnis.
- **Wort:** Extension forken | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar — Einzelbefehle ausführen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt die Befehle am Gerät aus.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) vorhanden: Laptop, Hibreak Pro (Bigme), Pixel 9, Forerunner 945, Meta Quest 1 (Existenz unklar); Pixel-Mikrofonanzahl = 3. Offen: 945-Sample-Raten + Chip/FCC, Quest-1-Firmware + WebGPU/Generic-Sensor, Bigme-Näherung/Licht/Haptik/WebGPU, Pixel-`SensorManager`-Liste.
- **Blockade:** Messung nur am physischen Gerät.
- **Braucht:** `adb shell dumpsys sensorservice`; `chrome://gpu`; 945 → Einstellungen → System → Info + `.FIT`-Header (`data/garmin-945/2026-09-26/`); Quest `adb devices` + `dumpsys sensorservice`.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### Puls-Pfad 945 — Weg (b): Live-BLE-HR nach dem Membran-Fix
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Harte-Läufe-LOCK gefallen (verifiziertes `star-dmax-probe`-Artefakt + Folgelauf).
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow` + Backup) 945 gesichert (`data/garmin-945/2026-09-26/`, 36 MB); alle 134 `.fit`/`.FIT` tragen `nn: 0` (RR nur live über BLE; `decode_hr_measurement`, `src/archivar/ble.rs:1220`).
- **Blockade:** Harte-Läufe-LOCK.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 bin/omegaflow` mit 945-BLE-HR; `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig (ohne Platine)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via sread/sgrep) Pfad steht (`src/mathematikerin/omega.rs:200/349`), HRV-Reader gebaut (`src/archivar/ble.rs:715/926`); absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>` — unter dem Harte-Läufe-LOCK.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### HRV/Puls-Arrival → Radiations-Pfad (Quelle 945 per BLE)
- **Status:** wartend | **Bindung:** operator
- **Trigger:** der Puls-Arrival liegt am Kanal (nach verifiziertem `star-dmax-probe`-Lauf).
- **Lage:** (gemessen 2026-09-26 via sread/sgrep) Code-Pfad steht: `feed_beat_to_hrv` (`main_flow.rs:79`), ω-Loop `tone_scale` (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale` (`omega.rs:349`); Test `tests.rs:547`. Absent ist der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Handy und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`) TLS-Material erzeugt (gitignored): ca.pem/ca.key `CN=omegaflow-relay-ca`, relay-leaf.pem/relay-leaf.key `CN=omegaflow-relay`; `bin/relay-tls.stunnel.conf` zeigt darauf; Spec `docs/specs/relay-tls-terminator.md`.
- **Blockade:** geräteseitiger CA-Trust + Test-Run (Harte-Läufe-LOCK).
- **Braucht:** `ca.pem` installieren, `stunnel bin/relay-tls.stunnel.conf`, `bin/omegaflow` ohne `OMEGAFLOW_HIDDEN`, `https://<lan-ip>:1619/consent?ja`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

#### Mantis Shrimp — Sensor-Hardware beschaffen (BOM)
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); Platine nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

#### Harte Läufe (sichtbar/hidden) — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Membran verifiziert (star-dmax-probe-Artefakt + verifizierender Folgelauf).
- **Lage:** (gemessen 2026-09-26 via sgrep/git) Stern-Gitter (`StarCellKey`), `enclosure_rho`, Sprung-Vektor `jump_residual_breached` und die drei Umgehungen sind gebaut; die Verifikation fehlt.
- **Blockade:** Verifikation (siehe Linie-Punkt star-dmax-probe).
- **Braucht:** Probe-Artefakt lesen + verifizierenden Folgelauf; dann LOCK aufheben (Operator-Wort).
- **Wort:** Harte Läufe LOCK | 2026-09-26 | Operator-Wort folge36.

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via sread) präregistrierte JUICE-Kette wartet auf das Perigäum; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
