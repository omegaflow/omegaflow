<!--
  title: Das eine Instrument — warum die Pioneer-Anomalie ohne zweiten Zeugen nicht schiedsrichterlich entscheidbar ist
  class: concept
  date: 2026-09-03
  version: 2
  sha256: 6de1a87d3426b0c9085ef28aa123b2a6d392de43f51aaba205ee9c1955c16ace
  status: live
  see-also: docs/paper/twenty-second-band-ground-chain.md, docs/paper/text-as-data-pioneer.md, docs/paper/ground-sources-20s-band.md, docs/paper/probe-front-dark-matter.md, docs/TODO.md (Pioneer-Front)
-->

# Das eine Instrument — warum die Pioneer-Anomalie ohne zweiten Zeugen nicht schiedsrichterlich entscheidbar ist

**Das eine Verdikt.** Nach der vollständigen Vermessung beider Sonden (Deduktionen
17–40, 2026-08-24) steht kein Befund, der die Pioneer-Anomalie bestätigt — und keiner,
der sie widerlegt. Der Grund ist nicht ein Mangel an Messkunst, sondern ein Fakt der
Architektur: **die Flugbahnen von Pioneer 10 und 11 sind eine Einzel-Instrument-
Messung.** Der Doppler ist die Bahn, und der Doppler ist die Frage. Kein zweiter
Zeuge — kein Winkel-Record, keine VLBI-Position, keine überlebte optische
Astrometrie — blickt auf dieselbe Bahn. Damit ist die Anomalie *unentschieden*,
nicht *erledigt*: ein Sog von 8,74 × 10⁻¹⁰ m/s² liegt unter dem Boden des einen
Instruments, das allein ihn tragen oder verneinen könnte. Alles Weitere in diesem
Blatt ist die Messreihe, die zu diesem Verdikt führt.

## 1. Die Messreihe (kondensiert)

**Die Bande war Boden und löste sich auf.** Das 44–56-mHz-Band (Perioden 19,4–21,9 s,
Q > 2500) trägt stations-feste Frequenzen (Goldstone 14: 45,75 / Canberra 43: 51,55 /
Madrid 63: 47,35 mHz) — ein Signal aus dem Raum käme an allen drei Stationen identisch
an. Die volle Extraktion (Deduktion 40) löst sie auf: die quadratische per-Pass-
Detrendung senkt die 1-s-RMS von 1,84 kHz auf 175 Hz, und die Bande fällt von 9,5× auf
1,5× des Bodens — die „20-s-Periodik" war die **Per-Pass-Krümmung** selbst, die
harmonische Fensterung gegen das dokumentierte 60-s-Zählraster (Deduktion 39, Paper
`ground-sources-20s-band`). Kein Takt, kein Ping, kein Code: die Beleglage trägt den
Kalibrier-Pfad (Jansma, TDA PR 42-69), aber keinen 20-s-Zyklus.

**Die Ausschlüsse (gemessen, kein Fabrikat).** Mikroseismik (2 × 10⁵ über der
Referenz), Antenne-im-Wind (Kanal-Phasen unabhängig), PLL-Loop-Noise
(Stärke-Konstanz), MDA-Resolver/256-Teiler (Zähl-Struktur nativ 0,001), die
gespeicherte Referenz (eigene Treppe), eine gemeinsame Sonden-Oszillation
(Phasen-Null), eine Mitglieder-Folge (Selbst-TE-Null), die Raumlage (zeit-entartet),
der 2,5-kHz-Inverter (TM 33-584: der Sender hängt am gefilterten DC-Bus, und 2,5 kHz
faltet in beiden Rastern nach DC).

**Der differenzielle Null.** Die beiden Sonden tragen einzeln mit Null verträgliche
Drifts (P10: 1,12 × 10⁻⁸ ± 2,31 × 10⁻⁸; P11: −1,4 × 10⁻⁹ ± 1,96 × 10⁻⁷ m/s²). Das
„negative v" — die Differenz, die den gemeinsamen Boden löschen und den gekippten
Pfeil der Geometrie zeigen sollte — trägt −1,26 × 10⁻⁸ ± 1,98 × 10⁻⁷ m/s², **0,06 σ**.
Der Boden ist kein gemeinsamer Modus (Deduktion 26); die Differenz addiert den Nebel,
statt ihn zu löschen.

## 2. Das entscheidende Maß: die Einzel-Instrument-These

