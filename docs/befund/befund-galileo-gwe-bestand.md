<!--
  title: Befund — Galileo-GWE-Bestandsaufnahme: gll.rss-ATDF vermessen + Vorfilter der eigenen Rausch-Kurve (≤5 AU)
  class: befund
  date: 2026-09-05
  sha256: 822737e8fb163705f2709f6a08fb19e2b72d751068c00745350ac9b69341babd
  status: done
  antwortet-auf: docs/auftrag/auftrag-quiet-zone-uebertragung.md
  see-also: docs/auftrag/auftrag-quiet-zone-vorfilter.md docs/befund/befund-voyager-roh-doppler-zugang.md docs/reference/woo-armstrong-1979-jgr-abstract.md docs/reference/pioneer-anomaly-lrr-2010-4.txt docs/TODO.md
-->

# Befund: Galileo-GWE-Bestandsaufnahme — gll.rss-ATDF vermessen + Vorfilter der eigenen Rausch-Kurve (≤5 AU)

## Kurzfassung

Der Vorbefund (Vorfilter 2026-09-04) trug eine falsche Adresse: **ein PDS4-Bündel
`urn:nasa:pds:galileo.rss` existiert nicht.** Eigenhändig gemessen (ds-view +
zentrale PDS-Registry, beide `0 Treffer`; ds-view wörtlich „No Bundle Information
found in the registry"). Damit sind auch die dort zitierten Kollektionen
`data_trk225_atdf`/`data_trk234_trknav`/`data_trk223_ionocal` gegenstandslos.

Was es **wirklich** gibt (PDS3, PPI-Knoten, UCLA), ist die `GO-…-RSS-…-V1.0`-Familie
(11 Datensätze). Darunter der GWE-Bestand selbst — `GO-X-RSS-1-ODR-V1.0` — und er
ist **open-loop ODR**, nicht Closed-Loop-ATDF. Der Closed-Loop-Tracking-Bestand
ist **TRK-2-25** (ATDF/TDF); das ODF-Volumen ist **TRK-2-18**. **TRK-2-34 kommt in
keinem Galileo-RSS-Bestand vor** (0 Dateien, 0 Bytes). Die „TRK-2-25 vs -2-34"-Frage
des Auftrags ist damit gemessen beantwortet: 6,3 GB TRK-2-25, 0 TRK-2-34, dazu
0,16 GB TRK-2-18.

Vorfilter-Verdikt (Galileos eigene Rausch-Kurve): die **Stabilisierungs-Schranke**
besteht (Dual-Spin, passiv — kein 3-Achsen-Selbst-Rauschen). Die **Rausch-Geometrie**
ist medium-getrieben (S-Band, Plasma — die Distanz-Achse greift). Aber die
Distanz-Achse ist auf **≤5 AU komprimiert**: Galileos ganze Bahn liegt in der
plasma-dominierten Zone, ihr leisestes Fenster (~5,2 AU, Sonnenopposition) liegt
rechnerisch (R^−3,45, Woo & Armstrong 1979) ~2 400× über Pioneers ruhiger Zone
(>50 AU). Die Quiet-Zone als Distanz-Regime ist nicht nachbaubar; was existiert,
ist eine **relative** „ruhige Fenster"-Achse (große Sonnenelongation + Apoapsis) —
genau die Fenster, die der GWE selbst schon belegt hat (1994/1995, 77 Tage,
open-loop ODR). Die archivierten Closed-Loop-Daten sind zudem auf die **lauten**
Fenster verschoben (Encounter + Sonnenkonjunktion). Galileo bleibt Reserve, aber
als **eigene Achse** (relativ-ruhig ≤5 AU), nicht als Quiet-Zone-Nachbau.

## Korrektur des Vorbefunds (gemessen, nicht Agenten-Wort)

- `urn:nasa:pds:galileo.rss` → ds-view: „No Bundle Information found in the
  registry." Zentrale Registry `q=(logical_identifier eq …)` → 0 Treffer für
  `galileo.rss`, `gll.rss`, `gll-rss`, `galileo_rss`, `galileo.rss.bundle`.
- Die Kollektionen `data_trk225_atdf`, `data_trk234_trknav`, `data_trk223_ionocal`
  → 0 Treffer (PPI-metadex + zentrale Registry). Absent.
- Der echte PDS4-Bestand zu Galileo-RSS:
  - `urn:nasa:pds:galileo-bundle` — Missions-**Dokument**-Bündel (8
    Dokument-Kollektionen: epd/hic/mag/mission/pls/ppr/rss/ssd); `:document-rss`
    = 4 Dateien ≈684 KB (nur PDF/Label). **Keine Tracking-Daten.**
  - `urn:nasa:pds:galileo-rss-jup-der` — 17 abgeleitete Elektronendichte-Profile
    (`.EDS`, ASCII), Epochen 1995-12-08 → 1997-08-03, ≈1,6 MB. Abgeleitet, nicht roh.

## Bestandsaufnahme — die fünf Maße

### 1. Die echte GWE-Quelle: GO-X-RSS-1-ODR-V1.0

PDS3-Volumen `GORS_9110`, Kurator PPI/UCLA, physisch
`https://pds-ppi.igpp.ucla.edu/annex/GO-X-RSS-1-ODR-V1.0/`. AAREADME wörtlich
(gelesen): „Galileo ODR Data: Gravitational Wave Experiments 1 (1994-128 to
1994-135) and 2 (1995-177 to 1995-179) … archival data produced during the
Gravitational Wave Experiment carried out using the Galileo spacecraft (in
cruise to Jupiter)". Datensatzname: „GALILEO RAW ODR GRAVITATIONAL WAVE DATA
V1.0" — **open-loop** ODR (566-Byte-Records = 166 B Header + 400 B 8-bit-Samples
X-RCP/S-RCP/X-LCP/S-LCP @ 200 sps; S-RCP trägt das Galileo-Signal). SIS:
`DOCUMENT/RSC11_11.TXT`. **Kein** TRK-Format.

