<!--
  title: Handover — River-Folge 41 (2026-09-27)
  session: River-Folge 41
  class: handover
  date: 2026-09-27
  sha256: 39dd4e4f3a456742b9bc5fd377a4ccc9a22b008c8a2e29fa92c2f07888c0c6b5
  status: live
-->
# Handover — River-Folge 41 (2026-09-27)

Dieses Register trägt nur Offenes. Nur eigene Arbeit: pfad-begrenzter Commit, fremde
uncommittete Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD
ist. Sortierung umsetzbar → nicht umsetzbar; kein Rang. Jeder Punkt trägt **Trigger** /
**Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-27-river-folge40.md` (nach `archiv/`).

## Wort-Register
- **gic-Paper-Einreichung: höchste Priorität** | 2026-09-27 | Operator-Wort.
- RX100-K-Beschaffung: **kein Kauf vor Förderung** | 2026-09-27 | Operator-Wort (Träger `mountain-folge176.md:32`).
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.

## Verweise (Prosa mit offenen Markern)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, gebaute Atome, offene Messpunkte (Z. 346-352).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).

## Offen

### gic-Paper — Einreichung (HÖCHSTE PRIORITÄT)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort „gic einreichen".
- **Lage:** (gemessen 2026-09-27 via `export_latex --check`) Papier reif: Gate grün (title 58 / abstract 199 / nums 712 / sha `45a20a09…`); interne Systemsprache entfernt. Vorbereitung an der Kante: CJSS 2022 (`10.11728/cjss2022.03.210406045`) **Volltext** (Sektionen 0–4, alle Tabellen) über `POST /article/htmlContent` gelesen und **PDF** (`/article/exportPdf`, 1 757 420 B) beschafft; Novelty-Zeile bestätigt — nicht vorweggenommen, in der Richtung gestützt (E/Bz @ 60 min, Quell-Shuffle-Null ohne family-wise) — und als Zitat aufgenommen (Related Work + Referenz). Paper-Bestand `docs/paper/yu-tong-fang-hu-2022-transfer-entropy-solar-wind-drivers.md`; PDF `data/cjss.ac.cn/210406045.pdf`; Paket `docs/auftrag/auftrag-gic-einreichung.md` aktualisiert. Venue #1 = **Space Weather** (AGU), Cover Letter + Prior-Art + APC im Auftrag. Autorenblock privat `state/paper/gic-autoren-2026-09-27.md`.
- **Blockade:** Send = Operator-Hand.
- **Braucht:** `docs/auftrag/auftrag-gic-einreichung.md`; dann GEMS-Einreichung + ESSOAr (`agupubs.onlinelibrary.wiley.com/journal/15427390`).

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

### vC-Permeabilität — 945-Zweig
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) Pfad `omega.rs:200/349`, Probe `perm_target_probe` gebaut; absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>`.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

### 945 — RR-Arrival
- **Status:** blockiert | **Bindung:** operator
- **Trigger:** Brustgurt liefert RR.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) HR/RSC gebaut (`ble.rs:1267` `decode_rsc_measurement`, `:31` RSC-Char, `:1871` HR-Service); die 945 liefert am Handgelenk HR bpm, kein RR (sensory `d766ed9a5`).
- **Blockade:** RR fehlt am Handgelenk (Hardware).
- **Braucht:** Brustgurt (sensory „RR-Kanal").

### Geräte-Cluster — offene Messpunkte
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Gerät zugänglich; Operator-Wort (Zugriff).
- **Lage:** (gemessen 2026-09-27 via `sread` `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md:346-352`) Inventare vollständig abgelesen (945/Quest/Pixel/HiBreak/XPS); laut Survey offen: Chip-/FCC-Identitäten (945/Quest), 945-Abtastraten, WebGPU/Generic-Sensor am Quest-Gerät, Bigme-Beschleunigung/Näherung/Licht/Haptik/Browser/WebGPU, Pixel-`SensorManager`-Liste + Thread-Zuordnung, Pixel-UWB (Riss).
- **Blockade:** nur am physischen Gerät.
- **Braucht:** `dumpsys sensorservice` (Pixel/Bigme/Quest); E-Label/FCC.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) präregistrierte JUICE-Kette; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

### Mantis Shrimp — BOM
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

### paper-check — yu-tong-Label >75
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36283676047`) der CI-Job `paper-check`
  ist rot: `docs/paper/yu-tong-fang-hu-2022-transfer-entropy-solar-wind-drivers.md` trägt
  **title=94 > 75** (`export_latex --check`, `MAX_TITLE = 75`,
  `tools/science/src/bin/export_latex.rs:9`); der Body-sha stimmt (`07395a7d…`). Die
  terminologie-sha-Abweichung (Sensory) ist in Sensory-Folge 183 behoben.
- **Blockade:** keine.
- **Braucht:** H1 (Zeile 10) **und** Header-`title:` auf ≤75 kürzen (z. B. „Yu, Tong, Fang
  & Hu 2022 — TE ranking of solar wind drivers"), danach
  `omega_sh sha docs/paper/yu-tong-fang-hu-2022-transfer-entropy-solar-wind-drivers.md` in
  den Header (die H1-Kürzung ändert den Body-sha).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent, nie das Commit-Wort.
