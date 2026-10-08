<!--
  title: Survey — Gegen-Audit: Quellen-Delta zum Open-Sources-Audit (2026-10-08)
  class: survey
  date: 2026-10-08
  sha256: 5380102eaf5e103330836ebf94c7f1d44d7c8f8aeca7003dc0e64141d204b5ba
  status: live
  see-also: state/future/open-sources-audit-2026-10-08.md docs/surveys/survey-2026-10-08-fmhy-research-landscape.md docs/SOURCE_PORT.md
-->
# Survey — Gegen-Audit: Quellen-Delta

Auftrag (Operator-Wort 2026-10-08): prüfen, ob **andere** open/keyless Quellen findbar sind als
die im Future-Audit (`state/future/open-sources-audit-2026-10-08.md`, sha `ff17a1f5…`). Ein
unabhängiger `general`-Diver (volle Kaskade) + eigene SkyView-Probes. Alle HTTP via
`curl -sS -o /dev/null -w '%{http_code}' --max-time 18`.

**Verifikation des Audits am Baum** (`sgrep -c … phi/sources.φ`): HI4PI · BK18 · `gll_psc_v32` ·
GEBCO · `occupation_data` · `opendata.cern` · Zenodo XENONnT/LZ · `super-k` · `DVN/MMIIZA` ·
`hepdata` · `FRBSTATS` = **je 0** (unregistriert bestätigt); `FRBCAT`=2 · `cosmicflows_cf4`=1 ·
`bp_rp`=2 (die drei Risse bestätigt). **Das Audit trägt.**

## Delta — Quellen, die das Audit NICHT nannte

### I · Galaktisches HI 21cm (ergänzt HI4PI)
| Quelle | URL | HTTP | neu |
|---|---|---|---|
| LAB (Kalberla 2005) | `vizier.cds.unistra.fr/viz-bin/VizieR?-source=J/A+A/440/775` | 200 | ja |
| EBHIS DR1 (Winkel 2016) | `…?-source=J/A+A/585/A136` | 200 | ja |
| GASS (McClure-Griffiths 2009) | `…?-source=J/ApJS/181/398` | 200 | ja |
| GALFA-HI DR2 (Peek 2018) | `…?-source=J/ApJS/234/2` | 200 | ja |
| SkyView `Survey=LAB`/`EBHIS`/`GASS` | `skyview.gsfc.nasa.gov/current/cgi/runquery.pl?Survey=…` | je 200 | ja |
| HEASARC W3Browse `hi4pi` | `heasarc.gsfc.nasa.gov/db-perl/W3Browse/w3table.pl?tablehead=name%3Dhi4pi` | 200 | ja (anderer Host) |

### XII · CMB / B-Moden (ergänzt BK18)
| Quelle | URL | HTTP | neu |
|---|---|---|---|
| Planck Legacy Archive | `pla.esac.esa.int/pla/` | 200 | ja |
| NASA LAMBDA (ACT/SPT/Planck/WMAP) | `lambda.gsfc.nasa.gov/product/{act,spt,planck,wmap}/` | 200 | ja |
| ACT (NERSC) | `portal.nersc.gov/project/act/` | 200 | ja |
| SPT-3G public data | `pole.uchicago.edu/public/data/` | 200 | ja |
| Simons Observatory / CMB-S4 | `simonsobservatory.org` / `cmb-s4.org` | 200 | ja |

### III · Koronale Heizung (neue Kanäle)
IRIS `iris.lmsal.com` · SDO/AIA `sdo.gsfc.nasa.gov` · VSO `vso.stanford.edu` · HEK `lmsal.com/hek`
· Solar Orbiter `spdf.gsfc.nasa.gov/pub/data/solar-orbiter/` · SPICE `spice.ias.u-psud.fr` · EUI
`sidc.be/EUI/data/` · PSP `spdf.gsfc.nasa.gov/pub/data/psp/` · Hinode (`sot.lmsal.com`,
`hinode.nao.ac.jp`, `solarb.msfc.nasa.gov`) · SOHO · NOAA SWPC `services.swpc.noaa.gov` · CDAW —
**alle 200, keyless, neu** (Audit nannte nur den `ssd.jpl`↔`ncei`/`cdaweb`-Netloc-Riss).

### IV · LAIC / CSES (der unbestimmte Kanal)
CSSDC `cssdc.ac.cn/en` · NSSDC China · SEPC `sepc.ac.cn` · SpaceWeather China · INTERMAGNET ·
ISGI `isgi.unistra.fr` · NOAA NCEI geomagnetic · Swarm `swarm-diss.eo.esa.int` · GIRO/DIDBase ·
OMNIWeb — **alle 200, keyless, neu** (`www.cses.ac.cn` SSL abgelaufen → `pending`).