### 2. Epochen

- GWE-Volumen (INDEX.TAB, gemessen): erster Start **1994-04-28T14:16:19Z** →
  letzter Stopp **1995-06-28T19:04:59Z**. AAREADME-Volumenzeile:
  `1994-118T15:16:19Z – 1995-179T19:04:59Z`. Daten an **77 UTC-Tagen** (1994: 43,
  1995: 34). Die Begleittexte widersprechen sich über die Fenstergrenzen
  (DATASET.CAT: GWE1 1994-04-28→06-11, GWE2 1995-05-20→06-02; AAREADME:
  1994-128→135, 1995-177→179; DESCRIPT: 1994 DOY 118–158, 1995 DOY 140–160) —
  die gemessene Dateiabdeckung gewinnt.
- TRK-2-25-TDF-Familie (5 Volumen): 1990-11-29 → 1997-11-18 (Earth1/Earth2-Masse
  1990, Jupiter-Okkultation 1995–1997, Satelliten-Okkultation 1996–1997,
  Satelliten-Feld/Masse 1996–1997, Sonnenwind-Szintillation 1991–1997).
- TRK-2-18-ODF (1 Volumen): 1996-07-17 → 1997-12-10 (Jovian-Ephemeride).

### 3. Stationen (DSS)

GWE-ODR: `DSN_STATION_NUMBER` aus jedem vorhandenen `.LBL` gelesen (241/241
aufgelöst; 10 fehlende Dateien unmeßbar). **Nur die drei 70-m-Stationen:**
**DSS 14 (Goldstone) 57 Dateien · DSS 43 (Canberra) 111 · DSS 63 (Madrid) 73.**
TDF/ODF-Stationen: eine Stichproben-Label (`6177179A.LBL`) trägt
`DSN_STATION_NUMBER = {14,43,63}`; der vollständige TDF/ODF-Stationsbestand ist
`pending` (nicht je Datei gezählt).

### 4. Pass-Zahl

