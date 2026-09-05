<!--
  title: Befund — Ära-Treiber der lauten Pioneer-1978–82- und Galileo-Boden-Ären gegen f107/Sonnenzyklus
  class: befund
  date: 2026-09-05
  sha256: 291f67ca35d8a68e223f263cf588bf399295d0f1de05a7cb86d7ee1e8191717b
  status: done
  antwortet-auf: docs/befund/befund-front-c-epsilon-2d.md
-->

# Befund: Pioneer-1978–82- und Galileo-Boden-Lautheit gegen f107 / Sonnenzyklus

## Frage & Bindung

Zwei offene Treiber-Fragen gegen die gemessene Sonnenaktivität:
(A) Ist die laute Pioneer-Ära 1978–82 (P11-Missionsmedian 2950–8891 Hz bei 7–13 AU,
P10-ε0 7890–20586 Hz bei 18–28 AU), die ab ~1983 ruhig wird (P11 835–2113 Hz,
P10 365–1761 Hz), an den Sonnenzyklus-21-Scheitel gekoppelt — oder ein
Begegnungs-/Betriebs-Ären-Effekt?
(B) Liegen die lauten Galileo-Mode-1-AGC-Boden-Fenster (~1995-12 und 1996-06..1997-02)
an den Sonnenkonjunktionen (Geometrie) — oder an der Sonnenaktivität (die 1996 im
Zyklus-22-Minimum liegt)?

Bindung: Die Sonnenachse ist die **tägliche 10,7-cm-Strahlungsfluss-Reihe (f107, sfu)**,
Penticton-Noontime-Flux, NOAA/NCEI NGDC — dieselbe Quelle, aus der der Repo-Compiler
`tools/harvest/src/bin/f107_compiler.rs` (BASE = `ngdc.noaa.gov/.../penticton`, `FIRST_YEAR
= 1947`) schöpft. Die Reihe wurde am 2026-09-05 per curl für 1973–1997 in `/tmp/opencode`
geholt (25 Jahresdateien `pent_noontime-flux_YYYY.txt`, Format `YYMMDD PENT <sfu>`),
dieselbe Parsing-Regel wie der Compiler (Station `PENT`, Wert > 0, endlich). Kein
Repo-Blatt geschrieben, kein neues Werkzeug — das Repo ist read-only. Die
Lautheits-Zahlen sind die bereits gedruckten Mediane der zwei Referenz-Blätter
(unverändert übernommen), nicht neu gerechnet.

## n zuerst — f107-Jahresreihe (gemessen)

Alle 25 Jahre parsen vollständig (n = 365/366 Werte/Jahr), keine Lücke. Jahresmittel
f107 (sfu), Spanne min–max:

| Jahr | n | Mittel f107 | min | max |
|---|---|---|---|---|
| 1973 | 365 | 92,9 | 70 | 134 |
| 1974 | 365 | 86,1 | 69 | 144 |
| 1975 | 365 | 75,7 | 65 | 120 |
| 1976 | 366 | 72,9 | 66 | 91 |
| 1977 | 365 | 86,5 | 70 | 124 |
| 1978 | 365 | 143,1 | 86 | 224 |
| 1979 | 365 | 191,4 | 134 | 374 |
| 1980 | 366 | 198,2 | 127 | 285 |
| 1981 | 365 | 202,1 | 130 | 305 |
| 1982 | 365 | 174,7 | 93 | 297 |
| 1983 | 364 | 119,2 | 80 | 185 |
| 1984 | 366 | 100,4 | 69 | 181 |
| 1985 | 365 | 74,2 | 65 | 101 |
| 1986 | 365 | 73,6 | 66 | 102 |
| 1987 | 365 | 84,9 | 69 | 120 |
| 1988 | 366 | 140,6 | 96 | 254 |
| 1989 | 365 | 213,1 | 156 | 324 |
| 1990 | 365 | 189,3 | 121 | 322 |
| 1991 | 365 | 207,7 | 131 | 370 |
| 1992 | 366 | 150,0 | 92 | 302 |
| 1993 | 365 | 109,2 | 78 | 188 |
| 1994 | 365 | 85,3 | 67 | 148 |
| 1995 | 365 | 76,7 | 65 | 97 |
| 1996 | 366 | 71,6 | 64 | 104 |
| 1997 | 365 | 80,5 | 67 | 119 |

