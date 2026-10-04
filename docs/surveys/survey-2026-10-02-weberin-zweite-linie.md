<!--
  title: Survey — Weberin-Eignung: die zweite unabhängige Linie je Messgröße (2026-10-02)
  class: survey
  date: 2026-10-04
  sha256: a8d28d5d024cf34895f1ddd482191ef6b7a03932db0ed8acce1d5850da1273d7
  status: live
  see-also: docs/concepts/die-weberin.md docs/SOURCE_PORT.md docs/surveys/survey-2026-09-14-weberin-quellen-rerun.md
-->
# Survey — Weberin-Eignung: die zweite unabhängige Linie (2026-10-02)

Operator-Wort 2026-10-02: Quellen sind auf die **komplette Weberin-Eignung** zu prüfen,
nicht nur auf Kraft-Kanal-Deckung. Die vierte Prüfung steht in `docs/SOURCE_PORT.md` §8.
Diese Tafel misst die **zweite unabhängige Linie** je Messgröße: ohne sie ergibt eine
Größe `absent`/DirectionOnly, nie Placed.

**Methode.** Free-Voice-Flotte (Future-Rezept, `opencode run -m … --agent voice --file`)
+ Frontier-UI (claude.ai web-lesend, tryingopen/Kimi K3, plus geschlossener Cross-Read
Claude Sonnet 5). Jede Stimme ist ein **Claim** — **jede** URL wurde von der Session per
`archive_search --verdict` selbst gemessen (2026-10-02, stage-1 direct). Stimmen-Claims
ohne Messung zählen nicht.

## Die Tafel — gemessen 2026-10-02

