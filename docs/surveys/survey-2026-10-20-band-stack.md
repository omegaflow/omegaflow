<!--
  title: Survey — Band-Stack: Nachbarstationen unter dem Oval bündeln (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: d6108e17a0c0b2707de55c2ab5c93db6e046c633764fed7f3715f5752de7cdab
  status: done
  see-also: docs/surveys/survey-2026-10-20-sturm-komposit.md tools/measure/src/bin/band_stack_probe.rs phi/sources.φ
-->
# Survey — Band-Stack: Nachbarstationen unter dem Oval bündeln

**Anlass:** Der Sturm ist kein Punkt, sondern ein Band (Polarlichtoval). Einzelne
Bodenstationen liegen mal im Band, mal außerhalb; dieser Wechsel ist ein großer
Teil des Rauschens. Dieser Survey bündelt 20 Nachbarstationen zu einem Stack,
wählt zu jeder Minute die Stationen unter dem Oval und misst, wie viel Rauschen
verschwindet, wie viel Signal bleibt und wie oft das Verfahren auf ruhigen Tagen
fehlalarmiert.

**Grenze vorab:** Die Ovalmaske ist ein **AE/AL-Proxy**, kein kalibriertes
Ovalmodell. Die Kamera (THEMIS/ASI) ist im Haus nur für Mai 2024 vorhanden und
liegt außerhalb des Ereignisfensters 2014–2023; SSUSI ist `absent`. Der Survey
misst daher die AE/AL-Variante und benennt die Kamera als nicht verwendete
zweite Spur, nicht als stillschweigend ersetzte.

## Daten

| Faden | Quelle (lokal/Abruf) | Abdeckung | Kadenz | n |
|---|---|---|---:|---:|
| Ereignisse | `survey-2026-10-20-sturm-komposit.md` | 2014-02-18 .. 2023-12-02 | — | **166** |
| Bodenmagnetfeld | SuperMAG `data-api.php`, 20 Stationen | 2014–2023, stationsweise lückenhaft | 1 min | 166 Fenster |
| AE / AL | `wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin` | 2014-01-01 .. 2023-12-31 | 1 min | 5.258.880 |
| Dst | dito (`hour_dst`) | dito | 1 h | 87.648 |
| Ruhige Fenster | dieselben Quellen, 50 Tage | 2014–2023 | 1 min | 50 |
| THEMIS/ASI | `themis.ssl.berkeley.edu` | nur 2024-05-10 | Bild | **nicht verwendet** |
| SSUSI | nicht im Haus | — | — | **absent** |

**Stationen (20):** ABK, KIL, MUO, SOD, PEL, IVA, ALT, NOR (Fennoskandien);
YKC, C06, C09, GIM, MEA (Kanada); CMO, BET, KAV (Alaska); SKT, STF (Grönland);
TIK, CHD (Russland). Koordinaten sind AACGM-Werte aus dem SuperMAG-Katalog
(Epoche 2025). Pro Ereignis sind im Mittel **13,87** Stationen verfügbar, im Band
median **10,3** (min 5,0, max 13,4); in **166/166** Ereignissen liegen mindestens
drei Stationen im Band.

Die lokalen BGS-1-h-Bins (`abk_dbdt_1h.bin`, `sod_dbdt_1h.bin`) decken alle
166 Ereignisse, sind für die Frontausrichtung aber zu grob; der Stack nutzt
deshalb die 1-min-SuperMAG-Reihen. Wo SuperMAG für ABK/SOD früh keine Daten
liefert, bleibt die Einzelstation `absent` (n = 42 bzw. 43), nicht Null.

## Maske

Magnetische Lokalzeit aus AACGM-Länge und UT:

```
MLT = (Λ_CGM / 15 + UT) mod 24
```

Oval-Proxy (Mitternacht equatorwärts, Mittag poleward, Aufweitung mit |AL|):

```
center = 67 + 5·cos(2π(MLT − 12)/24) − 0.006·|AL|
half   = 4 + 0.004·|AL|
im Band ⇔ |Λ_CGM − center| ≤ half
```

Bei AL = −1000 nT liegt das Band um Mitternacht bei 53°..69°, um Mittag bei
63°..79°. Das ist eine transparente geometrische Näherung; sie ist **nicht**
gegen Kamera-Ovalgrenzen kalibriert. Eine Stationshöhe wird nicht verwendet:
SuperMAG schreibt `alt = 0.0` für alle 20 Stationen (20/20 nominal), eine
Höhenkorrektur der Maske gibt es daher nicht.

## Methode

1. **dB/dt:** aus den 1-min-N/E-Komponenten (NEZ) als
   `|dB/dt| = sqrt((ΔN)² + (ΔE)²) / Δt`, Einheit nT/min. Das ist ein
   vorzeichenloses Maß; positive und negative Bay-Vorzeichen werden nicht
   gegeneinander gestapelt.
2. **Maske:** Für jede Minute und jede Station wird `in_band` aus AE/AL und MLT
   bestimmt. Nur In-Band-Samples gehen in den Stack.
3. **Frontankunft:** Referenz ist das AL-Minimum im Fenster
   `[Onset − 1 h, Onset + 2 h]`. Die erwartete Ankunft an einer Station ist
   `t_front = t_AL + 0,25 h · min(MLT, 24 − MLT, 12)`; das modelliert eine
   Ausbreitung vom Mitternachtssektor nach Dämmerung/Morgen, keine gemessene
   Frontgeschwindigkeit.
4. **Stack:** Mittel über die In-Band-Stationen pro relativer Minute im Fenster
   `t_front ± 90 min`. Peak = Maximum in `±15 min`; Rauschen = robustes Sigma
   (1,4826·MAD) der Minutenbins `25..75 min` Abstand.
5. **Null 1 (ruhige Tage):** 50 Fenster aus 65 Kandidatentagen mit
   `Dst > −20 nT`, `AL > −50 nT`, `AE < 200 nT` und ≥ 5 Tagen Abstand zu jedem
   der 166 Ereignisse; dieselbe Maske, dieselbe Frontausrichtung. 500 Ziehungen
   mit Zurücklegen.
6. **Null 2 (zufällige Stationsgruppen):** 500 Ziehungen von 3–7 Stationen aus
   dem 20er-Katalog, ohne Ovalmaske, auf denselben Sturmfenstern; zusätzlich
   500 Ziehungen auf den ruhigen Fenstern.
7. **Schwellen:** 3σ ist die willkürliche Nachweisgrenze; zusätzlich werden die
   5-%-Quantile der ruhigen Verteilung (SNR und Amplitude) als kalibrierte
   Schwellen berichtet.

## Bilanz

| | Zahl |
|---|---:|
| Ereignisse gelesen / mit Stack | **166 / 166** |
| Stationen im Katalog | **20** |
| Stationen im Band pro Ereignis (Median) | **10,3** |
| Stack-SNR median (q95) | **4,59** (13,78) |
| Einzelstation ABK / SOD, SNR median | **3,07 / 2,78** |
| Rausch-Sigma Stack / ABK / SOD | **6,57 / 11,05 / 9,45 nT/min** |
| Peak \|dB/dt\| Stack median | **26,84 nT/min** |
| Detektionen Stack bei 3σ | **125 / 166 = 75,3 %** |
| Fehlalarme ruhig bei 3σ | **444 / 500 = 88,8 %** |
| Detektionen Stack bei 5-%-Amplitude | **164 / 166 = 98,8 %** |
| Fehlalarme ruhig bei 5-%-Amplitude | **25 / 500 = 5,0 %** (durch Konstruktion) |
| Detektionen Stack bei 5-%-SNR | **23 / 166 = 13,9 %** |
| Fehlalarme zufällige Gruppen ruhig bei 3σ | **339 / 470 = 72,1 %** |
| Fehlalarme zufällige Gruppen ruhig bei Amplitude | **77 / 470 = 16,4 %** |

**Rauschen:** Der Stack senkt das robuste Rausch-Sigma gegenüber ABK von
11,05 auf 6,57 nT/min (**−40,5 %**, Faktor 1,68) und gegenüber SOD von 9,45 auf
6,57 nT/min (**−30,5 %**, Faktor 1,44). Das ist deutlich weniger als √N ≈ 3,2
für N ≈ 10: benachbarte Stationen sehen korreliertes Rauschen, und die
Ovalmaske wählt nicht zehn unabhängige Sensoren.

**Signal:** Der mediane Stack-Peak bleibt mit 26,84 nT/min klar über dem
medianen Ruhe-Peak von 1,71 nT/min (Ruhe-q95 = 5,49 nT/min). Bei einer
5-%-Amplitudenschwelle werden 164 von 166 Stürmen detektiert; die zwei nicht
detektierten Ereignisse bleiben unter der Schwelle — als Datenausfall sind sie
nicht geprüft.

**SNR allein trägt nicht:** Auf ruhigen Tagen ist das Rausch-Sigma so klein
(median 0,38 nT/min), dass schon ein Ruhe-Peak von 1,71 nT/min ein SNR von ~4,5
ergibt. Die 3σ-Regel detektiert daher 88,8 % der ruhigen Fenster. Der Stack
verbessert das SNR gegenüber der Einzelstation um Faktor **1,49** (ABK) bzw.
**1,65** (SOD), aber die SNR-Schwelle ist kein brauchbarer Sturmdiskriminator.
Die Amplitude trennt sauber; die SNR-Angabe bleibt als relative Größe nützlich.

**Zufällige Stationsgruppen:** Ohne Ovalmaske liegt der mediane Sturm-SNR bei
4,21 statt 4,59 (maskierter Stack), die 5-%-Amplituden-Fehlalarmrate auf ruhigen
Tagen bei 16,4 % statt 5,0 %. Die Maske hilft also, aber sie ist kein
Wundermittel: der Gewinn ist ein Faktor ~1,1 im SNR und ~3 im Fehlalarm-Budget
auf Amplitudenseite.

## Tabelle

| Gegenstand | Urteil | tragende Quelle | die eine Lücke |
|---|---|---|---|
| Ovalmaske | **Proxy**, nicht kalibriert | AE/AL + AACGM-Länge, Formel oben | keine Kamera/SSUSI-Kreuzprobe im Ereignisfenster |
| Stack-Rauschen | **−40,5 %** gegen ABK, **−30,5 %** gegen SOD | SuperMAG 1 min, 20 Stationen | korreliertes Nachbarrauschen, kein √N |
| Signal | **26,84 nT/min** Median-Peak | dito | vorzeichenlose \|dB/dt\|, keine Bay-Polarität |
| Detektion | **98,8 %** bei 5-%-Amplitude | 166 Ereignisse | zwei schwache Ereignisse bleiben unter der Schwelle |
| Fehlalarm | **5,0 %** bei Amplitude; **88,8 %** bei 3σ | 50 ruhige Fenster, 500 Ziehungen | 3σ ist auf dieser Rauschskala unbrauchbar |
| Zufallsgruppen | **16,4 %** Fehlalarm bei Amplitude | 500 Ziehungen | ohne Maske, gleiche Stationszahl |
| Stationen im Band | **10,3** Median | Maske pro Minute | AACGM-Epoche 2025, Höhe 0,0 |

## Was quer liegt

- **Die 3σ-Regel ist irreführend.** Sie detektiert fast alle Stürme (75,3 %),
  aber auch fast alle ruhigen Fenster (88,8 %). Wer nur SNR berichtet, ohne die
  Amplitude zu nennen, verkauft Rausch als Detektion.
- **Der Stack ist kein √N-Gewinn.** 20 Stationen, aber effektiv 10 im Band und
  korrelierte Nachbarstörungen: Faktor 1,4–1,7, nicht 3,2.
- **Die Frontausrichtung ist eine Annahme.** `t_AL + 0,25 h/MLT-h` ist eine
  plausible Ausbreitungsnäherung; sie ist nicht aus den Daten gefittet und
  ersetzt keine Kamera-Front.
- **Die Ovalmaske ist eine Annahme.** Der Proxy kann Stationen am Rand des
  Bandes falsch ein-/ausschließen; die Streuung der In-Band-Zahl (5,0–13,4)
  zeigt, wie stark die Maske pro Ereignis arbeitet.
- **Die Einzelstationszahlen sind nicht deckungsgleich.** ABK/SOD haben über
  SuperMAG nur 42/43 der 166 Ereignisse; die lokalen 1-h-Bins decken mehr, sind
  aber für die Minuten-Front zu grob. Der Vergleich Stack vs. Einzelstation ist
  daher ein Vergleich auf dem verfügbaren SuperMAG-Ausschnitt, kein
  Vollständigkeitsvergleich.
- **Kamera und SSUSI bleiben offen.** THEMIS/ASI liegt nur für Mai 2024 vor
  (außerhalb 2014–2023), SSUSI ist `absent`. Eine kalibrierte Ovalgrenze aus
  Bilddaten ist mit diesem Survey nicht gemessen.
- **Höhen sind nominal.** SuperMAG schreibt `alt = 0.0`; 20/20 Stationen tragen
  keine echte Höhe. Für eine Breitenmaske ist das tolerierbar, für eine
  Höhenkorrektur der Magnetfeldprojektion nicht.

## Reproduktion

```bash
# Ereignisliste aus dem Sturm-Survey
awk -F'|' '/^\| [0-9]+ \|/ && $3 ~ /T[0-9][0-9]:[0-9][0-9]Z/ {gsub(/^ +| +$/,"",$3); print $3}' \
  docs/surveys/survey-2026-10-20-sturm-komposit.md > /tmp/opencode/band_stack/events.txt

# Ruhige Tage (schreibt quiet.txt)
./target/debug/band_stack_probe --wdc ../omegaflow-work/sources/wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin \
  --events /tmp/opencode/band_stack/events.txt --list-quiet \
  --quiet-out /tmp/opencode/band_stack/quiet.txt

# SuperMAG 1-min, je Ereignis/Station 6 h (Onset − 3 h .. + 3 h)
curl -s 'https://supermag.jhuapl.edu/services/data-api.php?fmt=json&logon=omegaflow'\
'&start=2015-03-16T21:00:00Z&extent=21600&all&station=ABK' \
  -o /tmp/opencode/band_stack/raw/2015-03-17T00:00Z_ABK.json

# Messung
./target/debug/band_stack_probe --raw /tmp/opencode/band_stack/raw \
  --wdc ../omegaflow-work/sources/wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin \
  --events /tmp/opencode/band_stack/events.txt \
  --quiet-events /tmp/opencode/band_stack/quiet.txt
```

Der Abruf ist lesend; es gab **keinen CDN-Upload**. Die 166×20-Rohantworten
(≈166 MB) bleiben lokal unter `/tmp/opencode/band_stack/raw/` und werden nicht
committet.
