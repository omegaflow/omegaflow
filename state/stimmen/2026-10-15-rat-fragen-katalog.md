# Rat-Fragen — Katalog + Manifest (2026-10-15)

**Usus je Frage:** Forschungsschicht (`bin/all_wide`, weite Suche) → UI-Roster (Frontier-Sitze,
das Manifest unten) → Rat (fünf Stimmen durch die fünf Axiome). Die Session verifiziert jede
Behauptung am Baum; der Rat trägt das Verdikt. Der Operator reicht die Manifest-Frage bei den
Sitzen ein.

## Stand

| # | Frage | weite Suche | Manifest | Rat-Verdikt |
|---|---|---|---|---|
| 1 | Netz-Fusion + Stationsplatzierung | ✅ | ✅ `2026-10-15-netzfusion-manifest.md` | ✅ `…-rat-verdikt.md` |
| 2 | Frühwarnkette (Stufen · Metrik · Null) | ✅ (river-214/215) | ⬜ | teil (Rat 2026-10-15) |
| 3 | Aurora als dynamischer Stationsschalter | ✅ (river-216) | ⬜ | ⬜ |
| 4 | Schumann-Antrieb (Blitz vs. geomagnetisch) | ✅ (river-216) | ⬜ | ⬜ |
| 5 | Null-Form des Netz-End-Prüfers | ✅ (river-217) | ⬜ | ⬜ |
| 6 | Blocklängen-Regel (Blatt-Kalibrierung) | ✅ (river-217) | ⬜ | ⬜ |
| 7 | Aktuator-Richtung (Σω=0: Stille oder Partiale) | ✅ (river-217) | ⬜ | ⬜ |

---

# Manifest-Kopf (mit **jeder** Frage zusammen einreichen)

**An dich, Frontier-Modell.** Antworte als **Urteil, nicht als Erwartung.** Kein Aufsatz —
Verdikt, Begründung, benannte Risse.

**Die fünf Axiome (der Maßstab).**
1. **A = A** — die Messung ist die Sache selbst; Fabrikation, Defaults, Fallbacks bleiben absent;
   nichts Offenes steht als 0.0, sondern `pending`.
2. **0 geehrt** — Null nur, wo sie die physikalische Wahrheit ist; sonst `pending`.
3. **Alle Wesen gleich** — jedes Gewicht kommt aus der Messung, nie aus Rang, Name oder Default.
4. **Die Linse ist ein ethischer Akt** — jede Regel wird gewogen: spricht die Messung oder der Gradient?
5. **Konsens des Operators** — die Maschine fragt; der Operator entscheidet.

**Die fünf Stimmen (jede einmal).** *Mountain* (Quellen/Register/Datenbestand — kein Verdikt ohne
Messung) · *River* (Membran/Rhythmus/Ethik — das Recht zu verweigern) · *Mycelium* (Netz/CI/Fluss) ·
*Sensory* (Sinnesorgane/Hardware/Konsens — Korrelat ≠ Erleben) · *Future* (Mensch als Adresse —
ehrliche Registrierung, das Morgen erbt das Heute).

**Form.** Keine Spekulation. Jeden Riss benennen, keinen glätten. Sag es, wo eine Doktrin
geschlagen ist.

---

## Frage 1 · Netz-Fusion und Stationsplatzierung  *(Manifest vorhanden)*

Siehe `2026-10-15-netzfusion-manifest.md`. Stand: Runde gelaufen, Verdikt gefallen (Min-Fusion
geschlagen), Messschritt läuft.

## Frage 2 · Frühwarnkette Sonne→Erde

**Frage.** Trägt die Kette als gemessene **Vorlauf↔Sicherheit-Leiter**, und wie wird jede Kante
gemessen?

**Gemessener/gelesener Stand.** Jede Stufe ist Ziel *und* Prädiktor der nächsten: aktive Region
(Tage) → Flare/EUV (8 min) → CME/Sonnenwind bei L1 (~25–60 min) → Magnetosphären-Kopplung →
Substurm/Elektrojet (~min) → Boden-dB/dt → GIC (~s). Der Apparat trägt die Bausteine (TE-Maschine,
GOES XRS, OMNI, INTERMAGNET dB/dt). Recherche: Sun→Erde-Ketten und Vorlauf-Modelle existieren
(NOAA SWPC, ESA, StormFocus, LiveWire, Wing & Johnson 2019 TE mit Lag); die *ganze* Kette als
gemessene Vorlauf↔Sicherheit-Leiter (Sicherheitszuwachs = **inkrementelle** TE über die Vorstufe,
mit Null) wurde nicht gefunden. Früheres Rat-Verdikt: die Kette trägt als **Diagnose-/
Notwendigkeits-Audit**, nicht als gemessene Kette (die Leiter als Ganzes `pending`); schwächstes
Glied = Riss (empirisch Boden, intrinsisch L1→Magnetosphäre/Bz).

