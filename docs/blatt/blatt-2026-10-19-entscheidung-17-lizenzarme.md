<!--
  title: Entscheidungstabelle — die 17 lizenzgesperrten Arme
  class: blatt
  date: 2026-10-19
  sha256: 0483aa06c1f6cdd992d603fc20f25e1ff2de7bab36e4588a55de5c1aecb18598
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
| 2 | `demeter` | `regards.cnes.fr` (CDPP/CNES, Order + WAF) | kein Block; Order 403/WAF | **Etalab-2.0** (gemessen 2026-10-11) | A, sobald Order frei; bis dahin C | **A** — offene Lizenz steht; der Rest ist Zugang, kein Recht |
| 3 | `dhm_gauge` | `dhm.gov.np` (`/hydrology/hms-Single`, `getRiverWatchBySeriesId_Single`) | kein Block | terms ungemessen | C; für causal-arrow die offene GloFAS-Reihe nehmen | **C** — ohne terms kein CDN; die Flut-Lücke über GloFAS/USGS schließen |
| 4 | `erbq` | `services7.arcgis.com/.../tohoku_seismicity/FeatureServer/5/query` | JMA-Quake-Block `phi:31512` trägt `terms jp-public-data-license-1.0`; der ArcGIS-Arm selbst hat keinen Block | für den ArcGIS-Arm ungemessen; JMA-Terms nur für die JMA-Zeile | A nur wenn dieselbe Produktzeile belegt ist; sonst C | **C** — den JMA-Block nutzen, den ArcGIS-Zwilling nicht ohne eigene terms |
| 5 | `exposome` | `exposome-explorer.iarc.fr/.../environmental_pollutants.csv.zip` | **declined** (`survey-2026-10-04-exposom-matrix.md:55`: Katalog, keine Konzentration) | kein Redistributionsrecht nötig — der Arm trägt die Messung nicht | B | **B** — bereits declined; kein Neubau |
| 6 | `fugin` | `jvo.nao.ac.jp` (Skymap) | kein Block | terms ungemessen | C | **C** — ohne terms keine Manifestation; JVO-Zugang bleibt lokal |
| 7 | `hamqsl` | `hamqsl.com/solarxml.php` | kein Block | keine offene Lizenz gemessen | B + SWPC | **B** — SWPC ist US-PD und trägt dieselben Solar-Indizes |
| 8 | `icrf3` | `datacenter.iers.org/.../icrf3sx.txt` + `icrf3k.txt` | kein Block | IERS-Terms nicht als SPDX gemessen; Produkte öffentlich | A nach terms; sonst C | **A** — IERS ist die autoritative Rahmenquelle; terms-URL nachtragen |
| 9 | `ieeg` | `www.ieeg.org/services` (UPenn, Konto + User Agreement) | kein Block | research-only; keine Redistribution | C | **C** — klinische iEEG-Rohdaten bleiben lokal; kein CDN |
| 10 | `ilrs_npt` | `edc.dgfi.tum.de` (ILRS Normal Points) | kein Block | terms ungemessen; ILRS-Daten sind forschungsoffen | A nach terms; sonst C | **A** — Public-Data-Route, terms-URL nachtragen |
| 11 | `lro_trk` | `imbrium.mit.edu/LRORS/DATA/TRK/` | kein Block | terms ungemessen | A nach terms; sonst C | **C** bis terms; der LRO-Tracking-Pfad ist nicht CDN-kritisch |
| 12 | `modvolc` | `nrt3.modaps.eosdis.nasa.gov` | kein Block; `modvolc_thermal`-Compiler steht | **keine Lizenz** am HIGP-Host gemessen | B + LANCE/NRT (NASA-PD) | **B** — die LANCE-Route tragen, den lizenzlosen Arm nicht |
| 13 | `ogimet` | `ogimet.com/cgi-bin/gsynres` | kein Block | keine offene Lizenz gemessen | B + Open-Meteo (CC-BY-4.0) / NCEI (PD) | **B** — offene Alternative ist gemessen |
| 14 | `pradan_ch2` | `pradan.issdc.gov.in` (ISSDC-Login) | kein Block; Login-Arm | terms ungemessen; Zugang account-gebunden | C | **C** — kein Re-Host ohne terms; Chandrayaan-2 bleibt lokal |
| 15 | `substorm` | `supermag.jhuapl.edu/lib/services/` | kein Block; SuperMAG-Konto existiert | Terms ungemessen; Account-Zugang | C | **C** — Account ist kein Redistributionsrecht |
| 16 | `tns` | `wis-tns.org` (Konto/UA) | kein Block; `TNS_API_KEY`/`TNS_UA` | terms ungemessen; Weitergabe unklar | C | **C** — öffentliche Objektlisten nur nach terms-Klärung; kein CDN |
| 17 | `wwlln` | `wwlln.net/climate/th_yr/data/WWLLN_th_2025.nc.zip` | **declined commercial** (UW-Copyright) | kein Redistributionsrecht | B + GOES GLM (PD) | **B** — GLM trägt die Blitz-Messung bereits |

## Was der Operator entscheidet

- **A (aufnehmen):** `demeter`, `icrf3`, `ilrs_npt` — die drei mit der klarsten
  offenen/öffentlichen Route; `demeter` ist bereits Etalab-2.0-gemessen.
- **B (ablehnen):** `exposome`, `hamqsl`, `modvolc`, `ogimet`, `wwlln` — die
  offene Alternative ist gemessen und trägt dieselbe Messung.
- **C (local-only):** `babamul`, `dhm_gauge`, `erbq`, `fugin`, `ieeg`, `lro_trk`,
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