### IX · FRB (ergänzt FRBCAT/FRBSTATS)
CHIME/FRB Catalog `chime-frb.ca/catalog` (200) · Blinkverse `blinkverse.zerooffset.xyz` (200, HTTP;
TLS invalide) · VOEvent `voeventnet.org` (200) · ATel (200) — **neu**. TNS bleibt 403 (Login/Key).

### V · Teilchen-/Dark-Matter-Open-Data (ergänzt CERN/XENONnT/LZ/Super-K/IceCube/PDG/HEPData)
ATLAS Open Data `opendata.atlas.cern` (200) · Belle II `docs.belle2.org/pub_data` (200) · Fermilab
MINERvA `minerva.fnal.gov/opendata` · MicroBooNE public datasets · NOvA · Muon g−2 ·
`lss.fnal.gov/lists/fermilab-reports-data.html` · DUNE docs · JLab Hall A · CMS open-data-guide ·
**GWOSC** `gwosc.org` · NNDC ENSDF — **alle 200, keyless, neu**. `opendata.fnal.gov` = NXDOMAIN
(per-Experiment-Seiten nutzen).

## Wertvollste neue Kanäle (nächster Schritt)
1. **VizieR-Arm** für HI (LAB/EBHIS/GASS/GALFA-HI; ein Host, vier Surveys) + SkyView-`Survey=`-Modus um EBHIS/GASS/LAB erweitern.
2. **Planck Legacy Archive** als CMB-Primärquelle (LAMBDA als Spiegel: ACT/SPT/WMAP).
3. **CHIME/FRB Catalog** als offene FRB-Quelle (kein TNS-Key).
4. **Solar-Erweiterung** (IRIS/SDO/VSO/HEK/Solar Orbiter/PSP) für die koronale-Heizung-Nuss.
5. **LAIC/CSES-Erweiterung** (CSSDC/NSSDC China, INTERMAGNET, ISGI, GIARD/DIDBase, OMNIWeb).
6. **Teilchen**: ATLAS Open Data · Belle II · GWOSC · Fermilab-Per-Experiment-Seiten.

## Riss (Audit-Rückfrage)
Die Audit-Rückfrage („welches Teilchenexperiment — Kuprat ist Festkörper, kein Teilchen-Experiment")
ist **operator-gebunden**: nur der Operator weiß, ob der Kuprat gemeint ist oder ein
Detektor-/Dark-Matter-Experiment. Bis dahin bleiben die Teilchen-Kanäle `pending` (Fundstücke,
keine Admission).

## Re-Audit — stale · angefragt · gescheitert (2026-10-08)

Mit den **neuen Armen** gemessen (`--jina`, `--perplexity`, `--consensus` endlich genutzt +
`--verdict`), gegen `state/zustand/external-state.md` und `phi/blocked_sources.φ`:

| Quelle | alt | neu (HTTP / Werkzeug) | Verdikt |
|---|---|---|---|
| SuperMAG `services/data-api.php` | stale, fällig 10-02 (PHP-Defekt) | **206**; `--jina`-Body `ERROR: No username` | **erreichbar** — braucht jetzt einen **`username`-Param** (neuer Fund) |
| BepiColombo PSA | pending/Anfrage, 10-02 | **200** (1562 B) | erreichbar; Produkt `release_date 2099` gated (MORE ab Science-Phase, ~April) |
| ESA LPF | stale 10-02 | **500** | still-blocked |
| Lasair `api/` | stale | proton **404** | still-blocked (Backend) |
| SSDC Limadou | Anfrage | **200** (60 809 B) | erreichbar; Zugang pending (CSES-02-Umbau, Sotgiu 2026-09-16) |
| DEMETER `rs-order` | pending, Order exp 10-05 | **403** (WAF) | still-blocked; `--perplexity` fand die ICE-Dataset-Route |
| GOSAT-GW Host | pending | Host tot; CDN-Asset `--sniff` **200** | Host tot, Asset ok |
| Chang'e `moon.bao.ac.cn` | pending/Konto | **206** | erreichbar; Portal-Ernte (Operator-Konto) |
| **CSES LEOS `www.leos.ac.cn`** | descoped (Captcha) | **206**; `--jina` rendert die App (`#/dataService/dataDownloadList`) | **NEU — via `--perplexity` gefunden**, live/user-gated, **unregistriert** |
| NSSDC (Voyager/Mariner/Viking/Juno) | offene Anfragen | keine Antwort (`mail_ledger`); Follow-up 2026-09-30 | Anfrage offen |
| NED bulk redshift | Anfrage | ByParams + Timeout-Token (2026-10-01) | resolved über Alternative |

**Riss (Rohr):** der Diver zitierte CSES als `phi/sources.φ:17149-17192` — der Baum trägt dort
**Vega 2 MISCHA** (`sgrep 'leos.ac.cn' phi/sources.φ` = **0**). Die Register-Zeilennummer driftet;
**der Baum gewinnt** → LEOS/CSES ist **nicht** registriert, der Fund bleibt `pending`
(Mountain-Admission). Fundort bleibt die Behauptung, die Messung ist der Baum.
