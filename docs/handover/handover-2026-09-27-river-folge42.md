<!--
  title: Handover — River-Folge 42 (2026-09-27)
  session: River-Folge 42
  class: handover
  date: 2026-09-27
  sha256: ab1b23dff5069a9a26254aadc86d3772ae17063f7b0c242c2216cd4bfa31dafd
  status: live
-->
# Handover — River-Folge 42 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt, was
gemacht wurde. Nur eigene Arbeit: pfad-begrenzter Commit, fremde uncommittete
Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD ist.
Sortierung umsetzbar → nicht umsetzbar; kein Rang. Jeder Punkt trägt **Trigger** /
**Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-27-river-folge41.md` (nach `archiv/`).

## Wort-Register
- gic-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort.
- RX100-K-Beschaffung: kein Kauf vor Förderung | 2026-09-27 | Operator-Wort (Träger `mountain-folge176.md:32`).
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.
- Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.
- HTTPS ja | 2026-09-26 | Operator-Wort folge36.
- vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.
- Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

## Verweise (Prosa mit offenen Markern)
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, gebaute Atome, offene Messpunkte (Z. 346-352).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).
- `docs/paper/gic-causal-driver.md` — gic-Papier (Träger des gic-Punkts; 2 `pending`-Platzhalter Repo-DOI/Software-URL).
- `docs/auftrag/auftrag-flyby2-kette.md` — Flyby-Path-2-Kette (Trigger/Fill-Run).

## Offen

### gic-Paper — Einreichung (Akt)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort „gic einreichen".
- **Lage:** (gemessen 2026-09-27 via `export_latex --check`) Papier reif (title 58 / abstract 199 / nums 717 / sha ok); Manuskript um Key Points, Plain Language Summary, Open Research, Conflict of Interest, Acknowledgements ergänzt; Auftrag `docs/auftrag/auftrag-gic-einreichung.md` trägt Cover-Letter mit QUELLEN, GEMS-14-Felderliste, APC ($3,240), ESSOAr-Schritte.
- **Blockade:** Send = Operator-Hand.
- **Braucht:** GEMS `https://spaceweather-submit.agu.org/` (Universal-Login → 14 Felder → Verify & Submit) + ESSOAr `https://essopenarchive.org/`.

### gic-Paper — Vorbereitungs-Rest
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Reviewers + Repo-Anlage).
- **Lage:** (gemessen 2026-09-27) Manuskript trägt 2 `pending`-Platzhalter (Data-Repo-DOI, Software-URL/-DOI); Suggested Reviewers fehlen.
- **Blockade:** Reviewers = Menschenurteil; Repo-DOI = Dritter (Deponierung).
- **Braucht:** Operator-Wort.

### Akustik-Sink — sichtbarer Lauf
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort „Lauf starten"; gefeuert 2026-09-27.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) Sink gebaut (`main_flow.rs:278` `acoustic_sink`, `:222` `AcousticFanout`, `:279` `OMEGAFLOW_ACOUSTIC`), Test-Modul `:5268`.
- **Blockade:** keiner — der Akt ist sichtbar (Radiator).
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto bin/omegaflow`; Lauf-Ergebnis messen.

### TLS im Relay — CA am Pixel
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator beendet den transienten stunnel, installiert `ca.pem` + startet stunnel.
- **Lage:** (gemessen 2026-09-27 via `lsof`) Port 1619 von `smail_recv` (pid 1207) gehalten; 1620 transient von `stunnel` (pid 8065, `/tmp`). Varianten-Config `bin/relay-tls-1620.stunnel.conf` gebaut; Material `state/tls/` (`ca.pem`, `relay-leaf.pem`, `relay-leaf.key`).
- **Blockade:** Port-Konflikt; Chrome/Android vertrauen Nutzer-CAs nicht.
- **Braucht:** `kill 8065`; `ca.pem` am Pixel installieren; `stunnel bin/relay-tls-1620.stunnel.conf`; `bin/omegaflow`.
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
- **Lage:** (gemessen 2026-09-27 via `sread` `survey-2026-09-23-geraete-anbindung-radiatoren.md:346-352`) Inventare gelesen; offen Chip-/FCC-Identitäten (945/Quest), 945-Abtastraten, WebGPU/Generic-Sensor am Quest, Bigme-Beschleunigung/Näherung/Licht/Haptik/Browser/WebGPU, Pixel-`SensorManager`-Liste + Thread-Zuordnung, Pixel-UWB (Riss).
- **Blockade:** nur am physischen Gerät.
- **Braucht:** `dumpsys sensorservice` (Pixel/Bigme/Quest); E-Label/FCC.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28 **11:45 UTC** (gemessen 2026-09-27 via JPL Horizons `-28` + ESA-Kanal).
- **Lage:** (gemessen 2026-09-27 via `cargo build`) Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen), Tube ±12 h aus gesiegeltem Arc; Kanäle RTSW/Kp GFZ/Swarm/OMNI2/ACE; Perigäum-Zelle 13, alle Zellen `pending` (ehrlicher Vor-Flyby-Zustand).
- **Blockade:** kein Workflow registriert — der Lauf muss zum Zeitpunkt erfolgen.
- **Braucht:** RTSW-Snapshots ~28.09. 12:00 + 29.09. 00:00 UTC (`curl -sSfL …/rtsw_mag_1m.json|rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`), dann `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

### Flyby-Path-2 — Trajektorie-Riss (CDN ≠ Siegel)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort „Bahn benannt" (Siegel-Klausel).
- **Lage:** (gemessen 2026-09-27 via `archive_search --sniff`) CDN `ephemeris_juice.bin` sha256 `eee376ef…` (538 696 B) ≠ Siegel `aeb3c82f…` (`docs/paper/flyby-path-2-preregistration.md:21`). Benennung im Addendum `:58-70` eingetragen (`eee376ef…`, Prädiktion nach Tracking bis 2026-09-16); das Bin trägt den Riss bei CDN-Ladung und baut keinen Tubus aus dem ungesiegelten Arc.
- **Blockade:** die Benennung ist Operator-Akt (Siegel-Klausel).
- **Braucht:** Operator-Wort „Bahn benannt"; dann lädt das Bin den CDN-Arc.

### Mantis Shrimp — BOM
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent, nie das Commit-Wort.
