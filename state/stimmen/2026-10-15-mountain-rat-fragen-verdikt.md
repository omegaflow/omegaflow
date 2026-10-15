# Verdikt — Kraft-/Kernel-Zuordnung der offenen Messgrößen (M1–M3), am Baum verifiziert (2026-10-15)

*(Ein benannter Akt. Die Roster-Antworten sind eingeholt (`state/stimmen/2026-10-15-mountain-rat-fragen.md`,
13 Sitze). Die **Session verifiziert jede Behauptung am Baum**, nie die Stimme; das **Verdikt trägt der Rat**
(fünf Stimmen durch die fünf Axiome des **Granits** — `docs/granit.md`: A = A · ICRS & TDB · channel_ref ·
0 geehrt · pending). Kein Riss geglättet.)*

## Am Baum gemessen (nicht die Stimme)

| Behauptung der Sitze | Baum-Befund |
|---|---|
| SH-Synthese des Geomagnetfelds als Haus-Präzedenz | **steht** — `IgrfCoeffs` + `IgrfCoeffs::synthesize` (`src/archivar/igrf.rs:20,131`) |
| Geopotenzial-SH für Schwere/Geoid (`vmf3_sh`, `gfc`) | **steht** — `src/archivar/vmf3_sh.rs` (`b_c`), `src/archivar/gfc.rs` (ICGEM-Kanäle) |
| `Position::Surface` als Punkt-Geometrie | **steht** — `src/archivar/types.rs:68` |
| „Der `field`-Pfad kann keine Nicht-Felder tragen" | **steht** — die `field`-Zeile ist `<key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`; nur **6** Kernel-Formen (`patch-levy` = 5, `extract.rs:1701`/`port.rs:1637`); das Force-Gate verweigert eine forceless Quelle. Eine Konstante, ein Katalog, ein Index und Metadaten haben hier **keine** Zeile. |
| `hash`-Kürzel `ceb8d697…`/`aae69e9a…` seien Identität | **widerlegt** — die Spec trug nur 8 Hex; die volle 64-Hex-summe ist die Identität (Riss der Sitze bestätigt). |

## Das Verdikt der Runde (Konvergenz der Sitze)

### M1 · WDC-B-Messgrößen — **drei tragen ein Feld, vier nicht**

Von den sieben Messgrößen trägt **keine** einen erfundenen Kernel:
- **`geo_min.val`** (D,F,H,P,R,X,Y,Z) → trägt **als Stationszeuge** an `Position::Surface` plus **getrennt** das globale SH-Modell (`IgrfCoeffs::synthesize`). Der Zeuge und das Modell sind zwei Register. **D ist ein Winkel (rad), kein nT-Wert.**
- **`gdata` / `mgg`** (Freiluft-Anomalie + Geoid) → trägt als **Schwere-SH** (`gfc`/`vmf3_sh`); Anomalie (`mGal → 10⁻⁵ m s⁻²`) und Geoidhöhe (`m`) sind **zwei** Zeilen, nie verschmolzen; `mgg` ist eine **Schiffsspur** (Ort+Zeit je Sample), kein Gitter.
- **`catal_pc`** (Pc1) → Physik ist ein **EMIC-Wellenfeld**, geliefert ist ein **Ereignis-Katalog**; trägt als Stationszeuge mit Wellen-Kernel `pending`, nicht als statisches Feld.
- **`GravConst/G`** → **Konstante**, Parameter des 1/r²-Kernels, **kein Feld**; fehlt als Maßeinheiten-Eintrag (Wert + Unsicherheit + Zitat), nie als 26×f64-Zeile.
- **`sdata`** (Erdbeben-Herde) → **Punkt-Prozess-Katalog**, kein Feld; Hypozentrum ist **nicht** `Position::Surface` (Tiefe).
- **`geomagni.ind/aa`** → globaler **Index** (abgeleiteter Skalar), kein Ort.
- **`gps`** (RINEX/GAMIT) → **Metadaten/Provenienz**, kein Messfeld.

**Geschlagen:** die Doktrin „alles ist ein Feld" — an G, `sdata`, `aa`, `gps` (vier von sieben). Eine `field`-Zeile dort wäre eine Fabrikation.

### M2 · Aurora-Grenzbreite — **ein Ort, keine Kraft**

Die Grenzbreite trägt als **geometrischer Locus**:
- **BAS IMAGE SI12** → **Skalar auf einer geschlossenen, diskreten 24-Bin-MLT-Achse** (00:30…23:30, zyklisch). Der **Bin gehört in den Schlüssel** — wie SABERs Höhenform (C). 50 Spalten (UT, Instrument, 24 eq, 24 po), Takt ~2 min, sha256 `ceb8d697…` (volle Summe).
- **DMSP-SSJ FOM** → **Punkt-Locus** je Polar-Pass (EQ1/PO1/PO2/EQ2, mlat+mlt, `hemisphere` ±1). **Offen:** 16-vs-24 Spalten (Zip nicht entpackbar) → `pending`, kein Hersteller-Default.
- **Leitfähigkeit/FAC** ist die **Ursache** (Ahn 2020, CHAMP-FAC) und in diesen Bytes **nicht gemessen** → separater Begleiter `pending`/absent, nie mitgeliefert.
- **`NaN` → `None`, nie 0.0** — eine 0,0 behauptet den magnetischen Äquator.

