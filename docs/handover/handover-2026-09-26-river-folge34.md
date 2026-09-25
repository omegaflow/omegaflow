<!--
  title: Handover — River-Folge 34 (2026-09-26)
  session: River-Folge 34
  class: handover
  date: 2026-09-26
  sha256: b9bd72ab3e6f40307d277f86543029311890e25514d17abf439d36ef3626b5be
  status: live
-->
# Handover — River-Folge 34 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter —, dann chronologisch**. Jeder Punkt trägt
**Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge33.md`.

**Wort | Datum | Quelle**
- health-check Variante 2 (erst messen, dann Cap-Wort) | 2026-09-25 | Operator-Wort im Plan-Pass (folge32).

## Stehender Pass (measured)

- **Postfach:** zitiert `docs/zustand/external-state.md` Postfach-Zeile (2026-09-26,
  Mountain-Folge 166): `state/mail/mail_ledger.φ` present (157 Zeilen), jüngste Eingänge
  Registrierung/Verify, keine fällige Korrespondenz. Der `mail_digest`-Befund „ledger
  absent" bleibt das Pfad-Artefakt (`state/` = privates Repo).
- **CI:** (gemessen 2026-09-26 via `ci_manage list`/`view` + `gh workflow run`) beim
  Session-Start REST HTTP 403 (GitHub-API-Rate-Limit, user `295896184`); gegen Session-Ende
  antwortet `ci_manage view 36194355313` wieder (REST frei, Lauf `pending`), `gh workflow
  run` bleibt GraphQL-blockiert (`GraphQL: API rate limit already exceeded`).
- **Safety-Snapshot:** `refs/safety/1790375657`.
- **`register_lookup --orphan-docs`:** 40 trägerlose Prosadokumente (folge33: 42 → 2
  getragen). Klassifikation (2026-09-26, `explore`): **1 River-Träger**
  (`docs/paper/gic-causal-driver.md` → in den gic-Punkt gefaltet), **1 River-Einzelpunkt**
  (Presence-Weltlinie, aus `survey-2026-09-17-omegaflow-legacy-konzepte.md:97` → Rat),
  **38 Owner-assigniert** (mountain/mycelium/sensory/science — Trägerpflicht bei den Ownern,
  nicht River).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic — Auflösung gebaut+committet, Dispatch durch Rate-Limit blockiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GitHub-API-Quota-Reset.
- **Lage:** (gemessen 2026-09-26 via `git log` + `ci_manage`/`gh`) die Instrumente sind
  **committet und gepusht** (`f916093ee river: build gic PCMCI cross-check + full-lag
  family bound, add SOD-2025 shards`; HEAD `4554fadc` == `origin/main`) — die
  folge33-Angabe „uncommitted" ist damit überholt. Das Träger-Papier
  `docs/paper/gic-causal-driver.md` trägt den offenen Marker (PCMCI cross-check +
  full-lag-sweep family bound „results pending", Z. 17). `gh workflow run
  bz-retro-probe.yml` → GraphQL 403, **kein Dispatch** (REST `ci_manage view` antwortet
  nach dem Push wieder). Die zwei Zeugen (Jahres-Pfeil vs. gehärteter Quartals-bound)
  stehen weiter als `Riss`, gebaut-aber-ungemessen.
- **Blockade:** GitHub-API-GraphQL-Quota (REST frei, `gh workflow run` → 403).
- **Braucht:** nach Quota-Reset `gh workflow run bz-retro-probe.yml`, Run-ID einmal per
  `ci_manage view <id>` lesen, die zwei Zeugen gegen den einen Bound auswerten.

#### health-check — Verdikt am Fix messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36194355313` ist beendet **und** die API-Quota ist zurück.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`) Lauf `36194355313` (HEAD
  `42471cde`, attempt 1) ist `pending`, nicht beendet. Der neue Tone→Apertur-Gate-Test
  (`mathematikerin/tests.rs:547`) hängt an der CI-Verifikation des Folge-33-HEAD; lokal
  `cargo check --tests` grün.
- **Blockade:** keine — wartet auf das Lauf-Ende (REST frei).
- **Braucht:** `ci_manage view 36194355313`, bei Rot `ci_manage log 36194355313`.

### Rat handelt

#### Presence-Weltlinien-Träger
- **Status:** blockiert | **Bindung:** rat
- **Trigger:** Rat-Sitzung.
- **Lage:** (gemessen 2026-09-26 via `sread` `relay.rs:590` + `main_flow.rs:817`) der
  `presence_tx`-Kanal trägt ein Einzel-Setzpunkt-Paket aus 10 Feldern
  (`pt, px, py, pz, pr, vx, vy, vz, tt, gs`) — **keine Bahn-/Weltlinien-Auswahl**; die
  Set-Site (`main_flow.rs:830-838`) setzt Position/Velocity je Paket. Die Survey-Frage
  (`survey-2026-09-17-omegaflow-legacy-konzepte.md:97`) „Ungemessen bleibt, ob eine
  Operator-gewählte Weltlinie durch diesen Kanal getragen wird" ist damit als **nicht
  getragen** gemessen.
- **Blockade:** Architektur-Entscheidung (Rat).
- **Braucht:** Rat beurteilt, ob eine Weltlinien-Auswahl (Bahn statt Setzpunkt) im Kanal
  gebaut werden soll — die Präsenz ruht, kein Selbstantrieb.

### Operator handelt

#### Operator-Queue — einfach, ein Akt je Eintrag
Je Eintrag: **Lage** (ein Satz) · **Frage** · **bei Ja** · **bei Nein**.

1. **Funk-Sensor über HTTPS** — Lage: kabellos braucht Verschlüsselung. Frage: lokale CA + Zertifikat erzeugen und stunnel starten? Ja: `stunnel bin/relay-tls.stunnel.conf`, `ca.pem` am Handy installieren, `https://<lan-ip>:1619/consent?ja`. Nein: bleibt am Kabel.
2. **Sonnenfarbe sichtbar** — Lage: Farbmodus gebaut, GPU-Ausführung ungemessen. Frage: Lauf mit `color: measured` starten? Ja: Farbe wird gemessen. Nein: ungemessen.
3. **Sensor am Kabel** — Lage: Vorbereitung steht. Frage: sichtbaren Lauf starten? Ja: `adb devices && adb reverse tcp:1618 tcp:1618`, dann `bin/omegaflow` ohne `OMEGAFLOW_HIDDEN`. Nein: nichts.
4. **Chrome-Debugger** — Lage: DevTools-MCP nicht angebunden. Frage: Debugger-Rechte am laufenden Chrome geben? Ja: MCP 1.9.0 pinnen + hängen. Nein: keine Einsicht.
5. **Kaltstart der Browser-Extension** — Lage: Kaltstart 0–2276 s; Fix ist eine Extension-Änderung beim Dritten. Frage: Änderung anstoßen (`chrome.alarms`)? Ja: Dritter ändert die Store-Extension. Nein: Werkzeuge erst nach Executor-Connect.
6. **FIT-Aktivität** — Lage: Brücke gebaut, echte Daten fehlen. Frage: 945-Aktivität aufzeichnen und einlesen? Ja: per USB mounten → `fit_compiler`. Nein: leer.
7. **Geräte-Inventar** — Lage: 945/Quest/Bigme/Pixel ungemessen. Frage: am Gerät nachmessen? Ja: `dumpsys sensorservice`, E-Label/FCC, WebGPU/Generic-Sensor-Detect. Nein: offen.
8. **Hardware beschaffen** — Lage: BOM bestellfertig. Frage: BOM bestellen? Ja: AliExpress-Login + Bestellung. Nein: Sensor-Bindung bleibt ohne Hardware.
9. **Sensor-Bindung vC** — Lage: Pfad + HRV-Reader gebaut, Hardware fehlt. Frage: nach Anschluss messen? Ja: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>`. Nein: pending.

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum Stunnel-Start.
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Spec `docs/specs/relay-tls-terminator.md` +
  `bin/relay-tls.stunnel.conf` gebaut; die exakten `openssl`-Befehle (CA + Leaf mit
  SAN `IP:<lan-ip>` + Phone-Trust) stehen in `bin/relay-tls.stunnel.conf:14-35`, auf
  `state/tls/relay-leaf.pem/.key` + `state/tls/ca.pem` abgestimmt (noch absent,
  Erzeugung ist der Akt).
- **Blockade:** geräteseitige CA operator-gebunden.
- **Braucht:** Operator: CA + Leaf-Cert (SAN = LAN-IP) erzeugen, `ca.pem` installieren,
  `stunnel bin/relay-tls.stunnel.conf`, `https://<lan-ip>:1619`.

