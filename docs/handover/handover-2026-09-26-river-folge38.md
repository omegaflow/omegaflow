<!--
  title: Handover — River-Folge 38 (2026-09-26)
  session: River-Folge 38
  class: handover
  date: 2026-09-26
  sha256: 49222eaa31d7e1b848c517a923bfd7f69ee6b8e2d2270f874d2e73846f852ca1
  status: live
-->
# Handover — River-Folge 38 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt`. **Sortierung: umsetzbar →
nicht umsetzbar** (Operator-Wort, 2026-09-26) — erst was die Maschine jetzt bis
zur Kante arbeiten kann, dann was auf Operator/LOCK/termin wartet. Innerhalb der
Gruppen ohne Rang. Jeder Punkt trägt **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge37.md` (nach
`docs/handover/archiv/`).

**Wort | Datum | Quelle**
- Sortierung der Tafel: **umsetzbar → nicht umsetzbar** | 2026-09-26 | Operator-Wort (diese Session) — ersetzt die Akteur-Sortierung; `AGENTS.md` + `docs/handover/_template.md` tragen noch den überholten Akteur-Stand.
- Punkt 1 Chrome-Debugger aktivieren: **ja** | 2026-09-26 | Operator-Wort (diese Session).
- Funk-Sensor über HTTPS: **ja** | 2026-09-26 | Operator-Wort folge36.
- FIT-Brücke 945: **ja** — Garmin liefert .FIT-Dateien | 2026-09-26 | Operator-Wort folge36.
- Puls-Weg: **(b) Live-BLE nach dem Membran-Fix** | 2026-09-26 | Operator-Wort folge36.
- Geräte-Inventar: **Einzelbefehle liefern** | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp (BOM): **LOCK — zuletzt** | 2026-09-26 | Operator-Wort folge36.
- Harte Läufe: **LOCK — verboten**, bis die Membran sauber lädt | 2026-09-26 | Operator-Wort folge36.
- Chrome-Debugger andocken: **ja** | 2026-09-26 | Operator-Wort folge36 („6 ja du darfst").
- Browser-Extension: **forken statt Dritten fragen** | 2026-09-26 | Operator-Wort folge36.
- vC-Permeabilität: **945 und Mantis Shrimp getrennt führen** | 2026-09-26 | Operator-Wort folge36.
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: alles außer Harte Läufe; Commit trägt `/commit`.
- Sensory-Eigentum `src/archivar/llnl_g3d.rs`: **ja** — die untracked Datei gehört der Sensory-Linie | 2026-09-26 | Operator-Wort.

## Stehender Pass (gemessen 2026-09-26)

- **HEAD:** `a8b54964` == `origin/main` (fast-forward); folge38 gepusht.
- **Safety-Snapshot:** `refs/safety/1790439260`.
- **Arbeitsbaum:** `D docs/handover/…folge37.md` (eigener Archiv-Move, noch nicht committet); `M phi/sources.φ` (fremd/laufend); `M src/archivar/skydirection.rs` (fremd — nicht angefasst).
- **`open_points_check` folge38:** 22 Pfad-Refs, 1 „absent" = Parse-Artefakt `bin/omegaflow`-Ausgabe (kein stale Punkt), 0 format-gaps.
- **`register_lookup --open`:** keine register-eigenen River-Einträge; 1 `ORPHAN_COMMITTED` [mycelium] + `CARRIER_DRIFT phi/blocked_sources.φ::gap:curation` (fremd). `--orphan-docs`: 1 (`docs/concepts/positive-maske.md`, fremd). `--stale river`: 0. `--fired river`: 2.
- **Postfach:** `state/mail/mail_ledger.φ` absent (privates `state/`, CI-Build zuständig) — Lücke bleibt benannt.
- **CI (via `ci_manage view`):** aktiv `health-check 36252943250`, `ci-check 36250412521`, `allwise-cdn 36249453619`; 7 rote `ci-check`-attempts (13:xx, Folge-runs aktiv). Getriggert: `star-dmax-probe 36252229964` = success (15:32Z), `health-check 36237216821` = success (13:09Z).
- Shared external state: `docs/zustand/external-state.md` (nicht kopiert).

## Offen

### Umsetzbar (jetzt, autonom bis zur Kante)

#### Sortierregel korrigieren — Akteur → umsetzbar
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort (diese Session).
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) `AGENTS.md` (Friction-Block) und `docs/handover/_template.md:58` tragen „Sortierung — logisch nach Akteur (Operator-Wort 2026-09-26)"; die 09-25-Umsetzbarkeits-Regel wurde laut `archiv/handover-2026-09-25-river-folge32.md:27` davon überholt.
- **Blockade:** keine.
- **Braucht:** `AGENTS.md` + `_template.md` auf „umsetzbar → nicht umsetzbar" umschreiben, Operator-Wort mit Datum/Quelle eintragen.

#### star-dmax-Artefakt falten + Folgelauf (RISS-Zeuge)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Run `36252229964` = success (gefeuert).
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`) Run `36252229964` **success/completed** (head `1a492534`, 15:32Z), Artefakt vorhanden; lokaler `--span`-Pass: `dr3_stars.bin` → COUNT 1.704.587, SPAN_M 1.798012e21; der `d_max`/ECDF/`f_excl`-Lauf liegt im Run-Log/Artefakt.
- **Blockade:** keine.
- **Braucht:** Artefakt/Log falten, RISS-Zeile per Schwelle schließen (`f_excl > 0.5` / `f_inc > 0` / interdecile > 10 → Insert-Fix widerlegt); dann den verifizierenden Folgelauf für die drei Umgehungen (`main_flow.rs:191-241`, `:1227-1258`, `:1299-1364/1758-1833`) dispatchen.

