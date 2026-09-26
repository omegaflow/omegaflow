<!--
  title: Handover — River-Folge 35 (2026-09-26)
  session: River-Folge 35
  class: handover
  date: 2026-09-26
  sha256: e188f292ab6565c9f82fc9153f07bced6a1fa475ba34de4d10950ba88bf8ccff
  status: live
-->
# Handover — River-Folge 35 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter —, dann chronologisch**. Jeder Punkt trägt
**Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge34.md`.

**Wort | Datum | Quelle**
- GIC-Papier hat Priorität 1 | 2026-09-26 | Operator-Wort im River-Pass (folge35).
- Hardware-Inventar | 2026-09-26 | Operator-Wort (folge35): vorhanden sind Laptop, Hibreak Pro (Bigme), Pixel 9, Forerunner 945, Meta Quest 1 (?); alles andere nicht vorhanden.
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: den Plan ausführen (`/river_go`); Commit trägt `/commit`.

## Stehender Pass (measured)

- **Postfach:** `state/mail/mail_ledger.φ` = privates Repo (`state/`) — hier absent;
  `mail_digest` „ledger absent" ist das Pfad-Artefakt. `docs/zustand/external-state.md`
  (2026-09-26, Mountain-Folge 166): keine fällige Korrespondenz.
- **CI-Status:** (gemessen 2026-09-26 via `ci_manage view`/`log`) `health-check 36194355313`
  = `completed / failure`, **transient**: `403` Rate-Limit (user `295896184`) + Zenodo
  `curl (28)` Timeout + `runner shutdown` → *cancelled* — kein Assertion-Rot. Rerun
  dispatcht (`gh run rerun 36194355313`), Status `queued`. `bz-retro-probe 36224176888`
  (gic) = `in_progress` (08:36-Snapshot).
- **Kanten-Arbeit (diese Session):** TLS-Material erzeugt — `state/tls/ca.pem` und
  `state/tls/ca.key` (`CN=omegaflow-relay-ca`), `state/tls/relay-leaf.pem` und
  `state/tls/relay-leaf.key` (Leaf `CN=omegaflow-relay`, SAN `IP:<lan-ip>`, von der CA
  signiert), OpenSSL 3.0.13; `state/tls/` ist gitignored.
- **Safety-Snapshot:** `refs/safety/1790404589`.
- **`register_lookup --orphan-docs`:** 38 trägerlose Prosadokumente — alle Owner-assigniert
  (mountain/mycelium/sensory/science), **keine** River-Trägerpflicht mehr. `--stale --persist 3`
  = 0, `--descoped-check` = 0, `--orphans` = 9 (nur future/mycelium).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic — dispatcht, Ergebnis am Lauf messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36224176888` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `gh workflow run` + `ci_manage view` + `git log`) die
  Instrumente sind committet und gepusht (`f916093ee river: build gic PCMCI cross-check +
  full-lag family bound, add SOD-2025 shards`). `gh workflow run bz-retro-probe.yml` gelang
  (Run `36224176888`, HEAD `5eeb76a`); Stand 2026-09-26 (08:36) `in_progress`. Das
  Träger-Papier `docs/paper/gic-causal-driver.md` ist auf „built and dispatched 2026-09-26
  (run 36224176888); results pending" gesetzt (Abstract Z.17 + Offene-Punkte Z.359). Die zwei
  Zeugen (Jahres-Pfeil vs. gehärteter Quartals-bound) stehen ungeglättet als `Riss`,
  dispatcht-aber-ungemessen.
- **Blockade:** keine — wartet auf das Lauf-Ende.
- **Braucht:** `ci_manage view 36224176888` (bei Rot `ci_manage log 36224176888`), PCMCI-Zeile +
  full-lag-Bound gegen die zwei Zeugen auswerten, dann `docs/paper/gic-causal-driver.md` auf das
  Ergebnis setzen.

