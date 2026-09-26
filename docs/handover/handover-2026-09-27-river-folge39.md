<!--
  title: Handover — River-Folge 39 (2026-09-27)
  session: River-Folge 39
  class: handover
  date: 2026-09-27
  sha256: a979811ece197bf4dd6b94b80b84ef2ecab4328599c6100201315665a755df0f
  status: live
-->
# Handover — River-Folge 39 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht erklärt; git
trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist.

Sortierung: umsetzbar → nicht umsetzbar. Kein Rang, kein härtester Punkt; jeder
Punkt trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge38.md` (nach
`docs/handover/archiv/`).

**Wort | Datum | Quelle**
- Punkt 1 Chrome-Debugger aktivieren: **ja** | 2026-09-26 | Operator-Wort — erledigt, CDP-Brücke verbunden (gemessen 2026-09-27).
- Funk-Sensor über HTTPS: **ja** | 2026-09-26 | Operator-Wort folge36.
- FIT-Brücke 945: **ja** — Garmin liefert .FIT-Dateien | 2026-09-26 | Operator-Wort folge36.
- Puls-Weg: **(b) Live-BLE nach dem Membran-Fix** | 2026-09-26 | Operator-Wort folge36.
- Geräte-Inventar: **Einzelbefehle liefern** | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp (BOM): **LOCK — zuletzt** | 2026-09-26 | Operator-Wort folge36.
- Browser-Extension: **forken statt Dritten fragen** | 2026-09-26 | Operator-Wort folge36.
- vC-Permeabilität: **945 und Mantis Shrimp getrennt führen** | 2026-09-26 | Operator-Wort folge36.
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: alles außer Harte Läufe; Commit trägt `/commit`.
- Sensory-Eigentum `src/archivar/llnl_g3d.rs`: **ja** | 2026-09-26 | Operator-Wort.
- Feld über **alle Radiatoren gleichberechtigt** ausgegeben | 2026-09-26 | Operator-Wort.
- Radiator-Transportmedien: **acoustic** / **visual** / **serial**; alle neun Kräfte je Radiator (Σω kanonisch, keine Kraft→Kraft-Mappung) | 2026-09-26 | Operator-Wort + Archäologie.
- Membran: **natürlich fixen** (Defaults + tycho-Divergenz) | 2026-09-26 | Operator-Wort.
- Geräte-Zugriff: **vor jedem Zugriff fragen** (adb/BT) | 2026-09-26 | Operator-Wort.
- **Harte-Läufe-LOCK aufgehoben** — Membran verifiziert (`membrane-hull-probe 36264567801` = 0/4) | 2026-09-26 | Operator-Wort.
- 945-BLE-HR-Bindung gebaut (HR bpm in Beat-Pfad; RR braucht Brustgurt) | 2026-09-26 | Linie.

## Stehender Pass (gemessen 2026-09-27)

- **HEAD:** `c1fae4bb8` == `origin/main` (fast-forward). Neu seit folge38: `6659f9366` (River, Parser-Arme), mountain 175, sensory 180, mycelium 174.
- **Safety-Snapshot:** am Sessionende.
- **Arbeitsbaum:** nur Fremdarbeit (`modis-cdn.yml`, `phi/*.φ`-modis/gosat, `skydirection.rs`, `modis_lst_cmg_compiler.rs`, `main_flow.rs`-fmt) — nichts Eigenes.
- **CI:** die register-coverage-Fehler (22:34–22:38) waren ein kaputter Baum (`main_flow.rs:5299`, `channels.rs` `PresenceSample`) — überholt; HEAD `c1fae4bb8` = mountain 175 „verify CI green". Aktuell rot: `ps1-cdn`, `swpc-mirror-cdn` (CDN, mycelium). Für `6659f9366` (00:28) noch kein Lauf gemessen.
- **`register_lookup`:** keine register-eigenen River-Einträge; `--fired`/`--stale` = 0. `--stale` ist blind für diese Punkte: es prüft identische Lage-Zeilen, jede Runde schreibt die Lage neu.
- **Postfach:** nur Dritte (NED-HelpDesk-AW, OpenAlex, Meta-Logins) — kein River-Eingang.
- **Browser (gemessen):** `browser_targets` = 2 verbundene Chrome-Executors; Gruppe `sensory` aktiv; `chrome-devtools`-MCP antwortet (`list_pages`/`snapshot`).
- Shared external state: `state/zustand/external-state.md` (nicht kopiert).

## Offen

### Umsetzbar (jetzt, autonom bis zur Kante)

#### `6659f9366`-Quellen — CI register-coverage
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Läufe `36277948083`/`36277951163` abgeschlossen.
- **Lage:** (gemessen 2026-09-27 via `git show` + `gh`) `6659f9366` baute 11 Parser-Arme (aia, eve, ephemeris_epm/inpop/de440/de442/noe4, spk, openneuro, catalog, mpcobs, apdb) + 293 Zeilen `phi/sources.φ`; Läufe dispatcht: `ci-check 36277948083`, `register-coverage 36277951163`.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36277948083` / `ci_manage view 36277951163` — Ergebnis messen.

#### `#body`-Deklaration — lokale Sensoren
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** keiner — durch Messung geschlossen (gemessen 2026-09-27).
- **Lage:** (gemessen 2026-09-27 via `sread`) Die folge38-Behauptung „ohne `#body` werden alle Stations-Samples verworfen, 10 Quellen betroffen" ist **falsch**: `#body=<body>,<lat>,<lon>,<alt>` (`main_flow.rs:800`) gate't allein die **lokalen** Sensor-Samples (`sensor_rx` → `sensor_config`, `membrane.rs:422`: Temperatur/Druck/Feuchte/Wind/Mikro/Licht/Batterie/HR/RSC). Die Remote-Quellen tragen ihre Position selbst (`on earth 35.68 139.69 0`, `phi/sources.φ:446`; `stations_lat`/`stations_lon`) — vom `#body` nicht betroffen. Kein Code-Defekt; `#body` ist der Operator-Standort (PII).
- **Blockade:** keine.
- **Braucht:** nichts — Betriebsnotiz: `bin/omegaflow '#body=Earth,<lat>,<lon>,<alt>'` für lokale Sensoren.

#### 945 Puls — HR-bpm-Relaxation
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eigen.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) HR bpm speist den Beat-Pfad (`ble.rs:1470`); `tone_scale` folgt binär `TONE_STRESSED` (`omega.rs:1682-1690`), kein kontinuierlicher bpm-Arm. Am Handgelenk liefert die 945 HR bpm, kein RR → der RR-Pfad bleibt stumm.
- **Blockade:** keine (der bpm-Arm fehlt; die Mapping-Basis ist zu messen).
- **Braucht:** `tone_scale` kontinuierlich aus bpm bauen (Mapping aus dem HRV-`tone_scale`-Test `tests.rs:621` als Basis) — oder Brustgurt (RR) beschaffen.

#### RX100 — K-Kalibrierung (Band verdrahtet)
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Kamera im Smart-Remote-LAN; Messung via `rx100_compiler`.
- **Lage:** (gemessen 2026-09-27) **Band verdrahtet** — `series_band` setzt `freq`/`bin_width` der Luminanz-Zeilen aus `FREQ_PHOTOPIC_HZ`/`BIN_WIDTH_PHOTOPIC_HZ`, Test `test_series_rows_carries_the_photopic_band_for_rx100`; `K=12.5` (ISO 2720) ungemessen.
- **Blockade:** Referenzmessung am Objekt.
- **Braucht:** `K` gegen ein kalibriertes Luminanzmeter messen; danach `cargo run -p omegaflow-harvest --bin rx100_compiler`.

### Nicht umsetzbar (wartet)

#### Akustik-Sink `OMEGAFLOW_ACOUSTIC` — sichtbarer Lauf
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den sichtbaren Lauf.
- **Lage:** (gemessen 2026-09-26) Sink gebaut (`acoustic_sink`, `auto` → `AcousticFanout`), committed `a6f84db01`, Test grün.
- **Blockade:** keiner — der Akt ist sichtbar (Radiator).
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto bin/omegaflow` — Bose + JBL + jeder Sink.
- **Wort:** Harte-Läufe-LOCK aufgehoben — sichtbarer Lauf frei | 2026-09-26 | Operator-Wort.

#### 945-BLE-HR — Puls-Arrival (RR)
- **Status:** blockiert | **Bindung:** operator
- **Trigger:** Brustgurt liefert RR.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) Bindung gebaut: RSC-Measurement-Char `00002a53` (`ble.rs:31`), `decode_rsc_measurement` (`ble.rs:1267`), HR-Service `0000180d` (`ble.rs:1871`); die 945 liefert am Handgelenk HR bpm, kein RR (sensory folge178, `d766ed9a5`).
- **Blockade:** RR fehlt am Handgelenk (Hardware).
- **Braucht:** Brustgurt beschaffen (sensory „RR-Kanal").

#### Geräte-Inventar — alle Wege je Gerät
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator paart/verbindet (Kopfhörer, Quest-Browser).
- **Lage:** (gemessen 2026-09-26 via `adb`/`bluetoothctl`/`lsusb -t`/`pactl`) RX100M5A (USB MSC + WiFi Remote), 945 (USB FIT + BLE HR/RSC), Quest 2 #1/#2 (adb + BLE), Pixel 10a (A2DP/PAN/BLE), HiBreak pro (A2DP/PAN), Bose Mini II, JBL TUNE500BT — vollständig vermessen.
- **Blockade:** keines.
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto`; 945 BLE-HR verbinden; Quest-Browser `http://<host>:1618`; RX100 mounten.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### Browser-Extension unpacked laden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt den Load aus.
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) CI-Build grün (Lauf `36243288134`, Artefakt `chrome-mv3/`).
- **Blockade:** keine.
- **Braucht:** `chrome://extensions` → Developer mode → „Load unpacked" auf den Fork `tools/browser-extension/` (Build `npm run build`; CI-Artefakt aus `browser-extension.yml`).
- **Wort:** Extension forken | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar — Quest-Rest
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Befehle am Gerät.
- **Lage:** (gemessen 2026-09-26 via `adb` + `chrome://gpu`) HiBreak/Pixel/945 vollständig; offen: Meta Quest `adb devices` + `dumpsys sensorservice`.
- **Blockade:** nur am physischen Gerät.
- **Braucht:** Quest `adb devices` + `dumpsys sensorservice`.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Pixel + startet `stunnel`.
- **Lage:** (gemessen 2026-09-26) Material in `state/tls/`; Leaf neu ausgestellt (SAN `IP:192.168.178.26`); Port 1619 doppelt belegt (`smail_recv` + `relay-tls.stunnel.conf`), Test-Config `/tmp/opencode/relay-tls-1620.conf`.
- **Blockade:** Port belegt; Chrome/Android vertrauen Nutzer-CAs nicht.
- **Braucht:** freien Port 1620; `ca.pem` am Pixel installieren; `stunnel /tmp/opencode/relay-tls-1620.conf`; `https://192.168.178.26:1620/consent?ja`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) Pfad steht (`omega.rs:200/349`), HRV-Reader gebaut (`ble.rs`); absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>`.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### Mantis Shrimp — Sensor-Hardware (BOM)
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) präregistrierte JUICE-Kette wartet; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent, nie das Commit-Wort.