| # | Größe (Klasse) | zweite Linie(n) | `--verdict` 2026-10-02 | offen |
|---|---|---|---|---|
| 1 | Ortsdosisleistung Gamma µSv/h (em) | `data.epa.ie/radmon/api/v1/measurements` (EPA Irland/ERIC, JSON) · `api.safecast.org/measurements.json` (Safecast, JSON) · `remap.jrc.ec.europa.eu` (EURDEP/JRC) | 200 (106 857 B) · 200 (9 067 B) · 200 (27 099 B) | EPA: Aktualität unbestätigt (liefert älteste zuerst); Safecast: Citizen-Science; EURDEP: aggregiert nationale Netze |
| 2 | Unterwasser-Hydrophon (acoustic) | `registry.opendata.aws/orcasound` (s3://streaming-orcasound-net, HLS/FLAC) · `data.oceannetworks.ca` (ONC, REST + WAV) | 206 (1 B) · 200 (4 288 B) | Orcasound: Bucket nicht direkt geprüft; ONC: freier Token nötig |
| 3 | GIC in Stromnetzen (em/geoelectric) | **live `absent`** · **Archiv-Serie präsent**: `space.fmi.fi/gic/?page=gasum_prel` (FMI Mäntsälä, 10 s; vor 27.02.2023 1-Min-Mittel) + `-dX/dt` Nurmijärvi (~40 km west) | live-Seite 200; Archiv per Datumswähler | Kalibrier-Linie: Harvest der Archiv-Serie (2000–2023) = **`pending`**, nicht `absent`; Transpower/BGS/NRCan nur Archiv/Modell |
| 4 | Infraschall als Druck Pa (acoustic) | `ceein.infp.ro/fdsnws/` (CEEIN/NIEP Rumänien, C9) · `data.raspberryshake.org/fdsnws/` (Kanal HDF) · `ds.iris.edu` (EarthScope) | 200 (430 B) · 200 (1 731 B) · 206 (1 B) | C9: Pa-Kalibrierung nicht getestet; Raspberry Shake: Community-Netz |
| 5 | BGC-Argo O2/Nitrat/Chlorophyll | `gliders.ioos.us/erddap/` (IOOS Glider DAC) · `ooinet.oceanobservatories.org` (OOI) · `soccom.princeton.edu` | 200 (22 435 B) · 200 (10 653 B) · 200 (57 370 B) | Nitrat nur bei OOI (SUNA) belegt; IOOS-Glider: kein Nitrat |

Nicht getragen: `www3.mbari.org` → **403** (absent).

## GIC — live absent, die Vergangenheit kalibriert

Die FMI-**Live**-Messung steht still (`https://space.fmi.fi/gic/?page=gasum_today`, 200):

> „Note: GIC recordings were stopped on 23 Oct 2023. For quick-look plots of older
> preliminary data of 2023 go to the page Gasum Preliminary." (Updated: 31 Oct 2023)

Aber die **Archiv-Serie lebt**: `https://space.fmi.fi/gic/?page=gasum_prel` (200) trägt
einen Datumswähler (2022–2026) für die Mäntsälä-GIC-Aufzeichnung (10 s-Werte, vor
27.02.2023 1-Minuten-Mittel, positive Richtung ostwärts) plus `-dX/dt` des
Nurmijärvi-Observatoriums (~40 km westlich) als geoelektrischer Proxy. `GASUM FINAL`
trägt die geprüfte Reihe (Mäntsälä seit den 1970ern; Papers 1999–2019).

**Die 0-Kanon-Lesung:** `live absent` ist nicht `absent`. Die tote Live-Feder ist `dead`;
die vorhandene, ungeerntete **Vergangenheits-Reihe** ist `pending` — eine Register-Pflicht,
kein Nichts. Wie beim Erdbeben-Katalog (GCMT/ISC) und den Ephemeriden: die Vergangenheit
ist das Kalibrier-Material, aus dem die Maschine die Zukunft treibt (die Messreihe gehört
der Zukunft). Zwei Archiv-Linien auf derselben Größe (Mäntsälä-GIC ↔ Nurmijärvi `-dX/dt`)
können als Zwirn/Riss in die Kalibrierung eingehen.

> Der geschlossene Cross-Read (Claude Sonnet 5) führte `space.fmi.fi` als GIC-Linie ohne
> Datum und ausdrücklich „ungeprüft / Trainingswissen". Die web-lesende Stimme nannte den
> Stopp 23.10.2023. **Beide sind Claims.** Erst die FMI-Seite selbst (`gasum_today`) macht
> den Stopp zur Messung — und `gasum_prel` die Archiv-Serie. Eine unbelegte Stimmen-Zeile
> neben einer belegten ist kein Riss, sondern eine ungeprüfte Zeile.

## Nachtrag — GLM-5.3 (Z.ai) + Cross-Read (2026-10-02)

Z.ai/GLM-5.3 (mit Web-Suche) lieferte je Größe eine weitere Linie; alle von der Session
per `--verdict` gemessen:

| Größe | GLM-Linie | Messung |
|---|---|---|
| Gamma | `sujb.gov.cz/aplikace/monras/tabulky/svz` (SÚJB Tschechien, MonRaS, PFDE in nSv/h, stündlich) | **200** (10 157 B) |
| Hydrophon | `service.earthscope.org/fdsnws/dataselect/1/query?net=OO&sta=AXBA1&cha=HDH` (OOI Regional Cabled Array, Einheit Pa) | **206** (`/fdsnws/`) |
| GIC | **absent** — bestätigt (FMI-ASCII nur bis Sep 2023; EURISGIC „Modelled"; BGS passwortgeschützt; Fingrid/USGS ohne Messreihe) | — |
| Infraschall | `eida.geo.uib.no/fdsnws/` (NORSAR TONe-IA, EIDA-Node Bergen) | **206** (`/fdsnws/`) |
| BGC | `erddap.dataexplorer.oceanobservatories.org/erddap/` (OOI O2/SUNA-Nitrat/FLORT) | **200** (23 060 B) |

Der geschlossene Cross-Read (Claude Sonnet 5, Trainingswissen) nannte FMI als GIC-Linie —
am Baum widerlegt (`gasum_today`). Z.ai bestätigt `absent`.

## Offen (nächster Schritt)

> **Nachtrag gemessen 2026-10-04 (Future 177).** GIC-ASCII: `space.fmi.fi/gic/man_ascii` liefert
> `man1999.zip`…`man2022.zip` + `man202301-09.zip` (200), `man2023.zip`/`man2024.zip` = **404**
> → kein ASCII nach 2023-09 (Stop 2023-10-23); Compiler steht (`fmi_gic_compiler.rs`,
> `phi/sources.φ:16506`). EPA `data.epa.ie/radmon/api/v1/measurements` = **aufsteigendes
> Bulk-API**; Seite 1 trägt `samp_time` 2013-02-20 (+`last_updated` 2015-07-15) — ein
> **Seite-1-Artefakt**, nicht „stale"; die jüngsten Werte liegen am Reihenende (9,45 M Records,
> siehe unten). C9-Infraschall:
> `http://ceein.infp.ro/fdsnws/station/1/query?level=channel&format=text` = 200 (39 Stationen),
> Kanal **BDF** trägt Einheit **Pa** + Sensitivität. Träger: Future-Folge 177.

- Die restlichen Klassen des Re-Runs (`survey-2026-09-14-weberin-quellen-rerun.md`) mit
  derselben Spalte „zweite Linie" fortschreiben (Gravimeter, Seismik, Lightning, HF-Radar …).
- **GIC-Archiv harvesten** (Kalibrier-Linie, nicht `absent`): `space.fmi.fi/gic/?page=gasum_prel`
  + `?page=gasum_final` liefern offen **nur Tages-Plots** (`gasum/{jahr}/manYYYYMMDD.png`,
  `gasum/prel/manYYYYMMDDprel.jpg`); die **Rohserie** ist request-only (`ari.viljanen@fmi.fi`),
  der Proxy `-dX/dt` kommt aus dem offenen IMAGE-Netz (Nurmijärvi). Verdikt Mountain
  (`phi/sources.φ`), Compiler/Manifestation Mycelium — Muster wie Erdbeben-Katalog/Ephemeriden.
- **GIC-ASCII geprüft (gemessen 2026-10-04, F229 + F229-Fortsetzung):** `space.fmi.fi/gic/gicdata/`
  (relativer Verzeichnisindex) + `gasum/{1998…2023}/` + `gasum/prel/` tragen **nur Plots**
  (PNG/JPG) und das alte `cgi-bin/imagecgi/pipegram.cgi`-Formular; `gasum_index.html` sagt es
  selbst: „These plots are only for quick-look purposes …." **Riss aufgelöst (selbst gemessen
  2026-10-04, `archive_search --verdict` + `curl`):** der **separate** Pfad
  `space.fmi.fi/gic/man_ascii/` = **200** trägt die **Rohserie als ASCII-Zips**
  `man1999.zip`…`man2022.zip` + `man202301-09.zip` (Header: „Final data are available here as
  ASCII files", **CC BY 4.0**, Updated 1 Nov 2023); `man2022.zip`/`man202301-09.zip` = 206
  (reachable). F229s „Rohserie request-only" war ein **Suchfehler** (nur `gasum/` geprüft,
  `man_ascii/` nicht). Der Pfad ist genau `fmi_gic_compiler.rs:8 BASE`
  (`https://space.fmi.fi/gic/man_ascii`) und in `phi/sources.φ:16507-16513` als `fmi_gic`
  registriert (`origin https://space.fmi.fi/gic/man_ascii`, Release-Asset
  `space.fmi.fi/fmi_gic.bin`); die Kalibrier-Linie `-dX/dt` (Nurmijärvi) bleibt der Proxy.
