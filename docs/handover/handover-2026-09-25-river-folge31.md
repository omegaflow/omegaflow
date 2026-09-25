<!--
  title: Handover — River-Folge 31 (2026-09-25)
  session: River-Folge 31
  class: handover
  date: 2026-09-25
  sha256: 9379eac9c6846764115bf757b0d4e2b2fd8b886812c04266b87043d74ee757c8
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

#### health-check — Root Cause gemessen, Fix hängt am fremden Baum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Mountain-folge163-Commit (gibt `src/archivar/fetch.rs` frei).
- **Lage:** (gemessen 2026-09-25 via `ci_manage log 36132614446 --all` + `sread`)
  Job `verify (4)` stirbt reproduzierbar (auch Vorlauf `36140394227`) an einem
  hängenden Fetch ohne Total-Timeout: `--verify phi --shard 4/16`
  (`src/archivar/port.rs:2467 shard_bounds`) trifft `phi/sources.φ:7344`
  (`zenodo.org/api/records/21132339/files/data.zip/content`); `src/archivar/fetch.rs`
  setzt nur `--connect-timeout`, kein `--max-time`/`--speed-limit` → Transfer hängt
  bis zum Runner-Kill (kein Assertion-Fehler; Job-Log 404).
- **Blockade:** `src/archivar/fetch.rs` trägt fremde uncommittete Mountain-folge163-Arbeit
  (gzip-Body-Fix) — ein pfad-begrenzter Commit würde fremde Hunks sweepen.
- **Braucht:** `--max-time <N>` (oder `--speed-limit`/`--speed-time`) an die
  curl-Aufrufe in `src/archivar/fetch.rs`; `phi/sources.φ:7344` als `pending`/`void`
  belegen; dann `gh workflow run health-check.yml` statt Re-Dispatch der alten Route.

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** wartend | **Bindung:** eigen/operator
- **Trigger:** kabelloser Sensor wird gebraucht.
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Spec `docs/specs/relay-tls-terminator.md` +
  `bin/relay-tls.stunnel.conf` gebaut.
- **Blockade:** geräteseitige CA operator-gebunden.
- **Braucht:** Operator: lokale CA + Leaf-Cert (SAN = LAN-IP), `stunnel
  bin/relay-tls.stunnel.conf`, `https://<lan-ip>:1619`.

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
