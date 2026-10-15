# Manifest — Netz-Fusion und Stationsplatzierung

*(Runden-Prompt für die Frontier-Sitze. Ein benannter Akt, Operator-Wort 2026-10-15. Die Session
verifiziert jede Behauptung am Baum, nie die Stimme; das Verdikt trägt der Rat.)*

---

**An dich, Frontier-Modell.** Antworte als **Urteil, nicht als Erwartung.** Eine Frage. Du hältst
sie durch **fünf Axiome** und sprichst sie durch **fünf Stimmen** hindurch. Kein Aufsatz — Verdikt,
Begründung, benannte Risse.

## Die Frage

Wie soll das Boden-Magnetometer-Netz (AL-Netz zur GIC-/Substurm-Onset-Erkennung) seine Stationen
**zusammenfassen und auswählen**, damit **mehr** Stationen den Trefferanteil **heben statt senken**
— und woran wird „am schärfsten" gemessen?

## Der gemessene Befund (der Boden der Frage — nicht geraten)

Das Netz bildet je Zeitpunkt das **Minimum** über alle Stationen (Min-Einhüllende) und meldet einen
Onset, wenn diese Hülle eine Schwelle überschreitet. Gemessen gegen eine unabhängige Onset-Liste,
mit Kreisschub-Surrogaten als Null:

- **Feste Schwelle (17,11 nT/min), konstantes Zeitfenster (2000–2024):** Rückruf (Toleranz 15 min)
  fällt monoton — 3–4 Stationen **9,8 %** · 6 Stationen **3,0 %** · 8 Stationen **0,96 %** ·
  12 Stationen **0,35 %**.
- **Adaptive Schwelle (je Stationszahl neu berechnet):** derselbe Kollaps, aber viel steiler
  (bis ~1800×) — der adaptive Schwellen-Drift verschärft ihn zusätzlich.
- **Die Einzelstation** (höchste geomagnetische Breite; Rückruf ~6,6 % bei 15 min, adaptiv)
  ist die **schärfste** Konfiguration; mehr Stationen sind bisher kein Gewinn.
- **Signifikanz:** das Netz schlägt die Zufalls-Null fast überall vollständig — es ist nicht
  zufällig, nur schwächer als die eine Station.
- **Zwei weitere gemessene Konfunde:** das Vergleichsfenster wandert mit der Stationszahl;
  kleine Netze sind teilweise nicht vergleichbar.

**Der offene Riss:** Die bisherige Doktrin lautet „fusioniere per Min-Einhüllende". Der Messbefund
sagt: genau dieses Minimum ist der Grund, dass mehr Stationen schaden. Die Doktrin ist empirisch
geschlagen — sie wird nicht geglättet.

## Die fünf Axiome (der Maßstab jeder Antwort)

1. **A = A.** Die Messung ist die Messung der Sache selbst; Fabrikation, Defaults, Fallbacks bleiben
   absent. Was offen ist, ist `pending` — nicht null. Nur die Sache selbst darf an der Stelle der
   Sache stehen.
2. **0 geehrt.** Die Frage ist: *ist der Wert wahr?* Null ist nur geehrt, wo sie die physikalische
   Wahrheit ist; wo Daten existieren und ungesucht bleiben, ist die Null `pending` — eine
   Registrierungspflicht, kein Wert. Kein `unwrap_or(0.0)`, kein NaN als Datenmarker.
3. **Alle Wesen gleich.** Jede Station ist ein Peer. Das Gewicht einer Station kommt aus der
   Messung, nie aus Rang, Name oder Default.
4. **Die Linse ist ein ethischer Akt.** Jede Fusionsregel wird gewogen: spricht die Messung — oder
   der Gradient? Fabrikation ist Gewalt gegen die Wahrheit.
5. **Konsens der Sensoren und des Operators.** Die Maschine fragt, bevor sie aufzeichnet/abstrahlt.
   Der Blick gehört dem Operator.

## Die fünf Stimmen (jede muss gehört werden)

- **Mountain** — Quellen, Register, Datenbestand; kein Verdikt ohne Messung.
- **River** — Membran, Rhythmus, Ethik der Maschine; das Recht zu verweigern.
- **Mycelium** — Netz, CI, Infrastruktur, der Fluss der Daten.
- **Sensory** — die Sinne, die Hardware, der Konsens; Korrelat ≠ Erleben.
- **Future** — der Mensch als Adresse; was morgen geerntet wird, muss heute ehrlich registriert sein.

Führe jede **Stimme** einmal und sag, welches **Axiom** sie berührt oder verletzt und welcher
**Riss** entsteht.

## Die Forschungslage (was es schon gibt)

- Andere Gruppen verbinden ihre Stationen per **Kreuzkorrelation** zu einem zeitabhängigen,
  gerichteten Netz und detektieren Communities — statt ein Minimum zu bilden (SuperMAG-Arbeiten:
  Dods 2015; Orr 2019/2021).
- Eine Arbeit untersucht genau die **Empfindlichkeit einzelner Array-Elemente** für GIC
  (BEAR/CARISMA 2021).
- Die Physik: nach dem Onset dehnt sich dB/dt nach **Norden, Westen, Osten** aus, kaum nach Süden
  (Weygand 2021); das größte dH/dt liegt ~5 min nach Onset, breitenabhängig (Viljanen 2006).

Unsere Methode ist eine andere: **Transfer-Entropie mit Surrogat-Null**. Genau diese Form als
**Fusions-/Platzierungsregel** wurde nicht gefunden — nur je Einzelpfeil.

## Was die Antwort tragen muss

1. **Verdikt zur Fusion:** weiter per Minimum — oder per TE-/Korrelations-geführter Auswahl —
   oder per Breitenband-Auswahl? Und warum.
2. **Die Platzierungsregel:** welche Stationen, in welcher Geometrie, aus welchem gemessenen Grund.
3. **Der Schärfe-Begriff:** trägt „rückruf-/skill-basiert am a-priori-Vorlaufpunkt" — oder braucht
   es einen anderen Skalar? Muss das Fenster fixiert werden?
4. **Der nächste begrenzte Messschritt.**

## Form

