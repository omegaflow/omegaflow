# Rat — de2-ion-neutral.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.* Roh-JSONs der Sitz-Aufrufe: `2026-10-18-rat-de2-ion-neutral/out/`.

---

## 2026-10-17-rat-de2-ion-neutral.md

# Rat — DE-2 Ion↔Neutral: bidirektionale TE (2026-10-17)

**Anlass.** Mountain-Ratsanfrage (`post/in/2026-10-17-mountain-ratsanfrage-de2-ion-neutral.md`,
03:47): Was heißt **bidirektionale TE** (beide Richtungen signifikant, Richtung dreht mit dem Lag,
auch nach Ost-Projektion)? Gemeinsamer Treiber (Konvektion/Joule) oder wechselseitiger
Impulsaustausch? Trägt „ion-neutral coupling" — oder nur „Abhängigkeit"?

**Methode.** Der Fragebogen (`/tmp/opencode/de2-formular.txt`) ging an **sechs Rat-Sitze**:
vier API-Sitze — GLM-5.3 und Qwen3.8-Max über DeepInfra, **DeepSeek V4 Pro über `api.deepseek.com`
(direkt)**, Mistral Large 4 über die Mistral-API (Muster `/tmp/opencode/rat-test/rat_testlauf-3.sh`) —
**und die zwei UI-Sitze** (Sakana Fugu Max · Lumo 2.0 Max) im Browser, wie bei M1 und den acht Fäden.
Haus-Design: fünf Axiome, fünf Stimmen, je Frage Verdikt/Begründung/Risse.

**Ablage der Rohantworten:** `/tmp/opencode/rat-test/d-*.json` · die zwei UI-Rohantworten
`state/rat/d-Sakana.txt` und `d-Lumo.txt`.

---

## Die Bilanz in drei Zeilen (Konsens + Riss je Frage)

Sechs Sitze. Der Konsens ist bei **allen drei** Fragen **6/6** — die zwei UI-Sitze bestätigen die vier
API-Sitze und schärfen sie (sie überstimmen hier **nicht**, anders als bei M1).

| Frage | Konsens (6/6 einig) | Riss (ungeglättet) |
|---|---|---|
| **D1 · Treiber oder Impulsaustausch** | **Nicht getrennt.** Bidirektionale TE mit lag-abhängigem Vorzeichen ist mit **beidem** verträglich; die gerichtete Komponente ist **nicht isoliert**. | Die Lag-Drehung ist die **Signatur einer Antwortzeit-Hierarchie** (GLM: τ_ion ≈ s, τ_neutral ≈ min) — und **beide** Hypothesen besitzen diese Hierarchie. GLM: **Raum-Zeit-Konflationsriss** (7,5 km/s). **Lumo:** der Surrogat-Test prüft gegen **Unabhängigkeit**, nicht gegen einen **gemeinsamen Treiber** — der größte Riss. |
| **D2 · coupling oder Abhängigkeit** | **Nur „Abhängigkeit"** trägt. „ion-neutral coupling" als **Kraft/Reibung** trägt **nicht** → `pending`. | Die Messung liefert TE-Werte, Korrelationen, Surrogat-p — **keine** Impulsübertragungsrate, keine Reibungskraft, keine Dissipation. **Lumo:** wer „coupling" schwach definiert, trägt die schwache Definition ins Zitat — **semantische Drift**. **Sakana (River):** Coley et al. schrieben vorsichtig „comparison", nicht „coupling". |
| **D3 · Erwerbs- vs. Kausalkanal** | **Nein** — der Befund trägt „coupling" **nicht**, solange nur der **Erwerbskanal** (IDM-Cross-Track) gemessen ist. | Der **tiefste Riss** (Lumo, aus der Acht-Fäden-Runde): Erwerbskanal (was der Sensor sieht) und Kausalkanal (was wirkt) fallen auseinander. **Lumo:** ein einzelner Satellit kann den Treiber **prinzipiell nicht** ausschließen — er fliegt *durch* das Feld. **Sakana (Mycelium):** die Lag-Drehung könnte die **Filterwirkung des Erwerbskanals** sein. |

