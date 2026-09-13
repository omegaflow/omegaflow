<!--
  title: Handover — Ernte-Folge 16 (Stand 2026-09-13)
  session: Ernte-Folge 16
  class: handover
  date: 2026-09-13
  sha256: e6fba2707c6eaaf964383f141a54be59e2a2385b4ba0a0b838cb75e498a9e1c9
  status: live
-->
# Handover — Ernte-Folge 16 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Wächter)

- hinet.bin — Wächter rot gemessen: Run 34757634879 (main 26e7c07) scheiterte
  „Failed in the data preparation" (8×240s erschöpft) — nicht am `split("<tr")`-Fix,
  der stand schon auf origin/main. Gemessene Ursache: `cont_search`/`submit_request`
  senden `month`/`hour`/`min` ungepolstert („1" statt „01"); das `<select>`-Formular
  verwirft „1" (Fallback September), `cont_request.php` lehnt den ungepolsterten
  Monat ab. Fix `window_date()` (Zero-Pad) + Regressionstest stehen committed
  (hinet_win32_compiler.rs); gepolsterter Request für 2025-01-15 (span 30) wurde
  gemessen „Available" (4853KB). Nächster Schritt: nach dem Push Re-Dispatch
  `hinet-cdn.yml` (--start 2025-01-15T00:00 --span 30 --station-count 10);
  grün → hinet.bin manifestiert, rot → die neue Log-Zeile trägt den nächsten Schritt.

## Weberin — offene Fäden (Ernte)

- Phobos/Deimos-Zweitlinie — die „Pforte nur Typ 2/20" war gemessen veraltet:
  `extract_granules` filtert bereits `2|3|9|13|20`, `SpkType3` + `eval_segment`
  stehen (bsp_reader/spk.rs), `naif_body_ids.tsv` trägt 401/402. NOE-4-2020.bsp
  (244004864 B, sha256 bd05fc94f3f2c873b26bfc420759bc82df306593c1ed4173716b8472b115ab9b)
  ist weiterhin nicht lokal. Nächster Schritt: .bsp fetchen → `ephemeris_compiler`
  für 401/402 → Zweitlinien-Name (eigener netloc, nicht ssd.jpl.nasa.gov) →
  sources.φ + CDN.

- WWLLN — gemessen: `wwlln.net/climate/th_yr/data/WWLLN_th_YYYY.nc.zip` lebt
  (HTTP 200, 58MB zip → klassisches netCDF). Ein Compiler-Scaffold wurde verworfen
  (nutzte `hdf5` statt des vorhandenen `netcdf.rs`). Compiler+CDN-Duty pending —
  bauen mit `NetcdfFile` (src/archivar/netcdf.rs).

- MPC — Distant-Filter gelöst: `mpcorb_compiler.rs` kompiliert jetzt den
  Vollkatalog (`--distant` opt-in); kernel-flatten.yml + sources.φ (mpcorb.bin)
  stehen. OFFEN: MPCAT-OBS/get-obs-Observations-Linie — gemessen 404 (alter Host)
  bzw. get-obs per-Designation ADES-XML (kein Bulk, kein OBS80-Format).

- Broker/GW-Positionen — re-gemessen: api.fink-portal.org tot (Connect 000);
  antares-Loci offen (Richtung+Mag, keine Distanz); neu gemessen
  `gwosc.org/eventapi/json/allevents/` (HTTP 200, anonym) trägt
  luminosity_distance/redshift — aber keine Himmelsposition (ra/dec absent).
  Kein anonymes Einzel-Endpoint liefert Punkt-Position → pending (Positions-Ernte).

- Split-Routing-Verifikation (Wieger) — `Table = off` + DNS aus stehen in den
  5 Proton-Configs; offen: `./bin/proton-exit.sh ca` neu hochfahren +
  direct↔tunnel + die 8 `000`-Hosts je Exit nachmessen (geo-block oder echt tot).
  sudo/Netz auf der Operator-Maschine.

## Abschluss

- Baum nicht ruhig: fremde Sessions arbeiten weiter (las.rs→las/ Refactor bricht
  die Kern-Lib — 16 Fehler in las/laszip.rs; volume_builder.rs + volume-cdn.yml;
  archive_search staged; 2 ungepushte fremde Commits a928a44/4a84b1d).
  Diese Session committet nur eigene Hunks; Push wartet auf ruhigen Baum + Wort.
