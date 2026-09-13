<!--
  title: Handover — Ernte-Folge 15 (Stand 2026-09-13)
  session: Ernte-Folge 15
  class: handover
  date: 2026-09-13
  sha256: 4a0d58ec6e6a092aa633460b9e00368835c8f56dca241092619852aef83cb4f6
  status: live
-->
# Handover — Ernte-Folge 15 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Wächter)

- hinet.bin — Fix `split("<tr")` steht auf origin/main (26e7c07). Dispatch
  34757634879 (main 26e7c07, --start 2025-01-15T00:00 --span 30 --station-count 10)
  läuft; die Conclusion ist zu lesen — grün → hinet.bin manifestiert, rot → die
  Log-Zeile ist der nächste Schritt. Kein weiterer Dispatch, bis der Lauf gelesen ist.

## Weberin — offene Fäden (Ernte)

- Phobos/Deimos-Zweitlinie — NOE-4-2020.bsp (ftp.imcce.fr/pub/ephem/satel/NOE/MARS/2020/,
  244004864 B, sha256 bd05fc94f3f2c873b26bfc420759bc82df306593c1ed4173716b8472b115ab9b)
  live. pck00010.tpc trägt BODY401/402; der generische ephemeris_compiler + SPK-Reader
  kennt 401/402 (naif_body_ids). Gemessener Blocker: die Granule-Pforte
  `extract_granules` (src/archivar/ephemeris.rs:289) lässt nur SPK-Typ 2/20 durch —
  NOE-4 trägt nur Typ 3 (Chebyshev pos+vel) → „SKIP phobos: no granules in any
  kernel". Nächster Schritt: Pforte auf Typ 3 erweitern (Design: pos+vel ins
  Granul-Format), dann Compile + Zweitlinien-Name + Registrierung + CDN.
- WWLLN — Nachfolger-Host ghrc.earthdata.nasa.gov (WWLLN-DAAC, HS3 WWLLN Storms V1
  C1979872496) + wwlln.net/climate/th_yr/ netCDF-Jahresgitter. Absent lokal.
  Compiler+CDN-Duty pending (wie iss_lis).
- MPC — Vollkatalog absent lokal (mpcorb_distant.bin = TNO-Teilmenge auf dem CDN);
  Distant-Filter lösen + MPCAT-OBS/get-obs-Linie. Host cgi.minorplanetcenter.net
  (mpeph2.cgi) neu → pending.
- Broker/GW-Positionen — antares.noirlab.edu abgelehnt (credential-gated, nur
  Counts/Position); api.fink-portal.org tot (DNS ok, Connect 000); bayestar: keine
  neuen Positions-Hosts. Der Compiler trägt nur die Richtung → pending (Positions-Ernte).

## Abschluss

- Baum nicht ruhig: eine fremde Session hard-resettet den Arbeitsbaum (`git reset HEAD`,
  im Reflog gemessen) — das hat diese Session's sources.φ- und Paper-Edits zweimal
  gewischt; sie stehen jetzt committed. Fremde untracked-Reste beim Abschluss: ??
  .playwright-mcp/, ?? tools/utils/src/bin/archive_search/{playwright.rs,playwright_fetch.cjs}.
  Commit lokal; Push wartet auf ruhigen Baum + Wort.