**Was die Antwort tragen muss.** (1) Welche Stufen, welche Fenster, welcher Vorlauf je Stufe.
(2) Die Metrik je Kante (Vorlauf **und** inkrementelle TE, mit Null). (3) Die Verkettungsregel.
(4) Das schwächste Glied (Riss, ungeglättet). (5) Der nächste begrenzte Messschritt.

## Frage 3 · Aurora als dynamischer Stationsschalter

**Frage.** Trägt die Polarlicht-Oval-Lage als **dynamischer Stationsschalter** (welche Stationen
liegen je Zeit unter dem Strom), oder ist sie als Vorhersage unbrauchbar?

**Gemessener/gelesener Stand.** Die Aurora ist eine **Schwester**-Erscheinung des Sturms (gleiche
Ursache), kein Vorläufer — der nutzbare Vorlauf Boden-dB/dt→GIC ist kurz. Messbar wäre der Lag
zwischen Aurora/AL-Onset und Boden-dB/dt. Physik (Weygand 2021): dB/dt dehnt sich nach Onset N/W/O
aus, kaum S.

**Weite Suche (river-216, `bin/all_wide`, Spill `…1792059953_auroral_oval_location_substorm_onset_mag.txt`).**
- **Die Oval-Lage ist nowcast-bar:** OVATION Prime / **PrecipNet** (McGranaghan et al. 2021,
  `10.1029/2020sw002684`) liefern die Partikel-Prezipitation/Oval-Nowcast; unser Ketten-Zeuge
  **OVATION-AACGM** (Zhao 2026, Zenodo 19651247, river-215) ist derselbe Arm; NOAA führt ein
  operatives Aurora-Tutorial (Oval = Funktion von Kp/Bz/Sonnenwind).