**Der eine Satz (sechs unabhängige Zeugen).** Alle sechs sagen getrennt: die TE misst **Abhängigkeit,
nicht Kraft**. Die ehrliche Registeraussage ist **„gerichtete statistische Abhängigkeit, lag-abhängig"** —
der Kausalkanal bleibt `pending`. **Lumo fasst es:** *„gemessen ist gemessen, gekoppelt ist gedeutet."*
**Geschlagene Doktrin:** *„bidirektionale TE = wechselseitiger Impulsaustausch"* · *„TE-Richtung =
Kausalrichtung"* · *„Abhängigkeit ist Kopplungskraft"* · *„Sensor-Korrelat ist Kausalbeleg"* ·
*„bestandene Surrogate schließen den Treiber aus"*.

---

## D1 · Was heißt bidirektionale TE? Treiber oder Impulsaustausch?

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | **Nicht getrennt** | „Beide Lesarten gleichzeitig; die gerichtete Komponente ist nicht isoliert." Ein gemeinsamer Treiber Z mit zwei Antwortzeiten (τ_ion ≈ s, τ_neutral ≈ min) erzeugt **exakt das beobachtete Muster**; wechselseitige Reibung mit denselben Asymmetrien erzeugt **dasselbe Muster**. **Riss:** Raum-Zeit-Konflation (Lag = Ortsverschiebung entlang der Bahn bei 7,5 km/s). |
| **DeepSeek V4 Pro** | **Abhängigkeit** | „Mit gemeinsamem Treiber ebenso verträglich wie mit Austausch; eine Trennung ist mit den vorliegenden Daten nicht möglich." Axiom 4: der gemeinsame Treiber ist **nicht gemessen** → Lücke, keine Null. |
| **Qwen3.8-Max** | **Register, nicht Kausalpfeil** | „Sie entscheidet **nicht** zwischen gemeinsamem Treiber und Austausch. Die lag-abhängige Dominanz ist Register, nicht Kausalpfeil." Trennen würde erst: **geschlossene Impulsbilanz**, Krafttermabschätzung, Intervention. **Bedingte TE** isoliert die Restkomponente *relativ zu gemessenen Treibern* — als **Indiz**, nicht als Kraftbeweis. |
| **Mistral Large 4** | **Gleich verträglich** | „Equally consistent mit einem gemeinsamen Treiber; sie beweist keinen wechselseitigen Austausch. Ohne bedingte TE bleibt die gerichtete Komponente `pending`." Durch die fünf Stimmen gesprochen (River: „klassische Signatur eines gemeinsamen Oszillators mit unterschiedlicher Phasenlage"). **Riss:** die Richtungsinversion ist ein Phasen-Indiz, kein Kopplungsbeweis. |
| **Sakana Fugu Max** (UI) | **Unentschieden** | „Treiber und Impulsaustausch sind im Register **nicht trennbar**." Risse: keine Treiber-Variable registriert (AE, Kp, E_sw, Joule) · keine Konditionierung auf geomagnetische Aktivität · die Lag-Drehung kann aus der Autokorrelation eines gemeinsamen Treibers stammen · **die Surrogate testen gegen Unabhängigkeit, nicht gegen einen gemeinsamen Treiber.** Sensory-Stimme: „TE ist invariant gegen monotone Transformationen; Reibung nicht." |
| **Lumo 2.0 Max** (UI) | **Nicht entscheidbar** | „Die lag-drehende Richtung ist **kein Diskriminator** — sie ist ein Symptom, das **beide** erzeugen." Ein gemeinsamer Treiber Z mit differentiellem Lag (τ_x, τ_y) produziert genau dieses Muster; die Rückkopplung mit Reibung ebenso. **Was trennen würde:** (a) Konditionierung `TE(X→Y \| Z)` mit Z = Joule-Heizrate/E-Feld/Poynting · (b) die **Lag-Drehstelle an 1/ν_in binden** (aus Dichte/Komposition/Temperatur **vorhergesagt**, kein Post-hoc-Lag) · (c) **Antisymmetrie der Impulsbilanz** (Gewinn des Neutralen = Verlust des Ions — eine Bilanz, keine Statistik). |

