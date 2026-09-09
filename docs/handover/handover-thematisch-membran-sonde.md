<!--
  title: Thematisches Handover — Membran-Sonde
  class: handover
  date: 2026-09-09
  sha256: ab28c2699b2f807b663f6b3acec79d39cbf0e49ca2a1b8a43cd11446aeb75c3a
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-matrix-fix-nachtrag.md docs/handover/archiv/handover-2026-09-09-finsternis-schattenortung.md
-->
# Thematisches Handover — Membran-Sonde

Stehendes Register der offenen Membran-/Orientierungs-Linie. Der Matrix-Fix
ist nachgetragen, die Orientierungs-Sonde gebaut (Sub-Solar, Matrix gegen den
analytischen Pfad, echte Bins); das Offene folgt.

- **Re-Verifikation galileo_elevation_match** (full + sanity) nach dem
  Earth-Recompile: lokal auffrischen (`data/ssd.jpl.nasa.gov/ephemeris_earth.bin`,
  Membran-Cache löschen), dann den Kontroll-Lauf re-laufen. Hypothese: die
  Haupttabellen unverändert, der Sanity-Sweep (Matrix-Pfad) jetzt im Einklang
  mit dem Lehrbuch (DSS43-Peak 04:00).
- **Getrennte CI-Lücken** (vorbestehend, nicht Matrix): TNO-Split fällt auf
  fehlendem `phi/pipeline/catalog/asteroid_gm_sb441.φ`; `--omega-g` liest
  `solar_omega_g.φ` void (beide gitignored, auf CI-Runner absent); der
  ssd-earth-bin-Struktur-Unterschied (18 MB vs 183 MB, 1400 vs 30 000 Jahre)
  ist ungemessen.
- **Orientierungs-Sonde Funktions-Check** — hidden run (`OMEGAFLOW_HIDDEN=1`),
  kein visible run.
- **witness presence** — bleibt reserviert; die Consent-Wurzel Art (c) ist
  recorded, nicht gebaut; ein maschinell gemessener Bio-Ton ist eine akustische
  Serie, nie presence (der Wal bleibt frei, namenlos).
- **feature-gate `gpu`** — eigenes Atom, `pending`: `mathematikerin` als
  `#[cfg(feature="gpu")]` + Co-Gate der main_flow-Verdrahtung + Feature-
  Propagation zum Default-Bin.
- **Membran-Reste** (aus dem Register übernommen): M02 ESP32-Firmware no_std;
  M03 Audio-Gain ohne tanh; M04 Navigation (Nebra-Kalibrierung); M07 ⌘K-Palette;
  M05/M06 Station-Sensoren als SI-4-Token; Kamera ~19k Pixel-Quellen als
  WS-Traffic-Hotspot; OPeNDAP-Integration; advective per-Quelle.

Gemessen und geschlossen (bleibt im Befund, nicht hier): der Matrix-Fix
(IAU-Produkt Rz(90°+α)·Rx(90°−δ)·Rz(W), Spalte-2-Polachse) ist nachgetragen;
die Orientierungs-Sonde misst sub-solar auf echten Bins.