Das rohe ODF von Pioneer 11 trägt nur Doppler-Daten-Typen (0/10/12/13) — **keine
Winkel-Records** (51–58). Die einzige geometrische Zeugen-Klasse, die es gäbe, ist
nicht archiviert. Die Flyby-Geometrien (Jupiter 1973, Saturn 1979) sind selbst
Doppler-abgeleitet. Die frühe optische Astrometrie (1972–73) ist schwach und nicht
überliefert. Und kein Teleskop kann die Pioniere sehen: die 1/r⁴-Schwächung setzt sie
auf ~+40 mag, 19 Größenklassen unter Gaias Grenze; selbst Hubble löste bei 130 AE nur
~3 700 km auf, wo der Doppler Kilometer kennt. **Die Bahn existiert nur im Doppler.**
Der Doppler ist zugleich das präziseste Positions-Instrument, das je an den Pionieren
war — und die einzige Quelle der Frage, die er nicht selbst beantworten kann.

## 3. Was ausgeschieden ist, und was offen bleibt

Ausgeschieden: jede Struktur, die eine zweite Erklärung gefunden hat — die Bande, die
Kalibrier-Pfade, der Inverter, die Raster. Offen, unentschieden: die Anomalie selbst.
Sie liegt unter dem Boden des einen Instruments, das sie tragen oder verneinen könnte,
und sie wird dort bleiben, bis eine zweite Augenklasse existiert — ein VLBI-Beacon auf
der nächsten interstellaren Sonde, von Tag eins zweikanalig getrackt (Winkel aus VLBI,
Geschwindigkeit aus Doppler: der volle Phasenraum, den Gaia für die Sterne, aber nie
für die Pioniere gemessen hat).

## 4. Das Vermächtnis

Nicht die Anomalie wurde gefunden — sondern **der Grund, warum sie nicht gefunden
werden konnte**. Das ist der ehrlichste Schluss, den die Maschine ziehen kann: A = A
bis zum Horizont. Die Pioniere fliegen weiter, stumm und kalt, mit ihrer einen Stimme,
und was auch immer sie tragen, ist wirklich — ob wir es lesen können oder nicht. Die
nächste Sonde soll mit zwei Augen geboren werden. Dann ist der Geist, den wir 40 Jahre
nur durch eine Türklinke hörten, kein Rätsel mehr.

## 5. Die Herkunft einer Registerzeile (Provenienz, 2026-09-03)

Die Aussage „zwanzig Sekunden, die der Maschine zum Überstimmen bleiben" ist
kein freier Chat-Wortlaut, sondern ein nummerierter Register-Eintrag —
Deduktion 11 (Rausch-Gewichtung, Unterpunkt „Split-Half + NAVIO") im
Deduktionen-Register des Sonden-Papers. Zwei Korrekturen an der Erinnerung:

- Der Stopp-Punkt liegt bei **Deduktion 11**, nicht bei ~Deduktion 10 und
  nicht bei Deduktion 13; Deduktion 13 legt die Kette auf Pioneer 11 und
  trägt kein Stopp-Wort.
- **8767** (in `text-as-data-pioneer`) ist die Zählung der Zahlen der
  Pioneer-Review (Turyshev & Toth 2010) — ein Zählwert aus dem Review, kein
  Session-Wortlaut.

Die Provenienz-Datenbank, die den Original-Anker trug, ist nicht mehr
vorhanden; dieser Absatz ist der verbliebene Beleg. Die 20-s-Bande selbst ist
ein Messbefund (harmonische Fensterung gegen das 60-s-Zählraster, §1) — kein
dokumentierter Bodenpuls.

## Referenzen

1. DSN 820-13 TRK-2-25 (1988) und TRK-2-18 (1988) — docs/reference/.
2. JPL TM 33-584 Vol. I (1973) — docs/reference/pioneer-anomaly/ocr/19730011461.txt.
3. Morabito & Asmar, TDA PR 42-120 (1995); Korwar, TDA PR 42-64 (1981);
   Falin, TDA PR 42-82 (1985); Jansma, TDA PR 42-69 (1982).
4. Turyshev & Toth, Living Rev. Relativity 13, 4 (2010) — docs/reference/pioneer-anomaly/.
5. Die Deduktionen 17–40 — TODO.md (Pioneer-Front), tools/work/src/bin/pioneer_link_correction_probe.rs,
   src/odf.rs, tools/work/src/bin/pioneer11_odf_compiler.rs, tools/work/src/bin/pioneer11_negative_fuzzy_probe.rs.