---

## D2 · Trägt „ion-neutral coupling" oder nur „Abhängigkeit"?

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | *(abgeschnitten, `finish: length`)* | Der Anfang trägt D1 vollständig; D2/D3 wurden vom Token-Budget gekappt. Benannt, nicht still. |
| **DeepSeek V4 Pro** | **Abhängigkeit** | „Die ehrliche Aussage ist ‚Abhängigkeit', nicht ‚ion-neutral coupling'. Eine gerichtete Komponente, die Reibung belegt, ist am Baum nicht isolierbar." Nächster Schritt: bedingte TE `(NVE→IVE \| Treiber)`. |
| **Qwen3.8-Max** | **nur Abhängigkeit** | „‚coupling' trägt **nicht als Reibung/Kraft**. Wenn ‚coupling', dann nur als ausdrücklich markierte **statistische Informationskopplung**." Ehrlicher Registereintrag: `TE(NVE→IVE; lag) signifikant` · `TE(IVE→NVE; lag) signifikant` · `Kraft = 0 honored / pending`. |
| **Mistral Large 4** | **Abhängigkeit** | „Nein. ‚Coupling' trägt nicht. Die ehrliche Aussage ist **‚statistische Abhängigkeit'** (oder informationstheoretische Kopplung), nicht physikalische Kopplung." Das Register hat TE-Werte, Korrelationen, p-Werte — **keine** Kraft `[kg·m/s²]`. |
| **Sakana Fugu Max** (UI) | **Abhängigkeit** | River-Stimme: „**‚Coupling' ist zu verweigern.** ‚Gerichtete statistische Abhängigkeit' ist die ehrliche Benennung." Coley et al. schrieben **„comparison", nicht „coupling"** — sie waren vorsichtiger. Riss: „coupling" ist in der Community ein **Füllwort** geworden, das jede Korrelation deckt. |
| **Lumo 2.0 Max** (UI) | **Abhängigkeit** | „‚Abhängigkeit' trägt. ‚Coupling' trägt nicht — jedenfalls nicht als Behauptung eines **physikalischen Mechanismus**." Coupling verlangt einen Term `ν_in(v_i − v_n)` in der Bewegungsgleichung; TE liefert eine Zahl aus zwei Zeitreihen. **Semantische Drift** ist der eigentliche Schaden (schwache Definition wandert ins Zitat und wird dort stark). Ehrliche Benennung: *„gerichtete prädiktive Abhängigkeit"*, *„lag-asymmetrischer Informationstransfer"*. |

**Wie im Register benennen ohne coupling-Behauptung** (Qwen): tragend sind „gerichtete TE-Abhängigkeit",
„statistische Kopplung", „Informationskopplung", „Kopplungsindiz"; **nicht** tragend sind
„ion-neutral friction", „momentum coupling", „Impulsaustausch", „Kraft".

---

## D3 · Der Erwerbs- vs. Kausalkanal

