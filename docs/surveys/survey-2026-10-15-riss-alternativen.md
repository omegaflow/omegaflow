<!--
  title: Survey — Riss-Alternativen: fünf Taucher, offene Ersatzwege (2026-10-15)
  class: survey
  date: 2026-10-15
  sha256: ceaa379d1d4b9a5995b75aaa22abd2120a010b809f92f90664e73535dedadb45
  status: live
  see-also: phi/sources.φ docs/concepts/source-curation.md docs/handover/archiv/handover-2026-10-15-mountain-folge351.md docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md
-->
# Survey — Riss-Alternativen: fünf Taucher, offene Ersatzwege (2026-10-15)

Operator-Frage 2026-10-15: „und für all die risse gibt es keine alternativen?" Fünf
Taucher suchten je Riss-Familie den offenen Ersatzweg. Ergebnis: **für jeden
Daten-Riss gibt es eine offene Alternative** — teils bereits im Register. Alle
Zeilen gemessen am 2026-10-15; keine Zahl kopiert.

## 1 · OCNS2019 (Host tot) → **NSDB (IMCCE)** — die Beobachtungs-Alternative
- Der Riss: OCNS2019 (`10.12149/101048`, `paperdata.china-vo.org` pending → nur
  Wayback) ist ein **Beobachtungs**-Katalog; JPL/NAIF ist die *berechnete*
  Ephemeride, kein Ersatz.
- **Alternative:** **NSDB — Natural Satellites DataBase**, `https://nsdb.imcce.fr/`
  (live, direkter Klartext-Download der Roh-Tabellen, z. B.
  `https://nsdb.imcce.fr/obspos/OBS_COLL/N/no0011.txt`, HTTP 206). **Neptun-System
  1847–2024** (Triton 1847–2024, Nereid 1949–2021, innere/äußere), Gaia- und
  MPC-Zeilen enthalten. **Lizenz** (`/nsdb/acknowledging.html`): *„free for
  scientific research under the condition that the users acknowledge IMCCE/Paris
  observatory and the FP7-ESPaCE program …"* — frei für Forschung + Nennung, **kein
  CC**. Nicht in `phi`; die einzige Beobachtungs-Astrometrie-Klasse dort ist VizieR
  `astrometry_series` (Uranier) + Gaia-DR3 SSO.
- **Hinweis:** die Query-Backend-Form ist auf dem HTTrack-Spiegel nicht erreichbar
  (GET 404) — die **Roh-Tabellen** sind es.

## 2 · GOCE (Token = Magnet, Handover = Schwere) → **ICGEM-GOCE-Schwere (CC-BY-4.0)**
- **Alternative:** ICGEM-Koeffizientenmodell **GO_CONS_GCF_2_DIR_R6**,
  DOI `10.5880/ICGEM.2019.004` → **CC-BY-4.0** (`dataservices.gfz.de/…`); freie
  ESA-Vorderseite `https://earth.esa.int/eogateway/missions/goce/data`. **Nicht in
  `phi`** — `sgrep -i "icgem"`/`"bgi"` = 0. Das ist die genuin fehlende offene
  Schwere-Route.

## 3 · CGrav/CEIC (Schwere gated) → **schon im Register: AGrav + IGETS**
- **Alternative:** der China-Spalt ist durch die **bereits registrierten** Quellen
  gedeckt: **AGrav** (BGI+BKG absolute-Gravity-DB, `phi:10011…10017`) und **IGETS**
  (`phi:10338…10345`, `CC-BY-4.0`) — IGETS trägt die **chinesischen Stationen**
  (Lhasa OSG 057, Lijiang OSG 066, Hsinchu GWR T048). BGI-Absolute-DB: *„free of
  charge without any restriction"* (`bgi.obs-mip.fr`). **Kein Doppel-Register.**
- Regionale offene Ergänzung: Zenodo `10.5281/zenodo.11212541` (NE-China-Schwere,
  CC-BY-4.0), Figshare `10.6084/m9.figshare.20073575` (CC-BY-4.0).
- Der gated CGrav/CEIC bleibt als `pending` benannt (Zugang gated, keine zwei
  vergleichbaren Messungen; kein `riss` — keine Notwendigkeit).

## 4 · MOSWOC-Kp (restricted) → **GFZ-Kp ist der offene Primär-Ursprung**
- **Alternative:** **GFZ Potsdam** (der Produzent): JSON `https://kp.gfz.de/app/json/?start=…&end=…&index=Kp&status=def,nowcast`, def. WDC-Archiv
  `https://datapub.gfz.de/download/10.5880.Kp.0001/Kp_definitive/Kp_defYYYY.wdc`
  **1932–heute**, **CC-BY-4.0**, DOI `10.5880/Kp.0001`.
- **Unabhängiger offener Spiegel:** **WDC Kyoto HAPI** `hour3h_kp`
  (`https://wdc.kugi.kyoto-u.ac.jp/hapi/data?id=hour3h_kp&…`), `CC-BY-4.0` deklariert,
  1932–2026. **NOAA SWPC** `noaa-planetary-k-index.json` nur ~7 Tage rollend (PD).
