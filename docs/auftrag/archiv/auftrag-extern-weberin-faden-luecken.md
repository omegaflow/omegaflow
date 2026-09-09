<!--
  title: Auftrag (extern) — Weberin-Faden-Lücken: offene Routen für neun pending Kategorien
  class: auftrag
  date: 2026-09-07
  sha256: 591a9db73c26ac2e2ab11555f886125029388a221e38d11ca7653443d8d96f8c
  status: archived
  see-also: docs/surveys/survey-2026-09-07-weberin-thread-matrix.md docs/concepts/die-weberin.md
-->

# Rechercheauftrag (extern): Weberin-Faden-Lücken — offene maschinenlesbare Routen

Die Weberin-Faden-Matrix (`docs/surveys/survey-2026-09-07-weberin-thread-matrix.md`)
registriert neun Faden-Kategorien mit noch keinem Actor. Für jede ist zu finden: eine
maschinenlesbare Route (Stationsliste ODER Ereignis-Feed MIT Position lat/lon — der
Feld-Block braucht datengetragene Position), ihr Zugangscharakter, und ein gemessenes
HTTP-Verdikt. Diese Datei ist selbsttragend; wer sie ausführt, liest vorab
`docs/SOURCE_PORT.md` (§7–§9, §11, §13) für die Taxonomie und die Prüf-Kaskade. Es
wird KEINE Datei geschrieben, kein Register editiert, kein Force-Gate entschieden —
es wird nur benannt, was die Messung IST, und Befunde gemeldet.

## Status-Vokabular (bindend)

- `live` — 200, offen, maschinenlesbar.
- `blocked` — lebt, Zugang gesperrt (Form nach Blocked-Kanon: key/account/ip-blocked; reg-Zeile).
- `declined` — lebt, aber keine physikalische Messung am Punkt (Modell/Vorhersage/positionslos) oder kommerziell.
- `pending` — Route bekannt oder geahnt, Verdikt ausstehend.
- `not-published` — kein offener maschinenlesbarer Weg nach abgeschlossener Suche. Ein abwesender/geschlossener Zugang wird IMMER als `not-published` gemeldet — nie durch einen Ersatz gefüllt (B-Feld statt GIC, Modell statt Messung, PNG statt Gitter).

Regel: Ein toter Endpoint ist kein Endzustand — erst die Kaskade direkt curl →
r.jina.ai-Präfix → WebArchive → Websuche, dann ein Verdikt mit note, die den
Recherche-Stand nennt. Spekulationswörter sind verboten; ein ungemessener Befund heißt
`pending`. Jeder Befund trägt das Datum der Messung.

## Ausgangslage pro Kategorie (gemessen am 2026-09-07, nicht neu suchen)

Jede Kategorie nennt die Disk-Funde (Adresse), die bereits gemessenen Status, und die
offene Frage. Der Bestand ist `apis/harvester-leads.txt`, die Arena `apis/APIs/`.

### 1. Boden-Gravimeter (gravity) — IGETS/GGP supraleitend
- Disk: harvester-leads §1.7j (Z.214-223): GFZ-IGETS-Index in katalog/, Level-2/3-Zeitreihen über ISDC/GFZ HTTP; „nicht in sources.φ" — Registrierungslücke.
- Gemessen: `isdc.gfz-potsdam.de` 200; alter Host `igets.u-strasbg.fr` refused.
- Offen: Liegt unter GFZ ISDC eine maschinenlesbare Stationsliste MIT Position und eine Zeitreihen-Route ohne Konto? Pfad, Format, Auth-Form. Finde den konkreten IGETS/GGP-Datenbank-Einstieg und prüfe ihn; Stations-Anzahl und Messgröße (nm/s²) notieren. Fehlt der offene Weg: `not-published` mit note, dass ISDC-Portal lebt.

### 2. Infraschall — CTBTO IMS (acoustic, eingeschränkt)
- Disk: harvester-leads Z.17-18, 51, 115-126, 429, 474, 515, 614, 732: Rohwellenformen nur NDCs/vDEC (Antrag + Vertrag, zero-cost); abgeleitete Detektionslisten offen publiziert (Hupe et al. 2022, ESSD, PMCC-reprocessed 2003-2020, ~60 Stationen).
- Gemessen: Standalone-`vdec.ctbto.org` 404; aktueller vDEC-Einstieg unbekannt.
- Offen: (a) Der offene abgeleitete Detektions-Datensatz — DOI/Repository (ESSD/Zenodo), Format, Spalten, Position der Stationen, Zeitraum; (b) der heutige vDEC-Einstiegspfad für den Vertragsweg (`blocked account`, reg-Zeile). Roh ohne Vertrag = `not-published`, KEIN Ersatz durch seismische Kanäle.

### 3. Hydrophon / Ozean-Lärm (acoustic)
- Disk: harvester-leads Z.132-152 (ONC NEPTUNE/VENUS Oceans 3.0 REST; NOAA/NPS NRS öffentlich, 63 kumulative Jahre); Arena batch_12 MBARI MARS; batch_15 NCEI ocean-noise (`ncei.noaa.gov/products/ocean-noise`); batch_06 PMEL DART GeoJSON.
- Gemessen: NCEI-Produktseite 404 (umgezogen); `pmel.noaa.gov/acoustics` 200.
- Offen: Wo liegt die aktuelle maschinenlesbare NRS/Ocean-Noise-Dateiliste (NCEI-Neupfad, THREDDS/ERDDAP)? ONC Oceans 3.0: existiert ein Hydrophon-Stations-/Deployment-Feed mit Position, getrennt vom bereits registrierten CTD-Kanal? Format + Beispielabfrage notieren.

