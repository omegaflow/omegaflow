<!--
  title: Handover — River-Folge 30 (2026-09-25)
  session: River-Folge 30
  class: handover
  date: 2026-09-25
  sha256: 21510437666b970ee23bd2f7a534949b4072f6450a2d5ffc6053d61b7fd7f5fd
  status: live
-->
# Handover — River-Folge 30 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Sortierung: **erst logisch nach Akteur
(wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch**. Jeder
Punkt trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.
Vorbereitung ≠ Akt: wo die Maschine eine Kante hat, läuft die Vorbereitung autonom;
`operator-gebunden` ist allein der Akt.

Diese Session konsumierte `handover-2026-09-25-river-folge29.md`.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** wartend | **Bindung:** eigen/operator
- **Trigger:** kabelloser Sensor wird gebraucht.
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Spec `docs/specs/relay-tls-terminator.md` +
  `bin/relay-tls.stunnel.conf` gebaut.
- **Blockade:** geräteseitige CA operator-gebunden.
- **Braucht:** Operator: lokale CA + Leaf-Cert (SAN = LAN-IP), `stunnel
  bin/relay-tls.stunnel.conf`, `https://<lan-ip>:1619`.

#### gic-causal-driver — Neu-Messung nach Härtung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des Laufs `bz-retro-probe 36136095463`.
- **Lage:** (gemessen 2026-09-25T20:00 via `ci_status.md`) der Lauf ist `in_progress`.
- **Blockade:** keine — der schwere Lauf gehört nach CI.
- **Braucht:** `ci_manage view 36136095463` nach Abschluss; Artefakt
  `bz-blatt-minute.txt`.

#### health-check — voller grüner Shard-Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene health-check-Lauf.
- **Lage:** (gemessen 2026-09-25T20:00 via `ci_status.md`) Lauf `36132614446`
  `in_progress`; der vorige `36140394227` rot durch externen Runner-Shutdown
  (`verify (4)`).
- **Blockade:** wartet auf einen Lauf ohne externen Shard-Abriss.
- **Braucht:** `ci_manage view <nächster health-check-Lauf>`.

### Operator handelt

#### Sonnenfarbe: erster Lauf `color: measured`
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren/hidden Lauf.
- **Lage:** (gemessen 2026-09-25 via git, Commit `2de359982`) der Farbmodus ist
  gebaut; Parity-Gate grün; die WGSL-Ausführung im Lauf ist ungemessen.
- **Blockade:** GPU-Lauf = Heavy compute (CI/Operator-Wort).
- **Braucht:** der erste Lauf, der `color: measured` rendert.

#### Sensor-Bindung vC-Permeabilität
- **Status:** operator-gebunden (wartet auf Hardware) | **Bindung:** operator
- **Trigger:** Smartwatch + Mantis-Shrimp sind angeschlossen.
- **Lage:** der Permeabilitäts-Pfad steht (`src/mathematikerin/omega.rs:200/349`),
  der HRV-Reader ist gebaut (`src/archivar/ble.rs:715/926`).
- **Blockade:** Hardware fehlt.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`,
  dann `perm_target_probe --live <pfad>`.

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (`relay.rs:11/73`, `main_flow.rs:612`,
  Consent `relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal).
- **Braucht:** der Operator führt aus: `adb devices && adb reverse tcp:1618
  tcp:1618`, dann `bin/omegaflow` **ohne** `OMEGAFLOW_HIDDEN`.

#### Akt: Sensor-Hardware beschaffen (Bestellung)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator bestellt die BOM und schließt die Hardware an.
- **Lage:** die BOM ist bestellfertig (`docs/specs/mantis-shrimp-bom.md`).
- **Blockade:** Beschaffung/Kosten; kein Node-Teil vorhanden.
- **Braucht:** Operator bestellt die BOM-Positionen (AliExpress-Login).

#### Chrome DevTools MCP anbinden (Membran-Debug, Pfad i)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am live Chrome).
- **Lage:** (gemessen 2026-09-25 via `sread` tools-map) Chrome DevTools MCP ist
  „noch nicht angebunden"; npm 1.9.0, Telemetrie-Flags `--no-usage-statistics`
  `--no-performance-crux` sind Bedingung.
- **Blockade:** Debugger-Rechte am live Chrome (Operator-Wort).
- **Braucht:** Operator-Wort; dann MCP 1.9.0 pinnen, an den Pfad-1-Chrome hängen,
  ein Membran-Lauf mit gelesener Konsole/Netz.
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md

#### MV3-Kaltstart-Latenz der Pfad-1-Extension entschärfen
- **Status:** operator-gebunden | **Bindung:** dritter
- **Trigger:** Operator-Wort / Extension-Änderung.
- **Lage:** (gemessen 2026-09-25 via `sread`) Kaltstart 0 s bis 2276 s, bimodal;
  Ursache ist der `setTimeout`-Backoff im MV3-Service-Worker ohne `chrome.alarms`
  (Store-Extension 0.16.1).
- **Blockade:** Extension-Änderung an der Store-Extension
  `cabnfapnafjlijmbpmgjkgobhdkbmpci` (dritter).
- **Braucht:** `"alarms"` (≥30 s-Periode) oder Offscreen-Keepalive in der
  Extension; bis dahin Werkzeuge erst nach Executor-Connect
  (`browser_targets` != leer) rufen.
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md

#### FIT-Brücke — echte 945-Aktivität
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945-FIT-Aktivität via USB gemountet.
- **Lage:** (gemessen 2026-09-25 via `sread`) `fit_compiler` (`src/archivar/fit.rs`)
  + Verdrahtung `main_flow.rs` gebaut; die echte 945-FIT-Aktivität fehlt.
- **Blockade:** es liegt keine 945-FIT-Aktivität vor.
- **Braucht:** 945-Aktivität aufzeichnen, per USB mounten und durch `fit_compiler`
  speisen (N-N-Intervalle → `VagusTone`).
- **Quelle:** docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md

#### Geräte-Inventar am Gerät nachmessen (945/Quest/Bigme/Pixel)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** die Geräte liegen am Operator / die Messung wird ausgeführt.
- **Lage:** (gemessen 2026-09-25 via `sread`) 945-Sample-Raten + Chip/FCC,
  Quest-1-Firmware + WebGPU/Generic-Sensor, Bigme-Näherung/Licht/Haptik/WebGPU,
  Pixel-`SensorManager`-Liste + Thread stehen offen; die Pixel-Mikrofonanzahl ist
  seit 2026-09-25 primär gemessen (3) und aus dem Inventar gestrichen.
- **Blockade:** die Messung ist nur am physischen Gerät möglich (Gerätezugriff).
- **Braucht:** `dumpsys sensorservice` (Android: Pixel/Bigme), E-Label/FCC
  (945/Quest), Feature-Detection WebGPU/Generic-Sensor im Quest-Browser.
- **Quelle:** docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) die präregistrierte JUICE-Kette
  wartet auf das Perigäum; RTSW-Retention ~24 h (Auftrag
  `docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
