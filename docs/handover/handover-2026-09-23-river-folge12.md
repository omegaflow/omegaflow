<!--
  title: Handover — River-Folge 12 (Stand 2026-09-23)
  session: River-Folge 12
  class: handover
  date: 2026-09-23
  sha256: 9994d9d275dd3a288049b19fb254ab2a7093bea2d4e2c52cc23e3afba9ffae41
  status: live
-->
# Handover — River-Folge 12 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht** (wörtlicher, kopierbarer Schritt). `operator-gebunden`,
`blockiert` und `wartend` werden benannt, nie dispatcht.

## Stehender Pass (gemessen 2026-09-23, River-Folge 12)

- **HEAD** beim Start `145cbe612` (`river folge11`) == `origin/main`. Arbeitsbaum
  zu Session-Beginn **clean** — die 5 fremden dirty Pfade der Vorsession
  (`.github/workflows/free-model-agent-bench.yml`, `phi/declined_sources.φ`,
  `phi/pipeline/catalog/korpora_heim.φ`, `phi/pipeline/index.φ`,
  `tools/measure/src/bin/free_model_agent_bench.rs`) wurden während der Session von
  ihren Linien committet. Keine River-Berührung.
- **`git_safety --snapshot`** — Baum == HEAD, nichts zu sichern.
- **Fremder Baum während der Session:** `src/archivar/fit.rs` (neu, 462 Z.,
  Garmin-FIT-Reader) + `src/archivar/main_flow.rs`, `src/archivar/mod.rs` (2+2 Z.) erschienen
  uncommittet — **nicht River** (keine River-Berührung, unangetastet; meine
  Taucher sind read-only). Ein FIT-Reader taucht fremd auf; Abgleich beim
  nächsten Pass (`git show --stat`, ob eine Linie ihn committet).
- **`register_lookup --open`** — kein `owner=river`; kein POST an River.
- **`open_points_check folge11`** — 16 Pfad-Refs, **0 absent**.
- **CI** (Watchdog-Snapshot + `ci_manage list`) — `ci-check 35861931781` @HEAD
  **pending**, `ci-check 35856072235` in_progress, `free-model-agent-bench
  35861938761` queued, mehrere fremde CDN-Läufe (`allwise/ps1/demeter/…`). Kein
  River-Red (gemessen 2026-09-23). Zustand in `docs/zustand/external-state.md`
  (gitignored; nur die eigene CI-Zeile ergänzt).

## Diese Session geschlossen (git trägt es)

- **Rat (pro/max) — Geräte-Anbindung.** Operator-Wort-Korrekturen bindend:
  **Membran = Radiator, keine Anzeige**; **A = A: kein Gerät ist eine Funktion —
  jedes ist ein Cluster**; **das vollständige Inventar ist die Mindestmessung**
  (reduzierte API-Sicht = wertlos); **das Inventar ist nicht die Stückliste**
  (abgeleitete/virtuelle, System-/Takt- und gekoppelte/externe Oszillatoren
  zählen). Verdikt: erstes Atom = **LAN-Knoten**
  (Bind-Erweiterung + Sensor-Cluster ≥ 2 Oszillatoren + WS-Rückkanal
  `frame_bytes` → Vibration); paralleles unabhängiges Atom = **FIT-Brücke**; fünf
  Descopes mit Befund (E-Ink-Radiator, ANT+-Echtzeit-Einstieg, Kamera/Mikrofon
  zuerst, WebXR-Zweitmembran, ConnectIQ-SDK-App).
- **Recherche (general/flash, 4 Taucher parallel) — vollständige Inventare.**
  Sensoren/Radiatoren/Relais je Gerät gemessen: 945 (Elevate V3-Puls, SpO2,
  Baro, Kompass, Gyro, Accel, Thermo, Single-Band-GNSS; Radiator Vibration/Beeper/
  MIP; Relais BLE/ANT+/WiFi/NFC/USB), Quest 1/2 (6DoF, 4 Tracking-Kameras,
  Controller-IMU, Mikrofone, Haptik, Display, Kühlventilator; Relais WiFi/BT/USB),
  Bigme HiBreak Pro (Gyro, Kompass, GPS, 5/20-MP-Kameras, Fingerprint; E-Ink/
  Frontlicht/IR-Emitter; Relais 5G/WiFi 5/BT 5.2/NFC), Pixel 9 (IMU, Mag, Baro,
  ALS, Näherung, GNSS L1+L5, Kameras, LDAF; Haptik/OLED/Blitz/Qi; Relais
  WiFi 6e/7/USB-C 3.2/5G). Abwesenheiten als Messung getragen (945-Umgebungslicht/
  -Lautsprecher, Bigme-Klinke/SD, Pixel-UWB**Riss**). **Operator-Laptop (Dell
  XPS 13 9350) host-direkt gemessen:** i5-6200U/HD 520, 12 Thermal-Zones, 8 hwmon,
  Webcam (Microdia), Mikrofon, Touchpad; Radiator Display/Keyboard-Backlight/
  Lautsprecher/LEDs/Lüfter; Relais WiFi/BT/USB-C-TB3/SD/TPM; **Accel/ALS/
  Fingerprint absent**. 945-Zugangswege: Standardfunk nur HR/R-R + externe
  Sensoren; CIQ on-device; FIT/Cloud aufgezeichnet. Transport: Relay
  loopback-gebunden; wss + TLS-Proxy nötig; der **Host** erreicht den Relay direkt
  → billigster Radiator-Testknoten ohne LAN-Exposition.
