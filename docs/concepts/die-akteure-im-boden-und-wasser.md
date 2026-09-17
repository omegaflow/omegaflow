<!--
  title: Die Akteure im Boden und Wasser — die Seismik als Multi-Akteur-Matrix (konsolidierter Plan)
  class: concept
  date: 2026-09-09
  sha256: 3bfb0c7a98918635421ce12a72e5bcda9ac8cb7453ac9d79b8171f065e542b65
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-flotte.md docs/concepts/der-kausalpfeil.md docs/concepts/blatt-papier-resultat.md docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md
-->

# DIE AKTEURE IM BODEN UND WASSER — die Seismik als Multi-Akteur-Matrix

## Der Rahmen

Ein Beben bewegt nicht nur den Boden. Es bewegt alles, was im Boden und im
Wasser steckt: den Seismographen, den Pegel, die Boje, den Brunnen, die
Ionosphäre darüber. Der Akteur-Begriff des Hauses (ein Messkanal: Wind,
Druck, Wasserstand, Bodenbewegung) ist die Linse. Die Seismik ist ein Strang
der Multi-Akteur-Matrix: dieselbe Quelle, viele Medien, ein Rahmen (ICRS/TDB).
Das ist der Kausalpfeil (`der-kausalpfeil.md`, Blatt 3 = LAIC), ausgewachsen
auf die Erde.

## Was steht (die Atome, alle mit Code)

- Seismische Ortung — ak135 + Gittersuche, Positivkontrolle bestanden (14,6 km).
- ak135 — P + S-Modell + pP/sP-Tiefenphasen (`ak135.rs`, 14 Tests).
- Tiefenphasen-Diagonale — der pP/sP-Lag als reines Tiefenmaß (5 Tests).
- Tiefenphasen-Feldpilot — echtes pP/sP-Picken an einem tiefen Ereignis
  (us10003re5, M7.5 Hindu Kush: Median 250 km gegen Katalog 231 km, +19 km,
  außerhalb des ±10-km-Gates — `depth_phase_field_probe.rs`).
- Tiefenphasen-Inversionsklasse — 1-km-Raste, gemeinsame Picking-Lib
  (`depthphase.rs`).
- pP/sP-Polarität — Freiflächen-Reflexion R_pp/R_sp abgeleitet (R_pp negativ
  im Pilotband, R_sp ≈ −1 — `ak135.rs` `free_surface_pp`).
- Die Flotte — 16 Ereignisse × Stationen, unverzerrt (+1,7 km, se 4,7 km),
  Streuung dominiert das Gate — `depth_phase_fleet_probe.rs`.
- Tōhoku-Kette — Pegel gemessen (766 km/h), Vorhersage 4/6, Eikonal/Dijkstra
  schließt die Beugung (Adak +57→−10, Hilo +82→+7 min). Wächter: Eikonal-CI-
  Run 34718368835 (completed, success, 2026-09-12) — die Schließung trägt
  die CI-Verifikation gegen ETOPO1.
- M9.1-Picker — gebaut, Streuung bleibt (W-Phase entschieden, offen).

## Was offen ist (die Reihenfolge)

1. **Die Flotte ist eine Messreihe** — 16 Ereignisse × Stationen, unverzerrt
   (+1,7 km, se 4,7 km), aber die Streuung (19 km über Ereignisse, 36 km über
   Stationen) dominiert das ±10-km-Gate — `depth_phase_fleet_probe.rs`.
   Offene Folge: die Streuung
   senken (besseres Picken / der mehrdeutige pP-Zweig bei Δ≈30°) und der
   Quell-Strahlungsterm (CMT-Lösung, `src/archivar/ndk.rs`). Die feinere Inversionsklasse
   ist gebaut (1-km-Raste, `tools/measure/src/depthphase.rs`); die pP/sP-Polarität
    ist abgeleitet (R_pp negativ im Pilotband, R_sp ≈ −1, `ak135.rs`
    `free_surface_pp`); die Tiefenmodell-Erweiterung über 250 km ist
    geschlossen (2026-09-09: `MAX_DEPTH_KM` 250 → 700, `DEPTH_KM`-Raster
    bis 700 km, Inversion bis 700 km; das Modell `ak135.dat` trägt bis
    6371 km).