Das Archiv kennt **keine** diskrete Pass-Einheit (eine Datei ≠ ein Pass: ein
Pass wird in Folgedateien gesplittet, zwei Stationen tracken parallel). Gemessene
Näherungen (GWE-ODR): 251 INDEX-Segmente (1994: 134, 1995: 117), davon 241
physisch vorhanden; 77 Tracking-Tage; 171 Station-Tag-Paare (DSS 14: 53, DSS 43:
68, DSS 63: 50); **≈176 zusammenhängende Empfangsbögen** (Gap ≤30 min je Station
gemergt; bis ≈178 mit den zwei Split-Dateien `5175024A/B`).

### 5. TRK-2-25 vs TRK-2-34 (und TRK-2-18) + Volumen

| Format | Volumen | INDEX-Segmente | Bytes (metadex) | Rolle |
|---|---|---|---|---|
| **TRK-2-25** (ATDF/TDF) | 5 (`GO-J`, `GO-JS`, `GO-JG`, `GO-SS`, `GO-SUN`-RSS-1-TDF) | 192 | 6 307 582 307 | Closed-Loop-Tracking |
| **TRK-2-18** (ODF) | 1 (`GO-J-RSS-1-ODF-V1.0`) | 23 | 155 964 611 | Jovian-Ephemeride |
| **TRK-2-34** | — | **0** | **0** | absent in allen Galileo-RSS-Beständen |
| open-loop ODR (GWE) | 1 (`GO-X-RSS-1-ODR-V1.0`) | 251 (241 vorhanden) | 4 504 344 660 | GWE-Rohdaten |

GWE-ODR exakt (HEAD-Census, 241 Dateien): 4 181 684 306 B; min 110 936 B,
Median 16 978 302 B, max 31 130 000 B (nominal 55 000 × 566 B). Aufteilung:
1994-Block 124 Dateien / 2 087 748 628 B, 1995-Block 117 / 2 093 935 678 B.
Archiv-interne Inkonsistenzen (gemessen): INDEX.TAB nennt 251 Zeilen, aber 10
`.ODR` fehlen physisch (nur `.LBL` übrig), `51750245.ODR` ist doppelt gelistet
und liegt physisch gesplittet als `5175024A/B` vor (nicht im INDEX); 7
INDEX-Zeilen tragen Start > Stopp (Label-Anomalie).

## Die Vorfilter-Frage: Galileos eigene Rausch-Kurve (≤5 AU)

Drei Vorfilter-Kriterien, je gemessen/eingeordnet:

1. **Stabilisierung — besteht.** Dual-Spin (Spun-Section ~3 rpm, passiv); kein
   kontinuierlicher 3-Achsen-Betrieb → kein Voyager-artiges Düsen-Selbst-Rauschen.
2. **Rauschquelle — medium-getrieben.** S-Band-only (der High-Gain-Öffnungsfehler
   zwang die Mission auf die Low-Gain-Antenne); S-Band-Doppler-Rauschen wird von
   Plasma-Szintillation dominiert → die **Distanz-Geometrie** ist da (die
   Rezept-Vorbedingung 1 greift).
3. **Distanz-Achse — auf ≤5 AU komprimiert.** Galileos VEEGA-Bahn reicht ~0,72 AU
   (Venus) bis ~5,2 AU (Jupiter). Mit dem gemessenen R^−3,45-Gesetz (Woo &
   Armstrong 1979, `docs/reference/woo-armstrong-1979-jgr-abstract.md`) spannt
   Galileos eigene Achse ~(5,2/0,72)^3,45 ≈ **900×**; ihr leisestes Fenster
   (~5,2 AU) liegt aber noch ~(50/5,2)^3,45 ≈ **2 400× über** Pioneers ruhiger
   Zone (>50 AU). Rechenwerte aus dem Literaturgesetz, keine eigene Reduktion —
   benannt als Ableitung, nicht als Messung.

**Die „ruhige Fenster"-Achse existiert — aber relativ.** Galileos ruhigste
Fenster sind Sonnenopposition + Apoapsis (maximale Sonnenelongation). Der GWE
selbst ist der Beleg: er lief 1994/1995 genau in diesen Fenstern, zur
Gravitationswellen-Suche (mHz-Band) — nicht zur Anomalie-Suche. Damit ist
Galileos „eigene ruhige Fenster"-Achse real, aber sie ist (a) ≤5 AU, (b) eine
**andere** Messung (GW-Suche, open-loop) und (c) nur 77 Tage über zwei Fenster.

