<!--
  title: Handover — River-Folge 37 (2026-09-26)
  session: River-Folge 37
  class: handover
  date: 2026-09-26
  sha256: af964d40647868b23ac45da52b86095894fef02bac9620b9b90faebffa73cbbf
  status: live
-->
# Handover — River-Folge 37 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter —, dann chronologisch**. Jeder Punkt trägt
**Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge36.md` (jetzt in
`docs/handover/archiv/`).

**Wort | Datum | Quelle**
- Funk-Sensor über HTTPS: **ja** | 2026-09-26 | Operator-Wort folge36 („1. Ja").
- FIT-Brücke 945: **ja** — Garmin liefert .FIT-Dateien | 2026-09-26 | Operator-Wort folge36.
- Puls-Weg: **(b) Live-BLE nach dem Membran-Fix**, kein Brustgurt | 2026-09-26 | Operator-Wort folge36 („b").
- Geräte-Inventar: **Einzelbefehle liefern**, nicht Operator-Messung | 2026-09-26 | Operator-Wort folge36 („3 einzelbefehle").
- Mantis Shrimp (BOM/Beschaffung): **LOCK — kommt zu allerletzt** | 2026-09-26 | Operator-Wort folge36.
- Harte Läufe: **LOCK — verboten**, bis die Membran sauber nur die Presence lädt | 2026-09-26 | Operator-Wort folge36.
- Chrome-Debugger andocken: **ja** | 2026-09-26 | Operator-Wort folge36 („6 ja du darfst").
- Browser-Extension: **forken statt Dritten fragen** | 2026-09-26 | Operator-Wort folge36 („7 können wir die extension nicht forken?").
- vC-Permeabilität: **945 und Mantis Shrimp getrennt führen** | 2026-09-26 | Operator-Wort folge36 („8 TRENNEN!!!!!").
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: den Plan ausführen; Commit trägt `/commit`.

## Stehender Pass (measured 2026-09-26)

- **HEAD:** `56799abc02be15601048a06f5ac5438234111d89`.
- **Safety-Snapshot:** `refs/safety/1790419204` (`66da06eac`, 2026-09-26 12:40).
- **Postfach:** `state/mail/mail_ledger.φ` = privates Repo (`state/`) — hier absent
  (Pfad-Artefakt); `docs/zustand/external-state.md` (Mountain): keine fällige Korrespondenz.
- **open_points_check folge36:** 19 Pfad-Refs, 0 absent, 0 format-gaps, 0 owner-drift.
- **`register_lookup --open`:** 117 docs, 451 offene Zeilen, 26 zustand-due, 8 orphan
  (future 4 / mycelium 4), **orphan-docs 0**; disposition 83 [handover 5, survey 38,
  auftrag 2, sheet 4, concept 33, paper 35]; pipeline: ledger 2, index 10, harvest 3,
  sources 0, witnesses 0, footprints 0, nrs 0, probes 0.
- **CI (gemessen 2026-09-26 via `ci_manage list`/`ci_manage view`):** health-check
  `36237216821` **pending** (dispatch 10:54:58Z, head `07d4b899`), paralleler
  Schedule-Lauf `36236800246` in_progress; `ci-check` `36239540093` pending;
  `paper-check` jüngster `36235264452` failure (fremde `terminologie-der-gegenstroemung`
  sha-Mismatch; gic steht mit sha-Match in der Liste, nicht im `->`).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### star-dmax-probe dispatchten + Artefakt falten (RISS-Zeuge)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Commit+Push dieser Session steht, `star-dmax-probe.yml` auf `main`.
- **Lage:** (gemessen 2026-09-26 via git/sread/`gh workflow run`) Probe-Bin
  `tools/measure/src/bin/star_dmax_probe.rs` (399 Zeilen) und Workflow
  `.github/workflows/star-dmax-probe.yml` (40 Zeilen) sind mit `f9ea3284` auf `main`;
  der Lauf wurde dispatcht, Run `36240676551`
  (https://github.com/omegaflow/omegaflow/actions/runs/36240676551). Lokaler
  `--span`-Pass: `dr3_stars.bin` → COUNT 1.704.587, SPAN_M 1.798012e21
  (≈ 58 kpc), EPOCH_MIN = EPOCH_MAX = 0.0; daraus `cell_size_star ≈ 2.57e19 m`
  und heutige Hülle `rho_star ≈ c·8.4e8 s + pad ≈ 8.2 pc` (< eine Zelle). Der volle
  `d_max`/ECDF/`f_excl`-Lauf ist ungemessen; die zwei RISS-Zeugenlinien bleiben
  ungeglättet getragen.
- **Blockade:** keine — der Lauf ist dispatcht, das Artefakt fehlt noch.
- **Braucht:** Run `36240676551` mit `ci_manage view 36240676551` lesen, Artefakt
  lesen; RISS-Zeile per
  Schwelle schließen (`f_excl > 0.5` / `f_inc > 0` / interdecile > 10 bei irgendeinem
  Floor → Insert-Fix widerlegt, Query-Seite steht). **Der Harte-Läufe-LOCK fällt erst,
  wenn der Folgelauf die drei Umgehungen (Bootstrap `main_flow.rs:191-241`, Per-Tick-Fetch
  `:1227-1258`, catalog tycho/dastcom `:1299-1364/1758-1833`) auf derselben Hülle
  verifiziert hat.**

#### health-check — Verdikt am Lauf `36237216821`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36237216821` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`/`gh run list`) Rerun
  `36237216821` = pending (dispatch 10:54:58Z, head `07d4b899`); paralleler
  Schedule-Lauf `36236800246` in_progress. Der Tone→Apertur-Gate-Test
  (`src/mathematikerin/tests.rs:547`) hängt daran.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36237216821`, bei Rot `ci_manage log 36237216821`.

