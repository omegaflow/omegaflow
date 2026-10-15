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

## Roster — Sitze (ein Antwort-Platzhalter je Modell)

Der Roster ist variabel; je Runde wird das gefahrene Modell am Sitz gemessen und hier eingetragen
(Platzhalter → gemessener Wert). Die langsamen Reasoning-/Research-Sitze zuerst stellen, das
langsamste zuletzt abholen. Ein Sitz, der nicht antwortet, wird als `pending` mit genanntem Grund
verbucht — nie als stille Null.

### 01 · Duck.ai
- **Site:** `https://duck.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 02 · Claude
- **Site:** `https://claude.ai/new`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 03 · Qwen
- **Site:** `https://chat.qwen.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 04 · Z.ai / GLM
- **Site:** `https://chat.z.ai/`
- **Modell/Einstellung:** _<Platzhalter>_ (Deep Think Max für Reasoning-Runden)
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 05 · DeepSeek
- **Site:** `https://chat.deepseek.com/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 06 · Mistral
- **Site:** `https://chat.mistral.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 07 · Gemini AI Studio
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 08 · MiniMax
- **Site:** `https://agent.minimax.io/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 09 · Kimi
- **Site:** `https://www.kimi.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 10 · Lumo (Proton)
- **Site:** `https://lumo.proton.me/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 11 · Perplexity (Deep Research)
- **Site:** `https://www.perplexity.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 12 · Sakana Namazu
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### 13 · Sakana Fugu
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** _<Platzhalter>_
- **Zeit:** _<Platzhalter>_
- **Antwort:**

### Geteilte Open-Weight-Seats (`tryingopen.com` · Gruppe `open-weight-ui`, Zugang nur über das Lock)
- **Modelle der Runde:** _<Platzhalter>_ (ein Tab je Modell, kein Modellwechsel im Chat)
- **Zeit:** _<Platzhalter>_
- **Antwort (je Modell):**