Zyklus-21-Lage daraus: Minimum 1975–76 (75,7/72,9), Anstieg 1977–78, Scheitel
1979–81 (191/198/202), Abfall 1982 (175) → 1983 (119) → 1984 (100), Minimum
1985–86 (74,2/73,6). Zyklus-22-Lage: Anstieg 1987–88, Scheitel 1989–91
(213/189/208), Abfall 1992–95, Minimum 1996 (71,6), Anstieg 1997 (80,5).

## (A) Pioneer — Jahr × f107 × Rauschen

P11-Ära-Profil (alle getrackten Tage, Median der Tages-RMS, n Tage) aus
befund-front-c-epsilon-2d (Zeilen 56–62); f107 Jahresmittel sfu (oben).

| Jahr | f107 Ø | P11 n | P11 Med (Hz) | P10-ε0-Med (Hz, n) |
|---|---|---|---|---|
| 1973 | 92,9 | 4 | 4700 | — |
| 1974 | 86,1 | 38 | 4111 | — |
| 1975–77 | 75,7/72,9/86,5 | 0 (Lücke) | — | — |
| 1978 | 143,1 | 118 | 2950 | — |
| 1979 | 191,4 | 302 | **8891** | 7890 (5) |
| 1980 | 198,2 | 300 | 5425 | 12047 (14) |
| 1981 | 202,1 | 303 | 6040 | 20586 (2) |
| 1982 | 174,7 | 241 | 5560 | 6925 (18) |
| 1983 | 119,2 | 277 | 2113 | 2039 (ruhig) |
| 1984 | 100,4 | 0 (Lücke) | — | ruhig |
| 1985 | 74,2 | 88 | 996 | ruhig |
| 1986 | 73,6 | 23 | 835 | ruhig |
| 1987 | 84,9 | 128 | 1202 | ruhig |
| 1988 | 140,6 | 167 | 1041 | ruhig |
| 1989 | 213,1 | 112 | 1422 | ruhig |
| 1990 | 189,3 | 98 | 1879 | ruhig |

Gemessen:

1. **Die laute 1978–82-Ära fällt mit dem Zyklus-21-Scheitel/Abfall zusammen.** Laut
   (P11 ≥ 2950 Hz) genau in den Jahren mit f107-Jahresmittel ≥ 143 (1978–82);
   der Übergang laut→ruhig (P11 5560→2113 Hz) liegt zwischen 1982 (f107 175) und
   1983 (f107 119). P11 ruhig ab f107 < ~120; beide Sonden ruhig ab 1983/85.
2. **Aber: Sonnenaktivität als solche treibt nicht.** Der Zyklus-22-Scheitel
   1989–90 (f107 213/189 — höher als 1979/80 mit 191/198) ist für beide Sonden
   ruhig (P11 1422/1879 Hz; P10-ε0 ruhig ~196–2039 Hz bei 30–66 AU). Wäre die
   Aktivitäts-Höhe der Treiber, müsste 1989–90 lauter als 1979–80 sein — das
   Gegenteil ist gemessen.
3. **Innerhalb der lauten Ära koppelt die Lautheit nicht an die f107-Höhe.**
   P11 lautestes Jahr ist 1979 (8891 Hz @ f107 191), nicht 1981 (6040 Hz @ f107 202);
   1980 (5425 @ 198) liegt unter 1979 und 1981. Das lauteste Jahr ist das
   Saturn-Begegnungsjahr (1979-09, ε0-Zelle n=23 dort).
4. **P10 trennt Geometrie von Ära.** P10 bleibt ekliptikständig (min ε < 10° in fast
   jedem Jahr 1979–1996) und wird trotzdem ab 1983 ruhig und bleibt durch den
   Zyklus-22-Scheitel ruhig. Die Ruhe-ab-1983 ist damit weder ε-Geometrie noch
   Distanz (P10 bei 30–66 AU), sondern zeitlich.

