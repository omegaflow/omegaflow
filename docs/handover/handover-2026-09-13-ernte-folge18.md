<!--
  title: Handover — Ernte-Folge 18 (Stand 2026-09-13)
  session: Ernte-Folge 18
  class: handover
  date: 2026-09-13
  sha256: c3cadfff1126d3ad61ec3a3a79712d8d92017566d1c51edbf47efd02bb7b718f
  status: live
-->
# Handover — Ernte-Folge 18 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Weberin — offene Fäden (Ernte)

- MPC-Beobachtungen — mpcobs.bin (Kometen CmtObs + Satelliten SatObs) steht
  verdrahtet + registriert (`format mpcobs`, netloc minorplanetcenter.net; Parser
  schrieb 96 % / 99 % der Zeilen); der Streaming-Umbau (`mpcobs_compiler`:
  `File::open` + `gunzip_stream` + Zeilen-Puffer + inkrementeller `BufWriter`-
  Schreib, nie mehr das ganze File im Speicher) steht mit Tests. OFFEN (Rat
  2026-09-13, Entscheid): die Shard-Verdrahtung von NumObs.txt.gz (9,1 GB) +
  UnnObs.txt.gz (455 MB) — Shard nach packed-number-Bereich, 2³⁰ Byte je Shard,
  Name `mpcobs-numobs-<lo>-<hi>.bin`, ein Quell-Eintrag mit Shard-Manifest
  (`format mpcobs` je Shard wahr); die Shard-Zahl wird berechnet, nicht
  festgeschrieben. Erster gemessener Schritt des nächsten Atoms: UnnObs (455 MB)
  in einem Job durch den streamenden Compiler — misst (a) wahre unnobs.bin-Größe
  (b) inflate-Durchsatz (c) Upload-Zeit; erst mit diesen drei Zahlen wird der
  NumObs-Job registriert (Schwelle aus Live-Daten).

- Broker/GW-Positionen — pending (gemessene Absenz, re-gemessen 2026-09-13):
  `gwosc.org/eventapi/json/allevents/` (200 anonym) trägt nur
  `luminosity_distance`/`redshift`, kein ra/dec/skymap-Schlüssel;
  `gracedb.ligo.org/api/superevents/` (200 anonym) trägt `far`/`SKYMAP_READY`,
  keine Punktposition — die Skymap ist nur als HEALPix-Map über den files-Endpoint
  erreichbar; `api/events/` 401. Crossref/ADS/Brave/GitHub fanden nur
  Skymap-basierte Lokalisierung (BAYESTAR/MOC, Galaxienkataloge) — keine anonyme
  ra/dec-Punktposition. Der Punkt bleibt pending (Positions-Ernte).

## Abschluss

- Baum nicht ruhig: fremde Sessions arbeiten weiter (src/archivar/las/* Refactor,
  ck.rs, tools/measure/*, ephemeris_compiler.rs — uncommittete fremde Hunks).
  Diese Session committet nur eigene Hunks; Push wartet auf ruhigen Baum + Wort.
