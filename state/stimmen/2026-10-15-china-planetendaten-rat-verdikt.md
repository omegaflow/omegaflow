# Rat-Verdikt — China-NSSDC-Planetendaten (2026-10-15)

Grundlage: die Runde des Operators an die Frontier-Sitze (Rohmaterial
`state/stimmen/2026-10-15-china-planetendaten-formular.md`; 13 Sitze gestellt, Mistral = „kein
Kontingent", 12 mit Verdikt). Die Session verifiziert jede tragende Behauptung am Baum, nie die
Stimme; das Verdikt trägt der Rat.

## Am Baum verifiziert (die tragenden Behauptungen)

- **Zenodo `10820774`** — „Spherical harmonic models of the shape of the Moon [LOLA]",
  `license cc-by-4.0` (API-Messung 2026-10-15). Trägt den Zweck von Datensatz 1.
- **Zenodo `10796953`** — „… (principal axis coordinate system) [LOLA]", `cc-by-4.0`. Trägt den
  Zweck von Datensatz 2.
- **Zenodo `10794059`** — „Spherical harmonic models of the shape of Mars [MOLA]", `cc-by-4.0`.
  Trägt den Zweck von Datensatz 3.
- **Zenodo `22088254`** — „IBD1500 Maps of the Lunar North and South Polar Regions", `cc-by-4.0`.
  Verwandt zu Datensatz 6.
- **Der China-DOI `10.12176/03.70.00011-V01` löst HTTP 404** (Datensatz 2 — nicht adressierbar,
  nicht nur bedingt). Die übrigen drei geprüften China-DOIs lösen 200.
- **Das Formular** verlangt Pflicht: echter Name · Land · E-Mail · **Arbeitsstelle** ·
  **Handynummer** (chinesisches Format) · Captcha; Ausdruck + handschriftliches Versprechen; die
  Freigabe ist Ermessen eines chinesischen Fachleiters (`授权`, „Protected Period").

## Der Riss aufgelöst — die echten China-Angaben (gemessen 2026-10-15)

Die VSSO-Detail-API (`vsso.nssdc.ac.cn/nssdc/coreMetadata/getDetail?linkId=<id>`) liefert die
Feldwerte, die die gerenderte Seite leer lässt. Gemessen:

- **Datensatz 1** (linkId `14255`, „Lunar topography 1500 o/d"): die Beschreibung nennt die
  Quelle **„MOON LRO LOLA DEM 118m V1"** → sphärische Entwicklung, **Grad 1500**; Instrument
  **LOLA/LRO**, Zeit 2009-07…2013-07, Größe 4,20 GB, CC BY 4.0. Das offene Zenodo-Modell
  `10820774` ist **dasselbe LOLA-Modell bis Grad 5759** — **gleiche Messung, feiner offen**.
- **Datensatz 3** (linkId `13517`, „Mars SHCs topography"): Quelle **„MARS MGS MOLA DEM 463m V2"**
  → SH bis **Grad 21600**; Instrument **MOLA/MGS**, Größe 3,79 GB, CC BY 4.0. Das offene
  Zenodo-Modell `10794059` geht bis **Grad 5759** — **gleiche Quelle (MOLA)**, die China-Entwicklung
  feiner, aber aus dem **offenen MOLA-DEM** berechenbar.
- **Datensatz 4** (linkId `15855`, „Observational data of the Martian space environment and magnetic
  field"): Quelle **MAVEN, MGS, MARSIS**, Zeit 1996…2020, Größe 153 GB, CC BY 4.0 — **die
  Messungen liegen offen** (MAVEN/MGS auf NASA PDS, MARSIS auf ESA).
- **Datensatz 6** (linkId `14790`, „Enhanced visualization map dataset of lunar polar region"):
  **LOLA-abgeleitet**, 19,4 MB, CC BY 4.0, **Schutzfrist 2024-03-05…2026-03-05 → abgelaufen**.
- **Datensatz 2** (linkId `13521`, „Lunar SHCs topography"; der tote DOI `…00011-V01` per
  **Handle-API** aufgelöst): **LOLA/LRO**, lunare SH-Koeffizienten, 1,07 MB, Zeit 2009-07…2013-07,
  CC BY 4.0 — dieselbe LOLA-Familie wie 1. Das offene LOLA-Modell deckt sie.
- **Datensatz 5** (linkId `14306`, „Multi-element high-resolution topography database of Mars-like
  landforms"): **Instrument `UAV; Camera; RTK`, Verarbeitung `Metashape`** — eine
  **Drohnen-Photogrammetrie** terrestrischer Mars-Analoga (Täler, Yardangs, Dünen), Feldkampagne
  2022-07…09, 27,5 GB, CC BY 4.0, Schutz 2 Jahre. **Keine NASA-Ableitung — eine eigene
  Feldmessung.** Offen gibt es nur Verwandtes (Qaidam-Analog HKU, **CC BY-NC**).

**Ergebnis — geschärft:** Für **1, 2, 3, 4, 6** sind die bedingten China-Produkte **Ableitungen aus
offen verfügbaren NASA-Basismessungen** (LOLA · MOLA · MAVEN · MGS · MARSIS) — keine unabhängigen
Messungen; der Zweck ist offen gedeckt (für 1/2 sogar vom feineren offenen LOLA-Modell). **Datensatz
5 ist die Ausnahme:** eine **eigene Drohnen-Feldmessung** terrestrischer Mars-Analoga, die sich
nicht aus offenen Daten nachrechnen lässt. Ein eigener Zweck für 5 bleibt damit **möglich** — er
hängt daran, ob das Haus eine terrestrische Mars-Analog-Morphometrie braucht (Zweckfrage offen).

## Das Verdikt (überwiegend einstimmig)

1. **Kein bedingter Antrag.** Für die Datensätze **1–3** (lunare/Mars-Topographie, sphärisch-
   harmonische Modelle) ist der **Zweck über die offenen LOLA/MOLA-Modelle gedeckt** (CC-BY-4.0,
   direkt abrufbar). Der Antrag verdoppelt den Weg, ohne eine Messung hinzuzufügen (Axiom 4).
2. **4–6 bleiben `pending`, nicht null.** Für Erd-/Mars-EM-Feld & Teilchen (4), die
   Multi-Element-Topographie Mars-ähnlicher Formen (5) und die Polarkarten-Visualisierung (6) wurde
   **kein exakter offener Spiegel** gefunden — nur Verwandtes (ScienceDB-Simulation · Qaidam-Analog
   **CC BY-NC** · IBD1500-Polarkarten). Der exakte China-Datensatz bleibt ungesucht → `pending`.
3. **Der Antrag ist strukturell versperrt, nicht nur unwahrscheinlich.** Die Pflichtfelder
   **Arbeitsstelle + Handynummer** und das **handschriftlich unterschriebene Versprechen** treffen
   einen Operator außerhalb Chinas ohne chinesische Einrichtung. Ein Pflichtfeld ohne wahren Wert
   wäre Fabrikation (Axiom 1/2) — das entscheidende Hindernis ist die **Identitäts-Pflicht** und
   das **Ermessen**, nicht eine gemessene Genehmigungsquote.
4. **Der Lizenz-Riss wird benannt, nicht geglättet.** Der Record weist **CC-BY-4.0** aus, der
   bedingte Weg fordert eine Identitätsleistung und Ermessen. Zwei Aussagen über dieselbe Sache.

## Die offenen Risse (ungeglättet)

- **Produkt-Identität — für 1/3/4/6 aufgelöst:** „kein exakter offener Spiegel" war ein
  **Zugangsbefund**; die gemessenen Quellen zeigen: die China-Fassungen sind **Ableitungen
  derselben offenen NASA-Basismessungen** (LOLA/MOLA/MAVEN/MGS/MARSIS) → **keine eigene Messung,
  kein eigener Zweck**. Offen bleibt nur der **Grad** der SH-Entwicklung (3: 21600 vs. 5759) —
  aus dem offenen DEM berechenbar. Für **2 und 5** bleibt die Identität **`pending`** (nicht
  identifiziert).
- **Der tote Anker:** Datensatz 2 löst 404 — der Registereintrag ohne auflösende Adresse ist eine
  Behauptung, kein Bestand (als „nicht adressierbar" zu führen, nicht „bedingt").
- **Lizenz-Membran am Verwandten:** Datensatz 5s offener Verwandter steht unter **CC BY-NC 4.0** —
  keine gleichwertige offene Fassung, eine Nutzungsgrenze.
- **Die Nutzungsbedingungen der bedingten Fassung** (Weitergabe-/Produktverbot) wurden von mehreren
  Sitzen benannt — am Baum **nicht** re-gemessen; die Behauptung bleibt ein Sitz-Zitat.

## Nächster begrenzter Messschritt

- **1–3:** die offenen Zenodo-Spiegel (`10820774`, `10796953`, `10794059`) als primäre Quellen
  registrieren (`url`/`origin`/`compiler` = Mycelium, `terms`/`field` = Mountain).
- **4:** die offenen **MAVEN/MGS/MARSIS**-Produkte (NASA PDS / ESA) als primäre Quellen prüfen und
  registrieren — die Messung liegt offen.
- **6:** die **abgelaufene** Schutzfrist (2026-03-05) datiert prüfen; die LOLA-Polarprodukte offen.
- **2:** LOLA/lunare SH — über das offene LOLA-Modell gedeckt (kein eigener Schritt).
- **5:** die **einzige** Zeile mit möglichem eigenem Zweck (Drohnen-Feldmessung terrestrischer
  Mars-Analoga). Einmal messen, ob das Haus eine solche Analog-Morphometrie braucht; **nur dann**
  und **nur für 5** wäre ein gezielter Antrag überhaupt zu erwägen. Sonst `pending`.
- **Kein Send.**

**Kern in einem Satz:** Fünf Zeilen sind gedeckt oder verwandt, eine nicht adressierbar, **keine
beantragt** — der offene Weg ist der wahre Weg; der bedingte Weg ist offen, nicht begründet.