#### Browser-Extension — Build + CI-Testverifikation
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Fork-Build + Testsuite laufen.
- **Lage:** (gemessen 2026-09-26 via git) Fork `tools/browser-extension/` (60 Dateien,
  uncommitted) trägt `chrome.alarms`: `wxt.config.ts:33` `alarms` in der Basis-Liste,
  `tools/browser-extension/src/background/bridge-client.ts` `KEEPALIVE_ALARM`/`scheduleReconnect`/`clearReconnect`/`resume`
  (4 Treffer), `tools/browser-extension/src/entrypoints/background.ts:69-75` `chrome.alarms.onAlarm`-Listener;
  `setTimeout`-Backoff entfernt. Die Testsuite ist **nicht gelaufen** — unverified,
  nicht grün.
- **Blockade:** keine.
- **Braucht:** Fork-Build + Suite in CI ausführen (`npm ci && npm run build && npx vitest run`
  im Fork, oder eigener Workflow) und `.output/chrome-mv3/` als Artefakt bereitstellen;
  das Unpacked-Laden ist der Operator-Akt (siehe Operator).

#### Sonnenfarbe color: measured — WGSL-Ausführung im Lauf
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein CI-Lauf, der den color:measured-Pfad rendert.
- **Lage:** (gemessen 2026-09-26) naga-Gate gelandet (`src/mathematikerin/tests.rs:19`,
  `browser_fieldshader_validates_and_carries_the_measured_branch`, cargo check clean),
  aber die WGSL-Ausführung bleibt ungemessen; kein Workflow rendert den inline
  fieldShader (`static/index.html:189-259`, lut binding :196, measured branch :231,
  LUT-Serve `relay.rs:431`) — alle 306 `.github/workflows/*.yml` starten keinen
  WebGPU-Adapter/`index.html`, die wgpu-Crosscheck-Tests überspringen auf
  ubuntu-latest ohne Vulkan/GL-ICD, und der Browser-Shader ist eine Render-Pipeline,
  nicht Compute.
- **Blockade:** kein compute-only CI-Renderpfad (Route (b) nicht gebaut).
- **Braucht:** headless compute-only WebGPU-Renderschritt in CI
  (mesa-vulkan-drivers/lavapipe oder wgpu-Rekonstruktion der Pipeline
  `static/index.html:285-313`), `compatible_surface: None`; der sichtbare Lauf bleibt
  unter dem Harte-Läufe-LOCK.

### Rat handelt

