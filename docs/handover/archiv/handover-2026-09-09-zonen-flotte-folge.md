<!--
  title: Thematisches Handover — Tiefenphasen-Flotte, Folge-Atom: die operatorfreien Linien der Zonen-Flotte gearbeitet (660-Kante benannt, Δ-Gate gebaut, Dopplung gehoben, ETOPO1 registriert)
  class: handover
  date: 2026-09-09
  sha256: 1a60499497899e9f2c7f1f7f1c3afcd064b0e1b1ee2dde18ae04e280f346e65b
  status: live
  see-also: docs/befund/befund-2026-09-09-zonen-flotte-kante.md docs/befund/befund-2026-09-09-zonen-flotte-delta-gate.md docs/befund/befund-2026-09-09-miniseed-konsolidierung.md docs/befund/befund-2026-09-09-etopo1-registrierung.md docs/handover/archiv/handover-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-zonen-flotte.md
-->
# Thematisches Handover — Tiefenphasen-Flotte (Folge-Atom 2026-09-09)

Stehendes Register der Tiefenphasen-/Seismik-Linie nach dem Folge-Atom, das die operatorfreien Linien des Zonen-Flotte-Handovers gearbeitet hat. Die Zonen-Flotte ist gemessen; ihre Engpässe sind jetzt benannt und, wo baubar, gebaut.

## Geschlossen in diesem Atom

- **Die 660-Kante — benannt, Tiefe unaufgelöst (2026-09-09).** Das Tiefen-Gate steht: `DepthInversion { Depth, EdgeDiscontinuity, SaturatedBound, Absent }` mit exakter 660/700-Gleichheit. Der Fleet misst 24/148 geklemmte Stationen (konzentriert auf die tiefsten Katalogereignisse). Gemessen, nicht angenommen: die Exklusion ist nicht bias-neutral — mit Klemmung −0,8 km, nach Exklusion −40,8 km; der vormalige „unverzerrte −0,8 km" war selbst **Wandsättigung**. Verdikt: **unaufgelöst** — die Wand trägt den Median, nicht die Tiefe. (befund-2026-09-09-zonen-flotte-kante.md)
- **Der pP-Δ-Zweig — Faltung gemessen, Δ-Restriktions-Gate gebaut (2026-09-09).** Die eigene ak135-pP-Tabelle ist in Δ-Bändern mehrdeutig (410 km Δ30–33 … 600 km Δ37–39; 660 km ≤46° keine). Das Gate (`delta_branch`, Schwelle = gemessener glatter Gradient × 3,0) übersprang 64 Stationen; die Stations-Streuung sank 108,1 → **88,6 km**. Rest-Streuung = Pick-Rauschen (flache Lag-Tiefen-Steigung), der nächste Hebel. (befund-2026-09-09-zonen-flotte-delta-gate.md)
- **MiniSEED-Dopplung — gehoben (2026-09-09).** `laic_probe.rs` (270 Zeilen) und `mseed_measure.rs` (253 Zeilen) lesen jetzt `miniseed::decode_body`; kein Parser-Gap gemessen — die „no decodable record"-Skips (AU/S1/GE/PS) sind Datenweg-Lücken, keine Dopplung. (befund-2026-09-09-miniseed-konsolidierung.md)
- **ETOPO1-Gitter — registriert (2026-09-09).** `ETOPO1_Ice_g_gdal.grd.gz` (395 MB) trägt eine verifizierte `phi/sources.φ`-Zeile (url/format reference/sha256/ttl); die CDN-Manifestation ist CI-Duty. Zitations-Defekt benannt: `befund-2026-09-09-tsunami-eikonal` wurde nie committed — der Eikonal-Stein lebt im archivierten Handover `handover-2026-09-09-seismische-ortung-tsunami.md`. (befund-2026-09-09-etopo1-registrierung.md)
- **quake_location_probe DEPTHS-Raster — entkappt (2026-09-09).** 10 → 19 Knoten (0–700 km, ak135-Knoten über 250 hinaus); ein Tiefherd-Lauf über die Demo-Sonde ist jetzt möglich. (Klein-Änderung, `cargo check` 0/0.)

## Benannt (Pendings, nicht still)

- **Die Tiefe der tiefen Zone bleibt an der Kante unaufgelöst** — kein Code-Instrument löst die Wand; die Auflösung ist eine Modell-/Referenzfrage.
- **Head-Wave-Lücke 410/660** — direktes-P-Triplikations-Gate gegen TauP; TauP/KEB95 external `pending` (Instrument benannt).
- **Externe Tiefen-Referenz (TauP/KEB95)** — `pending`, Instrument benannt.
- **Quell-Strahlungsterm (CMT)** — `pending` ohne CMT-Lösung. (befund-2026-09-09-tiefenphasen-polaritaet)
- **W-Phase-CMT als M9-Nachfolger-Atom** — Ersteinsatz emergent; STA/LTA-Pick bleibt Einsatz-Detektor.
- **Stromboli** — als Vulkan-Lehrer `pending`.
- **Hi-net/NIED** — NIED verlangt Registrierung; JP-2011-Wellenformen liefern HTTP 204.
- **Eikonal-Löser über das volle Gitter** — Dijkstra über ETOPO1; die Voraussetzung (das Gitter am dauerhaften Ort) steht jetzt, der Löser selbst bleibt `pending`.
- **Rest-Streuung (88,6 km)** — Pick-Rauschen (Lag-Tiefen-Steigung 0,16–0,41 s/5 km); √N trägt langsam.
- **Die Erde als Sender** — Kreuzbereichs-Kalibrierung (Tonga 2022) `pending`; sub-stündlicher Druck/Infraschall (BGR-Array) als nächster Datenweg.

Gemessen und geschlossen (bleibt im Befund, nicht hier): Feldstandard Seismik — jede Komponente hat ein Feld-Äquivalent; neu ist die Einbettung (ICRS, t_ref, Signal-Kegel).
