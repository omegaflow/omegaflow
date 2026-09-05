<!--
  title: Befund — Pioneer-1978–82-Lautheit gegen den omni2-Sonnenwind (V, HSS/CIR, IMF B)
  class: befund
  date: 2026-09-05
  sha256: 1f9967669aaff34a7f4549c9c9cfa5d3bb223cb70d5fe79fc8d8aaf4060970f4
  status: done
  antwortet-auf: docs/befund/befund-aera-treiber-solar.md
-->

# Befund: Pioneer-1978–82-Lautheit gegen den omni2-Sonnenwind (V, HSS/CIR, IMF B)

## Frage & Bindung

Die laut/ruhig-Trennung der Pioneer-Ära (P11 laut 1978–82, 2950–8891 Hz; ruhig ab 1983, 835–2113 Hz; P10 ruhig ab 1983) ist gegen f107 gemessen nicht durch die Sonnenaktivitäts-Höhe erklärt — der höhere Zyklus-22-Scheitel 1989–90 (f107 213/189) bleibt ruhig (befund-aera-treiber-solar, 2026-09-05). Offen blieb, ob die laute Ära ein **Zyklus-21-spezifisches Sonnenwind-Regime** trägt (High-Speed-Streams/CIR) oder eine **gemeinsame Empfangs-/Betriebs-Ära** (DSN-Basislinie).

Dieses Blatt misst die dritte Achse: die omni2-Nah-Erde-Sonnenwind-Reihe (stündlich V, IMF-B-Komponenten) 1973–1990. Bindung: dieselbe Quelle, aus der der Repo-Compiler `tools/harvest/src/bin/omni2_compiler.rs` schöpft — CDAWeb HAPI `OMNI2_H0_MRG1HR` (BASE `cdaweb.gsfc.nasa.gov/hapi`). Abgerufen am 2026-09-05 per curl, 18 Jahresabfragen (1973–1990), Parameter `BX_GSE1800,BY_GSM1800,BZ_GSM1800,V1800`, format=csv; gespeichert `/tmp/opencode/omni2_raw/omni2_YYYY.csv`. Kein Repo-Blatt, kein CDN — das Repo ist read-only. Parsing-Regel wie der Compiler: Zeilen ab führender Ziffer, 5 Felder; V gültig endlich, ≠ 9999, 0 < V ≤ 5000; B-Komponenten endlich, ≠ 999.9, |B| ≤ 1000. |B| = √(Bx²+By²+Bz²) je Stunde. Die Lautheits-Zahlen sind unverändert aus dem f107-Befund übernommen.

## n zuerst — omni2-Jahresreihe (gemessen)

Alle 18 Jahre parsen mit voller Stunden-Zahl (8760/8784). Die **Abdeckung** (gültige V-Stunden/Jahr) ist die Datenlage selbst: 1973–82 nV 6051–8230 (69–94 %), 1983–90 nV 2311–3897 (26–44 %) — die Nah-Erde-Missionen (IMP-8, ISEE-3) tragen die laute Ära dicht, die ruhige Ära dünn. Die Jahres-Kontraste der lauten Ära stehen auf der dichtesten Datenlage.

## Jahr × Sonnenwind × f107 × P11-Rauschen

P11-Med (Hz) und f107 Ø (sfu) aus befund-aera-treiber-solar; Sonnenwind gemessen. `p>500`/`p>600` = Anteil gültiger Stunden mit V über Schwelle (HSS/CIR-Aktivität); Vhss = Mittel der V>500-Stunden.