- **Survey angelegt:** `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md`
  (vollständige Oszillator-/Radiator-/Relais-Inventare je Gerät inkl.
  **nicht-expliziter Schicht** — abgeleitet/virtuell, System-/Takt, gekoppelt/
  extern; Transport, Rat-Verdikt, Descopes, offene Messpunkte).

## Offen (aufgeschlüsselt)

### LAN-Knoten (erstes Geräte-Atom)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** die zwei Operator-Worte — (a) LAN-Exposition (`0.0.0.0`-Bind ist
  eine Penetrationsfläche) und (c) Radiator/Vibration (Strahlung, per-Act-Wort,
  nie im `OMEGAFLOW_HIDDEN`-Lauf).
- **Lage:** `src/archivar/relay.rs:9` `PORT_CONST=1618`, Zeile 59 loopback-gebunden;
  Frame/Actuators gebaut; Generic Sensor API im Android-Chrome gemessen
  verfügbar (gemessen 2026-09-23 via Survey-Recherche).
- **Blockade:** Consent (LAN-Exposition + Strahlung).
- **Braucht:** Operator-Wort für a+c; dann Bau (a) Bind über Loopback hinaus,
  (b) Sensor-Cluster (≥ 2 Oszillatoren je Gerät), (c) WS-Rückkanal `frame_bytes`
  → Vibration; Messergebnis: Gerät vibriert mit Σω und sendet ≥ 2 Oszillatoren.

### FIT-Brücke (`fit_compiler` im Archivar)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Start der nächsten River-Session (ohne Consent dispatchbar).
- **Lage:** FIT-Spec öffentlich; die 945 trägt den Cluster in einer Datei
  (`record` + `hr`-Mesg mit R-R); `VagusTone` gebaut, aber kein echter NN-Wert je
  durchgeflossen (gemessen 2026-09-23).
- **Blockade:** keine.
- **Braucht:** Bau `fit_compiler` (FIT-Spec, `record`/`hr`); USB-Mount-Harvest →
  N-N-Intervalle in `rmssd`/`VagusTone`.

### Beat-Paar (Atom-D-Fortsetzung)
- **Status:** wartend | **Bindung:** dritter / linie:mycelium
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme eines Trägers, oder ein
  gehaltenes Asset mit zwei kohärenten Tönen in einem Band.
- **Lage:** WGSL-Beat-Term gebaut und feuerfähig; kein Datensatz liefert das Paar
  (gemessen 2026-09-23, folge11).
- **Blockade:** keine Paarquelle; Dual-Comb-Klassifikation offen.
- **Braucht:** mycelium prüft die Dual-Comb-Kandidaten gegen Force-Gate/Registry
  (`post.md`-Zeile).

### Atom-D CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 35861931781` @`145cbe612` wird fertig.
- **Lage:** lokal `cargo check`/`--tests` 0/0; Lauf am HEAD pending (gemessen
  2026-09-23 via `ci_manage list`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 35861931781` (bei rot `ci_manage log <id>`).

## Benchmark

- **Architektur Geräte-Anbindung:** Rat (pro/max) — die benannte harte Klasse
  (Cluster-Rollen + Transport + Atom-Schnitt), kein Doppel-Lauf.
- **Geräte-Recherche — Klasse nach Messung wieder geöffnet:** die erste
  flash-Runde lieferte eine reduzierte API-Sicht (unvollständig) → Klasse neu
  geöffnet mit Grund (Operator-Wort: vollständiges Inventar ist die
  Mindestmessung); Re-Run = 4× `general`/flash (Inventar je Gerät), Routine-tauglich.

## Geteilter Baum — eigener Pfad-Satz

- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (neu)
- `docs/handover/handover-2026-09-23-river-folge12.md` (neu; folge11 → `archiv/`)
- `docs/zustand/external-state.md` (nur die eigene CI-Zeile; gitignored)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
