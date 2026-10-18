# Register — die leeren `PD`-Träger bekommen eine gemessene Policy-URL

**Linie:** mountain · **Datum:** 2026-10-18 · **Herkunft:** Dispatch
`post/out/2026-10-18-mountain-register-formfehler…` Punkt (2), Lead
`omegaflow-work/recherche/leads/2026-10-18-terms-wahrheit-pd.md`.

## Der Defekt

`license_census` prüft die **Existenz** einer `terms`-Zeile, nicht ihre **Wahrheit**. 44 distinkte
`terms PD`-Referenzen zeigten auf eine **nackte Startseite ohne Lizenzsatz** — eine Behauptung ohne
Träger. Fix: **andere URL** (Behörden-Policy statt Startseite); wo die Seite `PD` widerlegt, auch der **Wert**.

## Die Messung (Browser, `document.body.innerText` — Roh-`curl` rendert JS-Seiten nicht)

| Behörde / Dienst | gemessene Policy-URL | wörtlich |
|---|---|---|
| NASA | `https://sti.nasa.gov/disclaimers/` | *„Generally, United States government works … are **not protected by copyright** in the U.S. (17 U.S.C. §105) and may be used without obtaining permission from NASA."* + Ausnahme: privat erstellte Werke bleiben geschützt. |
| NOAA | `https://gml.noaa.gov/about/disclaimer.html` | *„The information on government servers are in the **public domain**, unless specifically annotated otherwise …"* |
| USGS | `https://www.usgs.gov/information-policies-and-instructions/copyrights-and-credits` | *„USGS-authored or produced data and information are considered to be in the **U.S. Public Domain**."* |
| IRSA (IPAC) | `https://irsa.ipac.caltech.edu/data_use_terms.html` | *„Most data served by IRSA is public, with **no usage restrictions**."* (proprietäre und © DSS-Daten ausgenommen) — Titel: *Terms of Use of Data Hosted by IRSA* |
| ExoFOP (IPAC) | `https://exofop.ipac.caltech.edu/tess/pcp.php` | *„any data … should be considered **their property** and should not be claimed as anyone else's"*; TESS-Daten bis 12 Monate proprietär → **nicht PD** |
| NASA Exoplanet Archive | `https://exoplanetarchive.ipac.caltech.edu/docs/acknowledge.html` | *„Please include the following **standard acknowledgment** …"* (Caltech für NASA) |
| ORNL Open Energy | `https://openenergyhub.ornl.gov/terms/terms-and-conditions/` | Open-Data-Portal-GTC; Datensätze unter **OPEN LICENSE**, Rechte Dritter vorbehalten |
| US Census (API) | `https://www.census.gov/data/developers/about/terms-of-service.html` | API-**Terms of Service**; keine Re-Identifikation (13 U.S.C. §§ 8, 9) |
| Water Quality Portal | `https://www.waterqualitydata.us/portal_userguide/` | *„Cite the Water Quality Portal …"* (DOI `10.5066/P9QRKUVJ`); USGS-Daten provisorisch |
| AWS Open Data (CRAAM) | `https://registry.opendata.aws/craam-open-vlf/` | *„License: **There are no restrictions on the use of this data**."* — Seite trägt den Satz bereits |

## Die Ausführung

**44 Startseiten-URLs → gemessene Policy-Seiten; 698 `terms`-Zeilen in `phi/sources.φ`.**

| Familie | Zeilen | Wert | Referenz |
|---|---|---|---|
| NASA (GSFC · EOSDIS · Earthdata · JPL · PDS-Knoten) | 517 | `PD-USGov` | `sti.nasa.gov/disclaimers/` |
| NOAA (NDBC · NCEI · SWPC · PMEL · NOS …) | 136 | `PD-USGov` | `gml.noaa.gov/about/disclaimer.html` |
| USGS (Earthquake · Astrogeology · Water · Geomag · ScienceBase) | 17 | `PD-USGov` | `usgs.gov/…/copyrights-and-credits` |
| IRSA | 18 | `per-service-terms` | `irsa…/data_use_terms.html` |
| ExoFOP | 4 | `per-service-terms` | `exofop…/tess/pcp.php` |
| NASA Exoplanet Archive | 3 | `other-attribution` | `exoplanetarchive…/docs/acknowledge.html` |
| ORNL | 1 | `per-service-terms` | `openenergyhub…/terms/terms-and-conditions/` |
| Census | 1 | `per-service-terms` | `census…/terms-of-service.html` |
| WQP | 1 | `other-attribution` | `waterqualitydata.us/portal_userguide/` |

Der Wert `PD` wurde für die US-Behörden auf den im Register bereits etablierten **`PD-USGov`** gehoben
(die Referenz sagt „public domain", nicht bloß „PD"). Nur `terms`-Zeilen geändert.

## Was gemessen **keine** Lizenzseite hat (Riss, benannt — nicht ungemessen)

- **USNO/IERS** — `maia.usno.navy.mil` (1): die Seite hat keine Lizenz-/Copyright-Seite; der USNO-Link
  „Public Use of Limitations" zeigt auf `dimoc.mil/resources/limitations/` → **404**; `dimoc.mil/Terms/`
  fällt auf die DIMOC-Startseite zurück. US-Regierungswerk, aber **keine gemessene Policy-Seite**.
- **`ser.im-ldi.com/M10/`** (2): Mariner-10-Bildprojekt (ASU, NASA-finanziert) — keine Lizenz-/Terms-Seite
  auf Site oder Root. NASA-Förderung legt `PD-USGov` nahe, ist aber eine **Inferenz**, keine Messung.

**Beleg:** Commit mit dieser Datei + `phi/sources.φ`. Zitate oben, gemessen 2026-10-18 (Browser).