#### Ruhe-Ort (SSB-Origin) als Hüllen-Zentrum?
- **Status:** blockiert | **Bindung:** Rat
- **Trigger:** Rats-/Operator-Verdikt.
- **Lage:** (gemessen 2026-09-26 via build-Report) Kataloge warten jetzt auf eine Presence
  (`presence_gate`-Refusal); die ruhende Presence (`PresenceState::rest`, `omega.rs:99-108`)
  ist **kein Eintrag** in `archive.presence`, also trägt ein Hidden-Lauf ohne Browser/Relay
  keine Sterne.
- **Blockade:** Verdiktfrage — allein Rat/Operator-Wort entscheidet sie, kein Messweg.
- **Braucht:** Rat-/Operator-Wort, ob der Ruhe-Ort als Hüllen-Zentrum zählt.

### Operator handelt

#### Chrome DevTools MCP — Remote-Debugging aktivieren
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator aktiviert `chrome://inspect/#remote-debugging` und startet die
  opencode-Session neu.
- **Lage:** (gemessen 2026-09-26 via git diff) `opencode.json` MCP-Kommando auf
  `--autoConnect` umgestellt (`chrome-devtools-mcp@1.9.0`, `--no-usage-statistics
  --no-performance-crux`), `jq empty` grün; Chrome ≥136 ignoriert
  `--remote-debugging-port` für das Default-Profil ohne den Schalter.
- **Blockade:** geräteseitige Aktivierung.
- **Braucht:** in Chrome `chrome://inspect/#remote-debugging` einschalten, Session neu
  starten; dann ein Membran-Lauf mit gelesener Konsole/Netz (letzterer unter Harte-Läufe-LOCK).
- **Wort:** Chrome-Debugger andocken ja | 2026-09-26 | Operator-Wort folge36.

#### Browser-Extension unpacked laden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Build-Artefakt `.output/chrome-mv3/` liegt vor (Vorbereitung Linie).
- **Lage:** (gemessen 2026-09-26 via git) Fork `tools/browser-extension/` liegt; der Build
  ist noch nicht gelaufen.
- **Blockade:** Build (Linie) offen.
- **Braucht:** nach dem Build `chrome://extensions` → Developer mode → "Load unpacked" →
  das erzeugte Unpacked-Verzeichnis im Fork.
- **Wort:** Extension forken | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar — Einzelbefehle ausführen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt die Befehle am Gerät aus.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) vorhanden Laptop, Hibreak Pro (Bigme),
  Pixel 9, Forerunner 945, Meta Quest 1 (Existenz unklar); Pixel-Mikrofonanzahl = 3
  gemessen. Offen: 945-Sample-Raten + Chip/FCC, Quest-1-Firmware + WebGPU/Generic-Sensor,
  Bigme-Näherung/Licht/Haptik/WebGPU, Pixel-`SensorManager`-Liste.