#### health-check — Verdikt am Rerun messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rerun `36194355313` (nächster Versuch) ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`/`log`) attempt 1 (HEAD `42471cde`) =
  `completed / failure`, gemessen **transient**: `403` Rate-Limit + Zenodo `curl (28)` Timeout
  + `runner shutdown` → *cancelled* — kein Assertion-Rot. Rerun dispatcht (`gh run rerun
  36194355313`), Status `queued`. Der Tone→Apertur-Gate-Test (`mathematikerin/tests.rs:547`)
  hängt an dieser CI-Verifikation; lokal `cargo check --tests` grün.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36194355313` nach Rerun-Ende, bei Rot `ci_manage log 36194355313`.

### Operator handelt

#### Operator-Queue — einfach, ein Akt je Eintrag
Je Eintrag: **Lage** (ein Satz) · **Frage** · **bei Ja** · **bei Nein**.

1. **Funk-Sensor über HTTPS** — Lage: CA + Leaf erzeugt (state/tls), conf steht. Frage: `ca.pem` am Handy installieren und stunnel starten? Ja: `ca.pem` am Handy installieren, `stunnel bin/relay-tls.stunnel.conf`, `bin/omegaflow` (HIDDEN unset), `https://<lan-ip>:1619/consent?ja`. Nein: bleibt am Kabel.
2. **Sonnenfarbe sichtbar** — Lage: Farbmodus gebaut, GPU-Ausführung ungemessen. Frage: Lauf mit `color: measured` starten? Ja: Farbe wird gemessen. Nein: ungemessen.
3. **Sensor am Kabel** — Lage: Vorbereitung steht. Frage: sichtbaren Lauf starten? Ja: `adb devices && adb reverse tcp:1618 tcp:1618`, dann `bin/omegaflow` ohne `OMEGAFLOW_HIDDEN`. Nein: nichts.
4. **Chrome-Debugger** — Lage: DevTools-MCP nicht angebunden. Frage: Debugger-Rechte am laufenden Chrome geben? Ja: MCP 1.9.0 pinnen + hängen. Nein: keine Einsicht.
5. **Kaltstart der Browser-Extension** — Lage: Kaltstart 0–2276 s; Fix ist eine Extension-Änderung beim Dritten. Frage: Änderung anstoßen (`chrome.alarms`)? Ja: Dritter ändert die Store-Extension. Nein: Werkzeuge erst nach Executor-Connect.
6. **FIT-Aktivität** — Lage: Brücke gebaut, echte Daten fehlen. Frage: 945-Aktivität aufzeichnen und einlesen? Ja: per USB mounten → `fit_compiler`. Nein: leer.
7. **Geräte-Inventar** — Lage: 945/Quest/Bigme/Pixel ungemessen. Frage: am Gerät nachmessen? Ja: `dumpsys sensorservice`, E-Label/FCC, WebGPU/Generic-Sensor-Detect. Nein: offen.
8. **Hardware beschaffen** — Lage: BOM bestellfertig. Frage: BOM bestellen? Ja: AliExpress-Login + Bestellung. Nein: Sensor-Bindung bleibt ohne Hardware.
9. **Sensor-Bindung vC** — Lage: Pfad + HRV-Reader gebaut, Hardware fehlt. Frage: nach Anschluss messen? Ja: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>`. Nein: pending.

#### TLS im Relay (wireless) — Kanten-Arbeit erledigt, Phone-Trust offen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Handy und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`) das Material ist erzeugt: `state/tls/ca.pem`
  und `state/tls/ca.key` (`CN=omegaflow-relay-ca`), `state/tls/relay-leaf.pem` und
  `state/tls/relay-leaf.key` (Leaf `CN=omegaflow-relay`, SAN `IP:<lan-ip>`, von der CA
  signiert); `bin/relay-tls.stunnel.conf`
  `cert`/`key` zeigen darauf. `state/tls/` ist gitignored — die LAN-IP lebt im Leaf-SAN, nicht
  in getrackter Prosa. Spec `docs/specs/relay-tls-terminator.md` auf „Measured 2026-09-26" gesetzt.
- **Blockade:** geräteseitiger CA-Trust (Operator).
- **Braucht:** Operator: `ca.pem` am Handy installieren (Android: Einstellungen → Sicherheit →
  Zertifikat installieren → CA), dann `stunnel bin/relay-tls.stunnel.conf`, dann `bin/omegaflow`
  (OHNE `OMEGAFLOW_HIDDEN`), dann `https://<lan-ip>:1619/consent?ja`.

#### Sonnenfarbe: erster Lauf `color: measured`
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren/hidden Lauf.
- **Lage:** (gemessen 2026-09-25 via git, Commit `2de359982`) Farbmodus gebaut; Parity-Gate
  grün; die WGSL-Ausführung im Lauf ist ungemessen.
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
- **Lage:** (gemessen 2026-09-26 via `sread`+`sgrep`) der Code-Pfad steht: `feed_beat_to_hrv`
  (`main_flow.rs:79`) speist Beats aus beiden Kanälen (`main_flow.rs:1067/1078`) in das HRV-Gate
  (`VagusTone` → `tone_code`); der ω-Loop liest `tone_code` und relaxiert `tone_scale`
  (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale` (`omega.rs:349`); Gate-Test
  `the_tone_code_relaxes_the_aperture_between_floor_and_unity` (`mathematikerin/tests.rs:547`).
  Absent ist allein der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal (Firmware/Hardware ungemessen).
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann messen, dass `tone_code` in den
  Stress-Zustand wechselt und `tone_scale` gegen `TONE_FLOOR_SCALE` relaxiert.

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (`relay.rs:11/73`, `main_flow.rs:612`, Consent `relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal).
- **Braucht:** `adb devices && adb reverse tcp:1618 tcp:1618`, dann `bin/omegaflow` **ohne**
  `OMEGAFLOW_HIDDEN`.

#### Akt: Sensor-Hardware beschaffen (Bestellung)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator bestellt die BOM und schließt die Hardware an.
- **Lage:** die BOM ist bestellfertig (`docs/specs/mantis-shrimp-bom.md`).
- **Blockade:** Beschaffung/Kosten; kein Node-Teil vorhanden.
- **Braucht:** Operator bestellt die BOM-Positionen (AliExpress-Login).

