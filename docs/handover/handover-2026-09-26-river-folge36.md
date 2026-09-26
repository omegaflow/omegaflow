<!--
  title: Handover — River-Folge 36 (2026-09-26)
  session: River-Folge 36
  class: handover
  date: 2026-09-26
  sha256: 111810bf873cdc72067182fbf66bd75c7b53b70ed5228adfc9823979f3fc7072
  status: live
-->
# Handover — River-Folge 36 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter —, dann chronologisch**. Jeder Punkt trägt
**Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge35.md`.

**Wort | Datum | Quelle**
- Funk-Sensor über HTTPS: **ja** | 2026-09-26 | Operator-Wort folge36 („1. Ja").
- FIT-Brücke 945: **ja** — Garmin liefert .FIT-Dateien | 2026-09-26 | Operator-Wort folge36 („2. ja aber ich dachte Garmin liefert FIT dateien").
- Puls-Weg: **(b) Live-BLE nach dem Membran-Fix**, kein Brustgurt | 2026-09-26 | Operator-Wort folge36 („b").
- Geräte-Inventar: **Einzelbefehle liefern**, nicht Operator-Messung | 2026-09-26 | Operator-Wort folge36 („3 einzelbefehle").
- Mantis Shrimp (BOM/Beschaffung): **LOCK — kommt zu allerletzt** | 2026-09-26 | Operator-Wort folge36 („nein wie immer MAntisshrimp KOMMT ZU ALLERLETZT … SETZE EIN LOCK DARAUF").
- Harte Läufe: **LOCK — verboten**, bis die Membran sauber nur die Presence lädt | 2026-09-26 | Operator-Wort folge36 („harte läufe sind jetzt nicht erlaubt sie lassen den rechner abstürzen … der archivar alles lädt und die membran anstatt nur die presence").
- Chrome-Debugger andocken: **ja** | 2026-09-26 | Operator-Wort folge36 („6 ja du darfst").
- Browser-Extension: **forken statt Dritten fragen** | 2026-09-26 | Operator-Wort folge36 („7 können wir die extension nicht forken?").
- vC-Permeabilität: **945 und Mantis Shrimp getrennt führen** | 2026-09-26 | Operator-Wort folge36 („8 TRENNEN!!!!!").
- **„what's here now"** ist essentiell: ohne die gefilterte Presence ist es unmöglich | 2026-09-26 | Operator-Wort folge36. Schlagworte dieser Linie: `what's here now`, `Punktwolke`, `agnostisch`, `Enclosure`, `Lichtkegel`/`Signalkegel`, `Sprung`, `Dispersion`, `presence_gate`.
- z.ai-Sessions aus `state/zai-export` per Schlagwortsuche identifiziert und angeschrieben | 2026-09-26 | Operator-Wort folge36 („identifizieren und direkt anschreiben").
- Session-Dokumentation haarklein + Rat-Korrektur | 2026-09-26 | Operator-Wort folge36 („dokumentiere alles haarklein … essentielle themen"). Ergebnis: `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (sha `381ef5d4…`).
- GIC-Papier hat Priorität 1 | 2026-09-26 | Operator-Wort folge35.
- Hardware-Inventar: vorhanden Laptop, Hibreak Pro (Bigme), Pixel 9, Forerunner 945, Meta Quest 1 (?) | 2026-09-26 | Operator-Wort folge35.
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: den Plan ausführen; Commit trägt `/commit`.

## Stehender Pass (measured 2026-09-26)

- **HEAD:** `6de8fa3ff70721cb6300e135bdcb221080dedb5f`.
- **Postfach:** `state/mail/mail_ledger.φ` = privates Repo (`state/`) — hier absent
  (Pfad-Artefakt); `docs/zustand/external-state.md` (Mountain): keine fällige Korrespondenz.
- **Safety-Snapshot:** `refs/safety/1790407176`.
- **open_points_check folge35:** 30 Pfad-Refs, 0 absent, 0 format-gaps, 0 owner-drift.
- **`register_lookup --open`:** 26 zustand-due (Mountain/geteilt), 5 orphan
  (future 4 / mycelium 1), orphan-docs 38 — alle fremd-owner-assigniert, keine River-Trägerpflicht.
- **CI (Watchdog 2026-09-26):** in_progress register-dropped/ci-check/allwise-cdn;
  `paper-check` rot (`big-bang-echo-sheet-12` = Sensory), gic-Abstract behoben;
  gic-Lauf `36224176888` / health-check-Rerun `36194355313` am Lauf messen.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### Membran lädt nur die Presence (Archivar-Ladepfad) — Fix-Fläche gemessen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** gebaut.
- **Lage:** (gemessen 2026-09-26 via explore/research + Rat) Die Physik ist gebaut —
  Enclosure Lemma (Dilatation `rmax + anchor_vmax·Δt + ½·anchor_amax·Δt² + extent`,
  `spatial.rs:509-574`), Signalkegel (`signal_reach = v_force·age`, `membrane.rs:336-351`,
  Gate `age > ttl·2⁶`), Sprung (`jump_epoch`, `main_flow.rs:817-828`), proaktives Laden
  (`presence_gate`, `fetch.rs:412-449`) und `record_in_enclosure` (`fetch.rs:464-507`).
  Der harte Lauf stürzt ab, weil **drei heiße Pfade die gebauten Gates umgehen**:
  (1) Sterne mit `extent = f64::INFINITY` → `hash.unbounded` → `query_hash` iteriert jede
  Frame JEDEN Stern ohne Kegel (`spatial.rs:348,393-505`); (2) Bootstrap lädt alle Bodies
  (`main_flow.rs:180-223`); (3) Katalog-Branches tycho/dastcom ohne Kegel
  (`main_flow.rs:1673-1750/1224-1292`), netcdf/opendap nutzen nur `record_in_enclosure`.
- **Blockade:** keine.
- **Braucht (korrigiertes Rat-Verdikt + Review-Verfeinerung 2026-09-26):**
  **(a) Stern-Gitter (Membran-scoped Cache).** Stern-Klasse behält `extent = ∞` (Draht
  0.0) — **kein radialer Radius** (Atom 8: gemessener Physikfehler), **kein endliches
  Stern-`extent`** (Atom-6-`cell_size`-Regression). Eigenes Gitter mit **Newtype
  `StarCellKey((i64,i64,i64))`** (nie mit dem bounded `CellKey` gemischt), **rigides
  `cell_size_star`** (erbt die organische Dilatation NICHT — Erben kollabiert auf 8 kpc →
  eine Zelle → O(N); 1 pc → 4·10¹² Zellen), abgeleitet aus der Stern-**Positions**-
  Verteilung per Ziel-Besetzung (~1–10 Sterne/belegte Zelle, grob p50–p75 des nächsten
  Nachbarabstands) — **nicht** `span/1024` (15 pc < ~30 pc mittlerer Abstand → ~10⁷ leere
  Zellen). Span aus dem **committeten** Katalog-Span (nicht dem ladereihenfolgen-
  abhängigen Live-Span), Rehash nur bei >2× Span-Änderung (Hysterese). Query-Hülle
  `rho_star = c·age + pad`, nur Zellen im Box-Schnitt iterieren, Diode (val-Gate vor
  `motion.at`, Quergate nach) läuft innen. `StarCellKey` an der `motion.at`-Position
  (retardierte Epoche). **Kapazitätsprüfung vor dem Bau:** Offline-Pass misst die
  Hüllen-Fraktion (Selektion vs. Evaluation), sonst evtl. kein Gewinn.
  **(b)** Ein `enclosure_rho(vmax, amax, dt, pad)` für alle Ladepfade (`spatial.rs:511`,
  `:554`, `fetch.rs:492`).
  **(c) Sprung Vektorform** `|dp − v·Δt| ≥ Φ·JUMP_GRID + ½·amax·Δt²` (kein transversaler
  Blindkegel, kein Thrash bei Dauer-Schub, `v=0` fällt gratis); `½|a|Δt²` bleibt in der
  Dilatation.
  **(d)** Reader `value >= 0.0` statt `is_finite()`.
  **(e)** Gewebtes (Dichte/TE/Verdict) als abgeleiteter Query-Term im ω-Loop, kein Sample-Slot.
  **RISS (getragen, nicht geglättet):** Claude Max + Session 18 fordern einen Insert-Fix
  (`extent` endlich = `min(floor_radius, v_force·2⁶·ttl)` als einziger Normalisierungspunkt,
  damit kein Pfad ∞ sieht) — der Legacy-Baum maß den radialen `d² ≤ val/floor` als
  Physikfehler. Zwei unabhängige Linien, die sich weigern zu konvergieren; Auflöser ist
  die Messung der realen `d_max`-Verteilung, nie ein Mittelwert.
  Voll-Doku: `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`. Kandidat
  `grind-max`/`grind-pro`. Danach fällt der Harte-Läufe-LOCK.

#### gic — Ergebnis am Lauf messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36224176888` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`) Instrumente committet und gepusht
  (`f916093ee`), Lauf `36224176888` gestartet; die zwei Zeugen (Jahres-Pfeil vs.
  gehärteter Quartals-bound) stehen ungeglättet als `Riss`. Träger-Papier
  `docs/paper/gic-causal-driver.md` auf „built and dispatched … results pending".
- **Blockade:** keine — wartet auf das Lauf-Ende.
- **Braucht:** `ci_manage view 36224176888` (bei Rot `ci_manage log 36224176888`),
  PCMCI-Zeile + full-lag-Bound gegen die zwei Zeugen, dann Paper auf das Ergebnis setzen.

#### health-check — Verdikt am Rerun messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rerun `36194355313` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`/`log`) attempt 1 = transient
  (`403` Rate-Limit + Zenodo `curl (28)` Timeout + `runner shutdown`); Rerun dispatcht.
  Tone→Apertur-Gate-Test (`mathematikerin/tests.rs:547`) hängt daran.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36194355313`, bei Rot `ci_manage log 36194355313`.

#### paper-check — gic fällt aus der `->`-Liste
- **Status:** wartend | **Bindung:** eigen (Rest fremd: Sensory)
- **Trigger:** jüngster `paper-check`-Lauf nach Push.
- **Lage:** (gemessen 2026-09-26 via `ci_manage log`) gic-Abstract 302→194 W,
  Header-sha `1aada7d3…` deckt; `big-bang-echo-sheet-12` Header ≠ Body bleibt
  Sensory-Eigentum (`handover-2026-09-26-sensory-folge174.md`).
- **Blockade:** `big-bang-echo-sheet-12` (fremde Linie).
- **Braucht:** `ci_manage list` → jüngster `paper-check`; der gic-Eintrag muss aus der
  `->`-Liste fallen.

#### Browser-Extension forken (MV3-Kaltstart)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Fork gebaut und unpacked geladen.
- **Lage:** (gemessen 2026-09-26 via `sread`) Store-Extension 0.16.1
  (`cabnfapnafjlijmbpmgjkgobhdkbmpci`) ohne `chrome.alarms`, `setTimeout`-Backoff,
  Kaltstart 0–2276 s; Paket `@vymalo/opencode-browser` (npm, Protokoll v1).
- **Blockade:** keine — Fork ist eigenes Handeln, kein Akt beim Dritten.
- **Braucht:** prüfen, ob das npm-Paket die MV3-Quelle (`extension/`) mitliefert; wenn ja
  forken, `setTimeout`-Backoff durch `chrome.alarms.create("keepalive",{periodInMinutes:0.5})`
  + `chrome.alarms.onAlarm` ersetzen, unpacked im Operator-Profil laden; wenn nur Bundle,
  ersten Schritt die Quelle finden.

#### Chrome DevTools MCP anbinden
- **Status:** eigen (Operator-Wort steht) | **Bindung:** eigen
- **Trigger:** MCP gepinnt und am laufenden Chrome.
- **Lage:** (gemessen 2026-09-25 via `sread` tools-map) MCP nicht angebunden;
  npm 1.9.0, Flags `--no-usage-statistics` `--no-performance-crux`.
- **Blockade:** keine (Operator hat Debugger-Rechte gegeben).
- **Braucht:** MCP 1.9.0 pinnen, an den Pfad-1-Chrome hängen
  (`--autoConnect`, Operator aktiviert `chrome://inspect/#remote-debugging`), ein
  Membran-Lauf mit gelesener Konsole/Netz — letzterer unter dem Harte-Läufe-LOCK.

#### Einzelbefehle fürs Geräte-Inventar bereitstellen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Befehle geliefert.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) Operator will die Einzelbefehle,
  nicht selbst suchen.
- **Blockade:** keine.
- **Braucht:** die Messbefehle kopierbar liefern: `dumpsys sensorservice` (Pixel 9/Bigme),
  E-Label/FCC (945/Quest), WebGPU/Generic-Sensor-Detect im Quest-Browser.

#### Sonnenfarbe `color: measured` — erster Lauf
- **Status:** eigen (CI/Lauf) | **Bindung:** eigen
- **Trigger:** Lauf mit `color: measured`.
- **Lage:** (gemessen 2026-09-25 via git, Commit `2de359982`) Farbmodus gebaut,
  Parity-Gate grün; die WGSL-Ausführung im Lauf ist ungemessen.
- **Blockade:** ein sichtbarer/harter Lauf steht unter dem Harte-Läufe-LOCK; ein
  compute-only CI-Lauf ist der Weg.
- **Braucht:** CI-Lauf mit `color: measured` dispatchen (kein Operator-Lauf).

### Operator handelt

#### Puls-Pfad 945 — Weg (b): Live-BLE-HR nach dem Membran-Fix
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Membran lädt nur die Presence — das Stern-Gitter in `src/archivar/spatial.rs` ist gebaut; der Harte-Läufe-LOCK fällt.
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow` + Backup) die 945 wurde gesichert
  (`data/garmin-945/2026-09-26/`, 36 MB: 25 Aktivitäten + Monitor/Metrics + Musik, ohne
  Karten-`.img`); der Standalone-Parse liest jede Datei, **alle 134 `.fit`/`.FIT` tragen
  `nn: 0`** (keine `hr`-Nachricht/global 132, nur `record`-HR). RR liefert die Uhr nur
  live über BLE — `decode_hr_measurement` liest RR (`src/archivar/ble.rs:1220`), das
  läuft in der Membran (`ble_ingress`).
- **Blockade:** Harte-Läufe-LOCK (Membran-/Archivar-Bug).
- **Braucht:** nach dem Membran-Fix `OMEGAFLOW_HIDDEN=1 bin/omegaflow` mit BLE-HR der 945;
  dann messen, dass `tone_code` in den Stress-Zustand wechselt und `tone_scale` relaxiert.
- **Wort:** Puls-Weg (b) Live-BLE nach Membran-Fix | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig (ohne Platine)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via `sread`+`sgrep`) der Pfad steht
  (`src/mathematikerin/omega.rs:200/349`), HRV-Reader gebaut (`src/archivar/ble.rs:715/926`);
  das 945 ist physisch vorhanden.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann
  `perm_target_probe --live <pfad>` — der Run unter dem Harte-Läufe-LOCK.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### HRV/Puls-Arrival → Radiations-Pfad (Quelle: 945 per BLE)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Membran-Fix (`src/archivar/spatial.rs` Stern-Gitter) ist gebaut, dann 945-BLE-HR live (Weg (b)).
- **Lage:** (gemessen 2026-09-26 via `sread`+`sgrep`) Code-Pfad steht: `feed_beat_to_hrv`
  (`main_flow.rs:79`) speist Beats in das HRV-Gate (`VagusTone` → `tone_code`); ω-Loop
  relaxiert `tone_scale` (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale`
  (`omega.rs:349`); Test `tests.rs:547`. Absent ist der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann `tone_code`-Stresswechsel +
  `tone_scale`-Relaxation messen.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Handy und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`) das TLS-Material ist erzeugt und liegt
  gitignored im state-Verzeichnis (ca.pem/ca.key `CN=omegaflow-relay-ca`,
  relay-leaf.pem/relay-leaf.key `CN=omegaflow-relay`, SAN `IP:<lan-ip>`, von der CA signiert);
  `bin/relay-tls.stunnel.conf` zeigt darauf. Spec `docs/specs/relay-tls-terminator.md`
  auf „Measured 2026-09-26".
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
- **Blockade:** LOCK (Operator-Wort: „kommt zu allerletzt").
- **Braucht:** Operator-Wort zum Aufheben; dann BOM bestellen (AliExpress-Login).
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar am Gerät nachmessen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt die gelieferten Einzelbefehle aus.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) vorhanden Laptop, Hibreak Pro
  (Bigme), Pixel 9, Forerunner 945, Meta Quest 1 (Existenz unklar); Pixel-Mikrofonanzahl
  primär gemessen (3). Offen: 945-Sample-Raten + Chip/FCC, Quest-1-Firmware + WebGPU/
  Generic-Sensor, Bigme-Näherung/Licht/Haptik/WebGPU, Pixel-`SensorManager`-Liste.
- **Blockade:** Messung nur am physischen Gerät.
- **Braucht:** die gelieferten Einzelbefehle ausführen (siehe Linie-Punkt).
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### Harte Läufe (sichtbar/hidden) — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Membran-Reparatur (Archivar lädt nur die Presence) ist gebaut.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) harte Läufe stürzen den Rechner ab,
  weil der Archivar alles lädt und die Membran alles statt nur der Presence.
- **Blockade:** Membran-/Archivar-Bug (siehe Linie-Punkt).
- **Braucht:** den Archivar-Ladepfad auf Presence-only bauen, dann LOCK aufheben.
- **Wort:** Harte Läufe LOCK | 2026-09-26 | Operator-Wort folge36.

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) die präregistrierte JUICE-Kette wartet auf
  das Perigäum; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene Abschluss-Check läuft
dann mit Commit und Push. `/consent` ist der session-weite Consent (Delegation), nie das
Commit-Wort.
