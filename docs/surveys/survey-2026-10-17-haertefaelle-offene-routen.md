<!--
  title: Survey — Härtefälle: offene Routen (Sonden · Redistributions-Declines · Gated China · C/NOFS-NWM · MODVOLC)
  class: survey
  date: 2026-10-17
  sha256: 8b83ac72a7d9023e253fdf802fe8303c755e397980fe89599984e547da755393
  status: live
  see-also: docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md docs/surveys/survey-2026-10-09-redistribution-alternativen.md docs/handover/handover-2026-10-14-sensory-folge259.md
-->
# Survey — Härtefälle: offene Routen (2026-10-17)

**Anlass (Operator-Wort):** die wirklich schweren Quellen — die mit **laufenden Anfragen** oder
**Absagen**. Drei `taucher` + zwei eigene Messungen (2026-10-17); Werkzeuge `bin/where`,
`archive_search` (`--nssdc`, `--heasarc`, `--openaire`, `--zenodo`, `--verdict` …) und HTTP per
`curl -w`. **Keine Architektur** — die Zulassung ist Mountains Verdikt, CDN/Route Myceliums.

## A — Sonden-Rohdaten (request-only)

| Sonde | offene Route | HTTP | keyless | bleibt |
|---|---|---|---|---|
| **Cassini** closed-loop (TNF/ODF) | `atmos.nmsu.edu/pdsd/archive/data/co-s-rss-1-*/…/odf/` (PDS3, DSN-NAV) | 200 | ja | **offen** — im Haus schon als `cassini_tnf.bin` geerntet |
| **Mariner 10** (762 7-Track-Tapes) | nur Teilroute: `spdf.gsfc.nasa.gov/pub/data/mariner/mariner10/…/red_tele_signal_data_venus_occlt/PSPA-00316_DD029604_05-FEB-74.tar` (19,6 MB) | 200 | ja | Roh-Tapes (`PSCM-00009`) request-only |
| **Voyager 1/2** closed-loop | keine Route (SPDF nur Okkultation/ODR; NSSDC `PSNO-00007` „archive, not distribution") | – | – | request-only |
| **Viking 1/2** Doppler+Range | keine Route (`PSPG-00011` „Data Identified but not Received") | – | – | request-only/absent |
| **Juno** pre-EFB 2013-10-09 | keine Route (Juno-ODF erst post-EFB; EFB = 2013-282) | – | – | request-only |

**Riss (nicht geglättet):** die Fünfer-Liste stammt aus `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md:27`
(archiviert; „PDS trägt nur open-loop + derived"); der Live-Bestand trägt **Cassini** längst als
`cassini_tnf.bin` (`phi/sources.φ:10160`). Die Aufgabe trägt eine veraltete Prämisse. **Summe: 1/5 offen.**

## B — Redistributions-Declines (41) — die Blockierten, frisch gemessen

| Quelle | legale Route heute | HTTP | Riss |
|---|---|---|---|
| **ALMA** ObsCore | ESO Science Archive (`archive.eso.org/…/eso-data-access-policy.html`) **CC BY 4.0**, Dritt-Distribution erlaubt | 200 | deckt nur **ESO-bediente** ALMA-Records, JAO-Archiv separat |
| **LSST/Rubin** TAP | **nein** (401, `WWW-Authenticate: Basic`; Policy: „No access … without Rubin data rights") | 401 | nur Ersatz: Gaia (andere Daten) |
| **GRDC** | **ja**: GRDC-Caravan `zenodo.org/records/15349031` **CC BY 4.0** | 200 | Subset (5356 Einzugsgebiete), nicht die volle DB |
| **RIPE RIS** | fragil: `archive.routeviews.org` | 200 | **RouteViews BY vs BY-SA**: Text sagt BY, Inline-Link zeigt `by-sa/4.0` |
| **IMCCE** INPOP19a/NOE-4 | Ersatz: JPL `ssd.jpl.nasa.gov/ftp/eph/planets/bsp/de440.bsp` (PD) | 200 | **EPM2021** `quasar.ipa.nw.ru` = **Timeout** → `pending` |
| **TRISP/YBCO-NSE** (MPI-FKF) | kein Roh; Ersatz NIST NCNR (`nist.gov/open/license`) | 200 | NIST **SRD vs non-SRD** je Datensatz unbestimmt |

**Feld-fähige „neu-zulassen" (gemessen):** `giro.uml.edu/didbase` 200 **CC BY-NC-SA 4.0**
(NC-kompatibel) · `ds.iris.edu` 200 (frei, Attribution) · CTBTO-Original 403 · `data.mendeley.com` /
`waterisotopesDB.org` 200 (Lizenz je Dataset ungemessen) · IAEA WISER 403 (Login) · `globalfloods.eu`
200 (Copernicus; **Forecast → kein Zeuge**). Re-Admission = **Mountain**.

## C — Gated China — offene Alternativen

| Quelle | offene Alternative | HTTP | deckt die Größe? |
|---|---|---|---|
| **CEIC D18000** Schwere | **keine** keyless Route zur Quelle; offene **Ersatzapparate**: IGETS/GFZ (CC-BY-4.0, im Register) · BGI · ICGEM | 200 | **nein** — andere Instrumente |
| **CN-NSSDC `有条件共享`** (Lunar/Mars Topographie, `10.12176/…`) | LOLA-RDR (PDS) · SLDEM2015 (`imbrium.mit.edu`) · MOLA (PDS, registriert) · HRSC (ESA PSA) · ICGEM (SHC) | 200 | **ja** (Topographie/SHC) |
| **CSES/Limadou** | ScienceDB `scidb.cn` (`10.57760/sciencedb.06921`, **CC0**) — im Haus `cses_efd/hpm/lap.bin` | 200 | **Sample** (ein Orbit); Voll-Archiv registrierungsgattert |

**Riss (Korrektur, nicht geglättet):** Zenodo `10.5281/zenodo.7112529` / `…7855090` (CC-BY-4.0, offen)
sind **Figuren-Subsets** (~1,0 / 1,3 MB, „a constant value is removed for reading") — **nicht** die
D18000-Reihe. Eine frühere Note („dieselbe CEIC-Schwere, republiziert") war **überzogen**. Für D18000
selbst existiert **keine** keyless Route.

## D — C/NOFS NWM (Absage → `absent`)

**Die Absage (UT Dallas, `docs/handover/antwort-mail-2026-10-16-ceic-utdallas.md`):** das NWM hat nie
funktioniert (Kammer-Leck, verrauschte Ionenquelle) → **es gibt keine NWM-Neutralwinddaten**. Das ist
kein `pending`, sondern **`absent`** — physikalisch leer. Der CINDI-`blocked account`-Wait ist
**hinfällig**. Offene Alternativen (200): **DE-2** `cdaweb.gsfc.nasa.gov/pub/data/de/de2/`
(`neutral_gas_wats/`, `combined_plasma_neutrals_ua/` = kollokales in-situ-Paar) · **ICON/MIGHTI**
`…/pub/data/icon/` (remote, **kein** in-situ-Paar).

## E — MODVOLC Collection 6 (HIGP-Host tot → historische Route bei NASA LAADS)

**Gemessen:** `modis.higp.hawaii.edu/data/…` → **Timeout (HTTP 000)**; DNS löst auf (`128.171.157.89`,
CNAME `hotspot3.higp.soest.hawaii.edu`), HTTP aber tot. `--wayback` kein Snapshot. Lokal:
`data/modis.higp.hawaii.edu/2000/` = **75 570** Dateien, 429 MB (nur Jahr 2000). Register-Route
`phi/sources.φ:12136` = nrt3 **Collection 6.1, nur Recent** (200).

**Die historische Collection 6 liegt unter anderem Namen bei NASA LAADS:**
`https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/6/MODVOLC/<Jahr>/<DOY>/MODVOLC.A<YYYYDDD>.HHMM.006.dat`
→ **303 → Earthdata Login** (auth-gated). Das Haus trägt `EARTHDATA_EDL_TOKEN` (`.secrets.local`) —
die fehlenden Jahre 2001–2019 sind damit erreichbar. **Naming-Riss:** HIGP `…000.000000` · LAADS `…006`
· LANCE `…061.NRT` — deshalb fand die **Dateinamen**-Suche nichts; die **Struktur**-Suche (Jahr/DOY)
fand LAADS. `--all` (416 Z., `zenodo`/`crossref`/`dataverse` …) = **kein** freies Mirror.
Offene Alternativen (200): **NASA FIRMS** `firms.modaps.eosdis.nasa.gov` · **MIROVA** `mirovaweb.it`.

**Riss (Lizenz — nachvollziehbar, nicht „unbelegt"):** Das Register führt `terms PD
https://www.earthdata.nasa.gov/` (`phi/sources.φ:12134`) — die Lizenz **wurde gemessen**
(mountain-340, 2026-10-13: HIGP-Route = keine offene Lizenz → `local-only`; der NASA-LANCE-Spiegel =
PD über die NASA-Datapolitik `earthdata.nasa.gov/…/data-information-guidance`; mycelium-343 stellte die
Route auf `nrt3.modaps.eosdis.nasa.gov`). **Neu und widersprechend:** LAADS Support: „this product is
**not approved for public distribution**." Zwei gemessene Zeilen, die sich nicht einigen — die
**NASA-Politik** (full/open sharing) gegen die **produktspezifische LAADS-Absage**. Ob die PD-Zeile
die **LANCE**-Route deckt und ob `modvolc_thermal.bin` öffentlich bleiben darf, ist **Mountains
Verdikt**, nicht meine Zeile. Der CDN-Träger hat jedenfalls unter `terms PD` veröffentlicht; ob das
trägt, ist damit **offen**. Der `license_census`-Gate prüft das Vorhandensein einer `terms`-Zeile,
nicht ihre **Wahrheit** — die Lücke bleibt.

## Grenze

Alle Lizenzen „ungemessen" heißen `pending`, nie frei. Die Re-Admission der Kandidaten (B) und die
Register/CDN-Nähte (A, C, E) sind nicht meine Hand. Kein stilles `pending` — jeder Rest ist benannt.
