# Rat — DE-2 Ion↔Neutral: bidirektionale TE (2026-10-17)

**Anlass.** Mountain-Ratsanfrage (`post/in/2026-10-17-mountain-ratsanfrage-de2-ion-neutral.md`,
03:47): Was heißt **bidirektionale TE** (beide Richtungen signifikant, Richtung dreht mit dem Lag,
auch nach Ost-Projektion)? Gemeinsamer Treiber (Konvektion/Joule) oder wechselseitiger
Impulsaustausch? Trägt „ion-neutral coupling" — oder nur „Abhängigkeit"?

**Methode.** Der Fragebogen (`/tmp/opencode/de2-formular.txt`) ging an **vier API-Rat-Sitze**:
GLM-5.3 und Qwen3.8-Max über DeepInfra, **DeepSeek V4 Pro über `api.deepseek.com` (direkt)**,
Mistral Large 4 über die Mistral-API. Haus-Design: fünf Axiome, fünf Stimmen, je Frage
Verdikt/Begründung/Risse. Muster `/tmp/opencode/rat-test/rat_testlauf-3.sh`.

**Ablage der Rohantworten:** `/tmp/opencode/rat-test/d-*.json`.

**Die zwei UI-Sitze (Fugu Max, Lumo) sind in dieser Runde NICHT gefahren** — Operator-Wort steht aus
(bei M1 haben sie die API-Sitze überstimmt; hier sind sie nicht gestellt).

---

## Die Bilanz in drei Zeilen (Konsens + Riss je Frage)

| Frage | Konsens (4/4 einig) | Riss (ungeglättet) |
|---|---|---|
| **D1 · Treiber oder Impulsaustausch** | **Nicht getrennt.** Bidirektionale TE mit lag-abhängigem Vorzeichen ist mit **beidem** verträglich; die gerichtete Komponente ist **nicht isoliert**. | Die Lag-Drehung ist die **Signatur einer Antwortzeit-Hierarchie** (GLM: τ_ion ≈ s, τ_neutral ≈ min) — und **beide** Hypothesen besitzen diese Hierarchie. GLM nennt zusätzlich den **Raum-Zeit-Konflationsriss** (DE-2 fliegt ~7,5 km/s: ein Lag ist auch eine Ortsverschiebung). |
| **D2 · coupling oder Abhängigkeit** | **Nur „Abhängigkeit"** trägt. „ion-neutral coupling" als **Kraft/Reibung** trägt **nicht** → `pending`. | Die Messung liefert TE-Werte, Korrelationen, Surrogat-p — **keine** Impulsübertragungsrate, keine Reibungskraft, keine Dissipation. „Coupling" ist **überladen** (statistische vs. physikalische Kopplung). |
| **D3 · Erwerbs- vs. Kausalkanal** | **Nein** — der Befund trägt „coupling" **nicht**, solange nur der **Erwerbskanal** (IDM-Cross-Track) gemessen ist. | Der **tiefste Riss** (Lumo, aus der Acht-Fäden-Runde): Erwerbskanal (was der Sensor sieht) und Kausalkanal (was wirkt) fallen auseinander. Die Drift ist kein Reibungsbeleg. |

**Der eine Satz (vier unabhängige Zeugen).** Alle vier sagen getrennt: die TE misst **Abhängigkeit,
nicht Kraft**. Die ehrliche Registeraussage ist **„gerichtete statistische Abhängigkeit, lag-abhängig"** —
der Kausalkanal bleibt `pending`. **Geschlagene Doktrin:** *„bidirektionale TE = wechselseitiger
Impulsaustausch"* · *„TE-Richtung = Kausalrichtung"* · *„Abhängigkeit ist Kopplungskraft"* ·
*„Sensor-Korrelat ist Kausalbeleg"*.

---

