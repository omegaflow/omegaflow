<!--
  title: Paper-Landschaft — 35 Papiere gegen die externe Literatur (2026-10-19)
  class: survey
  date: 2026-10-19
  sha256: ebb9f9fa7458466eda48ed91d9e16bc92368797f3adf579c66f9b07a83856d22
  status: live
  see-also: docs/paper/ docs/concepts/te-literatur-matrix.md
-->
# Paper-Landschaft — 35 Papiere gegen die externe Literatur

**Gemessen 2026-10-19** in vier Parallel-Audits (je 8–9 Papiere), Kanäle: `archive_search --arxiv --openalex --crossref --semanticscholar`, je Papier zwei Suchläufe, jeder Treffer mit Host/ID und Jahr. Die vollständigen Abschnitte liegen als `paper_audit_g{1..4}.md` im Lauf-Archiv.

## Kanal-Ausfälle (gemessen, nicht stillschweigend)

| Kanal | Befund |
|---|---|
| **arXiv** | **HTTP 429 in jedem Lauf** (alle 35 Papiere, auch nach Pausen). Null Treffer — als Ausfall ausgewiesen, nicht als Null-Ergebnis. |
| **Semantic Scholar** | 429 in ~9 von 35 Läufen, trotz Key. |
| **OpenAlex** | lief; Kappung bei 252 Treffern. |
| **Crossref** | lief; Kappung bei 26 Treffern; **Jahr fehlt im Output** (Feld `issued`/`published` wird verworfen). |
| **Mehrere Flags** | `archive_search --arxiv --openalex` fährt nur **einen** Kanal (die letzte Flag gewinnt) — die Audits haben deshalb je Kanal einzeln aufgerufen. |

## Bilanz

| Urteil | Papiere |
|---|---:|
| gedeckt (tragende externe Literatur) | **15** |
| teilweise (Methode gedeckt, konkreter Befund nicht) | **18** |
| nicht gedeckt | **2** |
| **Summe** | **35** |

Die 18 „teilweise" sind kein Mangel der Papers, sondern die Signatur der Methode: die **TE-Pfeile selbst** sind in den meisten Domänen neu; gedeckt ist die Phänomenologie, nicht der gerichtete Test.

## Die 35, je Papier

| Papier | Urteil | tragende Quelle | die eine Lücke |
|---|---|---|---|
| `big-bang-echo-sheet-12` | teilweise | Cruz et al. 2025 (JCAP 2025/04/079), Sun et al. 2021 (MNRAS) | kein TE-Test CMB×Dichte |
| `cross-screening-tibet` | ja | Krug et al. 2021, Meteorol. Z. 10.1127/metz/2021/1071 | keine Himalaya-Konditionierung |
| `flyby-path-1-cold-cases` | ja | Nieto & Anderson 2009; Acedo 2017 | kein TE aus dem Solarwind |
| `flyby-path-2-preregistration-revised` | teilweise | Boutonnet et al. 2024; Dietz et al. 2026 | DE441/DE442-Rift nicht quantifiziert |
| `h0-lines-register` | ja | Perivolaropoulos 2024 (PRD 110, 123518) | — |
| `laic-arrow-direction` | teilweise | Fidani et al. 2022, Entropy 24, 359 | TEC-Empfänger nicht gemessen |
| `planet-nine-kbo-residue` | ja | Brown & Batygin 2019; Brown 2017 | — |
| `solar-seconds-matrix` | teilweise | Kennedy et al. 2013; Del Zanna & Mason 2018 | kein TE auf Sekundenkadenz |
| `tonga-lamb-crosscheck` | ja | Wright et al. 2022 (Nature); Matoza et al. 2022 (Science) | — |
| `broken-null-control` | teilweise | Rapp et al. 2001, 10.1142/S021812740100250X | TE-spezifischer 16→2-Kollaps nicht quantifiziert |
| `dark-flow-sheet-8` | ja | Planck 2013; „no signs of dark flow" 10.1063/pt.5.026899 | — |
| `flyby-path-2-addendum-2026-09-29` | teilweise | Solarwind-Propagationsvergleich 2008; ML-Timing 2021 | 1,0–1,26 h an diesem Perigee unbelegt |
| `galileo-rotor-spin-era-floor` | teilweise | Galileo-Okkultations-Doppler 1997; Coronal Radio Sounding 2008 | 52,39-mHz-Linie unabhängig unbestätigt |
| `hyperscanning-te-method` | ja | Zalesky et al. 2010 (NBS), 10.1016/j.neuroimage.2010.06.041 | — |
| `lead-geometry-direction` | teilweise | „Information Flow Among ECG Leads" 2024 (`W4404036195`) | MLII↔V1-Asymmetrie nicht repliziert |
| `probe-front-dark-matter` | ja | Pitjev & Pitjeva 2013 (EPM2011) | — |
| `sturzflut-tibet-pfeil` | ja | `W7210268981` (2026); `W7204990442` (2026) | — |
| `twenty-second-band-ground-chain` | teilweise | Pioneer-Doppler 2008/2011 | stationsfeste 44–58-mHz-Traces unbestätigt |
| `causal-arrow-preregistration` | **nein** | 10.31223/x5250r; 10.31223/x51j54 (2026) | **Prämisse bestritten**: Fels-Eis-Kaskade statt Regen→Abfluss |
| `depth-phase-echo-fleet` | ja | Tiefenphasen-Inversion etabliert | Flottenstatistik ohne externe Gegenprobe |
| `flyby-path-2-falsification-metric-addendum` | ja | Flyby-Anomalie-Literatur dicht | — |
| `gic-causal-driver` | teilweise | Huttunen et al. 2008; Manshour et al. 2021 | Kette bis dB/dt an GIC-Station unpubliziert |
| `hyperscanning-te-preregistration` | teilweise | Xu et al. 2022 | Triaden-TE-Screen mit FWER nicht publiziert |
| `nadel-v-fresh-area-dip-scan` | ja | Arnold 2005; Ĝ-Suche 2016 | — |
| `signal-cone-audit-sheet` | teilweise | Stenner et al. 2003; Boyd et al. 2009 | kein Fast-Light-Kontrollkanal |
| `terminologie-der-gegenstroemung` | ja | dicht publiziert | — |
| `uranus-rift-ephemerides` | ja | Moiseev & Emelyanov 2024 | — |
| `corona-heating-ladder` | teilweise | Schreiber 2000; Aschwanden 2008 | kein TE über eine EUV-Temperaturleiter |
| `eclipse-clock-worldlines` | teilweise | Espenak & Meeus 2006 | Fünf-Ephemeriden-Rechnung des Greatest Eclipse fehlt |
| `flyby-path-2-preregistration` | **nein** | — | Vorregistrierung ist kein Literaturgegenstand |
| `ground-sources-20s-band` | ja | Levy et al. 2009 | — |
| `jwst-disequilibrium-survey` | teilweise | SO2 2023/2024; K2-18 b 2024 | kollektive P=0,94-Aussage ohne Replikation |
| `neptune-rift-ephemerides` | teilweise | Moiseev & Emelyanov 2024; Neptune-Refit 2025 | km-Zahlen am Planetenzenrum nicht publiziert |
| `solar-cycle-dynamo` | teilweise | informationstheoretischer Sonnenzyklus 2018 | Stille gegen F10.7 nicht repliziert |
| `text-as-data-pioneer` | teilweise | PRL 2012, 10.1103/PhysRevLett.108.241101 | Text-als-Zahlensequenz gegen Doppler nicht publiziert |

