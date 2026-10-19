# Zwei Quellen ohne Lizenzseite — Alternativen mit offener Lizenz (gemessen)

**Linie:** mountain · **Datum:** 2026-10-18 · **Herkunft:** Operator-Auftrag nach
`state/rat/2026-10-18-terms-policy-urls-gemessen.md` (die zwei verbliebenen Quellen).

Zwei Agenten (`general`) haben per Archiv-/Web-Suche Alternativen gesucht. Ihre Belege habe ich
**selbst nachgemessen** (Browser bzw. DataCite-API). Urteil und Eintrag bleiben bei der Linie.

## 1 · Earth Orientation Parameters (`maia.usno.navy.mil`)

Die USNO-Seite hat keine Lizenzseite. Dieselben EOP-Daten führt **NASA CDDIS** als offene Daten.

| Alternative | Lizenz | Beleg (wörtlich) |
|---|---|---|
| **NASA CDDIS / Earthdata** — `cddis.nasa.gov/archive/products/iers` (Spiegel des IERS RS/PC: `finals.daily`, `finals2000A.daily`, `gpsrapid.daily`, Bulletin A, `deltat.*`, `leapsec.dat`) | **CC0-1.0** (NASA-Open-Data) | *„Unless the content is marked with a use restriction or license, data provided from a NASA-led mission are licensed as Creative Commons Zero (CC0)."* — `earthdata.nasa.gov/engage/open-data-services-software-policies/data-use-guidance` |
| **astropy-iers-data** — `github.com/astropy/astropy-iers-data` (enthält exakt `finals2000A.all`, `eopc04.1962-now`, `Leap_Second.dat`) | **BSD-3-Clause** | *„Redistribution and use in source and binary forms … are permitted provided that the following conditions are met"* — `…/blob/main/LICENSE.rst` |
| IERS Data Center — `datacenter.iers.org/eop.php` | **keine offene Lizenz messbar** (nur Impressum/Disclaimer) | — |

**Eingetragen:** `terms CC0-1.0 https://www.earthdata.nasa.gov/engage/open-data-services-software-policies/data-use-guidance`
(für `finals2000A.all`). *Grenze:* die CC0-Zuordnung ist NASA-Open-Data für den CDDIS-Spiegel; USNO
selbst nennt nur „Distribution Statement A — distribution unlimited" (Freigabe, keine Lizenz). Das
exakte `.all`-File liegt zusätzlich in `astropy-iers-data` (BSD-3).

## 2 · Mariner-10-Bilder (`ser.im-ldi.com/M10/`)

Die ASU-Seite hat keine Lizenz-/Terms-Seite. Dieselben Roh-EDRs führt das **NASA PDS**.

| Alternative | Lizenz | Beleg (wörtlich) |
|---|---|---|
| **PDS Mariner 10 Imaging Archive EDR** — `MR10-H/L/V-NAC/WAC-2-EDR-V1.0`, DOI `10.17189/1520314`, Volumes `mvm_0001`–`mvm_0015` (`planetarydata.jpl.nasa.gov/img/data/mariner10/`) | **CC0-1.0 + US-Government-Werk** | DataCite `rightsList`: *„This is a work of the U.S. Government and is not subject to copyright protection in the United States. Foreign copyrights may apply."* **und** *„Creative Commons Zero v1.0 Universal"* (`rightsIdentifier: cc0-1.0`) |
| **NSSDCA Image Catalog** — `nssdc.gsfc.nasa.gov/imgcat/…/MC_Mariner_10_page1.html` (fertige Merkur-Mosaike) | **Public Domain (NASA)** | *„Images presented on NSSDC's Image Catalog are in the public domain and rights to the image are not held unless otherwise noted."* |
| NASA Photojournal PIA02418 | keine eindeutige offene Lizenz | — |

**Eingetragen:** `terms CC0-1.0 https://doi.org/10.17189/1520314` für
`ser.im-ldi.com/…/mariner_m10_edr.bin`. Die `origin` `…/MVM/mvm_0001.tgz` ist die Volumen-ID
`mvm_0001` des PDS-Archivs — dieselben EDR-Bytes, offen lizenziert.

## Was offen bleibt (Riss, benannt)

- **`mariner_m10_camera_angles.txt`** (`origin …/M10_ISIS/camera_angles.txt`): ASU-/ISIS-**Ableitung**;
  kein Archiv mit offener Lizenz führt diese Datei. Die zugrunde liegende Geometrie liegt in den
  PDS-SPICE-Kernels (CC0) — eine **Neu-Ableitung** wäre Bau, kein Quellen-Tausch. Darum bleibt
  `terms PD https://ser.im-ldi.com/M10/` hier stehen (eine Zeile).
- **Provenienz:** die `origin`-Felder zeigen weiter auf `maia`/`ser.im-ldi.com`. Ein Umzug der
  Harvest-Quelle auf CDDIS/PDS (neue `origin`, neuer `sha256`, CDN-Release) ist ein eigener Bau-Schritt.

**Beleg:** Commit mit dieser Datei + `phi/sources.φ`. Zitate oben, gemessen 2026-10-18.

## Nachtrag 2026-10-18 — die Attitude hat doch eine PD-Quelle

Der Satz oben („kein Archiv mit offener Lizenz führt diese Datei") gilt für die **ASU-Datei selbst**.
Für die **Haltung** gibt es aber eine public-domain-Alternative, die bei der ersten Messung übersehen
wurde (NAIF-only geprüft): die **ISIS-Datenarea** `usgs_data/mariner10` (S3) führt **4 CK + 4 SPK**.
Verifizierte Kommentare: `MERCURY_MARINER_10_A.bc` = *„RAND Corrected Type 1 Segment Mariner10"*,
`MERCURY_MARINER_10_A.bsp` = *„RAND Corrected Type 9 Segment Mariner10"* — USGS/NASA, public domain.

**Grenze:** RAND-korrigiert (1976er Netz) = **externer Fit**, kein Unabhängigkeitsbeweis; die
ASU/Robinson-Pose kann genauer sein. `*Nominal.pvl` = Reseaus, keine Attitude.