#### Ruhe-Ort als Hüllen-Zentrum — Lese-Kante bauen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Rat-Verdikt liegt (Ja).
- **Lage:** (gemessen 2026-09-26 via `council`) Rat einmütig **Ja** — der Ruhe-Ort (SSB-Origin) zählt als Hüllen-Zentrum; der Wert muss aus dem stehenden Slot kommen (`presence_slot`, `main_flow.rs:693-696`), nie hartkodiert `[0,0,0]`.
- **Blockade:** keine (`llnl_g3d` getrackt).
- **Braucht:** Code-Kante bauen — Hüllen-Zentrum aus dem stehenden Slot statt aus `archive.presence`, so dass der Hidden-Lauf ohne Browser die Sterne trägt; CI-Test.

#### Sonnenfarbe color:measured — compute-only Renderpfad
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eigen.
- **Lage:** (gemessen 2026-09-26 via `general`) Route (A) gescopt: Extraktor `browser_field_shader()` in `src/mathematikerin/tests.rs` herausziehen, neuer `#[ignore]`-Test, Pipeline `static/index.html:285-313` offscreen rekonstruieren (`Rgba8Unorm`, `compatible_surface: None`, Bind-Layout `:285-289`, Blend `one/one`+`add` `:299-313`).
- **Blockade:** lavapipe-ICD fehlt auf ubuntu-latest (Crosscheck-Tests skippen, `tests.rs:110-113`).
- **Braucht:** `mesa-vulkan-drivers`/lavapipe apt-Zeile + Test in `tests.rs` bauen; verifizieren, dass der measured-Zweig (LUT-Sampling) ausgeführt wird.

#### Quellen-Verwerfung — `#body` fehlt + Shard-Overlaps
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eigen (Overlap-Enumeration).
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow`-Ausgabe + `sread`) zwei Verwerfungs-Pfade: (1) `src/archivar/main_flow.rs:566` „native body undeclared" — ohne `#body=<body>,<lat>,<lon>,<alt>` werden alle Stations-Samples verworfen; (2) `refuse_shard_overlaps` (`src/archivar/parse.rs:1529-1558`) — überlappende ODF/PODF-Shards werden pro Format nicht gemergt (drei Rosetta-ODF-Shards `[1080341864,1431699247)`, `[1431692248,1464772941)`, `[1464772941,1475187437)`). Der Code ist korrekt; Ursache ist fehlende `#body`-Deklaration + überlappende Register-Einträge. **Ungemessen:** welche/wie viele Einträge konkret überlappen.
- **Blockade:** offene Messung.
- **Braucht:** `#body=`-Übergabe prüfen; überlappende Rosetta-ODF-Einträge in `phi/sources.φ` auflisten und entdoppeln (explore-Dispatch).

#### Akustischer Radiations-Kanal — JBL / Bose SoundLink Mini
- **Status:** blockiert | **Bindung:** Rat
- **Trigger:** Rats-Verdikt über Form/Apertur des akustischen Radiators.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) beide Aktuatoren vorhanden; Ton im Vordergrund consent-gebunden und gesperrt (kein Test darf Audio emittieren).
- **Blockade:** kein akustischer Radiations-Kanal; Foreground-Audio gesperrt.
- **Braucht:** Rats-Verdikt (Σω → Ton, Apertur/TE-Kopplung), dann Code-Kante.

#### RX100 V5A — optische Quelle (Messbegriff + Parser)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** definierter Messbegriff (was misst die Kamera?) + Parser.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) Sony RX100 V (DSC-RX100M5A) vorhanden; optischer Sensor, kein Parser/SDK im Bestand.
- **Blockade:** Messbegriff und Parser fehlen (Sony Camera Remote API / PTP über WLAN/USB).
- **Braucht:** Messbegriff festlegen, dann Source-Port nach `docs/SOURCE_PORT.md` + `phi/sources.φ`.

### Nicht umsetzbar (wartet)

