# M10-Bau-Auftrag — die drei Agenten-Messungen (2026-10-18)

**Linie:** mountain · **Auftrag:** `docs/auftrag/auftrag-m10-haltung-eigen.md` · Drei `general`-Agenten
(Archiv-/Web-Suche), Belege von der Linie **selbst nachgemessen** (curl/HTTP).

## 1 · ISIS (Agent 1)

| Befund | verifiziert |
|---|---|
| Lizenz | `LICENSE.md`: *„Unless otherwise noted, this project is in the public domain in the United States."* + *„…waive copyright … through the CC0 1.0 Universal public domain dedication."* |
| Install | Conda-Binary `isis 10.0.0`, `usgs-astrogeology` — **22 793 722 B**, HTTP 200; `mamba install -c usgs-astrogeology isis` |
| Tars `ENC1/ENC2/ENC3/ALL_RES` | **404** überall (Repo, Releases 0 Assets, S3, USGS-Pfade) — nicht erreichbar |
| Datenarea `usgs_data/mariner10` | **45 Objekte**, 321 MiB: Kalibrierung, Reseaus, **4 CK + 4 SPK**, IK/FK/SCLK |

**Korrektur (wichtig):** Der Handover `handover-2026-10-12-mountain-folge333.md` sagt „ASU ist die
einzige Pose-Quelle". Das war eine **NAIF-only**-Messung. Die **ISIS-Datenarea** führt eigene
SPICE-Kernel — verifiziert an den Kommentaren:

- `kernels/ck/MERCURY_MARINER_10_A.bc` (69 632 B): *„RAND Corrected Type 1 Segment Mariner10"*
- `kernels/ck/MARINER_10_A_gem.bc` (429 056 B): Moon/Venus/Mercury-Segmente
- `kernels/spk/MERCURY_MARINER_10_A.bsp` (49 152 B): *„RAND Corrected Type 9 Segment Mariner10"*

Also: **lizenzklare Pose vorhanden** (USGS/NASA PD) — aber **RAND-korrigiert** (1976er Netz), ein
**externer Fit**, kein Unabhängigkeitsbeweis. Die ASU/Robinson-Pose bleibt ggf. genauer, ist aber
unlizenziert. `*Nominal.pvl` = **Reseau-Koordinaten**, keine Attitude.

## 2 · Shape-Model (Agent 2)

| Befund | verifiziert |
|---|---|
| MESSENGER Global DEM v2 | `MSGR_DEM_USG_SC_I_V02`, DOI **10.17189/1520282**, 665,24 m/px, 11520 × 23040 |
| Download | globales TIFF **530 934 581 B**; „Access Constraints: None", „Use Constraints: Please cite authors" |
| MDIS-BDR | DOI **10.17189/1520406**, 166 m, global TIFF 4,25 GB |
| Abdeckung M1–M3 | DEM global deckt alle drei; subsolarer Punkt (Agent-Rechnung) ≈ 258° E / 23° S, beleuchtet ≈ 168–348° E; „gore" nördlich des Äquators (NASA SP-423) |

**Grenze:** DEM ist ein geglättetes, interpoliertes Photogrammetrie-Produkt — taugt als
A-priori-Shape/Orthorektifizierungsfläche, nicht als hochgenaues Stereomodell. Encounter→Kachel-
Zuordnung beruht auf Beleuchtungsgeometrie + SP-423, nicht auf Bildfußabdrücken.

## 3 · G2-Machbarkeit (Agent 3)

| Pfad | Ergebnis | Beleg |
|---|---|---|
| Sterne in den 811 Pose-Frames | **nein** (Stichprobe 0/12; Median-Belichtung 22,8 ms) | NSSDCA `1973-085A-01` (3 ms–12 s, 832×700), ASU-Metadaten |
| Limb in den 811 Pose-Frames | **ja**: **84** Frames δ < 0,27°, **33** mit sauberem Kreis-Fit (RMS 1,0–1,2 px), **~30–40** brauchbar | 12-Frame-Stichprobe, Kreis-Fit |
| Genauigkeit | 1–3″/Achse, **2 DOF**; Roll schwach; 2–10″ absolut gegen DEM | Sonke & Robinson 2026, DOI `10.3847/PSJ/ae32f5` |
| CDN-EDR-Asset | **kein Vollbild** — 677×832 Spaltenmittel (`M10E`) | Asset-Header |

NSSDCA-Zitat verifiziert: *„Exposure time ranged from 3 ms to 12 s … 700 scan lines with 832 picture
elements per line … 1500 mm focal length and 0.5 degree field of view."*

**Folge für G2:** Der Unabhängigkeits-Pfad ist **Limb-basiert und 2-DOF** — nicht Stern-basiert.
Für Roll braucht es eine zweite Quelle (CK/Reseaus). Die 811-Zahl ist Robinsons Pose-Subset, nicht
die Gesamtzahl der M10-Bilder (NSSDCA nennt „over 2,800").

## Was konkret noch fehlt (vor dem Bau)
1. **Vollbilder** statt des Spaltenmittel-CDN-Assets (PDS-Volumes `mvm_*`).
2. **Validierter Limb-Fit** gegen den DEM inkl. Verzerrungs-/Reseau-Modell.
3. **Roll-Quelle** (ISIS-CK/Reseaus) für die dritte Achse.
4. **Stern-Grenzgröße** der Vidicons (Thorpe 1975 paywalled) — nur falls Sternframes doch auftauchen.
5. DEM-basierte **absolute** Genauigkeit durchrechnen (nicht nur Stichprobe).

**Beleg:** dieser Record + die Auftrags-Änderung. Zitate oben, gemessen 2026-10-18.
