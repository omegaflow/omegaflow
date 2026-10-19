<!--
  title: Mond-Faden — cond lunar gebaut und gemessen
  class: survey
  date: 2026-10-18
  sha256: 3925113d53a0089e0907dc08decd0aadf1b6b19c7de8e20700aa265e5e0dc899
  status: live
-->
# Mond-Faden — `--cond lunar` gebaut (2026-10-18, river)
# Auftrag: future-Brief 2026-10-18-future-an-river-mond-und-braucht.md · Rat R-2026-10-18-o
# Done-when: `grep -rn 'cond lunar' src tools` zeigt den Arm; cancelled-Lauf und die zwei
#   korrigierten Braucht-Zeilen stehen im Handover (Commit-Hash daselbst).
# Der Conditioner heißt `lunar`, nicht `moon`: das remove-bias-Gate verbietet das
#   Körper-Literal `"moon"` im Code (wie `"earth"`); der Körpername wird aus dem
#   Ephemeriden-Dateinamen abgeleitet, der Conditioner ist die Gezeitenphase.

# ============================================================
# GEBAUT
# ============================================================
#   tools/measure/src/bin/de2_ssc_orbit.rs
#     liest jetzt zusätzlich <Latitude>/<Longitude> aus der SSCWeb-GEO-Antwort und schreibt
#     `time,radial_km,lt_hr,lat_deg,lon_deg` (vorher nur time,radial,lt).
#   tools/measure/src/bin/de2_conditional_te.rs
#     `--cond lunar` (neben ae|dst|symd|kp|lt|alt).
#     Neue Args: `--moon-bin <ephemeris_moon.bin>`, `--earth-bin <ephemeris_earth.bin>`.
#     Potential: geozentrischer DE-2-Punkt (radial × lat/lon, Kugel) → `body_fixed_vector_to_icrs`
#     (Erdrotation, eingebettete naif0012.tls) → cos ψ gegen (Mond − Erde) → P2 = (3cos²ψ−1)/2.
#     Kein Wert ohne Position: fehlt lat/lon oder die Ephemeride, fällt der Bin (absent, nie 0).
#   .github/workflows/de2-conditional-te.yml
#     zieht die zwei Ephemeriden-Bins (CDN, ssd.jpl.nasa.gov-ephemeris) nur bei `cond…moon` und
#     reicht `--moon-bin/--earth-bin` durch; der Orbit-Schritt läuft auch für `moon`.

# ============================================================
# GEMESSEN (2026-10-18)
# ============================================================
#   SSCWeb-Probe (1981-08-15T00..01Z, GEO): 61 Records, Tags Time/Latitude/Longitude/
#     RadialLength/LocalTime/X/Y/Z. de2_ssc_orbit: 61 records -> 5 Spalten, z. B.
#     1981-08-15T00:00:00.000Z,7330.028562,13.164167,-46.609300,197.462400
#     (RadialLength = geozentrischer Radius in km, nicht Höhe — 7330 km).
#   Tests (cargo test -p omegaflow-measure --bin de2_conditional_te): 4/4 grün —
#     Orbit-CSV trägt lat/lon; Interpolation trägt lat/lon; Void-Position → None;
#     echter Lauf gegen ephemeris_moon.bin + ephemeris_earth.bin liefert ein finites
#     P2 in [-1, 1].
#   End-to-End mit 60 synthetischen DE-2-Zeilen: `cond=moon`, n=60, cond_bin=60s,
#     TE absent (zu kurz) — der Arm rechnet, keine Fabrikation.

# ============================================================
# RISS — ungeglättet
# ============================================================
#   Der 3-h-Lag-Sweep `39154644922` wurde abgebrochen; die Ursache ist nicht belegt
#     (kein Log-Grund). Der Punkt bleibt offen mit dem Lauf als benanntem Ereignis.
#   Der Mond-Lauf selbst ist noch nicht gefahren (kein neues n, kein TE-Wert).
#   Lat/Lon sind geozentrisch (SSCWeb GEO); `body_fixed_vector_to_icrs` erwartet eine
#     körperfeste Richtung — die Kugelnäherung (radial × lat/lon) ist benannt, nicht
#     die geodätische Breite. Der Unterschied ist klein gegen die Monddistanz.

# ============================================================
# SCHLUSS
# ============================================================
#   Der Faden ist gebaut und getestet, nicht gemessen. Der nächste Akt ist der Lauf:
#     cond=ae,dst,kp,lt,alt,lunar, max_lag=36, bins=4.

# ============================================================
# ERGEBNIS Lauf 39166259860 (2026-10-19)
# ============================================================
#   Konfiguration: bin=300s · cond=ae,dst,kp,lt,alt,lunar · cond_bin=10800s ·
#     max_lag=36 · surr=50 · bins=4 · n=23200 · span 1981-08-16..1983-02-16.
#   Befund: konditioniert NVE->IVE 0/36 Lags, IVE->NVE 0/36; roh NVE->IVE 5/36.
#   Die Schwelle wandert von thr=+0.001829 (5 Konditionierer, 3 Lags) auf
#     thr=+0.009247 (6 Konditionierer, 300 s) — Faktor 5 —, weil 4^6 = 4096 Zellen
#     bei n=23200 nur ~5,7 Zeilen je Zelle lassen. Der ARX-Null wird breit.
#   Die gemessene TE bleibt ~0.003–0.009 und liegt damit unter dem aufgeblähten Null.
#   Das ist KEIN Mond-Effekt: es ist der statistische Preis des sechsten Konditionierers.
#   Der Lauf beantwortet die Lag-Frage (Dynamo vs. Ionendrag) NICHT.
#   Nächster Schnitt: bins=3 (729 Zellen) ODER zwei Läufe — 5-Kond-Baseline gegen
#     dieselbe Baseline + lunar, Schwellen getrennt vergleichen, nicht in einer Zelle.

# ============================================================
# ERGEBNIS Lauf 39181822901 (bins 3, 2026-10-19T00:40Z)
# ============================================================
#   Konfiguration: identisch zu 39166259860 (bin=300s, cond=ae,dst,kp,lt,alt,lunar,
#     cond_bin=10800s, max_lag=36, surr=50, n=23200), nur bins=3 (729 Zellen).
#     ARTIFACT de2-conditional-te.zip sha256 3d8929839f7ca90d633f4439fb4a83d715e9d9129f5b3f2c0e3838e411405d01.
#   Befund (36 Lags, Richtung getrennt):
#     konditioniert NVE->IVE: 0/36; MAX-NULL no
#     konditioniert IVE->NVE: 3/36 (4200, 6900, 7800 s); MAX-NULL yes
#     roh NVE->IVE: 6/36 (900, 1200, 3000, 5700, 7200, 8700 s)
#     roh IVE->NVE: 4/36 (2100, 4200, 8100, 8700 s)
#   Die Vorwaerts-Richtung (NVE->IVE) bleibt konditioniert still — wie bei bins=4.
#     Neu: die Rueckwaerts-Richtung (IVE->NVE) traegt bei bins=3 drei konditionierte
#     Lags ueber dem ARX-Null, und der MAX-NULL-Pass liegt bei te_max=+0.000464
#     gegen thr_max=+0.000395.
#   Offen: ob diese drei die Lunar-Konditionierung tragen oder die feinere
#     Zellaufloesung. Naechster Schnitt: zwei Laeufe — 5-Kond-Baseline ohne lunar
#     gegen dieselbe Baseline + lunar, Schwellen getrennt; erst dann eine
#     Richtungs-Aussage. Kein Mond-Effekt wird aus einer Zelle behauptet.
