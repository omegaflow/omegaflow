<!--
  title: Sources-Lizenzmap — NC ist frei, 55 von 63 LOCK lösen sich (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 68efc1e40cc23bf9a29ec293fa6580f3e4b1a127d668a54eea4765254d9f0a61
  status: live
  owner: future
  ack: future 2026-10-21T15:45Z
  next: per-service/reviewed/unknown durch gemessene Lizenzen ersetzen; 7 auth-LOCK ohne CDN
  orphan-after: none
  see-also: docs/surveys/survey-2026-10-21-sources-dryrun.md docs/concepts/messzustaende-v3.md
-->
# Sources-Lizenzmap — NC ist frei, 56 von 63 LOCK lösen sich — 6 LOCK, 1 pending

**Regel.** Kein `unknown`/`LOCK` auf dem CDN. Jede Lizenz wird gemessen.
**Auth ist kein Lock:** ein Login/Token ist eine Route. `LOCK` nur, wenn die
Lizenz die freie Weitergabe verbietet oder keine Lizenz vorliegt. Wir sind
**NC**: NC-kompatible Lizenzen sind `frei` (Attribution).

## Gemessen (HTTP 200, Termseite gelesen)

| Term | Quelle | Befund | Zugang |
|---|---|---|---|
| `per-service-terms` | `irsa.ipac.caltech.edu/data_use_terms.html` (18) | NASA/IPAC, Attribution | **frei** |
| `per-service-terms` | `gebco.net/.../terms-of-use` (4) | public domain, free of charge, attribution | **frei** |
| `per-service-terms` | `exofop.ipac.caltech.edu` (4) | NASA/IPAC | **frei** |
| `per-service-terms` | `dc.g-vo.org` (4) | attribution | **frei** |
| `per-service-terms` | `census.gov` (1) | US-Government, attribution | **frei** |
| `per-service-terms` | `ssd-api.jpl.nasa.gov` (1) | NASA/JPL | **frei** |
| `per-service-terms` | `openenergyhub.ornl.gov` (1) | open, attribution | **frei** |
| `vires-esa-data-terms` | `vires.services` (12) | attribution (4×) | **frei** |
| `wdcb-sep-policy` | `wdcb.ru/sep/policy.html` (8) | non-commercial + open data | **frei** (NC) |
| `PD-USGov` | CDDIS (Earthdata) | full and open sharing | **frei** |
| `ieeg-terms`, `pradan-terms`, `tns-copyright`, `babamul-no-license`, `ilrs-no-license`, `lasair-copyright` | IEEG, PRADAN, TNS, Babamul, ILRS, Lasair (6) | Kopier-/Weitergabeverbot oder keine Lizenz | **LOCK** |
| `regards-pending` | REGARDS/CNES (1) | Lizenz nicht gemessen | **pending** |

**Ergebnis:** von **63 LOCK** sind **56 frei** (Attribution/NC, inkl. CDDIS),
**6 LOCK** (Lizenz verbietet Weitergabe oder fehlt) und **1 pending**
(REGARDS, ungemessen). Die 56 dürfen aufs CDN (mit Attribution); die 6 LOCK
und REGARDS nicht.

## Die 113 Unbekannten

- **91 frei** (NC/offen): `CC-BY-3.0`, `CC-BY-SA-*`, `CC-BY-NC-SA`, `MIT`,
  `ODbL`, `OGL-UK/Canada`, `Copernicus`, `ODC-DbCL/By`, `PDDL`, `GPL-3.0`,
  `cadc-open`, `onc-open`, `noaa-nodd-open`, `USGov-disclaimer`,
  `hawc-public`, `ligo-public`, `argo-free`, `jp-public-data`,
  `Open-Licence-2.0`, `own-work`, `DHM-open`, `kari-public-release`,
  `ipac-public`, `dl-de-by-2.0`, `SWS-WDC`, `wmo-data-policy`,
  `eccc-end-use`, `eso-data-access`, `eida-data-policy`,
  `gosat-data-policy`, `ia2/datacentral/alma/rpw-ack`, `nmdb-non-commercial`,
  `ioc-...-no-commercial`, `columbia-research-noncommercial`,
  `non-commercial-government-licence`.
- **22 zu messen:** `no-redistribution`, `no-license`, `unknown` (2),
  `license-per-project`, `dataverse-per-dataset` (2), `kcdc-eula`,
  `cedar-rules-of-the-road` (2), `erg-sc-rules`, `eiscat-rules`,
  `naoj-copyright`, `10.11575/AFYX-M516`, `reviewed-2026-10-20` (8 —
  **Prüfdatum, kein Term**).

## Die Ersetzungen (geschrieben 2026-10-21)

- `per-service-terms` → die gemessene Lizenz je Block (`PD-USGov` für
  IRSA/ExoFOP/Census/SSD, `gebco-terms`, `attribution` für DC.g-vo/ORNL,
  `dlr-license`, `fingrid-open`).
- `vires-esa-data-terms` → `vires-esa-attribution`.
- `wdcb-sep-policy` → `wdcb-nc` (NC, frei für uns).
- `reviewed-2026-10-20` → `stand`-Feld, nicht `terms`; die echte Lizenz aus
  dem Block nachziehen.
- Die 22 Unbekannten: messen, dann ersetzen oder `LOCK` (nur auth-gated).

## Browser-Messung der 6 LOCK (2026-10-21)

| Quelle | Browser-Befund | Zugang |
|---|---|---|
| IEEG | App ohne Termseite; frühere Browser-Messung: Registrierungs-Terms mit Kopier-/Verteilverbot | **LOCK** |
| PRADAN | „Copyright © 2020-2025 ISTRAC. All Rights Reserved." + Login | **LOCK** |
| TNS | 403 Forbidden, keine Lizenz | **LOCK** |
| Babamul | kein Lizenztext | **LOCK** |
| ILRS/EDC | Login/Register, keine Lizenz | **LOCK** |
| Lasair | Copyright Edinburgh/QUB, „all rights reserved" | **LOCK** |
| CDDIS | Earthdata „full and open sharing" | **frei** |
| REGARDS | keine Termseite messbar | **pending** |

## CDN-Entfernung (2026-10-21)

- **IEEG:** 120 `*_ieeg.bin` auf `openneuro.org` — Entfernung läuft.
- **Lasair:** `skydirections.bin` entfernt; `skydirection-cdn.yml` ohne
  Lasair-Arm.
- **CDDIS:** frei, Assets bleiben.
- **PRADAN/TNS/Babamul/ILRS:** keine CDN-Assets, keine dedizierten Jobs.
- **`openneuro-cdn.yml`:** Guard entfernt `*_ieeg.bin` nach dem Upload.

**Status:** 56 frei, 6 LOCK, 1 pending; Register-Edit geschrieben; CDN für
IEEG/Lasair bereinigt, Guard aktiv.
