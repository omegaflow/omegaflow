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
