<!--
  title: Handover — River-Folge 31 (2026-09-25)
  session: River-Folge 31
  class: handover
  date: 2026-09-25
  sha256: f58612a1a9d194fdf2d97fc9a71a9d2acf36e9e7a131dcb96a11bacebf4c351d
  status: live
-->
# Handover — River-Folge 31 (2026-09-25)

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

Diese Session konsumierte `handover-2026-09-25-river-folge30.md`.

**Wort | Datum | Quelle**
- Sortierung der Tafel nach Umsetzbarkeit | 2026-09-25 | Operator-Wort im Plan-Pass (ersetzt den stehenden Sort Akteur→chronologisch für diese Linie).
- health-check Variante 2 (erst messen, dann Cap-Wort) | 2026-09-25 | Operator-Wort im Plan-Pass.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic-causal-driver — Sub-6h-Shardung, Re-Dispatch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des re-dispatchen `bz-retro-probe`-Laufs `36176580764` (12 Quartals-Shards).
- **Lage:** (gemessen 2026-09-25 via `ci_manage view 36136095463` + `gh run download`)
  der `minute`-Job war success; die Neu-Messung liegt vollständig:
  `TE(Bz→dB/dt)=1.8094e-1 | threshold 1.8286e-1 | lag 115 min | n 741 | still`;
  `TE(Speed→dB/dt)=2.2118e-1 | threshold 2.5107e-1 | lag 120 min | n 720 | still` —
  beide unter dem Family-Schwellwert, „still is a finding (0 honored)". Die drei
  `hourly`-Volljahre (abk-2024/abk-2025/sod-2024) rissen alle den GitHub-6h-Cap
  (6h01, Annotation `exceeded the maximum execution time of 6h0m0s`); nur
  `abk-2024` trug eine fam-Zeile, abk-2025/sod-2024 endeten vor der Messzeile.
  `.github/workflows/bz-retro-probe.yml` in 12 Quartals-Shards (`timeout-minutes: 300`)
  geteilt. Der CDN-Fallback greift (gemessen 2026-09-25 via `archive_search --sniff`:
  `abk_dbdt_1h.bin`/`sod_dbdt_1h.bin` HTTP 200, ~4,58 MB), also entfällt die
  Jahres-Ernte (`bz_retro_probe.rs:605`); die Kosten sind die TE über das Fenster,
  die Shardung senkt sie auf ~¼. **Namensänderung der Messung:** der Family-Schwellwert
  wird nun je Quartalsfenster gebildet (vorher je Jahr) — jede Shard-Zeile trägt ihre
  eigene Null; quartalsweise Fam-Werte sind untereinander nicht identisch mit dem
  Jahres-Fam.
- **Blockade:** keine.
- **Braucht:** `gh run download 36176580764` → 12 `bz-retro-*-q*.txt`
  (fam-Zeile je Quartal) + `bz-blatt-minute.txt`.

#### health-check — Rats-Verdikt gebaut, Commit blockiert durch fremde Kaskade
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** fremde `ttl`-Kaskade landet im Baum (`cargo check` grün) oder wird verworfen.
- **Lage:** (gemessen 2026-09-25 via `grind-pro` + `git status` + Rat) die Prämisse „kein
  `--max-time`" ist **widerlegt** — `-m` steht auf allen curl-Pfaden
  (`fetch.rs:63/122/220/256/1155`, `http_code:1133`), abgeleitet
  `ttl_transfer_bound(ttl)=ceil(ttl/Φ²)` (`fetch.rs:25`). Quelle zenodo `21132339`
  (`phi/sources.φ:7359` url, `:7363 ttl 86400`) → Bound **33002 s (≈9,17 h)** gegen
  `health-check.yml:30 timeout-minutes: 120` (**7200 s**). Riss: Frische (`ttl`) treibt
  Netz-Geduld; der `-m`-Kill-vor-Verdikt durchtrennt die Messreihe; die Shard-Schleife
  (`port.rs:2522`) ist seriell. Das Rats-Verdikt ist **gebaut** (Hunks liegen im Baum):
  `TRANSFER_BOUND_S = 1<<11` (`fetch.rs:19`), `ttl`-Parameter entfernt (Rufer
  `fetch.rs/range.rs/main_flow.rs` + 3 `tools/harvest`), `--speed-limit 1 --speed-time 128`
  + `--retry-max-time 2048` (`append_retry:32`), Test `tests.rs:5795` neu, Gate-Fixture
  `ttl_transfer_bound_regression` (`commit_gate_vocab.json` + `commit_gate.rs`).
