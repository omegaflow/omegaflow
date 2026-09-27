<!--
  title: Handover — River-Folge 39 (2026-09-27)
  session: River-Folge 39
  class: handover
  date: 2026-09-27
  sha256: 2d880bd710e214d363c310bfa4aeddb2cac5ec4479d596864bfab12a63523557
  status: live
-->
# Handover — River-Folge 39 (2026-09-27)

Dieses Register trägt nur Offenes. Nur eigene Arbeit: pfad-begrenzter Commit, fremde
uncommittete Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD
ist. Sortierung umsetzbar → nicht umsetzbar; kein Rang. Jeder Punkt trägt **Trigger** /
**Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge38.md` (nach
`docs/handover/archiv/`).

## Wort-Register
- RX100-K-Beschaffung: **kein Kauf vor Förderung** | 2026-09-27 | Operator-Wort (Träger `mountain-folge176.md:32`).
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.

## Verweise (Prosa mit offenen Markern)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, gebaute Atome, offene Messpunkte (Z. 346-352).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.

## Stehender Pass (gemessen 2026-09-27)
- **HEAD:** `66a66b9a1` == `origin/main`; Arbeitsbaum nur Fremdarbeit (mountain/sensory/mycelium).
- **Browser:** CDP-Brücke verbunden (`browser_targets` = 2 Chrome-Executors; `chrome-devtools` antwortet).
- **Benchmark (Star-Epoch-Fix):** `grind-flash` lieferte **1991.25** (falsch — las die Quell-Kataloge I/239/I/259); `grind-pro` korrigierte auf **2000.0** (richtig — `tycho2_compiler.rs:199-200` liest `tyc2.dat` J2000.0 roh, Supplement/tyc1/tgas werden nach J2000 propagiert). **Sieger: pro.** Register `catalog_epoch 2000.0` (`phi/sources.φ:9343`); Probe-Bins lesen ihn, `membrane-hull-probe` kompiliert wieder.
- Shared external state: `state/zustand/external-state.md` (nicht kopiert).

## Offen

### Geräte-Cluster — offene Messpunkte
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Gerät zugänglich; Operator-Wort (Zugriff).
- **Lage:** (gemessen 2026-09-27 via `sread` `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md:346-352`) Inventare vollständig abgelesen (945/Quest/Pixel/HiBreak/XPS); laut Survey offen: Chip-/FCC-Identitäten (945/Quest), 945-Abtastraten, WebGPU/Generic-Sensor am Quest-Gerät, Bigme-Beschleunigung/Näherung/Licht/Haptik/Browser/WebGPU, Pixel-`SensorManager`-Liste + Thread-Zuordnung, Pixel-UWB (Riss).
- **Blockade:** nur am physischen Gerät.
- **Braucht:** `dumpsys sensorservice` (Pixel/Bigme/Quest); E-Label/FCC.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

### Akustik-Sink — sichtbarer Lauf
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den sichtbaren Lauf.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) Sink gebaut (`main_flow.rs:278` `acoustic_sink`, `:222` `AcousticFanout`, `:279` `OMEGAFLOW_ACOUSTIC`), Test-Modul `:5268`.
- **Blockade:** keiner — der Akt ist sichtbar (Radiator).
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto bin/omegaflow`.
- **Wort:** Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.

### 945 — RR-Arrival
- **Status:** blockiert | **Bindung:** operator
- **Trigger:** Brustgurt liefert RR.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) HR/RSC gebaut (`ble.rs:1267` `decode_rsc_measurement`, `:31` RSC-Char, `:1871` HR-Service); die 945 liefert am Handgelenk HR bpm, kein RR (sensory `d766ed9a5`).
- **Blockade:** RR fehlt am Handgelenk (Hardware).
- **Braucht:** Brustgurt (sensory „RR-Kanal").

### TLS im Relay — CA am Pixel
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` + startet `stunnel`.
- **Lage:** (gemessen 2026-09-27 via `sread`) Terminator-Config `bin/relay-tls.stunnel.conf`; Material `state/tls/`; Port 1619 doppelt belegt.
- **Blockade:** Port belegt; Chrome/Android vertrauen Nutzer-CAs nicht.
- **Braucht:** freier Port (1620); `ca.pem` am Pixel installieren; `stunnel bin/relay-tls.stunnel.conf`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

### vC-Permeabilität — 945-Zweig
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) Pfad `omega.rs:200/349`, Probe `perm_target_probe` gebaut; absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>`.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

### Mantis Shrimp — BOM
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) präregistrierte JUICE-Kette; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent, nie das Commit-Wort.
