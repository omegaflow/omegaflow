# Register — die leeren `PD`-Träger bekommen eine gemessene Policy-URL

**Linie:** mountain · **Datum:** 2026-10-18 · **Herkunft:** Dispatch
`post/out/2026-10-18-mountain-register-formfehler…` Punkt (2), Lead
`omegaflow-work/recherche/leads/2026-10-18-terms-wahrheit-pd.md`.

## Der Defekt

`license_census` prüft die **Existenz** einer `terms`-Zeile, nicht ihre **Wahrheit**. 44 distinkte
`terms PD`-Referenzen zeigten auf eine **nackte Startseite ohne Lizenzsatz** — eine Behauptung ohne
Träger. Der Vorschlag des Leads: **andere URL, nicht anderer Wert** — die Behörden-Policy statt der
Startseite. Zwei Behörden (NASA allgemein, IPAC/Caltech allgemein) waren noch **ungemessen**.

## Die Messung (Browser, `document.body.innerText` — Roh-`curl` rendert JS-Seiten nicht)

| Behörde | gemessene Policy-URL | wörtlich |
|---|---|---|
| NASA | `https://sti.nasa.gov/disclaimers/` | *„Generally, United States government works … are **not protected by copyright** in the U.S. (17 U.S.C. §105) and may be used without obtaining permission from NASA."* + Ausnahme: privat erstellte Werke bleiben geschützt. |
| NOAA | `https://gml.noaa.gov/about/disclaimer.html` | *„The information on government servers are in the **public domain**, unless specifically annotated otherwise, and may be used freely by the public so long as you do not 1) claim it is your own …"* |
| USGS | `https://www.usgs.gov/information-policies-and-instructions/copyrights-and-credits` | *„USGS-authored or produced data and information are considered to be in the **U.S. Public Domain**."* + Ausnahme: Fremdmaterial. |

## Die Ausführung (nur die URL, der Wert bleibt unangetastet)

44 Startseiten-URLs → die drei Policy-Seiten; **664** `terms PD`-Zeilen in `phi/sources.φ`:

| Familie | Zeilen | neue Referenz |
|---|---|---|
| NASA (GSFC · EOSDIS · Earthdata · JPL · PDS-Knoten SBN/Rings/Geo/UMD) | 517 | `sti.nasa.gov/disclaimers/` |
| NOAA (NDBC · NCEI · SWPC · PMEL · NOS … · Aviation Weather) | 135 | `gml.noaa.gov/about/disclaimer.html` |
| USGS (Earthquake · Astrogeology · Water · Geomag · ScienceBase) | 13 | `usgs.gov/…/copyrights-and-credits` |

Nur `terms PD`-Zeilen geändert (`git diff` = 664 insertions / 664 deletions, sonst nichts).

## Was NICHT geschlossen ist (Riss, benannt)

Weiterhin **nackte Startseite** — keine gemessene allgemeine Policy gefunden (nicht erfunden):

- **IPAC/Caltech** — `irsa.ipac.caltech.edu` (18) · `exofop.ipac.caltech.edu` (4) ·
  `exoplanetarchive.ipac.caltech.edu` (3). Nur die *Image Use Policy* ist messbar; NED trägt bereits
  `© Caltech`. Eine **Daten**-Policy fehlt.
- **USNO** — `maia.usno.navy.mil` (1). **US Census** — `www.census.gov` (1). **ORNL/DOE** —
  `openenergyhub.ornl.gov` (1, „openly licensed" ohne konkreten Satz). **WQP** —
  `www.waterqualitydata.us` (1, Mehrbehörden-Konsortium). **kommerziell** — `ser.im-ldi.com/M10/` (2).
  **AWS Open Data** — `registry.opendata.aws/craam-open-vlf/` (1, dataset-eigene Lizenz).

**Zweiter Riss:** der Wert bleibt `PD` (unpräzise, der Lead schlägt `PD-USGov` vor) — das ist das
Register-Verdikt und bleibt bei Mountain, hier nicht mitentschieden.

**Beleg:** Commit mit dieser Datei + `phi/sources.φ`. Zitate oben, gemessen 2026-10-18.