- **Blockade:** eine **fremde, aktive** `ttl`-Entfernungs-Kaskade bewegt dieselben Dateien
  (`fetch.rs`, `main_flow.rs`, `range.rs`, `tests.rs`, `commit_gate*.{rs,json}`, 3
  `tools/harvest`-Bins); `cargo check` rot (23 `E0061`, keiner aus unseren Hunks). Ein
  pfad-begrenzter Commit würde fremde Hunks sweepen — verboten.
- **Braucht:** fremde Kaskade abwarten (oder verwerfen lassen), dann eigene Hunks erneut
  anwenden/committen; Snapshot `refs/safety` trägt beide Stände. `phi/sources.φ` fremd-dirty,
  (c) abgelehnt → `ttl 86400` bleibt. Danach `gh workflow run health-check.yml`.

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** wartend | **Bindung:** eigen/operator
- **Trigger:** kabelloser Sensor wird gebraucht.
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Spec `docs/specs/relay-tls-terminator.md` +
  `bin/relay-tls.stunnel.conf` gebaut.
- **Vorbereitet (2026-09-25 gemessen):** die exakten `openssl`-Befehle (CA + Leaf mit
  SAN `IP:<lan-ip>` + Phone-Trust) stehen bereits in `bin/relay-tls.stunnel.conf:14-35`,
  auf die Pfade `state/tls/relay-leaf.pem/.key` + `state/tls/ca.pem` abgestimmt — nichts
  zu ergänzen; offen ist allein der Operator-Akt (Zertifikate erzeugen, `ca.pem`
  installieren).
- **Blockade:** geräteseitige CA operator-gebunden.
- **Braucht:** Operator: lokale CA + Leaf-Cert (SAN = LAN-IP), `stunnel
  bin/relay-tls.stunnel.conf`, `https://<lan-ip>:1619`.

#### HRV/ESP32-Puls-Bindung → Radiations-Pfad
- **Status:** wartend | **Bindung:** eigen/operator
- **Trigger:** ESP32-Firmware liefert den Puls-Arrival auf dem Pfad.
- **Lage:** (gemessen 2026-09-17 via `survey-2026-09-17-verlorene-diskussionen.md:84`)
  das RMSSD/tone-Gate steht (`src/archivar/hrv.rs`); die Bindung (Puls-Arrival via
  ESP32-Firmware → Radiations-Pfad) ist ungebaut — der Socket hält.
- **Blockade:** die Firmware/Hardware-Bindung ist ungemessen.
- **Braucht:** Puls-Arrival im Radiations-Pfad verdrahten (HRV-Gate → `omega.rs`-
  Apertur/tone-scale) und mit der angeschlossenen Hardware messen.

### Operator handelt

#### Operator-Queue — einfach, ein Akt je Eintrag
Je Eintrag: **Lage** (ein Satz) · **Frage** · **bei Ja** · **bei Nein**. Die Messlage steht im jeweiligen Block darunter.

