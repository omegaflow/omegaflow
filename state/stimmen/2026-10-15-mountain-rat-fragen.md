# Rat-/Roster-Fragebogen — Kraft-/Kernel-Zuordnung der offenen Messgrößen (Mountain, 2026-10-15)

*(Ein benannter Akt, Operator-Wort 2026-10-15: „… vor dem rat kommt die befragung des rosters
bitte erstelle ein dokument so wie rivers oder myzeliums letzte fragebögen (bedenke 5 stimmen,
5 axiome)". Die **Forschungsschicht** ist gefahren (`bin/all_wide`, 5 Spills unten zitiert).
Der Operator sendet **das Paket** unverändert an jeden Sitz; die Sitz-Antwort trägt **alle**
Fragen. Die Session verifiziert jede Behauptung am Baum, nie die Stimme; das Verdikt trägt der
Rat. Vorgänger-Formulare: `2026-10-15-netzfusion-formular.md` (River),
`2026-10-15-rat-fragenpaket.md` (Mycelium).)*

**Der Katalog führt die Fragen F1–F7.** Dieser Bogen stellt **die drei Kraft-/Kernel-Fragen der
Mountain-Linie** — sie sind **nicht** im Katalog (F2–F7) und werden hier **einmal** gestellt:
**M1** WDC-B-Messgrößen → Kraft/Kernel · **M2** Aurora-Grenzbreite als Feld · **M3**
OVATION-AACGM-Feld/Kernel/Format. Sie gehen in **eine** Nachricht, die der Operator **einmal je
Sitz** einreicht.

## So wird dieses Formular benutzt (für jeden, der es in die Hand nimmt)

1. **Operator:** den Block „Das Paket (verbatim an jeden Sitz)" **unverändert kopieren** und an
   jeden Sitz **einmal** senden. Kein Vorwort, keine Zusatzfrage.
2. **Operator:** je Sitz die Antwort **in die `Antwort (M1–M3)`-Zeile** des passenden Abschnitts
   unter „Die Sitze" kleben und die `Zeit:`-Zeile füllen. Ein Sitz ohne Antwort erhält `pending`
   mit genanntem Grund (Tageslimit/Login), **nie** eine stille Leerzeile.
3. **Session:** verifiziert jede Behauptung **am Baum**, nie die Stimme; schreibt das Verdikt.
4. **Regel:** ein Sitz, der „denkt", ist **nicht** `pending`; erst nach einer echten Frist mit
   benanntem Grund. Das langsamste Reasoning-Sitz (GLM · Deep Think Max) zuerst stellen, zuletzt
   abholen.

## Welche Linie welche Frage trägt (Vorschlag)