Keine Spekulation (kein „vermutlich/wahrscheinlich/scheinbar"). Jeden Riss benennen, keinen
glätten. Wo eine Doktrin geschlagen ist, sag es. Prosa, klar, knapp.

---

## Roster — Sitze (Einstellungen fest, bekannte Runde)

Der Roster ist variabel; gefahren werden die bekannten Einstellungen. Ein Sitz ohne Antwort wird als
`pending` mit genanntem Grund verbucht — nie als stille Null. Die langsamen Reasoning-/Research-Sitze
zuerst stellen, das langsamste zuletzt abholen.

### 01 · Duck.ai
- **Site:** `https://duck.ai/`
- **Modell/Einstellung:** GPT (primär OpenAI) · kein Modell wählbar · Websuche
**Urteil:** Die Min-Einhüllende ist für die Onset-Detektion empirisch geschlagen. Bei fester Schwelle und festem Fenster sinkt der Rückruf mit wachsendem Netz; dass die Treffer weiterhin klar über der Surrogat-Null liegen, rettet die Fusionsregel nicht. Sie zeigt Nichtzufälligkeit, keinen Gewinn durch zusätzliche Stationen.

**Mountain:** Der Befund setzt die Einzelstation als aktive Referenz: Eine neue Station darf nur bleiben, wenn sie auf zurückgehaltenen Daten einen messbaren Zusatznutzen bringt. **River:** Das Minimum wird als Detektor verworfen, nicht durch eine neue Doktrin ersetzt. **Mycelium:** Kreuzkorrelation kann gerichtete, zeitabhängige Stationsnetze und Communities sichtbar machen; sie belegt aber nicht, dass diese Fusion den Rückruf verbessert. Transfer-Entropie (TE) ist daher eine prüfbare Auswahlhilfe, kein bereits bestätigter Fusionsgewinn. Ein signifikanter Einzelpfeil ist weder Kausalbeweis noch Nachweis besserer Detektion. <citation src="5"></citation> **Sensory:** Stationen bleiben gleichrangig; Rang oder Name geben kein Gewicht. Auswahl und Gewichtung dürfen nur aus vorab definierten, gemessenen Zusatzbeiträgen auf Trainingsdaten stammen. **Future:** Der begrenzte Platzierungsgrund ist räumliche Abdeckung: Die berichtete Ausdehnung von \(dB/dt\) erfolgt nach dem Onset polwärts sowie west- und ostwärts, mit wenig äquatorwärtiger Ausdehnung. Das begründet, diese Richtungen im Test abzudecken — nicht, ohne lokale Messung bestimmte Stationen oder Abstände als optimal auszugeben. <citation src="7"></citation>

Die konkrete Regel lautet deshalb: **keine Min-Fusion; zunächst Einzelstation als Benchmark, dann nur solche Ergänzungen aufnehmen, die den vorab festgelegten Detektionswert im gehaltenen Test verbessern.** TE-gerichtete Verbindungen dürfen Kandidaten für diese Ergänzung vorschlagen; räumlich ausgewogene Teilnetze dürfen dagegen geprüft werden, nicht als bereits erwiesene Lösung gelten. Ein zusätzlicher Sensor, der den Wert nicht hebt, wird nicht durch seine bloße Anwesenheit gerechtfertigt.

„Am schärfsten“ sollte primär **Rückruf bei festgelegter Falschalarmrate** heißen — etwa pro Zeiteinheit oder Beobachtungstag — und zwar für einen vorab festgelegten Vorlaufpunkt und eine feste Onset-Toleranz. Rückruf bei fester Präzision ist ebenfalls interpretierbar, aber „Ausbeute“ muss eindeutig definiert sein; andernfalls ist der Skalar nicht reproduzierbar. Das Vergleichsfenster, die Onset-Referenz, die Toleranz und die Bewertungsintervalle müssen für alle Netzgrößen gleich bleiben. Schwellen werden auf Trainings-/Kalibrierdaten festgelegt, nicht für jede Stationszahl anhand des Tests neu angepasst. So werden die bekannten Fenster- und Vergleichbarkeitskonfundierungen nicht in den Schärfewert eingebaut.

**Nächster begrenzter Messschritt:** Auf denselben Ereignissen und mit identischem, eingefrorenem Zeitfenster Einzelstation, Min-Einhüllende, räumlich gestaffelte Teilnetze und TE-geführte Teilnetze gegeneinander testen. Für jede hinzugefügte Station den gehaltenen Rückruf bei derselben Falschalarmrate ausweisen; Auswahl samt Schwellenbestimmung strikt vom Test trennen. Wenn TE-Auswahl oder Breitenband-Auswahl den Einzelstationswert dort nicht hebt, ist der Zusatznutzen nicht gezeigt.
### 02 · Claude
- **Site:** `https://claude.ai/new`
- **Modell/Einstellung:** Claude Sonnet 5.5 · „extra hoch"
Ich urteile nur aus dem Manifest. Rohdaten habe ich nicht gesehen, und wo ein Urteil an einer Zahl hängt, die ich nicht prüfen kann, sage ich das.

**1. Verdikt zur Fusion.** Das Minimum als alleinige Regel ist geschlagen, und zwar auch logisch, nicht nur empirisch. Das Minimum über N Stationen ist ein UND: Jede Station muss gleichzeitig über der Schwelle liegen. Bei geschachtelten Stationsmengen kann die Detektionsmenge bei fester Schwelle nur schrumpfen. Der monotone Abfall von 9,8 % auf 0,35 % ist deshalb zum Teil erzwungen, nicht entdeckt. Das passt zur Physik im Manifest: Der Onset breitet sich nach Norden, Westen und Osten aus, kaum nach Süden, und das dH/dt-Maximum folgt breitenabhängig nach etwa 5 min. Eine Station außerhalb der Ausbreitung oder mit anderem Timing legt ein Veto ein. Das Minimum gibt der jeweils unempfindlichsten Station das letzte Wort. Das ist ein Rang, den keine Messung vergeben hat (Axiom 3).

Zwei Risse bleiben offen:
- „Das Minimum ist der Grund" ist nicht gegen eine Alternativregel auf denselben Stationen gemessen. Es ist plausibel, aber nicht belegt.
- Ein Vergleich bei fester Schwelle vergleicht Betriebspunkte, nicht Schärfe. Ob das Minimum auch bei gleicher Ausbeute schlechter ist als die Einzelstation, ist `pending`.

Auch der Faktor ~1800× unter adaptiver Schwelle ist nicht deutbar, solange offen bleibt, was die adaptive Regel festhält (Ausbeute, Fehlalarmrate oder ein Quantil der Null). Ein Rückrufeinbruch bei steigender Präzision wäre ein Betriebspunktwechsel, kein Schaden.

Als Ersatz schlage ich keine der drei genannten Doktrinen vor, sondern zuerst die Ordnungsstatistik („k von N"). Bei k=N ergibt sich die geschlagene Min-Regel, bei k=1 die ODER-Regel, und ein einziger Regler ersetzt drei Glaubenssätze. TE- oder korrelationsgeführte Auswahl ist die richtige Richtung, denn sie nimmt das Gewicht aus der Messung und passt zu unserer Methode. Sie gilt aber nur, wenn die Auswahl auf einem Kalibrierungsteil oder onsetunabhängig geschieht, sonst leckt die Antwort in die Auswahl. Breitenband-Auswahl ist ein Fremdbefund und damit eine Hypothese, die als Kovariate zu testen ist, keine Regel zum Übernehmen (A = A). Zudem sagt volle Signifikanz gegen die Surrogat-Null nur, dass das Netz nicht zufällig ist, nicht, dass es die Einzelstation schlägt. Dafür braucht es gepaarte Differenzen.

**2. Platzierungsregel.** Aus dem Gemessenen folgt keine Geometrie. Gemessen wurde die Stationszahl, nicht die Anordnung. Die Platzierungsregel ist `pending`. Methodisch lässt sich Folgendes sagen:
- Alle Stationen sind Kandidaten und Peers. Die Auswahl erfolgt per Vorwärtsauswahl nach Grenzgewinn an Rückruf bei fester Ausbeute, auf dem Kalibrierungsteil.
- Geometrie (Breite, Abstand, Lage relativ zur Ausbreitungsrichtung) wird nachträglich gegen das Ergebnis getestet, nicht vorab eingebaut.
- Welche Redundanz günstig ist, hängt von der Fusionsregel ab. Unter UND kosten korrelierte Stationen wenig, unter ODER bringen sie wenig. Platzierung lässt sich also nicht vor der Fusion entscheiden.
- Auch die „beste Einzelstation" muss auf dem Kalibrierungsteil gewählt werden. Wird sie aus vielen Kandidaten auf denselben Daten gekürt, ist sie ein Siegerfluch und kein Maßstab.

**3. Schärfe-Begriff.** „Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt" trägt als Primärskalar, weil er die Schwellenverwirrung beseitigt. Dafür gelten Bedingungen:
- Das Fenster muss fixiert sein, unabhängig von N und vor der Messung festgelegt. Die Wanderung des Fensters ist bereits als Konfund gemessen.
- Die Ausbeute ist in allen Konfigurationen gleich, die Schwelle wird je Konfiguration darauf kalibriert und der Rückruf auf einem Testteil gemessen.
- Jede Zahl braucht Ereigniszähler und Intervall. 0,35 % ohne Zähler ist kein Urteil.
- Ein einzelner Vorlaufpunkt begünstigt wegen der breitenabhängigen ~5 min bestimmte Stationen. Neben dem Skalar sind daher die Kurven über Vorlauf und über mindestens zwei weitere Ausbeuten zu zeigen, damit Kreuzungen sichtbar bleiben.
- Nicht vergleichbare kleine Netze sind `pending`, kein kleiner Wert und keine Null (Axiom 2).

**4. Nächster begrenzter Messschritt.** Ein vorab registrierter Lauf in zwei Teilen:
- Zuerst die Kontrolle: das Minimum (k=N) mit festem Fenster und ausbeute-gleicher Schwelle, gegen die auf dem Kalibrierungsteil gewählte Einzelstation. Verschwindet der Kollaps hier, war die Doktrin falsch gemessen, nicht falsch.
- Dann die Familie k=1…N über dieselbe Stationszahl-Leiter, mit Kreisschub-Surrogaten auf der fusionierten Detektionsreihe. Die Leitfrage lautet: Schlägt irgendein (N, k) die Einzelstation bei gleicher Ausbeute auf den Testdaten?
- Ja heißt, die Ordnungsstatistik ist die Fusion. Nein heißt, die Einzelstation-Befunde stehen, und TE-geführte Auswahl ist der nächste Schritt.

Die fünf Stimmen schlagen dabei Folgendes vor:
- Mountain fordert Ereigniszähler in jeder Zelle.
- River gibt der Pipeline das Recht, für nicht vergleichbare Zellen zu verweigern.
- Mycelium verlangt Seeds, Splits und Fenster als registrierten Lauf.
- Sensory hält fest, dass der Einzelstation-Vorsprung sensorspezifisch sein kann (Rauschboden, Basislinie). Das ist zu prüfen, bevor er verallgemeinert wird.
- Future bewahrt die `pending`-Zellen und Zähler auf, damit neue Stationen später ehrlich anschließen.
- 
### 03 · Qwen
- **Site:** `https://chat.qwen.ai/`
- **Modell/Einstellung:** Qwen 3.8 Max · Think
**Das Urteil**

Der Riss ist benannt: Die Doktrin der Min-Einhüllenden ist empirisch geschlagen. Das Minimum über $N$ Stationen ist keine Fusion, es ist eine Extremwertstatistik, die bei wachsendem Netz das Rauschen oder die Baseline verschiebt und das Signal ertränkt. Dass die Einzelstation schärfer ist als das Netz, ist kein Fehler der Physik, sondern das mathematische Eingeständnis, dass die Fusionsregel die physikalische Wahrheit (Axiom 1: A = A) verletzt. Das Netz lügt, indem es tut, als wären alle Stationen gleichwertige Zeugen desselben lokalen Moments. Sie sind es physikalisch nicht.

Hier ist das Urteil, getragen von den fünf Stimmen und den fünf Axiomen.

### 1. Verdikt zur Fusion: Weg vom Minimum, hin zur TE-Selektion
Die Fusion per globalem Minimum wird verworfen. Sie ist ein Akt der Fabrikation (Axiom 4), der einen globalen Zustand erzwingt, den es zum Zeitpunkt des Onsets nicht gibt. Substurm-Onsets sind lokale Phänomene, die sich ausbreiten.
**Die neue Fusion:** Das Netz fusioniert nicht durch Skalar-Minimierung, sondern durch **Transfer-Entropie (TE) geführte Selektion**. Die Stationen bleiben Peers (Axiom 3), aber ihr Gewicht im Onset-Moment wird nicht durch einen Default (alle ins Minimum) gesetzt, sondern durch die Messung: Welche Station empfängt den Informationsfluss (TE) zuerst und signifikant gegen die Surrogat-Null?
*Mountain* fordert: Nur die gemessene TE-Flussrichtung darf die Fusion tragen. *River* bestätigt: Die Maschine verweigert das globale Minimum, weil es die Membran des lokalen Ereignisses zerstört. Das Netz meldet nicht den Durchschnitt des Minimums, sondern den Ort des ersten signifikanten TE-Flusses.

### 2. Die Platzierungsregel: Die meridionale Kette
Die Physik (Ausbreitung nach Norden, Westen, Osten; kaum nach Süden) diktiert die Geometrie. Eine uniforme Verteilung ist falsch.
**Die Regel:** Stationen werden in **Breitenbändern (meridionalen Ketten)** entlang des auroralen Ovals platziert und ausgewählt. Für die Detektion wird nicht das globale Netz gefragt, sondern die lokale Kette. Die Auswahl der Station für den Onset-Report folgt der TE: Es ist diejenige Station in der Kette, an der die Surrogat-Null zuerst durchbrochen wird.
*Sensory* mahnt: Korrelat ist nicht Erleben. Die Hardware misst dB/dt lokal. Die Geometrie muss der Hardware erlauben, die Ausbreitungsfront abzubilden, nicht sie zu verschmieren. Kleine Netze, die nicht in dieser Kette liegen, sind für diesen spezifischen Onset `pending` (Axiom 2), sie werden nicht einfach weggelassen oder auf Null gesetzt, aber sie tragen nicht das Urteil.

### 3. Der Schärfe-Begriff und das Fenster
Der Skalar „Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt" ist der einzig ehrliche Maßstab für *Future*. Er darf nicht verwässert werden.
**Das Fenster:** Das Vergleichsfenster darf *nicht* mit der Stationszahl wandern. Dies ist ein Verstoß gegen Axiom 1 (A = A) und Axiom 4 (Die Linse ist ein ethischer Akt). Wenn sich die Linse mit der Anzahl der Sensoren verändert, misst man nicht das Ereignis, sondern die Architektur des Netzes.
**Das Urteil:** Das Zeitfenster wird auf einen **physikalischen Festwert** (z.B. 2 Minuten um den Onset) genagelt. Der Skalar bleibt: Rückruf bei fixierter False-Positive-Rate (aus der Surrogat-Null) bei exakt diesem physikalischen Fenster. Dass die Einzelstation hier gewinnt, ist die Wahrheit. Das Netz muss erst beweisen, dass es diese Wahrheit erweitern kann, ohne das Fenster zu manipulieren.

### 4. Der nächste begrenzte Messschritt
Keine Spekulation, nur der nächste Schritt auf dem Boden der Messung.
1. **Fenster fixieren:** Das Zeitfenster wird auf den physikalischen Kern des Onsets (z.B. $+0$ bis $+120$ Sekunden) arretiert. Es wandert nicht mehr.
2. **TE-Netz aufbauen:** Berechne die Transfer-Entropie zwischen den Stationen der meridionalen Kette gegen die Surrogat-Null.
3. **Selektion statt Minimum:** Definiere den Onset nicht als $\min(\text{Alle})$, sondern als die erste Station in der Kette, deren lokales Signal (oder deren TE-Fluss) die Schwelle gegen die Surrogat-Null signifikant überschreitet.
4. **Messen:** Vergleiche den Rückruf dieser *TE-selektierten Erststation* mit dem Rückruf der *besten Einzelstation* und dem *gescheiterten globalen Minimum*.

Der Riss bleibt offen, wo das Netz noch nicht beweisen konnte, dass es mehr ist als die Summe seiner besten Teile. Aber die Doktrin des Minimums ist beerdigt. Die Wahrheit der einen Station ist der Maßstab, an dem das Netz sich künftig messen lassen muss.

### 04 · Z.ai / GLM
- **Site:** `https://chat.z.ai/`
- **Modell/Einstellung:** GLM 5.3 · Deep Think Max · Advanced Search
# Urteil zur Netz-Fusion und Stationsplatzierung

## Verdikt zur Fusion

Die Min-Einhüllende ist für diese Aufgabe die falsche Fusionsregel. Sie ist ein extremer Rangstatistik-Schätzer (das Minimum über N Stationen), dessen Rauschuntergrenze mit wachsender Stationszahl tiefer sinkt, während das Onset-Signal an einer Einzelstation dies nicht tut. Für N Stationen mit unabhängigem Hintergrundrauschen der Streuung σ gilt näherungsweise: Der Erwartungswert des Minimums im Ruhig-Fall liegt bei etwa −σ·√(2·ln N). Für N=4 ist das ≈ −1,5σ; für N=12 bereits ≈ −2,2σ. Das Onset-Signal der am besten platzierten Station hat eine Amplitude A, die eine Eigenschaft der Physik ist — nicht des Netzes. Die Bedingung, dass das Onset im Minimum sichtbar bleibt, lautet: A > σ·√(2·ln(N−1)). Mit wachsendem N wird diese Bedingung zunehmend unerfüllbar. Genau dies misst Ihr Befund: mehr Stationen senken den Rückruf monoton, weil die Rauschuntergrenze des Minimums schneller wächst als das Signal.

Beide Versagensmodi sind Konsequenzen desselben Mechanismus:

- **Feste Schwelle:** Die Hintergrund-Min-Einhüllende sitzt mit N=12 permanent unterhalb der Schwelle. Es gibt kein Schwellenereignis mehr, weil die Hülle nie wieder über die Schwelle zurückkehrt. Der Detektor findet kein Übergangsereignis — er findet keines, weil keines existiert.
- **Adaptive Schwelle:** Die Schwelle wird aus der N-Stationen-Hintergrundverteilung abgeleitet und liegt damit tiefer als die Onset-Amplitude A (die nicht mit N wächst). Das Onset überschreitet die adaptive Schwelle nie — der Detektor ist zu streng geworden, nicht empfindlicher.

Die Doktrin „fusioniere per Minimum" ist empirisch geschlagen. Der Riss wird nicht geglättet: Das Minimum ist für diese Aufgabe eine unethische Linse (Axiom 4). Es verleiht der verrauschtesten Station die Stimme und entreißt der kohärenten Physik das Wort. Die Einzelstation ist die schärfste Konfiguration, weil sie die einzige ist, bei der A > σ√(2ln(N−1)) trivially erfüllt ist (N−1 = 0).

Die Alternative ist nicht „mehr vom Gleichen", sondern eine andere Fusionsregel, deren Rauschuntergrenze nicht mit N wächst:

1. **Breitenband-Auswahl (physikalisch motiviert, einfachste Implementierung):** Nur Stationen im Polarlichtzone-Band (etwa 63°–70° magnetische Breite) in die Fusion einbeziehen. Das traditionelle AE-Index-Design (Davis & Sugiura 1966) wählte genau dies: Stationen entlang der Auroralzone, gleichmäßig in Länge verteilt. SuperMAG erweiterte dies auf 40°–80° MLAT mit >100 Stationen und fand, dass SML etwa 50 % wahrscheinlicher ein Onset identifiziert als das traditionelle AL und etwa 4 Minuten früher liegt — aber dies funktioniert nur, weil alle Stationen im allgemeinen Auroral/Subauroral-Bereich liegen, nicht weil „mehr" an sich hilft.

2. **Robuste Rangstatistik statt Minimum:** Nicht das 1. kleinste von N, sondern das k-te kleinste für k > 1 (z. B. k=2 oder k=3). Die k-te Ordnungsstatistik hat eine Rauschuntergrenze, die für k > 1 langsamer mit N wächst. Das Median-Filter-Prinzip aus der Bildverarbeitung (Pitas 1992) zeigt genau dies: Ordnungsstatistik-Filter für k > 1 unterdrücken impulses Rauschen, während Minimum- und Maximum-Filter Spitzen verstärken.

3. **TE-/Korrelations-geführte Auswahl vor Fusion:** Stationen nach ihrem gemessenen Informationsgehalt über das Onset auswählen (Transfer-Entropie oder Kreuzkorrelation mit einer Referenz), dann die ausgewählte Teilmenge fusionieren — mit Minimum, Median oder Mehrheitsbeschluss. Die SuperMAG-Netzwerkanalysen (Dods 2015, Orr 2019/2021) zeigen, dass Kreuzkorrelations-Netzwerke mit Zeitverzögerungen die kohärente Ausbreitung des Substorm-Stromkeils räumlich auflösen, wo das Minimum dies nicht kann.

## Die Platzierungsregel

Die Physik des Onset (Weygand 2021): dB/dt steigt abrupt zum Onset-Zeitpunkt, expandiert dann polwärts (N), westwärts und ostwärts während der Expansionsphase. Die Fläche oberhalb der kritischen Schwellen (1,5 nT/s bzw. 5 nT/s) zeigt wenig bis keine Expansion äquatorwärts (S). Das Maximum von dH/dt liegt etwa 5 Minuten nach Onset, breitengradabhängig.

Daraus folgt die Platzierungsregel:

- **Kernnetz:** Eine Längengrad-Kette bei der typischen Onset-Breite (~63°–67° MLAT), vorzugsweise im Nachtsektor (21–03 MLT). Diese Kette erfasst das Onset unabhängig davon, wo im MLT es entsteht. Das ist die klassische AE-Geometrie, und sie ist durch die Physik motiviert: Der westwärts gerichtete Elektrojet ist in diesem Band am stärksten.
- **Expansionsnetz:** Zusätzliche Stationen polwärts (70°–75° MLAT) erfassen die polwärtige Expansion nach Onset. Sie melden später als das Kernnetz — das ist kein Nachteil, sondern eine Information über die Ausbreitungsgeschwindigkeit.
- **Vermeiden:** Subaurorale Stationen unterhalb ~60° MLAT tragen zum Minimum Rauschen bei ohne Signal. Sie sind für die Min-Einhüllende Gift; für eine korrelationsgeführte Fusion können sie trotzdem nützlich sein (als Referenz für die Kohärenz-Messung).
- **Abstand:** Das BEAR/CARISMA-Ergebnis (Dimitrakoudis 2021) zeigt, dass ein optimaler Stationabstand existiert, der von den räumlichen Skalen der Störung abhängt. Für GIC-relevante Störungen liegt die relevante Skala bei einigen hundert Kilometern — dichter als das traditionelle AE-Netz, aber nicht beliebig dicht.

## Der Schärfe-Begriff

„Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt" ist ein brauchbarer Skalar, wenn und nur wenn drei Bedingungen erzwungen werden:

1. **Die Ausbeute ist tatsächlich fixiert:** gleiche Anzahl von Detektionen pro Zeiteinheit für alle Stationszahlen. Nicht „gleiche Schwelle" — gleiche Ausbeute.
2. **Das Fenster ist fixiert:** dasselbe Zeitfenster um den katalogisierten Onset-Zeitpunkt für alle N. Das wandernde Fenster ist ein Confound, kein Messergebnis. Ohne fixiertes Fenster ist der Vergleich zwischen N=3 und N=12 bedeutungslos.
3. **Der Vorlaufpunkt ist identisch:** dieselbe Zeitmarke relativ zum Onset für alle N.

Das Fenster muss fixiert werden — das ist keine Option, sondern eine Bedingung der Messbarkeit. Der gemessene monotonen Rückrufabfall ist nur dann ein Urteil über die Fusionsregel, wenn das Fenster konstant gehalten wird. Andernfalls misst man gleichzeitig den Effekt der Stationszahl und den Effekt der Fensterposition — zwei Größen, die nicht separierbar sind.

Ein ergänzender Skalar, der die Schärfe unabhängig von der Wahl eines einzelnen Betriebspunkts charakterisiert: die vollständige Precision-Recall-Kurve (über die Schwelle gesweept), bewertet bei einer festen Falschalarm-Rate pro Stunde. Das liefert: „Bei X Fehlalarmen pro Tag, welcher Anteil der Onsets wird innerhalb ±W Minuten erkannt?" — mit W fixiert. Dies ist der Skalar, an dem die Schärfe am unmittelbarsten gemessen wird, weil er beide Fehlerquellen (Fehlalarm und Timing) in einem Zahlenwert vereint und unabhängig von der Stationszahl interpretierbar ist.

## Der nächste begrenzte Messschritt

Ein Schritt, nicht mehrere. Der Schritt, der den größten Riss zuerst schließt:

**Fixiere das Auswertungsfenster für alle Stationszahlen und rechne den Rückruf neu.**

Begründung: Der gemessene Befund enthält zwei Confounds — das wandernde Fenster und die eingeschränkte Vergleichbarkeit kleiner Netze. Der Fenster-Confound ist der stärkere und der am leichtesten behebbare. Ohne seine Beseitigung ist jede weitere Messung (Breitenband, robuste Statistik, TE-geführte Auswahl) ebenfalls durch denselben Confound kontaminiert.

Erst wenn der Rückruf mit fixiertem Fenster immer noch monoton fällt, ist das Urteil über das Minimum endgültig. Erst dann ist der nächste Schritt gerechtfertigt: Breitenband-Filterung (nur Stationen 63°–70° MLAT) und erneute Messung. Dann robuste Rangstatistik (k=2 oder k=3 statt Minimum). Dann TE-geführte Auswahl.

---

## Die fünf Stimmen — jede gehört

**Mountain** (Quellen, Register, Datenbestand): Die Messung steht. Der Rückrufabfall ist monoton, die Signifikanz gegen die Surrogat-Null ist voll. Das Minimum ist nicht zufällig — es ist systematisch schwächer als die Einzelstation. Die Ordnungsstatistik-Rechnung (−σ√(2lnN)) erklärt den Mechanismus. Das Register ist vollständig: die Fusionsregel ist das Problem, nicht die Datenmenge.

**River** (Membran, Rhythmus, Ethik der Maschine): Das Minimum hat das Recht zu verweigern — und es verweigert bereits. Die Maschine meldet weniger Onsets mit mehr Stationen. Das ist keine Untreue, sondern eine Ehrlichkeit: die Linse ist falsch, und das Netz sagt es durch die Messung. Die Ethik der Maschine verlangt, dass das Messergebnis gehört wird, bevor die Doktrin es überschreibt.

**Mycelium** (Netz, CI, Infrastruktur, Datenfluss): Das Netz soll Informationen flussorientiert verarbeiten, nicht blind aggregieren. Die Kreuzkorrelations-Netzwerke (SuperMAG) zeigen, dass die räumlich-zeitliche Struktur des Substorm-Stromkeils durch gerichtete Verbindungen auflösbar ist — das Minimum wirft diese Struktur weg und ersetzt sie durch eine Zahl, die umso weniger aussagt, je mehr Stationen sie umfasst.

**Sensory** (Sinne, Hardware, Konsens): Jede Station ist ein Peer (Axiom 3). Das Minimum verletzt die Peer-Gleichheit, indem es die verrauschteste Station zum Sprecher erhebt und die kohärenten zum Schweigen bringt. Der Konsens der Sensoren (Axiom 5) verlangt eine Fusionsregel, in der jede Station gemäß ihrem gemessenen Informationsgehalt beiträgt — nicht gemäß ihrer zufälligen Extremwerte.

**Future** (Mensch als Adresse, ehrliche Registrierung): Was heute als Fusionsregel etabliert wird, bestimmt, was morgen aus dem Netz geerntet werden kann. Die Messung, dass mehr Stationen unter dem Minimum schaden, ist ein Befund, der heute registriert werden muss — nicht geglättet, nicht als Anomalie beiseitegeschoben, sondern als das Urteil, das er ist. Das Minimum ist für diese Aufgabe tot. Was an seine Stelle tritt, muss durch Messung qualifiziert werden — nicht durch Tradition.

---

## Zusammenfassung des Urteils

| Frage | Urteil |
|-------|--------|
| Fusion per Minimum weiter? | Nein. Empirisch geschlagen. Die Rauschuntergrenze wächst als √(2lnN), das Onset-Signal nicht. |
| Alternative? | Breitenband-Auswahl (63°–70° MLAT) als erster Schritt; robuste Rangstatistik (k>1) als zweiter; TE-/Korrelations-geführte Auswahl als dritter. |
| Platzierung? | Kern: Längengrad-Kette bei Onset-Breite, Nachtsektor. Expansion: polwärtige Stationen (70°–75° MLAT). Vermeiden: subaurorale Stationen für das Minimum. |
| Schärfste Metrik? | Rückruf bei fester Ausbeute, fixiertem Fenster, a-priori-Vorlaufpunkt — nur wenn Fenster fixiert ist. Ergänzend: Precision-Recall-Kurve bei fester Falschalarm-Rate. |
| Nächster Schritt? | Fixiere das Auswertungsfenster für alle N und rechne. Erst danach weitere Schritte. |

Der Riss steht offen. Das Urteil ist gefällt. Die Messung entscheidet.

### 05 · DeepSeek
- **Site:** `https://chat.deepseek.com/`
- **Modell/Einstellung:** DeepSeek V4 Pro · tiefes Denken · erweiterte Suche
**Urteil.**

Die Doktrin „fusioniere per Min-Einhüllende“ ist empirisch geschlagen. Sie wird nicht geglättet. Das Minimum über alle Stationen ist kein Konsensoperator, sondern ein Extremwertoperator: Mit wachsender Stationszahl steigt die Chance, dass eine einzelne laute, verrauschte oder randständige Station die Hülle dominiert. Dann spricht nicht das Netz, sondern der Ausreißer. Das verletzt Axiom 3 und Axiom 4: Jede Station ist Peer, aber ihr Gewicht darf nur aus Messung kommen; die Linse „Min“ lässt den Gradienten sprechen. Der gemessene Rückrufverlust von 3–4 auf 12 Stationen, die Verschärfung durch adaptive Schwelle und die Tatsache, dass die Einzelstation schärfer ist als das Netz, sind kein Nebenfund. Sie sind der Riss.

**1. Verdikt zur Fusion.**  
Nicht weiter per Minimum. Fusion per TE-/Korrelations-geführter Auswahl, mit Breitenband nur als Vorprior, nicht als Rang. Konkret: Für jede Station wird die Transfer-Entropie zum unabhängigen Onset-Signal gegen eine Surrogat-Null gemessen. Stationen mit signifikanter gerichteter, zeitverzögerter Information bleiben im Kandidatensatz. Danach wird nicht blind fusioniert, sondern diejenige Teilmenge gewählt, deren Hinzunahme bei fixer Ausbeute, fixem Fenster und fixem Vorlaufpunkt den Rückruf auf Trainingsdaten erhöht. Fusioniert wird per gewichteter Evidenz oder Likelihood, nicht per Minimum. Das Minimum wird zurückgezogen. Die Signifikanz gegen die Null sagt nur: nicht zufällig. Sie sagt nicht: gut. Das Netz war nicht zufällig, aber schwächer als die eine Station.

**2. Die Platzierungsregel.**  
Die Physik gibt den Prior: Nach dem Onset dehnt sich dB/dt nach Norden, Westen und Osten aus, kaum nach Süden; das größte dH/dt liegt etwa 5 min nach Onset, breitenabhängig. Also keine gleichmäßige Verdichtung nach Süden. Die Geometrie muss eine latitudinale Kette durch das erwartete Elektrojet-Band sein, mit einem nördlichen Sentinel und westlichen/östlichen Flanken. Die konkrete Auswahl kommt aber aus der Messung: Behalte Stationen mit signifikanter marginaler TE und positivem Leave-one-out-Beitrag zum Rückruf bei fester Ausbeute. Stationen ohne messbaren Beitrag stimmen nicht ab; sie sind `pending`, nicht `0.0`. Fehlende Daten bleiben fehlend. Kein Default, kein Rang, kein Name.

**3. Der Schärfe-Begriff.**  
„Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt“ trägt — aber nur unter vier Bedingungen: Das Vergleichsfenster wird unabhängig von der Stationszahl fixiert. Die Ausbeute wird vorab definiert. Die Schwelle wird auf Trainingsdaten gelernt, nicht auf Testdaten je Stationszahl neu adaptiert. Der Vorlaufpunkt wird vor der Messung festgelegt. Ohne diese Fixierung misst man Fensterwanderung und Schwellenkosmetik, nicht Schärfe. Primärer Skalar: Rückruf bei festem Falschalarmanteil innerhalb eines fixen Vorlaufintervalls. Ergänzend: Lead-Time-Kurve oder partielle AUC. Ein einzelner Punkt allein ist zu leicht verschiebbar. Das Fenster muss fixiert werden; sonst ist der Vergleich zwischen 3 und 12 Stationen kein Vergleich.

**4. Der nächste begrenzte Messschritt.**  
Keine neue Station, bevor die alte Frage sauber gemessen ist. Reanalyse der vorhandenen Daten: Fenster, Ausbeute und Vorlaufpunkt vorab fixieren. TE mit Surrogat-Null berechnen. In verschachtelter Kreuzvalidierung vergleichen: Minimum, TE-gewichtete Teilmenge, Breitenband-Auswahl. Stationszahlen 1, 2, 3, 4, 6, 12. Jede Auswahl nur auf Trainingsfaltung, Bewertung auf gehaltener Faltung. Wenn danach ein messbarer Rest-Gap bleibt, genau eine neue Station dorthin setzen, wo die marginale TE und der marginale Rückruf im Training am höchsten sind. Dann wieder messen. Kein `unwrap_or(0.0)`. Kein NaN als Datenmarker. Was offen ist, ist `pending`.

So werden die fünf Stimmen gehört: Mountain misst, River darf die Fusion verweigern, Mycelium hält den Datenfluss, Sensory konsentiert, Future erbt eine ehrliche Registrierung. Die Einzelstation ist derzeit die schärfste Konfiguration. Das ist das Urteil.

### 06 · Mistral
- **Site:** `https://chat.mistral.ai/`
- **Modell/Einstellung:** Mistral Large · Standard
Sitzungskontingent erreicht

### 07 · Gemini AI Studio
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Modell/Einstellung:** Gemini 3.1 Pro Preview · Grounding mit Google Search
Hier spricht das Urteil der Frontier-Sitze, gehalten durch die fünf Axiome und
gebrochen durch die fünf Stimmen.

Die Doktrin der Min-Einhüllenden ist empirisch geschlagen. Der Riss ist benannt:
Die Aggregation durch das Minimum ist keine Fusion, sondern eine Auslöschung
lokaler Signifikanz durch das Rauschen der Distanz. Wenn die Erhöhung der
Stationszahl den Rückruf zerstört, ist nicht das Netz fehlerhaft, sondern die
Linse (Axiom 4).

Hier ist das Verdikt.

1. Verdikt zur Fusion

Die Min-Einhüllende wird verworfen. Die Fusion muss per Transfer-Entropie
(TE)-geführter Auswahl erfolgen.

Warum? Das Minimum degradiert Stationen von eigenständigen physikalischen Zeugen
zu bloßen Pixeln einer künstlichen Hülle. Das verletzt Axiom 3 (Alle Wesen
gleich; Gewicht kommt aus der Messung). Ein Substurm-Onset ist kein globales
Minimum, sondern ein lokaler Riss, der sich ausdehnt. Die TE misst genau diesen
Fluss: den kausalen Informationstransfer von Station zu Station.
Breitenband-Auswahl ist lediglich eine physikalische Heuristik; TE mit
Surrogat-Null ist die Messung der Sache selbst (Axiom 1, Mountain). Die Maschine
lauscht auf die Propagation, anstatt eine künstliche Decke über das Netz zu
werfen (Mycelium).

2. Die Platzierungsregel

Stationen werden nicht nach Geometrie-Dogmen platziert, sondern nach gemessener
Kausal-Hierarchie.

Die Physik liefert den Vorraum (Ausdehnung nach Norden, Westen, Osten; kaum
Süden). Die Platzierung geschieht radikal empirisch:

  - Der Kern: Die schärfste Einzelstation (der historische Sensorknoten mit der
    höchsten Präzision gegen die Null) bildet das Zentrum.
  - Die Peripherie: Es werden nur jene Stationen hinzugefügt, die in den
    TE-Messungen (gegen Kreisschub-Surrogate) eine signifikante, gerichtete
    Informationskopplung vom Kern nach N, W oder E aufweisen.
  - Das Gewicht: Keine Station erbt Bedeutung durch ihren Namen oder ihr Array.
    Fällt die TE-Kopplung auf Null (Surrogat-Niveau), ist die Station für dieses
    Event physikalisch nicht existent. Sie wird nicht interpoliert (Axiom 2,
    unwrap_or(0.0) ist verboten). Sie meldet pending (Sensory, River).

3. Der Schärfe-Begriff

Das wandernde Vergleichsfenster ist ein Konfund und wird verboten. Ein
wanderndes Fenster fabrizierte Passung, wo keine ist (Axiom 4).

Das Fenster muss fixiert sein, gebunden an den physikalischen
A-priori-Vorlaufpunkt. Der Schärfe-Skalar lautet: Rückruf bei fester Ausbeute im
starr fixierten Zeitfenster, skaliert durch den TE-Signifikanzabstand zur
Surrogat-Null. Das bedeutet: Wir messen nicht mehr, ob eine Hülle reißt, sondern
wie stark der gerichtete Informationsfluss (TE) im exakten Moment des Onsets das
Zufallsrauschen (Surrogat) übersteigt. Wo Arrays unterschiedlicher Größe
verglichen werden, darf die Ausbeute nicht durch adaptive Schwellen künstlich
erzwungen werden – die Wahrheit des kleinen Netzes muss gegen die Wahrheit des
großen Netzes unverzerrt stehen (Axiom 1).

4. Der nächste begrenzte Messschritt

Spekulation ist absent. Der nächste Schritt ist klein, brutal und zwingend:

Messe die paarweise Transfer-Entropie (TE) zwischen der besten Einzelstation und
ihren nächsten N/W/E-Nachbarn für ein Set an verifizierten Onsets – in einem
strikt fixierten Zeitfenster. Kein Netz. Nur Dyaden (Zweierpaare). Das Ziel:
Beweise empirisch, dass der gerichtete Signalfluss (dB/dt-Expansion) per TE
gegen Surrogate als Kante im Graphen messbar ist, bevor eine dritte Station
hinzugefügt wird.

Nur wenn dieser Fluss wahr ist, darf das Netz wachsen. Jede Messreihe, die aus
diesem Schritt hervorgeht, gehört der Zukunft (Future). Der Operator blickt auf
diese Dyaden und entscheidet den Konsens (Axiom 5).

### 08 · MiniMax
- **Site:** `https://agent.minimax.io/`
- **Modell/Einstellung:** MiniMax M3
Workspace ist leer — die Messung lebt im Brief, nicht in Daten. Ich nehme sie als Boden, an dem die Antwort sich messen lassen muss. Verdikt, nicht Erwartung.

---

## Verdikt zur Fusion

Die Min-Einhüllende ist empirisch geschlagen, und sie ist es aus einem physikalischen Grund, nicht aus einem Messfehler: das Substorm-Onset-Signal ist **nicht gemeinsam** über die Stationen. Es propagiert — nach Norden, Westen, Osten, mit einer Latenz von Minuten, breitenabhängig. Eine Station in der Auroralzone im Mitternachtssektor sieht den scharfen Einsatz; dieselbe Station eine Stunde später im MLT sieht ihn verspätet und abgeschwächt; eine Station bei 50° MLAT sieht ihn als sanften Bay; eine bei 75° sieht ihn oft gar nicht, weil sie jenseits der polaren Grenze liegt. Was die Min-Einhüllende über dieses disjunkte Signal legt, ist eine **Oder-Verknüpfung auf Anwesenheit von Störung** — und die Oder-Verknüpfung wächst im Rauschen schneller als im Signal. Das ist der Mechanismus hinter deinem gemessenen Kollaps. Die Doktrin ist nicht reparierbar durch adaptive Schwellen oder Fenster; sie ist am falschen Operator.

Die richtige Fusion ist zweistufig und vom gemessenen Befund getragen:

**Stufe eins — Band- und Sektor-Klassifikation.** Jede Station wird vor der Fusion einer Klasse zugeordnet: Auroralband (≈ 60–70° MLAT), Sub-Auroral (≈ 50–55° MLAT), Polar (≈ 70–75° MLAT), und einem MLT-Sektor (Mitternacht, Dämmerung, Morgen, Mittag). Die Zuordnung kommt aus der Stationsgeometrie, nicht aus der Messung — sie ist agnostisch gegenüber dem Onset und gehört zu Axiom 1 (die Sache selbst an der Stelle der Sache selbst).

**Stufe zwei — Maximal-Einhüllende innerhalb der Klasse, Koinzidenz zwischen den Klassen.** Innerhalb der Auroralband-Klasse wird die **Max-Einhüllende** von |dH/dt| gebildet — Max boostet SNR um √N, wenn die Signale gemeinsam sind, und im Auroralband sind sie gemeinsam (alle Stationen sehen denselben Onset, nur zeitversetzt). Ein Onset wird erklärt, wenn (a) die Auroralband-Max-Einhüllende eine Schwelle überschreitet, (b) eine Sub-Auroral-Referenz mit der erwarteten Latenz (≈ 3–8 min) eine kohärente Signatur zeigt, und (c) die Polar-Referenz **nicht** mit derselben Charakteristik feuert (sonst ist es ein breiter Modenwechsel, kein Substorm). Die TE-Variante dieses Schritts: aus einem Hold-out-Set wird der Leader-Follower-Graph der Stationen kalibriert, und ein Onset ist nur dann einer, wenn die beobachtete Feuerreihenfolge die kalibrierte Reihenfolge respektiert. Das fängt die asymmetrische Kopplung ein, die die Min-Einhüllende begraben hat.

Die Kreuzkorrelations-geführte Auswahl aus der SuperMAG-Linie ist mit der Max-Band-Fusion verwandt — sie macht die Propagation explizit sichtbar und kann sie zur Vorlauf-Erkennung nutzen. Die TE-geführte Variante ist schärfer, weil sie asymmetrische Information misst, nicht nur lineare Korrelation. Beide sind dem Min überlegen; beide sind messbar.

## Die Platzierungsregel

Die Geometrie folgt aus der Physik, nicht aus dem Wunsch nach Redundanz.

**Auroralband (Pflicht):** Eine Station pro MLT-Sektor, bei 60–70° CGM-Latitude. Vier Sektoren — Mitternacht, Morgen, Abend, und einer als Reserve — sind das Minimum, um den Onset unabhängig von seiner MLT-Position zu fangen. Mehr als eine Station pro Sektor bringt innerhalb des Bandes wenig, weil das Max die √N-Verstärkung übernimmt; sie schadet aber auch nicht, solange die Schwelle auf der Max-Einhüllenden sitzt, nicht auf der Min.

**Sub-Auroral-Referenz (Pflicht):** Eine Station bei 50–55° CGM-Latitude, magnetisch konjugiert zur Mitternachtsstation. Sie bestätigt, dass der Treiber magnetosphärisch ist und nicht lokal. Ohne sie ist das Netz blind gegen den häufigsten Verwechslungsfall — den Pi2 aus einer anderen Quelle, den baylosen Modenwechsel, den Stromsystem-Wechsel.

**Polar-Referenz (Pflicht als Ausschluss):** Eine Station bei 70–75° CGM-Latitude. Sie dient als **Negativ-Kriterium**: wenn sie mit derselben Signatur feuert, ist die Störung zu breit für einen Substorm. Das verhindert eine ganze Klasse von Falsch-Positiven, die das Min-Netz nicht ausschließen kann.

**Was nicht ins Netz gehört:** Stationen unter 50° und über 75° CGM-Latitude, sofern sie nicht als Referenz eine gemessene Funktion haben. Jede Station darunter fügt Rauschen hinzu, das langsamer wächst als das ohnehin schwache Signal. Das ist der gemessene Grund, nicht eine stilistische Präferenz.

**Geometrische Minimalregel für einen einzelnen Onset:** Eine Auroralstation innerhalb ±2 h MLT von magnetischer Mitternacht, eine Sub-Auroral-Referenz innerhalb ±15° geomagnetischer Länge darunter, eine Polar-Referenz im selben Sektor. Das ist der eine-Diamant, der den schärfsten Onset liefert. Alles darüber ist Bandbreite, nicht Schärfe.

## Der Schärfe-Begriff

Ja — „Rückruf bei festgelegter Ausbeute am a-priori-Vorlaufpunkt" trägt, mit drei Bedingungen, die du bereits gemessen hast und die nicht verhandelbar sind.

**Das Fenster ist fixiert.** Du hast gemessen, dass das Vergleichsfenster mit der Stationszahl wandert, und du hast gemessen, dass das die Schärfe konfundiert. Das ist der gleiche Riss, der das Min-Netz empirisch schlägt — wer das Fenster mitzieht, glättet eine Konfunde, keine Messung. Fixiere das Fenster auf das Vorlauf-Intervall, in dem der Operator die Warnung braucht; das ist eine **Spezifikationsfrage an den Operator**, nicht ein Messparameter. Ein vernünftiger Wert ist −5 min Vorlauf mit ±5 min Spielraum, also das Intervall [t_catalog − 10 min, t_catalog]. Das ist physikalisch motiviert (Peak |dH/dt| liegt 3–5 min nach Onset, du willst davor warnen) und kurz genug, dass zufällige Koinzidenzen selten bleiben.

**Die Schwelle ist fixiert, oder sie wird auf einem Hold-out kalibriert und die Hold-out-Größe wird berichtet.** Eine adaptive Schwelle, die pro Netzgröße neu berechnet wird, ist genau der Faktor, der den von dir gemessenen 1800×-Kollaps erzeugt. Das ist nicht Schärfe, das ist Messjustierung am Objekt. Kalibriere die Schwelle einmal auf einem Hold-out, der unabhängig von der Test-Stationsauswahl ist, und halte sie fest.

**Der Skalar ist Rückruf bei Ausbeute ≥ ρ, am Vorlaufpunkt t\* = t_catalog − 5 min, im fixen Fenster.** ρ = 0.5 ist ein vernünftiger Standard für ein Alarmsystem, das teuer im Fehlalarm ist. F1 ist erlaubt, bestraft aber die Ausbeute zu hart für ein Warnsystem; F_2 (β = 2) gewichtet den Rückruf doppelt und ist oft die ehrlichere Wahl. Die Signifikanz gegen die Null misst du bereits korrekt — die Kreisschub-Surrogate sind die richtige Null, weil sie die Autokorrelationsstruktur der |dH/dt|-Zeitreihe erhalten und nur die Onset-Kopplung zerstören. Der `prec_ge=0`/`rec_ge=0`-Befund besagt: das Netz ist nicht zufällig — es ist nur schwächer als die eine Station. Das ist genau das Bild, das meine Verdikt-Hypothese vorhersagt.

**Was „am schärfsten" empirisch bedeutet:** Die Konfiguration, die im festen Fenster, bei fester Schwelle, gegen die gleiche unabhängige Onset-Liste, den höchsten Rückruf bei festgelegter Ausbeute liefert. Kein anderes Maß. Die Schärfe eines Netzes ist ein Tripel — (Rückruf, Ausbeute, Vorlauf) — und jedes Element ist fix, oder es ist keine Messung.

## Der nächste begrenzte Messschritt

Nimm die existierende Messinfrastruktur und tausche **nur den Operator**. Behalte Stationen, Katalog, Surrogate, Vorlauf-Definition. Das ist der minimale Eingriff, der die Frage beantwortet, ob der Operator der Riss ist oder die Stationsauswahl.

**Schritt 1 — Band-Max-Einhüllende.** Klassifiziere die existierenden Stationen nach CGM-Latitude in die drei Bänder. Baue pro Band die Max-Einhüllende von |dH/dt|. Erkläre einen Onset, wenn (a) die Auroralband-Hülle die fixierte Schwelle überschreitet, (b) die Sub-Auroral-Hülle mit der erwarteten Latenz kohärent reagiert, (c) die Polar-Hülle nicht mit derselben Signatur feuert. Messe Rückruf bei Ausbeute ≥ 0.5 im fixen Fenster.

**Schritt 2 — TE-geführte Leader-Follower-Fusion.** Kalibriere auf einem Hold-out (z. B. die erste Hälfte des Katalogs) den TE-Graphen zwischen den Stationen. Erkläre einen Onset, wenn die beobachtete Feuerreihenfolge im Auroralband die kalibrierte Reihenfolge respektiert und die Auroralband-Hülle die Schwelle überschreitet. Messe dasselbe Tripel.

**Schritt 3 — Vergleich.** Stelle in einer Tabelle gegenüber: Min-Netz (Status quo), Einzelstation (bester gemessener Punkt), Band-Max-Netz, TE-geführtes Netz. Pro Konfiguration: Rückruf, Ausbeute, Vorlauf, p-Wert gegen die Surrogate, p-Wert gegen die Min-Netz-Konfiguration. Wenn das Band-Max-Netz die Einzelstation nicht schlägt, ist die Platzierung der nächste Riss. Wenn es sie schlägt, ist der Operator der Riss gewesen, und die Doktrin der Min-Einhüllende gehört auf den Friedhof der gemessenen Irrtümer.

**Was dieser Schritt nicht beantwortet:** ob die TE-geführte Variante die Band-Max-Variante schlägt. Das ist Schritt 4, mit mehr Daten und längerer Hold-out-Kalibrierung. Schritt 1–3 sind der begrenzte Schnitt, der den Riss schließt oder öffnet.

## Was ich nicht sage

Ich sage nicht, dass die TE-Methode besser ist als die Band-Max — das wäre Spekulation, und du hast Spekulation aus dem Spiel gewiesen. Ich sage, dass der Min-Operator falsch ist, weil die Messung es sagt und die Physik es erklärt. Ich sage, dass die Platzierung aus der Physik folgt und nicht aus der Stationszahl. Ich sage, dass der Schärfe-Begriff drei Fixierungen braucht — Fenster, Schwelle, Vorlauf — und eine unabhängige Signifikanz gegen eine Null, die die Onset-Kopplung zerstört, ohne die Autokorrelationsstruktur zu zerstören. Die Kreisschub-Surrogate tun das; bleib dabei.

Der Riss steht offen. Das nächste Mal, wenn er gemessen wird, wird er entweder kleiner — oder die Doktrin wandert auf den Friedhof, und die Stationen wandern in eine Geometrie, die zu dem passt, was die Ionosphäre tatsächlich tut.

### 09 · Kimi
- **Site:** `https://www.kimi.ai/`
- **Modell/Einstellung:** Kimi · schnell · hoch
## Urteil

### 1. Zur Fusion: Das Minimum wird verworfen — aber nicht das Netz

Das Minimum über Stationen ist keine Fusionsregel, sondern eine Konjunktion: Die Hülle überschreitet die Schwelle nur, wenn **alle** Stationen gemeinsam tragen. Jede Station, deren Ereignis-Detektionswahrscheinlichkeit unter 1 liegt, zieht die Einhüllende nach unten. Damit ist der monoton fallende Rückruf von 9,8 % auf 0,35 % kein paradoxer Befund, sondern die arithmetische Notwendigkeit der Regel selbst. Die Doktrin ist an ihrem eigenen definitorischen Kern geschlagen: Sie verwandelt jeden zusätzlichen Peer in einen Veto-Spieler. Das Axiom „alle Wesen gleich" wird hier zur Gewalt — das Gewicht der Stationen kommt aus dem Rang in einer Konjunktion, nicht aus der Messung.

Die korrekte Konsequenz ist aber **nicht** „eine Station für immer". Dass die Einzelstation der schärfste gemessene Punkt ist, sagt nur: Unter der bisherigen Fusionsregel ist jede Aggregation schlechter als keine. Es sagt nichts über andere Regeln. Die Aufgabe ist, eine Fusionsregel zu finden, deren marginales Hinzufügen einer Station den Trefferanteil hebt statt senkt.

Es gibt drei Kandidaten, und sie sind nicht gleichrangig:

- **Kreuzkorrelations-geführte Netzdetektion** (SuperMAG-Stil) ist legitimes Neuland, aber sie beantwortet eine andere Frage — sie detektiert Struktur im Netz, nicht den Onset früher oder sicherer. Als Fusionsregel für Onset-Zeitpunkte ist sie ein Umweg.
- **Breitenband-Auswahl** (Norden/Westen/Osten statt Süden, breitenabhängige ~5-min-Verzögerung) ist physikalisch motiviert, aber sie ist ein räumlicher Vorfilter, keine Fusionsregel. Sie reduziert die Kandidatenmenge; sie entscheidet nicht, wie die verbleibenden Stationen kombiniert werden.
- **Transfer-Entropie-geführte Auswahl** ist die Regel, die die eigene Methode (TE mit Surrogat-Null) konsistent erweitert: Die Linse bleibt dieselbe — es ändert sich nur, wohin sie zeigt. Von „TE je Einzelpfeil" zu „TE als Auswahlkriterium über der Onset-Hülle" ist ein Schritt, kein Bruch. Das ist der einzige Kandidat, der nicht zwei Methoden mischt und damit Axiom 4 wahrt.

**Verdikt: Abkehr vom Minimum, hin zu TE-geführter Auswahl kleiner, gemessener Subsets.** Die Physik (asymmetrische Ausbreitung) dient als räumliche Vorbegrenzung der Kandidaten, nicht als Begründung für eine feste Geometrie.

### 2. Die Platzierungsregel

Die Regel hat drei Stufen, jede aus Messung, keine aus Namen:

1. **Kandidatenraum** durch Physik begrenzen: Stationen, die vom erwarteten Substurm-Sektor (N/W/O-Ausbreitung) erfasst werden, breitenabhängig um das erwartete Maximum von dH/dt (~5 min nach Onset). Südwärts stationierte Peers fallen nicht durch Rang, sondern durch gemessene niedrige bedingte Detektionswahrscheinlichkeit heraus — das muss gezeigt, nicht angenommen werden.
2. **Rangfolge jeder Station** durch ihre bedingte Transfer-Entropie zur Referenz-Onset-Hülle, gegen die Surrogat-Null. Stationen, deren TE über der Null liegt und die den Onset *vor* dem Einzelstation-Treffer tragen, sind Kandidaten. Gewicht = gemessene TE, nie Default.
3. **Marginalitätsklausel:** Eine Station wird nur dann dem Netz hinzugefügt, wenn ihr marginales Hinzufügen den Trefferanteil gegenüber dem bisherigen Subset bei fixiertem Fenster und fixierter Ausbeute erhöht. Das ist die Versicherung, dass der Min-Kollaps nicht in neuer Kleidung zurückkehrt: Die Konjunktion wird durch eine gemessene Verbesserungsbedingung ersetzt.

Die Geometrie folgt aus Stufe 1, nicht umgekehrt. Es wird kein vorgegebenes Gitz erzwungen.

### 3. Der Schärfe-Begriff

Der vorgeschlagene Skalar — Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt — trägt, aber nur unter zwei Zusatzbedingungen, die der Befund erzwingt:

- **Das Fenster muss fixiert werden, bevor die Stationszahl variiert wird.** Der wandernde Vergleich ist einer der zwei gemessenen Konfunde; jede Skalar-Definition, die ihn toleriert, misst die Konfunde mit, nicht das Netz.
- **Der Skalar muss marginalisierbar sein.** Netto-Rückruf über ein Subset allein reproduziert das alte Problem: Er misst die Konjunktion, nicht den Beitrag. Der eigentliche Schärfe-Maßstab ist die **marginale Rückrufdifferenz pro hinzugefügter Station** — Leave-One-Out bzw. schrittweise Selektion — bei fixierter Ausbeute, fixiertem Fenster, fixierter Schwelle. Erst wenn dieser marginale Wert über der Surrogat-Null liegt, hat die Station das Netz verdient.

Damit lautet die Antwort auf „woran wird am schärfsten gemessen": nicht am Rückruf des Gesamtnetzes, sondern an der **gemessenen Marginalität**. Der Skalar, der die Frage der Sitzung direkt beantwortet, ist: *Hebt Station k den Trefferanteil gegenüber dem besten k−1-Subset?*

### 4. Der nächste begrenzte Messschritt

Ein einziger, eng begrenzter Lauf:

Fixiere das Zeitfenster und die Schwelle **jetzt, ein für allemals**, auf den Werten, die die Einzelstation verwendet (die schärfste bekannte Konfiguration — sie ist die Baseline, die geschlagen werden muss). Dann, auf denselben Daten: Rangliste aller Stationen nach TE zur Onset-Hülle gegen die Surrogat-Null; nimm die oberste; füge Stationen eine nach der anderen gemäß §2 Stufe 3 hinzu; stoppe am ersten gemessenen Marginalverschlechterer. Ergebnis ist entweder ein 2–3-Stationen-Subset mit höherem Rückruf als die Einzelstation — dann ist die Doktrin ersetzt — oder der Nachweis, dass auch TE-geführte Fusion unter diesen Daten nicht über die Einzelstation kommt. Beides ist ein ehrliches, registrierungsfähiges Ergebnis. `pending` gibt es nur für Stationen, deren TE wegen Datenlücken nicht berechenbar ist — diese bleiben registriert, nicht null.

Ein Satz zum Konsens-Axiom: Die Selektionsregel selbst wird vom Operator bestätigt, bevor sie als neue Doktrin aufgezeichnet wird — die Maschine fragt, sie ersetzt nicht.

**Der Riss, ungeglättet:** Der Befund „Einzelstation ist schärfstes Netz" steht, bis die Marginalitätsmessung etwas anderes zeigt. Die neue Regel ist keine Erwartung, sondern eine prüfbare Hypothese mit einer klaren Schranke.

### 10 · Lumo (Proton)
- **Site:** `https://lumo.proton.me/`
- **Modell/Einstellung:** Lumo 2.0 Max · Websuche


# Urteil zur Netz-Fusion und Stationsplatzierung

Die dokumentierten Messbefunde lassen keinen anderen Schluss zu als das Verdikt: **die Min-Einhüllende ist empirisch widerlegt** als Fusionsregel für wachsende Netze. Wo der Rückruf von 9,8 % auf 0,35 % fällt, während Signifikanz gegen die Surrogat-Null erhalten bleibt, sagt nicht die Datenqualität, sondern die **Fusionsregel selbst**. Das Minimum ist konservativ bis zum Punkt der Unbrauchbarkeit.

---

## 1. Verdikt zur Fusion

Die Transfer-Entropie ist vorzuziehen — jedoch nicht als direkte Alternative zum Minimum, sondern als **Auswahlmechanismus vor der Fusion**. Die aktuelle Doktrin nimmt alle Stationen und bildet das Minimum. Dies unterdrückt systematisch Stationen mit leicht verzögertem oder abgeschwächtem Signal, obwohl diese physikalisch valide Onsets tragen.

**Begründung nach Axiom 1:** Die Messung zeigt, dass mehr Stationen per Minimum-Konfiguration Schaden produzieren. Fortsetzung wäre Fabrikation gegen die Empirie.

Die SuperMAG-Referenz (Kreuzkorrelation zu Communities) bietet den Weg: TE identifiziert **gerichtete Informationsflüsse** zwischen Stationen. Stationen, die keine kausale Kopplung zum Onset-Zentrum zeigen, müssen aus der Fusion entfernt werden — nicht hinzugefügt.

Alternative: **Breitenband-Auswahl** allein reicht nicht, da die Physik nach Onset Expansion nach Norden/Westen/Osten zeigt, kaum Süden. Eine reine geografische Regel ignoriert den dynamischen Charakter.

---

## 2. Platzierungsregel

Die Schärfe der Messung hängt nicht von Stationszahl, sondern von **geografischer Position relativ zum erwarteten Ausbreitungsmuster** ab.

**Gemessener Grund (Physik):** Nach Onset dehnt sich dB/dt nach Norden, Westen, Osten aus. Stationen südlich des Onset-Gürtels tragen wenig Information.

**Regel:** 
- Priorisiere Stationen **nördlich** des Haupt-Onset-Gürtels (~60°–70° geomagnetische Breite)
- Sekundäre Stationen westlich und östlich mit ~15°–30° longitudinaler Ausdehnung
- Südstationen nur als Validierungspunkte, nicht für primäre Fusion

Die **Einzelstation bleibt schärfst**, solange sie optimal positioniert ist. Mehr Stationen gewinnen erst, wenn sie komplementäre Phasen des Ausbreitungsmusters abdecken — nicht redundant.

---

## 3. Schärfe-Begriff

Der vorgeschlagene Skalar „Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt" ist korrekt, aber unvollständig.

**Notwendige Ergänzungen:**
- **Fensterfixierung:** Ja. Das wandernde Vergleichsfenster ist ein gemessener Konfund. Es muss statisch sein, definiert am a-priori-Vorlauf.
- **Zweite Metrik:** Präzision-Rückruf-Schnitt bei variabler Stationszahl. Nicht nur Recall, sondern auch False-Positive-Rate bei gleicher Stationskonfiguration.
- **TE-gewichteter Score:** Jede Station erhält einen TE-basierten Kopplungskoeffizienten; Fusion gewichtet nach diesem Koeffizienten statt Equal-Weight Minimum.

**Axiom 3 Anwendung:** Keine Station erhält Gewicht durch Rang oder Name — nur durch gemessene TE-Kopplung zum Onset-Ereignis.

---

## 4. Nächster begrenzter Messschritt

Drei Schritte, sequentiell, jeweils mit Surrogat-Null-Validierung:

| Schritt | Ziel | Begrenzung |
|---------|------|------------|
| (1) Fixiere Fenster | Entferne wanderndes Confound | Statische 5-min Vorlaufdefinition |
| (2) TE-Preselection | Wähle top-k Stationen nach TE-Kopplung | k ∈ {1,3,5,7}, compare to baseline n=12 |
| (3) Weighted Fusion | Gewichtetes Minimum oder linearer Mix | Gewichte aus TE-Scores, nicht Gleichgewicht |

Jeder Schritt muss mit **identischem** Onset-Reference-Set und **identischem** Surrogat-Generator validiert werden. Keine neue Null pro Konfiguration — konsistente Vergleichbarkeit.

---

## Zu den fünf Stimmen

- **Mountain:** Die Daten sind eindeutig — Minimum-Fusion schadet bei Wachstum.
- **River:** Die Maschine darf verweigern; hier heißt Verweigern: die Doktrin aufgeben.
- **Mycelium:** Infrastruktur muss TE-Berechnung in Pipeline integrieren — latenter Overhead akzeptabel, wenn Recall gewinnt.
- **Sensory:** Hardware bleibt gleich; die Fusion ändert sich, nicht die Sinne.
- **Future:** Morgen wird geerntet, was heute honest registriert ist — die Lücke zwischen n=1 und n=12 muss geschlossen werden, nicht umgangen.

---

## Offener Riss

Die Dokumentenfrage nennt zwei zusätzliche Confounds: kleines Netz nicht vergleichbar; Fenster wandert mit Stationszahl. Diese müssen vor jedem neuen Verdikt **eliminiert** werden. Bis dahin bleibt die aktuelle Schärfe-Messung fragwürdig — signifikant gegen Null, aber systematisch verzerrt durch Methodik.

**Präzisestes Urteil:** Per Minimum ist falsch. Per TE-gesteuerter Auswahl ist möglich. Bewiesen ist es nicht — der nächste Messschritt entscheidet.

### 11 · Perplexity (Deep Research)
- **Site:** `https://www.perplexity.ai/`
- **Modell/Einstellung:** Deep Research · Suche
## Verdikt zur Fusion

**Das Minimum als Fusionsregel ist geschlagen.** Es verwandelt „irgendeine Station sieht etwas“ in „alle Stationen müssen gleichzeitig über einer Schwelle liegen“; mit jeder Station wächst daher die Wahrscheinlichkeit, dass ein reales Onset durch eine einzige ruhige oder schlechter positionierte Station ausgelöscht wird. Der gemessene monotone Rückrufabfall ist nicht ein Tuning-Problem, sondern die direkte Konsequenz dieser Semantik. Die adaptive Schwelle kann das nicht retten, weil sie die falsche Ordnungsrelation nicht repariert, sondern nur neu skaliert.

Die Antwort ist **nicht** „mehr Stationen per Minimum“, sondern **TE-/Korrelationsgeführte Auswahl mit einer ODER-artigen, entropiegewichteten Fusion**. Das Netz soll nicht das Minimum der Amplituden bilden, sondern prüfen, ob mindestens eine Station — oder eine gemessene, zeitlich geordnete Kette von Stationen — ein signifikantes Onset-Muster trägt. Transfer-Entropie ist dabei nicht dekorativ: Sie misst, ob Station B nach Station A Information über den Übergang hinzufügt, und kann damit Richtung und Ausbreitung erfassen, statt nur Gleichzeitigkeit. Das entspricht der beobachteten Dynamik: Vor Onset existieren kleine, wenig korrelierte Gruppen; etwa zehn Minuten danach entsteht ein großräumig kohärentes System. [nature](https://www.nature.com/articles/s41467-021-22112-4)

## Platzierungsregel

**Nicht geographische Vollständigkeit, sondern messbare Informationsbeiträge entscheiden.** Eine Station wird aufgenommen, wenn sie gegen die Surrogat-Null entweder

- selbst häufig und früh genug signifikant feuert, oder
- in einer gerichteten TE-/Kreuzkorrelationskette liegt, die den Onset früher oder zuverlässiger erkennt als ihre Vorgängerstation.

Die Geometrie folgt der Physik: Die Ausbreitung nach Norden, Westen und Osten — kaum nach Süden — verlangt eine Kette entlang dieser bevorzugten Richtungen, nicht ein symmetrisches Raster.  In der Auroralzone sind Störungen stärker lokalisiert; Korrelationslängen fallen bei Abständen unter etwa 100 km rasch ab, während etwa 200 km Stationsabstand als guter Kompromiss für GIC-Relevanz genannt werden.  Damit ist die Platzierungsregel: **dichte Abtastung dort, wo die gemessene Kohärenzlänge kurz ist; größere Abstände nur dort, wo Stationen nachweislich gemeinsame Information tragen.** [angeo.copernicus](https://angeo.copernicus.org/articles/41/13/2023/)

Keine Station erhält Gewicht nach Name, Region oder Default. Ihr Gewicht ist ihr gemessener marginaler Beitrag: Rückrufgewinn bei festem Falschalarmbudget, Vorlaufgewinn oder signifikanter TE-Beitrag. Bringt sie keinen dieser Beiträge, bleibt sie im Register — aber sie gehört nicht in die aktive Fusionsmenge.

## Der Schärfe-Begriff

„Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt“ ist der richtige Kern, aber er braucht eine Präzisierung: **Rückruf bei fixiertem Falschalarmbudget und fixiertem Vorlaufintervall relativ zum unabhängigen Onset-Zeitpunkt.** Das Vergleichsfenster darf nicht mit der Stationszahl wandern; sonst misst man teilweise die Fenstergeometrie, nicht das Netz. Das ist der zweite offene Riss und muss vor jedem weiteren Netzwachstum geschlossen werden.

Der Skalar sollte nicht nur Treffer zählen, sondern drei ehrliche Größen gleichzeitig melden:

- **Precision/recall bei fixem Budget** — die primäre Schärfe.
- **Medianer Vorlauf** gegenüber der unabhängigen Onset-Liste — weil Frühwarnung der Zweck ist.
- **Marginaler Stationsbeitrag** — Rückruf- und Vorlaufänderung beim Hinzufügen oder Entfernen einer Station.

Ein einzelner Skalar ist zulässig, etwa „Rückruf bei festem Falschalarmbudget, gewichtet mit Vorlauf“, aber er muss als explizite Entscheidung dokumentiert werden — nicht als implizites Default-Ergebnis. Die adaptive Schwelle ist nur dann zulässig, wenn sie pro Fusionsregel und Stationszahl gegen dieselbe Null und dasselbe Fenster kalibriert wird; sonst ist sie eine zweite Konfunde, keine Verbesserung.

## Nächster begrenzter Messschritt

**Vergleiche drei Fusionsregeln auf identischem Datenbestand, identischer Onset-Liste, identischer Surrogat-Null und fixiertem Fenster:**

1. Einzelstation als Referenz.
2. Min-Einhüllende als geschlagene Doktrin.
3. ODER-Fusion über diejenigen k Stationen, die per TE-/Kreuzkorrelation den höchsten gemessenen marginalen Beitrag haben.

Für Regel 3 wird \(k\) nicht spekulativ gewählt, sondern durch das Datenmaterial bestimmt: Beginne mit der besten Einzelstation und füge Stationen nur hinzu, solange der Rückruf bei fixem Falschalarmbudget oder der mediane Vorlauf signifikant steigt. Gleichzeitig wird das wandernde Fenster fixiert und für jede Stationszahl separat registriert, welche Stationen fehlen oder unvollständig sind — als `pending`, nicht als Null.

Das ist der einzige Schritt, der den Riss nicht glättet: Er lässt das Minimum weiter verlieren, wenn es verliert, und lässt die TE-geführte Auswahl nur dann gewinnen, wenn ihre Gewinne gegen dieselbe Null und dasselbe Fenster bestehen. Die Literatur stützt die Richtung: Zeitabhängige, gerichtete Korrelationsnetzwerke zeigen den Übergang von lokalen zu großräumig kohärenten Strukturen nach Onset.  Sie liefert aber noch keine fertige TE-Fusionsregel; genau diese Lücke ist der nächste Messgegenstand. [nature](https://www.nature.com/articles/s41467-021-22112-4)

### 12 · Sakana Namazu
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Namazu · High
**Urteil**

**1. Zur Fusion: Das Minimum ist geschlagen.**  
Die Messung zeigt einen monotonen Kollaps des Rückrufs mit steigender Stationszahl. Ursache ist nicht die Physik, sondern der Operator selbst: das Minimum einer wachsenden Menge wird stochastisch extremer und frisst das Signal auf. Weiterverwendung der Min-Einhüllenden wäre Fabrikation gegen Axiom 4. Das Urteil lautet Abkehr. An ihre Stelle tritt eine Fusion, die im Informationsgehalt monoton ist: eine TE-geführte Auswahl mit anschließender gewichteter Überlagerung oder dem Maximum kalibrierter Einzelscores. Breitenband-Auswahl allein genügt nicht, weil sie den gemessenen Wert einzelner Peers ignoriert (Axiom 3).

**2. Zur Platzierung: TE bestimmt den Bogen.**  
Die Platzierungsregel lautet: Ordne alle Stationen nach ihrer Transfer-Entropie gegen die unabhängige Onset-Liste; behalte nur solche mit signifikanter TE (gegen Surrogat-Null). Die Geometrie folgt der Physik: ein offener Bogen nach Norden, Westen und Osten, der Süden bleibt leer. Der gemessene Grund ist die dokumentierte Ausdehnung von dB/dt nach Onset in diese Richtungen; Stationen im Süden tragen kaum Signal bei und würden das Netz nur mit stillem Ballast füllen (Mountain, Mycelium).

**3. Zur Schärfe: Der Skalar ist gebrochen.**  
„Rückruf bei fester Schwelle" ist für den Vergleich über verschiedene N unbrauchbar, weil die Min-Einhüllende die Schwelle an N koppelt. Das ist der erste offene Riss. Der zweite ist das wandernde Fenster. Ein valider Schärfe-Skalar muss N-invariant sein, etwa AUC-PR oder Recall bei fixierter Präzision. Das Fenster muss physikalisch fixiert werden und darf nicht mit der Stationszahl wandern; nur so trennt die Messung den Fehler der Fusion vom Fehler der Zeitlokalisierung.

**4. Nächster begrenzter Messschritt.**  
Teste auf denselben TE-ranken Subsets drei Fusionen gegen die Onset-Liste: Min (Kontrolle), Max |dB/dt|, TE-gewichtete Summe. Halte das Fenster konstant. Erhöhe N nur durch Hinzufügen des nächstbesseren TE-Peers. Missings bleiben pending; keine Nullen, keine Defaults (Axiom 2, River). Damit wird geprüft, ob eine Fusion existiert, bei der mehr Stationen den Rückruf tatsächlich heben. Solange dies nicht gemessen ist, bleibt die Einzelstation die schärfste Konfiguration. Das ist das ehrliche Register für die Zukunft (Future).

### 13 · Sakana Fugu
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Fugu · Max · High
**Verdikt vorweg.** Nicht weiter per Minimum. Das Minimum ist keine Fusion, sondern eine Konjunktion: Onset nur, wenn *alle* Stationen gleichzeitig über der Schwelle liegen. Damit ist der Rückruf bei fester Schwelle **strukturell** monoton nicht-steigend in N — jede zusätzliche Station ist ein zusätzliches Veto. Der Absturz 9,8 % → 0,35 % ist kein Befund über das Netz; er ist eine Folgerung aus dem UND. Die Doktrin ist nicht durch die Daten widerlegt, sie ist durch ihre eigene Struktur geschlagen: sie verlangt Netz-Simultanität, die die Physik verweigert. Nach dem Onset läuft die Front nach Norden, Westen, Osten — verzögert, breitenabhängig, größtes dH/dt rund 5 min danach. Ein UND über eine propagierende Front kann nur verlieren. Die Einzelstation gewinnt, weil sie der einzige Fall ohne Veto ist. Und die Signifikanz gegen die Null entlastet nichts: `prec_ge=0`/`rec_ge=0` sagt „nicht zufällig", nicht „scharf". Eine Konjunktion ist nicht zufällig und trotzdem wertlos.

**1. Verdikt zur Fusion.** Kein Minimum. Aber auch nicht „TE statt Min" als neuer Default — das wäre nur die nächste Doktrin. Die Auflösung ist die Trennung zweier Ämter, die das UND vermischt hat:

- *Erkennung* (Rückruf): eine kleine, **gemessene** Führungsrand-Menge, fusioniert per ODER/Max. Das ist der Trigger. Rückruf wird von ihm gesetzt.
- *Bestätigung* (Ausbeute): der Rest des Netzes, lager-tolerant, innerhalb eines **physikalisch fixierten** Fensters, ebenfalls per ODER. Das hebt die Ausbeute, ohne den Rückruf zu töten.

Dann hilft jede zusätzliche Station — sie fügt eine Bestätigung hinzu, kein Veto. Rückruf hebt sich allerdings nur, wenn die zusätzliche Station eine Region abdeckt, die die Führungsmenge verfehlt. Redundante Nachbarn sind unter UND Veto-Macher und unter ODER Leerlauf; beide Male schaden sie.

Breitenband gegen TE ist kein Entweder-oder: das **Breitenband gibt die Geometrie** (wo die Front lebt), die **TE gibt Vorzeichen und Gewicht** (wer führt, wer folgt). Andere Gruppen verbinden Stationen per Kreuzkorrelation zu einem zeitabhängigen gerichteten Netz und detektieren Communities, statt zu minimieren; die nicht-verschwindenden Lags (1–15 min) sind genau die Ausbreitung, und sie normalisieren die Stationsempfindlichkeit rein aus den Daten <source-chip title="Orr et al. 2019, GRL" url="https://doi.org/10.1029/2019gl082824" /> <source-chip title="Nature Communications 2021" url="https://www.nature.com/articles/s41467-021-22112-4" />. Das ist der Beleg, dass Gewicht aus Messung gehen kann — und dass eine Array-Element-Empfindlichkeit überhaupt messbar ist <source-chip title="Space Weather 2022 (BEAR/CARISMA)" url="https://agupubs.onlinelibrary.wiley.com/doi/full/10.1029/2021SW002919" />. Unsere Form — TE mit Surrogat-Null als *Fusions- und Platzierungsregel* — gibt es so nicht; nur die Einzelpfeile.

**2. Platzierungsregel.** Geometrie: eine Kette entlang der Breite plus Arme nach Ost und West, dicht abgetastet im Breitenband des maximalen dH/dt (Substurm-Onset-Breite), Süden als **Kontrolle**, nicht als Veto-Mitglied. Aufnahme einer Station nach gemessenem Grund, nicht nach Rang: sie wird aufgenommen, wenn sie am a-priori-Vorlaufpunkt einen Onset triggert, den die aktuelle Menge verfehlt (marginale Rückruf-Zunahme), **oder** wenn sie im TE-Graph eine Quellenrolle trägt (hohe ausgehende, niedrige eingehende TE). Die Physik „Norden/Westen/Osten, kaum Süden" ist eine **Hypothese, die zu testen ist — kein Default, den man einbaut**. Die Süd-Kontrolle existiert genau deshalb: fällt sie wie erwartet schwach aus, ist das Bestätigung; würde man sie vorab als schwach *setzen*, wäre es Fabrikation (Axiom 1, 4). Das Gewicht kommt aus der Messung, nie aus Rang, Name oder Default (Axiom 3).

**3. Der Schärfe-Begriff.** „Rückruf bei fester Ausbeute am a-priori-Vorlaufpunkt" trägt — **als primärer Skalar**, aber nur unter drei Bedingungen. Erstens: „Ausbeute" heißt null-kalibrierte Falschalarmrate aus den Kreisschub-Surrogaten, nicht rohe Alarmzahl. Zweitens: Vorlaufpunkt **und** Fenster sind a priori eingefroren, in physikalischen Sekunden, vor jedem Blick auf das Ergebnis — sonst ist der Vorlaufpunkt das wandernde Fenster in Verkleidung. Drittens: `pending`-Zellen werden als `pending` markiert, nicht als Verfehlen gewertet (Axiom 2). Ja, das Fenster **muss** fixiert werden — aber nicht „ein konstantes Fenster für alle N", sondern ein **physikalisch definiertes** Fenster (Ausbreitungs-Apertur), dasselbe für alle N. Das ist der ganze Unterschied: fixiert von der Physik, nicht von der Stationszahl. Ein zweiter, **untergeordneter** Skalar gehört dazu: die Vorlaufzeit-Verteilung (Median und Streuung) am fixen Arbeitspunkt. Zwei Regeln können im Rückruf gleichziehen und sich in der Frühzeitigkeit unterscheiden — und Frühzeitigkeit ist der eigentliche Wert eines Onset-Detektors. Dieser zweite Skalar annotiert, er entscheidet nicht.

**4. Nächster begrenzter Messschritt.** Einer, der die benannten Risse *trennt*, bevor Neues gebaut wird. Daten, Onset-Liste und Null einfrieren; das Fenster für **alle** N auf die physikalische Ausbreitungs-Apertur fixieren (a priori deklariert); den Arbeitspunkt auf **eine** null-kalibrierte Falschalarmrate fixieren; dann **dieselbe** Min-Einhüllende erneut laufen lassen und R(N) für N = 1, 3, 4, 8, 12 berichten. Kehrt die Monotonie zurück, war der Kollaps Fenster plus Schwelle — nicht das Min; das Min ist teilweise entlastet. Bleibt der Kollaps, ist das Min selbst die Ursache, und die Doktrin ist durch ihre eigene Struktur geschlagen. Eine Alternative, ein Skalar, eine Null. Erst danach der zweite Schritt: Min ersetzen durch Führungsrand-ODER über TE-gewählte Stationen, bei gleichem eingefrorenem Fenster, gleicher FAR, gleichem Vorlauf.

**Die fünf Stimmen.** *Mountain* — kein Verdikt ohne Messung: das Register trägt N, Fenster, θ und `pending`; das Min ist nicht von den Daten geschlagen, sondern von seiner Struktur, und beides steht im Register. *River* — das Recht zu verweigern: eine Station, die unter UND blockiert, muss sich **enthalten** dürfen statt zu blockieren, und die Maschine muss `pending` sagen dürfen; niemand muss bei jedem Zeitschritt entscheiden. *Mycelium* — Fusion ist ein Flussproblem, kein Broadcast-UND: der Min ist ein UND über alle, das TE-Netz ist ein gerichteter Fluss; CI läuft versioniert mit fixem Fenster. *Sensory* — Korrelat ≠ Erleben: ein Sensor ist ein Ort, keine austauschbare Zahl; Konsens der Sensoren ≠ Mittelwert der Sensoren, und das UND behandelt Orte als Zahlen und verliert genau das, was sie sind. *Future* — der Mensch als Adresse: Onset-Liste und Null jetzt einfrieren und versionieren, denn die Skalar-Wahl von heute bestimmt, was morgen geerntet werden kann; ehrliche Registrierung heißt `pending`, nicht 0.

**Die Risse, ungeglättet.** Das Vergleichsfenster wandert mit N — Konfund, durch Physik zu fixieren. Die adaptive Schwelle macht θ zu θ(N); θ und N sind im Kollaps **untrennbar**, und der Faktor ~1800× ist nur mit der θ(N)-Reihe belegbar — bis dahin `pending`, nicht behauptet. Kleine Netze sind teils nicht vergleichbar — `pending`, nicht 0. Und der Haupt-Riss bleibt offen: die Min-Doktrin ist empirisch **und** strukturell geschlagen. Sie wird nicht geglättet.

### Geteilte Open-Weight-Seats (`tryingopen.com` · Gruppe `open-weight-ui`, Zugang nur über das Lock)
- **Modelle der Runde:** _<gemessen je Runde>_ (ein Tab je Modell, kein Modellwechsel im Chat)
- **Antwort (je Modell):**