- **Blockade:** Messung nur am physischen Gerät.
- **Braucht (Kopierbefehl, bis zur Kante geliefert):**
  - Pixel 9 / Bigme, ADB über USB-Debugging: `adb shell dumpsys sensorservice`
    (Sensorliste, Raten, Näherung/Licht/Haptik).
  - Pixel 9 / Bigme WebGPU: im Browser `chrome://gpu` öffnen (WebGPU/Vulkan-Status).
  - Forerunner 945: Gerät → Einstellungen → System → Info (Software-/FCC-/IC-Nummer);
    Sample-Raten aus dem `.FIT`-Header (`data/garmin-945/2026-09-26/`).
  - Meta Quest 1: `adb devices` dann `adb shell dumpsys sensorservice`; Firmware/WebGPU
    in Einstellungen → Info bzw. im Quest-Browser `chrome://gpu`.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### Puls-Pfad 945 — Weg (b): Live-BLE-HR nach dem Membran-Fix
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Harte-Läufe-LOCK gefallen (verifiziertes `star-dmax-probe`-Artefakt + Folgelauf), dann 945-BLE-HR live.
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow` + Backup) die 945 wurde gesichert
  (`data/garmin-945/2026-09-26/`, 36 MB: 25 Aktivitäten + Monitor/Metrics + Musik, ohne
  Karten-`.img`); alle 134 `.fit`/`.FIT` tragen `nn: 0` (RR liefert die Uhr nur live über
  BLE; `decode_hr_measurement`, `src/archivar/ble.rs:1220`, läuft in `ble_ingress`).
- **Blockade:** Harte-Läufe-LOCK.
- **Braucht:** nach dem Lock-Fall `OMEGAFLOW_HIDDEN=1 bin/omegaflow` mit BLE-HR der 945;
  dann messen, dass `tone_code` in den Stress-Zustand wechselt und `tone_scale` relaxiert.
- **Wort:** Puls-Weg (b) Live-BLE nach Membran-Fix | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig (ohne Platine)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via sread/sgrep) der Pfad steht
  (`src/mathematikerin/omega.rs:200/349`), HRV-Reader gebaut (`src/archivar/ble.rs:715/926`);
  das 945 ist physisch vorhanden; absent ist der Puls-Arrival am Kanal.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann
  `perm_target_probe --live <pfad>` — der Run unter dem Harte-Läufe-LOCK.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### HRV/Puls-Arrival → Radiations-Pfad (Quelle 945 per BLE)
- **Status:** wartend | **Bindung:** operator
- **Trigger:** der Puls-Arrival liegt am Kanal (nach verifiziertem `star-dmax-probe`-Lauf).
- **Lage:** (gemessen 2026-09-26 via sread/sgrep) Code-Pfad steht: `feed_beat_to_hrv`
  (`main_flow.rs:79`) speist Beats in das HRV-Gate (`VagusTone` → `tone_code`); ω-Loop
  relaxiert `tone_scale` (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale`
  (`omega.rs:349`); Test `tests.rs:547`. Absent ist der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann `tone_code`-Stresswechsel +
  `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) Live-BLE nach Membran-Fix | 2026-09-26 | Operator-Wort folge36.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Handy und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`) das TLS-Material ist erzeugt und liegt
  gitignored im state-Verzeichnis (ca.pem/ca.key `CN=omegaflow-relay-ca`,
  relay-leaf.pem/relay-leaf.key `CN=omegaflow-relay`, SAN `IP:<lan-ip>`, von der CA
  signiert); `bin/relay-tls.stunnel.conf` zeigt darauf; Spec
  `docs/specs/relay-tls-terminator.md` auf „Measured 2026-09-26".
- **Blockade:** geräteseitiger CA-Trust + der Test-Run (Harte-Läufe-LOCK).
- **Braucht:** `ca.pem` am Handy installieren (Einstellungen → Sicherheit → Zertifikat
  installieren → CA), `stunnel bin/relay-tls.stunnel.conf`, `bin/omegaflow` ohne
  `OMEGAFLOW_HIDDEN`, `https://<lan-ip>:1619/consent?ja`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

#### Mantis Shrimp — Sensor-Hardware beschaffen (BOM)
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) die BOM ist bestellfertig
  (`docs/specs/mantis-shrimp-bom.md`); die Platine ist **nicht bestellt**.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort zum Aufheben; dann BOM bestellen (AliExpress-Login).
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

#### Harte Läufe (sichtbar/hidden) — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Membran verifiziert (star-dmax-probe-Artefakt + verifizierender Folgelauf
  über die drei Umgehungen).
- **Lage:** (gemessen 2026-09-26 via sgrep/git) Stern-Gitter (`StarCellKey`, 12 Treffer),
  `enclosure_rho` (membrane/spatial/fetch), Sprung-Vektor `jump_residual_breached`
  (12 Treffer) und die drei Umgehungen auf der Hülle sind **gebaut**; die Verifikation
  fehlt.
- **Blockade:** Verifikation (siehe Linie-Punkt star-dmax-probe).
- **Braucht:** Probe-Artefakt lesen + verifizierenden Folgelauf; dann LOCK aufheben
  (Operator-Wort).
- **Wort:** Harte Läufe LOCK | 2026-09-26 | Operator-Wort folge36.

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via sread) die präregistrierte JUICE-Kette wartet auf
  das Perigäum; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene Abschluss-Check läuft
dann mit Commit und Push. `/consent` ist der session-weite Consent (Delegation), nie das
Commit-Wort.