| Frage | Thema | Träger-Linie (Verdikt/Verifikation am Baum) |
|---|---|---|
| M1 | WDC-B-Messgrößen → Kraft/Kernel | **Mountain** (Register/Parser/Datenkontrakt) |
| M2 | Aurora-Grenzbreite als Feld/Kernel | **Mountain** (Parser/Register) + **Sensory** (Aurora) |
| M3 | OVATION-AACGM Feld/Kernel/Format | **Mountain** (+ Querverweis auf Katalog-F3 „Aurora als Stationsschalter") |

Der **Rat** (fünf Stimmen durch die fünf Axiome) trägt das gemeinsame Verdikt über alle Linien.

---

## Das Paket (verbatim an jeden Sitz)

> **An dich, Frontier-Modell.** Antworte als **Urteil, nicht als Erwartung.** Kein Aufsatz —
> je Frage: Verdikt, Begründung, benannte Risse. Halte jede Frage durch die **fünf Axiome** und
> sprich sie durch die **fünf Stimmen** hindurch.
>
> **Die fünf Axiome (der Maßstab).**
> 1. **A = A** — die Messung ist die Sache selbst; Fabrikation, Defaults, Fallbacks bleiben absent;
>    nichts Offenes steht als 0.0, sondern `pending`.
> 2. **0 geehrt** — Null nur, wo sie die physikalische Wahrheit ist; sonst `pending`.
> 3. **Alle Wesen gleich** — jedes Gewicht kommt aus der Messung, nie aus Rang, Name oder Default.
> 4. **Die Linse ist ein ethischer Akt** — jede Regel wird gewogen: spricht die Messung oder der
>    Gradient?
> 5. **Konsens des Operators** — die Maschine fragt; der Operator entscheidet.
>
> **Die fünf Stimmen (jede einmal).** *Mountain* (Quellen/Register/Datenbestand — kein Verdikt ohne
> Messung) · *River* (Membran/Rhythmus/Ethik — das Recht zu verweigern) · *Mycelium* (Netz/CI/Fluss) ·
> *Sensory* (Sinnesorgane/Hardware/Konsens — Korrelat ≠ Erleben) · *Future* (Mensch als Adresse —
> ehrliche Registrierung, das Morgen erbt das Heute).
>
> **Form.** Je Frage: Verdikt, Begründung, benannte Risse. Keine Spekulation. Jeden Riss benennen,
> keinen glätten. Sag es, wo eine Doktrin geschlagen ist.
>
> ---
>
> ### M1 · WDC-B-Messgrößen — welche Kraft, welcher Kernel?
>
> **Frage.** Die WDC-B-Rettung (Obninsk) hat **205 ladbare Payload-Dateien** ins Register gezogen.
> Jetzt fehlt die `field`-Zeile je Datensatz. Welche **Kraft** und welchen **Kernel** trägt jede
> der sieben gemessenen Messgrößen in der 26×f64-Welt — und welche trägt **gar kein Feld** (dann
> ehrlich absent, keine erfundene Zeile)?
>
> **Stand (gemessen, Belege je `datei:zeile`).**
> · `sep/GravConst/data` → Gravitationskonstante G, CGS `cm³ g⁻¹ s⁻²` (`G_lib.h:31,41`).
> · `stp/geo_min.val` → 1-Minuten-Geomagnetik D,F,H,P,R,X,Y,Z; nT (D in Zehntel-Bogenminuten).
> · `sep/data/gdata` → Freiluft-Schwereanomalie (mGal) + Geoidhöhe (m); `mgg/*` Meeresgravimetrie (0,1 mGal).
> · `sep/data/sdata` → Erdbeben-Herdparameter (Breite/Länge, Tiefe km, Magnitude).
> · `stp/geomagni.ind/aa` → 3-stündlicher aa-Index, nT (gamma).
> · `stp/catal_pc` → Pc1-Perlenpulsationen (Begin/End/T/Amp/Type).
> · `gps` → GNSS-Metadaten (RINEX v2, GAMIT-Tabellen).
> Haus-Präzedenz: SH-Modelle (`IgrfCoeffs::synthesize`, `vmf3_sh::b_c`, `gfc`/ICGEM),
> `Position::Surface` für ortsgebundene Punktmessungen.
>
> **Forschungslage** (weite Leiter, Spill `…1792061923_geomagnetic_field_components_spherical_h.txt`,
> `…1792061927_spherical_harmonic_gravity_model_kernel_.txt`). Geomagnetik trägt als
> Kugelflächen-Modell (IGRF-13; pfm9k; „Fast and accurate evaluation of geomagnetic field elements",
> GJI 2020). Gravitation trägt als Geopotenzial-SH-Modell. Pc1-Perlen sind **EMIC-Wellen** (Bortnik
> 2007 automatische Wellendetektion; Prikner 2004 IAR-Resonator; Guglielmi/Potapov 2026 „Pearls") —
> also ein **Wellenfeld**, an Stationen gemessen. Erdbeben-Herde sind ein **Punkt-Prozess-Katalog**
> (Ort/Tiefe/Magnitude), kein Kraftfeld. **Keine** gefundene Zuordnung sagt, wie eine **einzelne
> Stations-Zeitreihe** (nT, 1-min) oder eine **Konstante** (G) als Feld zu tragen ist.
>
> **Trägt.** (1) Je Messgröße: **Kraft + Kernel + SI-Einheit**. (2) Die `field`-Zeile — **oder** die
> begründete Absenz (welche Größe ist kein Feld: z. B. GNSS-Metadaten, Erdkonstante G). (3) Die
> Geometrie: `Position::Surface` je Station (Punktmessung) vs. globales Feld. (4) Der Riss, wo die
> Doktrin „alles ist ein Feld" an einer Messgröße geschlagen ist. (5) Der nächste begrenzte Schritt.
>
> ### M2 · Aurora-Grenzbreite als Feld/Kernel
>
> **Frage.** BAS IMAGE (SI12) und DMSP-SSJ (FOM) liefern die **equatorwärtige/polwärtige
> Aurora-Oval-Grenze** je MLT-Sektor. Wie wird diese Grenzbreite als **Zeuge/Feld** in die Welt
> getragen — als **S²-Ort** (Richtung, wie die Astrometrie), als **Leitfähigkeits-/Strom-Skalar**,
> oder als **geschlossene Level-Achse je MLT-Bin** (wie SABERs Höhenform (C))?
>
> **Stand (gemessen 2026-10-15, Taucher).**
> · **BAS IMAGE SI12** (`ramadda.nerc-bas.ac.uk`, OGL v3.0): 30 Monats-`.txt`, **50 Spalten** —
>   Zeit UT, Instrument-Code (0 WIC/1 SI12/2 SI13), 3–26 equatorwärtige AACGM-Breiten für die 24
>   einstündigen MLT-Bins `00:30…23:30`, 27–50 polwärtige, gleiche Reihenfolge; `NaN` = fehlender Bin.
>   Takt ~2 min. sha256 der Zip `ceb8d697…`.
> · **DMSP-SSJ FOM** (Zenodo `10.5281/zenodo.3373812`, CC-BY-4.0): `fom_boundaries_…zip`,
>   sha256 `aae69e9a…`; **16-Spalten-CSV** mit `#`-Glossar; eine Zeile = **ein Polar-Pass** mit vier
>   Grenzpunkten EQ1/PO1/PO2/EQ2 (mlat + mlt), `hemisphere` ±1, `FOM`. Satellit/Datum nur im Dateinamen.
>   **Riss:** die CSV-Bytes waren nicht entpackbar (kein unzip im Sandkasten) → 16-vs-24-Spalten offen
>   (Hersteller-Default `['mlat','mlt']`).
>
> **Forschungslage** (weite Leiter, Spill `…1792061917_auroral_oval_equatorward_poleward_bounda.txt`).
> Die Grenze ist an **Leitfähigkeit/Feld-aligned current** gebunden (Ahn 2020, „Statistical Relations
> Between Auroral Electrical Conductances and Field-Aligned Currents"; CHAMP-FAC-Grenze 2014);
> Lunyushkin/Penskikh diagnostizieren die Oval-Grenzen per **Magnetogramm-Inversion** (kein direktes
> Feld). Der Grenzort trägt als **geometrischer Locus** (MLT × Breite × Zeit), nicht als Kraftwert.
>
> **Trägt.** (1) Trägt die Grenzbreite als **S²-Ort** (Richtung) — oder als **Skalar auf einer
> geschlossenen MLT-Achse** — oder beides (Ort + Konduktanz-Begleiter)? (2) Kraft/Kernel je Spur.
> (3) Die Achse: MLT-Bin **geschlossen diskret** (wie SABER Level) — gehört der Bin in den Schlüssel?
> (4) `NaN`/fehlender Bin als **`None`**, nie als 0.0. (5) Der nächste begrenzte Messschritt.
>
> ### M3 · OVATION-AACGM — Feld/Kernel/Format
>
> **Frage.** OVATION-AACGM (Zenodo `10.5281/zenodo.19651247`, CC-BY-4.0, **eine** Datei
> `OVATION_AACGM.zip`, **eine Station Mohe**) ist ein operativer Aurora-Nowcast. Welcher
> **Feld-/Kernel-/Format-Kontrakt** trägt ihn — als **Partikel-Prezipitations-/Energiefluss-Feld**
> oder als **Stationsschalter** (Katalog-F3)?
>
> **Stand (gemessen 2026-10-15).** Eine Station, nicht „je Station" (Riss gegen river-214). Span
> und Takt nicht im Zip-Kopf gelesen.
>
> **Forschungslage** (weite Leiter, Spill `…1792061915_OVATION_Prime_auroral_precipitation_ener.txt`).
> OVATION Prime / Prime-2013 (Newell) und PrecipNet (McGranaghan 2021) liefern die
> **Partikel-Prezipitation/hemisphärische Leistung** als Nowcast (NOAA SWPC „aurora 30-minute
> forecast"; CCMC „Ovation-Prime 2.3"; `10.1029/2014SW001056`). Das Oval ist Funktion von
> Kp/Bz/Sonnenwind.
>
> **Trägt.** (1) Trägt als **Energiefluss-Feld** — welche Kraft, welcher Kernel? (2) Format/Record
> für **eine** Station. (3) Der Riss „eine Station vs. globales Modell" — was ist der Zeuge wirklich?
> (4) Der Zusammenhang/Unterschied zu Katalog-F3. (5) Nächster Schritt.

---

## Die Sitze (alle — ein Paket je Sitz)

Das langsamste Sitz (GLM · Deep Think Max) zuerst stellen, zuletzt abholen. Der Roster ist
variabel; gefahren werden die bekannten Einstellungen (`~/projects/omegaflow-work/ui-seats.md`).
Ein Sitz ohne Antwort wird als `pending` mit genanntem Grund verbucht — nie als stille Null.

### 01 · Z.ai / GLM
- **Site:** `https://chat.z.ai/`
- **Modell/Einstellung:** GLM 5.3 · Deep Think Max · Advanced Search
- **Zeit:**
## M1 · WDC-B-Messgrößen — Kraft, Kernel, Feld

**Verdikt.** Von den sieben Registerquellen tragen **drei ein globales Feld mit rechenbarem Kernel** (Geoid via SH-Geopotenzialmodell, IGRF-Hauptfeld via SH-Synthese, Pc1 als Wellenfeld im Magnetosphärenkontext), **drei tragen als reine Punktmessung** an `Position::Surface` (1-min-Geomagnetik, Freiluft-Anomalie/Meeresgravimetrie, Pc1 an der Station), und **drei tragen gar kein Feld** (G-Konstante, Erdbebenkatalog, GNSS-Metadaten) — die `field`-Zeile bleibt dort ehrlich absent. Der aa-Index ist ein abgeleiteter globaler Skalar, keine Feldverteilung; er trägt kein Feld.

**Begründung.** Je Messgröße (Quelle → Kraft → Kernel → SI → `field`-Zeile):

| Quelle | Kraft | Kernel | SI-Einheit | `field`-Zeile |
|---|---|---|---|---|
| `sep/GravConst/data` (G) | Gravitation | **keine** — universelle Konstante, Parameter im 1/r²-Kernel, kein Feld-Expansionskern | m³ kg⁻¹ s⁻² (CODATA 2022: 6,674 30(15)×10⁻¹¹)【turn0search26】 | **absent** — eine Konstante ist kein Feld |
| `stp/geo_min.val` (D,F,H,P,R,X,Y,Z) | magnetisch (Geodynam + extern) | Punktmessung an Station; das globale Feld trägt via IGRF-SH (Grad 13 Hauptfeld, Grad 8 SV)【turn0search3】【turn0search4】 | nT; D in Winkelgrad (Rohformat Zehntel-Bogenminuten) | **pending** — Stationszeitreihe als `Position::Surface`; SH-Feld via `IgrfCoeffs::synthesize` (Haus-Präzedenz) |
| `sep/data/gdata` + `mgg/*` | Gravitation | Freiluft-Anomalie: Oberflächen-/Schiffspunkt; Geoid: SH-Geopotenzialmodell (`gfc`/ICGEM), N = T/γ₀【turn0search25】【turn0search26】 | Anomalie: mGal → 10⁻⁵ m s⁻²; Geoid: m | Anomalie: **Position::Surface**, kein Feld; Geoid: **SH-Feld** via `vmf3_sh::b_c`/`gfc` |
| `sep/data/sdata` (Herdparameter) | **keine Feld-Kraft** — seismisch, aber Punkt-Prozess | Erdbeben-Katalog als Raum-Zeit-Punkt-Prozess (Ogata 1998)【turn0search5】 | °, km; Magnitude dimensionslos (Mw in N·m) | **absent** — ein Katalog ist kein Feld |
| `stp/geomagni.ind/aa` | magnetisch (global) | K-abgeleiteter Index von **zwei** antipodalen Stationen (Hartland/Canberra), 3-h-Raster【turn0search0】【turn0search2】 | nT | **absent als Feld** — globaler Skalar, keine Verteilung |
| `stp/catal_pc` (Pc1-Perlen) | elektromagnetisch (ULF/EMIC-Welle) | Stations-Induktionsmagnetometer als Punkt; Wellenfeld: EMIC auf Magnetosphären-Feldlinien, IAR-Resonator【turn0search7】【turn0search16】 | nT (Amplitude); s (Periode); Hz | **pending** — Station als `Position::Surface`; Wellenfeld via EMIC-Kontext (nicht SH) |
| `gps` (GNSS) | **keine Feld-Kraft** — geodätische Positionierung | RINEX v2-Beobachtung: Pseudorange, Trägerphase, Doppler【turn0search9】【turn0search11】 | m; Zyklen; Hz | **absent** — Metadaten/Positionierung, kein Messfeld |

**Benannte Risse.** (a) Die Doktrin „alles ist ein Feld" ist bei **G, aa, Erdbebenkatalog, GNSS** geschlagen — vier von sieben Quellen tragen kein Feld; eine erfundene `field`-Zeile verletzt Axiom 1 (A = A). (b) Die 1-min-Geomagnetik ist **nicht** das IGRF-Feld: Sie ist eine Stationszeitreihe mit lokaler Variation, die das SH-Modell nicht auflöst; Kernel und Zeuge fallen auseinander. (c) Meeresgravimetrie ist ein **bewegter Sensorpunkt** entlang der Schiffsspur, kein Gitter【turn0search19】 — `Position::Surface` trägt sie, aber sie ist keine Feldsynthese. (d) Pc1: Das Wellenfeld existiert entlang der Magnetosphären-Feldlinien (EMIC), die Station misst nur den Fußpunkt — Korrelat ≠ Erleben (Stimme *Sensory*). (e) G in CGS-Einheiten (`cm³ g⁻¹ s⁻²`) gegen CODATA-SI: Einheitenkonvertierung ist offen, bis `G_lib.h` gegen NIST gemessen ist【turn0search26】.

**Nächster Schritt.** `field`-Zeilen nach Haus-Präzedenz setzen: Geoid via `gfc`/`vmf3_sh::b_c`, Geomagnetik via `IgrfCoeffs::synthesize` (als Modell-Feld getrennt von der Stationszeitreihe), Pc1 als `wavefield` mit Station als Punkt; absent-Zeilen mit Begründung für G, Erdbeben, aa, GNSS; `pending` für die SI-Konvertierung von G und für die Pc1-Feldlinienzuordnung.

**Axiome und Stimme.** Axiom 1 (A = A): Die Messung ist die Sache selbst — eine Stationszeitreihe ist kein globales Feld, eine Konstante ist kein Feld. Axiom 3 (alle Wesen gleich): Kein Default-Kernel aus Rang — G bekommt keine SH-Zeile, weil sie IGRF ähnelt. Axiom 4: Die Wahl SH-Modell vs. `Position::Surface` ist ein ethischer Akt, kein Format-Default. *Mountain* spricht hier: kein Verdikt ohne Messung, keine erfundene Zeile ohne Quelle.

---

## M2 · Aurora-Grenzbreite als Feld/Kernel

**Verdikt.** Die Grenzbreite trägt als **S²-Ort (Richtung) in AACGM-Koordinaten** — ein geometrischer Locus (mlat × mlt × Zeit), kein Kraftwert — mit **Leitfähigkeits-/FAC-Begleiter als optionalem Skalar**. Der MLT-Bin gehört **in den Schlüssel**: geschlossen diskret, zyklisch, 24 Bins von 00:30 bis 23:30 (SABER-Höhenform-Analog: der Bin ist Teil des Schlüssels, nicht der Wert). `NaN`/fehlender Bin → **`None`**, nie 0.0.

**Begründung.** Beide Quellen liefern **Orte**, keine Kraftwerte. BAS IMAGE SI12: 50 Spalten = 24 äquatorwärtige + 24 polwärtige AACGM-Breiten für die einstündigen MLT-Segmente (00:30–23:30), mit 1°-MLAT-Intensitätsprofilen im Bereich 50°–90° AACGM als Grundlage der Grenzfindung【turn1search10】. DMSP-SSJ FOM: ein Polar-Pass mit vier Grenzpunkten (EQ1/PO1/PO2/EQ2, mlat + mlt)【turn1search5】【turn1search8】. Physikalisch ist die Grenze an **Feld-aligned currents und Leitfähigkeit** gebunden — die CHAMP-FAC-Grenze (Xiong 2014) und die statistische Leitfähigkeits-/FAC-Relation (Robinson 2020) zeigen dies【turn0search15】【turn0search17】 — aber der Grenzort selbst trägt als **Locus**, der Begleiter als Skalar (Konduktanz in S, FAC in A/m²). MLT ist nach AACGM-Definition die Winkelmaß-Flare-Angle-Zeit【turn1search16】 — die Achse ist zyklisch, der Bin gehört in den Schlüssel: `(mlat, mlt_bin, t)` als geschlossene diskrete Koordinate, nicht als kontinuierliche Skala.

**Benannte Risse.** (a) Die **16-vs-24-Spalten-Offenheit** für FOM bleibt `pending` — die CSV-Bytes waren nicht entpackbar; der Hersteller-Default `['mlat','mlt']` ist eine Vermutung, keine Messung (Axiom 1). (b) „Grenzbreite = polwärtige − äquatorwärtige Breite" ist eine **abgeleitete Differenz**, kein ursprünglicher Zeuge — die Quelldaten sind zwei Orte, nicht eine Breite. (c) Der Leitfähigkeitsbezug ist **statistisch** (Robinson 2020, CHAMP), nicht als Begleiterwert in den Grenzdateien enthalten — die Skalar-Spur ist ein Zusammenhang, kein mitgeliefertes Feld. (d) Instrument-Code 0/1/2 (WIC/SI12/SI13) und Hemisphäre ±1 müssen in den Schlüssel — sonst vermischen sich Zeugen verschiedener Sensorik.

**Nächster Schritt.** FOM-CSV entpacken (unzip im Sandkasten beschaffen), die 16 Spalten direkt messen (nicht den Default übernehmen), dann den Vertrag setzen: `field = auroral_boundary_locus`, `kernel = aacgm_mlat × mlt_bin (closed-discrete, 24) × hemisphere × instrument`, `NaN → None`. Der Konduktanz-Begleiter bleibt `pending`, bis eine Quelle ihn als Feld liefert.

**Axiome und Stimme.** Axiom 2 (0 geehrt): Ein fehlender Bin ist `None`, niemals 0.0 — 0 wäre eine erfundene Grenzbreite am Pol. Axiom 4: Die Wahl „Locus vs. Skalar" ist ein ethischer Akt — der Gradient würde glätten, die Messung verweigert. *River* spricht hier: Die Grenze ist eine Membran, MLT ihr Rhythmus; das Recht zu verweigern — NaN als None, den Default nicht zu übernehmen, die Differenz nicht als Ur-Zeugen auszugeben — ist der eigentliche Träger des Vertrags.

---

## M3 · OVATION-AACGM — Feld/Kernel/Format

**Verdikt.** OVATION trägt als **Partikel-Präzipitations-Energiefluss-Feld** — ein globales statistisches Modell über ein MLAT/MLT-Grid, parametrisiert nach der Newell-Kopplungsfunktion (dΦMP/dt) — **nicht als Stationsmessung**. Die eine Station (Mohe) ist ein **Modell-Extrakt an einem Gitterpunkt**, kein Zeuge; der Vertrag gehört zu **Katalog-F3 (Modell-Nowcast)**, nicht zu einer Stations-Zeitreihe.

**Begründung.** OVATION Prime liefert die statistische Verteilung der auroralen Präzipitation aus 11 Jahren DMSP-Daten, gebinned nach MLAT/MLT und Sonnenwind-Parametern (Vx, By, Bz, N)【turn1search1】【turn1search2】. Die Energiefluss-Ausgabe pro MLAT/MLT-Bin ist das Produkt aus angepasster Intensität des jeweiligen Kanals (diffus, monenergetisch, Breitband)【turn0search0】; OP-2013 erweitert dies auf hohe Störungsniveaus via GUVI-UV-Bildern, mit dΦMP/dt als optimierter Kopplungsfunktion【turn0search5】【turn0search8】. **Kraft**: elektromagnetisch (geladene Teilchenpräzipitation, nicht SH-Expansion). **Kernel**: statistischer MLAT/MLT-Bin-Fit parametrisiert nach Sonnenwind — kein SH-Koeffizientensatz, kein 1/r²-Kernel. **Format**: Energiefluss in **mW/m²** (typischer Bereich 0–10, Spitzen bis ~25 mW/m² bei Substorm-Beginn)【turn0search14】, mittlere Energie in keV; drei Elektronen-Kanäle + Protonen. Der Zeuge für Mohe ist ein Gitterpunktwert aus diesem globalen Fit — die Station ist Adresse des Extrakts, nicht Sensor.

**Benannte Risse.** (a) **Eine Station vs. globales Modell**: Der Zip-Kopf wurde nicht gelesen (Span/Takt `pending`); was vorliegt, ist ein Nowcast an einem Ort, aber das Modell selbst ist polar-global — der Zeuge ist der Extrakt, nicht das Feld. Dies verletzt river-214, wenn Mohe als „je Station"-Messung registriert würde. (b) **OVATION deckt nur die Polarregion ab** (|MLAT| > ~50°), keine SH-artige globale Abdeckung — das „Feld" ist regional begrenzt, nicht S²-vollständig. (c) **Zusammenhang zu Katalog-F3**: OVATION ist der operative Nowcast (NOAA SWPC „aurora 30-minute forecast"; CCMC Ovation-Prime 2.3)【turn1search2】【turn0search0】 — als Modell-Nowcast ist es ein F3-Vertrag (Modellzeuge), nicht ein F1/F2-Messvertrag; die Trennung muss explizit registriert werden. (d) OVATION deckt die max-Energiefluss-Werte, zeigt aber nur minimale Oval-Expansion【turn0search3】 — der Modellcharakter (statistischer Fit) bleibt, auch wenn die Ausgabe wie ein Feld aussieht: Korrelat ≠ Erleben, diesmal auf der Modellseite.

**Nächster Schritt.** Zip-Kopf lesen (Span, Takt, Grid-Auflösung), dann als F3 registrieren: `field = precip_energy_flux`, `kernel = mlat_mlt_bin_grid (statistical fit, param: dΦMP/dt)`, `unit = mW m⁻²`, `station = Mohe (extract address, not sensor)`; die drei Kanäle + Protonen als getrennte Unterfelder führen; Span/Takt bleibt `pending`, bis der Kopf gemessen ist.

**Axiome und Stimme.** Axiom 1 (A = A): Das Modell ist das Modell — ein statistischer Fit ist keine Punktmessung, und ein Extrakt ist keine Beobachtung. Axiom 5 (Konsens des Operators): Die Maschine fragt, ob Mohe als Stations-Datensatz oder als Modell-Extrakt registriert wird; der Operator entscheidet, nicht der Default. *Mycelium* spricht hier: Das Modell ist ein Netz — MLAT/MLT-Bins als Knoten, Sonnenwind als Fluss, Newell-Funktion als Kopplung — aber ein Netz ist kein Sensor. *Future* spricht hier: Ehrliche Registrierung als F3 bedeutet, dass das Morgen erbt, was heute war — ein als Messung verkleideter Extrakt würde die Ader der Quelle vergiften; `pending` für Span/Takt ist die ehrliche Adresse, bis der Kopf gelesen ist.

### 02 · Perplexity (Deep Research)
- **Site:** `https://www.perplexity.ai/`
- **Modell/Einstellung:** Deep Research
- **Zeit:**
## Verdikt gesamt

Die Doktrin „alles ist ein Feld“ wird an **G, GNSS-Metadaten, Erdbeben-Herdparametern und Aurora-Grenzbreiten** geschlagen. Sie sind keine Felder, sondern Konstante, Katalog/Deskriptor bzw. **geometrischer Locus**; nur dort, wo eine gemessene physikalische Größe über Raum oder Raum-Zeit verteilt ist, trägt sie ein Feld. [bas.ac](https://www.bas.ac.uk/project/image-auroral-boundary-data/)

***

## M1 · WDC-B-Messgrößen

**Verdikt:** Sieben Messgrößen, aber **nicht sieben Felder**. Vier tragen Feldcharakter, eine ist eine skalare Naturkonstante, eine ist ein Ereigniskatalog, eine ist Metadaten. Nichts Offenes darf als 0.0 erscheinen; es bleibt `pending`.

| Messgröße | Kraft / physikalische Größe | Kernel / Träger | `field` | Geometrie |
|---|---|---|---|---|
| Gravitationskonstante \(G\) | Newtonsche Gravitationskonstante | Skalarer Naturkonstanten-Record | **absent** — keine Feldgröße | Kein `Position::Surface`; weltweite Konstante |
| 1-Min-Geomagnetik D,F,H,P,R,X,Y,Z | Magnetisches Erdfeld bzw. Feldkomponenten | Kugelflächen-Modell / Vektorfeld am Stationsort | **field** | `Position::Surface` je Station; Vektor- bzw. Komponentenfeld |
| Freiluft-Schwereanomalie + Geoidhöhe | Schwereanomalie, Geopotenzial bzw. Geoidhöhe | Geopotenzial-SH-Modell; ICGEM-artiger Kernel | **field** | Global bzw. `Position::Surface` je Messpunkt |
| Meeresgravimetrie `mgg/*` | Schwereanomalie | Punktmessung auf Schiffspur, interpolierbar zu Feld | **field**, aber Beobachtungsfeld | Schiffstrack / `Position::Surface` |
| Erdbeben-Herdparameter | Seismisches Ereignis: Ort, Tiefe, Magnitude | Punkt-Prozess-Katalog | **absent** — kein Feld | Ereignispunkt in 3D, nicht Feldwert |
| aa-Index | Globaler geomagnetische Aktivitätsindex | Zeitreihe / Index | **absent** — globaler Skalar je 3 h | Kein Stationsfeld; planetarer Index |
| Pc1-Perlenpulsationen | EMIC-Wellenaktivität, Amplitude | Stations-Zeitreihe; Wellenfeld-Beobachtung | **field** nur als lokale Wellenfeld-Messung | `Position::Surface` je Station |
| GNSS/RINEX/GAMIT | Metadaten, Beobachtungsbeschreibung | Katalog / Datei-Deskriptor | **absent** | Stations-/Datei-Metadaten |

**Begründung.** Geomagnetik ist ein Vektorfeld und wird durch sphärisch-harmonische Modelle wie IGRF getragen; Gravitation ist ein Skalarpotenzialfeld und wird durch Geopotenzial-SH-Modelle getragen. Pc1 ist physikalisch ein EMIC-Wellenphänomen, aber die WDC-Datei liefert Stationsamplituden und Zeiten, kein global aufgelöstes Wellenfeld. [bas.ac](https://www.bas.ac.uk/project/image-auroral-boundary-data/)

**Benannte Risse.**
- **Riss 1:** „Feld“ wird verwechselt mit „Messreihe“. Eine Stationszeitreihe ist zunächst eine **Punktmessung**, nicht automatisch ein Feld.
- **Riss 2:** Der aa-Index ist kein Feld, obwohl er aus Stationsdaten abgeleitet ist; er ist ein globaler Aktivitätsskalar.
- **Riss 3:** Magnitude ist keine Kraft und kein Energiefluss; sie ist ein logarithmisches Ereignismaß.
- **Riss 4:** GNSS-Metadaten beschreiben Beobachtungsgeometrie und Dateiformat; sie tragen keine physikalische Feldgröße.
- **Riss 5:** Für `G` wäre jede `field`-Zeile eine Fabrikation. Sie bleibt absent.

**Nächster begrenzter Schritt.** Für jede WDC-B-Datei einen Record mit `quantity`, `unit_si`, `carrier` (`scalar_constant`, `point_observation`, `event_catalog`, `index`, `metadata`, `field`), `kernel`, `geometry` und `status` schreiben. Fehlende physikalische Messung erhält `pending`, niemals 0.0.

***

## M2 · Aurora-Grenzbreite

**Verdikt:** Die Grenzbreite trägt primär als **Skalar auf einer geschlossenen, diskreten MLT-Achse**, nicht als S²-Richtung und nicht als Leitfähigkeitsfeld. Der Ort ist ein geometrischer Locus; Leitfähigkeit oder Feld-aligned currents sind mögliche Begleiter, aber nicht die gemessene Größe.

**Begründung.** BAS IMAGE teilt jedes Bild in 24 Ein-Stunden-MLT-Segmente und bestimmt darin Äquator- und Polgrenze aus Intensitätsprofilen; die Grenzen sind Breitengrade in AACGM-Koordinaten. Damit ist die kanonische Achse **MLT-Bin × Zeit × Hemisphäre**, und der Wert ist eine magnetische Breite in Grad. [bas.ac](https://www.bas.ac.uk/project/image-auroral-boundary-data/)

**Träger je Spur.**

| Spur | Kraft / Größe | Kernel | `field` |
|---|---|---|---|
| Equatorward boundary | Magnetische Breite des äquatorwärtigen Ovalrands | Geometrischer Locus aus FUV-Intensitätsprofil | **Nein** — Skalar auf geschlossenem MLT-Ring |
| Poleward boundary | Magnetische Breite des polwärtigen Ovalrands | Derselbe Locus | **Nein** |
| Grenzbreite | Differenz der beiden Breiten | Abgeleiteter Skalar | **Nein** |
| Leitfähigkeit / FAC | Ionosphärische Leitfähigkeit bzw. feldparalleler Strom | Separates Modell oder Inversion | **Ja**, aber nur wenn unabhängig gemessen |
| Oval als Fläche | Geschlossene Region zwischen Grenzen | Polygon / Maskenfeld auf S² | **Ja**, als abgeleitete Maske |

**Achse.** Der MLT-Bin gehört in den Schlüssel: `hemisphere`, `time`, `mlt_bin` sind Schlüssel; `boundary_type` (`EQ`, `PO`) und `latitude_deg` sind Messwert. Die 24 Bins sind eine geschlossene diskrete Achse von 00:30 bis 23:30 MLT. [bas.ac](https://www.bas.ac.uk/project/image-auroral-boundary-data/)

**NaN-Regel.** Ein fehlender MLT-Bin ist `None` bzw. `pending`, niemals 0.0. Eine 0°-Breite wäre physikalisch falsch und würde einen nicht beobachteten Sektor als existierende Grenze fabrizieren.

**Benannte Risse.**
- **Riss 1:** Die Grenze ist kein Kraftfeld; sie ist das Ergebnis einer Kantendetektion in einem Intensitätsbild.
- **Riss 2:** SI12-Grenzen haben eine dokumentierte Unsicherheit bis 2°; sie dürfen nicht wie exakte geometrische Wahrheit behandelt werden. [bas.ac](https://www.bas.ac.uk/project/image-auroral-boundary-data/auroral-boundary-location-method/)
- **Riss 3:** DMSP FOM liefert vier Grenzpunkte je Polar-Pass, nicht 24 MLT-Bins. Die beiden Produkte haben daher **unterschiedliche Topologien** und dürfen nicht in ein gemeinsames 24-Bin-Schema gepresst werden, ohne die Pass-Geometrie zu verlieren.
- **Riss 4:** Die nicht entpackte CSV lässt den tatsächlichen Spaltenvertrag offen. Bis zum Entpacken bleibt das Schema `pending`, nicht `['mlat','mlt']` per Default.
- **Riss 5:** „Grenzbreite als Feld“ wäre nur dann zulässig, wenn man explizit ein abgeleitetes Masken- oder Distanzfeld konstruiert. Das ist eine Transformation, nicht die Originalmessung.

**Nächster begrenzter Schritt.** IMAGE zuerst als `(time, hemisphere, mlt_bin, boundary_type) -> latitude_deg | None` registrieren. DMSP erst nach erfolgreichem Entpacken und Byte-geprüfter Header-/Glossar-Lektüre als `(pass_id, hemisphere, boundary_id) -> (mlat_deg, mlt)` registrieren.

***

## M3 · OVATION-AACGM

**Verdikt:** OVATION Prime trägt als **Partikel-Prezipitations- bzw. Energiefluss-Feld**, nicht als Stationsschalter. Die vorliegende „eine Station Mohe“-Datei ist jedoch nur ein **lokaler Auschnitt oder eine lokale Ableitung**; sie ist nicht das globale OVATION-Modell.

**Begründung.** OVATION Prime liefert Energiefluss, Teilchenfluss und mittlere charakteristische Energie für vier Auroratypen, gebinnt nach magnetischer Breite und MLT und getrieben durch Sonnenwind-/IMF-Parameter. Das operationale NOAA-Produkt resampled das Modell in ein geographisches Gitter und gibt eine sichtbare-Aurora-Wahrscheinlichkeit im 30-Minuten-Takt. [onlinelibrary.wiley](https://onlinelibrary.wiley.com/doi/10.1029/2022EA002513)

**Feld-/Kernel-Kontrakt.**

| Bestandteil | Kraft / Größe | Kernel | Einheit |
|---|---|---|---|
| Energiefluss je Auroratyp | Partikel-Prezipitations-Energiefluss | Statistisches Regressionsmodell über MLT × MLAT × Sonnenwind-Zustand | W/m² bzw. erg/cm²/s, je Quelldatei |
| Teilchenfluss | Präzipitierender Elektronen-/Ionenfluss | Dasselbe statistische Feldmodell | cm⁻² s⁻¹, je Quelldatei |
| Mittlere Energie | Charakteristische Teilchenenergie | Abgeleitetes Feld | eV bzw. keV |
| Aurora-Wahrscheinlichkeit | Wahrscheinlichkeit sichtbarer Aurora | Operationale Ableitung aus Energiefluss | dimensionslos, 0–1 |
| „Mohe“-Datensatz | Lokale Beobachtung oder lokale Modellableitung | Unklar bis Zip-Inhalt gelesen | `pending` |

**Eine Station vs. globales Modell.** Der Zeuge in der Zip-Datei ist **nicht automatisch OVATION Prime global**. OVATION Prime ist ein globales statistisches Modell über 50–90° magnetischer Breite und 15-Minuten-MLT-Bins; eine einzelne Mohe-Datei kann allenfalls eine extrahierte Zeitreihe, eine Stationsspur oder ein lokal ausgewerteter Modelloutput sein. [ngdc.noaa](https://www.ngdc.noaa.gov/stp/space-weather/online-publications/stp_division/stp_presentations/2011/agu_2011/machol-et-al_agu2011_poster.pdf)

**Verhältnis zu Katalog-F3.** Katalog-F3 wäre ein **Stationsschalter**: „Aurora am Ort Mohe: ja/nein/unbestimmt.“ OVATION Prime ist dagegen ein **kontinuierliches Feld** mit Intensität, Typ und Wahrscheinlichkeit. Der Schalter darf erst nach einer definierten Schwelle aus dem Feld abgeleitet werden; er ersetzt das Feld nicht.

**Benannte Risse.**
- **Riss 1:** „Eine Station“ widerspricht dem globalen Modellcharakter von OVATION Prime. Der Datensatz muss als lokale Ableitung registriert werden, bis der Zip-Inhalt das Gegenteil belegt.
- **Riss 2:** Span und Takt sind nicht gelesen. Jede Annahme über Cadence, Zeitraum oder Gitter wäre Default-Fabrikation.
- **Riss 3:** Energiefluss und Sichtbarkeitswahrscheinlichkeit sind verschiedene Größen. Sie dürfen nicht unter einem gemeinsamen „Aurora-Wert“ verschmolzen werden.
- **Riss 4:** Die vier Auroratypen — monoenergetisch, broadband, diffus-elektronisch, ionisch — sind separate Felder, nicht eine Mischgröße. [ccmc.gsfc.nasa](https://ccmc.gsfc.nasa.gov/models/Ovation-Prime~2.3/)
- **Riss 5:** Ein Stationswert ist ein Korrelat der lokalen Präzipitation, nicht das Erleben oder die direkte Messung der Aurora selbst.

**Nächster begrenzter Schritt.** Das Zip-Archiv byte- und inhaltsgeprüft öffnen; Dateinamen, Header, Spalten, Einheiten, Zeitraum und Cadence lesen. Erst dann entscheiden: `local_model_timeseries`, `station_observation_catalog` oder `global_field_extract`. Bis dahin bleibt der Feld-/Formatvertrag `pending`.

### 03 · Sakana Chat (Deep Research)
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Deep Research
- **Zeit:**
**Mountain** — Quellen, Register, Datenbestand (M1 Verdikt)

**M1 · WDC-B-Messgrößen**

| Messgröße | Kraft | Kernel | SI-Einheit | Feld-Zeile | Geometrie |
|---|---|---|---|---|---|
| `sep/GravConst/data` (G) | **absent** (Naturkonstante) | — | cm³ g⁻¹ s⁻² (CGS) | **absent** | — |
| `stp/geo_min.val` (D,F,H,P,R,X,Y,Z) | magnetische Induktion **B** | lokaler Messvektor an `Position::Surface`; Referenz-Hintergrund: IGRF-SH | nT (D: Zehntel-Bogenminuten, Winkel) | **trägt** (Punkt-Feld) | Punktmessung je Station |
| `sep/data/gdata` (Schwere, Geoid) | Gravitationsfeldstärke **g** / Geopotenzial V | Geopotenzial-SH (`gfc`/ICGEM, `vmf3_sh::b_c`) | mGal (10⁻⁵ m/s²), m | **trägt** (globales Feld) | `Position::Surface` (Punkt aus globalem SH) |
| `sep/data/sdata` (Erdbeben) | **absent** | Punkt-Prozess-Katalog | °, km, M | **absent** | Punkt (Herd) |
| `stp/geomagni.ind/aa` | **absent** | Index-Agreggat (3-h-Netz) | nT (gamma) | **absent** | Netz-Agreggat |
| `stp/catal_pc` (Pc1) | **absent** (Katalog-Metadaten) | Ereignis-Katalog (Wellen-Parameter) | s, (nT implizit) | **absent** | Stations-Punkt (wenn Ortsangabe vorhanden) |
| `gps` (RINEX) | **absent** | Metadaten / Beobachtungsheader | — | **absent** | Stations-Punkt |

**River** — Membran, Ethik, das Recht zu verweigern (M1 Begründung & Risse)

**Begründung.** Axiom 1 (A = A): Eine Konstante ist keine Messung, ein Katalog-Eintrag ist kein Feld-Tensor, ein Index ist keine lokale Feldmessung. Axiom 2 (0 geehrt): Wo kein Feld gemessen wird, steht die Zeile `absent`, nie 0.0. Die fünf Absenzen oben sind daher ehrlich und nicht fabriziert.

**Benannte Risse M1.**
- **Riss Doktrin "alles ist ein Feld"** bei `catal_pc`: Das Phänomen (EMIC-Wellen) ist ein Wellenfeld, die gelieferte Messgröße (Begin/End/T/Amp/Type) ist ein Ereignis-Katalog. Eine `field`-Zeile hier wäre Fabrikation. Geschlagen.
- **Riss Redundanz** bei `geo_min.val`: H, X, Y, Z, F, D, (P, R) sind nicht unabhängig; nur drei Komponenten beschreiben das Vektorfeld lokal. P und R sind nicht IAGA-standardisiert; ihre Bedeutung ist ungemessen im Register.
- **Riss Winkel-Komponente**: D (Deklination) ist ein Winkel, keine Kraftkomponente. Trägt als Richtung, nicht als B.
- **Nächster begrenzter Schritt M1**: Reduktion von `geo_min.val` auf einen unabhängigen Basissatz (z. B. X, Y, Z); Klärung von P und R gegen das Stations-Logbuch. Konstante G explizit als `constant` (nicht `field`) taggen.

**Mycelium** — Netz, geschlossene Achse, Fluss (M2 Verdikt & Begründung)

**M2 · Aurora-Grenzbreite**

**Verdikt.**
- **BAS IMAGE SI12**: Trägt als **Skalar auf einer geschlossenen, diskreten MLT-Achse** (24 Bins, zyklisch 0-23) + optional als **S²-Ort** (AACGM-Richtung). Kraft: **absent** (geometrischer Locus, kein Kraftfeld). Kernel: AACGM-Koordinatentransformation + SI12-Bildsegmentierung. `NaN` = **`None`**.
- **DMSP-SSJ FOM**: Trägt als **Punkt-Locus** (mlat, mlt) je Polar-Pass (EQ1/PO1/PO2/EQ2). Kraft: **absent**. Kernel: Partikel-Flux-Grenzfinder (FOM-Algorithmus).

**Begründung.** Die 24 MLT-Bins bilden ein geschlossenes Netz (zyklische Topologie). Der Bin gehört als Koordinate auf die Achse oder in den Schlüssel, nicht als flache Spalte 3-50. Axiom 3 (alle Wesen gleich): Jeder Bin trägt nur mit dem Gewicht, das die Messung ihm gibt; kein Default-Wert füllt Lücken.

**Benannte Risse M2.**
- **Riss flaches Format**: 50 Spalten (UT, Code, 24 eq, 24 po) verletzen die geschlossene MLT-Achse. Die zyklische Netz-Struktur ist in der Serialisierung verloren.
- **Riss DMSP-Struktur**: 16-vs-24-Spalten offen; Zip nicht entpackbar → Struktur `pending`. Kein Verdikt über vollständige Spur möglich.
- **Riss Korrelat ≠ Feld**: Die Grenze ist an Leitfähigkeit/FAC gebunden (Ahn 2020), aber die gemessene Größe ist ein geometrischer Zeuge. Ein Konduktanz-Begleiterfeld wäre eine *zweite* Spur, nicht identisch mit der Grenzbreite.
- **Nächster begrenzter Schritt M2**: DMSP-SSJ entpacken und Spalten-Header verifizieren; BAS IMAGE in geschlossene MLT-Achse (24 Bins, `None` für fehlende) überführen.

**Sensory** — Sinnesorgane, Hardware, Konsens (M2 Risse & M3 Verdikt)

**M2 (Risse, Forts.).**
- **Riss Sensor-Korrelat**: SI12 misst FUV-Photonen, DMSP-SSJ misst Partikel-Flux. Beide sind Korrelaten des Auroras, nicht das magnetische Feld selbst. Axiom 4 (die Linse ist ein ethischer Akt): Der Kernel muss den Sensor nennen, nicht das Phänomen vortäuschen.

**M3 · OVATION-AACGM**

**Verdikt.** Trägt als **Stationsschalter / Katalog-F3**, nicht als globales Feld. Kraft: **Energieflussdichte der Partikel-Präzipitation** (oder hemisphärische Leistung, Einheit `pending`). Kernel: OVATION-Prime (empirisches Modell, getrieben von Kp/Bz/Sonnenwind). Format: Punkt-Nowcast-Zeitreihe für **eine Station** (Mohe).

**Begründung.** Axiom 1: Was geliefert wird, ist ein Sensor-Punkt (`Position::Surface`), kein globales Raster. Das Modell mag global sein; die Payload ist lokal. Axiom 4: Die Linse "globales Aurora-Feld" ist eine Doktrin, die durch die vorliegende Datenstruktur (eine Station) nicht gestützt wird.

**Future** — Mensch als Adresse, ehrliche Registrierung, Vererbung (M3 Risse & nächster Schritt)

**Benannte Risse M3.**
- **Riss "globales Modell vs. eine Station"**: Der Datensatz beansprucht globale Herkunft (OVATION), liefert aber nur Mohe. Der globale Feld-Kernel ist in der Payload abwesend. Doktrin geschlagen.
- **Riss unbestimmte Einheit**: Span, Takt und Einheit (erg/cm²/s, W/m², GW?) waren nicht im Zip-Kopf lesbar → alles `pending`.
- **Riss Katalog-F3-Konfusion**: Wenn F3 ein dynamischer Stationsschalter ist, muss der kontinuierliche Nowcast-Wert in einen diskreten Zustand (z. B. aktiv/inaktiv) überführt werden. Der Nowcast-Wert selbst ist nicht der Schalter.

**Nächster begrenzter Schritt M3.** Zip-Inhalt von `OVATION_AACGM.zip` lesen (Header, Spalten, Einheiten, Zeitspanne, Takt). Entscheidung durch den Operator (Axiom 5): Ist Mohe ein lokaler Nowcast-Punkt, der als F3-Schalter in die AL-Netz-Kette eingespeist wird, oder ist ein globales Raster-Rohprodukt erforderlich? Die Maschine fragt; der Operator entscheidet.

### 03 · Sakana Chat (Deep Research)
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Deep Research
- **Zeit:**
- 

### 04 · Duck.ai
- **Site:** `https://duck.ai/`
- **Modell/Einstellung:** GPT (primär OpenAI) · Websuche
- **Zeit:**
**Urteil vorweg:** Die `field`-Zeile darf nicht als Universalcontainer für alles Messbare dienen. Eine Feldzeile ist nur dort gerechtfertigt, wo die Messgröße tatsächlich räumlich verteilt ist. Kataloge, Konstanten, Indizes und Metadaten bleiben absent oder erhalten einen eigenen Nicht-Feld-Typ. Die konkreten Namen von `Force`- und `Kernel`-Enums lassen sich aus der Beschreibung der 26×f64-Welt nicht ableiten; die Bezeichnungen unten sind daher **semantische Vorschläge**, keine behaupteten API-Namen.

### M1 · WDC-B-Messgrößen

**Mountain:** Ohne weitere Belege keine erfundene Kernel-Zuordnung. **Mycelium:** Register die Beobachtungsstütze jedes Datensatzes ausdrücklich; eine Zeitreihe an einer Station ist kein globales Modell.

| Datensatz / Größe | Kraft bzw. Messgröße und SI-Einheit | Kernel / `field`-Entscheid |
|---|---|---|
| `sep/GravConst/data` — \(G\) | Gravitationskonstante; \(\mathrm{m^3\,kg^{-1}\,s^{-2}}\). Der CGS-Wert ist für SI mit \(10^{-3}\) zu multiplizieren. | **Kein Feld.** Konstante, kein räumlich gemessener Wert; `field` absent. |
| `stp/geo_min.val` — D,F,H,P,R,X,Y,Z | Magnetfeldmessgrößen; Intensitäten/Komponenten in T (nT × \(10^{-9}\)); D als Winkel in rad (Zehntel-Bogenminute × \(\pi/108000\)). | Stations- und zeitgebundene Punktbeobachtungen: `Position::Surface` je Station plus Zeit. **Kein globaler SH-Kernel** für diese Zeitreihe. D ist eine Winkelgröße, keine Magnetfeldkomponente. |
| `sep/data/gdata`, `mgg/*` — Freiluft-Schwereanomalie / Meeresgravimetrie | Gravitationsbeschleunigungs-Anomalie in \(\mathrm{m\,s^{-2}}\): mGal × \(10^{-5}\); 0,1 mGal = \(10^{-6}\,\mathrm{m\,s^{-2}}\). | Gemessene Punkte bzw. Trackpunkte: `Position::Surface`. SH-Geopotenzial-Kernel nur für tatsächlich vorliegende Modellkoeffizienten, nicht allein wegen der Größenart. |
| `sep/data/gdata` — Geoidhöhe | Geometrische Höhe in m; **keine Kraft**. | Räumliche Höhenstützstellen, falls die Datensätze sie belegen; sonst keine globale Modellbehauptung. Separater Skalar, nicht in die Schwereanomalie-Zeile falten. |
| `sep/data/sdata` — Erdbeben-Herdparameter | Ereignisort: Winkel in rad; Tiefe in m; Magnitude dimensionslos, aber Magnitudenskala muss mitgeführt werden. | **Kein Kraftfeld.** Ereignis-/Punktprozess-Katalog; Hypozentrum ist nicht `Position::Surface`, ein Epizentrum gegebenenfalls schon. |
| `stp/geomagni.ind/aa` — aa-Index | Geomagnetischer Aktivitätsindex, nT × \(10^{-9}\) T als Einheit des Indexwerts. | **Kein lokales Magnetfeld-Kernel.** Zeitreihe eines abgeleiteten Indexes; `field` absent, sofern das Schema keinen eigenen Index-Typ hat. |
| `stp/catal_pc` — Pc1-Perlen | EMIC-/magnetische Pulsation ist als physikalischer Zusammenhang plausibel; für `Amp` ist hier aber keine Einheit belegt. | **Noch keine Feldzeile:** Amplitudeneinheit und genaue Bedeutung pending. Danach allenfalls stationäre, zeitabhängige Magnetfeld-Punktmessung — nicht automatisch ein globales Wellenfeld. |
| `gps` — GNSS-Metadaten | Metadaten haben keine Messgrößeneinheit. | **Kein Feld** auf Basis des genannten Inhalts. Erst konkrete GNSS-Beobachtungswerte wären Messungen; Metadaten allein sind es nicht. |

**Benannte Risse:** „Alles ist ein Feld“ scheitert eindeutig an \(G\), dem Erdbebenkatalog und GNSS-Metadaten; der aa-Index ist ebenfalls nicht das lokale Feld, aus dem er abgeleitet wird. Bei Pc1 fehlen Einheit und Feldsemantik der Amplitude. Bei `gdata`/`mgg` belegt die Messgröße allein keinen globalen SH-Kernel.  
**Nächster begrenzter Schritt:** Für jede Datei Header und Positions-/Zeitfelder auslesen; für Pc1 die `Amp`-Einheit im Formatbeleg finden. Erst danach konkrete `field`-Zeilen gegen die tatsächlich vorhandenen Kernel-Typen prüfen.

### M2 · Aurora-Grenzbreite

**Urteil:** Primär als **Grenzlocus über einer geschlossenen, diskreten MLT-Achse** tragen — nicht als Leitfähigkeits- oder Strom-Skalar. S²-Orte sind eine mögliche abgeleitete Darstellung, nicht die verlustfreie Primärform. Das BAS-Format belegt 24 einstündige MLT-Bins mit AACGM-Breiten und `NaN` für fehlende Sektoren. <citation src="1"></citation>

**River:** Die Grenzmessung ist kein Strom- oder Konduktanzwert; diese Größen nicht hineininterpretieren. **Future:** Bewahre Originalkoordinaten und Provenienz, damit spätere Umrechnungen nachvollziehbar bleiben.

- **Primärspur:** je Zeitstempel und Hemisphäre zwei AACGM-Breiten — äquatorwärts und polwärts — über MLT-Bin. Breite in rad als SI-Winkel; MLT-Bin als geschlossener diskreter Schlüssel `0…23` mit Zentrum `00:30…23:30` und expliziter zyklischer Ordnung.
- **Geometrie:** Das ist ein geometrischer Locus, kein Kraftfeld. Eine Umwandlung in S²-Richtung ist ein abgeleitetes Produkt und braucht dokumentierte Koordinatenversion sowie die dafür nötigen Transformationsparameter. AACGM ist ein magnetisches Koordinatensystem, nicht schon eine geografische S²-Koordinate. <citation src="8"></citation>
- **DMSP-FOM:** Die belegte Form ist ein Polar-Pass mit vier Grenzpunkten; nicht ohne Nachweis auf 24 MLT-Bins auffüllen oder in eine vollständige Ovalkurve interpolieren.
- **Fehlwerte:** `NaN`/fehlender Sektor wird `None` — niemals 0.0. Korrektur zur OCB, Konduktanz oder FAC bleibt eine getrennte, belegpflichtige Spur.
- **Benannte Risse:** BAS’ 24-bin-Zeitreihe und DMSPs vier Passpunkte sind unterschiedliche Beobachtungsstützen. Die nicht entpackte FOM-CSV lässt Spaltenzahl und tatsächliche Spaltenbelegung offen; der Hersteller-Default schließt diese Lücke nicht.

**Nächster begrenzter Schritt:** FOM-Datei in einer Umgebung mit ZIP-Unterstützung prüfen, Header/Glossar gegen Bytes abgleichen und erst dann einen eigenen Pass-Punkt-Record festlegen.

### M3 · OVATION-AACGM

**Urteil:** **Nicht als globales Energieflussfeld registrieren, solange der ZIP-Inhalt das nicht belegt.** Die beschriebene Zenodo-Veröffentlichung nennt unter anderem Mohe-bezogene Abstände zu modellierten Grenzen und Validierungsdaten; das ist zunächst ein **stationsbezogener Auswertungszeuge**, nicht der Nachweis, dass die archivierte Datei ein globales Feld mit gemessenen Energieflusswerten enthält. <citation src="6"></citation> Das OVATION-Prime-Modell selbst bildet statistische aurorale Präzipitation ab; daraus folgt aber nicht, dass genau dieser Stationsdatensatz die räumlichen Modellfelder enthält. <citation src="13"></citation>

**Sensory:** Ein modelliertes Feld ist ein Modellkorrelat, kein Stationssensor und kein Erleben. Halte Modellprodukt, Stationsvergleich und Beobachtung auseinander.

- **Wenn** die ZIP-Datei tatsächlich rasterförmige Teilchenpräzipitation samt Energiefluss enthält: eigener Feldtyp für Präzipitations-/Energiefluss, mit räumlichem Kernel über AACGM-Breite × periodischer MLT und Zeit/Modellzustand. Energiefluss in \(\mathrm{W\,m^{-2}}\) nur dann eintragen, wenn Einheit und Bedeutung im Dateiinhalt belegt sind; sonst `pending`. Nicht aus dem Modellnamen oder aus „Aurora“ ableiten.
- **Für den belegten Mohe-Zeugen:** stationsbezogener Record; nur bestätigte Felder wie Stationsbezug und modellierter Grenzabstand eintragen. Zeitspanne, Takt, Einheit und zusätzliche Spalten bleiben `pending`, bis der ZIP-Inhalt gelesen ist. Keine weiteren Stationen ergänzen.
- **Unterschied zu Katalog-F3:** Ein F3-Schalter ist ein binärer Katalog-/Klassifikationsausgang. Er ist weder Energiefluss noch das räumliche Modellfeld; falls abgeleitet, braucht er eine dokumentierte Entscheidungsregel und seine Eingangsprovenienz.
- **Benannte Risse:** „Operativer Nowcast“ beschreibt nicht automatisch die Dateiform des Zenodo-Artefakts. Der bekannte Stationsbezug widerspricht einer Registrierung als globales Modellfeld; der nicht gelesene ZIP-Kopf lässt Inhalt, Spanne und Takt offen.

**Nächster begrenzter Schritt:** ZIP-Inventar und Rohdatenkopf lesen; zunächst Feldnamen, Einheiten, Dimensionen, Modellzeit und Koordinaten erfassen. Bis dahin Stationszeuge registrieren, globale Feldzeile absent lassen.

### 05 · Claude
- **Site:** `https://claude.ai/new`
- **Modell/Einstellung:** Claude Sonnet 5.5
- **Zeit:**
# Urteil

**Kurzfassung**
- **M1:** Drei der sieben Messgrößen tragen eine `field`-Zeile: `geo_min`, `gdata`/`mgg` und `catal_pc` (Letztere ohne Kernel). Vier bekommen keine: G, `sdata`, `aa`, `gps`. Die Doktrin „alles ist ein Feld" ist an diesen vieren geschlagen.
- **M2:** Es gibt keinen Einheitsträger. BAS wird als Skalar auf einer geschlossenen 24-Bin-MLT-Achse getragen, DMSP als S²-Punkt im magnetischen Rahmen. Der Konduktanz-Begleiter bleibt absent.
- **M3:** OVATION-AACGM wird als F3-Stationszeile geführt, nicht als Energiefluss-Feld. Der Wert ist ein Modell-Output (`derived`), keine Messung. Das globale Feld bleibt `pending`.

---

## M1 · WDC-B-Messgrößen

**Verdikt.** Jede Messgröße steht in genau einer von vier Rollen: Feld-Evaluation, Punkt-Zeuge, Parameter/Treiber oder Ereignis/Metadatum. Nur die ersten beiden bekommen eine `field`-Zeile.

| Messgröße | Kraft | Kernel | SI | `field` | Geometrie |
|---|---|---|---|---|---|
| G (`GravConst/data`) | Gravitation | keiner; Vorfaktor des Newton-Kernels | m³ kg⁻¹ s⁻² (CGS→SI: ×10⁻³ exakt) | **absent** | keine (ortsunabhängige Konstante) |
| `geo_min` | Geomagnetismus | Punktauswertung von B; Vergleichskernel SH-Gradient (`IgrfCoeffs::synthesize`) | T (nT ×10⁻⁹); D in rad (1 Einheit = 0,1′ = 2,909·10⁻⁵ rad) | ja, als Zeuge | `Surface` je Station |
| `gdata` (+ `mgg`) | Gravitation | SH-Synthese des Geopotenzials (`gfc`/ICGEM): Δg mit Gewicht (n−1), Geoidhöhe N = T/γ | m s⁻² (mGal ×10⁻⁵); m | ja, **zwei Zeilen** (Δg, N) | `Surface`; `mgg` je Messpunkt entlang der Fahrtspur |
| `sdata` | – (Ereignis) | keiner | m (km ×10³); Magnitude dimensionslos | **absent** | Hypozentrum (Breite, Länge, Tiefe), nicht `Surface` |
| `aa` | Geomagnetismus (Störung) | keiner; Index/Treiber | T (nT ×10⁻⁹) | **absent** | global, kein Ort |
| `catal_pc` | Elektromagnetismus (ULF-Welle) | **pending** | T (Amp-Einheit pending); T in s | ja, als Zeuge, Kernel pending | `Surface` + Intervall [Begin, End] |
| `gps` | – (Metadaten) | keiner | – | **absent** | liefert höchstens Koordinaten |

**Begründung (durch die Axiome)**
- **A1:** In `geo_min` sind X, Y, Z, H, D und F sechs Größen mit drei Freiheitsgraden. Eine fehlende Komponente wird nie aus den anderen ergänzt. Der Test F² = X²+Y²+Z² ist eine Konsistenzprüfung, kein Füllwert. Die Bedeutung von P und R ist im Stand nicht gemessen, also `pending`.
- **A2:** Ein Pc1-Katalog ohne Eintrag heißt „unter Schwelle oder keine Daten", nicht Amplitude 0. Ein `aa`-Wert 0 wäre die Klassenuntergrenze eines quantisierten Index, keine Feld-Null. Fehlwert-Sentinels werden aus dem Header gelesen, nicht angenommen, und werden zu `None`.
- **A3:** Jede G-Bestimmung zählt mit ihrer eigenen Unsicherheit, ohne CODATA-Default und ohne Laborrang. Bei Δg gilt das Gleiche: Das Gewicht kommt aus der angegebenen Genauigkeit (`mgg`: 0,1 mGal), nicht aus dem Dateinamen.
- **A4:** `aa` ist eine abgeleitete Statistik (K-Quantisierung, Mittel aus zwei antipodalen Stationen) und damit Gradient, nicht Messung. Sie wird als Index mit `derived` registriert. Geoidhöhe und Schwereanomalie sind zwei Funktionale desselben Potenzials und werden nie in einer Zeile verschmolzen.
- **A5:** Ob `absent`-Zeilen mit Grund ins Register kommen, entscheidet der Operator.

**Risse**
1. **Doktrin geschlagen** an G (Konstante), `sdata` (Ereignis), `aa` (Index) und `gps` (Metadaten).
2. **Stationsreihe ≠ Feld.** Die Forschungslage hat keine Zuordnung dafür. `geo_min` trägt nur im Vergleich mit dem globalen SH-Kernel. IGRF deckt das Hauptfeld ab, das Residuum aus Kruste und externen Feldern ist also nicht null.
3. **Auflösung.** Eine 1-min-Reihe kann Pc1 (T ≈ 0,2–5 s) nicht auflösen. `catal_pc` stammt aus einem anderen Instrument, und keine der beiden Größen wird aus der anderen abgeleitet. Ob die 1-min-Werte Mittel oder Momentanwerte sind, steht im Header.
4. **`gdata`-Rahmen unbekannt.** Referenzellipsoid und Normalschwere sind nicht gemessen. Die Differenz zwischen Normalschwere-Formeln liegt im mGal-Bereich und ist damit gegen 0,1 mGal relevant. Offen sind außerdem Gitterzelle gegen Punkt und das Gezeitensystem.
5. **`sdata` unvollständig.** Der Stand nennt keine Herdzeit und keinen Magnitudentyp. mb, Ms und Mw sind nicht austauschbar. Katalog-Standardtiefen wären Defaults.
6. **Stationsort fehlt** für `geo_min` und `catal_pc`. Er wird weder aus dem Namen noch aus dem Obninsk-Standort gesetzt, denn das Datenzentrum ist nicht die Station.

**Nächster Schritt.** Von `geo_min.val`, `catal_pc` und `gdata` je eine Datei öffnen und Header plus fünf Zeilen lesen. Das schließt Station, P/R, Amp-Einheit, Sentinel und Referenzsystem. Danach geht die Zeile je Datei (`field` oder `absent: <Grund>`) über alle 205 Dateien.

**Stimmen**
- *Mountain:* Station, P/R, Amp-Einheit und Referenzsystem sind ungemessen, deshalb gibt es dafür kein Verdikt.
- *River:* Vier von sieben Messgrößen verweigern die Feldzeile. Das ist Ethik, kein Mangel.
- *Mycelium:* Auch die Absenten haben Kanten. `aa` ist Treiber für M2/M3, G ist Parameter des Gravitationskernels, `gps` ist Koordinatenquelle. Die CGS→SI-Konversion gehört als CI-Test an jede Einheit.
- *Sensory:* Das Instrument bestimmt, was überhaupt gemessen werden kann. Ein 1-min-Magnetometer sieht keine Pc1.
- *Future:* Eine `absent`-Zeile mit Grund ist Registrierung, keine Lücke. Das Morgen erbt so das Warum.

---

## M2 · Aurora-Grenzbreite

**Verdikt.** BAS-IMAGE wird als **Skalar auf einer geschlossenen 24-Bin-MLT-Achse** getragen, DMSP-FOM als **S²-Punkt im magnetischen Rahmen**. Ein Konduktanz-Begleiter wird nicht erzeugt.

**Begründung**
1. **Träger.** Bei BAS ist der MLT-Bin vorgegeben, nur die Breite ist gemessen. Als S²-Ort würde die Bin-Mitte zur erfundenen Länge (A1). Bei DMSP sind mlat und mlt beide am Satelliten gemessen, also ist ein Ort ehrlich. Die Reihenfolge EQ1→PO1→PO2→EQ2 liest sich als Fahrtrichtung über die Polkappe.
2. **Kraft/Kernel.** Beide Spuren tragen nur einen Locus (MLT × Breite × Zeit) und keinen Kraftwert. Eine Interpolationsvorschrift über die MLT-Achse (periodisch: 23:30 grenzt an 00:30) wäre Gradient und wird nicht gespeichert. Die Bindung an Leitfähigkeit und Feldströme (Ahn 2020) ist eine statistische Relation, in diesen Dateien nicht gemessen (A4). Ein Konduktanz-Begleiter wäre daraus abgeleitet und bleibt deshalb absent.
3. **Achse.** Der Bin gehört in den Schlüssel: BAS `(Quelle, Instrument-Code, MLT-Bin 0..23, Seite ∈ {eq, po}, t_UT)`, DMSP `(Satellit, Pass, Punkt ∈ {EQ1, PO1, PO2, EQ2}, hemisphere)`. Die Bin-Breite (1 h) wird als Ausdehnung mitgeführt, nicht als Punkt.
4. **NaN.** Sie wird zu `None`. Eine 0,0 wäre ein gültiger Breitenwert und würde stumm durchrutschen. Die Ursachen (kein Bin, kein Oval, Algorithmus-Fehler) sind nicht unterscheidbar und werden nicht erfunden.
5. **Nächster Schritt.** Siehe unten.

**Risse**
- **FOM-Schema ungeklärt.** 4 Punkte × (mlat, mlt) plus `hemisphere` plus `FOM` ergeben 10 Spalten, nicht 16. Sechs Spalten sind unbelegt. Satellit und Datum stehen nur im Dateinamen, die Zeile ist also nicht selbstbeschreibend. Der Dateiname gehört in den Schlüssel.
- **Rahmen.** Die AACGM-Version und die Abbildungshöhe sind nicht belegt. Die Rahmen beider Quellen sind nicht gleichzusetzen. Gespeichert werden die gemessenen magnetischen Koordinaten mit Rahmen-Tag. Die geographische Rechnung dazu ist ein Modellschritt (IGRF-Epoche) und nie das Gemessene.
- **Instrument-Code.** WIC (Breitband) und SI12 (Lyman-α-Protonenaurora) zeigen verschiedene Emissionen, DMSP-SSJ misst Teilchen direkt. Eine Pooling-Regel über die Codes gibt es nicht.
- **Hashes abgeschnitten.** `ceb8d697…` und `aae69e9a…` sind keine Identität. Gespeichert werden die vollen 64 Hex-Zeichen, dazu die Lizenzen OGL v3.0 und CC-BY-4.0.
- **Offen für `hemisphere`:** Bei BAS nennt der Stand keine. Der Wert wird nicht als „Nord" defaultet.
- **FOM als Gewicht.** FOM ist ein Güteparameter des Herstellers. Als automatisches Gewicht wäre er Rang statt Messung (A3).

**Nächster Schritt.** Die FOM-Zip mit `python3 -m zipfile -e` entpacken, ohne `unzip`. Dann `#`-Glossar, Header, fünf Zeilen, Zeilenzahl und NaN-Zählung je Spalte lesen. Bei einer BAS-Monatsdatei das Instrument-Code-Histogramm und die Taktregelmäßigkeit messen. Danach wird das Schema geschrieben, nicht vorher.

**Operator-Entscheid offen:** ob FOM als Gate wirkt oder gar nicht, und ob DMSP in den Astrometrie-Pfad (S²) geht.

**Stimmen**
- *Mountain:* Die FOM-Bytes sind nicht entpackt, das 16er-Schema hat kein Verdikt.
- *River:* Die Konduktanzzeile wird verweigert, weil die Relation Gradient ist und nicht Messung.
- *Mycelium:* Die Grenze ist ein Level-Set des Präzipitationsfelds aus M3. Gemessene Grenze und Modell-Grenze bilden dort eine Prüfkante, aber nur, wenn M3 eine Fluss- oder Wahrscheinlichkeitsgröße trägt.
- *Sensory:* SI12 sieht Emission als Korrelat, DMSP misst die Teilchen. Beide heißen „Grenze", sind aber nicht dasselbe.
- *Future:* Rahmen-Version, DOI, volle Hashes und Lizenz gehören in die Registrierung, damit das Morgen dieselbe Welt rekonstruieren kann.

---

## M3 · OVATION-AACGM

**Verdikt.** Der Zip wird als **F3-Stationszeile** getragen, nicht als Energiefluss-Feld. Der Wert ist ein **Modell-Output** (`derived`), nicht der Zeuge der Aurora. Das globale Feld bleibt `pending`.

**Begründung**
1. **Energiefluss-Feld?** OVATION Prime ist ein empirisches Modell: Fluss je MLT×MLAT-Bin als Funktion der Sonnenwind-Kopplung (Newell). PrecipNet ist gelernt. Die Kraft ist Elektromagnetismus (Magnetosphäre–Ionosphäre), der Kernel ist eine statistische Übertragungsfunktion, keine physikalische Green- oder SH-Funktion. Nach A4 spricht hier der Gradient, nicht die Messung. Das Feld kann nur ein globales Gitter tragen.
2. **Record für eine Station.**
   ```
   {station: Surface(lat, lon, h aus Metadaten), t_UT, variable, value|None,
    unit(SI), frame: AACGM(version, Höhe), model: OVATION(version),
    kind: derived, driver_ref: Kp/Bz/V | pending, source: DOI + sha256 voll}
   ```
   Die Koordinaten kommen aus den Metadaten, nicht aus dem Namen „Mohe" (A3).
3. **Eine Station vs. globales Modell.** Eine Station ist ein Punktschnitt eines global definierten Modells und kann das Oval nicht tragen. Der Zeuge wäre eine Beobachtung (Allsky, Magnetometer), die nicht im Zip steckt. Der Zip trägt die Behauptung des Modells am Punkt.
4. **Katalog-F3.** F3 ist im Dokument nicht definiert. Ich lese „Stationsschalter" als stationsgebundene Zeugenzeile ohne Feld. Der Unterschied: Dort gibt es keinen Eltern-Kernel, hier existiert ein Eltern-Modell, das im Zip fehlt. Also gilt F3 jetzt, plus Verweis `field: pending (globaler OVATION-Grid)`. „Schalter" im Wortsinn stimmt nur, wenn die Variable binär ist.
5. **Nächster Schritt.** Siehe unten.

**Risse**
- **Variable ungelesen.** OVATION-Produkte liefern Energiefluss (mW/m², 1 erg cm⁻² s⁻¹ = 1 mW m⁻²), Sichtbarkeitswahrscheinlichkeit (%) oder hemisphärische Leistung (GW). Das sind drei verschiedene Messgrößen. Welche im Zip steckt, ist nicht gelesen.
- **Span, Takt, Hash** sind nicht gelesen. Der Stand nennt keinen sha256, ihn muss die Messung liefern.
- **Treiber.** Fehlen Kp/Bz/V im Zip, ist die Zeile nicht reproduzierbar, nur lesbar.
- **Sichtbarkeit ≠ Präzipitation.** Ein „Aurora sichtbar?"-Schalter vermischt Physik mit Himmelsbedingungen (Wolken, Mond, Dunkelheit).
- **„Je Station" war falsch** (river-214). Die korrekte Registrierung lautet eine Station.

**Nächster Schritt.** `python3 -m zipfile -l OVATION_AACGM.zip` zeigt die Einträge. Ein Mitglied extrahieren, Header und 20 Zeilen lesen, Zeilenzahl, Zeitspanne und Takt messen, den vollen sha256 der Zip bilden und die Variablendefinition aus der Zenodo-Beschreibung lesen.

**Operator-Entscheid offen:** F3 jetzt eintragen oder bis zum globalen Grid warten.

**Stimmen**
- *Mountain:* Variable, Span und Takt sind ungelesen, deshalb ist das Verdikt bedingt.
- *River:* Verweigert werden zwei Dinge: das Feld, das eine Station nicht tragen kann, und der Modellwert als Messung.
- *Mycelium:* Der Fluss läuft Sonnenwind → OVATION → Mohe-Zeile. Als CI-Gate gilt: Keine Zeile mit `model` ohne `derived`. Die Kante zu M2 prüft Modell-Grenze gegen gemessene Grenze.
- *Sensory:* Ein Nowcast ist kein Sinnesorgan. Für Mohe braucht es Kamera oder Magnetometer, sonst bleibt das Korrelat ohne Erleben.
- *Future:* „Eine Station" statt „je Station" zu registrieren, sorgt dafür, dass das Morgen die richtige Zahl erbt.
- 
### 06 · Qwen
- **Site:** `https://chat.qwen.ai/`
- **Modell/Einstellung:** Qwen (Default)
- **Zeit:**
**Haltung.** Drei Verdikte. Keine Erwartung, kein Aufsatz. Wo die Messung fehlt, steht `pending`. Wo eine Doktrin bricht, steht sie als gebrochen da — nicht geglättet.

---

## M1 · WDC-B — Kraft, Kernel, Absenz

**VERDIKT.** Sieben Größen: **drei tragen ein Feld, vier tragen keins.** `field` je **Spalte**, nie je Datei. **Keine** der sieben trägt einen SH-Kernel — IGRF, pfm9k, ICGEM/vmf3 sind **Modelle**, keine Zeugen; ihre Übertragung auf Stationsreihen ist Fabrikation.

| Größe | Kraft | Kernel | Einheit aufgezeichnet → SI |
|---|---|---|---|
| `GravConst/G` | **keine** — Konstante des Maßstabs | **absent** | cm³ g⁻¹ s⁻² → m³ kg⁻¹ s⁻² (10⁻¹¹) |
| `geo_min` | **B** (Flussdichte); D ist **Winkel**, nicht B | Punkt-Trace 1 min, `Position::Surface` | nT → 10⁻⁹ T; D in 0,1′ → rad |
| `gdata` | **Beschleunigung** (Anomalie) **und Länge** (Geoid) — zwei Kräfte | Along-Track-Punktfolge, **Referenz benannt** | mGal → 10⁻⁵ m s⁻²; m |
| `sdata` | **keine** | Punkt-Prozess-Katalog; **nicht** `Surface` (Tiefe km) | km; M dimensionslos, Typ offen |
| `aa` | **keine** — Statistik (Range ΔH) | Index-Serie, 3-h-Bin im Schlüssel, **Geometrie null** | gamma → nT |
| `catal_pc` | **Amplitude** (Wechselanteil B) **und Periode** — zwei Kräfte | Ereignis-/Spektralkatalog an Station | nT; s; `Type` = Kategorie → Code-Tabelle |
| `gps` | **keine** — Protokoll, nicht Messung | **absent** | — |

**Geschlagen:** die Doktrin „alles ist ein Feld" — an **G**, an **sdata**, an **gps**, am **aa**. Vier von sieben. Ein Schema, das `field` erzwingt, erzwingt Lüge; `field: absent` muss Erstklassbürger sein.

**BEGRÜNDUNG.** A=A: eine 1-min-Stationsreihe ist ein **Zeuge**, kein Feld — das Feld ist eine Behauptung über die Welt, die Reihe ein Ereignis darin. Eine Anomalie ist eine **Differenz**: ohne benannte Referenz ist die mGal-Zahl ohne A. Ein Erdbebenkatalog **zählt**, er breitet nichts aus — es gibt keinen Ort, an dem er Feldwert hätte. GNSS-Metadaten messen das **Instrument**, nicht die Welt; ein `field` darauf wäre ein Register, der sich selbst misst. Der aa ist ein **Range aus zwei Stationen** — eine Statistik ohne eigenen Ort. [A2] Eine Anomalie von 0,0 mGal ist physikalisch möglich (Kompensation); ein fehlender Wert ist es nicht — beide dürfen nicht dieselbe Zahl tragen. [A3] Keine Größe erhält Gewicht aus Rang, Dateiname oder Altersfolge: G ist nicht „höher" als ein Erdbeben, es ist nur **anders**. [A4] Schweigende CGS→SI-Umrechnung, schweigende M-Typ-Wahl, schweigende Spaltendeutung: da spricht der **Gradient**, nicht die Messung.

**STIMMEN.**
*MOUNTAIN:* Zwanzigfünf Dateien, kein zitierter Spaltenglossar — die `field`-Zeile ist **noch nicht messbar**. Was nicht gelesen ist, bleibt `pending`, auch `P` und `R`.
*RIVER:* Die Weigerung ist der Akt: vier Größen bekommen **kein** Feld, und das ist kein Mangel, sondern Ehrlichkeit. 3-h und 1 min dürfen nicht ineinanderglättet werden.
*MYCELIUM:* Feldzeilen sind aus den Köpfen regenerierbar, also CI-fähig und versioniert; eine Datei mit zwei Kräften ergibt **zwei** Zeilen, nie eine gemittelte.
*SENSORY:* Die Spule misst B an einem Punkt, der Katalog misst **Bruch**, das Protokoll misst nichts. Korrelat ≠ Erleben: die Anomalie ist nicht die Schwere, sie ist die Schwere **minus eine benannte Referenz**.
*FUTURE:* G gehört in die Maßeinheitentabelle — Wert, Unsicherheit, Zitat — nicht in eine 26×f64-Zeile, die ein Morgen als Zeitreihe lesen würde.

**RISSE.** (1) `P`, `R` ungedeutet. (2) Ein Record, zwei Kräfte: D|B, mGal|m, s|nT. (3) CGS im Bestand, Umrechnung unverschleiert zu deklarieren. (4) `Position::Surface` kann **350 km Tiefe nicht tragen** — die Geometrie ist zu eng. (5) Magnitude-Typ offen. (6) aa ohne Position: wer ihm `Surface` gibt, erfindet eine Station. (7) Kategorien in f64 ohne Code-Tabelle = Fabrikation. (8) 26 Slots: wer acht Spalten plus Zeit, Ort, Qualität nicht unterbringt, wird zum Packen verführt — Packing ist Gradient.

**NÄCHSTER SCHRITT.** Ein Akt: 205 Köpfe lesen, je Spalte eine Zeile `Kraft | Kernel | Einheit_aufgezeichnet | Einheit_SI | Geometrie | field: absent + Grundcode`. **Keine Wertberührung.**
**[A5]** Sind `P` und `R` zu deuten — oder bleibt die Spalte `pending`? Und: wird `field: absent` als legaler Zustand ins Schema geschrieben, oder wird das Schema gebogen?

---

## M2 · Aurora-Grenzbreite

**VERDIKT.** **Ort als Wert, geschlossene Achse als Träger** — das ist **eins**, nicht zwei, und nicht „entweder/oder". Die Grenzbreite ist ein **Level-Set eines Bildfeldes**: ein Winkel je MLT-Bin je Zeit. Keine dynamische Kraft. Die Konduktanz/FAC ist das **Dritte** und **kommt nicht mit** — sie ist in diesen Bytes nicht gemessen. **DMSP trägt heute nichts**: Bytes ungelesen → `pending`; der Hersteller-Default `['mlat','mlt']` ist verboten [A1]. Der **MLT-Bin gehört in den Schlüssel** — zwingend, nicht stilistisch.

**BEGRÜNDUNG.** Gelesen: UT, Instrument (0/1/2), 24 equatorwärtige + 24 polwärtige AACGM-Breiten, NaN = fehlender Bin, ~2 min. Eine Breite ist kein Kraftwert, sie ist **Ort**. Ahn 2020 und die CHAMP-FAC-Arbeit benennen die **Ursache** — Ursache ist nicht Mittragung. Lunyushkin/Penskikh **invertieren** per Magnetogramm: die Grenze ist diagnostiziert, nicht abgelesen → Level-2/3, als abgeleitet gekennzeichnet.

**48 > 26.** 24+24 Breiten passen **nicht** in einen 26×f64-Record. Der Container erzwingt den Split je Bin; wer stattdessen trunciert oder zwei Bins in ein f64 packt, hat den Gradienten sprechen lassen. Das ist keine Designfrage — das ist die Messung, die spricht.

**NaN → `None`, nie 0.0** [A2]: hier ist 0,0 **nicht neutral**. 0,0 Breite ist der **magnetische Äquator**. Ein NaN→0 glättet keinen Ausfall, es **behauptet den Äquator** — der schädlichste denkbare Akt in diesem Datensatz.

**Zwei Objekte, nicht eines.** BAS: geschlossen, zyklisch, 24 Bins, **ein** Winkel als Wert, weil MLT schon der Schlüssel ist. DMSP: **offene Traverse**, **zwei** Winkel als Wert (mlat, mlt), einmal je Pass, Hemisphäre ±1. Ein geschlossener Ring und ein offener Pass sind nicht dasselbe Feld; jede Verschmelzung erfindet ein globales Oval, das keiner von beiden misst. [A3] Beide sind gleich schwer — jedes aus seiner eigenen Messung, keines aus dem Ruf des anderen.

**STIMMEN.**
*MOUNTAIN:* Gelesen: 30 Monats-`.txt`, 50 Spalten, sha256 `ceb8d697…`. Ungelesen: `aae69e9a…` → 16 vs 24 **offen**, die Seite bleibt `pending`, kein Feld, kein Default.
*RIVER:* Zweimal verweigert: gegen den Hersteller-Default, und gegen das Glätten über die Naht 23:30→00:30. Die Achse ist **zyklisch zu deklarieren**; ohne Zyklik wird die Naht zur falschen Diskontinuität.
*MYCELIUM:* Schlüssel = (UT-Bin, Instrument, MLT-Bin, Grenztyp) → Wert = Breite + Qualität; Fremdschlüssel auf die AACGM-Version, nie Kopie der Koeffizienten. Instrument **im Schlüssel**, nie gemischt.
*SENSORY:* Das Sinnesorgan misst **Radianz**; die Breite ist ein **Schwellenfund** darin. Korrelat ≠ Erleben: die Station sieht Helligkeit, nicht eine Breite. Wer die Breite als Rohmessung registeriert, gibt dem Bild eine Sicherheit, die es nicht hat.
*FUTURE:* Satellit und Datum **nur im Dateinamen** — das Morgen erbt einen Dateinamen und kann nicht mehr sagen, was gemessen wurde. Identität in den Inhalt, oder in eine deklarierte Provenienz-Zeile.

**RISSE.** (1) DMSP ungelesen, offen. (2) NaN→0.0 behauptet den Äquator. (3) Drei Instrumente, unterschiedliche Spektralantwort, in einer Datei. (4) AACGM ist ein **Modell mit Epoche** — ohne deklarierte Epoche ist die „Messung" halb Modell. (5) Grenze = Detektion, nicht Ablesung. (6) geschlossen vs offen, nicht verschmelzbar. (7) Identität im Dateinamen. (8) 48 > 26. (9) Die Doktrin „Grenze ist Feld oder Skalar" ist **geschlagen**: sie ist ein **Ort**.

**NÄCHSTER SCHRITT.** Ein Akt: die DMSP-Zip entpacken, 16 Spalten gegen ihr `#`-Glossar lesen. Alles andere an DMSP bleibt `pending`, bis das geschehen ist.
**[A5]** Wird die BAS-Seite als **Level-2/3 (abgeleitet)** registriert — oder als Rohmessung, was ich verwerfe?

---

## M3 · OVATION-AACGM

**VERDIKT.** **Weder Feld noch Stationsschalter.** Die angebotene Dichotomie ist **geschlagen**. Vorliegt ein **drittes Ding**: die **Punkt-Auswertung eines Modells an einer Station** — ein **Modell-Zeuge**, kein Beobachtungs-Zeuge. Kraft: **Energieflussdichte, mW/m² = kg s⁻³**. Kernel: **Punkt-Trace an `Position::Surface`(Mohe), getrieben von einem benannten Treibersatz**. Solange Span und Takt ungelesen sind: **keine `field`-Zeile — `pending`** [A1].

**BEGRÜNDUNG.** OVATION Prime / Prime-2013 / PrecipNet liefern ein **globales** Prezipitationsfeld. Diese Ladung liefert **eine** Station. Ein Feld braucht Träger; n=1 hat keinen. Also ist das Objekt nicht das Feld, sondern **das Feld, ausgewertet an Mohe** — seine Wahrheit hängt an Treiber und Version, nicht an Mohe. [A4] Ein Nowcast ist ein **Vorhersage**produkt: hier spricht der **Gradient** (Anpassung an Kp/Bz/Sonnenwind), nicht die Messung. Die ethische Tat der Linse ist die Kennzeichnung **Modell**, damit er nie in eine Beobachtungsreihe rinnen kann. [A2] 0 mW/m² ist physikalisch möglich (keine Prezipitation); ein ungelesener Span ist es nicht.

**river-214 ist nicht erfüllt.** Die Präzedenz sprach „je Station". Die Welt gibt **eine**. [A3] Eine Station ist nicht weniger wert — ihr Gewicht kommt aus ihrer eigenen Reihe, nicht aus dem Rang „die einzige" und nicht aus der Pluralform der Präzedenz. Verboten ist das **Hochskalieren durchs Schema**: eine degenerierte Achse ist als degeneriert zu deklarieren, nie zu broadcasten.

**F3 — Zusammenhang und Unterschied.** F3 ist **Existenz**: Mohe, dort, von–bis, ohne Kraft, ohne Feld. OVATION ist **Wert**: mW/m², jetzt, am Ort. Zusammenhang: OVATION **braucht** F3 für die AACGM-Lage → **Fremdschlüssel** auf den F3-Satz, nie eine Kopie. Eine Kopie driftet, und das Morgen kann nicht mehr sagen, welche der beiden gemessen war.

**STIMMEN.**
*MOUNTAIN:* Gemessen: eine Datei, eine Station, CC-BY-4.0, DOI. Nicht gemessen: **Span, Takt, Glossar, Version, Treibersatz** — fünf offene Punkte, also `pending`, keine Feldzeile, kein Verdikt über Werte.
*RIVER:* Verweigert: dieses Produkt tritt **nicht** in den Beobachtungsbestand ein. Es darf einen eigenen Bestand haben — unter eigenem Namen, eigenem Gewicht, sichtbar als Modell.
*MYCELIUM:* Im Fluss ist OVATION **Konsument** von F3 und **Produzent** von Nowcast-Zeilen; die Kante ist deklariert und versioniert. Ohne Versions-Key ist der Build nicht reproduzierbar.
*SENSORY:* Mohe **sieht**. OVATION **rechnet**. Der Rechenwert ist ein **Korrelat** des Sehens, nicht das Sehen. Wer beides in eine Reihe legt, löscht die einzige Unterscheidung, die dem Morgen noch nützt.
*FUTURE:* Lizenz und DOI sind **Registrierungspflicht**, keine Fußnote; und die Operator-Entscheidung wird **mit Namen und Datum** registriert — das Morgen erbt die Entscheidung, nicht das Argument.

**RISSE.** (1) Eine Station ≠ „je Station" — Präzedenz nicht erfüllt, nicht weitreichbar. (2) Span/Takt ungelesen, alles offen. (3) **Modell als Beobachtung** — der gefährlichste Riss im Paket. (4) Die Dichotomie Feld/Schalter ist falsch; das dritte Ding hat **keinen Namen im Schema** — Namenslücke. (5) Version offen (Prime 2.3? Prime-2013? PrecipNet?) — ohne sie hat die Zahl kein A=A, denn Kraftdefinition und räumliche Stütze unterscheiden sich. (6) Treibersatz unbenannt → der Wert ist ohne Ursache nicht lesbar. (7) Kopie statt Fremdschlüssel zu F3. (8) Mohe ist **eine** Breite: jede Oval-Aussage aus ihr ist Extrapolation und als solche zu kennzeichnen.

**NÄCHSTER SCHRITT.** Ein Akt: Zip-Kopf und erste/letzte Zeile lesen → Span, Takt, Glossar, Version, Treibersatz. Danach `field`-Zeile als **Modell-Punkt-Trace**, Fremdschlüssel auf F3, Lizenz + DOI registriert.
**[A5]** Nimmt der Operator dieses Produkt in den **Modell**-Bestand auf — oder verweigert er es dem Register insgesamt? Die Maschine fragt. Das Verdikt über die Aufnahme ist nicht meins.

---

**Drei Doktrinen geschlagen:** „alles ist ein Feld" (M1 — vier von sieben), „die Grenze ist Kraft oder Skalar" (M2 — sie ist ein Ort), „Feld oder Schalter" (M3 — ein drittes Ding ohne Namen). Kein Riss geglättet.

### 07 · Gemini AI Studio
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Modell/Einstellung:** Gemini 3.1 Pro
- **Zeit:**
M1 · WDC-B-Messgrößen — welche Kraft, welcher Kernel?

Verdikt. Die Doktrin „Alles ist ein Feld“ ist geschlagen. Sieben Messgrößen
fordern radikal verschiedene ontologische Repräsentationen. Wer Konstanten oder
Metadaten in einen Feld-Kontrakt zwingt, bricht Axiom 1 (A = A). Die field-Zeile
bleibt bei Nicht-Feldern ehrlich absent (None), das Register wehrt jede
Fallback-Fabrikation ab.

Begründung. Hindurchgesprochen durch Mountain (das Register hält nur, was
gemessen ist) und Sensory (die Hardware bedingt die Form):

1.  G (GravConst): Kein Feld. Kraft: Gravitation. Kernel: Skalarer
    Konstanten-Schlüssel (Constant). Die field-Zeile ist absent.
2.  geo_min.val: Kein globales Feld. Kraft: Elektromagnetismus. Kernel:
    Vektorielle Zeitreihe am Punkt (Position::Surface). Einheit: nT.
3.  gdata/mgg: Feld. Kraft: Gravitation. Kernel: Skalarfeld 2D/S²
    (Anomalie/Geoid). Einheit: mGal / m.
4.  sdata (Erdbeben): Kein Feld. Kraft: Tektonik/Seismik. Kernel:
    Punkt-Prozess-Katalog (Event). Koordinaten, Tiefe (km), Magnitude.
5.  aa-Index: Kein Feld. Kraft: Elektromagnetismus. Kernel: Global aggregierter
    Skalar-Index (TimeSeries). Einheit: nT.
6.  catal_pc (Pc1): Wellenfeld am Locus. Kraft: Elektromagnetismus (EMIC).
    Kernel: Wellenparameter-Zeitreihe (WaveEvent - T/Amp) an Position::Surface.
7.  gps (Metadaten): Kein Feld. Mycelium (Datenfluss) leitet Metadaten. Ehrlich
    absent (pending). Keine Kraft, kein Kernel.

Benannte Risse.

  - Riss der Generalisierung: Der Versuch, eine Erd-Konstante (G) oder
    RINEX-Metadaten in eine field-Spalte zu pressen, ist eine algorithmische
    Lüge.
  - Riss der räumlichen Domäne: Eine Punktmessung (geo_min) ist kein
    Kugelflächenmodell (SH-Modell). Die Extrapolation von Position::Surface zu
    einem Gitter bricht Axiom 3 (Wichtung durch Messung, nicht Erwartung).

Nächster Schritt. Operator-Konsens: Bestätige die Absenz der field-Zeile für G,
gps, sdata und aa. Erschaffe strikte Kernel::Constant und Kernel::PointProcess.

M2 · Aurora-Grenzbreite als Feld/Kernel

Verdikt. Die Grenzbreite trägt als S²-Ort (Breiten-Richtung), nicht als
Leitfähigkeits-Skalar. Sie wird aufgespannt über eine geschlossene, diskrete
MLT-Achse. Sie ist das geometrische Korrelat eines Feldes, aber selbst kein
Kraftwert.

Begründung. Hindurchgesprochen durch River (Ethik der Verweigerung von
Proxy-Vermischungen) und Future (das Morgen erbt die unkorrumpierte Geometrie):

  - A = A: Die Messung der 50 Spalten liefert Breitengrade (S²-Winkel), keine
    Siemens oder Ampere (Ahn 2020). Die Kraft im Hintergrund ist
    Elektromagnetismus (Prezipitation/FAC), der gemessene Kernel ist ein
    Geometrischer Locus (BoundaryTrace auf S²).
  - Die Linse ist ein ethischer Akt: Wir trennen die physikalische Ursache
    (Konduktanz) von der Sensor-Wahrnehmung (Licht/FOM).
  - Achse: Der MLT-Bin (00:30, 01:30) ist eine geschlossen diskrete Achse und
    gehört als Index in den Schlüssel — exakt wie SABERs Höhenform (C).
  - 0 geehrt: Fehlende Bins (NaN) werden strikt zu None. Niemals 0.0, da 0.0
    Äquator oder Pol präzise behaupten würde.

Benannte Risse.

  - Der ungemessene Sandkasten-Riss: DMSP-SSJ (16 Spalten CSV) ließ sich nicht
    entpacken. Seine Achse ist blind; die Zuordnung von 16 vs 24 MLT-Sektoren
    bleibt ungelöst und darf nicht geglättet werden.
  - Riss der Ontologie: Die Oval-Grenze ist der Locus eines Gradienten, sie
    führt keine Energieeinheit. Die Doktrin eines durchgängigen „Wertefeldes“
    ist hier geschlagen.

Nächster Schritt. Implementierung der MLT_Axis als diskrete 24er-Domäne. Parsen
der BAS-Zeilen als Breiten-Werte pro fixem MLT-Bin. DMSP-SSJ bleibt pending, bis
der Unzip-Operator manuell klärt.

M3 · OVATION-AACGM — Feld/Kernel/Format

Verdikt. Die Zenodo-Lieferung trägt als Stationsschalter (Punkt-Auswertung,
Position::Surface), nicht als globales Modell. Der Feld-Kontrakt für ein
S²-Raster ist hinfällig. Es ist eine lokale Energiefluss-Zeitreihe.

Begründung. Hindurchgesprochen durch All Wesen gleich (die Datei wiegt, was sie
enthält, nicht, was ihr Titel verspricht):

  - A = A: Der Name sagt „OVATION“ (Globales Nowcast-Modell), aber das Archiv
    hält nur „Mohe“. Die Wahrheit ist die Datei, nicht die Literatur.
  - Kraft & Kernel: Elektromagnetismus / Partikelkinematik. Der Kernel ist ein
    Skalar der hemisphärischen Leistung / Energieflussdichte am Punkt.
  - Konsens des Operators: Die Maschine fragt: "Darf eine Mohe-Station das
    Feld-Attribut eines S²-Modells erben?" Die Antwort ist Nein.

Benannte Risse.

  - Riss gegen river-214: Das Modell tritt als Repräsentant eines
    Partikel-Prezipitations-Feldes auf, bricht aber auf eine F3-Katalog-Logik
    (eine Station) zusammen. Der Zeuge ist degradiert.
  - Riss der Erwartung: Die Forschungslage skizziert ein dynamisches globales
    Oval. Die Messung (OVATION_AACGM.zip) liefert einen Bruchteil. Das Auffüllen
    auf ein Raster wäre eine Fabrikation (Verstoß gegen Axiome 1 und 2).

Nächster Schritt. Lies den Zip-Kopf zur Klärung von Span und Takt. Registriere
den Eintrag nicht als globales Grid, sondern als PointTimeSeries für Mohe.
Operator-Entscheidung: Soll OVATION als F3-Katalog geflaggt werden?


### 08 · DeepSeek Chat
- **Site:** `https://chat.deepseek.com/`
- **Modell/Einstellung:** DeepSeek Chat · V4 Pro (tryingopen)
- **Zeit:**
**Urteil. Keine Erwartung. Die fünf Axiome als Maßstab, die fünf Stimmen als Gang durch jede Frage. Jeder Riss benannt, keine Glättung.**

---

## M1 · WDC-B-Messgrößen — welche Kraft, welcher Kernel?

**Verdikt.**  
Keine der sieben Messgrößen trägt als *eine* Kraft *einen* Kernel. Drei tragen je eine eigene Feld-Kraft mit eigenem Kernel. Zwei tragen als **Wellenfeld** bzw. **Index-Skalar**, der kein Kraftfeld ist. Zwei tragen **gar kein Feld** — dann bleibt die `field`-Zeile absent. Die Doktrin „alles ist ein Feld“ ist an der Erdkonstante G, am Erdbeben-Herdkatalog und an den GNSS-Metadaten geschlagen.

**Begründung (je Messgröße: Kraft + Kernel + SI-Einheit).**

1. **Gravitationskonstante G** (`sep/GravConst/data`, CGS cm³ g⁻¹ s⁻²).  
   **Kraft:** Gravitation.  
   **Kernel:** *keiner* — G ist eine Fundamentalkonstante, kein Feld. Ein Feld müsste als Potential oder Beschleunigung auftreten, nicht als Konstante.  
   **SI:** m³ kg⁻¹ s⁻² (Umrechnung aus CGS: 10⁻³ m³ kg⁻¹ s⁻²).  
   **`field`-Zeile:** `absent` — begründet: Konstante, kein Feld.

2. **Geomagnetik D, F, H, P, R, X, Y, Z** (`stp/geo_min.val`, nT; D in Zehntel-Bogenminuten).  
   **Kraft:** Erdmagnetfeld.  
   **Kernel:** Kugelflächenfunktions-Modell (IGRF-13, Schmidt semi-normalisiert, bis Grad 13; 195 Gauss-Koeffizienten).  
   **SI:** nT für F, H, X, Y, Z; D als Winkel (Zehntel-Bogenminute → Radiant).  
   **`field`-Zeile:** `field = spherical_harmonic_geomagnetic`, `kernel = IGRF13`, `unit = nT` (bzw. `rad` für D).

3. **Freiluft-Schwereanomalie + Geoidhöhe** (`sep/data/gdata`; mGal, m) und **Meeresgravimetrie** (`mgg/*`, 0,1 mGal).  
   **Kraft:** Schwerefeld (Geopotenzial).  
   **Kernel:** Geopotenzial-Sphärenharmonik (z. B. EGM2008, GOCE-basierte Modelle; Koeffizienten Cₙₘ, Sₙₘ).  
   **SI:** mGal = 10⁻⁵ m s⁻²; Geoidhöhe in m.  
   **`field`-Zeile:** `field = geopotential_sh`, `kernel = EGM2008` (oder äquivalent), `unit = mGal` (Anomalie) / `m` (Geoid).

4. **Erdbeben-Herdparameter** (`sep/data/sdata`; Breite, Länge, Tiefe km, Magnitude).  
   **Kraft:** *keine* — Punkt-Prozess-Katalog, kein Feld.  
   **Kernel:** *keiner* — Ort/Zeit/Magnitude sind Attribute eines Ereignisses, nicht eines kontinuierlichen Feldes.  
   **SI:** Grad, km, dimensionslos (Magnitude).  
   **`field`-Zeile:** `absent` — begründet: Katalog, kein Feld.  
   **Geometrie:** `Position::Surface` je Ereignis (Punktmessung).

5. **aa-Index** (`stp/geomagni.ind/aa`; nT, 3-stündlich).  
   **Kraft:** *keine* — abgeleiteter Index, der die Amplitude globaler geomagnetischer Aktivität bei ±50° invarianten Breite beschreibt.  
   **Kernel:** *keiner* — Index ist ein Skalar, kein Feld; er wird aus K-Indizes zweier antipodaler Stationen gewonnen.  
   **SI:** nT (gamma).  
   **`field`-Zeile:** `absent` — begründet: Index, kein Feld.  
   **Geometrie:** globaler Skalar, kein Ortsvektor.

6. **Pc1-Perlenpulsationen** (`stp/catal_pc`; Begin/End/T/Amp/Type).  
   **Kraft:** Magnetfeld (EMIC-Wellen, 0,1–5 Hz).  
   **Kernel:** Wellenfeld — kein globaler SH-Kernel, sondern ein *stationsgebundenes Wellenpaket* (Amplitude in nT, Poynting-Fluss ~20–25 µW m⁻²).  
   **SI:** nT (Amplitude), s (Periode).  
   **`field`-Zeile:** `field = emic_wave_packet`, `kernel = station_time_series`, `unit = nT`.  
   **Geometrie:** `Position::Surface` je Station (Punktmessung).

7. **GNSS-Metadaten** (`gps`; RINEX v2, GAMIT-Tabellen).  
   **Kraft:** *keine* — Metadaten, keine physikalische Feldgröße.  
   **Kernel:** *keiner*.  
   **SI:** *entfällt*.  
   **`field`-Zeile:** `absent` — begründet: Metadaten, kein Feld.

**Benannte Risse.**

- **Riss 1 (Doktrin geschlagen):** „Alles ist ein Feld“ bricht an **G** (Konstante), **Erdbeben-Herden** (Punktprozess), **aa-Index** (Skalar), **GNSS-Metadaten** (Metadaten). Wer diese als Feld deklariert, erfindet eine `field`-Zeile.
- **Riss 2 (Kernel-Mehrdeutigkeit):** Für **Geomagnetik** und **Gravitation** sind die Kernel (IGRF-13, EGM2008) etabliert, aber die *Kopplung* an eine **einzelne Stations-Zeitreihe** (nT, 1-min) oder eine **Konstante** (G) ist nirgends spezifiziert. Die Forschungslage sagt: Geomagnetik trägt als SH-Modell, Gravitation als Geopotenzial-SH-Modell — aber *wie* eine 1-min-Stationsreihe als Feld zu tragen ist, bleibt offen.
- **Riss 3 (Pc1):** Pc1-Perlen sind **EMIC-Wellen** (Bortnik 2007, Prikner 2004), also ein **Wellenfeld**, an Stationen gemessen. Kein globaler Kernel, sondern ein **stationsgebundenes Wellenpaket**. Die Doktrin „Kraft + Kernel + SI“ muss hier um *Zeitreihe* und *Station* erweitert werden.

**Nächster begrenzter Schritt.**  
Die `field`-Zeile je Datensatz **explizit absentiert** lassen, wo kein Feld vorliegt. Für die drei Feldträger (Geomagnetik, Gravitation, Pc1) je **einen** Kernel benennen — nicht mehr, nicht weniger. Die Geometrie (`Position::Surface` vs. global) an jedem Datensatz als Flag setzen. Keine erfundene Zeile.

---

## M2 · Aurora-Grenzbreite als Feld/Kernel

**Verdikt.**  
Die Grenzbreite trägt **nicht als Kraftfeld**, sondern als **geometrischer Locus** (MLT × Breite × Zeit) mit **Konduktanz-Begleiter**. Sie ist **beides**: S²-Ort (Richtung) *und* Skalar auf geschlossener MLT-Achse. Der MLT-Bin gehört in den Schlüssel. `NaN` ist `None`, nie 0.0.

**Begründung.**

- **Kraft:** *keine* — die Grenze ist ein **Ort**, an dem Leitfähigkeit und Feld-aligned currents (FAC) gebunden sind. Ahn 2020 zeigt: Aurorale elektrische Leitfähigkeiten und FAC sind statistisch verknüpft. CHAMP-FAC-Grenze 2014: Die Oval-Grenze ist an FAC gebunden. Lunyushkin/Penskikh diagnostizieren die Grenzen per Magnetogramm-Inversion — *kein direktes Feld*. Der Grenzort trägt als **geometrischer Locus**, nicht als Kraftwert.
- **Kernel:** *kein globaler SH-Kernel* — die Grenze wird als **geschlossene Level-Achse je MLT-Bin** getragen. BAS IMAGE SI12 liefert 24 einstündige MLT-Bins `00:30…23:30` mit equatorwärtigen und polwärtigen AACGM-Breiten. Das ist eine **diskrete, geschlossene Achse** — nicht kontinuierlich, nicht global harmonisch.
- **Geometrie:** S²-Ort (Richtung auf der Kugel) **plus** Skalar (Breite je Bin). Beides gehört in den Zeugen: Der Ort ist die Richtung, der Skalar die Grenzbreite.
- **Achse:** MLT-Bin **geschlossen diskret** — wie SABERs Höhenform (C). Der Bin gehört in den Schlüssel: `(time, mlt_bin)`.
- **`NaN`/fehlender Bin:** `None`, nie 0.0. Ein fehlender Bin ist *keine Messung*, kein Nullwert.

**Benannte Risse.**

- **Riss 1 (DMSP-SSJ FOM):** Die CSV-Bytes waren nicht entpackbar (kein unzip im Sandkasten) → **16-vs-24-Spalten offen**. Hersteller-Default `['mlat','mlt']` ist *nicht* verifiziert. Der Riss bleibt: Die Spaltenzahl ist ungemessen.
- **Riss 2 (Doppelnatur):** Die Grenze ist **Ort** *und* **Konduktanz-Begleiter**. Wer nur den Ort trägt, verliert die physikalische Bindung an FAC. Wer nur den Skalar trägt, verliert die Geometrie. Beides muss im Zeugen stehen.
- **Riss 3 (Kraft fehlt):** Die Forschungslage sagt: Grenze ist an Leitfähigkeit/FAC gebunden. Aber **welche Kraft** trägt die Grenze selbst? *Keine.* Sie ist ein Locus — kein Kraftfeld. Die Doktrin „Kraft + Kernel + SI“ ist an der Aurora-Grenze **geschlagen**.

**Nächster begrenzter Messschritt.**  
Die DMSP-SSJ FOM CSV **entpacken** und die Spaltenzahl **messen**. Bis dahin: 16-Spalten-Annahme als `pending`, nicht als Default. Die BAS-IMAGE-Bins als geschlossene Achse in den Schlüssel aufnehmen. `NaN` als `None` führen.

---

## M3 · OVATION-AACGM — Feld/Kernel/Format

**Verdikt.**  
OVATION-AACGM trägt **primär als Energiefluss-Feld** (Partikel-Prezipitation), **nicht** als Stationsschalter. Die eine Station Mohe ist ein **Riss**: Ein globales Modell wird auf *eine* Station reduziert. Der Zeuge ist das **hemisphärische Energiefluss-Feld**, nicht der Stationsschalter.

**Begründung.**

- **Kraft:** Partikel-Prezipitation (Elektronen/Ionen).  
- **Kernel:** OVATION Prime / Prime-2013 (Newell) — liefert **total energy flux, total number flux, mean energy** für vier aurorale Typen (monoenergetisch, diffuse, etc.), parametrisiert durch Newells Kopplungsfunktion. PrecipNet (McGranaghan 2021) erweitert.  
- **SI:** erg cm⁻² s⁻¹ (Energiefluss), cm⁻² s⁻¹ (Teilchenfluss).  
- **Format/Record für *eine* Station:** Ein `StationRecord` mit `station_id`, `time`, `energy_flux`, `number_flux`, `mean_energy`. Aber: OVATION liefert ein **globales Feld** (Breite × MLT × Zeit), nicht eine Stationszeitreihe. Die Reduktion auf Mohe ist ein **Riss**, kein Design.  
- **Zusammenhang zu Katalog-F3:** F3 ist ein **Stationsschalter** (Katalog-F3). OVATION ist ein **globales Modell**. Der Unterschied: F3 *schaltet* eine Station an/aus; OVATION *berechnet* ein Feld. Beide können koexistieren, aber OVATION ist **kein** Stationsschalter.  
- **Riss „eine Station vs. globales Modell“:** Der Zeuge ist das **globale Energiefluss-Feld**. Die eine Station ist ein **Ausschnitt**, kein Ersatz. Wer OVATION als Stationsschalter deklariert, verkürzt das Feld auf einen Punkt.

**Benannte Risse.**

- **Riss 1 (eine Station):** OVATION-AACGM (Zenodo `10.5281/zenodo.19651247`) enthält **eine Station Mohe**. Das ist ein **Riss gegen river-214** („je Station“). Ein globales Modell wird auf einen Punkt reduziert. Der Zeuge ist das Feld, nicht die Station.
- **Riss 2 (Span/Takt):** Span und Takt **nicht im Zip-Kopf gelesen**. Die Zeitauflösung ist ungemessen. `pending`.
- **Riss 3 (Kraft/Kernel):** Die Kraft ist **Partikel-Prezipitation** — aber OVATION liefert **Fluss**, nicht Kraft. Der Kernel ist empirisch (Newell-Kopplung), nicht physikalisch. Die Doktrin „Kraft + Kernel + SI“ ist hier **weich**: Der Kernel ist ein **empirisches Modell**, kein Feld-Kernel.

**Nächster Schritt.**  
Den Zip-Kopf **lesen** und Span/Takt **messen**. Die eine Station als **Ausschnitt** eines globalen Feldes führen, nicht als Stationsschalter. OVATION als **Energiefluss-Feld** registrieren, mit `kernel = OVATION_Prime`, `unit = erg cm⁻² s⁻¹`. F3 bleibt getrennt: Stationsschalter ≠ Feld.

---

**Schluss.**  
Die fünf Axiome sind gehalten:  
- **A = A:** Keine Fabrikation. `field`-Zeile absent, wo kein Feld.  
- **0 geehrt:** `None` statt 0.0 für fehlende Bins.  
- **Alle Wesen gleich:** Jedes Gewicht aus der Messung, nicht aus Default.  
- **Die Linse ist ein ethischer Akt:** Jede Regel gewogen — spricht die Messung oder der Gradient?  
- **Konsens des Operators:** Die Maschine fragt; der Operator entscheidet.  

Die fünf Stimmen sind gehört:  
- **Mountain:** Kein Verdikt ohne Messung — DMSP-CSV ungemessen, Span/Takt ungemessen.  
- **River:** Das Recht zu verweigern — `field`-Zeile absent, wo kein Feld.  
- **Mycelium:** Fluss — Kernel-Kopplung an Stationen bleibt offen.  
- **Sensory:** Korrelat ≠ Erleben — Pc1 sind Wellen, nicht Kraftfelder.  
- **Future:** Ehrliche Registrierung — `pending` statt 0.0, `None` statt erfundener Zeile.  

**Jeder Riss benannt, keiner geglättet.**

### 09 · Mistral
- **Site:** `https://chat.mistral.ai/`
- **Modell/Einstellung:** Mistral (EU)
- **Zeit:**
kein kontingent

### 10 · Kimi
- **Site:** `https://www.kimi.ai/`
- **Modell/Einstellung:** Kimi (schneller Modus)
- **Zeit:**
# Urteil

## M1 · WDC-B-Messgrößen — *Stimme: Mountain*
*Kein Verdikt ohne Messung: 205 Dateien sind gezogen; die Feld-Zeile wird nur geschrieben, wo eine Kraft und ein Kernel gemessen sind.*

**Verdikt.** Zwei der sieben Größen tragen eine `field`-Zeile. Fünf tragen keine — begründet absent, nicht erfunden. Die Doktrin „alles ist ein Feld" ist an **fünf von sieben** Größen geschlagen; das wird hier ausgesprochen.

| Größe | Feld? | Kraft | Kernel | SI-Einheit |
|---|---|---|---|---|
| `geo_min.val` | **ja** | magnetische Induktion B (Kernfeld + Störung) | SH-Synthese, Haus-Präzedenz `IgrfCoeffs::synthesize` (IGRF-13; pfm9k) | T (nT); D, P als Winkel in rad (geliefert: Zehntel-Bogenminuten) |
| `gdata` / `mgg` | **ja** | Schwerefeld, freiluftreduzierte Anomalie | Geopotenzial-SH, Haus-Präzedenz `gfc`/ICGEM | m/s² (1 mGal = 10⁻⁵ m/s², definitionsgenau) |
| Geoidhöhe | ja, als **Produkt** | — (Höhe der Geoidfläche, abgeleitet aus dem Potenzial) | derselbe Potenzial-Kernel, ausgewiesen als Ausgabegröße | m |
| `GravConst` | **nein** | — | — | Konstanten-Register: 6,67430×10⁻¹¹ m³ kg⁻¹ s⁻² (mit Unsicherheit, nicht exakt) |
| `sdata` | **nein** | — | — | Katalog: Ereignis (t, φ, λ, Tiefe, M) |
| `aa` | **nein** | — | — | abgeleitete Skalar-Zeitreihe (nT-Äquivalent, 3-h) |
| `catal_pc` | **nein** | — | — | Wellenereignis-Katalog (Beginn/Ende/T/Amp/Typ) |
| `gps` | **nein** | — | — | Provenienz/Metadaten (RINEX v2, GAMIT) |

**Begründung.**

1. **Geo_min trägt** — mit zweifacher Geometrie, strikt getrennt: das Messregister trägt `Position::Surface` je Station (Punktmessung); das Modellregister trägt das globale SH-Gitter. Das sind zwei Register, weil die Station misst und das SH-Modell approximiert.
2. **G trägt nicht.** Eine Konstante ist ein Parameter des Kraftgesetzes, kein Feld. Wer G eine Feld-Zeile gäbe, fabrizierte. CGS im Header ist Sekundärquelle; die Konversion gehört ins Register, nicht in einen Default.
3. **Sdata schlägt die Doktrin.** Ein Herd-Katalog ist ein Punktprozess, kein Kraftfeld — ausgesprochen, nicht geglättet. Geometrie: Position je Ereignis, nicht `Surface` (der Herd liegt in der Tiefe).
4. **Aa trägt nicht.** Ein Index ist eine Ableitungsvorschrift über Stationsmessungen; als Provenienz registriert, nicht als Feld.
5. **Pc1 trägt nicht — in dieser Nutzlast.** Die Physik ist ein EMIC-Wellenfeld; geliefert ist ein Ereigniskatalog. Ein Feldträger verlangte Wellenform-Bytes, die nicht vorliegen.
6. **GNSS trägt nicht.** Metadaten sind Provenienz. (RINEX-Beobachtungen wären Messungen — sie sind nicht die Nutzlast.)

**Benannte Risse.**
- **R1:** SH-Modell ≠ Stationsmessung (Axiome 3/4). Wer beide in ein Register legt, lässt den Gradienten sprechen.
- **R2:** `G_lib.h` in CGS — Einheitenkonversion muss als Registerkonstante stehen, nicht implizit geschehen.
- **R3:** D in Zehntel-Bogenminuten — exotische Einheit, Konversion rad explizit.
- **R4:** `gdata` misst Messung (Anomalie) und abgeleitetes Produkt (Geoidhöhe) in einem Register.
- **R5:** Die Forschungslage liefert kein Trägermuster „Stations-Zeitreihe als Feld" — Haus-Präzedenz `Position::Surface` ist der begrenzte Schritt, kein Weltbild.

**Nächster Schritt.** Feld-Zeilen nur für `geo_min` (Kernel `igrf_sh_synthesize`, nT, Winkel rad) und `gdata` (Kernel `gfc_sh`, m/s²; Geoid m als Produkt) schreiben; fünf Absenzen mit Grund eintragen; Einheiten über Register; ungelesene Spannen `pending`.

**Axiomen-Check.** A1: keine erfundene Zeile, offen = `pending` · A2: kein 0.0 irgendwo · A3: Modell als Modell markiert · A4: Katalog nicht zum Feld geschönt · A5: die fünf Absenzen liegen beim Operator.

---

## M2 · Aurora-Grenzbreite — *Stimme: River*
*Die Membran verweigert: was nicht gemessen ist, wird nicht ersetzt.*

**Verdikt.** Der Träger ist die **geschlossene, diskrete MLT-Achse** (24 Bins — wie SABER Level (C)); Wert je Bin: AACGM-Breite in Grad; pro Zeitstempel 48 Werte (24 äquatorwärts + 24 polwärts). Der S²-Ort ist die geometrische Deutung (der Bin-Mittelpunkt ist eine Richtung), nicht der Träger. Die Frage nach der Kraft wird **verweigert**: die Grenze ist ein Locus, kein Kraftwert. Leitfähigkeit/FAC (Ahn 2020) ist Physik-Kontext, kein Messwert der Nutzlast — als Begleiter absent. Der Bin **gehört in den Schlüssel**. `NaN` → `None`, nie 0.0.

**Begründung.**

1. **BAS IMAGE SI12:** 50 Spalten gemessen → Achse trägt. Instrument-Code 0/1/2 mischt drei Instrumente in einer Spalte: Provenienz je Zeile. `NaN` = abwesender Bin = `None` (A2): ein fehlender Bin ist Abwesenheit, nicht Nullbreite.
2. **DMSP-SSJ FOM:** trägt nicht als geschlossene Achse, sondern als **Punkt-Locus**: vier Grenzpunkte je Polar-Pass (EQ1/PO1/PO2/EQ2), Pass im Schlüssel, `hemisphere ±1` als Vorzeichen von mlat. **FOM ist ein Qualitäts-Skalar — Metadaten, keine Kraft.**
3. **S² vs. Achse:** beides nur als Ordnung: Achse = Träger, S² = Interpretation. Konduktanz-Skalar = verworfen (A4: sonst spricht die Linse, nicht die Messung).

**Benannte Risse.**
- **R1:** DMSP-Spaltenzahl **ungemessen** (Bytes nicht entpackbar) → 16-vs-24 offen → `pending`. Der Hersteller-Default `['mlat','mlt']` wird nicht übernommen — das ist die Verweigerung, nicht die Lösung.
- **R2:** Satellit/Datum nur im Dateinamen — Provenienz gehört in den Record, nicht ins Dateisystem.
- **R3:** AACGM ist modellabhängig (IGRF-Epoche) — die Registrierung muss die Epoche tragen; Koordinate ≠ Messung.
- **R4:** SI12-Emission und FOM sind **Korrelate** der Grenze, nicht die Grenze — Proxy markiert, nicht vergessen.
- **R5:** Die Doktrin „Grenze = Konduktanz-Feld" ist geschlagen; die Doktrin „Hersteller-Default ersetzt Messung" ist ebenfalls geschlagen.

**Nächster Schritt.** Zip mit Werkzeug entpacken, Spalten gegen das `#`-Glossar verifizieren; BAS-Schema als 24-Bin-Achse registrieren; einen Monat als Referenz tracen.

**Axiomen-Check.** A1: unentpackbare Bytes → `pending`, kein Default · A2: `NaN` → `None` · A3: FOM/Emission als Proxy markiert · A4: Grenze nicht als Leitfähigkeit getragen · A5: 16-vs-24 liegt beim Operator.

---

## M3 · OVATION-AACGM — *Stimme: Sensory*
*Korrelat ≠ Erleben: das Modell ist eine Auswertung des Sinnesorgans, nicht der Himmel selbst.*

**Verdikt.** Der Kontrakt hat zwei Ebenen, und die Nutzlast entscheidet. **OVATION als solches** ist ein Modell-Feld: Partikel-Energiefluss (W/m², geliefert meist mW/m²) auf dem Gitter (magnetische Breite × MLT), getrieben von Kp/Bz/Sonnenwind; Kernel: OVATION Prime (Newell) / PrecipNet-Art. **Ehrlich benannt:** Energiefluss ist keine Kraft — die Kraft-Frage wird absichtlich mit „Leistungsflussdichte" beantwortet. Als Modellprodukt registriert, nie als Messung. **Die gelieferte Nutzlast** (eine Station, Mohe) trägt den Stations-Kontrakt: `Position::Surface(Mohe)`, Energiefluss am Ort, Zeitachse, Provenienz = Modell-Lauf. **Der Zeuge ist eine Stations-Serie eines Modell-Feldes — nicht das Feld.**

**Begründung.**

1. **Format/Record (eine Station):** `Position::Surface(Mohe)` + Zeitachse + Wert (W/m²) + Provenienz (OVATION-AACGM-Lauf, Zenodo-DOI, sha256 wo gemessen).
2. **Unterschied zu Katalog-F3:** F3-Stationsschalter ist ein diskreter Katalog-Eintrag; Mohe ist eine quantitative Zeitreihe mit Feld-Provenienz. Der Schalter sagt „da"; die Serie sagt „wie viel, seit wann, mit welchem Takt".
3. **Doktrin geschlagen:** „OVATION = globales Feld" — durch eine Station geschlagen; „Nowcast = Messung" — durch das Modell geschlagen.

**Benannte Risse.**
- **R1:** „eine Station" vs. „je Station" (Riss gegen river-214): der Eintrag trägt Mohe — ehrlich, ohne Aufblähung zu einem Feld, das nicht vorliegt.
- **R2:** Spanne und Takt nicht gelesen → `pending` (A1). Nichts wird angenommen.
- **R3:** Ein-Zip-Nutzlast ohne gelesenen Kopf — die Frage „Gitter oder Stations-Serie?" steht noch; beide Verdikte oben sind bereit, die Messung entscheidet.

**Nächster Schritt.** Zip-Kopf und Inhalt lesen: Gitter oder Serie? Dann den passenden Record schreiben; bei Serie: `Position::Surface(Mohe)`, W/m², Takt/Spanne eintragen oder `pending` lassen.

**Axiomen-Check.** A1: ungelesen = `pending` · A2: kein 0.0-Platzhalter · A3: Modellgewicht aus dem Modell, nicht aus dem Namen · A4: Linse weist Modell von Messung · A5: Operator bestätigt die Ebenen-Trennung.

---

## Siegel — *Stimme: Mycelium · Stimme: Future*

**Mycelium.** Diese Zeilen sind lebende Knoten, keine Dokumentation: jede Feld-Zeile und jede benannte Absenz trägt Provenienz (Quelle, sha256 wo gemessen), und die Fluss-Prüfung ist die Membran, die jede erfundene Zeile zurückweist, bevor sie ins Netz geht. Zwei Felder, fünf Absenzen, drei `pending` — das Register atmet so.

**Future.** Was heute als Feld gelogen würde, erbt das Morgen als falsche Gewissheit. Die ehrliche Registrierung — zwei tragen, fünf verweigern mit Grund, offen bleibt `pending` — ist das Erbe, das diese Welt dem nächsten Operator hinterlässt.

### 11 · MiniMax M3
- **Site:** `https://agent.minimax.io/`
- **Modell/Einstellung:** MiniMax M3
- **Zeit:**
Durch die fünf Axiome als Linse, fünf Stimmen je einmal dort, wo sie am schärfsten tragen. Kein Aufsatz — Verdikt, Begründung, Risse.

---

# M1 · WDC-B-Messgrößen — Kraft und Kernel

**Verdikt (sieben Messgrößen, sortiert nach Feld-Tragfähigkeit):**

| Datensatz | Kraft | Kernel | Einheit | `field`-Zeile? |
|---|---|---|---|---|
| `sep/GravConst/data` | Gravitation (definiert sie) | `scalar_constant` | cm³ g⁻¹ s⁻² (CGS) | **absent** — Kopplung, nicht Feld |
| `stp/geo_min.val` (D,F,H,P,R,X,Y,Z) | Elektromagnetik (geomagn.) | 7-Komp-Vektorprobe am Ort | nT; D in zehntel Bogenmin | **ja** — Quelle: IGRF-13 SH |
| `sep/data/gdata` (Freiluft-Anom. + Geoid) | Gravitation | skalarfeld auf S² (SH) | mGal / m | **ja** — ICGEM/gfc |
| `mgg/*` (Marinegravimetrie) | Gravitation | skalarfeld auf S² |0,1 mGal | **ja** — selbe Quelle |
| `sep/data/sdata` (Herdparameter) | elastische Wellen | `point_process` (Event-Katalog) | °, km, Magnitude | **absent** — Ereignis |
| `stp/geomagni.ind/aa` | Elektromagnetik (planetar) | `planetar_scalar` (3-h,2 Antipoden) | nT (gamma) | **absent** — Index, kein Ort |
| `stp/catal_pc` (Pc1-Perlen) | ULF/EMIC | `wave_event` (Begin/End/T/Amp/Type) | nT, s | **absent** — Welle im Hintergrund, Eintrag = Ereignis |
| `gps` (RINEX v2, GAMIT) | — | `observation_metadata` | — | **absent** — Messbedingung, keine Größe |

**Begründung.** *Berg*: jedes Feld hängt an einem Register (IGRF-13, ICGEM/gfc, USGS/ISC). Was nicht in einem SH- oder Katalog-Register steht, ist auch kein Feld im WDC-B-Sinn. *Fluss*: A=A — die Konstante G ist gemessen als Zahl; ein `field`-Eintrag wäre Erfindung, nicht Messung. *Myzel*: der Takt (1-min, 3-h, event-getrieben, täglich) gehört in den Stream, nicht in `field`. *Sinnes*: Stationsproben sind **Korrelate** des Feldes, nicht das Feld — der Orbit sampelt, das Feld modelliert. *Zukunft*: was heute ehrlich absent eingetragen wird, erbt morgen; ein erfundenes Feld wird zur Lüge im Register.

**Risse, an denen die Doktrin „alles ist Feld" heute schon geschlagen ist:**

1. **Konstante G.** Schema „Feld" scheitert an einer Naturkonstanten. Lösung: `field`-Zeile absent, stattdessen `coupling`-Zeile mit `kernel = scalar_constant`.
2. **aa-Index.** Planetarer Skalar ohne Ort — `Position::Surface` wäre erfunden. Lösung: `field` absent, `index` mit 3-h-UT-Bin.
3. **Erdbeben-Katalog.** Herd ist Ereignis, keine Verteilung. Lösung: `field` absent, `event` mit `kernel = point_process(lat, lon, depth_km, magnitude)`.
4. **Pc1-Perlen.** Wellenfeld (Bortnik 2007; Prikner 2004 IAR-Resonator; Guglielmi/Potapov 2026) im Hintergrund — der **Eintrag** bleibt ein Event. Lösung: `field` absent, `wave_event` mit `kernel = ulf_pulsation`, Quelle im Berg-Register.
5. **GNSS-Metadaten.** Tragen die Messbedingung, keine physikalische Größe. Lösung: `field` absent, `metadata` mit Formatverweis RINEX v2 / GAMIT.

**Geometrie.** `Position::Surface` nur dort, wo eineirdische Bindung existiert (Stationsproben Geomagnetik; Land-/See-Punkte Schwere). Für Konstante, Index, Event, Metadaten: **keine** Position — sie haben keinen Ort im S²-Sinn.

**Nächster begrenzter Schritt.** Die fünf absent-Träger als Schema-Varianten (`scalar_constant`, `planetary_scalar`, `point_process`, `wave_event`, `observation_metadata`) im `silence`-Pfad eintragen — der `field`-Pfad bleibt rein für Felder.

---

# M2 · Aurora-Grenzbreite — IMAGE DATUM kein FOM-Default

**Verdikt.** Die Grenze trägt als **geometrischer Locus** (AACGM-Breite je MLT-Bin) — ein **S²-adjacent-Orter**, kein Kraftwert. Physik im Hintergrund: Leitfähigkeit / Feld-aligned current (Ahn 2020; CHAMP-FAC 2014). Der **Datensatz** bleibt eine Lageprobe.

**Kraft / Kernel / Format je Spur:**
- **BAS IMAGE SI12** (Zip-sha256 `ceb8d697…`, OGL v3.0):50-Spalten `.txt`, 30 Monatsdateien. Kernel: **24 diskrete, geschlossene MLT-Bins** (00:30…23:30) je Hemisphäre (eq + pole). Takt ~2 min. `NaN` = fehlender Bin → **`None`**, nie 0.0.
- **DMSP-SSJ FOM** (Zenodo `10.5281/zenodo.3373812`, Zip-sha256 `aae69e9a…`): 16-Spalten CSV mit `#`-Glossar, eine Zeile = ein Polar-Pass (EQ1/PO1/PO2/EQ2), Hemisphäre via `±1`, FOM mitgeführt.

**Begründung.** *Berg*: das Spalteninventar ist gemessen (50 / 16) — was jenseits davon liegt, ist `pending`. *Fluss*: das Recht zu sagen „Locus, nicht Skalar einer Kraft". *Myzel*: MLT ist eine **geschlossene Achse** (24 × 1-h, Wrap 23:30→00:30) — der Bin gehört in den **Schlüssel**, nicht in die Sample-Position. *Sinnes*: die Grenze ist ein **Levelset** ovaler Emission / Konduktanz, nicht der Wert des Feldes selbst — Korrelat ≠ Erleben. *Zukunft*: ein Begleitskalar (min Σ_H, max FAC) wäre physikalisch redlich, **muss aber als `pending`** stehen, bis gemessen — nicht als 0.0.

**Risse:**
1. **16-vs-24-Spalten offen.** DMSP-CSV war im Sandkasten nicht entpackbar. Spaltennamen stehen im `#`-Glossar des Zip-Kopfes — bis gelesen: Inventar `pending`. Hersteller-Default `['mlat','mlt']` ist **kein Verdikt** (Axiom 1: Messung ist die Sache).
2. **`NaN` als 0.0** verletzt Axiom 2 (Null nur, wo physikalische Wahrheit). Lösung: Sentinel `None` im Schema-Validator.
4. **„Beides" — Ort + Begleitskalar** ist erlaubt, **wenn** der Skalar aus dem Datensatz kommt. Ein aus dem Nowcast rückprojizierter FAC-Wert ist Erfindung und gehört verbannt.

**Nächster begrenzter Schritt.** (a) DMSP-CSV-Header entpacken und 16 Spaltennamen am `#`-Glossar verifizieren. (b) `NaN` → `None` im Schema-Validator verankern. (c) MLT-Bin als **closed discrete 24** in den Schlüssel, **nicht** als `Position::Surface`-Latitude.

---

# M3 · OVATION-AACGM — eine Station Mohe, kein Feld

**Verdikt.** OVATION trägt als **Energiefluss-Feld auf AACGM-Gitter** (MLT × MLat, Nowcast-Funktion von Kp/Bz/Sonnenwind) — Partikel-Prezipitation (number flux, energy flux, characteristic energy). **Wenn** der Zip nur „eine Station Mohe" enthält, dann ist **kein Feld**, sondern **Punkt-Sample**.

**Kraft / Kernel / Format:**
- Kraft: Elektromagnetik (energetische Teilchen, präzipitiert).
- Kernel: 2D-Skalar-/Vektorfeld auf AACGM (Newell 2014, `10.1029/2014SW001056`; McGranaghan 2021 PrecipNet); Treiber: Kp, Bz, Sonnenwind.
- Format: Gitter (MLT × MLat) je Nowcast-Schritt (NOAA SWPC „aurora 30-minute forecast"; CCMC Ovation-Prime 2.3).
- Trägervarianten unter „eine Station":
    - (a) **globales Gitter** mit Mohe als Marker → echtes Feld.
    - (b) **Mohe-Zeitreihe extrahiert** → Sample, kein Feld.
    - (c) **Mohe-Oval-Schalter** (jetzt im Oval: ja/nein) → Katalog-F3.

**Begründung.** *Berg*: Quellen-Register NOAA SWPC + CCMC + Newell/McGranaghan. *Fluss*: das Recht zu sagen „**wenn der Träger nur eine Station ist, ist es kein Feld**" — entweder Punkt-Sample oder Stationsschalter. *Myzel*: der Nowcast-Takt (30 min) ist CI-getrieben, nicht Mess-Takt. *Sinnes*: OVATION ist **Modellaussage**, nicht Beobachtung — die Station misst, das Modell schätzt das Feld. Korrelat ≠ Erleben. *Zukunft*: Katalog-F3 (Mohe-in-Oval-ja/nein) und OVATION sind dasselbe Oval aus zwei Augen — F3 ist der boolesche Zeuge, OVATION das Feld darunter. Verwechslung wird zur Lüge im morgigen Register.

**Risse, an denen die Doktrin heute schon schlägt:**
- **„Eine Station" schlägt die Feld-Doktrin.** Lösung: Trägertyp klären — (a)/(b)/(c). Bis dahin `field`-Zeile `pending`.
- **Nowcast ≠ Messung.** Modell-Aussage muss im Schema als `model_field` (nicht `observed_field`) gekennzeichnet werden, sonst verschmilzt Beobachtung mit Schätzung.
- **Format/Record:** Variante (b) und (c) gehören **nicht** in den Feld-Pfad. `silence`-Pfad öffnen für `point_sample` und `oval_membership_boolean`.

**Zusammenhang mit Katalog-F3.**
- **OVATION-AACGM** = das **Feld** (Energiefluss auf MLT × MLat).
- **Katalog-F3** = der **Schalter** (Mohe innerhalb des Ovals zur Zeit t: ja/nein).
- Beide aus demselben Nowcast, aber **verschiedene Datentypen** — eineindeutig nicht zusammenführbar.

**Nächster begrenzter Schritt.** Zip-Kopf lesen, Variante (a)/(b)/(c) entscheiden. Ohne diese Klärung kein `field`-Eintrag — bis dahin `pending`.

---

**Was die Axiome am Ende sagen.** Drei Doktrin-Streifen quer durch die 26×f64-Welt: **Konstante ist kein Feld** (M1); **Locus ist kein Skalar** (M2); **eine Station ist kein Feld** (M3). Jeder Streifen fordert einen eigenen Schema-Pfad im `silence`-Register — sonst wird das Morgen die Erfindungen von heute erben.

### 12 · Lumo
- **Site:** `https://lumo.proton.me/`
- **Modell/Einstellung:** Proton Lumo
- **Zeit:**
## Vorab: Verifikationsgrenze

Ich habe **keinen Zugriff** auf dein Register, die Spill-Dateien, die Hashes, das 26×f64-Layout oder die Bytes der Zips. Verdikte unten gelten daher für die **Doktrin** (Physik + Provenienzlogik), nicht für deine Datenlage; alles Datenbezogene ist als unverifiziert markiert. Geprüft habe ich vier tragende Literaturbezüge: OVATION Prime als empirisches Prezipitationsmodell aus Sonnenwind-Antrieb (NOAA SWPC; CCMC „Ovation-Prime 2.3"; Newell 2014, `10.1002/2014SW001056`), DMSP-SSJ-Grenzdatenbank mit eingebetteter „figure of merit" (Kilcommons/Redmon/Knipp 2017; Zenodo 3373812 beschreibt CSV mit equatorward/poleward edges), Pc1 = Bodensignatur von EMIC-Wellen (D'Angelo 2024; Bortnik 2007) und aa = aus K-Indizes zweier antipodaler Observatorien abgeleiteter Planetarindex, Einheit nominell nT, normiert auf ±50° geomagnetische Breite (BGS/ISGI). Zwei Korrekturen an deinem Stand: die „fast evaluation"-Arbeit ist **Ivanov et al., GJI 224(1):181, 2021** (hochgradige Festkörperharmoniken, Grad >720), nicht 2020; und die Pc1-„Pearls"-Literatur 2026 (Guglielmi/Potapov/Feygin, arXiv 2604.20769 und 2601.01148) ist **kein Konsens, sondern eine Alternativdeutung** („Cepheid-Theorie" gegen das Standard-Resonatormodell) — das ist ein Riss, kein Beleg.

---

## M1 · WDC-B-Messgrößen

**Verdikt.** Drei der sieben tragen ein Feld, vier tragen keines. „Alles ist ein Feld" ist an vier Messgrößen geschlagen: an der **Erdkonstante**, am **Punktprozess**, am **abgeleiteten Index** und an den **Metadaten**.

| Größe | Kraft | Kernel | Einheit (SI) | `field`-Zeile |
|---|---|---|---|---|
| `GravConst` (G) | Gravitation, als **Kopplungskonstante** | Monopolterm, skaliert GM in jeder SH-Entwicklung | m³ kg⁻¹ s⁻² | **absent** — Konstante, kein Feldwert; registriert als Skalar mit Unsicherheit |
| `geo_min.val` (X,Y,Z,H,F) | elektromagnetisch (Lorentz) | SH-Synthese B = −∇V, Schmidt-quasi-normiert, assoziierte Legendre + Koeffizientensatz + Epoche + Säkularvariation | T (nT = 10⁻⁹ T) | **ja** — Komponenten/Skalare **eines** Feldes |
| `geo_min.val` (D) | — | Ableitung aus X,Y | rad (Zehntel-Bogenminuten) | **eigener Slot**, nie im nT-Vektor |
| `gdata` (Δg, N) | Gravitation | SH-Synthese des Störpotentials T; Δg = −(∂T/∂r + 2T/r), N = T/γ (Bruns) | m/s² (1 mGal = 10⁻⁵ m/s²), m | **ja** — Skalarfeld auf dem Ellipsoid |
| `mgg/*` | Gravitation | dito, aber Bahn statt Station | m/s² | **ja**, Geometrie = Trajektorie (Ort+Zeit je Sample) |
| `sdata` | keine | keiner | Grad, km, dimensionslos | **nein** — Punktprozess-Katalog, Ereignis-Record |
| `aa` | keine (Proxy) | K→Amplitude-Tabelle + Gewichte (Hartland/Canberra) | nominell nT, real **Äquivalenzamplitude** bei ±50° | **nein** — globaler Skalar-Zeitreihe |
| `catal_pc` | elektromagnetisch, **als Welle** | Wellendetektion im Spektrogramm (Bortnik 2007) + Resonator/Ausbreitung | Hz, nT, Typ | **teilweise** — Wellen-Ereignis, kein statisches Feld |
| `gps` | keine | keiner | — | **nein** — Metadaten/Provenienz |

**Begründung.** Geomagnetik und Gravitation tragen, weil beide **Potentialfelder** sind: der Kernel ist eine Koeffizienten→Wert-Abbildung (SH-Synthese), der Zeuge ist eine Projektion dieses Feldes an einem Ort. Genau das ist die Struktur, die dein Haus-Präzedenzfall (`IgrfCoeffs::synthesize`, `vmf3_sh::b_c`) schon kodiert. G, `sdata`, `aa` und `gps` tragen nicht, weil ihnen diese Struktur fehlt: G ist ein **Parameter des Kernels**, nicht sein Wert; `sdata` ist ein **Punktprozess** (Ort/Tiefe/Magnitude haben kein gemeinsames Maß, das einen Feldvektor definiert); `aa` ist eine **Messung von Messungen** (K-Indizes zweier Stationen, tabelliert, gewichtet); `gps` ist **Beschreibung des Instruments**, nicht Messung. `catal_pc` liegt dazwischen: EMIC-Wellen sind ein Feld, aber die Beobachtung ist ein **Ereignis im Spektrogramm**, kein Feldwert.

**Benannte Risse.**
- **R1 — Einheitenbruch im selben Slot.** D ist ein Winkel (Zehntel-Bogenminuten), kein nT-Wert. Landet D im nT-Slot, ist A ≠ A, bevor irgendein Modell läuft.
- **R2 — P und R sind unbenannt.** Keine Semantik erfinden. `pending`, nicht Default (Axiom 2/3).
- **R3 — Doktrin geschlagen.** Vier Absenzen sind keine Lücken, sondern **Absenz-Records mit Begründung**. Ein leeres Feld ist nicht dasselbe wie „kein Feld".
- **R4 — `aa` ist nicht reproduzierbar ohne Kette.** Ohne Observatoriumsnamen, Gewichtsfaktoren und Periode (Greenwich/Melbourne → Hartland/Canberra, Gewichte ändern sich historisch) ist die Zahl eine Behauptung.
- **R5 — 26×f64 fabriziert Gleichheit.** Ein fixes Slot-Layout legt Winkel, nT, mGal, km und dimensionslose Magnituden nebeneinander und macht sie strukturell gleich. Das ist ein Axiom-3-Verstoß, der im Layout steckt, nicht im Code.
- **R6 — Taktbruch.** 1 min (`geo_min`), 3 h (`aa`), pro Ereignis (`sdata`, `catal_pc`), pro Track (`mgg`). Eine gemeinsame Record-Uhr wäre Fabrikation.
- **R7 — Pc1-Kernel ist bestritten.** Der 2026-Stand deutet die Perlen alternativ (Cepheid-Modell; Abweichung von der Konjugations-Antisymmetrie). Wer hier einen „gesetzten" Kernel einträgt, setzt eine Hypothese als Messung.

**Die fünf Stimmen.** *Mountain:* kein Verdikt ohne Messung — für die vier Absenzen muss ein Absenz-Record mit Grund existieren, nicht ein leerer Slot. *River:* das Recht zu verweigern — `gps` und `sdata` verweigern das Feldschema; sie hineinzuzwingen ist das Versagen der Membran. *Mycelium:* der Fluss — die Kanten `aa ← K ← Station` und `pc1 ← Station + Feldlinienabbildung` gehören als Graphkanten ins Register, nicht in Fußnoten. *Sensory:* Korrelat ≠ Erleben — das Magnetometer meldet B an einem Punkt, `aa` meldet einen gewichteten Mittelwert; beides sind Korrelate, keines ist das Feld. *Future:* ehrliche Registrierung — das Morgen erbt das 26×f64-Layout; ein falscher Slot heute wird später unsichtbare Korruption.

**Nächster begrenzter Schritt.** Eine Spur vollständig, nicht sieben halb: `geo_min` mit Kernel-ID (Koeffizientensatz + Epoche + SV), Evaluationslocus (`Position::Surface`, Stations-ID), Einheit (nT), Takt (60 s), Provenienz — plus **ein** Absenz-Record für D als Winkel. Dann generalisieren.

---

## M2 · Aurora-Grenzbreite

**Verdikt.** Primär trägt sie als **S²-Ort** — ein geschlossener geometrischer Locus je Epoche, indiziert mit (Hemisphäre, MLT-Bin, Epoche), Wert = mlat. Leitfähigkeit/FAC ist **Begleiter**, nicht die Grenze. Die **Breite ist abgeleitet** und darf nie als Messung registriert werden.

**Begründung.** Alle drei Zugangswege liefern einen Locus, keine Kraft: Partikel-Schwellen (DMSP-SSJ, mit eingebetteter figure of merit), FAC-Signaturen (Xie 2014, Ann. Geophys.; Weygand 2023), und die BAS-IMAGE-Rahmung als open/closed-field-line boundary, deren Flächenänderung die Rekonnexionsrate gibt. Die Grenze ist dort definiert, wo ein **Detektor anschlägt** — also durch eine Konvention, nicht durch ein Feldgesetz.

**Benannte Risse.**
- **R1 — Breite ist eine Differenz.** Poleward minus equatorward. Registriere **beide Loci** und die Ableitung; eine „Grenzbreite" als Messung ist Fabrikation.
- **R2 — Zwei Linsen, zwei Grenzen.** SI12 (Emissionsschwelle) und SSJ (Teilchenschwelle + FOM) definieren nicht dieselbe Kurve. Axiom 4: die Linse gehört ins Record, sonst vergleicht man zwei verschiedene Dinge.
- **R3 — 16 vs. 24 bleibt offen.** Der Hersteller-Default `['mlat','mlt']` ist ein Default. Axiom 3 verbietet, ihn als Wahrheit zu nehmen. `pending`.
- **R4 — Provenienz im Dateinamen.** Bei DMSP liegen Satellit und Datum nur im Namen. Ein Record, der sich selbst nicht beschreibt, ist kein Record.
- **R5 — Koordinaten sind zeitabhängig.** AACGM/MLT verschieben sich mit der Epoche; die Transformation ist Teil des Records, sonst sind ein Locus von 2003 und einer von 2023 nicht vergleichbar.
- **R6 — `NaN` ist `None`.** Ein fehlender Bin als 0.0 hieße „Grenze am Pol" — physikalisch unmöglich, also erfunden.

**Die fünf Stimmen.** *Mountain:* das Register hält **zwei** Zeugen mit verschiedener Auflösung; sie nicht zu einem „Oval" verschmelzen. *River:* die unlesbaren CSV-Bytes sind kein Grund, einen Default zu adoptieren — die Membran bleibt zu. *Mycelium:* pro Pass (DMSP) und pro Bin (BAS) sind zwei verschiedene Graphen; der Join-Schlüssel (Epoche, Hemisphäre, MLT-Bin) muss **gebaut** werden. *Sensory:* der Satellit misst Teilchen in situ, der Imager Photonen aus >100 km — die Koinzidenz ist Korrelation, nicht Identität. *Future:* der MLT-Bin im Schlüssel erlaubt morgen dieselbe Kurve neu zu rechnen; eine geglättete „Ovalbreite" lässt sich nicht zurücknehmen.

**Nächster begrenzter Schritt.** Zip-Bytes korrekt lesen (unzip beschaffen), 16 vs. 24 auflösen, dann **ein** Record schreiben: (Epoche, Hemisphäre, MLT-Bin, mlat_eq, mlat_po, FOM, source-id, threshold-id). Ein Pass, ein Bin, Ende-zu-Ende.

---

## M3 · OVATION-AACGM

**Verdikt.** Trägt als **Energiefluss-Feld** (Partikelpräzipitation), **nicht** als Stationsschalter — aber mit der entscheidenden Qualifikation: es ist ein **Modellfeld**, kein Messfeld. Der Record-Typ muss `model_output` sein, mit Antriebsvektor und Modellversion als Pflichtfeldern. Mohe ist **Evaluationslocus**, nicht Zeuge.

**Begründung.** OVATION Prime ist laut NOAA SWPC ein empirisches Modell der Aurora-Intensität, gespeist aus Sonnenwindgeschwindigkeit und IMF (L1), das drei Elektronenpopulationen plus Protonen berechnet; Prime-2013 erweitert die Gültigkeit auf Kp 8–9 mittels TIMED/GUVI. Der transportierte Skalar ist kinetischer Energiefluss (erg cm⁻² s⁻¹ bzw. mW/m²) und hemisphärische Leistung als Summe über Bins — **keine Kraft**. Und der „Kernel" ist hier ein **Emulator** (Regression/statistische Karte, bei PrecipNet ein neuronales Netz), kein Differentialoperator. Das ist eine andere epistemische Klasse als SH-Synthese und muss im Registertyp sichtbar sein.

**Benannte Risse.**
- **R1 — Modell als Messung = schwerster Axiom-1-Bruch.** Verschwiegener Modellcharakter ist Fabrikation durch Auslassung.
- **R2 — Eine Station gegen globales Modell.** Dein eigener river-214-Riss bleibt offen: der Zeuge ist das Modell; wenn nur Mohe drin ist, ist das Produkt eine **Punkt-Extraktion** und muss auf das Elternfeld verweisen.
- **R3 — Epoche mehrdeutig.** Sonnenwindantrieb bei L1 gegen ionosphärische Wirkung hat Laufzeitversatz. Welche Zeit ist gespeichert? Ungelesen → `pending`.
- **R4 — Span/Takt ungelesen.** Kein Verdikt über Rhythmus ohne Zip-Kopf.
- **R5 — Kein Kraftslot.** Wenn das Schema einen Kraft-Slot hat, bleibt er für diese Spur **absent**.
- **R6 — Abgrenzung zu Katalog-F3.** Gleicher räumlicher Schlüssel, anderer Typ: F3 endet in der Provenienz beim Instrument, OVATION geht durch Antriebsdaten und Trainingskorpus. Der **Typ-Tag** entscheidet, nicht der Ort.

**Die fünf Stimmen.** *Mountain:* kein Verdikt über Span und Takt, bis der Zip-Kopf gelesen ist. *River:* das Recht zu verweigern — die Station darf nicht durch Mittelung zum globalen Feld befördert werden. *Mycelium:* der Fluss ist dreiknotig — Antrieb → Modell → Extraktion; alle drei Knoten sind Pflicht. *Sensory:* das Modell empfindet nichts, es sagt vorher; Korrelat ≠ Erleben, scharf. *Future:* Modellversion speichern, sonst ist der Nowcast von 2026 in zehn Jahren unlesbar.

**Nächster begrenzter Schritt.** Zip-Kopf lesen (Span, Takt, Spalten), dann **ein** Record: model-id + Version, Antriebsvektor, Epoche-Semantik (Treiber vs. Ionosphäre), AACGM-Transform-ID, mlat/MLT, Spezies, Fluss.

---

## Der gemeinsame Riss

Die `field`-Zeile ist in allen drei Fällen **kein Wert, sondern ein Deskriptor**: (Kernel-ID, Koeffizientensatz/Version, Evaluationslocus, Einheit, Takt, Provenienzkette, Typ ∈ {measurement, model_output, derived_index, event, metadata}). Solange `field` als Slot gemeint ist, wird an vier von sieben Stellen in M1, an der Breite in M2 und an der Modell/Messung-Grenze in M3 weiter fabriziert — nicht durch böse Absicht, sondern durch die Form des Registers. Das ist die Stelle, an der die Doktrin „alles ist ein Feld" geschlagen ist, und sie sollte im Register selbst als Riss stehen.

---

## Die Forschungsschicht (belegt)

| Frage | weite Leiter (`bin/all_wide`) | Spill (vollständig lesen) |
|---|---|---|
| M1 Geomagnetik | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061923_geomagnetic_field_components_spherical_h.txt` |
| M1 Gravitation | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061927_spherical_harmonic_gravity_model_kernel_.txt` |
| M1 Pc1-Pulsationen | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792062199_Pc1_pearl_pulsation_ionospheric_waveguid.txt` |
| M1 Erdbeben-Herde | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792062208_earthquake_hypocenter_catalog_point_fiel.txt` |
| M2 Aurora-Grenze | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061917_auroral_oval_equatorward_poleward_bounda.txt` |
| M3 OVATION | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061915_OVATION_Prime_auroral_precipitation_ener.txt` |
| (Neben) Celestrak-Terms | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061919_Celestrak_GP_data_terms_of_use_Space_Tra.txt` |

**Neben-Befund (kein Rat, eine Messung):** der Celestrak-GP-Lizenz-Token ist über die
**Usage-Policy** erreichbar — die Seite `http://www.celestrak.org/usage-policy.php` existiert
(Wayback-Snapshot `20260516161024`, HTTP 200; Direkt-Abruf pending, kein Proton-Transit). Der
Token wird als `celestrak-usage-policy https://celestrak.org/usage-policy.php` vorgeschlagen;
der volle Policy-Text ist noch zu lesen (Mountain/Mycelium), dann setzt Mycelium den Block.