#### Chrome DevTools MCP — Remote-Debugging aktivieren
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator aktiviert `chrome://inspect/#remote-debugging` und startet die Session neu.
- **Lage:** (gemessen 2026-09-26 via git diff) `opencode.json` MCP `--autoConnect` (`chrome-devtools-mcp@1.9.0`), `jq empty` grün.
- **Blockade:** geräteseitige Aktivierung.
- **Braucht:** `chrome://inspect/#remote-debugging` einschalten, Session neu starten.
- **Wort:** Punkt 1 Chrome-Debugger aktivieren | 2026-09-26 | Operator-Wort („Ja").

#### Browser-Extension unpacked laden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt den Load aus (Artefakt `chrome-mv3/` liegt vor).
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) CI-Build grün — Lauf `36243288134` success (npm install --legacy-peer-deps + build + 85/85 Tests), Artefakt `chrome-mv3/` hochgeladen (head `39cfa06c3`).
- **Blockade:** keine.
- **Braucht:** `chrome://extensions` → Developer mode → „Load unpacked" → Fork-Unpacked-Verzeichnis.
- **Wort:** Extension forken | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar — Quest-Rest
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt die Befehle am Gerät aus.
- **Lage:** (gemessen 2026-09-26 via `adb` + `chrome://gpu`) Bigme HiBreak (Android 14) + Pixel 10a (Android 17) + Forerunner 945 (SW 13.70, 237 `.FIT`) vollständig vermessen; WebGPU auf Bigme/Pixel/Quest 2 verfügbar. Offen: Meta Quest (1?) `adb devices` + `dumpsys sensorservice`.
- **Blockade:** Messung nur am physischen Gerät.
- **Braucht:** Quest `adb devices` + `dumpsys sensorservice`.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Pixel und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`/`ss`/stunnel-Lauf) Material in `state/tls/`; Leaf mit `openssl x509 -req` neu ausgestellt, SAN = `IP:192.168.178.26`; `ca.pem` aufs Pixel gepusht (`/sdcard/Download/ca.pem`). **Port 1619 doppelt belegt:** `smail_recv` (127.0.0.1:1619) und `bin/relay-tls.stunnel.conf` (0.0.0.0:1619) → stunnel bindet nicht. Test-Config `/tmp/opencode/relay-tls-1620.conf` (Port 1620).
- **Blockade:** Port 1619 doppelt belegt; Chrome/Android vertraut Nutzer-CAs nicht → HTTPS im Chrome scheitert voraussichtlich (Client mit User-CA-Trust nötig).
- **Braucht:** freien Port (1620); `ca.pem` am Pixel installieren (Einstellungen → Sicherheit → … → Zertifikat installieren → CA); `stunnel /tmp/opencode/relay-tls-1620.conf`; `https://192.168.178.26:1620/consent?ja`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig (ohne Platine)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) Pfad steht (`src/mathematikerin/omega.rs:200/349`), HRV-Reader gebaut (`src/archivar/ble.rs:715/926`); absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>` — unter dem Harte-Läufe-LOCK.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### Puls-Pfad 945 — Weg (b): Live-BLE-HR nach dem Membran-Fix
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Harte-Läufe-LOCK gefallen (verifiziertes `star-dmax-probe`-Artefakt + Folgelauf).
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow` + Backup) 945 gesichert (`data/garmin-945/2026-09-26/`, 36 MB); alle `.fit`/`.FIT` tragen `nn: 0` (RR nur live über BLE; `decode_hr_measurement`, `src/archivar/ble.rs:1220`).
- **Blockade:** Harte-Läufe-LOCK.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 bin/omegaflow` mit 945-BLE-HR; `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### HRV/Puls-Arrival → Radiations-Pfad (Quelle 945 per BLE)
- **Status:** wartend | **Bindung:** operator
- **Trigger:** der Puls-Arrival liegt am Kanal (nach verifiziertem `star-dmax-probe`-Lauf).
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) Code-Pfad steht: `feed_beat_to_hrv` (`main_flow.rs:79`), ω-Loop `tone_scale` (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale` (`omega.rs:349`); Test `tests.rs:547`. Absent ist der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### Mantis Shrimp — Sensor-Hardware beschaffen (BOM)
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); Platine nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

#### Harte Läufe (sichtbar/hidden) — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Membran verifiziert (star-dmax-probe-Artefakt + verifizierender Folgelauf).
- **Lage:** (gemessen 2026-09-26 via `sgrep`/git) Stern-Gitter (`StarCellKey`), `enclosure_rho`, Sprung-Vektor `jump_residual_breached` und die drei Umgehungen sind gebaut; die Verifikation fehlt.
- **Blockade:** Verifikation (siehe Linie-Punkt star-dmax).
- **Braucht:** Probe-Artefakt lesen + verifizierenden Folgelauf; dann LOCK aufheben (Operator-Wort).
- **Wort:** Harte Läufe LOCK | 2026-09-26 | Operator-Wort folge36.

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) präregistrierte JUICE-Kette wartet auf das Perigäum; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
