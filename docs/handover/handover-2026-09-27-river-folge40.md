<!--
  title: Handover — River-Folge 40 (2026-09-27)
  session: River-Folge 40
  class: handover
  date: 2026-09-27
  sha256: 00498c6a218114a8e9713bbd7e381ecbcf2421ef6739de0c2a78095b1f706ddd
  status: live
-->
# Handover — River-Folge 40 (2026-09-27)

Dieses Register trägt nur Offenes. Nur eigene Arbeit: pfad-begrenzter Commit, fremde
uncommittete Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD
ist. Sortierung umsetzbar → nicht umsetzbar; kein Rang. Jeder Punkt trägt **Trigger** /
**Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-27-river-folge39.md` (nach `archiv/`).

## Wort-Register
- **gic-Paper-Einreichung: höchste Priorität** | 2026-09-27 | Operator-Wort (diese Session).
- RX100-K-Beschaffung: **kein Kauf vor Förderung** | 2026-09-27 | Operator-Wort (Träger `mountain-folge176.md:32`).
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.

## Verweise (Prosa mit offenen Markern)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, gebaute Atome, offene Messpunkte (Z. 346-352).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).

## Offen

### gic-Paper — Einreichung (HÖCHSTE PRIORITÄT)
- **Status:** operator-gebunden | **Bindung:** operator (Akt) / eigen (Reife)
- **Trigger:** Operator-Wort „gic einreichen".
- **Lage:** (gemessen 2026-09-27 via `export_latex --check`) Papier reif: Gate grün (title 58 / abstract 199 / nums 700 / sha `6e31b556…`); interne Systemsprache entfernt (`Bz-Blatt`, `the machine`, `0 honored`, `VerdictWord::Riss`). Bau-Auftrag aus `future-folge112` (7 Lücken) geschlossen — lag-sweep 0-6 h, n_surr = 100, KDE-Bandbreite, PCMCI, SOD. Venue #1 = **Space Weather** (AGU), Cover Letter + Prior-Art + APC im Auftrag. Titel entschieden (englisch, 58 Zeichen); Autorenblock privat `state/paper/gic-autoren-2026-09-27.md` (ORCID `0009-0007-5565-6348`). Offen: CJSS-2022-Volltext.
- **Blockade:** Send = Operator-Hand.
- **Braucht:** `docs/auftrag/auftrag-gic-einreichung.md`; dann GEMS-Einreichung + ESSOAr (`agupubs.onlinelibrary.wiley.com/journal/15427390`).

### Vorbereitung — gic-Papier (autonom, bis zur Kante)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (läuft im Atom).
- **Lage:** (gemessen 2026-09-27 via `omega_sh sha` + `export_latex --check`) Gate grün; Paket-Artefakt `docs/auftrag/auftrag-gic-einreichung.md` geschrieben; Autorenblock privat `state/paper/gic-autoren-2026-09-27.md`.
- **Blockade:** keine.
- **Braucht:** CJSS 2022 (`10.11728/cjss2022.03.210406045`) Volltext lesen, Novelty-Zeile bestätigen.

### Akustik-Sink — sichtbarer Lauf
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den sichtbaren Lauf — **gefeuert** (gemessen 2026-09-27 via `register_lookup --fired`).
- **Lage:** (gemessen 2026-09-27 via `sgrep`) Sink gebaut (`main_flow.rs:278` `acoustic_sink`, `:222` `AcousticFanout`, `:279` `OMEGAFLOW_ACOUSTIC`), Test-Modul `:5268`.
- **Blockade:** keiner — der Akt ist sichtbar (Radiator).
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto bin/omegaflow`; Lauf-Ergebnis messen.
- **Wort:** Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.

### TLS im Relay — CA am Pixel
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` + startet `stunnel`.
- **Lage:** (gemessen 2026-09-27 via `sread`) Terminator-Config `bin/relay-tls.stunnel.conf`; Material `state/tls/`; Port 1619 doppelt belegt.
- **Blockade:** Port belegt; Chrome/Android vertrauen Nutzer-CAs nicht.
- **Braucht:** freier Port (1620); `ca.pem` am Pixel installieren; `stunnel bin/relay-tls.stunnel.conf`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

### Geräte-Cluster — offene Messpunkte
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Gerät zugänglich; Operator-Wort (Zugriff).
- **Lage:** (gemessen 2026-09-27 via `sread` `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md:346-352`) Inventare vollständig abgelesen (945/Quest/Pixel/HiBreak/XPS); laut Survey offen: Chip-/FCC-Identitäten (945/Quest), 945-Abtastraten, WebGPU/Generic-Sensor am Quest-Gerät, Bigme-Beschleunigung/Näherung/Licht/Haptik/Browser/WebGPU, Pixel-`SensorManager`-Liste + Thread-Zuordnung, Pixel-UWB (Riss).
- **Blockade:** nur am physischen Gerät.
- **Braucht:** `dumpsys sensorservice` (Pixel/Bigme/Quest); E-Label/FCC.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

### 945 — RR-Arrival
- **Status:** blockiert | **Bindung:** operator
- **Trigger:** Brustgurt liefert RR.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) HR/RSC gebaut (`ble.rs:1267` `decode_rsc_measurement`, `:31` RSC-Char, `:1871` HR-Service); die 945 liefert am Handgelenk HR bpm, kein RR (sensory `d766ed9a5`).
- **Blockade:** RR fehlt am Handgelenk (Hardware).
- **Braucht:** Brustgurt (sensory „RR-Kanal").

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
