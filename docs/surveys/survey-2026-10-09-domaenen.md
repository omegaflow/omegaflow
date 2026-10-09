<!--
  title: Survey — Domänen: was der Baum trägt (2026-10-09)
  class: survey
  date: 2026-10-09
  sha256: 834224df5ed200e1fe2b4dabfc82c93839421e9b6bf5cb6401e1528eac0a6751
  status: live
  see-also: docs/concepts/4d-membrane.md docs/concepts/kybernetische-astrophysik.md docs/surveys/survey-2026-10-04-exposom-matrix.md
-->
# Survey — Domänen: was der Baum trägt (2026-10-09)

Operator-Wort 2026-10-09 (Mountain): die Domänen-Landschaft dokumentieren. Die
Vorlage liegt als Future-Dossier privat (`state/future/omegaflow-anwendungsfelder-2026-10-09.md`,
`state/future/omegaflow-ist-stand-2026-10-09.md`). Diese Dossiers sind
**Behauptungen**, keine Messungen; diese Survey misst jede tragende Domäne am
Baum (HEAD `c8a509baf`) und nennt die Lücken als Lücken. Wo eine Dossier-Zahl
hier nicht nachgemessen ist, steht `pending` — keine kopierte Zahl.

## Methode

Gegen den Baum gemessen: `awk`/`find`/`wc` über `phi/sources.φ`,
`find src/archivar`, `find docs/paper`, `archive_search --count --root docs`.
Die Anker je Domäne sind gemessene Host-Zeilen im getrackten Baum.

## Was der Baum trägt (gemessen)

