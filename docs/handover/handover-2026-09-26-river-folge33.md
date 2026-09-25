<!--
  title: Handover — River-Folge 33 (2026-09-26)
  session: River-Folge 33
  class: handover
  date: 2026-09-26
  sha256: 377a48befdb87a1a95ceda794419f7723d7a3344be4283c9c7a1a184d203c502
  status: live
-->
# Handover — River-Folge 33 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter —, dann chronologisch**. Jeder Punkt trägt
**Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-25-river-folge32.md`.

**Wort | Datum | Quelle**
- health-check Variante 2 (erst messen, dann Cap-Wort) | 2026-09-25 | Operator-Wort im Plan-Pass (folge32).

## Stehender Pass (measured)

- **Postfach:** reachable — `state/mail/mail_ledger.φ` present (158 lines); last
  entries: GitHub-Support `4761801` (PII-Purge) auto-closed, Globus SuperDARN-Mirror
  FAILED (4994 files, 34 GB), OpenAlex/Exa notifications.
- **CI:** `ci_manage list` → HTTP 403 (GitHub API rate limit, `void`); no run
  readable, watchdog snapshot `/tmp/opencode/ci_status.md` (00:16) void.
- **Safety-Snapshot:** `refs/safety/1790374908`.
- **`register_lookup --orphan-docs`:** 42 carrier-less prose documents
  (cross-cutting, not River-owned — carrier per owner handover open).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic — Auflösung gebaut, Dispatch durch Rate-Limit blockiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener Commit+Push **und** GitHub-API-Quota-Reset.
- **Lage:** (gemessen 2026-09-26 via `cargo check` + `gh`) PCMCI-Cross-Check
  (`bz_retro_probe.rs:431`), Full-Lag-Sweep (`bz_retro_probe.rs:780`), SOD-2025-q1…q4-
  Shards (`bz-retro-probe.yml:64`) gebaut; `cargo check -p omegaflow-measure --bin
  bz_retro_probe` → 0 errors/0 warnings. Dispatch `gh workflow run bz-retro-probe.yml`
  → HTTP 403 (API rate limit), **kein Run**. Die zwei Zeugen (Jahres-Pfeil vs.
  gehärteter Quartals-bound) stehen weiter als `Riss`, jetzt gebaut-aber-ungemessen.
- **Blockade:** GitHub-API-Rate-Limit; `gh workflow run` feuert gegen **committed**
  HEAD, die Instrumente liegen uncommitted.
- **Braucht:** nach `/commit`+Push `gh workflow run bz-retro-probe.yml`, Run-ID einmal
  per `ci_manage view <id>` lesen, die zwei Zeugen gegen den einen Bound auswerten.

#### health-check — Verdikt am Fix messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36194355313` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage list`) frischer Lauf `36194355313` auf
  HEAD `42471cde` dispatcht, beim einmaligen Read `queued`; zweiter Alt-Lauf
  `36189444050` in_progress. Der neue Tone→Apertur-Gate-Test
  (`mathematikerin/tests.rs:547`) hängt an der CI-Verifikation des Folge-33-HEAD.
- **Blockade:** keine (lokal `cargo check --tests` grün).
- **Braucht:** `ci_manage view 36194355313`, bei Rot `ci_manage log 36194355313`.

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