**Der geschlossene Cruise-Doppler ist auf die lauten Fenster verschoben.** Die
archivierten TRK-2-25-TDF-Volumen sind Jupiter-/Satelliten-Okkultation,
Satelliten-Feld/Masse und **Sonnenwind-Szintillation** (Konjunktion — das lauteste
Plasma-Regime). Ein eigener ruhiger Cruise-Doppler (antisolar, Apoapsis) ist als
Closed-Loop **nicht** separat archiviert; er steckt nur im ODF (TRK-2-18,
Jovian-Ephemeride 1996–1997) oder wurde nie archiviert.

## Verdikt

Galileo **besteht die Stabilisierungs-Schranke** (Dual-Spin, passiv, medium-
getriebenes S-Band-Rauschen) — aber die Quiet-Zone als **Distanz-Regime** ist auf
Galileos ≤5-AU-Bahn **nicht nachbaubar**. Die Distanz-Achse ist um ~2 400× zu
kurz, um einen Pioneer-artigen Floor zu erreichen; Galileos „ruhige Fenster" sind
relative Minima (Sonnenopposition), die der GWE schon besetzt hat — als
open-loop-ODR, 77 Tage, für eine andere Fragestellung. Die archivierten
Closed-Loop-Daten sind auf die lauten Fenster verschoben. Galileo bleibt damit
**Reserve**, aber nicht als Quiet-Zone-Nachbau: als Quelle einer *eigenen*
relativ-ruhigen ≤5-AU-Achse — ein anderes Objekt, eine andere Achse, ein anderes
Verdikt als Pioneer. Die im Vorbefund behauptete offene ATDF/TRK-2-34-Quelle
existiert nicht (gemessen).

## Register-Satz

*Die ruhige Zone ist nicht die einzige Achse, auf der sich Rauschen verorten
lässt — aber jede Achse hat ihre eigene Länge. Galileos reicht von Venus bis
Jupiter, und sein leisestes Fenster ist noch immer drei Größenordnungen lauter
als Pioneers Stille. Die GWE-Daten sind der Beleg, dass Galileos ruhige Fenster
real sind — und der Beleg, dass sie einer anderen Frage gehören.*

## Offen / `pending`

- Die **empirische** Rausch-Kurve aus Galileos eigenen TRK-2-25-/TRK-2-18-Daten
  (Rauschen gegen Distanz/SEP reduzieren) ist nicht gemessen — verlangt Download
  (~6,5 GB Closed-Loop) + Reduktion; eigene Session, kein Vorfilter-Ergebnis.
- TDF/ODF-Stationsbestand je Datei (nur eine Stichprobe gemessen, {14,43,63}).
- `GO-SUN-RSS-1-ODR-V1.0`-Start (INDEX-Minimalzeile trägt korrupten „1934"-Stempel;
  Geschwister-TDF gemessen 1991-12-16).
- `GO-J-RSS-5-ROCC-V1.0`-Dateiinventar (Nachfolger PDS4 `galileo-rss-jup-der`
  stattdessen gemessen).
- Die 10 im GWE-INDEX referenzierten, physisch fehlenden `.ODR`-Dateien (Verbleib
  ungeprüft — Label vorhanden, Daten absent).

## Status

`done`. Bestandsaufnahme und Vorfilter der eigenen Rausch-Kurve abgeschlossen
(2026-09-05); die Vorbefund-Adresse `gll.rss` ist gemessen falsch (existiert
nicht) und durch den realen Bestand (PDS3 `GO-…-RSS-…-V1.0`, TRK-2-25/2-18,
open-loop-GWE) ersetzt. Galileo bleibt Reserve mit einer eigenen relativ-ruhigen
≤5-AU-Achse; die empirische Rausch-Kurve ist eine eigene Ernte-Session, kein
Vorfilter-Ergebnis.
