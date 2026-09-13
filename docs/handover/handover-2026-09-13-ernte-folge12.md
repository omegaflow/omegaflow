<!--
  title: Handover — Ernte-Folge 12 (Stand 2026-09-13)
  session: Ernte-Folge 12
  class: handover
  date: 2026-09-13
  sha256: b14998cb8fc2a663776527401c784183fea47e1b4233b96e1609e6c64bbc2b62
  status: live
-->
# Handover — Ernte-Folge 12 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — Wächter: der Redispatch 34739297039 (dispatch 2026-09-13T05:02Z)
  stand bei Sessionsende noch `queued`; das Release `igetsftp.gfz.de` ist weiter
  absent. Der Wächter misst das Asset im Release igetsftp.gfz.de.
- hinet.bin — Wächter: der hinet-cdn-Dispatch 34740497220 (2026-09-13T05:31Z,
  start=2025-01-15T00:00, span=30, station-count=10) steht an; das Release
  `hinetwww11.bosai.go.jp` ist weiter absent. Der Wächter misst das Asset im
  Release hinetwww11.bosai.go.jp.

## Ernte

- 3D-Geschwindigkeitsmodelle — anonymer Fetch gemessen (SOURCEPORT-Kaskade
  2026-09-13): LITHO1.0 `igppweb.ucsd.edu/~gabi/litho1/litho1.0.tar.gz`
  (12,4 MB) und `litho10.geotess.zip` (4,7 MB) live; S40RTS + GyPSuM via
  GitHub-Mirror `raw.githubusercontent.com/romaguir/sph_models/master/data/
  models/{S40RTS,GypSum_S}.sph` (je 427 kB) live; EarthScope-EMC-Netcdf-Archiv
  `data.earthscope.org/archive/seismology/products/emc/netcdf/` anonym 200
  (282 .nc; UUP07/S40RTS nicht im Listing); SubMachine → ORFEUS
  `orfeus-eu.org/submachine/` (Beta, kein Grid-Download gefunden). Befunde in
  `phi/pipeline/research/agent_output/{submachine,emc,tomography_models}_2026-09-13.φ`.
  Offen: die Oszillator-Gate-/Force-Gate-Klassifikation (Tomografie-Modell =
  Modell/Reanalyse vs. Messung) — pending.
- Hi-net — `HINET_PASS` ist gesynct (78 Keys aus `.secrets.local` →
  omegaflow/omegaflow, 2026-09-13); der CI-CDN-Versuch ist dispatcht (Run
  34740497220, oben). Der lokale Download scheitert serverseitig
  (36-s-Cut/Prep-Abbruch, gemessen 2026-09-12). Die Registrierung
  (sources.φ-Zeile, Format `hinet`, hinet-cdn.yml, Compiler) steht bereit.

## Weberin — offene Fäden (Ernte)

- Innenplaneten/Monde-Zweitlinie — INPOP/EPM-Kernel für Merkur…Mars + die
  Monde ernten (zweite unabhängige Abstammung; die Eisriesen tragen sie schon).
- WWLLN — offener Thunder-Hour-Host (Nachfolger des toten GHRC-ERDDAP), Format
  + Auflösung; Realtime-Roh ist `not-published` (Mitgliedschaft).
- NRS-Hydrophon — Position/Identität: NRS02–10/12/13-Koordinaten aus der
  Netz-Tabelle, Spektren-Anker (nur NRS01/11 tragen SHAPE).
- INPOP25c-Asteroidenmassen — gravity-Katalogroute (`die-weberin` §1).
- MPC-Orbits — unabhängige zweite Körper-Linie (`mpcobs_compiler` steht, Route
  live).
- Broker-/GW-Positionen — Lasair/ANTARES/Fink loci + bayestar-Sky-Maps:
  positions-pending (die Richtung trägt der Compiler, die Position fehlt).

## Offene Pendings

- Eclipse 2024-Kanon-Punkt — gemessen: Δ 793,3 km auf allen fünf Linien gleich
  (Kanon-vs-Algorithmus-Punktdefinition, keine Ephemeriden-Drift) — pending
  (im Paper getragen).

## Abschluss

- Baum beim Sessionsstart: HEAD == origin/main == 9395452, clean. Erledigt aus
  Folge 11: Slab2-Loader-Verdrahtung (parse_slab2 + Archivar-Branch),
  Archivar-Cache-UpdatedAt-Abgleich (cache_fresh_cdn), slab2-CDN-Duty (Asset im
  Release www.sciencebase.gov, Run 34738721765 success), Secrets-Sync (78 Keys),
  3D-Modell-Routen-Recherche (live Funde), Hi-net-Dispatch (Run 34740497220),
  igets-Redispatch (Run 34739297039). Fremde uncommittete Arbeit (beim
  Schreiben): die EEG/OpenNeuro-Linie (src/lib.rs, archivar/{mod,matfile,ndk,
  openneuro_eeg}.rs, tools/measure/src/eeglab.rs, placebo_pair_eeg_probe.rs,
  openneuro_compiler.rs, copernicus_dem_compiler.rs, phi/sources.φ) — unberührt.
  Gepusht wird erst mit dem Wort.