#### Chrome DevTools MCP anbinden (Membran-Debug, Pfad i)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am live Chrome).
- **Lage:** (gemessen 2026-09-25 via `sread` tools-map) Chrome DevTools MCP ist „noch nicht
  angebunden"; npm 1.9.0, Telemetrie-Flags `--no-usage-statistics` `--no-performance-crux` sind
  Bedingung.
- **Blockade:** Debugger-Rechte am live Chrome (Operator-Wort).
- **Braucht:** Operator-Wort; dann MCP 1.9.0 pinnen, an den Pfad-1-Chrome hängen, ein
  Membran-Lauf mit gelesener Konsole/Netz.
- **Vorbereitet (2026-09-25):** Block für die globale `~/.config/opencode/opencode.jsonc` als
  zweite `mcp`-Zeile neben `playwright`. `--autoConnect` hängt an den laufenden
  Default-Profil-Chrome (Chrome ≥144, Operator aktiviert `chrome://inspect/#remote-debugging`);
  kein Port zu erfinden. Fallback `--browserUrl` braucht einen separaten Chrome mit
  `--remote-debugging-port=<PORT>` + non-default `--user-data-dir`; `<PORT>` = `pending`.
  `"chrome-devtools": { "type": "local", "command": ["npx","-y","chrome-devtools-mcp@1.9.0","--autoConnect","--no-usage-statistics","--no-performance-crux"], "enabled": true }`
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md; npm
  registry.npmjs.org/chrome-devtools-mcp/1.9.0 (gitHead `1cec9cd1`).

#### MV3-Kaltstart-Latenz der Pfad-1-Extension entschärfen
- **Status:** operator-gebunden | **Bindung:** dritter
- **Trigger:** Operator-Wort / Extension-Änderung.
- **Lage:** (gemessen 2026-09-25 via `sread`) Kaltstart 0 s bis 2276 s, bimodal; Ursache ist der
  `setTimeout`-Backoff im MV3-Service-Worker ohne `chrome.alarms` (Store-Extension 0.16.1).
- **Blockade:** Extension-Änderung an der Store-Extension `cabnfapnafjlijmbpmgjkgobhdkbmpci`
  (dritter).
- **Braucht:** `"alarms"` (≥30 s-Periode) oder Offscreen-Keepalive in der Extension; bis dahin
  Werkzeuge erst nach Executor-Connect (`browser_targets` != leer) rufen.
- **Vorbereitet (2026-09-25):** in `manifest.json` die Berechtigung `"alarms"` ergänzen und den
  `setTimeout`-Backoff durch `chrome.alarms.create("keepalive", {periodInMinutes: 0.5})` +
  `chrome.alarms.onAlarm` ersetzen (30 s = MV3-Mindestperiode); Alternative
  Offscreen-Document-Keepalive. Die genaue Worker-Stelle ist ungemessen (liegt beim Dritten).
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md

#### FIT-Brücke — echte 945-Aktivität
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945-FIT-Aktivität via USB gemountet.
- **Lage:** (gemessen 2026-09-25 via `sread`) `fit_compiler` (`src/archivar/fit.rs`) +
  Verdrahtung `main_flow.rs` gebaut; die echte 945-FIT-Aktivität fehlt.
- **Blockade:** es liegt keine 945-FIT-Aktivität vor.
- **Braucht:** 945-Aktivität aufzeichnen, per USB mounten und durch `fit_compiler` speisen
  (N-N-Intervalle → `VagusTone`).
- **Quelle:** docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md

#### Geräte-Inventar am Gerät nachmessen (Pixel 9/Hibreak Pro/945/Quest 1)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** die Geräte liegen am Operator / die Messung wird ausgeführt.
- **Inventar (gemessen 2026-09-26 via Operator-Wort):** vorhanden sind Laptop, Hibreak Pro
  (Bigme), Pixel 9, Forerunner 945, Meta Quest 1 (Existenz unklar); alles andere nicht vorhanden.
- **Lage:** (gemessen 2026-09-25 via `sread`) 945-Sample-Raten + Chip/FCC, Quest-1-Firmware +
  WebGPU/Generic-Sensor, Bigme-Näherung/Licht/Haptik/WebGPU, Pixel-`SensorManager`-Liste + Thread
  stehen offen; die Pixel-Mikrofonanzahl ist seit 2026-09-25 primär gemessen (3) und aus dem
  Inventar gestrichen.
- **Blockade:** die Messung ist nur am physischen Gerät möglich (Gerätezugriff).
- **Braucht:** `dumpsys sensorservice` (Android: Pixel/Bigme), E-Label/FCC (945/Quest),
  Feature-Detection WebGPU/Generic-Sensor im Quest-Browser.
- **Quelle:** docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) die präregistrierte JUICE-Kette wartet auf das
  Perigäum; RTSW-Retention ~24 h (Auftrag `docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene Abschluss-Check läuft
dann mit Commit und Push. `/consent` ist der session-weite Consent (Delegation), nie das
Commit-Wort.