- **Im Register vorhanden:** **ein** Kp-Block `phi:1230-1234` (GFZ-JSON, aber auf
  *heute→morgen* gefenstert — erreicht das 1932-Archiv **nicht**). Die restricted
  Zenodo-Datei ist damit **nicht nötig**; der Zugangs-Antrag entfällt als Bedarf.

## 5 · WINDMI (kein numerisches Produkt) → **Dst/AL sind schon gemessen im Register**
- **Alternative:** die *gemessenen* Indizes statt der Modell-`.gif`s:
  **WDC Kyoto HAPI** `hour_dst` + `min_ae` (open) — **bereits registriert** als
  `wdc_dst_nt` (`phi:4035`) und `wdc_ae_al_nt` (`phi:4030`, 60 s) im `wdc_ae`-Block
  (`phi:4023-4035`), Reader `src/archivar/wdc_ae.rs`, CDN-Asset `wdc_ae.bin` (206).
  **SuperMAG** `SML`≈AL (`supermag.jhuapl.edu/services/indices.php`, offen, **kein
  Redistributionsrecht**; Code `supermag_index_compiler.rs`, kein `phi`-Block) und
  **OMNI** `AL_INDEX1800`/`DST1800` (CDAWeb HAPI, PD, `phi:636-645` nur Solarwind).
- WINDMI als Vorlauf-**Modell** bleibt ein Ketten-Baustein; seine Ausgabe muss nicht
  geerntet werden — die gemessenen Indizes tragen den Zielwert.

## 6 · OVATION-AACGM (eine Station) → **BAS-IMAGE + DMSP-SSJ (global, offen)**
- **Alternative (beste):** **BAS IMAGE Auroral Boundary Data** — globales Oval je
  1-h-MLT-Bin; CSV, AACGM-Grenzen; **Mai 2000 – Okt 2002**, ~2-min;
  **Open Government Licence v3.0** (NERC/BAS/UKRI). Direktdownload
  `http://ramadda.nerc-bas.ac.uk/repository/entry/get/…/SI12_Boundary_Data/si12_boundaries.zip…`.
- **Zweite (mehrsatellitisch, neuer):** **DMSP SSJ Auroral Boundaries 2010–2014**,
  Zenodo `10.5281/zenodo.3373812` (F16/17/18, **CC-BY-4.0**, Datei
  `fom_boundaries_F16_F17_F18_2010_2014.zip`, 8.1 MB, 206).
- **NOAA/SWPC OVATION** `https://services.swpc.noaa.gov/json/ovation_aurora_latest.json`
  (globales 1°-Raster, aber **nur „latest"**, kein Archiv). **DMSP SSUSI EDR-Aurora**
  offen auf CDAWeb (`…/dmsp/dmspf{16..19}/ssusi/data/edr-aurora/`).

## 7 · Die drei Risse, die der Baum selbst schon löst
- **MODVOLC (keine Lizenz):** die Alternative liegt im Register — der **LANCE/NRT**
  -Block `phi:12065-12072` (`terms PD`, Host `nrt3.modaps.eosdis.nasa.gov`). Das
  HIGP-Archiv bleibt `local-only`, kein neuer Block.
- **OpenTopography-Namens-Riss:** der Compiler liest Scripps-SRTM15+; die registrierte
  Alternative ist der **OpenTopography-API-Block** `srtm15_plus_globaldem`
  (`phi:26638-26644`, `terms PD`).
- **MAGION-Namens-Riss:** die offene Magnetometer-Alternative ist der **Hauptsatellit
  IMAP-3** (CDPP `10.24400/802406/mqcs-ddjr`, NASA/SPDF `IA_K0_MFI` CC0) — bereits
  gemessen, nicht MAGION.

## 8 · Ohne Alternative
- **`dead_sources.φ`/`declined_sources.φ`** (`survey-2026-10-05` zitiert): existieren
  nicht im Baum — kein Ersatz, nur die **Streichung** der Zitate.

## Register-Stand / was der Baum noch braucht

- **Neu zu registrieren (`pending`, nicht `absent`):** ICGEM-GOCE-Schwere (`10.5880/ICGEM.2019.004`),
  NSDB (`nsdb.imcce.fr`), BAS-IMAGE-Boundaries (OGL v3), DMSP-SSJ
  (`10.5281/zenodo.3373812`); je Blockform (Mycelium `url`/`origin`/`compiler`) +
  Mountain-`terms`.
- **Schon gedeckt:** CGrav (AGrav+IGETS), WINDMI-Zielwerte (wdc_dst/wdc_ae_al),
  MODVOLC (LANCE/NRT), Kp-Primär (GFZ — der Block existiert, aber nur als
  Heute→Morgen-Fenster; das Archiv ist die offene Ergänzung).
