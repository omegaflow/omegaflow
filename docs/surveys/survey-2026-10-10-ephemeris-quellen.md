<!--
  title: Survey — Referenz-Landschaft für eigene Ephemeriden (was noch zu ernten ist)
  class: survey
  date: 2026-10-10
  sha256: f38576a81f8215719e4f3737408330c8f2afa1927731cb51c956f6138892960b
  status: live
  see-also: docs/handover/handover-2026-10-10-mountain-folge299.md state/stimmen/2026-10-10_mountain_ephemeris-sources-round.md
-->
# Survey — Referenz-Landschaft für eigene Ephemeriden

**Frage.** Welche Referenzen/Datensätze fehlen noch, um eine **eigene Ephemeride** zu bauen
(Verbraucher → Produzent) — priorisiert, mit offener Quelle/Lizenz?

**Durchgeführt.** 2026-10-10, Mountain. Vorbereitung `archive_search --all`; Baum-Messung
(`phi/sources.φ`); lokaler Rat + voller UI-Roster (8 Frontier-Seats + Open-Weight; Z.ai noch im
Deep Think, Lumo Limit-Wall, Kimi Login-Wall, Open-Weight-Modellwahl blockiert → `pending`).
Rohmaterial: `state/stimmen/2026-10-10_mountain_ephemeris-sources-round.md`.

## Baum-Befund (vorhanden)
Zeugen: DE / INPOP19a / EPM2021 / PETREL19 + `ephemeris_house_gate`. Rohdaten: Pioneer-10/11 Doppler-ODF,
Voyager ODR, Mars-Express/Rosetta/Magellan/Viking-ODF, `dsn_snapshot`, **LLR-CRD-Parser gebaut**
(`src/archivar/llr.rs`, Zenodo 7818557). Ferner: EOP (`eop_iers_*`, Celestrak), IONEX-TEC,
Asteroidenmassen (`massINPOP25c`, `gm_de440.tpc`), `naif0012.tls`. **Fehlend: kein `llr`-Registereintrag
(Quelle), kein ITRF, keine VLBI/ICRF3-Rohsicht, kein VMF, keine Radar-Astrometrie.**

## Schließende Liste (Roster-Konvergenz)
| # | Referenz | url | Lizenz | Baum |
|---|---|---|---|---|
| 1 | **LLR-Normalpunkte** (ILRS/CDDIS; historisch 1969–1998 in Z&N/Revised-Lunar; POLAC `TOTALOBS6913`) | `https://cddis.nasa.gov/archive/slr/data/npt/moon/` · `http://polac.obspm.fr/llrdatae.html` | offen, CDDIS mit freiem Earthdata-Login | gap |
| 2 | **ITRF2020** Stationskoords + Geschwindigkeiten (SINEX) | `https://itrf.ign.fr/en/solutions/itrf2020` | offen (Attribution Altamimi) | gap |
| 3 | **JPL Planeten-Radar-Astrometrie** (1962–) | `https://ssd.jpl.nasa.gov/planets/obs_data.html` | US-Gov PD | gap |
| 4 | **VMF1/VMF3** Troposphären-Mapping (TU Wien) | `https://vmf.geo.tuwien.ac.at/` | offen, Zitierpflicht | gap |
| 5 | **ICRF3**-Katalog (Rahmenanbindung; VLBI-Rohdaten ersetzbar) | `https://cdsarc.cds.unistra.fr/viz-bin/cat/I/352` | offen | teils (Katalog/TAP) |
| 6 | **Mondschwerefeld** GRAIL GRGM1200A (PDS) | `https://pds-geosciences.wustl.edu/` | US-Gov PD | gap |
| 7 | Optionale: PDS-Radioscience fehlender Orbiter (MESSENGER/Cassini/Juno); Gaia DR3-SSO + MPC; NAIF SPICE-Kernels für ODF-Reduktion; Sonnenkorona-Plasmamodell nahe Konjunktion; DDOR | div. | offen | teils |

## Verzichtbar (vorerst)
- **IVS/VLBI-Rohdaten / DDOR**: bei EOP aus IERS C04 + ICRF3-Katalog reicht der Rahmen; Rohaufwand unverhältnismäßig.
- **`echo.jpl.nasa.gov`** (Asteroiden-Radar): nur Small-Body-Teil, nicht Planetenephemeride.
- **LLR < 1969**: marginal.
- **Weitere ODF-Varianten**: gedeckt, sobald die Missionsklasse einmal steht.

## Riss / Randbedingungen
- **DE/INPOP/EPM haben LLR selbst mitgefittet** → O−C gegen die Zeugen sind Fit-Residuen, kein Blindtest.
- **CDDIS/Earthdata-Login** = `blocked account` (Operator/future), kein `declined`.
- Der **Beobachtungsoperator** (native Rust) ist die Voraussetzung; ohne Station (ITRF) + Troposphäre (VMF)
  + Medium (IONEX/Korona) sind LLR/Doppler nicht auf den ICRS reduzierbar.
