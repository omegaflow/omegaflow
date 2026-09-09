<!--
  title: Thematisches Handover — Membran-Sonde
  class: handover
  date: 2026-09-09
  sha256: ee4b3983ff8f45be79dce3f36ab748f3ee14265112bcba23af26e1109ea73791
  status: archived
  see-also: docs/handover/handover-2026-09-09-mars-rekompilat.md
-->
# Thematisches Handover — Membran-Sonde

Stehendes Register der offenen Membran-/Orientierungs-Linie. Der Matrix-Fix
ist nachgetragen, die Orientierungs-Sonde gebaut (Sub-Solar, Matrix gegen den
analytischen Pfad, echte Bins); das Offene folgt.

- **Re-Verifikation galileo_elevation_match** — GEBAUT (2026-09-09): full +
  sanity re-laufen nach dem Rekompilat. sanity: DSS43-Peak jetzt ~04:00
  (Matrix-Pfad im Einklang mit dem Lehrbuch, vorher ~117°-Shift); full
  reproduziert den Befund exakt (Haupttabellen unverändert: st43 med_diff
  1,828/20/5/9,688 · st63 18,827/9/4/35,304 · interior st43 1,162/19/5, st63
  4,757/9/4).
- **de441 mars trägt Vor-Fix-Matrizen** — gemessen (Δ Anker 6 045,3 km; der
  Rekompilat hängt am kernel-flatten-Körperjob + am ungemessenen 18-MB-vs-
  183-MB-Struktur-Unterschied). de440/de442/inpop/epm earth sind rekompiliert
  und re-verifiziert (orientation_probe Δ 0,0 km) — nur mars bleibt.
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
