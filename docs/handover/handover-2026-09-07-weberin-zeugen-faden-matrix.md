<!--
  title: Handover — Weberin: die eine Mess-Anordnung gebaut (Bau-Linie 1-9, Architektur, S²-Zeugen, Thread-Matrix, Faden-Lücken)
  class: handover
  date: 2026-09-07
  sha256: d6d721f94d47c8349cbb7fdc0951bd8ca79a1df297837ddb3183303af7fff3a9
  status: live
  see-also: docs/concepts/die-weberin.md docs/handover/handover-2026-09-06-weberin-archivar.md docs/handover/handover-2026-09-06-s2-scanner-nadel.md docs/surveys/survey-2026-09-07-weberin-thread-matrix.md docs/TODO.md
-->

# Handover — Weberin: der Bau (konsolidiert)

Konsolidierte Übergabe der Weberin-Bau-Arbeit (2026-09-06/07). Vorausgehende Anker für die Archivar-/S²-Grundlage sind die zwei 09-06-Handovers `handover-2026-09-06-weberin-archivar.md` (Schritt 1 Motion::Kepler baryzentrisch) und `handover-2026-09-06-s2-scanner-nadel.md` (S²-Richtungssinn + SkyDirection). Diese Übergabe deckt den Rest des Baus.

## 1. Die Weberin Bau-Linie (die-weberin §9, Schritte 1-9)

Gebaut (größtenteils via Sub-Agenten), committet:

- **Schritt 1 + 8 (Körper-Verdict + Riss)**: `src/weberin.rs` (Body-Verdict Placed/Absent/Riss mit Knoten [SPK, DASTCOM]; voller Körper-Satz = union(eph, BODY_NUMBER)); Probe `weberin_body_verdict.rs`.
- **Schritt 2 (Stations-Konvergenz)**: `station_convergence_probe.rs` — INTERMAGNET (Boden) gegen SWARM (Überflug), Verdict Placed/Absent/Riss auf fanout-Ringen.
- **Schritt 3 (Topozentrik)**: `topocentric_coupling_probe.rs` — Rømer-Lichtzeit vom Stationspunkt, Stations-Parallaxe zweier Sichtlinien, ICRS·TDB.
- **Schritt 4 (Dichte-Feld)**: `vlies_density_compiler.rs` (VLDE, nside 128) + `vlies_density_probe.rs`.
- **Schritt 5 (Survey-Footprints)**: gemessen-verweigert (keine anonyme Exposure-Maske; CDS-MOCServer = Katalog-Coverage, refused). dead/blocked registriert.
- **Schritt 6 (Skymap-Routen)**: `src/archivar/amon.rs`/`auger.rs` + Compiler; direction-only-Witness.
- **Schritt 8 (Riss-Knoten)**: `riss_knoten_probe.rs` (Aggregat-Ledger der Risse mit Knoten).
- **Schritt 9 (Geliehener Sinn)**: `tools/measure/src/borrowed_sense.rs` + `nadel_gate`-Integration (Fink-LSST als Gestalt-Zeuge, nie einziger Zeuge).
- **Schritt 7 (CDN)**: `upload_asset`-Verdrahtung + `*-cdn.yml`-Workflows.

## 2. Architektur (Rat 2026-09-06/07)

- **`weberin.rs` → `src/weberin.rs`**: dritter Peer-Rollen-Baustein neben `archivar/` + `mathematikerin/` (Rats-Beschluss: keine neue Kiste, `weberin` nie `weave`).
- **Measure-Lib → `tools/measure/src/weberin/`**: der Schuss (nadel_gate/borrowed_sense/deredden) aus der anonymen `measure`-Lib umgezogen; Aufrufer auf `omegaflow_measure::weberin::…`.
- **Rat zur S²-Frage (2026-09-07)**: direction-only-Ereigniskataloge = eigene S²-Ereignis-Fäden (nicht die Healpix-Karte als Primär); die Karte ist nur abgeleitete Projektion.

## 3. Teilchen-Provenienz-Prinzip (die-weberin §4, löst SOURCE_PORT §12.6)