#### Sonnenfarbe: erster Lauf `color: measured`
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren/hidden Lauf.
- **Lage:** (gemessen 2026-09-25 via git, Commit `2de359982`) Farbmodus gebaut;
  Parity-Gate grün; die WGSL-Ausführung im Lauf ist ungemessen.
- **Blockade:** GPU-Lauf = Heavy compute (CI/Operator-Wort).
- **Braucht:** der erste Lauf, der `color: measured` rendert.

#### Sensor-Bindung vC-Permeabilität
- **Status:** operator-gebunden (wartet auf Hardware) | **Bindung:** operator
- **Trigger:** Smartwatch + Mantis-Shrimp sind angeschlossen.
- **Lage:** der Permeabilitäts-Pfad steht (`src/mathematikerin/omega.rs:200/349`), der
  HRV-Reader ist gebaut (`src/archivar/ble.rs:715/926`).
- **Blockade:** Hardware fehlt.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann
  `perm_target_probe --live <pfad>`.

#### HRV/Puls-Arrival → Radiations-Pfad (Code gebaut, Hardware absent)
- **Status:** operator-gebunden (Hardware) | **Bindung:** operator
- **Trigger:** ESP32-Firmware/Watch liefert den Puls-Arrival auf dem Kanal (`nn`/`rr`/`ibi`).
- **Lage:** (gemessen 2026-09-26 via `sread`+`sgrep`) der Code-Pfad steht:
  `feed_beat_to_hrv` (`main_flow.rs:79`) speist Beats aus beiden Kanälen
  (`main_flow.rs:1067/1078`) in das HRV-Gate (`VagusTone` → `tone_code`); der ω-Loop
  liest `tone_code` und relaxiert `tone_scale` (`omega.rs:1670-1678`), Apertur =
  `field_permeability * tone_scale` (`omega.rs:349`); Gate-Test
  `the_tone_code_relaxes_the_aperture_between_floor_and_unity`
  (`mathematikerin/tests.rs:547`). Absent ist allein der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal (Firmware/Hardware ungemessen).
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann messen, dass `tone_code`
  in den Stress-Zustand wechselt und `tone_scale` gegen `TONE_FLOOR_SCALE` relaxiert.

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (`relay.rs:11/73`, `main_flow.rs:612`, Consent
  `relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal).
- **Braucht:** `adb devices && adb reverse tcp:1618 tcp:1618`, dann `bin/omegaflow`
  **ohne** `OMEGAFLOW_HIDDEN`.

#### Akt: Sensor-Hardware beschaffen (Bestellung)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator bestellt die BOM und schließt die Hardware an.
- **Lage:** die BOM ist bestellfertig (`docs/specs/mantis-shrimp-bom.md`).
- **Blockade:** Beschaffung/Kosten; kein Node-Teil vorhanden.
- **Braucht:** Operator bestellt die BOM-Positionen (AliExpress-Login).

#### Chrome DevTools MCP anbinden (Membran-Debug, Pfad i)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am live Chrome).
- **Lage:** (gemessen 2026-09-25 via `sread` tools-map) Chrome DevTools MCP ist „noch
  nicht angebunden"; npm 1.9.0, Telemetrie-Flags `--no-usage-statistics`
  `--no-performance-crux` sind Bedingung.
- **Blockade:** Debugger-Rechte am live Chrome (Operator-Wort).
- **Braucht:** Operator-Wort; dann MCP 1.9.0 pinnen, an den Pfad-1-Chrome hängen, ein
  Membran-Lauf mit gelesener Konsole/Netz.
- **Vorbereitet (2026-09-25):** Block für die globale `~/.config/opencode/opencode.jsonc`
  als zweite `mcp`-Zeile neben `playwright`. `--autoConnect` hängt an den laufenden
  Default-Profil-Chrome (Chrome ≥144, Operator aktiviert
  `chrome://inspect/#remote-debugging`); kein Port zu erfinden. Fallback `--browserUrl`
  braucht einen separaten Chrome mit `--remote-debugging-port=<PORT>` + non-default
  `--user-data-dir`; `<PORT>` = `pending`.
  `"chrome-devtools": { "type": "local", "command": ["npx","-y","chrome-devtools-mcp@1.9.0","--autoConnect","--no-usage-statistics","--no-performance-crux"], "enabled": true }`
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md; npm
  registry.npmjs.org/chrome-devtools-mcp/1.9.0 (gitHead `1cec9cd1`).

#### MV3-Kaltstart-Latenz der Pfad-1-Extension entschärfen
- **Status:** operator-gebunden | **Bindung:** dritter
- **Trigger:** Operator-Wort / Extension-Änderung.
- **Lage:** (gemessen 2026-09-25 via `sread`) Kaltstart 0 s bis 2276 s, bimodal; Ursache
  ist der `setTimeout`-Backoff im MV3-Service-Worker ohne `chrome.alarms`
  (Store-Extension 0.16.1).
- **Blockade:** Extension-Änderung an der Store-Extension
  `cabnfapnafjlijmbpmgjkgobhdkbmpci` (dritter).
- **Braucht:** `"alarms"` (≥30 s-Periode) oder Offscreen-Keepalive in der Extension; bis
  dahin Werkzeuge erst nach Executor-Connect (`browser_targets` != leer) rufen.
- **Vorbereitet (2026-09-25):** in `manifest.json` die Berechtigung `"alarms"` ergänzen
  und den `setTimeout`-Backoff durch `chrome.alarms.create("keepalive",
  {periodInMinutes: 0.5})` + `chrome.alarms.onAlarm` ersetzen (30 s =
  MV3-Mindestperiode); Alternative Offscreen-Document-Keepalive. Die genaue Worker-Stelle
  ist ungemessen (liegt beim Dritten).
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md

#### FIT-Brücke — echte 945-Aktivität
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945-FIT-Aktivität via USB gemountet.
- **Lage:** (gemessen 2026-09-25 via `sread`) `fit_compiler` (`src/archivar/fit.rs`) +
  Verdrahtung `main_flow.rs` gebaut; die echte 945-FIT-Aktivität fehlt.
- **Blockade:** es liegt keine 945-FIT-Aktivität vor.
- **Braucht:** 945-Aktivität aufzeichnen, per USB mounten und durch `fit_compiler`
  speisen (N-N-Intervalle → `VagusTone`).
- **Quelle:** docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md

#### Geräte-Inventar am Gerät nachmessen (945/Quest/Bigme/Pixel)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** die Geräte liegen am Operator / die Messung wird ausgeführt.
- **Lage:** (gemessen 2026-09-25 via `sread`) 945-Sample-Raten + Chip/FCC,
  Quest-1-Firmware + WebGPU/Generic-Sensor, Bigme-Näherung/Licht/Haptik/WebGPU,
  Pixel-`SensorManager`-Liste + Thread stehen offen; die Pixel-Mikrofonanzahl ist seit
  2026-09-25 primär gemessen (3) und aus dem Inventar gestrichen.
- **Blockade:** die Messung ist nur am physischen Gerät möglich (Gerätezugriff).
- **Braucht:** `dumpsys sensorservice` (Android: Pixel/Bigme), E-Label/FCC
  (945/Quest), Feature-Detection WebGPU/Generic-Sensor im Quest-Browser.
- **Quelle:** docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) die präregistrierte JUICE-Kette wartet auf
  das Perigäum; RTSW-Retention ~24 h (Auftrag `docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