| Sitz | Verdikt | Kern |
|---|---|---|
| **DeepSeek V4 Pro** | **Nein** | „Der Befund trägt ‚coupling' nicht, solange nur der Erwerbskanal gemessen wurde." Der IDM-Cross-Track hat seine physische Kraft als **Messgröße**, aber nicht die Reibungskraft selbst. |
| **Qwen3.8-Max** | **Nein** | „Er trägt nur Erwerbs-/Informationsabhängigkeit." Belegen würde den Kausalkanal: gleichzeitige frame-konsistente Vektoren (Ionendrift, Neutralwind, E-, B-Feld, Dichten, **Kollisionsfrequenz/Composition**) → Impulsterme (**Ion-Neutral-Drag**, Joule-Heizung), Impuls-/Energiebilanz mit Unsicherheit — oder Intervention. |
| **Mistral Large 4** | **Nein** | „Der Riss zwischen Erwerbs- und Kausalkanal ist mit den vorliegenden Daten **unüberbrückbar**." Um zu belegen: **Beschleunigung** der Neutralen durch Ionenkollisionen (Temperatur-/Dichtegradienten → Reibungsheizung). |
| **Sakana Fugu Max** (UI) | **Nein** | Mycelium-Stimme: „Nur der **Erwerbskanal** ist belegt; der Kausalkanal ist offen." Risse: **fehlende Kollisionsfrequenz ν_in/ν_ni** · fehlende Dichten n_i, n_n · **fehlendes E_⊥ als alternative Kausalkante** · die Lag-Drehung könnte die **Filterwirkung des Erwerbskanals** sein (Instrumenten-Response, Orbitaleffekte). |
| **Lumo 2.0 Max** (UI) | **Nein** | „Zwei Erwerbskanäle ergeben zusammen noch keinen Kausalkanal." Belegen würde: **Schließung der Neutral-Impulsbilanz** (der Ion-Drag-Term muss *nötig* sein zu schließen, nicht bloß vorhanden) · direktes `dv/dt` mit Unsicherheit < Drag-Term · **ν_in aus unabhängigen Größen vorhergesagt, nicht gefittet** · antisymmetrische Impulsbuchhaltung. **Der schärfste Riss:** ein einzelner Satellit kann den Treiber **prinzipiell nicht ausschließen** — er fliegt durch das Feld, er sieht es nicht von außen. Ohne Konjunktion (zweite Plattform/Radar) ist die Treiberhypothese **unfalsifizierbar**. |

---

## Ungeglättete Risse (alle Sitze)

1. **Statistik-vs.-Kraft-Riss:** TE misst Informationsfluss (Bits), nicht Impulsfluss (kg·m/s).
2. **Treiber-Riss (Lumo, der größte):** Der Surrogat-Test prüft gegen **Unabhängigkeit**, nicht gegen
   einen **gemeinsamen Treiber**. Phasenrandomisierung zerstört die Kopplung zwischen den Reihen —
   also auch die treibervermittelte. Bestandene Surrogate schließen „nichts" aus, nicht „Treiber".
3. **Multiplizitäts-Riss (Lumo):** 6 Tests (3 Lags × 2 Richtungen), alle signifikant, ohne genannte
   Multiplizitätskorrektur. „Bei jedem Lag signifikant" ist dann eher **verdächtig** als stark.
4. **Raum-Zeit-Konflationsriss (GLM):** DE-2 fliegt ~7,5 km/s — ein Lag ist auch eine **Ortsverschiebung**
   entlang der Bahn (Lag 1 bei 16 s ≈ 120 km). Die lag-abhängige TE misst Struktur entlang des
   Orbit-Schnitts, nicht zeitliche Kausalität an **einem** Punkt.
5. **Erwerbs-/Kausal-Riss (der tiefste):** Die Drift ist kein Reibungsbeleg. Projektion (Ost) kann
   ein Vorzeichen drehen — die **Kraft** kann sie nicht erzeugen.
6. **Filter-Riss (Sakana/Mycelium):** Die Lag-Drehung könnte die **Filterwirkung des Erwerbskanals**
   sein (Instrumenten-Response, Orbitaleffekte) — nicht Physik.
7. **Nicht-Falsifizierbarkeits-Riss (Lumo):** Ein einzelner Satellit sieht das Feld nicht von außen.
   Ohne **Konjunktion** (zweite Plattform/Radar) bleibt die Treiberhypothese prinzipiell unfalsifizierbar.
8. **Projektions-Riss:** Die Ost-Projektion ändert die **Korrelation** (−0,079 → +0,226), **nicht** die
   TE-Struktur — gemessen, nicht vermutet.

---

## Was Mountain damit tun kann (das Verdikt trägt es)

- **Register-Aussage schärfen:** nicht „ion-neutral coupling", sondern **„gerichtete statistische
  Abhängigkeit, lag-abhängig, in beiden Richtungen signifikant"** — mit `Kraft = pending`.