| Jahr | nV | V Ø | p>500 | p>600 | \|B\| Ø | f107 Ø | P11 Med Hz |
|---|---|---|---|---|---|---|---|
| 1973 | 6404 | 484,8 | 39,7 % | 20,7 % | 5,50 | 92,9 | 4700 |
| 1974 | 8230 | 525,3 | 55,6 % | 29,4 % | 5,69 | 86,1 | 4111 |
| 1978 | 7204 | 428,0 | 20,5 % | 7,0 % | 6,47 | 143,1 | 2950 |
| 1979 | 7981 | 417,4 | 16,2 % | 3,2 % | 6,92 | 191,4 | **8891** |
| 1980 | 8090 | 390,5 | 10,7 % | 2,3 % | 6,40 | 198,2 | 5425 |
| 1981 | 7975 | 424,8 | 16,0 % | 4,3 % | 7,11 | 202,1 | 6040 |
| 1982 | 6051 | 466,6 | 35,1 % | 9,9 % | 7,98 | 174,7 | 5560 |
| 1983 | 3114 | 472,6 | 34,6 % | 10,7 % | 7,10 | 119,2 | 2113 |
| 1984 | 2311 | 476,5 | 34,8 % | 14,5 % | 7,14 | 100,4 | Lücke |
| 1985 | 2772 | 466,4 | 35,7 % | 17,1 % | 5,23 | 74,2 | 996 |
| 1986 | 3673 | 452,8 | 26,5 % | 13,7 % | 5,11 | 73,6 | 835 |
| 1987 | 3767 | 429,1 | 20,7 % | 8,0 % | 5,39 | 84,9 | 1202 |
| 1988 | 3778 | 429,4 | 20,8 % | 4,6 % | 6,64 | 140,6 | 1041 |
| 1989 | 3897 | 451,3 | 25,8 % | 11,0 % | 7,47 | 213,1 | 1422 |
| 1990 | 3603 | 444,7 | 22,8 % | 7,1 % | 6,52 | 189,3 | 1879 |

Ära-Mittel (V-gültig gewichtet):

| Ära | V Ø | p>500 | p>600 | \|B\| Ø | P11-Zustand |
|---|---|---|---|---|---|
| 1978–82 | 423,2 | 18,9 % | 5,1 % | 6,97 | laut (2950–8891 Hz) |
| 1983–90 | 450,6 | 26,9 % | 10,4 % | 6,26 | ruhig (835–2113 Hz) |
| 1989–90 | 448,1 | 24,4 % | 9,1 % | 7,02 | ruhig (1422/1879 Hz) |

## Gemessen

1. **Die laute 1978–82-Ära ist die sonnenwind-ruhigste Epoche des ganzen Records — nicht ein HSS/CIR-Regime.** Das Jahresmittel V liegt 1978–82 bei 390–467 km/s (Ära-Ø 423), die tiefsten Werte der Reihe; der HSS-Anteil p>500 liegt bei 10,7–20,5 % (Ära-Ø 18,9 %), p>600 bei 2,3–9,9 %. Das lauteste P11-Jahr 1979 (8891 Hz) hat zugleich eines der langsamsten Wind-Jahre (V Ø 417, p>500 16,2 %, p>600 3,2 %).
2. **Das HSS/CIR-Regime lebt in der ruhigen Pioneer-Ära.** Die Abklang-/Minimum-Phase Zyklus 21 (1983–86) — P11 ruhig (835–2113 Hz), P10 ruhig — trägt den höchsten HSS-Anteil der Reihe (p>500 26–36 %, p>600 11–17 %). Wäre der CIR/HSS-Zustand des Windes der Treiber, müsste genau diese Ära die lauteste sein — gemessen ist sie die ruhigste.
3. **Der Zyklus-22-Scheitel (1989–90, ruhig) hat mehr HSS-Stunden als der laute Zyklus-21-Scheitel:** p>500 24,4 % vs 16,2–20,5 % (1979–81). Der f107-Befund wiederholt sich auf der Windachse: höherer Aktivitäts- UND höherer HSS-Zustand bleibt ruhig.
4. **|B| trennt nicht.** |B| Ø 1978–82 (6,97) ≈ 1989–90 (7,02); 1983–86 fällt auf 5,1–7,1 nT ohne Lautheits-Kopplung. Kein IMF-Stärke-Signal in der Ära-Trennung.
5. **Abdeckungs-Konfundierung läuft gegen die Hypothese, nicht für sie.** Die ruhige Ära 1983–90 ist nur zu 26–44 % von Nah-Erde-Stunden gedeckt (dünne Ein-Sonden-Lage); die laute Ära zu 69–94 %. Der entscheidende Kontrast — laut bei langsamem, HSS-armem Wind — steht auf der dichtesten Datenlage.