- `data.epa.ie`-Aktualität und C9-Pa-Kalibrierung tiefer messen (`--sniff`/Inhalt).
- Kandidaten als `url`/`compiler`-Zeilen: Verdikt ist Mountain (`phi/sources.φ`).

## Nachtrag — restliche Klassen + EPA/C9 (gemessen 2026-10-04, F229-Fortsetzung)

Der Survey deckte die Größen 1–5 (Gamma, Hydrophon, GIC, Infraschall, BGC-Argo).
Hier die zweite unabhängige Linie der restlichen Klassen, mit dem Re-Run-Stand
(`survey-2026-09-14-weberin-quellen-rerun.md`) und den frisch gemessenen Linien:

| Größe (Klasse) | zweite Linie(n) | Beleg |
|---|---|---|
| Boden-Gravimeter (gravity) | **BGI AGrav** `api.sedoo.fr/get-agrav-rest/station/nearto` **200** (853 714 B, JSON, ~1518 Stationen; Punkt-`gravity` in m/s²) ↔ IGETS `isdc.gfz.de/igets-data-base` **200** (162 937 B) | **frisch 2026-10-04** — BGI/GET-OMP Toulouse + BKG/SEDOO, unabhängig von GFZ/IGETS |
| Seismik (seismic) | EarthScope IU **200** (10 345 B) ↔ GEOFON GE **200** (14 820 B) ↔ RaspberryShake AM **200** (3 775 512 B) | rerun 4.1/4.2/4.3 |
| Lightning (lightning) | WWLLN Thunder-Hour **200** (48 685 822 B) ↔ GLM S3 `noaa-goes16.s3.amazonaws.com` **200** (302 468 B) | rerun 6.1; **frisch 2026-10-04** |
| HF-Radar (hf_radar) | IOOS `hfradar.ioos.us/…/allDatasets.json` **200** (19 924 B) ↔ EMODnet `erddap.emodnet-physics.eu/erddap/index.json` **200** (766 B) | rerun 8.1; **frisch 2026-10-04** |
| Neutrino (neutrino) | HAWC 2HWC/3HWC **200** (18 588/51 833 B) ↔ LHAASO 1LHAASO **200** (16 456 B) | rerun 10.1/10.2/10.3 |
| Transients (transient) | Fink/LSST **200** (42 B) ↔ ALeRCE **200** (5 740 B) | rerun 11.2/11.4 |