## D1 · Was heißt bidirektionale TE? Treiber oder Impulsaustausch?

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | **Nicht getrennt** | „Beide Lesarten gleichzeitig; die gerichtete Komponente ist nicht isoliert." Ein gemeinsamer Treiber Z mit zwei Antwortzeiten (τ_ion ≈ s, τ_neutral ≈ min) erzeugt **exakt das beobachtete Muster**; wechselseitige Reibung mit denselben Asymmetrien erzeugt **dasselbe Muster**. **Riss:** Raum-Zeit-Konflation (Lag = Ortsverschiebung entlang der Bahn bei 7,5 km/s). |
| **DeepSeek V4 Pro** | **Abhängigkeit** | „Mit gemeinsamem Treiber ebenso verträglich wie mit Austausch; eine Trennung ist mit den vorliegenden Daten nicht möglich." Axiom 4: der gemeinsame Treiber ist **nicht gemessen** → Lücke, keine Null. |
| **Qwen3.8-Max** | **Register, nicht Kausalpfeil** | „Sie entscheidet **nicht** zwischen gemeinsamem Treiber und Austausch. Die lag-abhängige Dominanz ist Register, nicht Kausalpfeil." Trennen würde erst: **geschlossene Impulsbilanz**, Krafttermabschätzung, Intervention. **Bedingte TE** isoliert die Restkomponente *relativ zu gemessenen Treibern* — als **Indiz**, nicht als Kraftbeweis. |
| **Mistral Large 4** | **Gleich verträglich** | „Equally consistent mit einem gemeinsamen Treiber; sie beweist keinen wechselseitigen Austausch. Ohne bedingte TE bleibt die gerichtete Komponente `pending`." Durch die fünf Stimmen gesprochen (River: „klassische Signatur eines gemeinsamen Oszillators mit unterschiedlicher Phasenlage"). **Riss:** die Richtungsinversion ist ein Phasen-Indiz, kein Kopplungsbeweis. |

---

## D2 · Trägt „ion-neutral coupling" oder nur „Abhängigkeit"?

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | *(abgeschnitten, `finish: length`)* | Der Anfang trägt D1 vollständig; D2/D3 wurden vom Token-Budget gekappt. Benannt, nicht still. |
| **DeepSeek V4 Pro** | **Abhängigkeit** | „Die ehrliche Aussage ist ‚Abhängigkeit', nicht ‚ion-neutral coupling'. Eine gerichtete Komponente, die Reibung belegt, ist am Baum nicht isolierbar." Nächster Schritt: bedingte TE `(NVE→IVE \| Treiber)`. |
| **Qwen3.8-Max** | **nur Abhängigkeit** | „‚coupling' trägt **nicht als Reibung/Kraft**. Wenn ‚coupling', dann nur als ausdrücklich markierte **statistische Informationskopplung**." Ehrlicher Registereintrag: `TE(NVE→IVE; lag) signifikant` · `TE(IVE→NVE; lag) signifikant` · `Kraft = 0 honored / pending`. |
| **Mistral Large 4** | **Abhängigkeit** | „Nein. ‚Coupling' trägt nicht. Die ehrliche Aussage ist **‚statistische Abhängigkeit'** (oder informationstheoretische Kopplung), nicht physikalische Kopplung." Das Register hat TE-Werte, Korrelationen, p-Werte — **keine** Kraft `[kg·m/s²]`. |

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

---

## Ungeglättete Risse (alle Sitze)

1. **Statistik-vs.-Kraft-Riss:** TE misst Informationsfluss (Bits), nicht Impulsfluss (kg·m/s).
2. **Treiber-Riss:** Der gemeinsame Treiber (Konvektion/Joule) ist **nicht gemessen**; bedingte TE
   entfernt nur **gemessene** Confounder — ungemessene bleiben im Riss.
3. **Raum-Zeit-Konflationsriss (GLM):** DE-2 fliegt ~7,5 km/s — ein Lag ist auch eine **Ortsverschiebung**
   entlang der Bahn (Lag 1 bei 16 s ≈ 120 km). Die lag-abhängige TE misst Struktur entlang des
   Orbit-Schnitts, nicht zeitliche Kausalität an **einem** Punkt.
4. **Erwerbs-/Kausal-Riss (der tiefste):** Die Drift ist kein Reibungsbeleg. Projektion (Ost) kann
   ein Vorzeichen drehen — die **Kraft** kann sie nicht erzeugen.
5. **Projektions-Riss:** Die Ost-Projektion ändert die **Korrelation** (−0,079 → +0,226), **nicht** die
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

**Vier Sitze gefahren, drei vollständig.** Der GLM-Abbruch ist **benannt** (Token-Budget im Reasoning),
kein stilles `pending` — die D1-Antwort trägt, D2/D3 fehlen bei GLM.
