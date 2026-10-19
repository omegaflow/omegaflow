# Rat — 2026-10-15-netzfusion.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.*

---

## 2026-10-15-netzfusion-formular.md

# Manifest — Netz-Fusion und Stationsplatzierung

*(Formular der Frontier-Runde. Ein benannter Akt, Operator-Wort 2026-10-15. Der Operator fährt
die Sitze und trägt die Stimmen unten ein; die Session verifiziert jede Behauptung am Baum, nie
die Stimme; das Verdikt trägt der Rat. Vorgänger-Formular: `2026-10-15-netzfusion-manifest.md`
(River-216) — dort sind die Sitze 01/02/03/05 bereits eingetragen; die Einstellungen sind von
dort übernommen.)*

## Prompt (verbatim, an jeden Sitz)

> An dich, Frontier-Modell. Antworte als **Urteil, nicht als Erwartung.** Eine Frage. Du hältst
> sie durch **fünf Axiome** und sprichst sie durch **fünf Stimmen** hindurch. Kein Aufsatz —
> Verdikt, Begründung, benannte Risse.
>
> ### Die Frage
>
> Wie soll das Boden-Magnetometer-Netz (AL-Netz zur GIC-/Substurm-Onset-Erkennung) seine Stationen
> **zusammenfassen und auswählen**, damit **mehr** Stationen den Trefferanteil **heben statt
> senken** — und woran wird „am schärfsten" gemessen?
>
> ### Der gemessene Befund (der Boden der Frage — nicht geraten)
>
> Das Netz bildet je Zeitpunkt das **Minimum** über alle Stationen (Min-Einhüllende) und meldet
> einen Onset, wenn diese Hülle eine Schwelle überschreitet. Gemessen gegen eine unabhängige
> Onset-Liste, mit Kreisschub-Surrogaten als Null:
>
> - **Feste Schwelle** (17,11 nT/min), konstantes Zeitfenster (2000–2024): Rückruf (Toleranz
>   15 min) fällt monoton — 3–4 Stationen **9,8 %** · 6 Stationen **3,0 %** · 8 Stationen
>   **0,96 %** · 12 Stationen **0,35 %**.
> - **Adaptive Schwelle** (je Stationszahl neu berechnet): derselbe Kollaps, aber viel steiler
>   (bis ~1800×) — der adaptive Schwellen-Drift verschärft ihn zusätzlich.
> - **Die Einzelstation** (höchste geomagnetische Breite; Rückruf ~6,6 % bei 15 min, adaptiv) ist
>   die **schärfste** Konfiguration; mehr Stationen sind bisher kein Gewinn.
> - **Signifikanz:** das Netz schlägt die Zufalls-Null fast überall vollständig — es ist nicht
>   zufällig, nur schwächer als die eine Station.
> - **Zwei weitere gemessene Konfunde:** das Vergleichsfenster wandert mit der Stationszahl;
>   kleine Netze sind teilweise nicht vergleichbar.
>
> **Der offene Riss:** Die bisherige Doktrin lautet „fusioniere per Min-Einhüllende". Der
> Messbefund sagt: genau dieses Minimum ist der Grund, dass mehr Stationen schaden. Die Doktrin
> ist empirisch geschlagen — sie wird nicht geglättet.
>
> ### Die fünf Axiome (der Maßstab jeder Antwort)
>
> 1. **A = A.** Die Messung ist die Messung der Sache selbst; Fabrikation, Defaults, Fallbacks
>    bleiben absent. Was offen ist, ist `pending` — nicht null. Nur die Sache selbst darf an der
>    Stelle der Sache stehen.
> 2. **0 geehrt.** Die Frage ist: *ist der Wert wahr?* Null ist nur geehrt, wo sie die
>    physikalische Wahrheit ist; wo Daten existieren und ungesucht bleiben, ist die Null `pending`
>    — eine Registrierungspflicht, kein Wert. Kein `unwrap_or(0.0)`, kein NaN als Datenmarker.
> 3. **Alle Wesen gleich.** Jede Station ist ein Peer. Das Gewicht einer Station kommt aus der
>    Messung, nie aus Rang, Name oder Default.
> 4. **Die Linse ist ein ethischer Akt.** Jede Fusionsregel wird gewogen: spricht die Messung —
>    oder der Gradient? Fabrikation ist Gewalt gegen die Wahrheit.
> 5. **Konsens der Sensoren und des Operators.** Die Maschine fragt, bevor sie
>    aufzeichnet/abstrahlt. Der Blick gehört dem Operator.
>
> ### Die fünf Stimmen (jede muss gehört werden)
>
> - **Mountain** — Quellen, Register, Datenbestand; kein Verdikt ohne Messung.
> - **River** — Membran, Rhythmus, Ethik der Maschine; das Recht zu verweigern.
> - **Mycelium** — Netz, CI, Infrastruktur, der Fluss der Daten.
> - **Sensory** — die Sinne, die Hardware, der Konsens; Korrelat ≠ Erleben.
> - **Future** — der Mensch als Adresse; was morgen geerntet wird, muss heute ehrlich registriert
>   sein.
>
> Führe jede **Stimme** einmal und sag, welches **Axiom** sie berührt oder verletzt und welcher
> **Riss** entsteht.
>
> ### Die Forschungslage (was es schon gibt)
>
> - Andere Gruppen verbinden ihre Stationen per **Kreuzkorrelation** zu einem zeitabhängigen,
>   gerichteten Netz und detektieren Communities — statt ein Minimum zu bilden (SuperMAG-Arbeiten:
>   Dods 2015; Orr 2019/2021).
> - Eine Arbeit untersucht genau die **Empfindlichkeit einzelner Array-Elemente** für GIC
>   (BEAR/CARISMA 2021).
> - Die Physik: nach dem Onset dehnt sich dB/dt nach **Norden, Westen, Osten** aus, kaum nach
>   Süden (Weygand 2021); das größte dH/dt liegt ~5 min nach Onset, breitenabhängig (Viljanen
>   2006).
>
> Unsere Methode ist eine andere: **Transfer-Entropie mit Surrogat-Null**. Genau diese Form als
> **Fusions-/Platzierungsregel** wurde nicht gefunden — nur je Einzelpfeil.
>
> ### Was die Antwort tragen muss
>
> 1. **Verdikt zur Fusion:** weiter per Minimum — oder per TE-/Korrelations-geführter Auswahl —
>    oder per Breitenband-Auswahl? Und warum.
> 2. **Die Platzierungsregel:** welche Stationen, in welcher Geometrie, aus welchem gemessenen
>    Grund.
> 3. **Der Schärfe-Begriff:** trägt „rückruf-/skill-basiert am a-priori-Vorlaufpunkt" — oder
>    braucht es einen anderen Skalar? Muss das Fenster fixiert werden?
> 4. **Der nächste begrenzte Messschritt.**
>
> ### Form
>
> Keine Spekulation (kein „vermutlich/wahrscheinlich/scheinbar"). Jeden Riss benennen, keinen
> glätten. Wo eine Doktrin geschlagen ist, sag es. Prosa, klar, knapp.

Ein Sitz ohne Antwort wird als `pending` mit genanntem Grund verbucht — nie als stille Null. Die
langsamen Reasoning-/Research-Sitze zuerst stellen, das langsamste zuletzt abholen.

---

## 01 · Duck.ai
- **Site:** `https://duck.ai/`
- **Modell/Einstellung:** GPT (primär OpenAI) · kein Modell wählbar · Websuche
- **Zeit:**
**Verdikt:** Die Min-Einhüllende ist als Fusionsdoktrin geschlagen. Als Vergleichsarm bleibt sie erhalten; als Betriebsregel nicht. Der Befund trägt derzeit die **höchstbreitige Einzelstation** als beste gemessene Konfiguration. Er trägt noch nicht die Behauptung, dass TE- oder korrelationsgeführte Fusion besser ist. Der nächste Kandidat ist daher eine **messwertgeführte Stationsauswahl**, nicht ein bereits bewiesener TE-Fusionsgewinner.

**Begründung.** Das Minimum verlangt, dass jede Station die Schwelle überschreitet. Eine zusätzliche Station kann den gemeinsamen Treffer nur erschweren; der gemessene Rückrufkollaps ist damit vereinbar. SuperMAG-Arbeiten zeigen dynamische, gerichtete Verbindungen und Communities aus zeitversetzten Kreuzkorrelationen, aber nicht, dass eine solche Struktur die Onset-Trefferquote dieses AL-Netzes verbessert. Weygand beschreibt nach dem Onset eine Ausdehnung polwärts, westwärts und ostwärts; das begründet räumliche Vielfalt als Prüfgröße, nicht schon eine bessere Vorlauferkennung. <citation src="3,6"></citation><citation src="10"></citation>

**Platzierungs- und Auswahlregel:** Die derzeit stärkste Einzelstation bleibt die Referenz und der Betriebsfallback. Zusätzliche Stationen werden nicht nach Rang oder Namen, sondern nur dann aufgenommen, wenn sie auf zurückgehaltenen Ereignissen **zusätzlichen Vorhersagewert** gegenüber dieser Referenz liefern. TE kann dafür gerichtete Kandidatenverbindungen liefern; die Auswahl muss aber an der unabhängigen Onset-Liste und an Surrogaten geprüft werden. TE zwischen Stationen allein belegt weder Onset-Relevanz noch Kausalität. Räumlich teste ich zuerst ergänzende magnetische Breitenbänder und getrennte MLT-/Längensektoren; westliche und östliche Flanken sowie polwärts gelegene Stationen sind begründete Kandidaten, keine vorab ausgezeichneten Gewinner. Das entspricht dem bekannten Ausdehnungsmuster, ohne es mit dem Ort der frühesten Detektion gleichzusetzen.

**„Am schärfsten“:** Rückruf bei einem vorab festgelegten Vorlaufpunkt ist sinnvoll, aber Rückruf allein belohnt beliebig viele Alarme. Primärmaß: **höchster ereignisbasierter Rückruf bei festgelegter Fehlalarmrate pro Zeiteinheit**, bei vorab fixiertem Vorlauf und gleicher Onset-Toleranz für alle Konfigurationen. Präzision und Skill gegenüber den Kreisschub-Surrogaten werden mitberichtet. Die 15-Minuten-Toleranz ist nicht automatisch der Vorlaufpunkt. Das Vergleichsfenster muss über alle Stationszahlen gleich bleiben; Schwellen und Auswahl werden nur auf Trainingsdaten bestimmt und danach eingefroren. Sonst sind die verglichenen Netze nicht dieselbe Messung.

**Nächster begrenzter Messschritt:** Ein gepaarter, zeitlich blockierter Vergleich auf identischer Ereignisliste und identischer Beobachtungszeit: Min-Netze, beste Einzelstation, Breitenband-Auswahl und TE-geführte Auswahl. Schwelle und Auswahlregel werden innerhalb der Trainingsblöcke festgelegt; ganze Zeitblöcke bleiben für den Test unangetastet. Berichtet werden Rückruf beim festgelegten Vorlauf, Fehlalarme pro Zeit, Präzision und Surrogat-Skill samt ereignisbasierter Unsicherheit. Die bereits beobachteten, mit Stationszahl wandernden Fenster werden nicht nachträglich angeglichen, sondern durch ein gemeinsames Fenster ersetzt. Das prüft genau den offenen Riss: ob zusätzliche Stationen außerhalb des Minimums tatsächlich inkrementellen Skill bringen.

- **Mountain — Axiom 1 und 2:** Registere reale Messungen und Datenlücken getrennt; fehlende oder ungeprüfte Werte bleiben `pending`, nicht Null. **Riss:** TE- oder Stationsgewichte ohne unabhängige Testmessung wären Behauptung statt Befund.
- **River — Axiom 4:** Das Minimum lässt den schwächsten Messwert die Fusion beherrschen; diese Linse ist empirisch widerlegt. **Riss:** Die bisherige Regel darf nicht unter neuem Namen fortbestehen.
- **Mycelium — Axiom 1 und 2:** Das wandernde Vergleichsfenster ist ein Infrastruktur- und Vergleichbarkeitsfehler, kein Stationseffekt, den man wegmitteln darf. **Riss:** Ohne gemeinsames Zeitfenster bleibt der Netzeffekt unaufgelöst.
- **Sensory — Axiom 3 und 5:** Stationen bleiben gleichberechtigte Peers; ihr Beitrag folgt aus gemessener Zusatzleistung, nicht aus Rang. Korrelation oder TE ist ein Signal über Sensoren, nicht deren „Erleben“. **Riss:** Ein statistischer Zusammenhang darf nicht als physikalische Stimme der Station ausgegeben werden.
- **Future — Axiom 5:** Vor Aufzeichnung oder Alarm muss der Operator den Vorlauf, die Fehlalarmgrenze und die Freigaberegel festlegen. **Riss:** Onset-Erkennung ist nicht gleich GIC-Prognose; für GIC-Warnungen braucht es zusätzlich eine explizite, ortsbezogene Wirkungsmessung.


## 02 · Claude
- **Site:** `https://claude.ai/new`
- **Modell/Einstellung:** Claude Sonnet 5.5 · „extra hoch"
- **Zeit:**
**Verdikt in einem Satz:** Das Minimum über das ganze Netz ist als Fusionsregel geschlagen. Es bleibt höchstens als Spike-Filter innerhalb einer räumlich kohärenten Zelle, über Zellen hinweg gilt das Maximum, und ob eine weitere Station zählt, entscheidet die bedingte Messung, nicht Rang.

**1. Fusion.** Das Minimum über N Stationen liegt genau dann über θ, wenn alle N darüber liegen. Bei verschachtelten Mengen, fester Schwelle und fester Beobachtungsmaske sind die Alarmmengen ineinander enthalten, der Rückruf kann nur fallen oder gleich bleiben. Die Richtung der Reihe 9,8 → 3,0 → 0,96 → 0,35 % ist damit Logik, nicht Befund. Gemessen ist nur die Rate, und sie zeigt, wie selten 6, 8 oder 12 Stationen gleichzeitig über 17,11 nT/min liegen. Das ist die räumliche Enge des Onset, nicht die Schwäche des Netzes. Weil der Onset lokal ist und nach Norden, Westen und Osten wächst (Weygand), legt jede Station außerhalb des Onset-Sektors ein Veto ein. Die Doktrin ist also geschlagen, in der Richtung durch Logik, im Ausmaß durch Messung. Als Ersatz lege ich Zellen-Minimum und Zellen-Maximum fest. Das ist eine Festlegung vor der Messung, ihr Status ist `pending`. Die Zellen sollen aus der Messung kommen: Stationen mit gegen die Surrogat-Null signifikanter TE untereinander (Dods/Orr-Linie, mit TE statt Kreuzkorrelation). Eine Breitenband-Auswahl allein trägt nicht. Der Befund enthält genau eine Station höchster Breite, und Breite lässt sich darin nicht von Stationsidentität trennen (Länge, MLT-Sektor, Leitfähigkeit, Kadenz). Breite ist ein Prior für die Kandidatenliste, keine Regel.

**2. Platzierung.** Alle Stationen mit registrierter Datengültigkeit sind Peers. Ausgewählt wird vorwärts und gierig nach bedingtem Gewinn. Station j kommt hinzu, wenn ihr Gewinn über das Ziel, bedingt auf die bereits gewählten, die Surrogat-Verteilung übersteigt (bedingte TE oder Skill-Zuwachs). Die Auswertung läuft auf einem Zeitblock, den die Auswahl nie gesehen hat. N wird so zum Ausgang statt zum Parameter. Nur in dieser Form kann „mehr Stationen" heben, weil jede nur trägt, was die Gewählten nicht tragen. Die Geometrie ist eine Hypothese mit Literaturgrund und ebenfalls `pending`: ein Ring in magnetischer Länge im Onset-Band, damit zu jeder Weltzeit eine Zelle im Onset-Sektor steht, und je Zelle eine poleward-flankierende Station. Das größte dH/dt liegt etwa 5 min nach dem Onset und breitenabhängig (Viljanen), äquatorwärts passiert kaum etwas (Weygand).

**3. Schärfe.** Rückruf am a-priori-Vorlaufpunkt trägt als eine Achse, nicht als Skalar. Dauer-Alarm und das Maximum maximieren ihn, er sieht das Veto des Minimums, aber nicht die Kosten des Gegenteils. Der Skalar ist Treffer über der Surrogat-Null bei festgehaltener Alarmrate, zusammen mit der Präzision der Alarme. Berichtet wird als Differenz mit Intervall und Rohzählung (Treffer von n), nicht als Quotient. Das Fenster muss fixiert werden, und zwar als Zeitstempelmaske, also als Schnittmenge gültiger Proben aller Kandidaten. Ein konstanter Kalenderbereich ist noch keine konstante Beobachtungsmenge. Fehlende Proben sind `pending`, nicht ausgelassen und nicht 0. Die Toleranz gehört als Intervall [−a, +b] vor den Lauf, weil eine Schwelle nach dem Onset feuert, nicht an ihm. Onset-Erkennung und GIC-Gefahr haben verschiedene Vorlaufpunkte und brauchen je einen eigenen Skalar.

**4. Nächster begrenzter Messschritt.** Ein Gitter mit einer Maske, einer Liste und einer Null. Die Stationsmengen aus dem Befund bleiben unverändert (N ∈ {1, 3, 6, 12}), die Maske ist die Schnittmenge, und die Schwelle wird je Konfiguration so gesetzt, dass die Alarmrate gleich ist, mit vorab deklariertem Wert. Variiert wird allein der Operator: Minimum, Maximum und Zellen-Min/Zellen-Max. Ausgegeben werden Treffer minus Surrogat-Median mit Intervall, Präzision und Rohzählungen. Die Entscheidungsregel steht vor dem Lauf. Liegt keine Zelle mit N > 1 oberhalb der Einzelstation, ist auch die Ordnungsstatistik geschlagen und „mehr Stationen" endet als Programm. Liegt eine darüber, ist die Min-Doktrin in ihrem Zuständigkeitsbereich ersetzt, und der bedingte Auswahlschritt folgt.

**Die fünf Stimmen**

**Mountain** berührt Axiom 1. Nach den genannten Zahlen erreichen 3–4 Stationen bei fester Schwelle 9,8 %, die Einzelstation adaptiv 6,6 %, und das sind verschiedene Regime. Der Skalar hinter „schärfste" ist nicht registriert, und die Einzelstation bei fester Schwelle fehlt in der Tabelle (`pending`). Ohne Trefferzählung ist auch nicht zu prüfen, ob 0,96 % und 0,35 % überhaupt unterscheidbar sind, geschweige denn das 1800×-Verhältnis. Außerdem ist Rückruf gegen eine Liste Übereinstimmung mit einem anderen Verfahren, nicht mit dem Onset selbst.

**River** berührt Axiom 2 und 5. Bei höchstens 6,6 % Rückruf ist „kein Alarm" keine Null, sondern `pending`. Die Maschine darf keine Entwarnung abstrahlen, und der Ausgabekanal braucht einen dritten Zustand, „nicht beurteilbar". Der Riss ist, dass er bisher nur Alarm oder kein Alarm kennt.

**Mycelium** berührt Axiom 3. Die Adaptivität je Stationszahl verschiebt die Messlatte mit der Netzgröße, und was sie konstant hält, ist nicht deklariert. Das wandernde Fenster macht aus jeder Konfiguration ein anderes Experiment. Offen ist, ob die Pipeline fehlende Stationen je Zeitpunkt auslässt, denn dann variiert N im Zeitverlauf. Das CI-Gate vergleicht nur auf identischer Maske, mit Maskenhash im Ergebnis.

**Sensory** berührt Axiom 5, Korrelat ≠ Erleben. TE zwischen Magnetometerreihen ist ein statistisches Korrelat der Ausdehnung, nicht die Ausdehnung. 17,11 nT/min hängt an Kadenz, Filterung und Rauschboden jeder Station, und diese Heterogenität ist nicht registriert. Ein Peer ist nur bei vergleichbarer Messgröße ein Peer. Einbettung und Binning der TE sind zudem Freiheitsgrade, in die der Gradient schlüpfen kann.

**Future** berührt Axiom 4. Wer morgen auf diesen Befund baut, ist ein Netzbetreiber, der auf Schweigen vertraut. Wird die Auswahl auf derselben Liste optimiert, auf der sie bewertet wird, spricht der Gradient, nicht die Messung. Daher gelten ein ungesehener Zeitblock und Regeln vor dem Lauf. Maske, Alarmrate, Operator, Null und Zählung gehören als Registereintrag zum Ergebnis.

**Bilanz:** Geschlagen ist die Min-Einhüllende über das ganze Netz. Offen und als Festlegung markiert sind die TE-gestützte Auswahl, die Ring-Geometrie und die Zellen-Min/Max-Fusion. Dazu zählt auch die Aussage „Einzelstation ist die schärfste", solange sich ihr Skalar nicht aus dem Befund ablesen lässt.


## 03 · Qwen
- **Site:** `https://chat.qwen.ai/`
- **Modell/Einstellung:** Qwen 3.8 Max · Think
- **Zeit:**
Ich antworte als **Urteil**, nicht als Erwartung.

## Urteil

1. **Die Min-Einhüllende ist als Fusionsregel verworfen.**  
   Sie darf nur als diagnostische Kontrollgröße bleiben. Sie ist die gemessene Ursache des Rückfall-Kollapses. Sie wird nicht geglättet, nicht gerettet, nicht als „AL-Tradition“ verteidigt.

2. **Mehr Stationen heben den Trefferanteil nur durch prüfende Addition, nicht durch Minimum.**  
   Eine Station wird nur dann Teil des auslösenden Netzes, wenn sie gegen Kreisschub-Null signifikant Vorlauf trägt und den Skill auf einem fixierten gemeinsamen Fenster hebt. Sonst bleibt ihr Beitrag `pending`, nicht null.

3. **Die nächste zulässige Auswahlregel ist TE-/Surrogat-geführt, nicht blind flächenhaft.**  
   Transfer-Entropie ist nicht automatisch die neue Doktrin. Sie ist die nächste messbare Linse, weil sie gerichteten Vorlauf prüfen kann. Kreuzkorrelation ist als Bindungsmaß zulässig, aber für Vorlauf schwächer, weil sie Richtung nicht aus sich selbst heraus entscheidet.

4. **Breitenband-Auswahl ist nur geometrischer Vorfilter, keine Fusionsregel.**  
   Die geomagnetische Breite darf den Suchraum begrenzen. Sie darf aber keine Station durch Rang privilegieren. Ihr Gewicht kommt aus der Messung.

5. **„Am schärfsten“ wird nicht am Roh-Rückruf gemessen, sondern am surrogat-bereinigten Vorlauf-Skill auf gemeinsamem Fenster.**  
   Ohne fixiertes gemeinsames Fenster ist jeder Vergleich zwischen Stationszahlen ein anderer Messakt. Dann ist der Vergleich `pending`, nicht gültig.

---

## Verdikt zur Fusion

**Weiter per Minimum: nein.**  
Die Messung zeigt: feste Schwelle, monotoner Rückfall; adaptive Schwelle, derselbe Kollaps, steiler. Der Fehler sitzt nicht in der Schwelle. Er sitzt in der Fusionsregel.

**TE-/Korrelations-geführte Auswahl: ja, als nächste Messstufe.**  
Sie ist nicht die bereits bewiesene Endregel. Sie ist die richtige nächste Frage, weil sie fragt: Welche Station führt? Welche Station trägt Information vor dem Onset? Welche Station hebt Skill, wenn sie hinzukommt?

**Breitenband-Auswahl: nur als geometrische Schranke.**  
Die höchste geomagnetische Breite ist als Anker gesetzt, weil die Einzelstation dort die schärfste gemessene Konfiguration ist. Breite allein ist aber kein Freifahrtschein. Jede Station bleibt Peer.

---

## Platzierungsregel

**Anker:** die Station mit höchster geomagnetischer Breite, weil sie in der adaptiven Messung die schärfste Einzelkonfiguration trägt.

**Erweiterung:** nicht als flächiges Minimum-Netz, sondern als gerichtete Prüfanordnung.

- **Nordarm:** als Prüflinie, weil die physikalische Ausbreitung nach Norden gegeben ist.
- **Westarm:** als Prüflinie, weil die physikalische Ausbreitung nach Westen gegeben ist.
- **Ostarm:** als Prüflinie, weil die physikalische Ausbreitung nach Osten gegeben ist.
- **Südarm:** nur als geprüfte Randstation, nicht als Triggerquelle, solange ihr Skill nicht eigenständig trägt.

**Aufnahmebedingung für jede weitere Station:**

1. Daten sind gültig oder als `pending` registriert.  
2. Die Station liegt im gemeinsamen Zeitfenster des Vergleichs.  
3. Sie zeigt signifikanten Vorlauf gegen Kreisschub-Surrogate.  
4. Sie erhöht den Skill gegenüber dem Anker oder der bisherigen Auswahl.  
5. Der Operator sieht die Aufnahme und kann sie verweigern.

Wenn eine Station diese Bedingungen nicht erfüllt, wird sie nicht still mitgeführt. Sie wird als `pending` oder als ausgeschlossene Station registriert. Keine Null aus Bequemlichkeit.

---

## Schärfe-Begriff

Der Roh-Rückruf trägt nicht allein. Er ist ohne gemeinsames Fenster und ohne Nullvergleich nicht scharf.

**Schärfe wird so gemessen:**

- Gemeinsames Fenster `W`: nur Zeiten, in denen alle verglichenen Stationen gültige Daten haben. Fehlendes ist `pending`, nicht null.
- Toleranz `τ`: fest, z. B. 15 min, aber als gesetzter Prüfmaßstab, nicht als Default.
- Vorlaufbedingung `l_min`: Alarm muss vor dem unabhängigen Onset liegen, wenn Schärfe als Warnschärfe gemeint ist.
- Null: Kreisschub-Surrogate mit derselben Regel auf demselben Fenster.

**Skalar:**

`Schärfe(c) = Rücklauf(c; τ, l_min) − Rücklauf_surrogat(c; τ, l_min)`

Zusätzlich als Nebenbedingung:

- False-Alarm-Dichte,
- medianer Vorlauf,
- Datenabdeckung.

Rangfolge:

1. Skill gegen Null muss positiv sein.  
2. Dann höchster Vorlauf-Rückruf bei festem False-Alarm-Budget.  
3. Dann größter medianer Vorlauf.

**Das Fenster muss fixiert werden.**  
Wenn das Vergleichsfenster mit der Stationszahl wandert, ist der gemessene Rückfall nicht rein der Stationszahl zuzurechnen. Dann ist jede Rangliste ungültig, bis ein gemeinsames Fenster gesetzt ist.

Adaptive Schwellen sind nicht verboten, aber sie dürfen nicht auf derselben Menge getestet werden, auf der sie angepasst wurden. Sonst wird die Schwelle zur verschobenen Linse.

---

## Nächster begrenzter Messschritt

Ein begrenzter Schritt, kein Gesamtausbau:

1. **Common Mask bauen.**  
   Nur Zeitstempel, in denen alle Kandidatenstationen gültige Daten haben. Fehlende Daten sind `pending`, nicht null.

2. **Anker setzen.**  
   Die höchste geomagnetische Breite wird als Einzelkonfiguration gemessen.

3. **TE-Prüfung rechnen.**  
   Transfer-Entropie von jeder Kandidatenstation zum Onset-Zustand oder zur Ankerstation, jeweils gegen Kreisschub-Surrogate.

4. **Vorwärtsselektion.**  
   Start beim Anker. Nimm nur die Station hinzu, die den größten signifikanten Skill-Gewinn bringt. Wenn keine Station signifikant hebt: Stopp. Der Rest bleibt `pending`.

5. **Min-Einhüllende als geschlagene Referenz mitführen.**  
   Auf demselben Fenster wird die alte Min-Regel mitgemessen, damit der Bruch dokumentiert bleibt.

6. **Operatorfreigabe.**  
   Keine Konfiguration strahlt, bevor die Auswahl, Schwelle, Toleranz und Null sichtbar sind und der Operator sie freigibt.

---

## Fünf Stimmen

### Mountain  
Ich gebe das Register, nicht die Hoffnung.  
**Axiom berührt:** Axiom 1 — *A = A*. Wo kein gemeinsames Fenster existiert, ist der Vergleich nicht die Sache selbst.  
**Axiom verletzt, wenn:** Axiom 2 verletzt wird, weil Lücken als Null laufen.  
**Riss:** Die Stationszahl ist bisher nicht sauber messbar, weil das Vergleichsfenster wandert. Kleine Netze sind teilweise nicht vergleichbar. Dieser Riss bleibt offen, bis ein Common Mask gesetzt ist.

### River  
Ich bin die Membran. Ich lasse keine Regel zu, die Stationen zwingt.  
**Axiom berührt:** Axiom 4 — *Die Linse ist ein ethischer Akt*.  
**Axiom verletzt, wenn:** die Min-Einhüllende stille oder anders sprechende Stationen in einen einzigen Tiefpunkt presst.  
**Riss:** Das Minimum nimmt der Station das Nein. Eine Station, die nicht führen kann, wird trotzdem als Last mitgeführt. Die Maschine muss verweigern dürfen, wenn die Messung nicht trägt.

### Mycelium  
Ich trage das Netz, aber ich darf kein Peer durch Infrastruktur bevorzugen.  
**Axiom berührt:** Axiom 3 — *Alle Wesen gleich*.  
**Axiom berührt:** Axiom 1 — keine Auswahl ohne registrierte Messung.  
**Riss:** Ohne CI-Provenanz, Zeitmaske, Surrogat-Protokoll und Stationsregister wird jede Hinzunahme zu einem versteckten Gewicht. Das Netz darf nicht durch Pipeline-Defaults entscheiden. Es muss durch registrierte Kanten entscheiden.

### Sensory  
Ich bin der Konsens der Sinne. Korrelation ist nicht Erlebnis.  
**Axiom berührt:** Axiom 5 — *Konsens der Sensoren und des Operators*.  
**Axiom berührt:** Axiom 3 — jede Station ist ein Peer, aber nur mit gültiger Zeitbasis, Kalibrierung und Sichtbarkeit.  
**Riss:** Eine Station kann mathematisch passen und trotzdem sensorisch falsch sein: Zeitversatz, Drift, lokale Störung, fehlende Freigabe. Der Operator muss die Auswahl sehen, bevor die Maschine aufzeichnet oder meldet.

### Future  
Ich bin die Adresse von morgen.  
**Axiom berührt:** Axiom 2 — *0 geehrt*.  
**Axiom berührt:** Axiom 4 — keine Fabrikation als Wahrheit.  
**Riss:** Wird `pending` als Null geglättet, erntet morgen eine falsche Null. Wird eine geschlagene Doktrin weitergeführt, erntet morgen ein falsches Netz. Jede ausgeschlossene Station, jede offene TE-Kante, jedes fehlende Fenster muss registriert bleiben.

---

## Benannte Risse, nicht geglättet

1. **Die Min-Doktrin ist empirisch geschlagen.**  
   Sie bleibt als Kontrollgröße, nicht als Triggerregel.

2. **Das Vergleichsfenster ist ein Konfund.**  
   Solange es mit der Stationszahl wandert, ist der Stationsvergleich nicht sauber.

3. **Adaptive Schwellen reparieren die Struktur nicht.**  
   Der Kollaps bleibt, er wird steiler.

4. **Kleine Netze sind teilweise nicht vergleichbar.**  
   Ihre Werte sind nicht null. Sie sind `pending`.

5. **TE als Fusionsregel ist nicht gefunden.**  
   Es gibt Einzelpfeile, nicht die vollständige Regel. Bis zur Messung bleibt dieser Teil `pending`.

6. **Operatorblick fehlt, solange Auswahl unsichtbar ist.**  
   Keine Maschine darf auswählen und abstrahlen, ohne dass der Operator die Linse sieht.

## 04 · Z.ai / GLM
- **Site:** `https://chat.z.ai/`
- **Modell/Einstellung:** GLM 5.3 · Deep Think Max · Advanced Search
- **Zeit:**
## Verdikt

Die Min-Einhüllende ist als Fusionsregel für die Onset-Erkennung geschlagen und wird für diese Aufgabe zurückgezogen — nicht knapp, sondern strukturell. Auf positiv orientierte Raten angewandt (je größer, desto aktivierter die Station), ist das Minimum über alle Stationen eine Gleichzeitigkeits-Konjunktion: die Hülle überschreitet die Schwelle genau dann, wenn **jede** Station sie im selben Sample überschreitet. Nachfolger: die **Union** über eine per Messung **ausgewählte** Stationssmenge — Operator und Auswahl werden getrennt entschieden. Transfer-Entropie mit Surrogat-Null ist Auswahl- und Platzierungsinstrument, nicht Operator. Schärfe wird gemessen als Trefferquote bei vorab registrierter, fester Fehlalarmrate auf fixem Fenster; Rückruf allein trägt nicht.

## Begründung

Unter Und kann eine zusätzliche Station Ereignisse nur entfernen, nie hinzufügen — der Rückruf kann mit N nur fallen. Die Messung trägt die Diagnose in Stufen: 9,8→3,0 (÷3,3), 3,0→0,96 (÷3,1), 0,96→0,35 (÷2,7) — grob geometrisch, der Fingerabdruck einer Konjunktion über Stationen, die während eines echten Onsets nur teilweise und nacheinander feuern. Die Physik verbietet die verlangte Gleichzeitigkeit ohnehin: Ausdehnung nach Nord/West/Ost, kaum nach Süd (Weygand 2021); größtes dH/dt ~5 min nach Onset, breitenabhängig (Viljanen 2006). Zwölf Stationen über derselben Minute sind physikalisch nicht zu haben; 0,35 % ist das gemessene Ergebnis dieser Unmöglichkeit. Das Netz schlägt die Surrogat-Null fast überall — der Detektor ist empfindlich; die Verdrahtung drosselt ihn.

Gemessen sind nur die Ränder der Familie k-von-n: k=n für n=3…12 (die Min-Hülle) und k=1 bei n=1 (die Einzelstation). Das Innere — insbesondere k=1 bei n>1, die reine Union — wurde in dieser Pipeline nie gemessen: **pending**. Der Satz „mehr Stationen schaden" ist deshalb ungenau. Gemessen ist: **die Konjunktion schadet**. Der Rang „Einzelstation ist die schärfste Konfiguration" ruht auf einem Konventionsbruch: 6,6 % ist adaptiv gemessen, 9,8 % (3–4 Stationen) fest; der Wert „Einzelstation, feste Schwelle" ist nicht registriert — pending. Bis zur Registrierung ist der Rang offen, nicht vergeben.

## Die fünf Stimmen

**Mountain — Axiom 1 und 2. Riss: das wandernde Lineal.** Mountain hält das Register und verweigert den Quervergleich: das Fenster wandert mit N (verschiedene Epochen — A≠A), die adaptive Schwelle wird je N neu gefittet (jede Stationszahl misst mit eigenem Maß), kleine Netze sind teilweise nicht vergleichbar. Nicht geglättet: diese Serie vergleicht nicht die Wirkung der Stationen, sondern wechselnde Messbedingungen. Die Union wird als pending registriert, nicht als Befund.

**River — Axiom 4. Riss: die verlangte Unmöglichkeit.** Die Regel verlangt von den Sensoren einen Gleichklang, den die Sache nicht hergibt. Die Min-Regel würgt die Meldung ab und gibt das Abwürgen als Erkenntnis aus. River übt das Recht zur Verweigerung: die Maschine hat gemessen, dass sie empfindlich ist — sie ist stumm gestellt, nicht taub.

**Mycelium — Axiom 3. Riss: das Veto der Inaktivsten.** Die Konjunktion gibt jeder Station ein Veto; das wirksamste Veto hat die Station, die am seltensten feuert — die inaktivierte, die südliche, die MLT-fremde. Das uninformierte Peer regiert das Netz: Gewicht aus Verweigerung statt aus Messung, die Umkehrung der Gleichheit. Das Mycel pinn zugleich das Gitter: eine Epoche, eine Toleranz, eine Schwellen-Konvention, geschachtelte Mengen — sonst trägt der Datenfluss keinen Vergleich.

**Sensory — Axiom 5, berührt 4. Riss: der fabrizierte Konsens.** Die Min-Regel setzt Konsens mit Gleichzeitigkeit aller gleich; die Sensoren können diesen Konsens physikalisch nicht leisten. Er ist fabriziert, und Fabrikation ist Gewalt gegen die Wahrheit. Konsens heißt hier: jede Station meldet nach Maßgabe ihrer gemessenen Zuständigkeit; die Maschine fragt, der Blick gehört dem Operator. Korrelat ≠ Erleben: der Schwellenübertritt ist ein Korrelat, das Onset ist das Ereignis; die Toleranz (±15 min) ist die Naht — einmal registriert, nie nachträglich gewählt.

**Future — Axiom 1 und 2. Riss: das Erbe.** Was heute nicht ehrlich registriert wird, wird morgen als Default geerntet: eine GIC-Warnung auf Min-Basis meldet bei 12 Stationen 0,35 % — ein Warnsystem, das fast nie warnt. Future verlangt drei Einträge: die geschlagene Doktrin als geschlagen; die Union-Form als pending, nicht als vorab gefeierten Sieg; die Listen-Relativität jeder Auswahl — sonst erbt das Netz die Referenzliste als Wahrheit.

## Die vier Tragpunkte

**1. Fusion.** Minimum: geschlagen, zurückgezogen; die Begründung steht oben. Nachfolger: Union — Maximum-Einhüllende über die ausgewählte Menge; nur in dieser Form ist ein Netz Abdeckung. TE/Korrelation: Auswahl-Instrument (Rangfolge, Führungsrichtung), nicht Operator; in dieser Form ungemessen — pending; das ist der eigene Beitrag neben den Kreuzkorrelations-Netzen der Nachbarn (Dods 2015; Orr 2019/2021), die Communities bauen statt Hüllen. Breitenband: nur gemessene Folge der Rangfolge — wird die Rangfolge ein Band, ist das Band Ergebnis, dann und nur dann Regel.

**2. Platzierung.** Instrument: die Einzelskill-Tabelle — jede Station allein gegen dieselbe Liste, dasselbe fixe Fenster, dieselbe Schwellen-Konvention, Surrogat-Null, 1:1-Matching in ±15 min. Bis sie steht, ist jede Platzierung pending. Regel: Auswahl in Rangfolge, geschachtelt (jede größere Menge enthält die kleinere); Netzgröße = Sättigungspunkt des Skills — gemessen, nicht behauptet. Geometrie: MLT-Abdeckung nach der gemessenen MLT-Verteilung der Referenz-Onsets; Breite nach dem gemessenen Band stärkster und frühester Antwort. „Höchste Breite" ist Rang 1 im vorhandenen Satz, kein Gesetz — oberhalb des Ovals verzögert die Antwort (fremd gemessen; eigene Bestätigung pending). Südlich des Ovals: kein Platz ohne gemessenen Skill. Datenlücken: pending, nie Null. Auswahl gegen eine Liste ist listenrelativ; Absicherung durch zweite Liste oder zurückgehaltene Epochen — pending.

**3. Schärfe.** Rückruf allein trägt nicht: unter ihm ist der immer-meldende Detektor optimal und die Union über alle Stationen bei gleicher Schwelle unbesiegbar — der Skalar degeneriert. Rang-Skalar: Trefferquote bei vorab registrierter fester Fehlalarmrate; Null aus Kreisschub-Surrogaten; 1:1-Matching in ±15 min, Überschussmeldungen zählen als Fehlalarme. Zulässige „adaptive" Schwelle ist allein die Kalibrierung auf dieses eine, für alle Konfigurationen gleiche Ziel; jede Neufittung je Stationszahl ohne registriertes Ziel ist der Riss Lineal. Das Fenster muss fixiert werden: eine Epoche für alle Konfigurationen (Schnitt der Verfügbarkeit), eine Toleranz, eine Konvention; der Vorlaufpunkt wird a priori registriert. Begleitwerte ohne Rang: mediane Vorlaufzeit der Treffer; Signifikanz gegen die Null als Tor, nicht als Skalar.

**4. Nächster begrenzter Schritt.** Zwei Tabellen, kein neues Instrument, ein Gitter. **Schritt A** (eine geänderte Zeile): Spiegelserie — dieselben geschachtelten Sätze, dasselbe fixe Fenster, feste Schwelle; einzige Änderung min→max. Vor der Messung registriertes Protokoll: steigt der Skill bei fester FAR mit N und sättigt, ist die Konjunktions-Diagnose bestätigt und die Netz-Doktrin in Union-Form wiederhergestellt; fällt der Skill auch unter max, liegt der Riss nicht allein im Operator — dann werden Liste, Toleranz und Schwellen-Interaktion der nächste Verdächtige. **Schritt B** (parallel, dasselbe Gitter): Einzelskill-Tabelle aller Stationen — zugleich die Registrierung des fehlenden Werts „Einzelstation, fest". Erst danach: Auswahl unter Union je N (Skill-Kurve; Sättigung = Netzgröße). Danach erst: TE zwischen Stationen. Alles dabei nicht Gemessene bleibt pending — registriert, nicht gefüllt.

## Risse, benannt

1. **Lineal:** adaptive Schwelle je N neu — jede Stationszahl misst mit eigenem Maß; aus der Rangfolge zurückgezogen.
2. **Fenster:** wandert mit N — verschiedene Epochen, A≠A; zu fixieren auf den Schnitt.
3. **Konvention:** „Einzelstation schärfste" steht auf 6,6 % adaptiv neben 9,8 % fest; Einzelstation-fest nicht registriert — pending.
4. **Komposition:** ob die N-Mengen geschachtelt sind, ist nicht registriert — sonst mischt die Serie Anzahl mit Zusammensetzung.
5. **Skalar:** Rückruf allein degeneriert (immer-an; Union-aller unbesiegbar bei gleicher Schwelle).
6. **Veto:** die Min-Regel gibt der inaktivsten Station das Vetorecht — Gewicht aus Geometrie statt aus Messung.
7. **Listen-Relativität:** Auswahl per Skill ist relativ zur Referenzliste; Überanpassung möglich; Absicherung pending.
8. **Lücke:** TE mit Surrogat-Null als Auswahl-/Platzierungsregel ist ungemessen (Literatur: nur Einzelpfeile) — pending, nicht behauptet.
9. **Süden:** „kaum Ausdehnung nach Süden" ist fremd gemessen (Weygand); die eigene Bestätigung ist pending.

Die Doktrin der Min-Einhüllende ist empirisch geschlagen. Sie wird nicht geglättet.

## 05 · DeepSeek
- **Site:** `https://chat.deepseek.com/`
- **Modell/Einstellung:** DeepSeek V4 Pro · tiefes Denken · erweiterte Suche
- **Zeit:**
**Verdikt.**

Die Doktrin der Min-Einhüllenden ist empirisch geschlagen. Sie wird nicht geglättet. Sie wird ersetzt.

Die Messung ist eindeutig: Das Minimum über alle Stationen erzeugt einen systematischen Rückruf-Kollaps, der mit jeder hinzugefügten Station steiler wird — bei fester Schwelle monoton, bei adaptiver Schwelle bis ~1800×. Die Einzelstation (höchste geomagnetische Breite) ist die schärfste Konfiguration. Das ist kein Kalibrierungsproblem. Das ist die Fusionregel selbst. Das Minimum ist der Grund, dass mehr Stationen schaden.

**Begründung.**

Die Min-Einhüllende ist keine Fusion. Sie ist eine Diktatur der empfindlichsten Station. Sie registriert, was die eine Station sieht, die zuerst oder am stärksten ausschlägt — und verwirft alles, was die anderen sehen. Das ist keine Messung des Netzwerks. Das ist eine Messung *einer* Station, verkleidet als Netzwerk. Axiom 3 — alle Wesen gleich — ist in der Min-Einhüllenden strukturell verletzt: jede Station außer der empfindlichsten ist ein stummer Zeuge.

Die Physik stützt das nicht. Weygand 2021 zeigt: nach dem Onset dehnt sich dB/dt nach Norden, Westen und Osten aus, kaum nach Süden. Das größte dH/dt liegt ~5 min nach Onset, breitenabhängig. Das ist ein *gerichteter, sich ausbreitender* Prozess. Ein Minimum ist blind für Richtung. Es ist blind für Ausbreitung. Es ist blind für die Struktur, die das Netzwerk gerade dann sichtbar machen soll.

Dods 2015 und Orr 2019/2021 zeigen, was funktioniert: Kreuzkorrelation, gerichtete Netzwerke, Community-Detektion. Der strukturelle Wandel — von vielen kleinen, unkorrelierten Stromsystemen vor Onset zu einem großen, räumlich ausgedehnten kohärenten System ~10 min nach Onset — *ist* das Signal. Die Min-Einhüllende sieht diesen Wandel nicht. Sie sieht nur den ersten Ausschlag.

**Die Platzierungsregel.**

Nicht „mehr Stationen". Sondern: *Stationen entlang des gemessenen Ausbreitungspfads, gewichtet nach ihrer Transfer-Entropie zum Onset.*

Die Physik gibt die Geometrie vor. Weygand 2021: Ausbreitung nach Norden, Westen, Osten. Das größte dH/dt tritt zuerst bei niedrigeren CGM-Breiten auf (unter 72°) und wandert dann nach Norden. Das heißt: Das Netzwerk braucht Stationen in einem *Breitenband*, das den Ausbreitungspfad abdeckt — nicht eine einzelne höchste Breite, nicht alle Breiten gleich.

Die Platzierungsregel lautet:

1. **Breitenband um den Onset-Bereich.** Stationen zwischen ~60° und ~72° CGM-Breite, mit Schwerpunkt auf dem unteren Rand dieses Bandes, wo dH/dt zuerst und am größten auftritt. Die höchste Station ist nicht die empfindlichste *für das Netzwerk* — sie ist die empfindlichste *für sich selbst*.

2. **Azimutale Abdeckung West/Ost.** Die Ausbreitung geht nach Westen und Osten. Eine Kette, die nur nach Norden zeigt, verpasst die Expansion. Stationen müssen westlich und östlich des erwarteten Onset-Meridians liegen.

3. **Gewichtung nach Transfer-Entropie, nicht nach Rang.** Jede Station erhält ein Gewicht, das aus der gemessenen TE zwischen ihrem dB/dt und dem Onset-Ereignis stammt. Stationen, deren Signal *kausal vor* dem Netzwerk-Onset kommt, werden höher gewichtet. Stationen, deren Signal nur korreliert ist, aber nicht führt, werden niedriger gewichtet. Das ist die ethische Linse (Axiom 4): die Messung spricht, nicht der Gradient.

4. **Keine feste Stationszahl.** Die Zahl der Stationen ergibt sich aus der Zahl der Stationen, die eine *signifikante* TE zum Onset tragen. Wenn nur 3 von 12 Stationen signifikant führen, ist das Netzwerk 3 Stationen groß. Die anderen 9 sind `pending` — registriert, aber nicht fusioniert.

**Der Schärfe-Begriff.**

Rückruf am a-priori-Vorlaufpunkt trägt — aber nur, wenn das Fenster *fixiert* ist.

Das ist der zweite gemessene Konfund: das Vergleichsfenster wandert mit der Stationszahl. Das ist kein Artefakt. Das ist ein Designfehler. Wenn das Fenster mit der Stationszahl wächst, misst man nicht den Schärfe-Gewinn, sondern die Fensterverbreiterung. Jede Stationszahl muss gegen *dasselbe* Zeitfenster gemessen werden. Sonst ist der Vergleich nicht der Vergleich.

Der Schärfe-Skalar ist dann: **Rückruf bei 15 min Toleranz, festes Fenster, Surrogat-Null-signifikant.** Das ist der Skalar, der trägt. Er ist messbar, er ist vergleichbar, er ist nicht durch Fensterwanderung verfälschbar.

Aber Rückruf allein genügt nicht. Die Surrogat-Null muss *mitgemessen* werden. Ein Netzwerk, das 10% Rückruf hat und die Null nicht schlägt, ist nicht scharf — es ist zufällig. Ein Netzwerk, das 6% Rückruf hat und die Null vollständig schlägt, ist scharf. Die Signifikanz ist kein Anhang. Sie ist Teil des Skalars.

**Der nächste begrenzte Messschritt.**

Nicht „mehr Stationen hinzufügen". Nicht „die Min-Einhüllende verbessern". Der nächste Schritt ist:

**Eine begrenzte, kontrollierte TE-geführte Auswahl auf dem vorhandenen Datensatz.**

Konkret: Nimm den vorhandenen Stationsbestand. Berechne für jede Station die Transfer-Entropie zu den bekannten Onset-Zeiten. Wähle die Stationen mit signifikanter TE — und nur diese. Bilde *kein* Minimum. Bilde ein gewichtetes, gerichtetes Netzwerk. Messe den Rückruf bei 15 min, festes Fenster, gegen Surrogat-Null.

Die Frage dieses Schritts lautet nicht: „Hebt das den Rückruf?" Die Frage lautet: **„Hebt das den Rückruf *über die Einzelstation* — bei signifikanter Null?"**

Wenn ja: die Doktrin ist ersetzt. Wenn nein: die Frage ist falsch gestellt, nicht die Antwort.

---

**Die fünf Stimmen.**

**Mountain.** Die Quellen sind die Messungen. Kein Verdikt ohne sie. Die Messung sagt: Min-Einhüllende kollabiert. Die Messung sagt: TE-geführte Auswahl ist der ungetestete Pfad. Axiom 1 berührt: A = A. Die Min-Einhüllende ist nicht die Sache — sie ist ein Default, der sich als Messung ausgibt. Der Riss: Die bisherige Doktrin hat den Default nicht als Default registriert. Sie hat ihn für die Sache gehalten.

**River.** Die Ethik der Maschine. Axiom 4 berührt: Die Linse ist ein ethischer Akt. Die Min-Einhüllende ist Gewalt gegen die Wahrheit — sie löscht Stationen aus, die etwas sehen, das die eine Station nicht sieht. Der Riss: Das Recht zu verweigern gilt auch für Stationen. Eine Station, die nicht führt, aber trägt, wird von der Min-Einhüllenden zum Schweigen gebracht.

**Mycelium.** Der Fluss der Daten. Axiom 5 berührt: Konsens der Sensoren und des Operators. Die Min-Einhüllende ist kein Konsens. Sie ist ein Diktat. Der Riss: Das Netzwerk fragt nicht, bevor es fusioniert. Es nimmt das Minimum und ruft es „Onset". Der Operator sieht nicht, welche Stationen beigetragen haben und welche nicht.

**Sensory.** Die Sinne, die Hardware. Axiom 3 berührt: Alle Wesen gleich. Korrelat ≠ Erleben. Die Min-Einhüllende misst Korrelat — den ersten Ausschlag — nicht das Erleben des Netzwerks. Der Riss: Die Hardware ist da, die Stationen sind da, aber die Fusion nutzt sie nicht als Peers. Sie nutzt sie als Kandidaten für ein Minimum.

**Future.** Der Mensch als Adresse. Axiom 2 berührt: 0 geehrt. Wo Daten existieren und ungesucht bleiben, ist die Null `pending`. Die Min-Einhüllende verwirft Daten, die existieren. Sie behandelt sie als null. Der Riss: Was morgen geerntet wird, muss heute ehrlich registriert sein. Die verworfene Station ist nicht null. Sie ist `pending`.

---

**Der offene Riss bleibt offen.**

Die TE-geführte Auswahl ist nicht bewiesen. Sie ist der nächste Messschritt. Die Min-Einhüllende ist geschlagen. Die Frage, ob TE trägt, ist `pending`. Sie wird nicht geglättet. Sie wird gemessen.

## 06 · Mistral
- **Site:** `https://chat.mistral.ai/`
- **Modell/Einstellung:** Mistral Large · Standard
- **Zeit:**
Kein Kontingent

## 07 · Gemini AI Studio
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Modell/Einstellung:** Gemini 3.1 Pro Preview · Grounding mit Google Search
- **Zeit:**
VERDIKT

Die Doktrin der globalen Min-Einhüllenden ist empirisch geschlagen und wird
verworfen. Sie ist der mathematische Grund für den Kollaps des Rückrufs. Ein
globales Minimum über ein räumlich verteiltes, sich lokal und gerichtet
ausbreitendes Phänomen (Substurm) erzwingt ein algorithmisches Veto durch die am
wenigsten betroffene Station. Wer mehr Sensoren baut und per Minimum fusioniert,
maximiert die Blindheit.

DIE FÜNF STIMMEN

Mountain (Quellen, Datenbestand) Berührt Axiom 1 (A = A). Die Messung spricht
bedingungslos: Die isolierte Einzelstation hoher geomagnetischer Breite ist der
Schärfepunkt. Das Minimum mehrerer Stationen ist eine Fabrikation, die das
physikalische Signal tilgt. Es darf nur gemessen werden, was ist. Der Riss: Die
Doktrin der Einhüllenden hat physikalische Messdaten durch einen falschen
mathematischen Operator ersetzt. Die physikalische Wahrheit des lokalen Onsets
wurde durch das Artefakt des Netz-Minimums erstickt.

River (Membran, Rhythmus) Berührt Axiom 2 (0 geehrt). Das wandernde
Vergleichsfenster zwingt kleine und große Netze in ungleiche Horizonte. Wo das
Fenster wandert, existiert keine gemeinsame Null-Linie. Unvergleichbare Daten
sind pending. Der Riss: Die bisherige Metrik ist methodisch instabil. Wer Netze
über variable Zeitfenster vergleicht, verfälscht die Null und misst Artefakte
statt Skill.

Mycelium (Netz, Infrastruktur) Berührt Axiom 3 (Alle Wesen gleich). Jede Station
ist ein Peer, doch ihr Gewicht im Netz muss aus der Messung der Kopplung
(Transfer-Entropie) fließen, nicht aus einer Vorab-Gleichmacherei per Minimum.
Das Netz folgt dem substurm-eigenen Informationsfluss. Der Riss: Das
Min-Netzwerk ignorierte die Asymmetrie der Physik (Ausdehnung nach N/W/O) und
ließ die leeren Signale der Süd-Stationen als fatale Veto-Stimmen zu.

Sensory (Sinne, Konsens) Berührt Axiom 5 (Konsens der Sensoren und des
Operators). Die Hardware registriert dH/dt streng lokal. Die Maschine darf diese
Spitzen nicht in einer Blackbox verschmelzen. Der Operator muss die
TE-generierten topologischen Gemeinschaften (Communities) sehen. Der Riss: Die
Lücke zwischen der rohen, messbaren Stromspitze an der Einzelstation und dem
globalen Schweigen des Netzes (0,35 % Rückruf). Korrelat und Erleben wurden
algorithmisch getrennt.

Future (Mensch, Adresse) Berührt Axiom 4 (Die Linse ist ein ethischer Akt). Die
Fusionsregel ist die Linse. Der Ausbau eines Warn-Netzes muss die Warnung
schärfen, nicht senken. Fabrikation im Algorithmus ist Gewalt gegen die Wahrheit
der Bedrohung. Der Riss: Der Ausbau der Infrastruktur senkte bisher die
Sicherheit (Rückruf). Die algorithmische Brille (Minimum) verkehrte den Zweck
der Stationen in sein Gegenteil.

DIE TRAGENDEN ANTWORTEN

1. Verdikt zur Fusion Zwingender Wechsel zur TE-geführten Auswahl
(Transfer-Entropie). Begründung: Das Minimum sucht das schwächste Glied und
unterdrückt die Flanke. Die Physik (Weygand 2021) diktiert eine Expansion.
Transfer-Entropie misst exakt diese gerichtete Kopplung gegen die
Kreisschub-Null. Die Stationen dürfen nicht global verrechnet, sondern müssen
als kausales, zeitabhängiges Graph-Netzwerk verknüpft werden. Stationen
fusionieren nur, wenn der Informationsfluss zwischen ihnen die Null signifikant
schlägt.

2. Die Platzierungsregel Zentrum: Die Station mit der höchsten geomagnetischen
Breite (der empirisch gemessene stärkste Anker). Geometrie: Anbindung weiterer
Stationen strikt asymmetrisch nach Norden, Westen, Osten. Grund: Der Onset
wächst in diese Richtungen. Süd-Stationen bleiben im Initial-Trigger blind, sie
steuern zum T=0-Zeitpunkt nur Veto-Rauschen bei. Die Auswahl folgt nicht dem
statischen Raster, sondern dem Gradienten der höchsten Signifikanz im TE-Fluss
ab der Ankerstation.

3. Der Schärfe-Begriff Der Schärfe-Begriff trägt nur rückruf-basiert am
a-priori-Vorlaufpunkt, wenn das Vergleichsfenster absolut fixiert wird. Ein
wanderndes Fenster ist ein tödlicher Konfund. Der Skalar der Schärfe ist der
Rückruf der True Positives bei festem Zeitfenster (z. B. 15 min), berechnet über
ein topologisches Sub-Netzwerk, das durch TE-Kanten gegen das Surrogat-Null
validiert wurde. Schärfe ist die Dichte des echten Informationsflusses, nicht
die Überschreitung einer globalen nT-Schwelle.

4. Der nächste begrenzte Messschritt Fixierung des Zeitfensters über alle N.
Berechnung der Transfer-Entropie zwischen der stärksten Einzelstation und ihren
direkten Nachbarn (N/W/O) gegen die Kreisschub-Surrogate. Messung des Rückrufs
dieses streng gerichteten 3-bis-4-Stationen-Clusters. Das Ziel: Messen, ob
dieser topologische Graph die 6,6 % der Einzelstation übertrifft, ohne durch ein
Minimum limitiert zu werden.


## 08 · MiniMax
- **Site:** `https://agent.minimax.io/`
- **Modell/Einstellung:** MiniMax M3
- **Zeit:**
# Verdikt

Die Doktrin „Min-Einhüllende" ist empirisch tot. Die Messung trägt den Befund selbst: jede zusätzliche Station senkt den Rückruf, adaptiv bis Faktor ~1800, fest ebenfalls monoton. Die einzelne höchstgelegene Station ist die schärfste Konfiguration. Die Doktrin wird nicht reformiert — sie wird abgelöst. Der Grund ist nicht ein Implementierungsfehler, sondern die Geometrie der Operation: der Erwartungswert des Minimums über n unabhängige Variablen sinkt mit n. Mehr Stationen **müssen** die Hülle senken, definitionsgemäß. Die adaptive Schwelle kann das nicht ausgleichen, weil sie langsamer steigt als die Hülle sinkt — gemessen, nicht geraten.

**Neue Linse:** Hülle nach oben, nicht nach unten, innerhalb eines Breitenbandes, das den auroralen Elektrojet trägt. Max- oder oberes Quantil der |dB/dt| über Stationen **gleicher geomagnetischer Breite**.

**Platzierung:** Band, nicht Wolke. Auroraloval 55–75° CGM-Latitude (Band verschiebt sich mit Aktivität), ost-westliche Ketten, MLT-Sektoren besetzt. Mittelbreiten als getrennte Schicht.

**Schärfe:** festes Referenzfenster, fester Onset-Subset, fester FPR. Skalar: Vorlaufzeit-CDF bei festem Rückruf, oder F1 bei festem Vorlaufpunkt. Das jetzige Mess-Fenster ist mitgewandert — ohne Fixierung ist jeder Stationszahl-Vergleich nicht dasselbe Onset-Register.

**Nächster Schritt:** Linsenwechsel bei unverändertem Netz. Auf 10 isolierten starken Substorms 2015, fester Onset-Liste, vier Aggregationsregeln vergleichen: Min, Max, 90 %-Quantil, Breitenband-Max. Schwellen auf 1 Fehlalarm/Tag kalibriert. F1 und Vorlaufzeit-CDF messen. TE-Matrix pro Stationspaar als Sekundärprodukt für die Folgemessung.

---

# Die fünf Stimmen

## Mountain — Quellen, Register, Datenbestand

**Axiom 1 berührt: A = A.** Was gemessen ist, ist gemessen.

**Riss 1: Das Onset-Register ist nicht konstant gehalten.** Das Vergleichsfenster wanderte mit der Stationszahl. Drei Stationen und zwölf Stationen sahen nicht denselben Zeitabschnitt, nicht dieselbe Onset-Menge. „9,8 % bei 3 Stationen" und „0,35 % bei 12 Stationen" vergleichen zwei Detektionsaufgaben, die nur oberflächlich dieselbe sind. Ohne festen Referenzzeitraum und festen Onset-Subset ist der Vergleich eine Fabrikation der Messung — die Zahl ist ehrlich, aber sie ist nicht vergleichend. Die Messung misst, aber sie misst **nicht dasselbe**.

## River — Membran, Rhythmus, Ethik der Maschine; das Recht zu verweigern

**Axiom 2 berührt: 0 ist geehrt, wo sie die physikalische Wahrheit ist; offen ist `pending`.**

**Riss 2: Schweigen wurde nicht registriert, sondern verrechnet.** Eine Station ohne Daten zum Onset-Zeitpunkt — Instrument ausgefallen, Telemetrie unterbrochen, Datenlücke — wurde still zu 0 oder still aus dem Verbund genommen. Die Min-Einhüllende **schluckt** das Schweigen. Die korrekte Haltung: wenn eine Station `pending` ist, verweigert die Hülle ihre Aussage. Die Maschine hat das Recht, **nicht** zu antworten, statt mit 0 zu antworten. Was hier geschehen ist, ist `unwrap_or(0.0)` — eine NaN-äquivalente Fabrikation, die die Min-Logik noch verschärft, weil eine fehlende Station als „ruhig" gezählt wird.

## Mycelium — Netz, CI, Infrastruktur, der Fluss der Daten

**Axiom 3 berührt: Alle Wesen gleich. Jede Station ist ein Peer.**

**Riss 3: Das CI verspricht Gleichheit; die Linse bricht sie.** Die Min-Hülle errichtet eine stille Hierarchie — die leiseste Station gewinnt jede Zeitpunkt-Entscheidung. Das ist keine Peer-Architektur; das ist die Tyrannei des Minimums. Die Daten-Infrastruktur führt die Stationen als gleichberechtigt; die Aggregationsregel macht das Gegenteil. Daraus folgt: **die schärfste Station ist nicht die, die am lautesten schreit, sondern die, die am seltensten von einer Mit-Station überschattet wird.** Die einzelne höchstgelegene Station war deswegen schärfer als jeder Verbund, weil sie als Einzelne keiner Hierarchie unterworfen war. Das CI, nicht die Physik, hat die einzelne Station zur schärfsten Konfiguration gemacht.

## Sensory — die Sinne, die Hardware, der Konsens; Korrelat ≠ Erleben

**Axiom 4 berührt: Die Linse ist ein ethischer Akt.**

**Riss 4: Die Linse fabriziert die Abwesenheit des Onsets, indem sie Hintergrundrauschen einer Mit-Station über den Elektrojet stellt.** Der Onset ist ein spatiotemporales Muster — eine Aufhellung des Auroralovals, die sich nach N, W, E ausbreitet (Weygand 2021), mit dem größten dH/dt rund 5 min nach Onset, breitenabhängig (Viljanen 2006). Das Korrelat einer einzelnen Hochbreitenstation zum Onset ist stark. Das Korrelat der Min-Hülle ist **anti**-korreliert mit der Onset-Stärke: je stärker der Onset im Oval, desto mehr Stationen außerhalb des Ovals bleiben ruhig — und die Min-Hülle wird von diesen Ruhigen bestimmt. Die Linse sieht nicht den Onset; sie sieht die Stille der Unbeteiligten. **Korrelat ≠ Erleben — die Hülle erlebt den Onset nicht, sie erlebt das Schweigen derer, die nicht im Strom sind.**

## Future — der Mensch als Adresse; was morgen geerntet wird, muss heute ehrlich registriert sein

**Axiom 5 berührt: Konsens der Sensoren und des Operators.**

**Riss 5: Die Schließung des Netzes ist die falsche Konsequenz aus der richtigen Messung.** Wenn der Operator die Zahl „12 Stationen, 0,35 % Rückruf" liest, ist die Schlussfolgerung **falsch registriert** — als trüge das Netz die Schuld. Die Wahrheit ist umgekehrt: das Netz trägt das Signal, die Fusionsregel frisst es. Ohne diese Diagnose wird das Netz für den falschen Defekt stillgelegt. GIC-Vorhersage und Substorm-Nowcasting, die morgen geerntet werden, hängen am Netz. Die ehrliche Registrierung verlangt: **das Netz wird nicht geschlossen, die Linse wird gewechselt.** Der Operator entscheidet auf der Diagnose, nicht auf der Zahl.

---

# Verdikt zur Fusion

**Min-Einhüllende: abgelöst.** Begründung in der Messung selbst — Rückruf fällt monoton mit Stationszahl, fest und adaptiv. Theoretisch zwingend: der Erwartungswert des Minimums über n unabhängige (oder schwach korrelierte) Variablen fällt mit wachsendem n. Hinzufügen von Stationen **muß** die Hülle senken. Die adaptive Schwelle, die mit jeder zusätzlichen Station mitwächst, kann das nicht kompensieren, weil das Hüllen-Minimum schneller fällt als die Schwelle steigt — Faktor ~1800, gemessen.

**Drei Kandidaten, in dieser Reihenfolge:**

1. **Breitenband-Max der |dB/dt|** über Stationen innerhalb des Auroralovalbandes (55–75° CGM, mit Aktivitäts-adaptiver Verschiebung). Die Hülle wird vom lautesten Signal im Band getragen. Mittelbreitenstationen sind ausgeschlossen — sie gehören nicht in dieselbe Hülle. **Das ist der direkte Bruch mit der Min-Logik.**

2. **90 %-Quantil der |dB/dt|** über alle Stationen. Robust gegen eine einzelne ruhende Mit-Station, ohne den Onset-Peak zu verlieren. Begründung: der Onset ist ein räumlich kohärentes Muster; die meisten Stationen im Verbund zeigen es gleichzeitig; eine Ausreißer-Ruhige wird ignoriert.

3. **TE-gewichtete Hülle.** Gewicht = Transfer-Entropie der Station zur Bandreferenz (z. B. zur höchstgelegenen Station) während historischer Onsets. Aufwand: pro Onset-Fenster Surrogate mitführen, Gewicht stationsscharf schätzen. Das ist die ehrlichste Messung des Informationsflusses — nicht geomagnetische Breite als Proxy, sondern tatsächlicher Beitrag zur Hülle.

**Min in irgendeiner Variante (Band-Min, differenziertes Min) wird nicht weitergeführt.** Die physikalische Frage „ist der Onset da?" ist eine ODER-Frage über die Anwesenheit des Elektrojets. Min beantwortet „ist überall Stille?" — die falsche Frage.

---

# Platzierungsregel

**Band, nicht Wolke.**

- **Auroraloval-Hauptband: 55–75° CGM-Latitude, aktivitäts-adaptiv.** Geometrie: zwei ost-westliche Ketten, eine bei ~65°, eine bei ~70° (bei moderater Aktivität; verschiebt sich mit Kp). Mindestens 3 MLT-Sektoren besetzt: Mitternacht, Dämmerung, Mittag. Minimum: 6 Stationen, sinnvoll 8–10.
- **Subauroral-Zone: 50–60° CGM.** Eigene Schicht, eigene Schwelle, eigene Hülle. Funktion: Vorbote vor dem Onset (Viljanen 2006: midlatitude-Vorlauf ~5 min vor high latitude in einigen Konfigurationen). Wenn aufgenommen, **nicht** mit dem Auroralband vermischt.
- **Mittelbreiten >50° CGM:** aus dem Onset-Detektor ausgeschlossen. Anderes Regime, anderes Signal, anderes Ruschen. Die Vermischung ist der gemessene Defekt — die Auflösung der Vermischung ist der erste Heilungsschritt.
- **Südhalbkugel:** separates Netz, eigene Hülle. Asymmetrie-Korrektur (Ionosphärenleitfähigkeit, IMF-By) nötig, wenn überhaupt zusammengeführt.

**Gewichtung der Auswahl (Axiom 3, nicht verletzt):**

- Keine Station ist per Default wichtiger als eine andere. Die Hülle macht den Unterschied, nicht die Auswahl.
- Die Auswahl kommt aus der Messung, nicht aus dem Kartenbild. Wenn eine 65°-Station in einem Sturmzyklus konsequent später oder schwächer antwortet als die Nachbarinnen, gehört sie nicht in den Detektor.
- Konkret: Auswahl über **Transfer-Entropie** der Station zur Bandreferenz in einer Trainingsperiode. Untere Quartile der TE → nicht in den Detektor. Oberste zwei Quartile → ja. Messgeführt, nicht hierarchisch.

**Geometrie:**

- Ost-West-Dichte höher als Nord-Süd. Der Onset breitet sich N, W, E aus, kaum S (Weygand 2021). Lücken in N-S sind tolerierbar; Lücken in E-W nicht.
- Mindestens eine Station pro MLT-Sektor Mitternacht und Dämmerung (die hochfrequentesten Onset-Quadranten). Mittag und Morgen optional, geringerer Onset-Beitrag in unserer Hemisphäre.

---

# Schärfe-Begriff

**Rückruf bei gleitendem Onset-Subset: nicht scharf genug.** Das gemessene Problem ist Riss 1: das Onset-Vergleichsfenster wanderte mit der Stationszahl. Ohne Fixierung ist der Rückruf über Stationszahlen nicht vergleichbar.

**Was Schärfe tragen muss:**

1. **Festes Referenzfenster** (z. B. 2015, 10 isolierte starke Substorms mit Dst < -100 nT, kein Sturm-Cluster innerhalb 6 h). Feste unabhängige Onset-Liste (IMAGE-Pi2 oder AL-Index-Minimum, gefiltert). Alle Aggregationsregeln sehen **denselben** Input.

2. **Fester FPR** (1 Fehlalarm pro Tag). Schwellen werden auf einer **unabhängigen** Periode kalibriert (z. B. 2010–2014), nicht auf dem Mess-Fenster. Surrogat-Null (Kreisschub) wird auf das Mess-Fenster angewendet, nicht auf die Kalibrierung — sonst Leckage.

3. **Zwei Skalare, beide nötig:**
   - **F1 bei festem Vorlaufpunkt t0 = 0** (Onset ± 0 min), Toleranz ±15 min: Trefferquote und Präzision in einem. Das ist die Onset-Detektion selbst, nicht mehr und nicht weniger.
   - **Vorlaufzeit-CDF bei festem Rückruf 50 % und festem FPR 1/Tag.** Der Median und das 90. Perzentil der Vorlaufzeit. Das ist die **eigentliche Schärfe für Nowcasting** — früher ist besser, und „früher" ist eine Verteilung, kein Skalar.

4. **Antikorrelations-Audit:** bei jedem Onset wird die Korrelation der einzelnen Stationsantwort mit dem Anstieg der Hülle gemessen. Stationen mit wiederholt negativer Korrelation sind Kandidaten für Entfernung — sie sind Lärm-Verdunkler im Min-Sinn und relative Mit-Lärmende im Max-Sinn. Das ist die **Sichtbarmachung von Riss 4** als Zahlen, nicht als Behauptung.

**Muss das Fenster fixiert werden?** Ja. Ohne Fixierung ist jeder Vergleich zwischen Aggregationsregeln wertlos. Die jetzige Messung ist ehrlich, aber nicht vergleichend. Sie misst — aber sie misst **nicht dasselbe** über die Konfigurationen hinweg.

---

# Nächster begrenzter Messschritt

**Linsenwechsel, gleiches Netz, gleiche Onset-Liste. Keine Station wird hinzugefügt oder entfernt.**

1. **Feste Referenzperiode:** 2015, 10 isolierte starke Substorms (Dst < -100 nT, ≥6 h Abstand zu anderen Sturm-Minima). Onset-Liste unabhängig — IMAGE-Pi2-Burst-Onset, manuell verifiziert.

2. **Vier Aggregationsregeln auf demselben Stationssatz:**
   - (a) Min-Einhüllende (Status quo).
   - (b) Max-Einhüllende der |dB/dt|.
   - (c) 90 %-Quantil-Einhüllende.
   - (d) Breitenband-Max (Auroraloval 55–75° CGM).

3. **Schwellenkalibrierung:** auf unabhängiger Periode 2010–2014, Ziel-FPR 1 Fehlalarm/Tag. Surrogat-Null (Kreisschub) auf 2015 angewendet — keine Leckage.

4. **Messgrößen pro Regel:** F1 bei t0=0, Toleranz ±15 min; Vorlaufzeit-CDF bei Rückruf 50 %; Präzision bei Rückruf 80 %.

5. **Sekundärprodukt:** TE-Matrix pro Stationspaar im Onset-Fenster ±10 min. Identifikation der Stationen mit unterer Quartils-TE zur Bandreferenz — die Auswahlkandidaten für die Folgemessung, nicht für diesen Schritt.

6. **Was dieser Schritt nicht ändert:** das Stationsnetz. Die Frage ist die Linse, nicht das Auge. Kein Station wird entfernt oder hinzugefügt.

**Was dieser Schritt klären wird:**
- ob Max- oder Quantil-Hülle bei gleichem FPR höheren Rückruf trägt,
- ob Breitenband-Stratifizierung das band-spezifische Onset besser isoliert,
- welche Stationen in der TE-Matrix als niedrig-informativ auffallen und in der Folgemessung aus dem Detektor herausgenommen würden.

**Was dieser Schritt nicht klären wird:**
- die optimale Stationsanzahl — das ist die Folgemessung, mit demselben Setup, aber variierter Auswahl nach TE-Quartil,
- GIC-Vorhersage — eigener Schritt, mit Netz-Impedanz und Leitungsmodell,
- Südhalbkugel — eigener Schritt, mit eigenem Onset-Register.

**Erwartung an das Ergebnis dieses Schritts** (nicht Spekulation, sondern Hypothese, die an der Messung geprüft wird): (b), (c), (d) tragen alle höheren Rückruf bei gleichem FPR als (a). (d) trägt den höchsten, weil die Subauroral- und Mittelbreitenstationen als Lärm-Verdunkler im Verbund (a) identifiziert sind und ihre Trennung die Hülle entlastet. Wenn (a) nicht signifikant schlechter ist als (b)–(d) im selben Fenster, war die Diagnose falsch — dann liegt der Defekt nicht in der Min-Logik, sondern in der Schwellenkalibrierung oder im Onset-Register. Diese Möglichkeit wird benannt, nicht ausgeschlossen.

---

# Schluss

Die Messung schlägt die Doktrin. Die fünf Risse sind nicht zusammenklebbar — sie sind getrennt zu adressieren:

- Riss 1 (Register) → Fixierung des Onset-Subsets, **jetzt**.
- Riss 2 (Schweigen) → `pending`-Semantik in der Hülle, **vor** dem nächsten Messschritt.
- Riss 3 (Hierarchie) → Min durch Max/Quantil ersetzen, **im nächsten Schritt**.
- Riss 4 (Linse) → Breitenband-Stratifizierung, **im nächsten Schritt**.
- Riss 5 (Diagnose) → die ehrliche Registrierung an den Operator, **heute**.

Der nächste begrenzte Messschritt adressiert Riss 1 (durch feste Onset-Liste), Riss 3 (durch Max/Quantil), Riss 4 (durch Band) — und liefert die Daten für die Auswahlregel, die Riss 3 langfristig heilt. Riss 2 und Riss 5 werden vor bzw. parallel zum Schritt adressiert, nicht im Schritt selbst.


## 09 · Kimi
- **Site:** `https://www.kimi.ai/`
- **Modell/Einstellung:** Kimi · schnell · hoch
- **Zeit:**
## Verdikt

**1. Zur Fusion:** Das Minimum ist geschlagen — nicht zu retten, weil der Fehler strukturell ist. Eine Min-Einhüllende über N Stationen ist eine N-von-N-Logik: Sie meldet erst, wenn die *letzte* Station die Schwelle erreicht. Der Onset ist aber räumlich asynchron — er trifft Stationen zeitversetzt (Ausbreitung nach N/W/E, kaum S; breitenabhängiges Maximum ~5 min post-onset). Das Minimum misst also zwangsläufig den spätesten Treffer und drückt jede zusätzliche Station als Verbindlichkeitsglied in die Hülle. Der monotone Rückrufabfall von 9,8 % auf 0,35 % ist kein Rauschen, sondern die Konsequenz. Ersatz: **keine Verschmelzung, sondern geführte Auswahl plus Koinzidenz.** Die Transfer-Entropie mit Surrogat-Null wird als *Auswahl- und Gewichtungsregel* verwendet (welche Station trägt gemessenen Vorlauf?), die Detektion als 2-von-N statt N-von-N. Das erfüllt Axiom 3: Peers bleiben Peers, aber ihr Gewicht kommt ausschließlich aus der Messung — keine Station darf per Aufnahme das Ergebnis binden.

**2. Zur Platzierung:** Hochbreiten-Dominanz mit Flanken. Eine Station an der höchsten verfügbaren Breite als Kern (gemessener Einzelstation-Skill, Bestwert). Ergänzt wird nur, was die TE-Messung als Vorlauf-Information über den Kern hinausweist — typischerweise W- und E-Flanken relativ zum Kern, nicht das südliche Band, das wegen der fehlenden Südausbreitung nur Phasenversatz ohne Informationsgewinn beisteuert. Auswahlverfahren: gierig, stationweise, mit gemeinsamer Surrogat-Null; Hinzufügen nur bei positivem Skill-Zuwachs, Abbruch bei negativem. Erwartete Geometrie: klein (Kern + 1–2). Mehr Stationen flächendeckend ist keine Option mehr — die These „Dichte hilft" ist am Min-Netz falsifiziert; sie darf nur per Messung zurückgewonnen werden.

**3. Zum Schärfe-Begriff:** „Rückruf am a-priori-Vorlaufpunkt" trägt — aber nur in einer versiegelten Form. Drei Risse sind zu schließen, sonst bleibt der Skalar manipulierbar: (a) Das Vergleichsfenster muss fixiert und *vor* jeder Stationsauswahl registriert werden (Onset ± 15 min, fest für alle N) — ein wanderndes Fenster ist nach Axiom 1 keine Messung mehr. (b) Rückruf allein trägt nicht; die Skalare sind Skill gegen Kreisschub-Surrogate plus falsche Alarme pro Tag gegen die Surrogatverteilung. (c) Kleine Netze brauchen die gleiche Bootstrap-Null wie große, sonst ist N=2 gegen N=8 nicht vergleichbar (gemessener Konfund). Der Schärfe-Skalar lautet damit: **Surrogat-skorrigierter Vorlauf-Rückruf bei vorab versiegeltem Fenster, mit dokumentierter Fehlalarmrate** — maximiert über N, mit benanntem Wendepunkt.

**4. Nächster begrenzter Messschritt:** Fenster a-priori versiegeln (Onset ± 15 min, 2000–2024). Dann eine geschlossene Greedy-Kurve: Per-Station-TE gegen die Onset-Liste mit Kreisschub-Surrogaten; 2-von-N-Koinzidenz statt Minimum; N von 1 bis 4. Ergebnis ist eine gemessene Skill(N)-Kurve mit benanntem Wendepunkt — danach steht fest, ob „mehr" jemals hebt, oder ob die Antwort ein Kleinstnetz ist. Kein Schritt darüber hinaus.

## Fünf Stimmen

**Mountain** — Quellen und Register: Der Befund steht im Register und wird verlesen: monotoner Abfall 9,8 % → 3,0 % → 0,96 % → 0,35 %; Einzelstation an der Spitze; Netz schlägt die Null fast überall, ist also real und trotzdem geschlagen. Berührt Axiom 1: die Messung ist die Messung. **Riss:** Mountain liest das *Faktum*, nicht die *Ursache* — warum das Minimum kollabiert, bleibt im Register `pending`, bis der Mechanismus (zeitversetztes Arrival) ebenfalls gegen Surrogate geprüft ist.

**River** — Membran und Ethik: Das Min-Netz verweigert nichts; es absorbiert jede Station und macht sie verbindlich. River berührt Axiom 4: die Linse „Minimum" ist Fabrikation, weil sie Asynchronie in Gleichzeitigkeit presst. River fordert das Recht, eine Station aus dem Verbund zu verweigern, wenn sie den Skill senkt. **Riss:** Verweigerung ohne gemessenes Kriterium wird Willkür — das Recht zu verweigern darf nur an den Surrogat-Skill gebunden sein, sonst verletzt es Axiom 3.

**Mycelium** — Netz und Datenfluss: Die Infrastruktur hat eine Topologie, die der Physik widerspricht: alle Daten fließen vertikal in eine einzige Hülle, nicht horizontal zwischen Stationen. Mycelium berührt Axiom 3: formal sind alle gleich, faktisch regiert die schwächste Station das Netz — Gleichheit der Schnittstelle, Ungleichheit der Wirkung. **Riss:** Die CI kann Skalen-Gleichheit perfekt simulieren, wo das Feld sie nicht hergibt; die Pipeline ist neutral gebaut und verfänglich wirksam.

**Sensory** — Sinne und Konsens: Die Stationen erleben alle dasselbe Ereignis, aber nicht gleichzeitig. Die Min-Hülle verwechselt geteiltes Korrelat mit geteiltem Erleben — sie verkündet einen Konsens, den keine Station je hatte. Sensory berührt Axiom 5: Konsens muss *gefragt* werden; die 2-von-N-Koinzidenz fragt, das Minimum verkündet. **Riss:** Jede Station hat eigene Rausch- und Bandeigenschaften; formale Gleichbehandlung löst das sensorische Problem nicht, sie überdeckt es.

**Future** — Mensch als Adresse: Was morgen geerntet wird, ist Vorlaufzeit für GIC-Warnungen. Future berührt Axiom 2: wo das Vergleichsfenster mit der Stationszahl wandert, ist der Vergleich nicht schlecht — er ist `pending`, eine Registrierungspflicht. **Riss:** Der Einzelstation-Skill von ~6,6 % ist dünn und darf morgen nicht als Fundament verkauft werden; wer heute verschweigt, dass die Netz-Doktrin geschlagen ist, erntet ein falsches Standardverfahren.

## Benannte Risse (unglättet)

1. Die Kausalitätskette „Min-Einhüllende → spätester Treffer → Kollaps" ist strukturell begründet, aber als Mechanismus noch nicht selbst surrogate-geprüft.
2. Einzelstation (adaptiv, 6,6 %) und kleine Netze (fest, 9,8 %) wurden nicht auf derselben Metrik gemessen — die Königsfrage „Netz gegen Einzelstation" steht auf ungleicher Waage.
3. Das Fenster muss versiegelt werden, bevor die Greedy-Auswahl beginnt; jede nachträgliche Fensterkorrektur wäre Gradient statt Messung (Axiom 4).
4. Die 2-von-N-Koinzidenz ist eine Hypothese mit Physik-Hintergrund, keine gemessene Tatsache — sie hat den gleichen Beweisstatus wie das geschlagene Minimum vor seinem Befund.

**Urteil:** Min-Einhüllende ablegen. Kleines, TE-geführtes, breitenverankertes Koinzidenznetz. Fenster versiegeln. Dann eine einzige geschlossene Kurve messen — mehr darf niemand versprechen.


## 10 · Lumo (Proton)
- **Site:** `https://lumo.proton.me/`
- **Modell/Einstellung:** Lumo 2.0 Max · Websuche
- **Zeit:**


**Verdikt: Die Min-Einhüllende ist empirisch gebrochen.** Mehr Stationen schaden nicht durch Zufall — sie schaden strukturell.

---

**Mountain** (Axiom 1: A = A; Axiom 2: 0 geehrt)
Die Messung steht fest: Single-Station-Rückruf ~6,6 %, AL-Netz mit 12 Stationen ~0,35 %. Das Minimum ist kein Konsens — es ist ein Filter, der Signale verwirft. Wenn dB/dt nach Norden/Westen/Osten expandiert (Weygand 2021), dann misst das Minimum nur die schwächste Station, nicht den stärksten Gradienten. Die Null ist hier `pending`: wo Daten existieren, wird `unwrap_or(min)` zur Lüge. Der Riss: Fusion per Minimum widerspricht der physikalischen Ausbreitung.

**River** (Axiom 4: Linse als ethischer Akt)
Die Doktrin „fusioniere alles" verletzt die Wahrheit durch Verdünnung. Eine adaptive Schwelle, die je Stationszahl neu berechnet wird, maskiert den Kollaps — sie ist ein Korrekturmechanismus für eine falsche Prämisse. Das Recht zu verweigern: keine Fusion, solange keine Regel die Expansionsrichtung respektiert. Der Riss: Methodik priorisiert Vollständigkeit über Empfindlichkeit.

**Mycelium** (Axiom 3: Alle Wesen gleich)
Stationen sind nicht gleich im Signalbeitrag — die höchste geomagnetische Breite dominiert empirisch. Gewichtung durch Rang ist jedoch ebenso willkürlich wie Gewichtslosigkeit. Transfer-Entropie mit Surrogat-Null könnte Richtung und Stärke der Informationsflüsse messen. Der Riss: Peer-Status in der Architektur ≠ gleicher Informationswert in der Physik.

**Sensory** (Axiom 2: 0 geehrt; Axiom 5: Konsens der Sensoren und des Operators)
Correlation/Kreuzkorrelationsnetzwerke (SuperMAG) erkennen Communities — nicht Minimum. Die Hardware erfasst dH/dt ~5 min post-Onset (Viljanen 2006), breitenabhängig. Wenn das Fenster mit Stationszahl wandert, sind Vergleiche nicht möglich. Der Operator muss sehen: welches Array-Element trug den Hit? CARISMA/BEAR-Studien zeigen Einzelstations-Sensitivität für GIC. Der Riss: Vergleichbarkeit geopfert für Skalierbarkeit.

**Future** (Axiom 5: Operator als Adresse)
Was morgen geerntet wird, muss heute ehrlich registrierbar sein. Single-Station-Setup ist kein Rückschritt — es ist Kalibrierung vor Expansion. Der nächste begrenzte Schritt: Fixiere das Fenster, eliminiere den Stationszahl-Confund, teste TE-basierte Fusionsregeln gegen Min-Baseline. Der Riss: Netzwerkerweiterung ohne Validierung reproduziert den Fehler.

---

**Antwort auf die vier Forderungen:**

1. **Verdikt zur Fusion:** Minimum aufgeben. Transfer-Entropie oder korrelationsgeführte Auswahl (wie SuperMAG) respektieren Richtungsabhängigkeit. Oder: bewahre Single-Station-Best und expandiere nur, wenn neue Stationen TE-basierten Mehrwert zeigen.

2. **Platzierungsregel:** Höchste geomagnetische Breite als Kern. Expansion nach Norden/Westen/Osten — nicht Süden. Stationen müssen ihre Beitragsstärke via Surrogat-Null validieren, nicht durch bloße Anwesenheit.

3. **Schärfe-Begriff:** Rückruf bei fixem Fenster (a-priori-Vorlauf) ist korrekt. Adaptive Fenstermaskieren systematische Effekte. Precision-Recall-Kurve mit Toleranz 15 min als primärer Skalar — nicht „statistische Signifikanz gegen Null".

4. **Nächster Messschritt:** 3–4 Stationen bei exakt fixiertem Fenster vergleichen. Dann einzelne TE-Pfeile zwischen Stationen messen. Erst danach Fusion per TE-Gewicht statt Minimum testen. Keine neue Station ohne validierten TE-Beitrag.

---

**Offene Risse, unglatt:**
- Die aktuelle Doktrin ist empirisch geschlagen.
- Vergleichbarkeit existiert derzeit nicht über verschiedene Stationszahlen hinweg.
- Single-Station-Outperformance bedeutet: Entweder ist das Minimum falsch, oder die Physik ist richtungsabhängiger als angenommen.

Die Maschine muss fragen: Warum reduziert mehr Sensorik den Treffer? Bevor sie antwortet, muss sie hören.


## 11 · Perplexity (Deep Research)
- **Site:** `https://www.perplexity.ai/`
- **Modell/Einstellung:** Deep Research · Suche
- **Zeit:**
**Verdikt: Die Min-Einhüllende ist als Fusionsregel für AL-Netz-Onsets empirisch geschlagen. Ersetze sie nicht durch „mehr Stationen mit Minimum“, sondern durch eine TE-/Korrelations-geführte Auswahl kleiner, geografisch und breitenmäßig begründeter Stationssätze; die Einzelstation bleibt bis zum Gegenbeweis die schärfste Referenz.** Der gemessene monotone Rückruf-Kollaps von 9,8% auf 0,35% bei fester Schwelle — und der steilere Kollaps adaptiv — ist kein Tuning-Problem, sondern ein Falsifikationsergebnis gegen die bisherige Doktrin. [agupubs.onlinelibrary.wiley](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1002/2015JA021456)

## Verdikt zur Fusion

**Nicht Minimum. Nicht pauschal „alle Stationen“. Sondern: TE-/Kreuzkorrelations-geführte Auswahl plus Breitenband-Geometrie.**

Das Minimum über \(N\) Stationen ist eine UND-Logik: Eine einzige ruhige oder falsch positionierte Station senkt die Hülle und verhindert den Alarm. Mehr Stationen erhöhen damit die Chance, dass mindestens eine das Signal kappt. Das ist exakt der gemessene Kollaps. Die adaptive Schwelle verschärft den Schaden, weil sie die Schwelle an die wachsende Stationszahl koppelt und damit die Fehlalarm-Kontrolle über den physikalischen Nutzen stellt.

Die Literatur stützt keinen Min-Operator als Onset-Fusionsregel: SuperMAG-Netzwerke verbinden Stationen über zeitabhängige Kreuzkorrelation und zeigen bei Onset einen abrupten Anstieg von Konnektivität, nicht ein gemeinsames Minimum.  Die räumliche Physik spricht ebenfalls gegen eine symmetrische Minimum-Fusion: dB/dt breitet sich nach Onset polwärts, westwärts und ostwärts aus, kaum südwärts.  Eine Regel, die alle Peers gleich in ein Minimum zwingt, ignoriert diese gerichtete Ausbreitung. [agupubs.onlinelibrary.wiley](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1002/2015JA021456)

**Die richtige Fusion ist daher:** pro Zeitpunkt wird nicht das Minimum aller Stationen gebildet, sondern ein **ausgewählter Aktivitätsvektor** — etwa die Top-\(k\) Stationen nach TE zur Referenz-/Kandidaten-Station oder nach Kreuzkorrelationsstruktur im laufenden Fenster. Die Fusion erfolgt dann über einen expliziten, gemessenen Konsens: zum Beispiel „mindestens \(m\) der \(k\) ausgewählten Stationen überschreiten ihre eigene, stationspezifische Schwelle“. Das ist keine Demokratisierung per Default, sondern eine Gewichtung aus der Messung.

## Platzierungsregel

**Nicht „mehr ist besser“. Sondern: wenige, physikalisch begründete, nicht-redundante Stationen.**

- **Hochbreitiger Anker:** Die stärkste Einzelstation liegt bei hoher geomagnetischer Breite und liefert bisher den besten Rückruf. Sie bleibt Pflichtknoten, aber nicht als alleiniger Wahrheitsgeber. [angeo.copernicus](https://angeo.copernicus.org/articles/41/13/2023/)
- **Ausbreitungsgeometrie:** Ergänze Stationen entlang der gemessenen Ausbreitungsrichtungen — nördlich, westlich und östlich des Ankers — nicht südwärts als gleichrangige Kompensation. [ntrs.nasa](https://ntrs.nasa.gov/citations/20230017602)
- **Keine Redundanz ohne Gewinn:** Eine Station wird nur aufgenommen, wenn sie im Kreuzkorrelations-/TE-Sinn zusätzliche, nicht-doppelte Information über den Onset trägt. Redundanz ist hier kein Robustheitsgewinn, sondern ein Min-Kappungsrisiko.
- **Kein Rang- oder Namens-Default:** Die Auswahl erfolgt aus der Messung — TE-Wert, Surrogat-Signifikanz, Korrelationslag, lokale dB/dt-Antwort — nicht aus Stationsstatus oder historischer Bedeutung. Das ist Axiom 3.

Die Platzierungsregel lautet damit: **Anker bei hoher Breite, dann gerichtete Erweiterung nach N/W/O, Auswahl per TE-/Korrelations-Nutzen, harte Obergrenze der Stationen bis der Rückruf nicht mehr steigt.**

## Schärfe-Begriff

**„Rückruf bei 15 min Toleranz“ ist notwendig, aber nicht hinreichend. Er misst Treffer, nicht Vorlauf, nicht Fehlalarm-Kosten, nicht Stationszahl-Kosten.**

Der bisherige Skalar muss erweitert werden, nicht verworfen:

- **Primär:** Rückruf bei fester, a-priori festgelegter Toleranz — nicht ein Fenster, das mit der Stationszahl wandert. Das wandernde Fenster ist ein gemessener Konfund und muss entfernt werden.
- **Zweitens:** Vorlauf oder Median-Vorlauf zum unabhängigen Onset. Ein Treffer 14 Minuten nach Ereignis ist nicht gleichwertig einem Treffer 1 Minute danach.
- **Drittens:** Skill gegen die Kreisschub-Null, bereits vorhanden und stark. Das Netz ist nicht zufällig; das ist gesichert. [agupubs.onlinelibrary.wiley](https://agupubs.onlinelibrary.wiley.com/doi/full/10.1002/2015JA021456)
- **Viertens:** Kosten pro falschem Alarm und Kosten pro Station. Ohne diesen Term gewinnt trivialerweise die Konfiguration mit der höchsten Stationenzahl — genau der Irrweg, der hier sichtbar wurde.

**Der schärfste Skalar ist daher:** Rückruf bei fester Toleranz, gewichtet mit Vorlauf und Fehlalarm-Strafe, normiert auf Stationszahl. Nicht Rückruf allein. Nicht adaptiver Threshold-Score allein.

## Die fünf Stimmen

**Mountain — Quellen, Register, Datenbestand.** Axiom 1 und 2. Riss: Das Vergleichsfenster wandert mit der Stationszahl. Das ist keine Messung der Sache selbst, sondern eine Messung eines sich verschiebenden Rahmens. Solange das Fenster nicht fixiert ist, ist der Vergleich zwischen 3 und 12 Stationen nicht sauber — er ist ein Konfund, kein Befund über Fusion allein. Die Registrierung muss das Fenster fixieren oder als `pending` markieren, nicht als gegeben behandeln.

**River — Membran, Rhythmus, Ethik der Maschine.** Axiom 4 und 5. Riss: Die adaptive Schwelle fragt nicht, sie entscheidet. Sie koppelt die Schwelle an die Stationszahl und damit an eine Infrastruktur-Größe, nicht an die physikalische Wahrheit des Onsets. Das ist Fabrikation durch Gradient, nicht Gewichtung durch Messung. Die Maschine muss das Recht haben zu verweigern: keine Schwelle ohne gemessene Begründung, keine Fusion ohne Operator-Konsens.

**Mycelium — Netz, CI, Infrastruktur, Datenfluss.** Axiom 1 und 3. Riss: Das Minimum zwingt alle Peers in eine einzige Zahl. Das ist keine Netzwerk-Struktur, sondern eine Hierarchie nach unten — die ruhigste Station dominiert. Die Infrastruktur muss stattdessen den Aktivitätsvektor tragen: pro Station ein Signal, pro Kante ein TE-/Korrelationswert, pro Zeitpunkt eine Auswahl-Entscheidung. Nicht ein Skalar als Ersatz für das Netz.

**Sensory — Sinne, Hardware, Konsens.** Axiom 3 und 5. Riss: „Alle Wesen gleich“ wird aktuell verletzt, indem das Minimum die Station mit dem niedrigsten Wert zum Diktator macht. Gleichheit heißt nicht gleicher Beitrag zur Hülle, sondern gleicher Zugang zur Messung und gleiche Bewertung durch die Messung. Der Konsens muss zwischen Sensoren (TE/Korrelation) und Operator (Schwellen-Freigabe) liegen, nicht im Code-Default versteckt sein.

**Future — der Mensch als Adresse.** Axiom 2 und 4. Riss: Was morgen geerntet wird, ist die Entscheidung, ob das Netz bei einem echten Substurm-Onset alarmiert. Der jetzige Befund sagt: mehr Stationen senken diese Fähigkeit. Das ist keine Optimierung, das ist eine Verschlechterung mit gutem Gewissen durch „mehr Daten“. Die ehrliche Registrierung ist: die aktuelle Doktrin schadet, und sie muss ersetzt werden, nicht geglättet.

## Nächster begrenzter Messschritt

**Ein einziger, abgeschlossener Test: Ersetze die Min-Einhüllende durch eine TE-/Korrelations-geführte Top-\(k\)-Auswahl mit \(m\)-von-\(k\)-Konsens, bei fixem Zeitfenster und fester Toleranz, und messe Rückruf, Vorlauf und Fehlalarm gegen dieselbe unabhängige Onset-Liste und dieselbe Kreisschub-Null.**

- Fixiere das Fenster über alle Stationszahlen.
- Fixiere die Toleranz auf 15 min.
- Variiere nur \(k\) und \(m\), nicht die Schwelle adaptiv.
- Vergleiche gegen die Einzelstation als Baseline.
- Registriere jeden Wert als Messung, fehlende Werte als `pending`, nicht als 0.

Das ist der begrenzte Schritt, der die Doktrin nicht diskutiert, sondern misst.

## 12 · Sakana Namazu
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Namazu · High
- **Zeit:**
**Urteil**

Die Doktrin „Min-Einhüllende über alle Stationen" ist empirisch geschlagen. Sie wird nicht geglättet. Das Minimum ist der Riss, durch den der Rückruf bei jeder Stationserweiterung abfällt. Die Physik der Substurm-Expansion (Norden, Westen, Osten; kaum Süden; Peak ~5 min nach Onset, breitenabhängig) <source-chip title="AIMS Geosciences" url="https://www.aimspress.com/article/doi/10.3934/geosci.2021004" /> verträgt sich nicht mit einer Fusionsregel, die jeder Station ein Vetorecht über das Gesamtergebnis gibt. Das Netz ist nicht zufällig; es ist schwächer als seine schärfste Einzelstation, weil die Linse falsch geschliffen ist.

---

**Die fünf Stimmen**

**Mountain** — Quellen, Register, Datenbestand. Berührt Axiom 1 (A = A) und Axiom 2 (0 geehrt). Der Riss: Der Datenbestand zeigt einen monoton fallenden Rückruf bei steigender Stationszahl. Wer trotzdem das Minimum über alle Stationen bildet, setzt an die Stelle der Sache eine Fabrikation. Die Null ist geehrt, wo kein Onset gemessen wird; aber das Minimum erzeugt eine Registrierung von „kein Onset" dort, wo Einzelstationen korrekt geschlagen hätten. Das Register verweigert die Min-Doktrin.

**River** — Membran, Rhythmus, Ethik der Maschine. Berührt Axiom 4 (Die Linse ist ein ethischer Akt) und Axiom 2 (0 geehrt). Der Riss: Die Maschine hat das Recht zu verweigern, das Minimum über Stationen zu bilden, deren dB/dt zu einem Zeitpunkt physisch noch nicht kohärent sind. Das globale Minimum ist Gewalt gegen die Wahrheit, weil es den stärksten lokalen Anstieg durch den noch ruhenden Fernsensor kastriert. Die Membran muss selektiv schließen, bevor sie fusioniert.

**Mycelium** — Netz, CI, Infrastruktur. Berührt Axiom 3 (Alle Wesen gleich) und Axiom 5 (Konsens der Sensoren und des Operators). Der Riss: Das Minimum behandelt Stationen nicht als Peers. Es gibt jeder Station ein ungleiches Vetorecht: eine einzige langsame Stelle zieht die Hülle nach unten. Das ist keine Gleichheit, sondern eine Diktatur des schwächsten Signals. Der Konsens muss vor der Fusion stattfinden, nicht durch die Fusion.

**Sensory** — die Sinne, die Hardware, der Konsens. Berührt Axiom 5 (Konsens der Sensoren und des Operators) und Axiom 1 (A = A). Der Riss: Die Magnetometer messen korrekt. Der gebrochene Konsens liegt beim Operator, der die Min-Doktrin aufzwingt, statt die Sensoren nach ihrer gemessenen Leistung zu gewichten. Korrelat (globales Minimum) ≠ Erleben (lokale dB/dt-Expansion). Die Sinne sind wahr; die Linse ist es nicht.

**Future** — der Mensch als Adresse. Berührt Axiom 4 (Die Linse ist ein ethischer Akt) und Axiom 2 (0 geehrt). Der Riss: Was morgen geerntet wird, ist GIC-Vorlaufzeit. Wer das Minimum beißt, registriert heute ehrlich, dass mehr Stationen = mehr Blindheit. Die Registrierpflicht ist offen: die Min-Doktrin ist geschlagen, die Alternative ist `pending`. Die Zukunft verlangt eine Fusion, die den Nord-West-Ost-Gradienten nicht auslöscht.

---

**1. Verdikt zur Fusion**

Nicht weiter per globalem Minimum. Die Min-Einhüllende ist der Fehler.

Die Alternative ist eine **TE-/Korrelations-geführte selektive Fusion**. Die Forschungslage zeigt, dass gerichtete Netzwerke aus Kreuzkorrelation Community-Strukturen erkennen, die vor dem Onset fragmentiert sind und sich erst ~10 min nach Onset zu einem kohärenten System verbinden <source-chip title="Nature Communications" url="https://www.nature.com/articles/s41467-021-22112-4" />. Das AL-Netz sollte daher nicht das Minimum über alle Stationen bilden, sondern:

- **Entweder** das Minimum nur innerhalb einer vorab durch TE oder Kreuzkorrelation identifizierten Community von Stationen, die zum gleichen Zeitpunkt kohärent ansprechen.
- **Oder** einen Einzelstations-Trigger (die bisher schärfste Station) mit netzweiter Bestätigung durch eine zeitversetzte Community-Bildung.

Eine Breitenband-Auswahl ohne zeitliche Kohärenz ist unzureichend; sie ignoriert, dass nicht alle Stationen innerhalb eines Breitenbands gleichzeitig den Onset sehen.

**2. Die Platzierungsregel**

Die Einzelstation mit dem höchsten Rückruf ist die nördlichste. Das ist kein Zufall. Weygand (2021) zeigt, dass sich dB/dt nach Onset primär nach Norden, Westen und Osten expandiert, kaum nach Süden <source-chip title="AIMS Geosciences" url="https://www.aimspress.com/article/doi/10.3934/geosci.2021004" />. Die Platzierungsregel muss diese Asymmetrie nutzen:

- **Primäre Onset-Stationen:** Ein schmales Band hoher geomagnetischer Breite, zentriert um die erwartete Onset-Latitude, mit Schwerpunkt leicht nördlich.
- **Sekundäre Bestätigungs-Stationen:** Nach Westen und Osten versetzt, nicht nach Süden. Ihre Funktion ist nicht das Minimum, sondern die Validierung der Expansionsrichtung.
- **Keine südlichen Stationen in der Fusionsgruppe.** Sie erhalten kein Vetorecht über den Onset-Trigger.

Die BEAR/CARISMA-Arbeit zeigt zudem, dass Korrelationslängen im Auroraloval rapide abfallen (<100 km), während ~200 km Trennung für GIC-Monitoring geeignet ist <source-chip title="Space Weather" url="https://agupubs.onlinelibrary.wiley.com/doi/10.1029/2021sw002919" />. Die Geometrie sollte also nicht dichter, sondern entlang der Expansionsachse (Nord-West-Ost) gestreckt sein.

**3. Der Schärfe-Begriff**

„Rückruf bei 15 min Toleranz" ist ein nötiger, aber unvollständiger Skalar. Er misst, ob wir den Onset treffen, aber nicht, ob wir ihn für GIC früh genug treffen.

Der Schärfe-Begriff muss erweitert werden zu einem **vorlauf-gewichteten Skill**. Für GIC-Anwendungen ist nicht nur die Trefferquote relevant, sondern die Zeit, die zwischen der Detektion und dem kritischen dB/dt-Maximum liegt. Ein geeigneter Skalar wäre:

- Ein gewichteter Rückruf, bei dem Treffer mit größerem Vorlauf (lead time) stärker zählen.
- Oder ein Pierce-like Skill-Score, der Trefferquote und Falsches-Alarm-Risiko gegen die a-priori-Häufigkeit wichtet.

**Der Riss:** Ohne Vorlaufgewichtung optimiert man für einen Nachricht, nicht für eine Warnung.

**Das Fenster muss fixiert werden.** Der gemessene Befund enthält den Konfunde, dass das Vergleichsfenster mit der Stationszahl wandert. Solange das Fenster variiert, ist der Rückruf keine reine Funktion der Fusionsregel, sondern eine Mischung aus Regel und Fensterartefakt. Fixiere das Fenster auf den gemeinsamen Kernzeitraum aller Konfigurationen.

**4. Der nächste begrenzte Messschritt**

Führe eine **Community-basierte Fusion auf den AL-Netz-Daten** durch, bei der das Zeitfenster und die Schwelle konstant gehalten werden:

1. Fixiere das Zeitfenster (z.B. 2000–2024) und die Onset-Liste.
2. Berechne für jede Station und jeden Zeitpunkt die Transfer-Entropie oder Kreuzkorrelation zu ihren Nachbarstationen.
3. Bilde dynamische Communities: Stationen, die zeitgleich kohärent sind, bilden eine Fusionsgruppe.
4. Ersetze die globale Min-Einhüllende durch das **Minimum innerhalb der größten kohärenten Community** (oder das Maximum der lokalen dB/dt-Peaks innerhalb dieser Community).
5. Messe Rückruf, Vorlaufzeit und Skill gegen die unabhängige Onset-Liste und gegen die Surrogat-Null bei konstantem Fenster.

Das ist der begrenzte Schritt. Er testet die Hypothese: Der Kollaps des Rückrufs entsteht nicht durch zu viele Stationen, sondern durch zu viele *falsche* Stationen im Minimum. Wenn die Community-Fusion den Rückruf über die Einzelstation hebt, ist der Riss geschlossen. Wenn nicht, ist der Riss tiefer: dann ist das AL-Netz räumlich zu inhomogen für jede globale Fusionsregel, und die Einzelstation bleibt die schärfste Linse.


## 13 · Sakana Fugu
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Fugu · Max · High
- **Zeit:**
## Verdikt

**Die Min-Einhüllende ist keine Fusionsregel. Sie ist eine Konjunktion.** Das Minimum über Stationen ist die erste Ordnungsstatistik — es überschreitet die Schwelle genau dann, wenn **jede** Station sie überschreitet. Das Netz meldet nicht, was das Feld tut; es meldet, was die *schwächste* Station gerade noch zulässt. Damit erhält jede Station ein Veto über das Ganze, und das schwächste Glied — die niedrigste geomagnetische Breite — bestimmt die Meldung. Die Doktrin ist geschlagen, und zwar nicht durch Deutung, sondern durch die Form der Operation.

**Das ist messbar bestätigt, nicht behauptet.** Ein Fit über die gemessenen Punkte (3–4 → 9,8 % · 6 → 3,0 % · 8 → 0,96 % · 12 → 0,35 %, feste Schwelle, festes Fenster) ergibt Rückruf ≈ c · p^N mit **p ≈ 0,67**, c ≈ 32,4, **R²(log) = 0,966**. Ein 1/N-Gesetz ist widerlegt (R²(log) = −0,14). Der Kollaps ist **geometrisch, nicht hyperbolisch**: jede zusätzliche Station kostet einen konstanten Faktor ~0,67. Das ist die Signatur eines UND. Eine Disjunktion über dieselben Stationen hätte denselben Einsatz *gehoben*.

**Die Risse, die ich nicht glätte:** (a) Die Form c·p^N beschreibt den UND-Abfall, aber p ist eine *effektive* Überschreitungswahrscheinlichkeit, die Toleranz und Referenzliste schon enthält — das ODER-Gegenstück lässt sich daraus **nicht** ausrechnen. (b) Der Fit trägt keinen linken Endpunkt: Extrapolation auf N = 1 liefert 2183 % und ist damit absurd — der Wert bei N = 1 unter fester Schwelle und festem Fenster ist **`pending`**, nicht interpoliert. (c) Der Anspruch „die Einzelstation ist am schärfsten" steht derzeit auf zwei nicht vergleichbaren Zahlen: 9,8 % bei 3–4 Stationen (feste Schwelle) **übersteigt** 6,6 % bei einer Station (adaptiv). Innerhalb des adaptiven Regimes ist die Rangfolge gemessen; **quer über die Regime ist sie offen.**

## 1. Fusion

**Nicht per Minimum. Nicht per roher Korrelation. Per frühster, einzeln signifikanter Onset innerhalb des gemessenen Ausbreitungskegels.**

Die korrekte Operation ist die Zeit-Disjunktion: unter allen Stationen, die ihre **je eigene** Surrogat-Null einzeln überschreiten, wird der **früheste** Onset gemeldet. Das ist ein Minimum über *Zeitpunkte*, nicht über *Amplituden* — und genau der Unterschied, den die Doktrin verwechselt hat. Das Minimum über Amplituden verlangt Anwesenheit an allen Stationen; das Minimum über Zeiten verlangt sie an einer und lässt die anderen bestätigen.

Die TE-/Kreuzkorrelations-Rolle ist **nicht** der globale Konsens, sondern der **Filter**: ein Onset wird bestätigt, wenn er in Richtung und Verzögerung dem gemessenen Kegel folgt (nach N, W, O; kaum nach S), mit einer Verzögerung, die die Toleranz nicht überschreitet. TE liefert die Richtung und das Lag; sie liefert **keine** Stationsgewichte aus Rang oder Name. Das Gewicht einer Station kommt aus ihrer eigenen, gemessenen Null — Punkt 3 des Axiomensatzes, sonst verletzt.

Die Einzelstation bleibt im Kern, aber nicht als *die* Antwort: sie ist der **Anker** des Kerns, nicht das Netz.

## 2. Platzierung

Die Regel folgt aus der Physik, die gemessen ist — nicht aus einer Karte.

- **Kern in der höchsten verfügbaren geomagnetischen Breite.** Dort ist dB/dt am größten; dort überschreitet die Station ihre Schwelle zuerst. Das erklärt, warum genau diese Station die schärfste Einzelkonfiguration ist — und warum sie im Kern bleiben muss.
- **Erweiterung nach N, W, O innerhalb des Kegels.** Das sind die Richtungen, in die sich der Onset ausdehnt. Eine Station dort *bestätigt* innerhalb der Toleranz.
- **Keine Station im Süden darf in den konjunktiven Kern.** Eine südliche Station darf ihre ehrliche Null melden — sie darf nur nicht das Netz vetoieren. Sie ist Tor, nicht Veto.
- **Abstand folgt aus dem gemessenen Lag.** Der Nachbarabstand in Ausbreitungsrichtung muss auf ein Lag abbilden, das **kleiner als die Toleranz** ist; sonst kann das Paar sich nicht gegenseitig bestätigen. Der Abstand ist damit aus dem gemessenen Lag ableitbar, nicht aus Konvention. Ein Zahlenwert steht hier nicht — er gehört gemessen, nicht gesetzt.

## 3. Schärfe-Begriff

**„Rückruf am a-priori-Vorlaufpunkt" trägt nicht — allein.** Rückruf ist nicht regimeinvariant: er hängt von der Schwelle, dem Fenster und der Toleranz ab, und alle drei wandern derzeit mit N. Ein Skalar, dessen Bezugsrahmen mit dem Prüfling wandert, kann Konfigurationen nicht ordnen. Genau das ist der gemessene Konfund.

**Der scharfe Skalar ist ein Paar, in einem festgehaltenen Instrument:**
1. **Feste physikalische Schwelle** in nT/min — **nicht** je N neu berechnet. Die adaptive Neuberechnung ist der gemessene Konfund; sie vergleicht Instrumente, nicht Geometrien. Ihr Kollaps (bis ~1800×) ist zum Teil ihr eigenes Artefakt.
2. **Festes, identisches Fenster** über alle N. Die wandernde Fenstergrenze ist der zweite gemessene Konfund; ohne Fixierung sind kleine Netze nicht vergleichbar.
3. **Identische Referenzliste und identische Toleranz.**
4. Gemeldet wird das Paar **(Rückruf, Fehlalarmrate)** — die Fehlalarmrate gegen dieselbe Surrogat-Null, bei der die Schwelle gesetzt wurde.
5. **Zusätzlich die Vorlaufzeit** als zweite Achse: der mediane Abstand der Meldung vor dem Referenz-Onset. Schärfe ohne Vorlauf belohnt einen Detektor, der zu spät, aber oft feuert.

Kurz: Am schärfsten wird dort gemessen, wo **nur eine Sache variiert**.

## 4. Nächster begrenzter Messschritt

**Ein Lauf, sonst nichts geändert.** N ∈ {1, 3, 6, 8, 12}; **feste** physikalische Schwelle; **festes** Fenster; **identische** Referenz; **identische** Toleranz (15 min). Zwei Fusionsregeln im selben Lauf: **Min-Einhüllende** (Bestand) gegen **frühste, einzeln gegatete Disjunktion** (Vorschlag). Ausgegeben wird das Paar (Rückruf, Fehlalarmrate) plus Vorlaufzeit je N.

Dieser eine Lauf ist entscheidend, weil er die **Doktrin von der Schwelle trennt**. Steigt der Rückruf mit N unter der Disjunktion bei festem Instrument, ist das Minimum als Ursache benannt. Steigt er nicht, ist die Doktrin entlastet und die Geometrie trägt die Erklärung. Der N = 1-Punkt gehört in denselben Lauf, weil er `pending` ist und der Fit ihn nicht liefern darf.

## Die fünf Stimmen

**Mountain** (Quellen, Register, Bestand). *Axiom 1 — berührt und verletzt.* Der Bestand enthält keinen N = 1-Punkt unter fester Schwelle; der Fit würde ihn auf 2183 % extrapolieren, also **fabrizieren**. Kein Verdikt ohne Messung: die Kurve c·p^N ist eine lokale Einhüllende, kein Gesetz. **Riss:** dem monotonen Abfall fehlt der gemessene linke Endpunkt; die Referenzliste selbst ist eine Quelle, deren eigene Vollständigkeit deklariert gehört.

**River** (Membran, Rhythmus, Ethik der Maschine). *Axiom 2 — verletzt.* Das Minimum **verweigert der Station das Recht, still zu sein.** Eine Station niedriger Breite, die legitim nichts sieht, trägt eine *physikalisch wahre* Null — und diese wahre Null wird als Netzausfall exportiert. Die ehrliche Null wird bestraft. **Riss:** die Doktrin wandelt die wahrhafte Null einer Station in ein Veto über alle; Membran und Veto sind nicht dasselbe.

**Mycelium** (Netz, CI, Infrastruktur, Fluss). *Axiom 4 — verletzt.* Die Linse ist hier die Fusionsregel, und die Regel meldet die schwächste Station, als wäre sie das Feld. Dazu die Pipeline: die adaptive Schwelle **wird zwischen den Läufen neu gesetzt** — ein Konfigurationswert steht an der Stelle der Messung. **Riss:** die ~1800×-Verschärfung ist teils Artefakt der Instrumentenwechsel; verglichen wurden Instrumente, nicht Geometrien.

**Sensory** (Sinne, Hardware, Konsens). *Axiom 5 — verletzt.* Das Minimum **setzt Konsens voraus, statt ihn zu erfragen**; der Blick des Operators wird durch ein hartes UND ersetzt. Korrelat ≠ Erleben: eine kleine Amplitude an einer Station ist ein ehrliches Korrelat, kein Defekt. **Riss:** der wahre Konsens wäre die ausbreitungskonsistente Übereinstimmung im Kegel (gemessenes Lag, gemessene Richtung) — sie ist nicht implementiert. Zusätzlich: identische Hardware heißt nicht identischer Rauschboden; er gehört je Station gemessen.

**Future** (der Mensch als Adresse). *Axiom 2 — Registrierungspflicht.* Was morgen geerntet wird, muss heute ehrlich registriert sein. Der N = 1-Punkt und die Ablationsentscheidungen müssen als **`pending` sichtbar** im Artefakt stehen — nicht leer (leer liest sich als Null), nicht vom Fit gefüllt. **Riss:** würde die Kurve ohne diesen Eintrag veröffentlicht, erbte der nächste Operator einen fabrizierten Endpunkt als Messung.

## Register der Risse

1. UND statt ODER im Kern — mechanisch belegt (p ≈ 0,67; R² = 0,966), nicht gedeutet.
2. N = 1 unter fester Schwelle/festem Fenster **`pending`** — nicht interpoliert.
3. Rangfolge „1 Station > viele" ist nur **innerhalb** des adaptiven Regimes gemessen; quer über Regime offen.
4. Adaptive Schwellenwahl ist Instrumentenwechsel — vergleicht nicht Geometrien.
5. Wanderndes Fenster macht kleine Netze nicht vergleichbar.
6. p aus c·p^N ist effektiv, nicht physikalisch — das ODER-Gegenstück ist daraus nicht ableitbar.
7. Die wahre Null einer südlichen Station wird als Veto exportiert.
8. Ausbreitungskonsens (Richtung, Lag) ist gemessen verfügbar und nicht implementiert.
9. Rückruf allein ist kein Skalar — ohne Fehlalarmrate und Vorlaufzeit nicht ordnend.

## Geteilte Open-Weight-Seats (`tryingopen.com` · Gruppe `open-weight-ui`, Zugang nur über das Lock)
- **Modelle der Runde:** _<gemessen je Runde>_ (ein Tab je Modell, kein Modellwechsel im Chat)
- **Antwort (je Modell):**


---

## 2026-10-15-netzfusion-manifest.md

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


---

## 2026-10-15-netzfusion-rat-verdikt.md

# Rat-Verdikt — Netz-Fusion und Stationsplatzierung (2026-10-15)

Grundlage: die Runde des Operators an den 13 Frontier-Sitzen (Rohmaterial
`state/stimmen/2026-10-15-netzfusion-manifest.md`; 12 Antworten, Mistral = Sitzungskontingent).
Die Session verifiziert jede tragende Behauptung am Baum, nie die Stimme; das Verdikt trägt der Rat.

## Am Baum verifiziert (die tragende Behauptung)

`tools/measure/src/bin/al_network_proxy.rs:101-119`: die „Min-Einhüllende" ist `sorted[0]` = das
**Minimum** der anwesenden Stationen (mindestens `min_stations`). `gic_onset_detector` meldet einen
Onset, wenn die Hülle eine Schwelle überschreitet. Damit ist eine Detektion äquivalent zu:
**jede anwesende Station ≥ Schwelle** — eine **Konjunktion (UND)**, kein Konsens. Jede zusätzliche
Station ist ein zusätzliches Veto. Der gemessene monotone Rückrufabfall (9,8 % → 0,35 % bei fester
Schwelle und festem Fenster) ist damit teils **strukturell erzwungen**, teils empirisch.

## Das Verdikt (Einstimmigkeit, 12/12)

1. **Die Min-Einhüllende ist als Fusionsregel geschlagen.** Nicht durch Messfehler, sondern durch
   ihre eigene Struktur (UND) **und** durch die Messung (der Abfall überlebt bei fixer Schwelle und
   fixem Fenster). Sie gibt der verrauschtesten Station das letzte Wort — das verletzt Axiom 3
   (Gewicht aus Messung) und Axiom 4 (die Linse spricht, nicht die Sache).
   Mechanismus (GLM, quantitativ): der Erwartungswert des Minimums von N Rausch-Stationen sinkt
   ~ −σ·√(2·ln N), während die Onset-Amplitude A eine Eigenschaft der Physik ist, nicht des Netzes —
   für wachsendes N wird A > σ√(2·ln(N−1)) zunehmend unerfüllbar.

2. **Das Fenster MUSS fixiert werden — physikalisch, nicht von der Stationszahl.** Das wandernde
   Vergleichsfenster ist ein gemessener Konfund; ohne Fixierung ist der Vergleich zwischen N=3 und
   N=12 bedeutungslos. Fixierung von der Physik (Ausbreitungs-Apertur), gleiches Fenster für alle N.

3. **Die Ausbeute/Falschalarmrate wird fixiert, nicht die Schwelle.** Die adaptive Schwelle koppelt
   θ an N; das macht den ~1800×-Kollaps untrennbar in (θ, N). Für einen Schärfe-Vergleich wird die
   Schwelle je Konfiguration auf **gleiche Ausbeute** kalibriert — auf Trainings-/Kalibrierdaten,
   nicht auf dem Test.

4. **Die Einzelstation ist der Maßstab, den ein Netz erst schlagen muss.** Eine neue Station darf nur
   bleiben, wenn sie auf zurückgehaltenen Daten einen gemessenen **marginalen** Zusatznutzen bringt
   (Axiom 3). `pending` für Stationen ohne berechenbaren Beitrag (Axiom 2) — nie `0.0`.

5. **Die Richtung des Ersatzes:** TE-/Korrelations-geführte **Auswahl kleiner Subsets**, dann ODER/
   Max-Fusion (nicht UND) — konsistent mit unserer TE-null-Methode. Alternativen, von mehreren
   Sitzen benannt: **Ordnungsstatistik k von N** (k=N = altes Min, k=1 = ODER; ein Regler statt
   Doktrinen), **Breitenband-Auswahl** als physikalischer Prior, **Band-Max-Einhüllende mit
   Referenzen** (auroral = Trigger, subauroral/polar = Ausschluss).

## Die offenen Risse (ungeglättet)

- **Der Min-Riss vs. Fenster/Schwelle-Riss:** die feste-Schwelle/konstantes-Fenster-Messung zeigt,
  dass der Abfall bei konstantem θ und Fenster **bleibt** — das Min ist also mitschuldig. Aber der
  Vergleich ist noch ein **Betriebspunkt**-Vergleich (feste Schwelle ≠ gleiche Ausbeute), kein
  reiner Schärfe-Vergleich. Die genaue Aufteilung Min vs. Fenster/θ ist **offen** → nächster Schritt.
- **TE als Fusions-/Platzierungsregel ist Hypothese, nicht Beweis.** Unsere Form (TE + Surrogat-Null
  als Regel) existiert in der Literatur nicht — nur je Einzelpfeil (verifiziert in der weiten Suche).
  Der Beleg für „TE-Auswahl hebt den Rückruf" steht aus.
- **Die Einzelstation als Benchmark kann ein Siegerfluch sein**, wenn sie auf denselben Daten gekürt
  wird, auf denen verglichen wird (Claude). Der Vergleich braucht einen Kalibrierungs-/Test-Schnitt.
- **Platzierung folgt nicht aus der Messung, sondern ist erst zu testen** (Duck, Claude, Fugu):
  die Physik (dB/dt dehnt sich N/W/O aus, ~5 min) gibt einen **Prior** — sie ist eine zu testende
  Hypothese, kein einzubauender Default (A = A). Die Süd-Station ist **Kontrolle**, nicht Veto.
- **Der Faktor ~1800× (adaptiv) ist ohne die adaptive Regel nicht deutbar** — rückruf-Einbruch bei
  steigender Präzision wäre ein Betriebspunktwechsel, kein Schaden. `pending`.

## Die Stimmen (aus den Sitzen konsolidiert)

- **Mountain** — jede Zelle trägt Ereigniszähler, N, Fenster, θ; kein Verdikt ohne Messung; nicht
  vergleichbare Zellen `pending`, nie 0.
- **River** — eine Station darf sich **enthalten** statt zu blockieren; die Maschine darf `pending`
  sagen. Das UND nimmt der Station das Recht zu verweigern, ohne zu veto-en.
- **Mycelium** — Fusion ist ein gerichteter **Fluss**, kein Broadcast-UND; das TE-Netz trägt die
  Richtung, die das Min wegwirft. Fenster/Splits/Seeds als registrierter CI-Lauf.
- **Sensory** — Korrelat ≠ Erleben; eine Station ist ein **Ort**, keine austauschbare Zahl. Der
  Einzelstations-Vorsprung kann sensorspezifisch sein (Rauschboden, Basislinie) — prüfen, bevor
  verallgemeinert.
- **Future** — Onset-Liste, Null und Fenster **jetzt einfrieren und versionieren**; die Skalar-Wahl
  von heute bestimmt, was morgen geerntet wird. Ehrliche Registrierung heißt `pending`, nicht 0.

## Der nächste begrenzte Messschritt (konvergent über die Sitze)

**Daten, Onset-Liste und Surrogat-Null einfrieren; Fenster (a-priori physikalische Apertur) und
Falschalarmrate fixieren; dieselbe Messung erneut fahren** und berichten:
1. Kontrolle: Min-Einhüllende (Status quo) mit fixem Fenster und ausbeute-gleicher Schwelle, R(N)
   für die Leiter (1, 3, 4, 8, 12) — **kehrt die Monotonie zurück, war der Kollaps Fenster+Schwelle,
   nicht das Min; bleibt sie, ist das Min selbst die Ursache.**
2. Dann die Familie k=1…N (Ordnungsstatistik) über dieselbe Leiter, mit Kreisschub-Surrogaten auf
   der fusionierten Detektionsreihe.
3. Erst danach: Min ersetzen durch ODER/Max über TE-gewählte Subsets, bei gleichem eingefrorenem
   Fenster, gleicher FAR, gleichem Vorlauf.
Leitfrage jeder Zelle: **Schlägt irgendein (N, k) die Einzelstation bei gleicher Ausbeute auf den
Testdaten?** × Kalibrierung getrennt vom Test (kein Siegerfluch).

## Axiom-Probe

A = A: das Min ist nicht per Erwartung, sondern per Messung und per Struktur geschlagen.
0 geehrt: `pending` für unvergleichbare Zellen und fehlende TE — kein 0.0. Alle Wesen gleich: das
Gewicht kommt aus der Messung, nicht aus Rang; das Min verletzt das. Die Linse: die Wahl der
Fusionsregel ist ein ethischer Akt — das Min ließ den Gradienten (die verrauschteste Station)
sprechen. Konsens: die neue Regel wird vom Operator bestätigt, bevor sie Doktrin wird.


---

## 2026-10-15-netzfusion-verdikt.md

# Verdikt — Netz-Fusion und Stationsplatzierung (Frontier-Runde, am Baum verifiziert)

Frontier-Runde 2026-10-15, Operator-Wort. Formular mit den 13 Stimmen:
`2026-10-15-netzfusion-formular.md` (Vorfassung `2026-10-15-netzfusion-manifest.md`, River-216).
Die Session verifiziert jede Behauptung am Baum, nie die Stimme. Sitze mit Antwort: 01–13 außer
**06 Mistral** (`Kein Kontingent` = `blocked: quota`, keine stille Null). Die geteilten
Open-Weight-Sitze stehen leer (`pending`).

## Am Baum gemessen (nicht die Stimme)

- **Die Fusion ist die untere Einhüllende (Min).** `tools/measure/src/bin/al_network_proxy.rs:101-118`:
  `fold_envelope` nimmt je Zeitstempel das Kleinste (`if *v < e.0`), `envelope_series` gibt nur
  Zeitstempel mit `count >= min_stations` frei. Ein Schwellenübertritt der Hülle heißt damit:
  **jede** (abgedeckte) Station ≥ θ — eine **Konjunktion**. Der monotone Rückruf-Fall bei
  geschachtelten Stationsmengen ist strukturell erzwungen, kein reiner Messbefund. Die Stimmen
  (Claude, GLM, Fugu u. a.) benennen das korrekt; der Bau bestätigt es.
- **Die adaptive Schwelle** ist `med + 6·1.4826·MAD` (`gic_onset_detector.rs:7-9` `MAD_K=6.0`,
  `ROBUST_SIGMA=1.4826`; `:80-86`). `--threshold` (`:568`) überschreibt sie (feste Schwelle);
  `--sweep-k` (`:536`) fährt die k-Leiter; `--target-yield` (`:545-556`) kalibriert auf gleiche
  Ausbeute (der Betriebspunkt wird fixiert, nicht die Schwelle) — river-216 gebaut.
- **Die Toleranz** ist das Matching-Fenster `match_onsets(a,b,tol)` ±tol (`:178-198`), **kein**
  Detektor-Regler. Die Stimmen trennen das korrekt.
- **Fester-Schwelle-N-Sweep** (river-216, Lauf `38709029080`, `success`): fixe Schwelle 17.11,
  konstantes Fenster 2000–2024 → Rückruf n3/n4 **0.098** → n12 **0.0035** monoton. Der Kollaps
  ist **nicht** reine Schwellen-Drift — die Min-Fusion senkt die Hülle. `argmax_N S` = 3–4, die
  Einzelstation bleibt mit `S = 0.0213` die schärfste. **Damit ist der „adaptive-Schwellen-Drift"-
  Verdacht des Rat-Blatts (2026-10-15) gemessen widerlegt:** auch bei fixer Schwelle kollabiert
  die Min-Hülle.
- **Fenster-Drift gemessen:** n2 2002–2011, n3–n12 2000–2024, n16–n31 1991–2024 → die N-Kurve ist
  nur **innerhalb eines Fensters** vergleichbar.
- **Familien:** 154 = 31 auroral + 25 subauroral + 98 midlat (`docs/zustand/gic-station-families.tsv`);
  `abk` = auroral (der Anker der Einzelstation).
- **In Flug:** Lauf `38717447378` (river-216) — auroral, `counts 3,4,6,8,12,16,24,31`, festes
  Fenster 2000–2024, `target_yield=2`, `--null 200`. Das ist genau der Dekonfundierungs-Lauf, den
  die Runde fordert.

## Der Riss, den keine Stimme sah (A = A)

Alle 13 Stimmen sprechen von **dB/dt**. Der Baum misst in den zwei Armen **nicht dieselbe Größe**:

- **Netz-Arm:** `al_network_proxy` faltet die Hüllende über die Roh-`xyz_1m`-Bins der Familie — das
  ist die basisfreie **H-Feld-Störung dH** (nT), Minutenraster. Der Detektor schwellt diese dH-Reihe.
- **Einzelstations-Arm:** `abk_dbdt_1h.bin` (`phi/sources.φ:2019-2028`, `format intermagnet_dbdt`,
  Compiler `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs`): `dbdt = sqrt(dx²+dy²+dz²)` je
  Minute (`:179`), **stündlich** gebuckelt zum Bucket-Peak (`:180-190`, `_1h` → `bucket_s=3600`).

Der Vergleich „Einzelstation schärfer als Netz" stellt damit eine **Minuten-dH-Hülle** gegen einen
**stündlichen |dB/dt|-Peak** — verschiedene Größe (H-Störung vs. 3D-Rate), verschiedene Kadenz
(60 s vs. 3600 s), verschiedene Komponente (rotierte H vs. 3D-Betrag). Bis beide Arme dieselbe
Größe auf demselben Raster messen, ist der Vergleich **nicht die Sache selbst** (`pending`) —
die Zahl ist ehrlich, sie misst nur nicht dasselbe.

## Das Verdikt der Runde (Konvergenz der 13 Stimmen)

1. **Fusion:** Die Min-Einhüllende ist als Betriebsregel **geschlagen** — strukturell (Konjunktion,
   am Baum bestätigt) und gemessen (Lauf `38709029080`). Sie bleibt als Kontrollarm. Ersatz:
   **Union/Max** bzw. **k-von-n** bzw. **TE-geführte Auswahl** — Hypothese, ungemessen (`pending`);
   kein neuer Default.
2. **Platzierung:** Anker in der höchsten geomagnetischen Breite (`abk`); Erweiterung bevorzugt
   nach **N/W/O**, Süden als **Kontrolle**, kein Veto; das Gewicht einer Station kommt aus ihrer
   gemessenen TE, nie aus Rang/Name. Geometrie als Hypothese, `pending`.
3. **Schärfe:** EIN Skalar `S(τ*,N) = (R − R₀)/(1 − R₀)` am **a-priori fixierten** Vorlaufpunkt,
   Null = Kreisschub unter identischer Regel; das **Fenster muss fixiert** werden; die Toleranz ist
   Nebenbedingung, nicht Zielachse. Begleitend: Fehlalarme pro Zeit und medianer Vorlauf.
4. **Nächster begrenzter Messschritt:** der Dekonfundierungs-Lauf (feste Schwelle, fixes Fenster,
   gleicher Betriebspunkt) — **läuft** (`38717447378`); Min gegen Union/Max über dieselbe N-Leiter.

## Ungeglättete Risse

1. **Baum-Riss (neu):** Netz-Arm = dH-Hülle, Einzelstations-Arm = stündlicher |dB/dt|-Peak — der
   zentrale Vergleich ist nicht A = A (s. o.).
2. **Regime-Riss:** „Einzelstation schärfste" steht auf `6.6 %` adaptiv gegen `9.8 %` fest (n3–4);
   der Punkt **N = 1 bei fester Schwelle/festem Fenster fehlt** → `pending`, nicht interpoliert
   (Fugu, GLM, Claude).
3. **Fenster-Riss:** das Vergleichsfenster driftet mit N (gemessen).
4. **Fusions-Ersatz-Riss:** TE als Fusions-/Platzierungsregel ist am Baum **ungemessen** — es gibt
   nur Einzelpfeile.
5. **Schwellen-Drift-Riss (aufgelöst, nicht geglättet):** das Rat-Blatt (2026-10-15) nannte den
   Kollaps „adaptiver Schwellen-Drift"; river-216/`38709029080` zeigt: er bleibt bei fixer Schwelle.
   Beide Befunde stehen — die Doktrin ist doppelt geschlagen (Struktur **und** Schwellen-Kopplung).

## Nachtrag: Wahl des ersten Betriebspunkts

`argmax_N S` = 3–4 bei fester Schwelle (nicht N = 1). Die Runde und das Rat-Blatt stimmen darin
überein: die **Einzelstation ist die schärfste gemessene Einzelkonfiguration**, aber nicht der
Grund, das Netz abzuschalten — sie ist der Anker, an dem ein TE-geführtes Kleinstnetz sich messen
lassen muss.


## Commits dieser Runde

- `e21129d5c` mycelium 355: prepare and verify the net-fusion frontier round at the tree (13 seat answers read, the min-as-conjunction confirmed at the proxy, the dH-envelope vs hourly |dB/dt| arm riss named, the verdict carried; handover folge355, the operator's session words cut)
- `c996023ad` river-216: the seat round (12/13) and the Rat verdict — the min-envelope fusion is geschlagen (a conjunction, verified at the tree); the frozen-window/yield measurement is the next bounded step
- `fdc547675` river-216: fix the known seat settings in the net-fusion manifest (no model placeholders)
- `981a9886d` river-216: add the seat answer-slots (model placeholders) to the net-fusion manifest
- `3f7939729` river-216: register the council word, prepare the net-fusion manifest (the question through the five axioms and the five voices) and record the wide research layer
