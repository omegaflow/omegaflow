<!--
  title: Entscheidungstabelle — die 17 lizenzgesperrten Arme
  class: blatt
  date: 2026-10-19
  sha256: 73ba16fe265d9ce3e5ef7af5c58298947ef660c46d04e05de62d209aa1d24322
  status: live
  see-also: phi/sources.φ tools/harvest/src/bin/ docs/surveys/survey-2026-10-09-redistribution-alternativen.md
-->
# Entscheidungstabelle — die 17 lizenzgesperrten Arme

**Anlass:** Commit `d0de7ec7f` band sieben ungebundene Arme und ließ 17 stehen:
`babamul · demeter · dhm_gauge · erbq · exposome · fugin · hamqsl · icrf3 ·
ieeg · ilrs_npt · lro_trk · modvolc · ogimet · pradan_ch2 · substorm · tns ·
wwlln`. Die Begründung war je Arm „keine gemessene terms-Zeile oder bereits
declined" — keine Lizenz wurde erfunden.

**Was diese Tabelle ist:** eine Entscheidungsvorlage, **kein** Register-Akt. Sie
ändert weder `phi/sources.φ` noch eine Lizenzzeile und trifft keine
Redistributions-Entscheidung. Der Operator wählt je Zeile **A**, **B** oder
**C**; die Linie setzt danach den Register-/CDN-Schritt.

## Frage

Welchen der 17 lizenzgesperrten Arme trägt die Messung weiter — als
aufgenommener Arm (**A**), als abgelehnter Arm mit gemessener offener
Alternative (**B**) oder als local-only Zugang ohne CDN-Manifestation (**C**)?

Die Frage ist je Arm entscheidbar, sobald drei Dinge **gemessen** sind: eine
Produktzeile am HEAD (nicht nur der Arm-Name), eine `terms`-Zeile an der
Quelle, und — für B — eine offene Alternative mit eigener Messung. Fehlt eines
davon, bleibt der Arm `pending` mit dem fehlenden Beleg; aus dieser Tabelle
wird kein `phi`-Block geschrieben und keine Lizenz erfunden. Was der Operator
wählt, ist die Entscheidung; was hier steht, ist die Messlage, auf der sie
ruht.

**Optionen**

| | Bedeutung | Folge |
|---|---|---|
| **A** | aufnehmen | `terms` messen und eintragen; danach `url`/`origin`/`compiler`/`format` + CDN-Akt (Mycelium) |
| **B** | ablehnen | `decline` mit Grund; die gemessene offene Alternative trägt die Messung |
| **C** | local-only | keine CDN-Manifestation; Zugang/Login bleibt lokal, kein Re-Host |

**Legende der Zustands-Spalte:** „kein Block" = kein Registerblock am HEAD
(gemessen 2026-10-19, `grep` in `phi/sources.φ`); „terms ungemessen" = an der
Quelle kein Lizenztext geprüft; „declined" = bereits abgelehnt; „Etalab-2.0" =
gemessen (Operator-Gespräch 2026-10-11).