**Nachtrag 2026-10-19 (gemessen):** Die GIC-Seite von `gic-causal-driver` ist
nicht mehr ungemessen. Gegen die registerführende Reihe `fmi_gic.bin` [A]
(Tagesmaximum |GIC|, 2000-01-04 → 2023-09-29, 23,74 Jahre, n = 2 145 nach
Stride 3; Surrogat-Null §3.2, zehn Surrogate, fam = 0,1877) liefert Bz→GIC
TE = 1,9729e-2 (Schwelle 1,4848e-2, **family bound**, kein Arrow), Density→GIC
TE = 1,3386e-2 (Schwelle 1,2877e-2, family bound), Speed→GIC und alle
Rückrichtungen sind silent. Ergebnis: gemessen, aber kein fam-clearing Treiber;
Details §4.8 des Papiers, Token `…2~ba777f02`.

## Was daraus folgt

1. **`causal-arrow` darf so nicht weiterlaufen.** Die Ereignisliteratur erklärt die Trishuli-Flut als Fels-Eis-Kaskade; die vorregistrierte Regen→Abfluss-Prämisse ist bestritten. Entweder ko-lokale Abflussreihe beschaffen oder die Prämisse neu vorregistrieren.
2. **`flyby-path-2-preregistration` ist kein Literaturgegenstand** — das ist kein Mangel, sondern eine Kategorie-Verwechslung im Audit; die Vorregistrierung wird gegen die Messung geprüft, nicht gegen Literatur.
3. **Die drei billigsten Schließungen:** GIC-Reihe (`gic-causal-driver`), Fast-Light-Kontrollkanal (`signal-cone`), Replikationskohorte (`hyperscanning`, Yamasaki 2026).
4. **Die LAIC-Zeile war falsch** und ist korrigiert: `te-literatur-matrix.md` behauptete, es gebe keine TE-Arbeit im LAIC-Bereich — Fidani et al. 2022 (Entropy 24, 359) widerlegt das. Der Nullsatz gilt nur für den TEC-Empfänger.
5. **Der Kanal ist der Engpass, nicht die Literatur.** arXiv fiel in 35 von 35 Läufen aus; ohne stabile Route bleibt jede künftige Recherche auf OpenAlex/Crossref gekappt (252/26 Treffer) und ohne Jahr.
