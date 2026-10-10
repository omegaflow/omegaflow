<!--
  title: Survey — Gegen-Audit: Quellen-Delta zum Open-Sources-Audit (2026-10-08)
  class: survey
  date: 2026-10-08
  sha256: 6277dd6a2d7c751499b3648fbd42e1805e447b8d82b9482be0add4ccd1e1c941
  status: live
  see-also: state/future/open-sources-audit-2026-10-08.md docs/concepts/tools-map.md docs/SOURCE_PORT.md
-->
# Survey — Gegen-Audit: Quellen-Delta

Auftrag (Operator-Wort 2026-10-08): prüfen, ob **andere** open/keyless Quellen findbar sind als
die im Future-Audit (`state/future/open-sources-audit-2026-10-08.md`, sha `ff17a1f5…`). Ein
unabhängiger `general`-Diver (volle Kaskade) + eigene SkyView-Probes. **Vollständig, keine
Top-N-Kürzung** (Operator-Wort 2026-10-08). Alle HTTP via `curl … -w '%{http_code}'`.

**Verifikation des Audits am Baum** (`sgrep -c … phi/sources.φ`): HI4PI · BK18 · `gll_psc_v32` ·
GEBCO · `occupation_data` · `opendata.cern` · Zenodo XENONnT/LZ · `super-k` · `DVN/MMIIZA` ·
`hepdata` · `FRBSTATS` = **je 0** (unregistriert bestätigt); `FRBCAT`=2 · `cosmicflows_cf4`=1 ·
`bp_rp`=2 (die drei Risse bestätigt). **Das Audit trägt.**

## Delta — vollständig (Quellen, die das Audit NICHT nannte)

### I · Galaktisches HI 21cm
| Quelle | URL | HTTP |
|---|---|---|
| LAB (Kalberla 2005) | `vizier.cds.unistra.fr/viz-bin/VizieR?-source=J/A+A/440/775` | 200 |
| EBHIS DR1 (Winkel 2016) | `…?-source=J/A+A/585/A136` | 200 |
| GASS (McClure-Griffiths 2009) | `…?-source=J/ApJS/181/398` | 200 |
| GALFA-HI DR2 (Peek 2018) | `…?-source=J/ApJS/234/2` | 200 |
| SkyView `Survey=EBHIS` | `skyview.gsfc.nasa.gov/current/cgi/runquery.pl?Survey=EBHIS&position=0,0&Size=1` | 200 |
| SkyView `Survey=LAB`/`GASS` | ebenso `Survey=LAB`/`GASS` | je 200 |
| SkyView Survey-Liste | `skyview.gsfc.nasa.gov/current/cgi/survey.pl` | 302/200 |
| HEASARC W3Browse `hi4pi` | `heasarc.gsfc.nasa.gov/db-perl/W3Browse/w3table.pl?tablehead=name%3Dhi4pi` | 200 |
| CDSARC Katalogseiten | `cdsarc.cds.unistra.fr/viz-bin/cat/J/A+A/440/775` | 200 |
| GALFA-HI Dataverse | `dataverse.harvard.edu/dataverse/GALFA-HI` | 202 |

### XII · CMB / B-Moden
| Quelle | URL | HTTP |
|---|---|---|
| Planck Legacy Archive | `pla.esac.esa.int/pla/` | 200 |
| NASA LAMBDA | `lambda.gsfc.nasa.gov/` | 200 |
| LAMBDA ACT | `lambda.gsfc.nasa.gov/product/act/` | 200 |
| LAMBDA SPT | `lambda.gsfc.nasa.gov/product/spt/` | 200 |
| LAMBDA Planck | `lambda.gsfc.nasa.gov/product/planck/` | 200 |
| LAMBDA WMAP | `lambda.gsfc.nasa.gov/product/wmap/current/` | 200 |
| ACT (NERSC) | `portal.nersc.gov/project/act/` | 200 |
| SPT-3G public data | `pole.uchicago.edu/public/data/` | 200 |
| Simons Observatory | `simonsobservatory.org/` | 200 (noch keine Daten) |
| CMB-S4 | `cmb-s4.org/` | 200 |
| ACT Hauptseite | `act.princeton.edu/` | 403 Cloudflare |
| LiteBIRD | `litebird.jp` | TLS-Kette defekt → pending |