### Quellen-Register
`phi/sources.φ`: **2.699** `url`-Zeilen, **132** distinkte Hosts. Davon **1.994**
auf `github.com` (die CDN-manifestierten Assets) und **705** direkte
Upstream-Endpunkte. Die Dossier-Zahlen („2.698", 132 Hosts, 705/1.993) decken
sich bis auf eine Einheit — Register-Ordnung und Registerstand stimmen.

### Die sechs Träger (gemessene Host-Zeilen in `phi/sources.φ`)

| Träger | gemessene Anker-Hosts (Zeilen) |
|---|---|
| **Astro** | `naif.jpl.nasa.gov` 13 · `tapvizier.cds.unistra.fr` 9 · `vizier.cds.unistra.fr` 4 · `skyserver.sdss.org` 2 |
| **Helio / Space-Weather** | `imag-data.bgs.ac.uk` 154 (INTERMAGNET/BGS) · `cdaweb.gsfc.nasa.gov` 15 · `services.swpc.noaa.gov` 13 · `vires.services` 12 · `www.nmdb.eu` 2 |
| **Geophysik / Seismik** | `data.earthscope.org` 9 · `earthquake.usgs.gov` 3 · `seismic-api.science.unimelb.edu.au` 3 · `arclink.ethz.ch` · `webservices.ingv.it` · `api.geonet.org.nz` · `servisnet.afad.gov.tr` · `ceic.ac.cn` |
| **Ozean** | `hfradar.ioos.us` 204 · `www.ndbc.noaa.gov` 76 · `data.oceannetworks.ca` 4 · `data-argo.ifremer.fr` 3 · `data.neracoos.org` · `ceotr.ocean.dal.ca` · `erddap.marine.ie` · `uhslc.soest.hawaii.edu` |
| **Klima / Atmosphäre** | `gml.noaa.gov` 5 · `gs.llnl.gov` 4 · `aviationweather.gov` 3 · `api.open-meteo.com` |
| **Luft / Strahlung** | `api.openaq.org` · `aqs.epa.gov` · `www.imis.bfs.de` · `aeronet.gsfc.nasa.gov` · `api.woudc.org` · `data.sensor.community` |

Diese sechs sind die **gemessen getragenen** Domänen — dieselbe Achse, die
`state/future/omegaflow-ist-stand-2026-10-09.md` als Trägerschaft nennt
(Astro · Helio · Geophysik · Ozean · Klima · Luft).

### Code-Register (gemessen)
- `src/archivar`: **244** Rust-Dateien (`find src/archivar -name "*.rs"`); das
  Dossier nennt 243 Module — Zählweise, kein Widerspruch.
- `#[cfg(test)]`: **285** Treffer in **258** Dateien (`archive_search --root src`).
- `docs/paper`: **40** Dateien mit `class: paper`. Das Dossier nennt „~27
  Papers" — am Baum gemessen sind es **40**; die Zahl ist **nicht reproduziert**
  und bleibt offen (paper-Zählweise gegen Ergebnis-Papers).

### Der Instrument-Anker (DNA)
Die DNA benennt omegaflow als Messinstrument, nicht als Werkzeug:
`docs/concepts/4d-membrane.md:26` („one stretches a measurement surface …")
und `docs/concepts/kybernetische-astrophysik.md:18-20` („Das Brett ist
universal …"). Beide Dateien stehen im Baum. Der Anspruch „trägt bereits
Medizin/Psychiatrie/Klima/Erde/Ozean" hat seine gemessenen Quellenanker in
`docs/surveys/survey-2026-10-04-exposom-matrix.md` und
`docs/surveys/survey-2026-10-03-medizinische-datenquellen.md`.

## Die Lücken (gemessen, nicht geglättet)

`archive_search --root docs --count`:
- **Frieden** — 0 Treffer. · **Demokratie** — 0. · **Erdwohl** — 0.
  · **Psychotherapie** — 0.
- **Bildung/Didaktik** — als Erziehungs-Domäne 0; die 66 Rohtreffer sind
  `Abbildung`/`Pass-Bildung` (in `docs/concepts` gemessen nur „Pass-Bildung").
  Keine Bildungs-Vision.
- **Kunst** — 2 Rohtreffer, beide beiläufig (`Messkunst`, „Kunstprojekt" in
  einer FMHY-Aussage). Keine Kunst-Domäne.
- **Heilung** — 93 Dateien / 126 Treffer, alle als **Code-/Register-/CI-Heilung**;
  keine Patienten-Heilung (gemessen: kein medizinischer Heilungs-Armer).

Diese Lücken decken sich mit der Dossier-Aussage: getragen sind
Astro/Helio/Geo/Ozean/Klima/Luft; die sozial-humanistischen Sparten (Frieden,
Demokratie, Bildung, Kunst) und die Heilung tragen keine Quelle.

## Pending (in diesem Schritt nicht verifiziert)

- Die konsolidierte **44-Domänen-Liste** des Dossiers — nur die sechs
  Trägerfamilien sind hier gemessen; alle weiteren Einträge (Medizin/Krebs,
  Koma, Traum, Allergie, Tier/Pflanze, Verkehr, Stromnetze, SETI, Raumfahrt,
  Barrierefreiheit, Spiritualität, Archäologie …) bleiben **pending**, bis ihre
  Quelle im Register bzw. ihr Code-Pfad gemessen ist.
- Die gebauten Reader-Arme (FITS · HDF4/5 · NetCDF-4 · Zarr · PDS · SPICE ·
  QuakeML · STAC · FLAC) — im Dossier benannt, hier nicht einzeln nachgemessen:
  **pending**.
- Der medizinische Strang hat Quellen und ein Matrix-Design
  (`survey-2026-10-04-exposom-matrix.md`), aber **kein gemessenes medizinisches
  Resultat** — pending.

## Riss

Die Dossier-Zahlen liegen dicht am Baum (2.698/2.699 Quellen; 132 Hosts;
243/244 Archivar-Dateien; 283/285 `#[cfg(test)]`) — Zählweise, kein
Widerspruch. Der eine offene Riss ist die Paper-Zahl (Dossier „~27", Baum
**40**): sie wird als offen getragen, nicht geglättet.