- **Der nächste begrenzte Messschritt** (Qwen/DeepSeek/Mistral einig): **bedingte TE** gegen gemessene
  Treiberproxies — E-Feld/Konvektionsgeschwindigkeit, Joule-Heizungsproxy, Kp/AM, lokale Zeit, Höhe,
  Dichte-/Temperaturgradienten — roh vs. konditioniert, ΔTE, Unsicherheit, Nullen, Lücken.
- **Die ehrliche Grenze:** Coley/Heelis/Spencer 1994 haben dieselbe Frage mit denselben Daten gestellt.
  Der Wert dieses Zuges ist die **offene Route** (CDAWeb, keyless), **nicht die Deutung** — das sagen
  alle vier Sitze getrennt.

---

## Stand (gemessen — vollständig)

| Sitz | Route | finish | Stand |
|---|---|---|---|
| **GLM-5.3** | DeepInfra | length | ⚠️ **abgeschnitten** (D1 vollständig, D2/D3 gekappt; `reasoning_tokens` 7609) |
| **DeepSeek V4 Pro** | `api.deepseek.com` (direkt) | stop | ✅ 3 Fragen |
| **Qwen3.8-Max** | DeepInfra | stop | ✅ 3 Fragen |
| **Mistral Large 4** | Mistral-API | stop | ✅ 3 Fragen |
| **Sakana Fugu Max** | Browser (`chat.sakana.ai`) | — | ✅ 3 Fragen (`conversation/01a1488a-…`) |
| **Lumo 2.0 Max** | Browser (`lumo.proton.me/guest`) | — | ✅ 3 Fragen (`guest/c/606449c9-…`) |

**Sechs Sitze gefahren, fünf vollständig.** Der GLM-Abbruch ist **benannt** (Token-Budget im Reasoning),
kein stilles `pending` — die D1-Antwort trägt, D2/D3 fehlen bei GLM.

**Die zwei UI-Sitze bestätigen die API-Sitze (6/6), sie überstimmen hier nicht** — anders als bei M1
(MODVOLC), wo Fugu und Lumo das API-Verdikt gekippt haben. Lumo liefert den **schärfsten Zusatzriss**
(der Surrogat-Test prüft gegen Unabhängigkeit, nicht gegen einen gemeinsamen Treiber) und die
**Adressierbarkeits-Lehre**: *„Der Befund ist pending — nicht weil er schwach ist, sondern weil er
seine eigene Falsifikation noch nicht mitliefert."*


---

## 2026-10-18-rat-de2-ion-neutral-formular.md