## Urteil

Der **omni2-Nah-Erde-Sonnenwind (V, HSS/CIR, IMF B) erklärt die laute Pioneer-1978–82-Ära gemessen nicht.** Die Ära ist kein Zyklus-21-High-Speed-/CIR-Regime — sie ist im Gegenteil die sonnenwind-ruhigste Epoche 1973–1990 (tiefstes V-Ø, halber HSS-Anteil der Folgejahre), und das lauteste P11-Jahr (1979) hat eines der langsamsten Wind-Jahre. Der HSS/CIR-Zustand ist eine Eigenschaft der Abklang-Phase (1983–86), die genau mit dem Übergang zu den ruhigen Pioneer-Jahren zusammenfällt. Damit bleibt von den zwei offenen Erklärungen die **gemeinsame Empfangs-/Betriebs-Ära (DSN-Epoche, Modell-/Kalibrations-Basislinie, zeitlich 1978–82)** als stehende Erklärung — der Sonnenwind (Aktivitäts-Höhe wie f107, Struktur wie omni2) ist auf beiden gemessenen Achsen widerlegt.

## Grenzen

- omni2 ist **Nah-Erde** (auf 1 AU skaliert). Pioneer bei 7–28 AU erlebt den evolvierten Wind; CIRs versteilen und verschmelzen mit der Distanz zu Stream-Interaktionen/MIRs. Ein rein distanz-evolvierter Wind-Anteil, der nur weit draußen wirkt, ist mit dieser Reihe nicht messbar — bleibt `pending`.
- Die ruhige Ära 1983–90 trägt nur 26–44 % Stunden-Abdeckung; die Jahres-V-Mittel dort sind über die verfügbaren Stunden gerechnet (keine Fabrikation für die Lücken). Die HSS-Reichheit der Abklang-Phase ist über die bekannte Sonnenwind-Klimatologie plausibel, hier aber nur auf den gedeckten Stunden gemessen.
- HSS-Schwellen (500/600 km/s) sind gesetzte Lese-Schwellen, keine physikalischen Konstanten; die relativen Jahres- und Ära-Kontraste sind davon unabhängig (V-Ø zeigt dieselbe Form ohne jede Schwelle).
- Ein Korrelations-Koeffizient über die Jahresreihen wäre Schein-Präzision (n klein, autokorreliert, Abdeckung ungleich) und wird nicht gedruckt.

## Register-Satz

*Der omni2-Nah-Erde-Sonnenwind (CDAWeb HAPI OMNI2_H0_MRG1HR, 1973–1990, 18 Jahresdateien, vollständig geparst) erklärt die laute Pioneer-1978–82-Ära gemessen nicht: Die Ära trägt das tiefste V-Jahresmittel (423 km/s) und den halben HSS-Anteil der Folgejahre (p>500 18,9 % vs 26,9 % 1983–90), das lauteste P11-Jahr 1979 hat ein langsames Wind-Jahr (V 417, p>500 16,2 %); der HSS/CIR-Zustand gehört der Abklang-Phase 1983–86 (p>500 bis 36 %), die mit dem Übergang zu den ruhigen Pioneer-Jahren zusammenfällt. Ein Zyklus-21-HSS/CIR-Treiber ist damit gemessen widerlegt; die Empfangs-/Betriebs-Ära-Basislinie (DSN) bleibt stehende Erklärung. Der distanz-evolvierte Wind weit draußen (CIR→MIR) bleibt pending — omni2 misst nur die 1-AU-Lage.*

## Status

`done` (2026-09-05). omni2 1973–1990 gemessen (HAPI, 18 Jahresabfragen, volle Stunden-Zahl, Parsing-Regel des Repo-Compilers). Lautheits-Zahlen unverändert aus dem f107-Befund. Urteil: Sonnenwind-Treiber (HSS/CIR) für 1978–82 gemessen widerlegt; Empfangs-/Betriebs-Ära steht.
