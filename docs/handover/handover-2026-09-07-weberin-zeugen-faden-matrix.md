<!--
  title: Handover — Weberin-Zeugen (S²-Ereignis-Fäden), Thread-Matrix, Faden-Lücken-Kaskade
  class: handover
  date: 2026-09-07
  sha256: 9221085fbd1a96c2f4dbd25e6962b30d99885297457b0fd4eab4712724704600
  status: live
  see-also: docs/concepts/die-weberin.md docs/surveys/survey-2026-09-07-weberin-thread-matrix.md docs/auftrag/auftrag-extern-weberin-faden-luecken.md docs/TODO.md
-->

# Handover — Weberin-Zeugen, Thread-Matrix, Faden-Lücken

Diese Session hat die Weberin-Zeugen-Linie (GW-/Neutrino-/CR-Skymap-Zeugen, die-weberin §9 Stufe 6) gebaut, das Teilchen-Provenienz-Prinzip aufgelöst, die volle Thread-Matrix erstellt, das Recherche-Tool gebaut und die 9 Faden-Lücken durch die Source-Port-Kaskade geschoben. Stand 2026-09-07.

## Was gebaut + committet ist (main cd869b6)

- **S²-Ereignis-Faden-Entität** `src/archivar/s2event.rs` (S2E1): ra/dec, sigma, epoch_tdb, energy, signalness, far, particle_root — **kein Distanz-Slot**. `s2.rs` `S2Osc::from_event` (τ=0 ohne Epoche). Rats-Verdikt (2026-09-07): Fäden zuerst, Karte nur Projektion.
- **SKY1-Witness-Karte** `src/archivar/skymap.rs` (kind 0-4: generisch/neutrino/CR/gamma/**gravity**).
- **Harvest-Compiler**: `icecat_compiler` (IceCat-1, 348 Ereignisse → S2E1+SKY1), `antares_vo_compiler` (8754, vo.km3net.de POST), `gw_skymap_compiler` (bayestar-NUNIQ-FITS → gravity), `tap_skymap_compiler` (generisch), `vlies_density_compiler`, `amon/auger`.
- **CDN-Workflows**: `icecat/antares/gw-skymap/vlies-density/amon/auger`-cdn.yml.
- **Recherche-Tool** `tools/utils/src/bin/archive_search.rs`: Ganz-`$HOME`-Suche, Relevanz (Treffer-Dichte), `--leads`-Mode (nur unkuratierte Heime, zieht kuratierte Hosts aus sources/blocked/dead ab). 9 Tests.
- **Konzept/Register**: die-weberin §4 „Das Teilchen ist Abstammung, nicht Kraft" (löst SOURCE_PORT §12.6); Thread-Matrix-Survey `docs/surveys/survey-2026-09-07-weberin-thread-matrix.md`; Skymap-Zeugen-TODO-Zeile.

## Nicht publiziert (0 honored, kein Ersatz) — gemessen

TA (nur Papier-Figuren), KM3NeT (nur KM3-230213A), Super-K, JUNO, LHAASO (nur 90-Quellen-Katalog), GIC-kontinuierlich. KASCADE-Grande semi-offen (EULA).

## Faden-Lücken-Kaskade (9 Leads → Linse/Probe/Review, grind-pro 2026-09-07)

| Lead | Disposition |
|---|---|
| 1 IGETS-Gravimeter | `blocked account` (Tabelle offen = position-only; Zeitreihen brauchen Konto) |
| 2 BGR-Infraschall (4 DOIs) | `compiler-asset` pending (braucht netCDF-Detektionsreader) |
| 3 Hydrophon NODD/GCS | `pending` (braucht Bucket-Harvester) |
| 4 EarthScope/IRIS-FDSN-Station | `pending` (braucht miniSEED-Reader; `#`-Header = probe_csv-parser-gap); GEOFON `geofon.gfz-potsdam.de/waveform/` als Zusatz (aus gfz.md) |
| 5 GIC | `not-published` (kontinuierlich); Zenodo 10594301 = pending-Einzel-Asset |
| 6 WWLLN Thunder-Hour | `declined` (Aggregat-Klima-Gitter); Ereignis-Kanal `not-published` |
| 7 SuperDARN | Positionen declined; FITACF pending (braucht FITACF-Reader) |
| 8 HF-radar | **harvest-ready** (erddap_harvester; root ist nur RADIALS, kein Total — gemessen) |
| 9 BGC-Argo | **harvest-ready** (erddap_harvester; Dataset-ID `ArgoFloats-synthetic-BGC`) |

**Nächster Schritt für 8+9:** die von den Sub-Agenten entworfenen Register-Feldblöcke prüfen + in `phi/sources.φ` eintragen. WICHTIG (Sub-Agent-Messung): BGC-Argo-Slice `time>=max-2d` = ~22 MB → Block braucht enge Zeit-Bindung; der „synthetic"-Label ist ein Force-Gate-Review-Punkt. HF-radar-Root trägt NUR Radials (VELU/VELV = kartesische Zerlegung des einen Radial-Skalars VELO).

## Commit-Stand (HEAD 3bde17c, 2026-09-07)

Diese Session (Weberin-Zeugen 8adf234 → 3bde17c, interleaved mit der parallelen Galileo/Nadel-Session) ist **committet + gepusht**, inklusive:
- `phi/sources.φ` — **Merge-Konflikt-Marker gefixt** (Zeilen 3236-3237 `<<<<<<<`/`=======` entfernt; HEAD-Seite leer; beide Blöcke Mauna-Loa-CO₂ + Argo-DAC behalten; keine Marker, 410 url-Blöcke, keine Duplikate, Parser grün) — in 3bde17c.
- `docs/auftrag/auftrag-extern-weberin-faden-luecken.md` (der externe Rechercheauftrag, 9 Kategorien) — in 3bde17c.
- Dieses Handover — in 3bde17c.
- Die `archive_search`-Verbesserungen (d954bca → cd869b6) + Thread-Matrix-Survey (ab37dc2) + S²-Entität/Compiler (8adf234 → 6c13f85).

**Nicht von dieser Session (parallel Galileo/Nadel, unangetastet):** `bin/seconds_matrix_watchdog.sh`, `tools/measure/src/bin/solar_seconds_matrix_probe.rs` + die vielen `galileo_*`-WIP-Dateien.

## Offen / für die nächste Session

- Register-Feldblöcke für BGC-Argo + HF-radar prüfen + eintragen (drafts liegen bei den Sub-Agent-Ergebnissen).
- Neue Reader für Lead 2/3/4/7 (netCDF-Detektions, Bucket, miniSEED/FDSN, FITACF).
- Die externen Ergebnis-Files (`auftrag-extern-*-results*.md`, `gfz.md`) liegen auf dem Schreibtisch; Befunde im Register verarbeitet.
- GW-Skymap-CDN-Workflow ist per-Ereignis (ein Superevent); kein „allgemeiner" GW-Zeuge (Lokalisierungen sind per-Ereignis).

Die langen `archive_search`-Läufe über die ganze Platte brauchen >60 s — für Leads gezielt `--root` auf die unkuratierten Heime (oder `--leads`), nicht über alles.
