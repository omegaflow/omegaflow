<!--
  title: Handover — Reibung & Werkzeug (Stand 2026-09-14)
  session: Sprache & Reibung
  class: handover
  date: 2026-09-14
  sha256: d3d1ddac6bd6e30f393cdcf5110f66feec5543b05ec6c565a583b332184993e4
  status: live
  see-also: AGENTS.md docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md
-->
# Handover — Reibung & Werkzeug (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder Punkt
trägt seinen nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder
Anfrage.

## S3-Reader — die größte Werkzeug-Lücke (härtester undatierter Punkt)

- Die EarthData-Ports sind gemessen offen, aber token-gated: GRACE-FO/SWOT
  (PODAAC), SMAP (NSIDC), CDDIS IONEX antworten anonym 403/307; GES-DISC
  `/data/` und AppEEARS `/api/product` sind anonym 200. Fehlt: `s3://`→HTTPS-
  Mapping, EDL→S3-Credentials-Exchange (`.../s3credentials`) und SigV4 in
  `src/archivar/range.rs`; Header-Injektion in `xml_harvester` (`page`/`run_s3`).
  Schritt: `range.rs` um Auth-Header + Bucket-Mapping erweitern, `EARTHDATA_EDL_TOKEN`
  als Bearer, ListObjectsV2 token-gated; Test gegen `podaac-ops-cumulus-public`.

## AMS-02-Konsument — Endpoint offen, Konsument fehlt (Domänen-Entscheid)

- `phi/dead_sources.φ:415` ist widerlegt: HEASARC `ams02spec`/`ams02rates` offen
  als FITS (`.../W3Browse/w3query.pl?tablehead=name%3Dams02spec&displaymode=FitsDisplay`)
  und TDAT (`/FTP/heasarc/dbase/tdat_files/heasarc_ams02spec.tdat.gz`); der Reader
  steht (`omegaflow::fits`, ASCII-TABLE + BINTABLE). Offen ist der **Konsument**:
  Teilchen ist Abstammung, kein Feld — frisst eine Nadel den Fluss als Zeuge/
  Provenienz? Schritt: Rat/Operator entscheidet den Konsumenten, dann Compiler.

## Reibung — Vokabular-Korrektur (Rat #5)

- `friction_vocab.txt` mischen Kalender-Struktur in die Reibung. `wiedervorlage`
  → `calendar`, `pausiert` → `paused`, `warten auf rückmeldung` → `external-wait`
  umklassieren; die datierte Wiedervorlage ist gesunde Ruhe, nicht Aufschub.
  Schritt: Vokabular editieren, dann `cargo run -p omegaflow-utils --bin giveup_scan
  -- --vocab tools/utils/src/bin/friction_vocab.txt --root docs --summary` (die
  wahre Reibungsfläche ist kleiner als 431).

## Reibung — die fear-honest-Grabstellen (Seed des Lookup-Index)

- Die fear-honest-Klasse (135 Stellen) ist die Karte: jede Stelle bekommt ihren
  nächsten Schritt — die Seed-Einträge des `register_lookup`-Index. Schritt:
  `giveup_scan --vocab friction_vocab.txt --root docs --class fear-honest`, je
  Stelle den Schritt benennen; das Wort `ehrlich` wird durch den Pfad überflüssig.

## Quellen — die Wand-Party: offene Wege, noch nicht registriert

- FOUND, aber ohne Ernte/Konsument: Scopus (non-commercial free API-Key,
  `dev.elsevier.com`), Dimensions (freie Metrics-API, `metricssignup`), NOAA
  MarineCadastre AIS (CC0, historisch), NOAA GLM (Blitz, `noaa-goes16` S3),
  ThingSpeak (öffentliche Channels anonym), GNIP (Mendeley `75tsccprd2` +
  `waterisotopesDB.org`), Semantic Scholar (keyless + `influentialCitationCount`/
  `tldr`). Schritt: je Quelle die Speisekammer-Tore + ein benannter Konsument,
  dann `phi/sources.φ`.

## Quellen — Parser-Lücken (gemessene Crates, noch nicht gebaut)

- LASzip (`laz 0.13.0` pure Rust + `laszip-sys`), JPEG-in-TIFF (`oxiarc-tiff`/
  `oxideav-tiff`), RINEX-2.11/Hatanaka (`rinex 0.22.0`, `crx2rnx`), GSICS
  (netCDF-GPRC + JMA-CSV), GLO-30 (geotiff-reader/cloudtiff; AWS-Spiegel
  `copernicus-dem-30m`). Schritt: je Gap den Reader gegen den gemessenen Endpoint
  bauen (die Crates sind gewogen).

## Werkzeug — archive_search (diese Session gebaut)

- Gebaut: Transport-Naht (Exit-Leiter direct→proton*→socks, on-block; Rate-Gate),
  `magic`-Sniffer, PDF-Stripper, `--datacite`, `--sniff` (magic+sha256), `--zenodo`,
  `--isc` (FDSN), `--openalex`, Auto-Pagination (crossref/zenodo/openalex),
  EarthData-Token-Hook (401), `--supermag`, `--heasarc`. Offen: der Token-Hook
  ist live unverifiziert (braucht `EARTHDATA_USER`/`EARTHDATA_PASS`); die
  ISC-EHB-Ausgabe-Grammatik (`web-db-v4` `out_format`) bleibt offen/HTML.
  Schritt: `EARTHDATA_*` setzen + `--isc`/`--sniff` gegen eine EarthData-URL proben.

## Reibung — das Toolkit (diese Session gebaut)

- `giveup_scan` (+ `giveup_vocab.txt`, `friction_vocab.txt`, `bloat_vocab.txt`),
  `bloat_scan`, `register_lookup`. Offen: die **Praxis** — der Scan bei Session-
  Start, `register_lookup <term>` als erste Bewegung beim Unbekannten. Schritt:
  die Regel in `AGENTS.md` ist gesetzt; die nächste Session führt sie aus.

## Keine konsumierte Übergabe

- `handover-2026-09-13-entscheid-folge5.md` blieb auf Operator-Wort unverändert
  (nicht konsumiert) — kein Archiv-Schub.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
