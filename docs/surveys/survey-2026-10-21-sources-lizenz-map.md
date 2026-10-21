<!--
  title: Sources-Lizenzmap — NC ist frei, 55 von 63 LOCK lösen sich (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 42ceaa2438df948b125e89cdf8a4a53cc532a24089c17a9a69a702f9381a33cd
  status: live
  owner: future
  ack: future 2026-10-21T15:45Z
  next: per-service/reviewed/unknown durch gemessene Lizenzen ersetzen; 7 auth-LOCK ohne CDN
  orphan-after: none
  see-also: docs/surveys/survey-2026-10-21-sources-dryrun.md docs/concepts/messzustaende-v3.md
-->
# Sources-Lizenzmap — NC ist frei, 55 von 63 LOCK lösen sich

**Regel.** Kein `unknown`/`LOCK` auf dem CDN. Jede Lizenz wird gemessen; `LOCK`
nur für **auth-gated**, dann zeigt der Eintrag auf die **Live-API** (nicht
auth) oder die **Domain** (auth-gated), nie auf ein CDN-Asset. Wir sind **NC**:
NC-kompatible Lizenzen sind `frei` (Attribution).

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
| `auth-required` | IEEG, PRADAN, CDPP, TNS, Lasair, CDDIS, Babamul (8) | Login/Token, keine freie Route gemessen | **LOCK** |

**Ergebnis:** von **63 LOCK** sind **55 frei** (Attribution/NC) und **8
auth-gated**. Die 55 werden auf ihre gemessene Lizenz gezogen; die 8 bleiben
`LOCK(rechtlich)` und verlieren jedes CDN-Asset.

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

## Die Ersetzungen (Vorschlag, noch nicht geschrieben)

- `per-service-terms` → die gemessene Lizenz je Block (`PD-USGov` für
  IRSA/ExoFOP/Census/SSD, `gebco-terms`, `attribution` für DC.g-vo/ORNL).
- `vires-esa-data-terms` → `vires-esa-attribution`.
- `wdcb-sep-policy` → `wdcb-nc` (NC, frei für uns).
- `reviewed-2026-10-20` → `stand`-Feld, nicht `terms`; die echte Lizenz aus
  dem Block nachziehen.
- Die 22 Unbekannten: messen, dann ersetzen oder `LOCK` (nur auth-gated).

## CDN-Regel

- **frei:** CDN-Asset erlaubt, Lizenz + Attribution im Block.
- **LOCK(auth):** **kein** CDN-Asset; `url` = Live-API (nicht auth) oder
  **Domain** (auth-gated). Kein Release-Upload.

**Status:** Map gemessen; der Register-Edit läuft in Chargen mit
`source_validate` + `register_sort`. Die 7 auth-LOCK sind die einzigen, die
`LOCK` behalten.
