<!--
  title: Handover — Mountain-Folge 158 (2026-09-25)
  session: Mountain-Folge 158
  class: handover
  date: 2026-09-25
  sha256: 63cd4aba348effc1eaf902dc305720ee97ead850d9e9ed9adecceeecd5adf444
  status: live
-->
# Handover — Mountain-Folge 158 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte (Stehender-Pass-Ergebnis, geschlossen-Register,
Benchmark, Geteilter Baum): sie leben in git. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit:
`autonom` → `operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.

Diese Session konsumierte `docs/handover/archiv/handover-2026-09-25-mountain-folge157.md`
(zum Session-Beginn noch `docs/handover/`).

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### gap unit-auto-detect — der SI-Unit-Arm für 167 Einträge
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphans`) 167 `parser-def`-Einträge in `phi/blocked_sources.φ` tragen die Klasse `unit-auto-detect`; die Feld-Einheiten sind `unit absent` bzw. `unit/cadence absent`, der `convert_to_si`-Arm fehlt.
- **Blockade:** keine.
- **Braucht:** `phi/blocked_sources.φ::gap:unit-auto-detect ×167` — den Unit-Auto-Detect-Arm auf der Archivar-Vorlage bauen, dann `register_lookup --orphans` als Nulllinie messen.

### gap force-undetermined — der Kraft-Zuordnungs-Arm für 16 Einträge
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphans`) 16 `parser-def`-Einträge tragen die Klasse `force-undetermined`; `force_type` der Felder ist unbestimmt.
- **Blockade:** keine.
- **Braucht:** `phi/blocked_sources.φ::gap:force-undetermined ×16` — die Kraft-Zuordnung der Felder bauen.

### gap konverter — der Konverter-Arm für 4 Einträge
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphans`) 4 Gaia-DR3-Einträge tragen die Klasse `konverter`; `mag_g` erhält die Einheit `nT` (Konverter-Fehler, mag erwartet).
- **Blockade:** keine.
- **Braucht:** `phi/blocked_sources.φ::gap:konverter ×4` — den Einheiten-Konverter-Arm korrigieren.

### gap votable-reader — der VOTable-Reader für 1 Eintrag
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphans`) 1 Eintrag (ALMA TAP, votable-only) trägt die Klasse `votable-reader`; der VOTable-Reader-Arm fehlt.
- **Blockade:** keine.
- **Braucht:** `phi/blocked_sources.φ::gap:votable-reader ×1` — den VOTable-Reader bauen.

### gap html-parser-arm — der HTML-Parser-Arm für 1 Eintrag
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `register_lookup --orphans`) 1 Eintrag (AEC-FDSN Alaska, Drupal-HTML) trägt die Klasse `html-parser-arm`; kein JSON-API, der HTML-Arm fehlt.
- **Blockade:** keine.
- **Braucht:** `phi/blocked_sources.φ::gap:html-parser-arm ×1` — den HTML-Parser-Arm bauen.

#### Stufe 3 — blockiert

keiner. / Stufe 4 — wartend: keiner. / Stufe 5 — termin: keiner. / Stufe 6 — LOCK: keiner.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Diese Session trägt:
`phi/blocked_sources.φ` (189 `gap`-Direktiven, je Block der fehlende Arm), diese
Übergabe + der Move der konsumierten folge157 ins `archiv/`. Der lokale Zustand-Ledger
`docs/zustand/external-state.md` (gitignored) ist auf tools-latest-Manifest
frisch @HEAD nachgezogen.
