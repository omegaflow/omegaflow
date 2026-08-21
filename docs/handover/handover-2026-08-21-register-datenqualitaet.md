<!--
  title: Register-Datenqualität — geojson-Gate-Mismatch + Re-Verifikations-Lücke
  class: handover
  date: 2026-08-21
  sha256: 42e580a629b61abeaa01732a1edde1723611e684a0d239b7ab34f4c4b3482c48
  status: live
  see-also: docs/SOURCE_PORT.md TODO.md docs/handover/handover-2026-08-21-offene-atome.md
-->
# Handover: Register-Datenqualität — geojson-Gate-Mismatch + Re-Verifikations-Lücke

Registriert 2026-08-21. Die nächste Session liest genau dieses eine
Dokument und beginnt. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst
auf das Wort des Operators. Alle Befunde sind am 2026-08-21 gemessen.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit benennen, nichts übernehmen
cargo check                      # muss 0/0 sein (beide Features)
grep -n "GeojsonEvents" src/archivar.rs        # der Gate-Mismatch
grep -n "^geojson" phi/sources.φ               # der eine Block
```

## Befund 1 — der tote geojson-Block (Gate-Mismatch)

Der USGS-fdsnws-Block (`phi/sources.φ:45–49`,
`geojson mag 0.0 seismic_magnitude_mw seismic_depth_km 6.0 0.0 0.0`)
ist strukturell tot. Zwei Verifikations-Pfade widersprechen sich:

- **Die Port-Verifikation** (`extract()`, genutzt von
  `test_live_sources_extract`) versteht `Extract::GeojsonEvents`.
- **Das Laufzeit-Gate** (`dispatch_reach`, archivar.rs:10927) sammelt
  Feldzeilen über `extract_fields()` (archivar.rs:978) — die kennt
  `GeojsonEvents` **nicht** (fällt in `_ => &[]`). Null Feldzeilen →
  `dispatch_reach` → None → stderr: „source N carries no field lines —
  refused, retry in ttl/Φ" — bei **jedem** Tick, der Block fetched nie.

Beweiskette: nur dieser eine Register-Block nutzt `geojson`; alle
übrigen im Register vorkommenden Extract-Typen sind in `extract_fields`
abgedeckt. Die Refusal-Zeile nennt nur einen Index, keine URL — die
Empfänger-Session bestätigt den Befund per grep (die Zeile erscheint
bei jedem Hidden-Lauf).

## Befund 2 — die void-Klassen des Live-Tests

`test_live_sources_extract` (archivar.rs:8673) maß am 2026-08-21:
**65 ok, 26 void** (91 getestet, Limit 600). Die 26 zerfallen in vier
Klassen — nur eine ist echter API-Verfall:

1. **Eingefrorene Fixture-Daten:** der Test ersetzt Template-Marker mit
   fixen Werten (`{today}` → 2026-08-07 etc.). OMNI endet am 08-06
   (stopDate), `{hour_ago}`-Fenster und die DONKI-Flare-Woche laufen
   ins Leere — falsche voids.
2. **Key-Marker:** `{NASA_API_KEY}`, `{LASAIR_TOKEN}` … werden mit void
   substituiert — key-gated Quellen liefern lokal nie. Schlüsselfrage,
   kein Datenqualitäts-Befund.
3. **API-Drift nach dem Port:** INGV-fdsnws liefert HTML statt GeoJSON,
   PSP/SOLO-HAPI (cdaweb) liefern HTML, TAPVizieR-Spalten verschoben —
   die Server haben sich nach dem Eintrag geändert.
4. **Ruhige Fenster:** Beben-Feeds mit Fixture-Standort (Golf von
   Guinea) und minmagnitude — leer ist die physikalische Wahrheit.

## Befund 3 — die Pipeline-Lücke

- **Keine Re-Verifikation von `phi/sources.φ`:** Rechecks laufen nur
  über blocked/dead (recheck_b*.φ) — die lebenden Quellen misst
  niemand nach. APIs driften unbemerkt.
- **Der Test folgert nichts:** nicht im CI-Gate, und seine void-Liste
  wird nicht ins Register zurückgetragen.
- **Laufzeit-Refusals versickern:** die stderr-Zeilen sammelt kein
  Ledger; es existiert kein Weg von „source refused" zurück zur
  Kuration.

## Auftrag

A. **Der geojson-Block — eine Wahrheit wiederherstellen.** Zwei Wege
   stehen offen, die Session entscheidet nach Live-Probe (die
   fdsnws-URL live fetchen, Struktur gegen die Block-Grammatik halten):
   entweder `extract_fields` lernt `GeojsonEvents` (Force-Felder aus
   dem Extract ableiten), oder der Block wird ersetzt/entfernt
   (benannt, mit Register-Zeile). Ergebnis-Gate: Hidden-Lauf ohne die
   „no field lines"-Zeile.

B. **Der Live-Test als Instrument.** Die Fixture-Daten durch die
   echte Systemzeit ersetzen (oder ein Lauf-Flag), die void-Liste nach
   Klassen etikettieren (key-void / Drift-void / ruhig-void / kaputt)
   und in die Kurations-Queue zurücktragen — analog den blocked/dead-
   Rechecks, aber für die lebenden Quellen. healthcheck.yml läuft
   `--verify` (3-h-Cron); der Extrakt-Test gehört dort oder in ein
   eigenes Sweep-Kommando eingehängt.

C. **Das Refusal-Ledger.** Die Laufzeit-Refusals (dispatch_reach,
   extract-void) in ein Register einsammeln (stderr-Zeile + Liste in
   TODO/ledger.φ), damit nichts mehr versickert.

## Constraints

- **std-only** in Rust (std + curl). Keine Docstrings, keine
  Kommentare; `cargo check` 0/0 (beide Features); Warnungen sind
  Codrot.
- 0-Kanon: Ausfall = Sample-Skip (fehlt), nie 0.0-Fabrikation.
- Hidden-Lauf als Verifikation (`OMEGAFLOW_HIDDEN=1 cargo run`, die
  `φ window:`-Zeile lesen); kein Test öffnet ein Fenster oder strahlt.
- Register-Updates (TODO.md / ledger.φ) im selben Commit wie der Code.

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik, `src/te.rs`, der `wm2_1au`-
Luminositäts-Pfad, die OMEGAFLOW_HIDDEN-Radiator-Stille, die
NCEI-SSI-Ernte und der Asteroiden-Langbogen (eigene Handover), der
GPU-TE-Anschluss (eigenes Handover: solar-te-gpu-anschluss.md). Nur:
Gate-Konsistenz + Re-Verifikation.

## Gates & Abschluss

- cargo check 0/0; Hidden-Lauf ohne Refusal-Zeile für den
  geojson-Block (oder Block benannt entfernt).
- Der Live-Test etikettiert jede void-Quelle mit Klasse; die
  Kurations-Queue trägt die Befunde.
- Ein Commit je Einheit; Register-Update im selben Commit. Das letzte
  schließt die TODO-Zeile und archiviert dieses Handover nach
  `/home/johannes/projects/archive/handover/` (AGENTS-Regel).