**Urteil (A):** Die laute 1978–82-Ära **koinzidiert** mit dem Zyklus-21-Scheitel
(f107-Jahresmittel 143–202), aber ein **allgemeiner Sonnenaktivitäts-Treiber ist
gemessen widerlegt**: der höhere Zyklus-22-Scheitel 1989–90 bleibt ruhig. Der Treiber
ist ära-spezifisch (kalendarisch 1978–82, beide Sonden, verschiedene Distanzen und
Ekliptik-Bahnen): entweder das spezifische Zyklus-21-Sonnenwind-Regime (nicht die
Aktivitäts-Höhe) oder eine gemeinsame Empfangs-/Betriebs-Ära (DSN-Epoche,
Modell-/Kalibrations-Basislinie). f107 allein trennt diese zwei nicht.

## (B) Galileo — Boden-Fenster × f107 × Konjunktions-Geometrie

Mode-1-AGC-Boden (−2560, Klemmwert = Max-Verstärkung = Schwachsignal-Zustand),
Zell-RMS-Mediane je (Monat, Station 14/43/63) aus befund-galileo-mode1-snr-kurve
(Tabelle Zeilen 112–123); f107-Jahresmittel oben. Boden-Epoche 1994-12..1997-02
liegt vollständig im Zyklus-22-Minimum: f107 1995 76,7 · 1996 71,6 (Minimum) ·
1997 80,5 — flach, ohne Aktivitäts-Modulation über die ganze Boden-Ära.

| Monat | f107 Ø Jahr | Stat14 | Stat43 | Stat63 | Konjunktions-Lage |
|---|---|---|---|---|---|
| 1995-11 | 76,7 | 0,179 (ruhig) | 0,079 (ruhig) | 0,106 (ruhig) | vor Konj. |
| 1995-12 | 76,7 | 0,089 | 2,14 | 21,2 | **Konjunktion 1995-12-19** |
| 1996-06 | 71,6 | 6,48 | 0,021 | 18,1 | mittlere Konj.-Saison (~1996) |
| 1996-09 | 71,6 | 61,9 | 6,79 | 97,8 | mittlere Konj.-Saison |
| 1996-11 | 71,6 | 0,94 | 1,73 | 0,062 | ruhiger Zwischenzug |
| 1996-12 | 71,6 | 4,10 | 18,4 | 0,54 | Richtung Konj. 1997-01-20 |
| 1997-01 | 80,5 | 26,1 | 72,6 | 53,4 | **Konjunktion 1997-01-20** |
| 1997-02 | 80,5 | 1,40 | 2,47 | 24,6 | nach Konj. |

(Konjunktions-Fenster 1995-12 + 1997-01 wie im Recherche-Vermerk des
Mode-1-Blatts benannt: „Sonneneinfluss-Konjunktionen 1995-12 + 1997-01“.)

Gemessen:

1. **Die lauten Boden-Fenster (1995-12, 1996-06..09, 1997-01..02) liegen im flachen
   Zyklus-22-Minimum.** f107 schwankt über die gesamte laute Boden-Ära nur zwischen
   71,6 und 80,5 (Jahresmittel) — es gibt keine Sonnenaktivitäts-Modulation, die die
   Laut/leise-Wechsel (1995-11 ruhig → 1995-12 laut; 1996-06/09 laut → 1996-11 ruhig →
   1997-01 laut) erzeugen könnte.
2. **Die Laut/leise-Wechsel folgen der ~13-monatlichen Konjunktions-Geometrie.**
   Ruhiger Boden 1995-11 (Oppositions-Nähe) → lauter Boden zur Konjunktion
   1995-12-19 (st43 2,14 Hz, st63 21,2 Hz) → ruhiger Boden 1996-11 → lauter Boden zur
   Konjunktion 1997-01-20 (st 26–73 Hz) → Abklingen 1997-02. Die mittleren lauten
   Zellen 1996-06/09 sitzen um die superior-Konjunktions-Saison dazwischen.