### III · Koronale Heizung / Solar
| Quelle | URL | HTTP |
|---|---|---|
| IRIS | `iris.lmsal.com/data.html` | 200 |
| SDO/AIA | `sdo.gsfc.nasa.gov/data/aiahmi/` | 200 |
| Virtual Solar Observatory | `vso.stanford.edu/` | 200 |
| HEK | `www.lmsal.com/hek/` | 200 |
| Solar Orbiter (SPDF) | `spdf.gsfc.nasa.gov/pub/data/solar-orbiter/` | 200 |
| SPICE | `spice.ias.u-psud.fr/` | 200 |
| Solar Orbiter EUI (SIDC) | `www.sidc.be/EUI/data/` | 200 |
| Parker Solar Probe (SPDF) | `spdf.gsfc.nasa.gov/pub/data/psp/` | 200 |
| Hinode SOT | `sot.lmsal.com/` | 200 |
| Hinode (NAO/JAXA) | `hinode.nao.ac.jp/en/` | 200 |
| Hinode (NASA MSFC) | `solarb.msfc.nasa.gov/` | 200 |
| SOHO | `soho.nascom.nasa.gov/` | 200 |
| NOAA SWPC (JSON) | `services.swpc.noaa.gov/` | 200 |
| CDAW CME-Katalog | `cdaw.gsfc.nasa.gov/` | 200 |
| JSOC | `jsoc1.stanford.edu/login` | 200 (Login) |

### IV · LAIC / CSES
| Quelle | URL | HTTP |
|---|---|---|
| **LEOS (CSES-Portal, neu via `--perplexity`)** | `www.leos.ac.cn/#/home` | 206 (user-gated) |
| CSSDC (China Space Science Data Center) | `www.cssdc.ac.cn/en/` | 200 |
| NSSDC China | `www.nssdc.ac.cn/nssdc_en/html/index.html` | 200 |
| SEPC | `www.sepc.ac.cn/` | 200 |
| SpaceWeather China | `www.spaceweather.ac.cn/` | 200 |
| INTERMAGNET | `intermagnet.org/` | 200 |
| ISGI | `isgi.unistra.fr/` | 200 |
| NOAA NCEI geomagnetic | `www.ncei.noaa.gov/products/geomagnetic-data` | 200 |
| ESA Swarm dissemination | `swarm-diss.eo.esa.int/` | 200 |
| GIRO / DIDBase | `ulcar.uml.edu/DIDBase/` | 200 |
| OMNIWeb | `omniweb.gsfc.nasa.gov/` | 200 |
| CSES official | `www.cses.ac.cn` | SSL abgelaufen (pending) |
| DEMETER (CNES info) | `cnes.fr/projets/demeter` | 200 (Daten order/login) |

### IX · FRB
| Quelle | URL | HTTP |
|---|---|---|
| CHIME/FRB Catalog | `www.chime-frb.ca/catalog` | 200 |
| Blinkverse | `blinkverse.zerooffset.xyz/` | 200 (HTTP; TLS invalide) |
| VOEvent | `voeventnet.org/` | 200 |
| ATel | `www.astronomerstelegram.org/` | 200 |
| CHIME/FRB GitHub | `github.com/CHIMEFRB` | 200 |
| TNS | `www.wis-tns.org/` | 403 (Cloudflare/Login) |

### V · Teilchen-/Dark-Matter-Open-Data
| Quelle | URL | HTTP |
|---|---|---|
| ATLAS Open Data | `opendata.atlas.cern/` | 200 |
| Belle II public data | `docs.belle2.org/pub_data/documents/` | 200 |
| Fermilab MINERvA | `minerva.fnal.gov/opendata/` | 200 |
| MicroBooNE public datasets | `microboone.fnal.gov/documents-publications/public-datasets/` | 200 |
| NOvA | `novaexperiment.fnal.gov/` | 200 |
| Muon g−2 | `muon-g-2.fnal.gov/` | 200 |
| Fermilab DATA-Liste | `lss.fnal.gov/lists/fermilab-reports-data.html` | 200 |
| DUNE docs | `docs.dunescience.org/` | 200 |
| JLab Hall A | `hallaweb.jlab.org/` | 200 |
| CMS open data (CERN-Portal) | `opendata.cern.ch/search?f=experiment:CMS&q=` | 200 |
| CMS open-data-guide | `cms-opendata-guide.web.cern.ch/` | 200 |
| GWOSC (LIGO/Virgo) | `gwosc.org/` | 200 |
| NNDC ENSDF | `www.nndc.bnl.gov/ensdf/` | 200 |
| AMSC | `amsc.energy.gov/` | 200 |
| Fermilab Open-Data-Portal | `opendata.fnal.gov` | NXDOMAIN (per-Experiment nutzen) |

## Re-Audit — stale · angefragt · gescheitert (2026-10-08)

Mit den **neuen Armen** gemessen (`--jina`, `--perplexity`, `--consensus` + `--verdict`),
gegen `state/zustand/external-state.md` und `phi/blocked_sources.φ`:

| Quelle | alt | neu (HTTP / Werkzeug) | Verdikt |
|---|---|---|---|
| SuperMAG `services/data-api.php` | stale, fällig 10-02 (PHP-Defekt) | **206**; `--jina`-Body `ERROR: No username` | **erreichbar** — braucht jetzt **`username`-Param** |
| BepiColombo PSA | pending/Anfrage, 10-02 | **200** (1562 B) | erreichbar; Produkt gated (MORE ab Science-Phase ~April) |
| ESA LPF | stale 10-02 | **500** | still-blocked |
| Lasair `api/` | stale | proton **404** | still-blocked (Backend) |
| SSDC Limadou | Anfrage | **200** (60 809 B) | erreichbar; Zugang pending (CSES-02-Umbau) |
| DEMETER `rs-order` | pending, Order exp 10-05 | **403** (WAF) | still-blocked |
| GOSAT-GW Host | pending | Host tot; CDN-Asset `--sniff` **200** | Host tot, Asset ok |
| Chang'e `moon.bao.ac.cn` | pending/Konto | **206** | erreichbar; Portal-Ernte |
| **LEOS `www.leos.ac.cn`** | descoped (Captcha) | **206**; `--jina` rendert App | **NEU**, unregistriert |
| NSSDC (Voyager/Mariner/Viking/Juno) | offene Anfragen | keine Antwort; Follow-up 2026-09-30 | Anfrage offen |
| NED bulk redshift | Anfrage | ByParams + Token (2026-10-01) | resolved (Alternative) |

## Riss (benannt)
Der Diver zitierte CSES als `phi/sources.φ:17149-17192` — der Baum trägt dort **Vega 2 MISCHA**
(`sgrep 'leos.ac.cn' phi/sources.φ` = **0**). Register-Zeile driftet; **der Baum gewinnt** →
LEOS/CSES ist **nicht** registriert, der Fund bleibt `pending` (Mountain-Admission).

**Gegenprobe 2026-10-11:** `sgrep -i leos phi` trifft `declined_sources.φ:5096` — LEOS ist
declined (Lizenz NINH-MEM verbietet die Weitergabe); die CSES-Messung lebt als
`sources.φ:19529` (`scidb.cn/cses_efd` u. a.). Riss aufgelöst.

## Offen (operator-gebunden)
Die Audit-Rückfrage „welches Teilchenexperiment": **Kuprat** (Festkörper, kein Teilchen-Experiment)
oder ein **Detektor-/Dark-Matter-Experiment**. Bis dahin bleiben die Teilchen-Kanäle `pending`.

## Blockierte vier — harte Runde (Ergebnis 2026-10-08)

Zwei `general`-Taucher, volle Kaskade inkl. **Proton** (`socks5h://127.0.0.1:25344`, Exit `149.88.103.50`),
`--jina`/`--perplexity`/`--consensus`/`--playwright`/`--sniff`. **Ergebnis: keine der vier war geo-
oder Cloudflare-blockiert — es waren falsche Hosts/Pfade bzw. fehlende Parameter/Tokens.**

### ESA LPF — **GEKNACKT (offenes NASA/HEASARC-Mirror)**
- Der `data-action`-500 ist **kein Block**, sondern eine **malformte Anfrage** (`missing ProductType`);
  `lpf.esac.esa.int/lpfsa/` = **200** (GWT-Client lebt), `…/lpfsa-sl/rss-action` = 200 JSON.
- **Offenes Mirror:** `https://heasarc.gsfc.nasa.gov/lpf/cgi/selector?start=<MJD>&end=<MJD>&hdu=SCI_SCIENCE_1Hz`
  → **200 `application/fits`**; FTP-Listing `heasarc.gsfc.nasa.gov/FTP/lpf/data/fits/` (drs_*.fits, 42 K–268 M).
- ESDC-DOI-Metadaten: `esdcdoi.esac.esa.int/…/lisa-pathfinder/*.html` je 200; `doi.org/10.5270/esa-fc52vb6`.
- Kein Geo/CF (`.int` ohne Free-Exit); Root `lpf.esac.esa.int/` 403 = serverseitige Pfad-ACL.

### Lasair — **GEKNACKT (Route), Token-gated**
- `api.lasair.lsst.ac.uk/` über Proton = **200** (Portal); `/api/object/`, `/api/cone/`, `/api/query/`,
  `/api/sherlock/{position,object}/` = **401 (58 B) — am Leben, nur Token**. Singular `object` (nicht `objects`).
- **Exakte Endpunkte:** `https://api.lasair.lsst.ac.uk/api/cone/?ra=&dec=&radius=&requestType=all&token=<TOK>&format=json`
  (analog `query`, `object`, `sherlock`); Auth `?token=<TOK>` **oder** Header `Authorization: Token <TOK>`.
  Doku `lasair-lsst.readthedocs.io/en/main/core_functions/rest-api.html` (200).
- `LASAIR_LSST_TOKEN` liegt vor (nie gedruckt); die authentifizierte 200 ist aus Secret-Hygiene/Probe-Grund nicht gezeigt.