- **Der Vorlauf-Riss ist dokumentiert:** Lyons et al. 2013 (`10.1002/jgra.50384`, „Distinction
  between auroral substorm onset and traditional ground magnetic onset signatures") trennt auroralen
  und bodenmagnetischen Onset — genau die Frage, ob das Oval **führt** oder **folgt**.
- **Onset relativ zur Harang-Diskontinuität** (Weygand et al. 2008, `10.1029/2007JA012537`) — die
  Position des Onsets unter dem Stromsystem.
- **Oval-Mapping ist ein Riss:** „Problems with mapping the auroral oval" (2015, `10.1186/s40623-015-0336-6`);
  „Substorms on a contracted auroral oval" (2023).
- **Direkt verwandt:** „Forecasting auroras from regional and global magnetic field measurements"
  (2016) — der Schalter als Vorhersage.
**Was die Antwort tragen muss.** (1) Trägt der Schalter (ja/nein + warum). (2) Die Auswahlregel
(welche Station je Zeit, aus welchem Nowcast). (3) Der ehrliche Vorlauf-Riss (führt oder folgt das
Oval?). (4) Der nächste Messschritt.

## Frage 4 · Schumann-Antrieb — Blitz oder Geomagnetik?

**Frage.** Regelt geomagnetische Aktivität die Schumann-Frequenzen sichtbar, und in welcher
Richtung — oder ist es reines Blitzgeschehen?

**Gemessener/gelesener Stand.** Die Reihe liegt im CDN (HTTP 200); der Matrix-Deskriptor
`schumann_geomag` (Treiber `abk_dbdt_pt1h`, Ziele freq1/2/3, `expect cells 3`, Surrogat 10) und
der Job stehen. **Der Konfund:** globales Blitzgeschehen treibt Schumann **und** die Ionosphäre;
ohne einen Blitz-Kanal kann die reine TE(abk→f) den Antrieb nicht vom Konfund trennen.

**Weite Suche (river-216, `bin/all_wide`, Spill `…1792059867_Schumann_resonance_geomagnetic_activity_….txt`).**
Der geomagnetische/ionosphärische Anteil ist **gemessen und dokumentiert**, aber klein gegen den
Blitzantrieb:
- **Geomagnetische Sturmverstärkung** der Schumann-Amplitude, **Dosis-Wirkung** (Sierra-Nevada-ELF-
  Archiv, ESSOAr `10.22541/essoar.15002103/v1`) — der geomagnetische Link existiert, gerichtet
  (Sturm → Verstärkung).
- **Solarzyklus-/Ionosphären-Abhängigkeit** von SR-Intensität/Frequenz (Eskdalemuir; Liu et al. 2023
  Review `10.3390/rs15143557`); die **Ionosphäre** ist der gemeinsame Angriffspunkt.
- **Der Blitzantrieb ist quantifizierbar, unabhängig von der Geomagnetik:** Bozóki et al. 2023
  „Day-to-Day Quantification of Changes in Global Lightning Activity Based on Schumann Resonances"
  (`10.1029/2023jd038557`); Shvets/Hobara/Hayakawa 2010 (Dreistations-SR → Blitzverteilung);
  Hayakawa 2023 (integrierte SR-Intensität als Blitz-Indikator). **D. h. der Konfund ist trennbar:**
  ein unabhängiger Blitz-Proxy (aus SR selbst oder WWLLN/OTD) erlaubt den geomagnetischen Rest zu
  isolieren.
- **Ionosphärische Störung durch Transient Luminous Events** (Qiu et al. 2024, ACP 24) — der
  Blitz→Ionosphäre→SR-Pfad unmittelbar.
**Der Riss bleibt:** die reine TE(abk→f) ohne Blitz-Kanal kann den Antrieb nicht vom Konfund trennen.
**Was die Antwort tragen muss.** (1) Trägt die Messung ohne Blitz-Kanal (ja/nein). (2) Falls nein:
welcher Blitz-Kanal, aus welcher Quelle (SR-eigener Blitz-Proxy vs. WWLLN/OTD). (3) Die Null-Form.
(4) Der nächste Schritt.

## Frage 5 · Null-Form des Netz-End-Prüfers

**Frage.** Welche Surrogat-Null trägt die GIC↔Onset-Bin-Relation (MCON/`bpa_gic`-gegen-FMI)?

**Stand.** `bpa_gic` deckt den MCON-Zeitraum nicht (4-Tage-Fenster; kein Archiv, nur FOIA); die
Relation läuft über die FMI-Reihe. Vor jeder Behauptung steht die Null-Form.

**Weite Suche (river-217, `bin/all_wide`, Spill `…1792060736_surrogate_null_test_network_detection_si.txt`).**
- **Kreisschub/Permutation als Netzwerkkonnektivitäts-Null ist Standard:** Gilson et al. 2017
  (`10.1162/netn_a_00019`) — random permutations **oder circular shifts** der Zeitreihen erzeugen die
  Null-Verteilung; kontrolliert den Typ-I-Fehler und findet Kanten besser als der parametrische Test.
- **TE-spezifisch — genau unser Fall:** Diks & Fang 2017 (`10.3390/e19070372`) — „Transfer Entropy for
  Nonparametric Granger Causality Detection: An Evaluation of Different Resampling Methods": Surrogat-
  Algorithmen **und** smoothed bootstrap werden verglichen; die Referenz für den TE-Null.
- **Eigenschafts-basierte Randomisierung für komplexe Null-Hypothesen:** Vuokko 2012 (Aalto-Thesis,
  „Testing the Significance of Patterns with Complex Null Hypotheses"). Gerichtete Graph-Surrogat-
  Tests: Chan et al. 2026 (`arXiv:2606.00758`).
- **Zirkuläre Cluster-Statistik für phasen-gekoppelte Daten:** Berther et al. 2026
  (`10.1371/journal.pcbi.1014672`, robuste kreisförmige cluster-basierte Statistik).
- **Warnung dokumentiert:** Surrogat-Pathologien und falsch-positive Null-Verwerfung
  (Schreiber/…, `cs.toronto.edu/~schmah/pubs/RaCeWaAlSch01_Surrogate.pdf`); rs-fMRI-Null-Modelle
  (Miller et al. 2018, `10.3389/fnins.2018.00551`) — die Null muss zur Hypothese passen.
- **Riss:** keine Doktrin benennt die Null **für die GIC↔Onset-Bin**; der gebaute Kreisschub ist eine
  der anerkannten Familien (Gilson/Diks), aber die Wahl unter den Familien ist eine Messung, keine
  Konvention.

**Was die Antwort tragen muss.** (1) Die Null (Kreisschub? Block? Phasen-randomisiert?). (2) Warum
sie diesen Konfund erhält/zerstört. (3) Der nächste Schritt.

## Frage 6 · Blocklängen-Regel der Blatt-Kalibrierung

**Frage.** Wie wird die Blocklänge für den Signifikanz-Null dieses Korns (ABK dH/dt, Z = 58 min)
benannt?

**Stand.** Der BCA-Z-Null über die ABK-Reihe ist gemessen (`Z = 58 min`); der Signifikanz-Null
braucht eine Blocklänge (Autokorrelations-Nulldurchgang fehlt).

**Weite Suche (river-217, `bin/all_wide`, Spill `…1792060726_block_bootstrap_block_length_selection_a.txt`).**
- **Die automatische Regel existiert und ruht auf der Autokorrelation:** Politis & White 2004,
  „Automatic Block-Length Selection for the Dependent Bootstrap" (`10.1081/etc-120028836`) —
  die Standardregel (b̂_opt aus der geschätzten Autokorrelationsstruktur, nicht aus einem
  subjektiven Nulldurchgang). Künsch & Bühlmann 1994 (`10.3929/ethz-a-001366062`); R-Paket
  `blocklength` (CRAN) implementiert sie.
- **Periodisch-korreliert / langes Gedächtnis:** Bertail & Dudek 2024 (`10.3150/23-bej1683`)
  (optimale Blocklänge für periodisch korrelierte Reihen); Kim & Kim 2017 (`10.5351/csam.2017.24.1.001`,
  Bootstrap für long-memory); das passt auf geomagnetische 1-min-Reihen (Tag/Nacht-Periodik).
- **Zirkulärer Moving-Block-Bootstrap:** Yu et al. 2025 (`10.20944/preprints202512.1112.v1`) —
  CMBB für Signifikanz lokaler Ähnlichkeit, ausdrücklich gegen Non-Stationarität.
- **Raumphysik-Kontext:** Reiff 1990 (`10.5636/jgg.42.1145`, „use and misuse of statistics in space
  physics" — bootstrap/jackknife); Mudelsee 2018 (`10.1016/j.earscirev.2018.12.005`,
  Block-Bootstrap zur Erhaltung der Autokorrelationsstruktur).
- **Riss:** `Z = 58 min` ist ein **BCA-Konfidenzniveau** (95 %-Quantil), keine Blocklänge; ob der
  Autokorrelations-Nulldurchgang der ABK-Reihe mit einer Politis-White-`b̂_opt` zusammenfällt, ist
  **nicht gemessen** — die Regel ist benannt, ihr Wert am Korn fehlt.

**Was die Antwort tragen muss.** (1) Die Blocklängen-Regel (aus dem Autokorrelations-Nulldurchgang?
aus einer physikalischen Zeitskala?). (2) Warum sie hier trägt. (3) Der nächste Schritt.

## Frage 7 · Aktuator-Richtung bei Σω = 0

**Frage.** Wenn sich die Kanäle zu null summieren (Σω = 0): **Stille** (beide Kanäle 0) oder die
**Partialen erhalten**?

**Stand.** `acoustic_pcm` spannt nach dem Vorzeichen der Σω; Σω = 0 → beide Kanäle 0 = Stille. Der
Test will die Partiale erhalten — das ist eine Richtungs-Entscheidung, kein stiller Fix.

**Weite Suche (river-217, `bin/all_wide`, Spill `…1792060732_zero_sum_phase_cancellation_preserve_par.txt`).**
- **Das Prinzip ist etabliert:** aktive Schallkontrolle / **Phasenauslöschung** — De Bortoli et al.
  2024 (`10.17743/jaes.2022.0171`, „Active Acoustics with a Phase Cancelling Modal Reverberator");
  Cheer 2024 (`10.1121/at.2024.20.2.31`, „Beyond Anti-Noise"); Guicking (aktive Schall-/Vibrations-
  kontrolle).
- **Die genau passende Form — Null-Steering mit Signalerhaltung:** „Null-Steering Beamformer-Based
  Feedback Cancellation … **With Incoming Signal Preservation**" (U. Oldenburg, `SP2018_8.pdf`):
  eine Richtung wird ausgelöscht, die andere **erhalten** — exakt die Σω=0-Frage (Auslöschung eines
  Anteils bei Bewahrung des Rests).
- **Partiale/Phasen-Kohärenz:** Laroche & Dolson 1997 (`Phase-Vocoder: About this phasiness business`)
  — die Behandlung der Phase bei Überlagerung; Phasen-Dekorrelation (Guo 2012).
- **Riss:** das ist **unser** Apparat (`acoustic_pcm`); die Literatur liefert das **Prinzip**
  (Auslöschung vs. Erhaltung), aber keine Doktrin „bei Σω=0 die Partialen erhalten". Die Entscheidung
  ist axiomatisch (A = A: jeder Kanal ist, was er ist — die Summe 0 ist keine Stille, sondern die
  Superposition), nicht aus der Literatur ableitbar.

**Was die Antwort tragen muss.** (1) Das Verdikt (Stille oder Partiale). (2) Welches Axiom es
stützt (A = A? 0 geehrt?). (3) Der nächste Schritt.