1. **Funk-Sensor über HTTPS** — Lage: kabellos braucht Verschlüsselung, sonst sperrt der Browser die Sensoren. Frage: lokale CA + Zertifikat erzeugen und stunnel starten? Ja: `stunnel bin/relay-tls.stunnel.conf`, `ca.pem` am Handy installieren, `https://<lan-ip>:1619/consent?ja`. Nein: bleibt am Kabel (kein TLS nötig).
2. **Sonnenfarbe sichtbar** — Lage: Farbmodus gebaut, GPU-Ausführung ungemessen. Frage: Lauf mit `color: measured` starten (still oder sichtbar)? Ja: Farbe wird gemessen. Nein: ungemessen.
3. **Sensor am Kabel** — Lage: Vorbereitung steht. Frage: sichtbaren Lauf starten? Ja: `adb devices && adb reverse tcp:1618 tcp:1618`, dann `bin/omegaflow` ohne `OMEGAFLOW_HIDDEN`. Nein: nichts.
4. **Chrome-Debugger** — Lage: DevTools-MCP nicht angebunden. Frage: Debugger-Rechte am laufenden Chrome geben? Ja: MCP 1.9.0 pinnen + hängen; Konsole/Netz lesbar. Nein: keine Einsicht.
5. **Kaltstart der Browser-Extension** — Lage: Kaltstart 0–2276 s; Fix ist eine Extension-Änderung beim Dritten. Frage: Änderung anstoßen (`chrome.alarms`)? Ja: Dritter ändert die Store-Extension. Nein: Werkzeuge erst nach Executor-Connect.
6. **FIT-Aktivität** — Lage: Brücke gebaut, echte Daten fehlen. Frage: 945-Aktivität aufzeichnen und einlesen? Ja: per USB mounten → `fit_compiler`. Nein: leer.
7. **Geräte-Inventar** — Lage: 945/Quest/Bigme/Pixel ungemessen. Frage: am Gerät nachmessen? Ja: `dumpsys sensorservice`, E-Label/FCC, WebGPU/Generic-Sensor-Detect. Nein: offen.
8. **Hardware beschaffen** — Lage: BOM bestellfertig. Frage: BOM bestellen? Ja: AliExpress-Login + Bestellung. Nein: Sensor-Bindung bleibt ohne Hardware.
9. **Sensor-Bindung vC** — Lage: Pfad + HRV-Reader gebaut, Hardware fehlt. Frage: nach Anschluss messen? Ja: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>`. Nein: pending.

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
- **Vorbereitet (2026-09-25 via `grind-flash` + npm-gitHead-Messung):** Block für die
  globale `~/.config/opencode/opencode.jsonc` als zweite `mcp`-Zeile neben `playwright`
  (Schema gemessen an der bestehenden `playwright`-Zeile). `--autoConnect` hängt an den
  laufenden Default-Profil-Chrome (Chrome ≥144, Operator aktiviert
  `chrome://inspect/#remote-debugging`) — kein Port zu erfinden. Fallback `--browserUrl`
  braucht einen separaten Chrome mit `--remote-debugging-port=<PORT>` + non-default
  `--user-data-dir`; `<PORT>` = `pending`.
  `"chrome-devtools": { "type": "local", "command": ["npx","-y","chrome-devtools-mcp@1.9.0","--autoConnect","--no-usage-statistics","--no-performance-crux"], "enabled": true }`
- **Quelle:** docs/surveys/survey-2026-09-20-browser-anbindung.md; npm
  registry.npmjs.org/chrome-devtools-mcp/1.9.0 (gitHead `1cec9cd1`), docs/configuration.md:37-48.

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
- **Vorbereitet (2026-09-25, Vorschlag an den Dritten; Extension-Quelle nicht im
  Repo — `**/manifest.json`: 0 Treffer):** in `manifest.json` die Berechtigung
  `"alarms"` ergänzen und den `setTimeout`-Backoff durch
  `chrome.alarms.create("keepalive", {periodInMinutes: 0.5})` +
  `chrome.alarms.onAlarm` ersetzen (30 s = MV3-Mindestperiode); Alternative
  Offscreen-Document-Keepalive. Die genaue Worker-Stelle ist ungemessen (liegt
  beim Dritten).
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