### DEMETER (CNES/CDPP REGARDS) — **GEKNACKT bis zur Auth-Kante**
- Der Block wanderte **WAF-403 → auth-401**: `…/orders/public/files/9162386?orderToken=<JWT>&scope=cdpp` = **401**
  (direct + Proton); mit `?scope=cdpp` allein = 400; bar = 403. Backend antwortet.
- **Exakter Endpunkt:** `https://regards.cnes.fr/api/v1/rs-order/orders/public/files/<fileId>?orderToken=<JWT>&scope=cdpp`
  (erste Datei `9162386`, 576 512 B; 97 078 URLs im Metalink). Spiegel auf `cdpp-archive.cnes.fr` ebenso.
- **Ursache:** die lokalen Metalink-**JWTs sind abgelaufen** (`order_18400` exp **2026-10-05**). Kein WAF-, kein Geo-Block.
- **Nächster Schritt (Operator, REGARDS-Konto):** neu einloggen → frisches Metalink/Token → dieselbe URL liefert 200.

### GOSAT-GW (NIES) — **GEKNACKT bis zur Auth-Kante**
- Der Eintrag zeigte auf den **falschen Host**. Der echte Daten-Port: `https://product.gosat-gw.nies.go.jp/en/home/`
  = **200** (TANSO-3 Product Archive); Projekt `gosat-gw.nies.go.jp/en/` = 200.
- **API:** `…/product_search/api/cui-search/` = **401** `User authentication failed`; `cui-download/` = 401;
  Login `POST …/en/loginpage/api/login` (Felder `mail_address`, `password`, CSRF). Cookie-gated.
- **Nächster Schritt (Operator-Konto):** Login → Cookie; dann `cui-search`/`cui-download`. Kein Host-Block.

**Fazit:** alle vier sind **kein** toter Kanal — LPF ist offen (HEASARC), Lasair/DEMETER/GOSAT-GW
sind auth-/token-gated (Route offen). Je ein Operator-Akt (frisches Token/Cookie) bzw. eine
Source-Registrierung (LPF-HEASARC) schließt sie.

## Baum-Gegenprobe (2026-10-11)

Die offenen Punkte des Blatts je mit `sgrep` gegen `phi/*.φ` gemessen (Re-Audit-Tabelle,
Blockierte vier, Riss/Offen): **13 Punkte, 11 gedeckt, 2 ungedeckt.** Der Zensus nannte 18
Marker-Zeilen (2026-10-10); gemessen heute sind es 15 (Zeilen 51, 86, 126–132, 134, 142,
174, 181, 183, 184).

| Punkt | Verdikt | Register (2026-10-11) |
|---|---|---|
| SuperMAG `services/data-api.php` | gedeckt | `declined_sources.φ:4372`; `sources.φ:19865` |
| BepiColombo PSA | gedeckt | `sources.φ:4421`; `blocked_sources.φ:26` |
| ESA LPF | gedeckt | `sources.φ:11329` (HEASARC `drs_fits`) |
| Lasair `api/` | gedeckt | `declined_sources.φ:2692`; `dead_sources.φ:147` |
| SSDC Limadou | **ungedeckt** | nur `phi/pipeline/ledger.φ:15` (TAPSSDC 0 CSES/LIMADOU) |
| DEMETER `rs-order` | gedeckt | `declined_sources.φ:3277` |
| GOSAT-GW | gedeckt | `sources.φ:19368` |
| Chang'e `moon.bao.ac.cn` | gedeckt | `sources.φ:10957` (`clpds.bao.ac.cn`) |
| LEOS/CSES | gedeckt | `declined_sources.φ:5096` (Lizenz NINH-MEM) |
| NSSDC Voyager/Mariner/Viking/Juno | gedeckt | `blocked_sources.φ:30–44`; `sources.φ:19592,19507,11761` |
| NED bulk redshift | gedeckt | `declined_sources.φ:2840` |
| LiteBIRD (`litebird.jp`) | **ungedeckt** | keine der vier Register |
| Teilchen-/Dark-Matter-Experiment | gedeckt | `blocked_sources.φ:58` (opendata.cern pending) |

**Ungedeckte Reste:** SSDC Limadou (nur `ledger.φ:15` Kandidat), LiteBIRD (nur im Blatt).
Die Delta-Tabellen (I–V, IX, XII) sind Fund-Landschaft und wurden nicht je Zeile
gegenregistriert; die im Blattkopf als unregistriert bestätigten (HI4PI, BK18, `gll_psc_v32`,
GEBCO, `occupation_data`, `opendata.cern`, Zenodo XENONnT/LZ, `super-k`, `DVN/MMIIZA`,
`hepdata`, `FRBSTATS`) bleiben unregistriert.