„Das Teilchen ist Abstammung, nicht Kraft" — ein Detektor (CR/Neutrino) misst `em` (Schauer/Cherenkov); die Teilchen-Art ist `particle_root`-Provenienz, kein particle-Bit. Kein 10. Kraft-Medium.

## 4. S²-Ereignis-Fäden + Zeugen (committet 8adf234 → c4492f9)

- **`src/archivar/s2event.rs`** (S2E1): ra/dec, sigma, epoch_tdb, energy, signalness, far, particle_root — kein Distanz-Slot. `s2.rs` `S2Osc::from_event` (τ=0 ohne Epoche).
- **`src/archivar/skymap.rs`** (SKY1, kind 0-4 inkl. gravity).
- **Compiler**: `icecat_compiler` (IceCat-1 348), `antares_vo_compiler` (8754), `gw_skymap_compiler` (bayestar-NUNIQ), `tap_skymap_compiler`.
- **CDN-Workflows**: icecat/antares/gw-skymap.

## 5. Recherche-Tool + Recherchen

- **`tools/utils/src/bin/archive_search.rs`**: Ganz-`$HOME`-Suche, Relevanz (Treffer-Dichte), `--leads`-Mode (unkuratierte Heime, zieht kuratierte Hosts ab). 9 Tests.
- **Thread-Matrix-Survey** `docs/surveys/survey-2026-09-07-weberin-thread-matrix.md`: Kette (Body 73 / Station / Direction S²) × Schuss je Kraft-Medium; built/in-register/pending/not-published + Lücken.
- **Externe Recherchen** (Ergebnis-Files auf dem Schreibtisch): Neutrino-Routen (TA nicht-publiziert; IceCat-1 = Dataverse 7502710, 348), Thread-Lücken (9 Kategorien), `gfz.md`.

## 6. Faden-Lücken-Kaskade (9 Leads → Linse/Probe/Review)

| Lead | Disposition |
|---|---|
| 1 IGETS-Gravimeter | `blocked account` (Tabelle offen = position-only) |
| 2 BGR-Infraschall (4 DOIs) | `compiler-asset` pending (netCDF-Reader) |
| 3 Hydrophon NODD/GCS | `pending` (Bucket-Harvester) |
| 4 EarthScope/IRIS-FDSN-Station | `pending` (miniSEED-Reader + `#`-Header-parser-gap); GEOFON aus gfz.md |
| 5 GIC | `not-published` (kontinuierlich); Zenodo 10594301 = pending-Einzel-Asset |
| 6 WWLLN Thunder-Hour | `declined` (Aggregat); Ereignis `not-published` |
| 7 SuperDARN | Positionen declined; FITACF pending |
| 8 HF-radar | **harvest-ready** (erddap_harvester; root = RADIALS only) |
| 9 BGC-Argo | **harvest-ready** (erddap_harvester; Dataset `ArgoFloats-synthetic-BGC`) |

## 7. Commit-Stand (HEAD c4492f9) + Parallel-Session

Diese Arbeit ist committet + gepusht (bis c4492f9). Unangetastet von der parallelen Galileo/Nadel-Session: die vielen `galileo_*`-Dateien, `bin/seconds_matrix_watchdog.sh`, `tools/measure/src/bin/solar_seconds_matrix_probe.rs` — NICHT anfassen/committen.

## 8. Offen / nächste Schritte

- Register-Feldblöcke für **BGC-Argo + HF-radar** prüfen + eintragen (Drafts bei den Sub-Agent-Ergebnissen; BGC-Argo ~22 MB/Slice → enge Zeit-Bindung, „synthetic"-Label = Force-Gate-Review; HF-radar nur Radials).
- Neue Reader für Lead 2/3/4/7 (netCDF-Detektions, Bucket, miniSEED/FDSN, FITACF).
- GW-Skymap-Workflow ist per-Ereignis (ein Superevent), kein „allgemeiner" GW-Zeuge.
- `archive_search`-Ganz-Platten-Läufe brauchen >60 s — für Leads `--leads` oder gezielt `--root`.

Die externen Ergebnis-Files (`auftrag-extern-*-results*.md`, `gfz.md`) liegen auf dem Schreibtisch; Befunde im Register verarbeitet, aber nicht als Referenz ins Repo kopiert (falls gewünscht, nachziehen).
