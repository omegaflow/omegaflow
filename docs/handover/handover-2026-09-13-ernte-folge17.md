<!--
  title: Handover — Ernte-Folge 17 (Stand 2026-09-13)
  session: Ernte-Folge 17
  class: handover
  date: 2026-09-13
  sha256: 2de9bc504daa147a399ab59c1509ddad82b5eb712968387ff4ea716c8bbe71f3
  status: live
-->
# Handover — Ernte-Folge 17 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Wächter)

- hinet.bin — Zero-Pad-Fix steht auf origin/main; Re-Dispatch lief
  (`hinet-cdn.yml`, Run 34769189536, start 2025-01-15T00:00, span 30,
  station-count 10, dispatched 2026-09-13T16:38Z). Grün → hinet.bin manifestiert;
  rot → die neue Log-Zeile trägt den nächsten Schritt.

## Weberin — offene Fäden (Ernte)

- MPC-Beobachtungen — mpcobs.bin (Kometen CmtObs + Satelliten SatObs) steht
  verdrahtet + registriert (`format mpcobs`, netloc minorplanetcenter.net; Parser
  schrieb 96 % / 99 % der Zeilen). OFFEN: NumObs.txt.gz (9,1 GB) + UnnObs.txt.gz
  (455 MB) brauchen die Streaming-Umschrift — `mpcobs_compiler` lädt das ganze
  File in den Speicher (`std::fs::read` + `gunzip` + `String::from_utf8_lossy`);
  die Linie ist `gunzip_stream` (src/archivar/inflate.rs:351) + inkrementeller
  Zeilen-Puffer + inkrementeller Schreib. NumObs/UnnObs bleiben bis dahin aus dem
  Workflow.

- Broker/GW-Positionen — pending (gemessene Absenz): kein anonymer
  Einzel-Endpoint liefert Punkt-Position (gwosc allevents trägt Distanz/Redshift,
  aber keine ra/dec).

- Split-Routing-Verifikation — `Table = off` + DNS aus stehen in den 5 Proton-Configs;
  offen: `./bin/proton-exit.sh ca` neu hochfahren + direct↔tunnel + die 8 `000`-Hosts
  je Exit nachmessen (geo-block oder echt tot). sudo/Netz auf der Operator-Maschine.

## Abschluss

- Baum nicht ruhig: fremde Sessions arbeiten weiter (las/ Refactor + ck.rs WIP
  brechen die Kern-Lib — `cargo check` scheitert an fremden uncommitteten Hunks,
  nicht an dieser Session; volume_builder + volume-cdn.yml). Diese Session committet
  nur eigene Hunks; Push wartet auf ruhigen Baum + Wort.