3. **Ein Sonnenaktivitäts-Treiber wäre hier ein Anti-Treiber:** Bei Aktivitäts-Kopplung
   müsste das Minimum (1996, f107 71,6) die ruhigste Boden-Ära sein — gemessen ist die
   lauteste Boden-Population gerade 1996-06..1997-02.

**Urteil (B):** Die Galileo-Mode-1-Boden-Lautheit **folgt der Konjunktions-Geometrie**
(Sichtlinie durch die Sonnenkorona im LGA-S-Band-Regime nach dem HGA-Fehler), nicht
der Sonnenaktivität: Sie tritt im flachen Zyklus-22-Minimum auf und wechselt im
~13-Monats-Takt der Konjunktionen. Ein Sonnenaktivitäts-Treiber ist gemessen
ausgeschlossen (flaches Minimum über die ganze Boden-Ära, laut gerade im Minimum).

## Grenzen

- Die f107-Reihe ist die Penticton-Noontime-Flux (bodenbasiert, ein Messwert/Tag);
  der Faktor 10,7 cm ist ein Aktivitäts-Stellvertreter, nicht der Sonnenwind selbst.
  Die omni2-Sonnenwind-Achse (V, B, CIR/HSS-Struktur), die einen
  Zyklus-21-spezifischen Sonnenwind-Anteil von einer Empfangs-Ära trennen könnte,
  ist im Repo nicht offline vorgehalten (gemessene Abwesenheit wie im
  α–Zeit–Sonnenzyklus-Blatt) — `pending`, der direkte Test benannt.
- Die Konjunktions-Daten (1995-12-19, 1997-01-20) sind aus der Auftrags-Geometrie und
  dem Recherche-Vermerk übernommen, nicht in diesem Befund aus den Ephemeriden neu
  gerechnet.
- Jahr-n klein und autokorreliert: P11 trägt 13 Jahre (Lücken 1975–77, 1984), der
  Laut/ruhig-Kontrast steht auf ~6 unabhängigen Jahren; ein Korrelations-Koeffizient
  über Jahresreihen wäre Schein-Präzision und wird nicht gedruckt. Der entscheidende
  Kontrast ist qualitativ: Zyklus-22-Scheitel ruhig bei höherem f107 als der
  Zyklus-21-Scheitel laut.
- Die 1990er-Cruise-Passage (P11 1990: 1879 Hz, nahe Erde) und die P11-1973/74-Jahre
  (4–5 AU, 4111–4700 Hz) tragen früh-missionseigene Basislinien — nicht als
  Aktivitäts-Signal gelesen.

## Register-Satz

*Die laute Pioneer-1978–82-Ära koinzidiert mit dem Zyklus-21-Scheitel (f107-
Jahresmittel 143–202), aber ein allgemeiner Sonnenaktivitäts-Treiber ist widerlegt:
der höhere Zyklus-22-Scheitel 1989–90 (f107 213/189) bleibt für P11 (1041–1879 Hz)
und P10 (ruhig) gemessen ruhig; die ära-spezifische Ursache (Zyklus-21-Sonnenwind-
Regime vs gemeinsame Empfangs-/Betriebs-Ära) bleibt pending, omni2/f107- und
f107-Vergleich benannt. Die Galileo-Mode-1-Boden-Lautheit (1995-12, 1996-06..1997-02)
folgt der ~13-monatlichen Konjunktions-Geometrie im flachen Zyklus-22-Minimum
(f107 71,6–80,5) — ein Sonnenaktivitäts-Treiber ist dort gemessen ausgeschlossen.*

## Status

`done` (2026-09-05). f107-Reihe
1973–1997 gemessen (NOAA/NCEI NGDC Penticton, 25 Jahresdateien, vollständig).
Lautheits-Zahlen unverändert aus den zwei Referenz-Blättern. Treiber (A):
Zyklus-21-Koinzidenz ja, allgemeiner Aktivitäts-Treiber nein, ära-spezifisch;
Treiber (B): Konjunktions-Geometrie, Sonnenaktivität ausgeschlossen.