**EPA-Aktualität (gemessen 2026-10-04, `archive_search --verdict` + `curl` + `jaq`):**
`data.epa.ie/radmon/api/v1/measurements` ist ein **historisches Bulk-API**
(`count 9 453 260`, `page 1`, `per_page 50`, aufsteigend): Seite 1 trägt
`samp_time` **2013-02-20**, `last_updated` **2015-07-15** — kein Live-Feed auf
Seite 1; die jüngsten Werte liegen am Reihenende (9,45 M Records). Als zweite
Linie nur über den jüngsten Filter/letzte Seite brauchbar.

**C9 / CEEIN (gemessen 2026-10-04, flash-Taucher + Session-Verifikation):** **HTTPS**
`ceein.infp.ro/fdsnws/` = **Timeout** (curl 40 s, `archive_search` 60 s); **HTTP**
`http://ceein.infp.ro/fdsnws/station/1/query?level=network` = **200** (849 B),
`level=station` 200 (15 198 B), `level=channel` 200 (402 337 B);
`dataselect … net=C9 sta=PVCI2 cha=BDF` = **200** (MSEED) — die Zeile ist **live über HTTP**.
**Pa belegt** (StationXML): C9 = **Czech microbarograph network** (IAP Prague, Panska Ves),
Kanal `BDF` = `<InputUnits><Name>Pa</Name>`, Sensor `ISGM03` differential microbarometer.
**Riss (nicht geglättet):** der Survey-Eintrag „CEEIN/NIEP Rumänien, C9" verwechselt Host und
Netz — der Host ist rumänisch (`RO_NDC`), aber **C9 ist das tschechische Netz** (IAP Prag);
Rumäniens eigenes Netz ist RO/R1 (R1 `BDF` = `PA`/Pascals, Chapparal-Mikrofon). EIDA-Nodes
führen C9 nicht (204), IRIS-FedCatalog nicht (404). Der Marker „C9: Pa-Kalibrierung nicht
getestet" ist damit **aufgelöst** (Pa dokumentiert).

**Zweite Gravimeter-Linie (gemessen 2026-10-04, flash-Taucher + Session-Verifikation):**
**BGI Absolute Gravity Database (AGrav)** — anonyme REST-API `https://api.sedoo.fr/get-agrav-rest/`
(Station-Liste **200**/853 714 B JSON, ~1518 Stationen; `point/byStationUuid` liefert `gravity`
in **m/s²**, `meterUuid`/`laser`/`operator` — BGI/GET-OMP Toulouse + BKG/SEDOO, **unabhängig
von GFZ/IGETS**). Harvest: `curl -s "https://api.sedoo.fr/get-agrav-rest/station/nearto?longitude=0&latitude=0&radiusKm=20000&institutionUuid=" -o data/api.sedoo.fr/agrav_all_stations.json`.
Verdikt Mountain (`phi/sources.φ`), Compiler/Manifestation Mycelium. GFZ-Seiten
(`icgem.gfz-potsdam.de`, `ggp.gfz-potsdam.de`) bleiben **nicht** unabhängig.
**Chat-UI-Zweitstimme (Z.ai/GLM-5.3, Deep Think Max; `chat.z.ai/c/a0366dde-…`, gemessen
2026-10-04):** bestätigt (a) BGI/AGrav als zweite Linie — mit dem Caveat, dass **BKG an AGrav
und IGETS** beteiligt ist (nicht völlig getrennte Institutionen); (b) C9 = tschechisches Netz
(IAP Prag), verteilt über den rumänischen NIEP-EIDA-Knoten `ceein.infp.ro`/`www.ceein.eu` als
Teil von CEEIN, `BDF` in Pa. Es nennt weitere Kandidaten — **am Baum gemessen, beide tot**
(Claim ohne Messung zählt nicht): `www.ngs.noaa.gov/GRAV/` = **404** (nur Wayback),
`www.usgs.gov/toolsanddata` = **403** (blocked); `www.ceein.eu` = **200** (14 693 B) live.
Damit bleibt **BGI AGrav** die einzige gemessene unabhängige Gravimeter-Zweitlinie.
