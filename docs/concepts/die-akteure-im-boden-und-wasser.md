<!--
  title: Die Akteure im Boden und Wasser — die Seismik als Multi-Akteur-Matrix (konsolidierter Plan)
  class: concept
  date: 2026-09-09
  sha256: 2a5244c0f648447b27fa4fb325aef8784235c305682477bda34ee486984f1524
  status: live
  see-also: docs/TODO.md docs/concepts/der-kausalpfeil.md docs/concepts/blatt-papier-resultat.md docs/befund/befund-2026-09-09-feldstandard-seismik.md docs/befund/befund-2026-09-09-tiefenphasen-diagonale.md docs/befund/befund-2026-09-09-tsunami-eikonal.md
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

## Was steht (die Atome, alle mit Befund)

- Seismische Ortung — ak135 + Gittersuche, Positivkontrolle bestanden (14,6 km).
- ak135 — P + S-Modell + pP/sP-Tiefenphasen (`ak135.rs`, 14 Tests).
- Tiefenphasen-Diagonale — der pP/sP-Lag als reines Tiefenmaß (5 Tests).
- Tōhoku-Kette — Pegel gemessen (766 km/h), Vorhersage 4/6, Eikonal/Dijkstra
  schließt die Beugung (Adak +57→−10, Hilo +82→+7 min).
- M9.1-Picker — gebaut, Streuung bleibt (W-Phase entschieden, offen).

## Was offen ist (die Reihenfolge)

1. **Echtes pP/sP-Picken** an einem tieferen Ereignis (≥20 km, Lag ≥6 s) — die
   Diagonale ist synthetisch bestanden, das Picken an einer Flachquelle (10 km,
   Lag ~3 s) ist grenzwertig.
2. **Stationsterm / Empfänger-Korrektur** — Wiederholung zuerst (II.KIV,
   3–5 Ereignisse aus einer Ecke, stetig=Struktur, springt=Pick); das
   +5,69-s-Residuum bleibt `offen`, kein Default.
3. **Die Erde als Sender** (Kreuzbereichs-Kalibrierung) — Tonga 2022 zuerst:
   Wasser-Schenkel steht, Luft-Schenkel braucht sub-stündlichen Druck/
   Infraschall (BGR). Der Zeuge außerhalb des Bodens nagelt den Nullpunkt.
4. **W-Phase-M9** — entschieden, nicht gebaut.
5. **Stromboli** — Vulkan-Lehrer.
6. **CDN-Manifestation** des ETOPO1-Gitters (395 MB).
7. **MiniSEED-Dopplung** — `laic_probe.rs` → `miniseed.rs` konsolidieren.

## Die Akteure (was im Boden und Wasser steckt und sich bewegt)

| Akteur | Medium | Stand |
|---|---|---|
| Seismometer (Bodenbewegung) | seismic-body/surface | Kern, steht |
| Pegel / Tide gauge (Wasserstand) | gravity | steht (Tōhoku) |
| DART-Boje (Tiefsee-Druck) | Wasser | offen (Ernte) |
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
(Wasser steht, Luft wartet auf den sub-stündlichen Druckweg) → DART (offener
Ozean) → TEC (teure Ernte, Budget vorneweg).