2. **Stationsterm / Empfänger-Korrektur** — Wiederholung zuerst (II.KIV,
   3–5 Ereignisse aus einer Ecke, stetig=Struktur, springt=Pick); das
   +5,69-s-Residuum bleibt `offen`, kein Default.
3. **Die Erde als Sender** (Kreuzbereichs-Kalibrierung) — Tonga 2022 zuerst:
   das eine Ohr steht (NOAA CO-OPS Kwajalein 1820000, 8.732 N 167.736 E,
   `air_pressure` + `water_level` 6-min, keyless, gemessen 2026-09-17);
   Luft-Lamb (+2.4 hPa, 07:42 UTC) und Wasser-Trog (−0.578 m, 09:58 UTC)
   im selben Raster, die Kopplungszahl wird im Probe gewogen
   (`tonga_lamb_crosscheck_probe.rs`). Der Zeuge außerhalb des Bodens nagelt
   den Nullpunkt.
4. **W-Phase-M9** — entschieden, nicht gebaut.
5. **Stromboli** — Vulkan-Lehrer.
6. **CDN-Manifestation** des ETOPO1-Gitters (395 MB).
7. **MiniSEED-Dopplung** — geschlossen: `laic_probe.rs` nutzt
   `tools/measure/src/miniseed.rs` `decode_body`.
8. **Erd-Eigenmoden („die Glocke")** — registrierte Frage (Rat): freie
   Eigenmoden nach Großbeben, Meßkette miniSEED/ak135/Flotten-Stationen;
   erst Frage → Messung → Paper — kein Titel ohne Messung.

## Die Akteure (was im Boden und Wasser steckt und sich bewegt)

| Akteur | Medium | Stand |
|---|---|---|
| Seismometer (Bodenbewegung) | seismic-body/surface | Kern, steht |
| Pegel / Tide gauge (Wasserstand) | gravity | steht (Tōhoku) |
| DART-Boje (Tiefsee-Druck) | Wasser | registriert (`21414.dart`, `dart_tnf.bin`) |
| Grundwasser-Brunnen (poröelastisch) | Wasser im Boden | offen (USGS) |
| GNSS-Station (Bodenversatz) | em | RINEX steht |
| Hydrophon (NRS) | akustisch im Wasser | gehalten |
| Gravimeter | gravity | pending (SFTP) |
| Radon / Geochemie (Vorläufer) | Luft im Boden | kontestiert, `pending` |

## Die Matrix quer über die Domänen

Die Multi-Akteur-Matrix verdrahtet die Akteure paarweise (TE beide Richtungen,
Surrogat-Null, Signal-Kegel). Innerhalb der Seismik heißt das: dasselbe Beben
als Quelle im seismischen, tidalem, poröelastischen, ionosphärischen Gewebe
zugleich. Quer über die Domänen: Beben → Wasser (Tsunami) → Luft (Infraschall)
→ Ionosphäre (TEC). Nur der Lauf füllt das Blatt — was die Maschine nicht
misst, steht nicht drauf (auch nicht als 0.0).

## Das Eine, das nur wir können

Ein Zeuge außerhalb des Bodens bricht die Nullpunkt-Verwicklung: Luft und
Wasser teilen die Startzeit, nicht die Boden-Statis. Vier Uhren (Raum, Medium,
Quelle, Empfänger), alle Medien, ein Rahmen — das Residuum pro Medium zeigt den
Struktur-Fehler pro Medium. Der Beweis ist schon im Haus: die Tōhoku-Kette
schloss in einem Rahmen. Die offene Front ist die Ausweitung auf Luft und
Ionosphäre — nicht der Vorläufer.

## Die Reihenfolge (billig zuerst)

Stationsterm-Wiederholung → echtes pP/sP an tieferem Ereignis → Tonga
(Kwajalein-Ohr steht: Druck + Pegel ko-lokalisiert) → DART (offener
Ozean) → TEC (teure Ernte, Budget vorneweg).