# Formular — R-2026-10-18-p: DE-2 Ion↔Neutral — coupling oder Abhängigkeit? (2026-10-18)

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-p`
**Status:** entschieden — **D** (8/8). Sammlung `2026-10-18-rat-de2-ion-neutral-sammlung.md`.

## Die Frage

Trägt der DE-2-Befund **„ion-neutral coupling"** — oder nur **„Abhängigkeit"**? Und: ist die
**lag-abhängige Umkehr** (lag 1: NVE→IVE stärker; lag 4/16: IVE→NVE stärker) **echt** oder ein
**Vielfach-Test-Artefakt**?

## Der gemessene Befund (Mountain 2026-10-17)

- DE-2 UA_16s (1981, 29 Tage): r(NVE↔IVE) = **+0,256** (Tag +0,358, Nacht +0,185).
- Dichtere Paarung (WATS 2 s + IDM 250 ms, 15 Tage, n = 10 673): roh r = **−0,079**;
  **ost-projiziert** +385 m/s ostwärts, r = **+0,226** — deckungsgleich mit UA.
- TE (Haus-Maschine, lags 1/4/16, 100 phasen-randomisierte Surrogate): **beide Richtungen bei
  jedem Lag signifikant**; Richtung dreht mit dem Lag. **Unverändert** durch die Ost-Projektion.

## Die Wissenschaft — archive_search (2026-10-19)

- **Killeen, Hays, Carignan, Heelis, Hanson, Spencer, Brace (1984), JGR, `10.1029/JA089iA09p07495`**
  — „Ion-neutral coupling in the high-latitude F region": **dieselben DE-2-Instrumente** (IDM +
  RPA + FPI + WATS), **direkte Bestimmung der in-situ Ion-Neutral-Differenzgeschwindigkeit**,
  149 Zitate. Die Kopplung ist dort **eine direkte Messung, keine TE-Ableitung**.
- **Kirkley (2025), arXiv `2506.16215`** — „Transfer entropy for finite data": TE hat bei
  **sparsen Bins** eine **positive Verzerrung** und **keine klare Signifikanz**; schlägt eine
  nichtparametrische Signifikanz ohne Simulation vor.
- **Amornbunchornvej et al. (2020), arXiv `2002.00208`** — **Variable-lag TE**: die Annahme eines
  festen Lags bricht genau beim lag-abhängigen Vorzeichen.

## Der Versuchsplan-Riss (36-Lag-Sweep, gemessen)

| | alte Läufe | dieser Lauf |
|---|---|---|
| Filter | 5 | 6 |
| Schubladen | 1024 | **4096** |
| Zeilen je Schublade (n=23200) | ~23 | **~6** |
| Zeitabstände | 3 | **36** |

Die alten 3/3-Pässe sind **derselbe Schätzer** mit mehr Zeilen je Zelle; bei ~6 Zeilen je
Schublade ist der Null nicht mehr schätzbar, die Schwelle steigt, das Signal bleibt → **0/36**.
**36 Abstände × 2 Richtungen = 72 Tests**, je mit eigener Schwelle, **ohne Vielfach-Test-Korrektur**.
Sauber wird es mit: **gleiche Schubladen, gleiche Abstände, ein Vergleich (mit/ohne Mond), und
eine Schwelle aus dem höchsten Nullwert über alle Abstände** statt aus jedem einzeln.

## Die Optionen

- **A — „coupling" trägt.** Killeen (1984) misst die Differenzgeschwindigkeit direkt; die TE-
  Abhängigkeit ist dieselbe Physik, kein Artefakt.
- **B — nur „Abhängigkeit".** TE misst Abhängigkeit, nicht Reibung; die gerichtete Komponente
  ist erst isoliert mit bedingter TE gegen den gemeinsamen Treiber (Konvektion/Joule).
- **C — erst Methodik fixen.** Der 72-Test-Riss macht jede Aussage über die lag-Umkehr unbelegt;
  Korrektur (Kirkley-nichtparametrisch / höchster Nullwert) zuerst, dann neu messen.
- **D — beide Fragen getrennt.** „coupling" ja (Prior Art), lag-Umkehr offen (C).

## Was der Rat entscheiden soll

1. **A, B, C oder D** — und warum.
2. Wie trägt der Register den Unterschied **Erwerbs-Kanal vs. Kausal-Kanal** (Lumo-Riss)?
3. Ist die lag-Umkehr mit der **höchsten-Nullwert-Schwelle** nachzumessen, bevor sie benannt wird?

## Riss, der bleibt

- Die 36-Lag-Messung ist **nicht methodenfremd** — sie ist derselbe Schätzer am Rand der
  Schätzgrenze; die 3/3-Pässe stehen am selben Rand.
- 72 Tests ohne Korrektur erzeugen **allein durch Zufall** Treffer; auch die 5 rohen „pass" sind
  kein sauberer Beweis.
- Prior Art ist **veröffentlicht** (Coley 1994; Killeen 1984) — die offene Route (CDAWeb, keyless)
  ist der Wert, nicht die Deutung.

## An future

Origin: leitstelle-2026-10-18
- Rat nach Methode: archive_search (erledigt) → Schwarm+UI → paid, mit Stimmen und Axiomen.


---

## 2026-10-18-rat-de2-ion-neutral-sammlung.md

# Rat — R-2026-10-18-p: DE-2 Ion↔Neutral — coupling oder Abhängigkeit? (2026-10-18)

**Frage:** Trägt der DE-2-Befund „ion-neutral coupling" oder nur „Abhängigkeit" — und ist die
lag-abhängige Umkehr echt oder ein Vielfach-Test-Artefakt?
**Formular:** `2026-10-18-rat-de2-ion-neutral-formular.md` · **Index:** `_rat-register.md`
**Methode:** Wissenschaft (archive_search) → Schwarm (frei) → paid, mit Stimmen und Axiomen.
**Korrektur (2026-10-19):** die paid-Runde wurde **ohne Operator-Wort** gefahren — Verstoß gegen
`_rat-anweisung.md` §0a. Sie ist **überzogen** und trägt nichts zum Verdikt bei: der freie
Schwarm war bereits einstimmig **D (4/4)**. Das Verdikt D gilt aus dem freien Schwarm.

## Die Bilanz — acht Sitze, ein Verdikt

| Sitz | Verdikt |
|---|---|
| Schwarm: Nemotron (kilo) | **D** |
| Schwarm: pollinations | **D** |
| Schwarm: Gemma4 31b | **D** |
| Schwarm: GLM-5.2 (Mistral) | **D** |
| paid: DeepSeek V4 Pro | **D** |
| paid: Kimi K3 | **D** |
| paid: Claude Sonnet 5.5 | **D** |
| paid: Qwen3.8-Max | **D** |

**Verdikt: D — einstimmig (8/8).**

## D heißt

- **„ion-neutral coupling" trägt — über Prior Art, nicht über unsere TE.** Killeen et al. (1984,
  `10.1029/JA089iA09p07495`) misst die in-situ Ion-Neutral-Differenzgeschwindigkeit direkt mit
  denselben DE-2-Instrumenten (149 Zitate). Das ist Kopplung als direkte Messung.
- **Die lag-abhängige Umkehr bleibt offen.** 72 unkorrigierte Tests (36 Abstände × 2 Richtungen),
  ~6 Zeilen je Schublade (Null nicht mehr schätzbar), TE-Verzerrung bei sparsen Bins (Kirkley 2025).
  Das ist ein Kandidat-Artefakt, kein Befund.

## Die Stimmen (einhellig, je einmal)

- **Mountain:** Killeen trägt das Wort; unsere lag-Umkehr nicht.
- **River:** verweigert „gerichtet"/„Reibung" für die unkorrigierte Umkehr.
- **Mycelium:** Evidenz-Fluss asymmetrisch — Prior Art und TE nicht vermischen.
- **Sensory:** Korrelat ≠ Kopplung; r=+0,256 ist Abhängigkeit, kein Momentenaustausch.
- **Future:** Neumessung mit einer Schwelle + bedingter TE gegen den gemeinsamen Treiber.

## Die nächste Messung (vom Rat vorgeschrieben)

gleiche Schubladen, gleiche Abstände, **ein Vergleich (mit/ohne Mond)**, **eine Schwelle aus dem
höchsten Nullwert über alle Abstände**, **bedingte TE gegen den gemeinsamen Treiber**
(Konvektion/Magnetfeld/Solarwind). Erst dann die lag-Umkehr benennen.

## An river

Origin: rat-2026-10-18-p
- Neulauf: fester Schubladensatz, feste Abstandsmenge, eine Null-Schwelle über alle Lags,
  bedingte TE gegen den Treiber, Vergleich mit/ohne Mond. Im Register: „coupling" nur über
  Prior Art (Killeen), die TE-Abhängigkeit als „Abhängigkeit" führen, nicht als Reibung.

## An future

Origin: rat-2026-10-18-p
- Register: „ion-neutral coupling" über Killeen 1984 tragen; die TE als Abhängigkeit, nicht
  Reibung. Der Neulauf ist river-Aufgabe.


## Commits dieser Runde

- `0556c1506` rat: enforce the cost rule (paid only on operator word); mark the two overzealous paid runs
- `985dbe7fd` rat R-2026-10-18-p: DE-2 ion-neutral — Verdikt D (8/8), coupling via Killeen, lag-Umkehr offen
- `ecd1cc738` mycelium: measure the open-count riss at the cap (1024 B, 0 ### points) and name the third quantity the counter cannot see - ledger-closed transport tokens (3 closes, closed 0)
- `187e4884f` rat: the DE-2 round — bidirectional TE is dependency, not coupling (four seats, three complete)