| # | Arm | Route / Träger | Zustand am HEAD | Lizenz-Befund | A / B / C | Empfehlung |
|---:|---|---|---|---|---|---|
| 1 | `babamul` | `babamul.caltech.edu/api/babamul` (ZTF/LSST-Alert-Broker, Kafka) | kein Block; Konto/Token (`BABAMUL_API_TOKEN`) | terms ungemessen; Alerts sind öffentlich, der Broker-Zugang ist ein Konto | A nur nach terms-URL; sonst C | **C** bis die Broker-Terms gemessen sind; Konto ist kein Lizenzersatz |
| 2 | `demeter` | `regards.cnes.fr` (CDPP/CNES, Order + WAF) | **Block, `terms auth-required`** (off-CDN, 2026-10-19) | **CDPP/SIPAD-Lizenz V1.1: Weitergabe nur mit schriftlicher Zustimmung** (2026-10-20 gemessen, Web + Paper). Die Etalab-2.0-Messung vom 2026-10-11 ist **widerlegt** | C | **C** — Zugang ist kein Recht; kein Re-Host |
| 3 | `dhm_gauge` | `https://dhm.gov.np/` — `/hydrology/river-watch`, `/pages/data`; Preisliste `Hydrological_Data_Rate.pdf`, Zahlung `Payment_procedure_dhm1.pdf` | kein Block | **kostenpflichtig** (Rate list: Daily/Hourly flow 3 USD/Jahr, Discharge measurement 1 USD/Messung; NPR-Preisliste daneben) — **kein Redistributionsrecht**, Kauf ist Zugang, nicht Lizenz | B | **B** — kostenpflichtig ohne Weitergaberecht; die Flut-Messung traegt `openmeteo_flood` (`bc58013db`) |
| 4 | `erbq` | `services7.arcgis.com/.../tohoku_seismicity/FeatureServer/5/query` | JMA-Quake-Block `phi:31512` trägt `terms jp-public-data-license-1.0`; der ArcGIS-Arm selbst hat keinen Block | für den ArcGIS-Arm ungemessen; JMA-Terms nur für die JMA-Zeile | A nur wenn dieselbe Produktzeile belegt ist; sonst C | **C** — den JMA-Block nutzen, den ArcGIS-Zwilling nicht ohne eigene terms |
| 5 | `exposome` | `exposome-explorer.iarc.fr/.../environmental_pollutants.csv.zip` | **declined** (`survey-2026-10-04-exposom-matrix.md:55`: Katalog, keine Konzentration) | kein Redistributionsrecht nötig — der Arm trägt die Messung nicht | B | **B** — bereits declined; kein Neubau |
| 6 | `fugin` | `jvo.nao.ac.jp` (**NRO/FUGIN-CO-Würfel**, 3D-VRAD; nicht „Skymap") | kein Block; `fugin_skymap_compiler` steht | **kein Lizenztext** (2026-10-20 gemessen): JVO bittet um Zitierung, NAOJ-Copyright erlaubt nur die Free-Use-Scope (privat/Bildung/Nachricht), Weitergabe nicht | C | **C** — Zitierbitte ist keine Lizenz; kein Re-Host |
| 7 | `hamqsl` | `hamqsl.com/solarxml.php` | kein Block | keine offene Lizenz gemessen | B + SWPC | **B** — SWPC ist US-PD und trägt dieselben Solar-Indizes |
| 8 | `icrf3` | `datacenter.iers.org/.../icrf3sx.txt` + `icrf3k.txt` | kein Block | IERS-Terms nicht als SPDX gemessen; Produkte öffentlich | A nach terms; sonst C | **A** — IERS ist die autoritative Rahmenquelle; terms-URL nachtragen |
| 9 | `ieeg` | `www.ieeg.org/services` (UPenn, Konto + User Agreement) | kein Block | research-only; keine Redistribution | C | **C** — klinische iEEG-Rohdaten bleiben lokal; kein CDN |
| 10 | `ilrs_npt` | `edc.dgfi.tum.de` (ILRS Normal Points) | kein Block | terms ungemessen; ILRS-Daten sind forschungsoffen | A nach terms; sonst C | **A** — Public-Data-Route, terms-URL nachtragen |
| 11 | `lro_trk` | `imbrium.mit.edu/LRORS/DATA/TRK/` | **Block, `PD-USGov-PDS`** (2026-10-20) | PDS3-Label, Archivkopie beim PDS-Knoten; Compiler geprueft an 400 Dateien (13 830 Samples, 1 548 968 B, Roundtrip parses) | A | **A** — offen, kein Login |
| 12 | `modvolc` | `nrt3.modaps.eosdis.nasa.gov` | kein Block; `modvolc_thermal`-Compiler steht | **keine Lizenz** am HIGP-Host gemessen | B + LANCE/NRT (NASA-PD) | **B** — die LANCE-Route tragen, den lizenzlosen Arm nicht |
| 13 | `ogimet` | `ogimet.com/cgi-bin/gsynres` | kein Block | keine offene Lizenz gemessen | B + Open-Meteo (CC-BY-4.0) / NCEI (PD) | **B** — offene Alternative ist gemessen |
| 14 | `pradan_ch2` | `pradan.issdc.gov.in` (ISSDC-Login) | kein Block; Login-Arm | terms ungemessen; Zugang account-gebunden | C | **C** — kein Re-Host ohne terms; Chandrayaan-2 bleibt lokal |
| 15 | `substorm` | `supermag.jhuapl.edu/lib/services/` | **Block, `terms no-redistribution`** (2026-10-20) | **keyless**: `list=newell` liefert CSV (200, 3 439 B); Redistribution untersagt | C | **C** — Zugang frei, Weitergabe nicht |
| 16 | `tns` | `wis-tns.org` (Konto/UA) | kein Block; `TNS_API_KEY`/`TNS_UA` | terms ungemessen; Weitergabe unklar | C | **C** — öffentliche Objektlisten nur nach terms-Klärung; kein CDN |
| 17 | `wwlln` | `wwlln.net/climate/th_yr/data/WWLLN_th_2025.nc.zip` | **declined commercial** (UW-Copyright) | kein Redistributionsrecht | B + GOES GLM (PD) | **B** — GLM trägt die Blitz-Messung bereits |

## Was der Operator entscheidet

- **A (aufnehmen):** `demeter`, `icrf3`, `ilrs_npt` — die drei mit der klarsten
  offenen/öffentlichen Route; `demeter` ist bereits Etalab-2.0-gemessen.
- **B (ablehnen):** `dhm_gauge`, `exposome`, `hamqsl`, `modvolc`, `ogimet`, `wwlln` — die
  offene Alternative ist gemessen und trägt dieselbe Messung.
- **C (local-only):** `babamul`, `erbq`, `fugin`, `ieeg`, `lro_trk`,
  `pradan_ch2`, `substorm`, `tns` — der Zugang oder die terms fehlen; lokal
  erlaubt, CDN nicht.

Die Empfehlung ist eine Messlage, kein Lizenzspruch. Wo „terms ungemessen" steht,
ist die nächste Hand die Lizenzseite, nicht der CDN-Upload. Kein Arm wird ohne
Operator-Wort von C nach A gehoben; kein `phi`-Block wird aus dieser Tabelle
geschrieben.

## Riss, der bleibt

Die 17 Namen stammen aus dem Commit-Text, nicht aus einer je Arm neu gelesenen
Inventur-Zeile. `erbq` zeigt den Grund: der JMA-Block trägt terms, der ArcGIS-Arm
nicht — ein Name kann zwei Produkte meinen. Vor jedem A ist deshalb die
**Produktzeile** zu belegen, nicht nur der Arm-Name.

## Nachtrag 2026-10-20 — gemessen, nicht mehr empfohlen

**Die einstige A-Liste ist leer.** `demeter`, `icrf3`, `ilrs_npt` waren als A
vorgeschlagen; alle drei sind jetzt **belegt C**:

| Arm | Beleg (Web + Paper, 2026-10-19/20) | Block |
|---|---|---|
| `demeter` | CDPP/SIPAD V1.1: Weitergabe nur mit schriftlicher Zustimmung | `auth-required`, off-CDN |
| `icrf3` | CC0 nur über CDDIS/Earthdata; IERS-Katalog ohne Lizenztext | `auth-required`, off-CDN |
| `ilrs_npt` | EDC-Login, kein Lizenztext; EUROLAS-ToR regeln das Konsortium, nicht die Daten | `auth-required`, off-CDN |

Acht Arme sind am 2026-10-19 als **off-CDN** gebunden (`e10152951`):
`babamul`, `demeter`, `icrf3`, `ieeg`, `ilrs_npt`, `pradan_ch2`, `substorm`,
`tns` — je mit Origin-Endpunkt, `terms auth-required`, gemessener Begründungs-URL,
**keinem** CDN-Asset. Keine Seite trug einen admitierten Lizenz-Token.

## Die vier „freien Alternativen" sind längst im Register

Die Suche nach Ersatz für `modvolc`, `wwlln`, `ogimet`, `hamqsl` braucht **keine
neuen Arme** — die Träger stehen am HEAD (gemessen 2026-10-20):

| Alter Arm | Träger im Register | Blöcke |
|---|---|---:|
| `modvolc` | `firms.modaps.eosdis.nasa.gov` — inkl. **keyless** CSV-Archiv (`phi:14346`, `origin` = `data/active_fire/.../MODIS_C6_1_Global_7d.csv`) | 5 |
| `wwlln` | GOES GLM — `ghrc.nasa.gov` (`phi:10025`) und `noaa-goes18` (`phi:10384`) | 2 |
| `ogimet` | `www.ncei.noaa.gov` (ISD) | 9 |
| `hamqsl` | `services.swpc.noaa.gov` (Solar-Indizes) | 13 |

Das sind **dieselben Messungen in anderen Dateien** — sie tragen die Messung, sie
sind kein Re-Host des alten Arms. Deshalb bleibt es bei B für die vier, ohne
Neubau.

## Was noch offen ist — und zwar nur das

`dhm_gauge`, `exposome`, `fugin`: **keine Route gefunden** (2026-10-19). Das ist
`absent`, nicht „existiert nicht". Nächste Hand: zweiter Pass mit Aliasen und
Datenhalter-Namen. `exposome` bleibt declined (Katalog, keine Konzentration).

## Nachtrag 2026-10-20 (2) — `dhm_gauge`: die Messung ist da, der Ort nicht

Ein zweiter Pass (Zenodo/DataCite, „Nepal river discharge / hydrology streamflow")
fand **keine offene Nepal-Reihe**. Gefunden wurde die Lücke dahinter: das Register
trug **kein Abfluss-Feld**. Der Arm `openmeteo_flood` (`bc58013db`) schließt sie:

- **keyless**, `terms CC-BY-4.0` („API data are offered under Attribution 4.0
  International", Quelle nennt GloFAS ausdrücklich)
- **92 Tageszeilen** gemessen (27.7 N 85.3 E, Narayani-Becken), `m3/s`
- `format openmeteo_flood_axis_value_text`, `advective m3/s` (Kanon)

Das ist **nicht** die DHM-Stationsreihe — es ist dieselbe Größe an einem Punkt,
keyless und redistribuierbar. `dhm_gauge` bleibt **B**: die Stationsdaten sind
**kostenpflichtig** (`Hydrological_Data_Rate.pdf`: 3 USD/Jahr für Daily/Hourly flow,
1 USD je Discharge-Messung; Zahlung nur per Cash/Bank an die Regierung), und der
Kauf ist Zugang, kein Weitergaberecht. Die Messlücke ist trotzdem offen geschlossen.

Das Antragsformular (`Hydrological_Data_Request_Form.pdf`) sagt es selbst:
„I/We hereby declare that the data obtained for the purpose analysis **will not be
used or distributed for any other purposes**." Also selbst nach Zahlung: keine
Weitergabe. Ein schriftliches Nutzungsrecht, das C zu A machen würde, gibt es
nicht — das Formular ist ein Kaufantrag mit Verwendungsbindung.

## Nachtrag 2026-10-20 (3) — `fugin`: die Google-Zusammenfassung hält nicht

Eine Fremdzusammenfassung („JVO Sky") behauptete, die JVO-Daten seien „in der Regel
für Forschungs- und Bildungszwecke frei", es werde „lediglich eine Zitierung
verlangt", und stützte das auf „offene Standards der IVOA". Nachgemessen an den
Seiten (2026-10-20):

| Quelle | Befund (wörtlich) |
|---|---|
| `jvo.nao.ac.jp/portal/nobeyama/fugin.do` | „the following acknowledgment would be appreciated: '[Part of] the data were retrieved from the JVO portal …'" — eine **Bitte um Zitierung**, keine Lizenz |
| `nao.ac.jp/en/terms/` | „some websites and content are subject to different usage rules"; Copyright liegt bei NAOJ oder den jeweiligen Rechteinhabern |
| `nao.ac.jp/en/terms/copyright.html` | Nutzung ohne Genehmigung **nur** „within the scope of free use granted under Japanese copyright law: private use, educational use, news reporting"; darüber hinaus „you must obtain permission from all copyright holders" |

Drei Fehler der Zusammenfassung:

1. **IVOA-Standards sind Interoperabilität, kein Datenrecht.** VOTable/SIA regeln
   Formate und Protokolle, nicht die Weitergabe der Inhalte.
2. **„frei für Forschung und Bildung" ist die japanische Free-Use-Scope** — privat,
   Bildung, Berichterstattung. Ein CDN-Upload ist keine dieser drei Nutzungen.
3. **Zitierbitte ≠ Lizenz.** Eine Acknowledgment-Bitte begründet kein
   Redistributionsrecht; das Copyright bleibt bei NAOJ/NRO.

**Ergebnis:** `fugin` bleibt **C** (off-CDN, kein Block). Kein Lizenz-Token auf
JVO, NRO/FUGIN oder NAOJ. Der Arm ist damit nicht mehr „ungemessen" — er ist
**gemessen ohne Weitergaberecht**.

## Nachtrag 2026-10-20 (4) — der echte DHM-Endpunkt, außerhalb der Repositorien

Der zweite Pass lief nicht über Zenodo/DataCite, sondern über **Common-Crawl-Index,
Wayback-CDX und das Seiten-JavaScript des Datenhalters**. Ergebnis:

| Fund | Befund |
|---|---|
| `https://dhm.gov.np/hydrology/hms-Single/<id>` | **200** je Station (106, 259, 4601, 5082 geprüft); die Seite trägt die Stationsdaten als JSON: `latitude`, `longitude`, `series_id`, `warning_level`, `danger_level`, `elevation` |
| `https://dhm.gov.np/site/getRiverWatchBySeriesId` | **POST** mit CSRF-Token (Formular `SeriesId_Data`), liefert Tabelle + Chart; GET → 404 |
| `https://dhm.gov.np/aggregated-data-series`, `/-stations.json`, `/-stations.csv` | **404** (nur noch im Wayback, Snapshot 2022-06-26) |
| `https://hydrology.gov.np/cm/pause/api/` | **200**, aber nur die React-Hülle; das JS nennt `/api/upload` und `important_links` |

Das ist der Unterschied zwischen „in elf Repositorien nicht gefunden" und „beim
Datenhalter nachgesehen": der Pfad `/hydrology/hms-Single` (ohne `<id>`) ist 404,
mit ID liefert er Daten. Gefunden über den Common-Crawl-Index, nicht über mein
Gedächtnis.

**Die Lizenzlage ändert das nicht:** kein Lizenztext, und die Reihe ist
**Wasserstand**, nicht Abfluss. `dhm_gauge` bleibt **B**.

## Nachtrag 2026-10-20 (5) — was die keyless Web-Engines wirklich liefern

Nachlauf mit **einer** Binary (Release `01:22`, Commit `99f484ef6`):

| Kanal | Ergebnis für „Nepal river discharge data" |
|---|---|
| `--mwmbl` | 1 Treffer (Thames Water EDM-Karte) — **nicht** Nepal |
| `--marginalia` | 0 |
| `--duckduckgo-instant` | 0 |
| `--wiby` | 0 |

Das ist der gemessene Grund, warum die erste Suche repository-scoped war: die
keyless Web-Engines tragen bei Fachbegriffen fast nichts. Die Konto-Engines
(Brave 402, Tavily 432, SearchApi/SerpApi/Consensus/Gemini 429, Google-CSE 400)
wären die breiten — sie sind es nicht.

## Nachtrag 2026-10-20 (6) — Zwillinge: die Signatur suchen, nicht den Namen

Der Suchlauf nach **Datei-Signaturen** (nicht nach Arm-Namen) hat zwei Zwillinge
gefunden:

| Arm | Zwilling | Befund |
|---|---|---|
| `icrf3` | **`deepspace.jpl.nasa.gov/dsndocs/810-005/107/catalog-fixed.txt`** | **200, 637 566 B**, Kopfzeile „`<ICRF3-SX for use with DDOR 2025>`" — dasselbe Produkt `icrf3sx`, als DSN-Dokument 810-005, also eine **NASA-Veröffentlichung** |
| `ilrs_npt` | `ilrs.gsfc.nasa.gov/.../npt/index.html` → CDDIS + EDC | derselbe Datenbestand, aber **dieselbe Wand**: CDDIS verlangt Earthdata-Login, EDC Login |

Der JPL-Zwilling ist der erste echte Fund: `icrf3` war als `auth-required` über
CDDIS gebunden; dieselbe ICRF3-SX-Tabelle liegt als **offenes DSN-Dokument** bei
JPL. Was fehlt, ist kein Recht, sondern ein **Compiler für das
Festspaltenformat** (Spalten 1–15 laut Dateikopf) — der bestehende
`icrf3_compiler` liest das IERS-Format.

Der ILRS-Zwilling bestätigt die Gegenprobe: ein Zwilling ist nur dann eine
Öffnung, wenn er **nicht** dieselbe Zugangswand hat.

## Nachtrag 2026-10-20 (7) — `lro_trk` ist offen

Der Arm ist gebunden: `terms PD-USGov-PDS`, `format lro_trk`, Compiler
`lro_trk_compiler.rs`, `at earth`, Feld `sky_frequency_hz`. Begründung: PDS3-Label,
Archivkopie beim PDS-Knoten — kein Login, keine Konto-Wand.

Gemessen mit dem neuen Deckel `--max-files`:

| Lauf | Dauer | Ergebnis |
|---|---:|---|
| 3 Dateien | 1,9 s | 22 Samples, 2 472 B, Roundtrip parses |
| 400 Dateien | 2 min 2 s | **13 830 Samples** (2009-06-19..21), **1 548 968 B**, Roundtrip parses |

Der Deckel ist Pflicht, nicht Komfort: ohne ihn crawlt der Compiler ~7 400
Tagesordner (≈48 min) und lädt bis zu 100 000 Dateien, **ohne eine Zeile
Fortschritt**. Jetzt bricht der Crawl bei `cap` ab.

**Riss:** der ausgelieferte Asset deckt die ersten 400 Dateien (drei Tage). Die
volle Serie baut CI; bei ~100 000 Dateien wird sie in Shards zerfallen, und der
Block muss dann die vom Compiler gedruckten Shard-Namen tragen.

## Nachtrag 2026-10-20 (8) — `icrf3` ist offen (JPL-Zwilling)

Der Zwilling ist gebaut und gemessen:

| | |
|---|---|
| Quelle | `deepspace.jpl.nasa.gov/dsndocs/810-005/107/catalog-fixed.txt` (200, 637 566 B) |
| Produkt | `<ICRF3-SX for use with DDOR 2025>` — dasselbe `icrf3sx` |
| Parser | `icrf3_compiler --source ddor` (Festspalten, 15 Spalten laut Dateikopf) |
| Ergebnis | **4 063 / 4 063 Zeilen**, 1 104 101 B, 0 übersprungen |
| Stichprobe | `2357-326` → RA 0,0849999°, Dec −32,3503° (00:00:20,4 / −32:21:01,2) ✓ |
| Block | `terms PD-USGov`, `format icrf3`, `at sun` |

Der alte CDDIS-Block bleibt fuer beide Baender (`sx` + `k`); der JPL-Zwilling
traegt **nur SX**, aber ohne Login.

## Nachtrag 2026-10-20 (9) — Zwillinge fuer SARAS, REACH, PRIZM, Baikal

Diese vier sind **nicht** durch neue Arme zu oeffnen; die Messung liegt schon im
Haus:

| Blockiert | Traeger im Register | Beleg |
|---|---|---|
| SARAS, REACH, PRIZM (global 21 cm) | **EDGES** (`loco.lab.asu.edu/download/790` + `792`, `other-attribution`) | 2 Bloecke, live |
| Baikal NT-200 | **ANTARES** (`free-open`, `vo.km3net.de`), **IceCube DR2**, **AMANDA** | 3 Bloecke, live |

Der Fund `galileo.sese.asu.edu/edges-data/` (200) ist **Rohdaten** (145 Tages-
dateien `*.gsh5`, `metadata/freqs[32768]`, `data/data[3,1,1946,32768]`) — nicht
dasselbe wie die zwei `T21`-Bloecke. Das waere ein **neuer Arm** `edges_raw`;
vorher ist zu pruefen, ob `loco.lab.asu.edu` und `galileo.sese.asu.edu` denselben
Bestand tragen (Spiegel) oder verschiedene Produkte (Roh vs. reduziert).