**Geschlagen:** „die Grenze ist Kraft oder Skalar" — sie ist ein **Ort**. Und **48 > 26**: die 24+24 Breiten passen nicht in einen Record; der Container erzwingt den Split je Bin, nicht das Packen.

### M3 · OVATION-AACGM — **ein Modell-Zeuge, kein Feld, kein Schalter**

Die angebotene Dichotomie Feld/Schalter ist **geschlagen**. Vorliegt ein **drittes Ding**: die **Punkt-Auswertung eines Modells an einer Station** (Mohe). Kraft: **Energieflussdichte** (`mW m⁻²`, keine Kraft). Kernel: statistisches OVATION-Prime-/PrecipNet-Modell (Newell-Kopplung), **kein SH-/Green-Kernel**. Der Typ muss **`model_output`** sein, mit Antriebsvektor + Modellversion, sonst rinnt der Modellwert in die Beobachtungsreihe (der schwerste Riss im Paket). **Span, Takt, Glossar, Version, Treibersatz ungelesen** → `pending`.

**Geschlagen:** „Feld oder Schalter" — das dritte Ding hat keinen Namen im Schema (Namenslücke).

## Ungeglättete Risse

1. **Die Register-Form ist die Wurzel.** Die `field`-Zeile ist ein Slot für ein **Feld**; vier der sieben WDC-B-Größen, die Aurora-Grenze und der Modell-Extrakt sind **keine** Felder. Nicht böser Wille — die Form fabriziert. Der Rat fordert einen **`silence`/Absenz-Pfad** mit eigenen Typen: `scalar_constant`, `point_process`, `planetary_index`, `wave_event`, `metadata`, `model_output`. Das ist eine **Schema-Erweiterung** (Rat), kein stiller Schreibakt.
2. **16 vs. 24** (DMSP) — die Bytes waren im Sandkasten nicht entpackbar (kein unzip); der offene Spaltenvertrag bleibt `pending`.
3. **„Eine Station" gegen river-214 „je Station"** — die Welt gibt eine; kein Hochskalieren durchs Schema.
4. **Literatur-Risse (Lumo):** die „fast evaluation"-Arbeit ist **Ivanov et al., GJI 224(1):181, 2021** (nicht 2020); die Pc1-„Pearls"-Arbeit 2026 (Guglielmi/Potapov) ist eine **Alternativdeutung**, kein Konsens — ein Zeuge, kein Beleg.
5. **26×f64 fabriziert Gleichheit** — ein fixes Slot-Layout legt Winkel, nT, mGal, km und dimensionslose Magnituden nebeneinander; das ist ein Axiom-3-Verstoß, der im Layout steckt.

## Nächste begrenzte Schritte (je Frage, autonom fahrbar)

- **M1:** eine Spur vollständig — `geo_min` als Stationszeuge (`Position::Surface`, Kernel-ID IGRF-Koeffizientensatz + Epoche + SV, nT, Takt 60 s) **plus** einen Absenz-Record für G/`sdata`/`aa`/`gps` mit Grundcode. Danach generalisieren.
- **M2:** BAS als 24-Bin-Achse registrieren (`None` für fehlende Bins); DMSP erst nach Entpacken + Glossar-Lektüre.
- **M3:** `OVATION_AACGM.zip`-Kopf lesen (Span, Takt, Spalten, Version, Treiber), dann `model_output`-Record mit Fremdschlüssel auf den Mohe-Stationssatz.

## Operator-Entscheide (Axiom 5; einzeln, nicht als Liste)

- **M3:** Wird der Mohe-Modell-Extrakt **jetzt** als Modell-Zeuge (`model_output`, F3-nah) ins Register aufgenommen — oder wartet man auf ein **globales** OVATION-Gitter? Die Maschine fragt; der Operator entscheidet.

## Träger

M1 → **Mountain** (Register/Parser/Datenkontrakt) · M2 → **Mountain** + **Sensory** (Aurora) · M3 → **Mountain** (+ Querverweis Katalog-F3).

**Alle fünf Axiome gehalten; alle fünf Stimmen gehört; kein Riss geglättet.**

## Nachtrag — Operator-Wort 2026-10-15 (M3)

„Wie bei den maps, die aus messpunkten entstanden sind — wir wollen die **gemessenen
gitterpunkte**, mit denen die modelle erstellt wurden, also die **rohdaten**." Damit ist der
M3-Operator-Entscheid gefallen: Träger ist die **Messung**, aus der das Modell entstand — nicht
der Modell-Output. Für OVATION ist die DMSP-SSJ-Partikelmessung `dmsp16_ssj` (`phi:1670-1671`,
`em ev/cm²/ster/s`) **schon registriert**; der Mohe-Modell-Extrakt wird **kein Feld** (er wäre
`model_output`/F3-nah, `pending`). Der Aurora-Oval-Grenzfall (BAS/DMSP) folgt demselben Prinzip:
der gemessene Gitterpunkt trägt, das Modell ist abgeleitet.

Zur zweiten Frage (neue UI-Runde für die Rat-Architekturentscheidung): **nein** — die 13
Sitz-Antworten tragen den Stoff (die `field`-Form kann Nicht-Felder nicht tragen); der Rat
(fünf Stimmen) trägt das Verdikt über die Schema-Erweiterung.