### 4. Seismische Stations-Netze als Weltlinien (seismic, nur Ereignisse im Register)
- Disk: API_gaps:27 IRIS Seismic Stations; Arena Z.2490 + 2715-2763 (FDSN-StationXML ist der Kanal; fertiger text-Query-Draft Raspberry Shake, Netz AM); grind_domain_coverage: EarthScope fdsnws/event 410; grind_b2find: fdsnws 404 auf anderem Host.
- Gemessen: `service.earthscope.org/fdsnws/station/1/query?level=network&format=text` 200 und `service.iris.edu/...` 200 — Stations-Webservice lebt.
- Offen: Vollständige aktive Stationsliste (`level=station`, `format=text`) für ein Breitband-Netz als Beleg ziehen — Netz-Codes, Kanal-Ebenen, Positionsfelder, Umfang. Die Station ist die Weltlinie; der Ereignis-Feed existiert bereits und ist nicht das Ziel.

### 5. GIC — geomagnetisch induzierte Ströme (electric)
- Disk: kein GIC-spezifischer Fund; nur generisch-geomagnetischer Bestand; dB/dt-Bins (abk/sod) als Induktions-Treiber in-register.
- Gemessen: keine offene GIC-Route; SuperMAG 200 (B-Feld — Nachbar-Messung, kein GIC).
- Offen: Gibt es einen offenen Datensatz GEMESSENER GIC an Transformatoren-Neutralen (Zeitreihe mit Stationsposition; publizierte Datensätze z. B. auf PANGAEA/Zenodo, nationale Netze)? SWPC-GIC-Prognose ist ein Modell → `declined`, kein Ersatz. Bleibt die Suche leer: `not-published` mit note „gemessene GIC-Stationen offen nicht auffindbar".

### 6. Blitz-Bodennetze — WWLLN u. a. (electric)
- Disk: harvester-leads Z.165-166, 361, 430 (WWLLN akademisch per Antrag, GLD360 kommerziell → refused); Arena Z.2472 + 2492-2493 (Realtime nicht öffentlich; nur Monthly Thunder Hour ab 2013 offen); API_gaps:56 GLM 50%.
- Gemessen: `wwlln.net` 200; GHRC-ERDDAP refused (Route down).
- Offen: Aktueller offener Host der WWLLN-Thunder-Hour-Daten (Nachfolge des GHRC-ERDDAP, NASA-Data-Path), Format + räumliche Auflösung. Echtzeit-Roh = `not-published` ohne Mitgliedschaft. GLM (em) ist bereits in-register — kein Doppel.

### 7. SuperDARN-Polar-Radar (electric)
- Disk: harvester-leads Z.445/482 (DMap-Binärformat — eigener Compiler nötig), Z.512, Z.670 (Virginia Tech + Kanada-Knoten); queue/master.φ:6230 Konvektionskarten-Block (PNG, positionslos — declined als Datenquelle); Archeology superdarn.ca/plotting/latest.
- Gemessen: `superdarn.ca` 200, `vt.superdarn.org` 200, `data.superdarn.ca` DNS-frei.
- Offen: Der maschinenlesbare Daten-Download (Grid-/FITACF-Dateien) MIT Stationsliste (Radar-Positionen): Portalpfad, Login-Form (`blocked account`?), Format DMap. Konvektions-PNG ist declined; das Gitter/der FIT-Strom ist das Ziel.

### 8. HF-Radar-Oberflächenströme (advective) — US IOOS HFRNet
- Disk: harvester-leads Z.23-24, 42, 62, 513 (National-DAC-Gitter, Stunden/Monat/Jahr, THREDDS/OPeNDAP + ERDDAP; SCCOOS/CeNCOOS/SECOORA); Archivkopie cmr_catalog CODAR Site Locations.
- Gemessen: `hfrnet-tds.ucsd.edu` timeout (kein dead — Datacenter-Exit, lokal erneut prüfen, sonst r.jina.ai/Kaskade); Alternativ-Host DNS-frei.
- Offen: Korrekte aktuelle TDS-/ERDDAP-Adresse des HFRNet-DAC, Stationsliste der Radar-Sites (Position), Gitter-Datensatz-IDs und Zeitauflösung. Timeout ist `pending`, nicht tot — erst Kaskade, dann Verdikt.

### 9. BGC-Argo (diffusion) — O2/pH/Nitrat/Chlorophyll
- Disk: harvester-leads Z.14-15, 184-193, 363, 401 (GDAC Ifremer `erddap.ifremer.fr`, PolarWatch-ERDDAP, Argovis); Arena batch_5 ARGO-O2 via IFREMER; argovis-api 400 dead (registriert); phys. Argo-Float R1901843 einzeln in-register.
- Gemessen: `erddap.ifremer.fr/erddap/index.json` 200.
- Offen: Die BGC-Datensatz-IDs auf dem Ifremer-ERDDAP (bio-/bgc-Präfixe), die Variablen (O2/pH/NO3/Chl-a) und die Float-Position je Profil; alternativ der offizielle BGC-Argo-Index (`argo_bio-profile-index`) mit Pfad. Der bestehende `erddap_harvester` ist das Werkzeug — der Befund nennt Dataset-ID + Variablen, nicht Code.

## Abschluss je Kategorie (ein Befund-Block)

Für jede der neun Kategorien ein fester Befund:
1. Was die Messung IST (Kraft, Größe, Einheit) — kein Force-Gate-Urteil.
2. Die gefundene Route (URL, Format, Stations-/Ereignis-Liste mit Position ja/nein) mit gemessenem HTTP-Status und Datum.
3. Das Verdikt aus dem Status-Vokabular.
4. note: was IST (Herkunft, Umfang, offene Restfrage). Absent/geschlossen heißt `not-published` — nie ein Ersatz, nie eine Null.
