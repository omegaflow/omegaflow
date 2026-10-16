# UI-Seat-Auswertung — wer setzte um, wer war einzigartig

**Zweck:** Messgrundlage für den **Cost-Quality-Rat** über das DeepInfra-Gateway + die zwei
UI-Schnell-Sitze (**Sakana Fugu**, **Lumo**), damit der Copy-Paste-Aufwand der UI-Chats entfällt.

**Methode:** Kein Zählen von Prosa — je Sitz zwei Maße, am Baum belegt:
- **Umsetzung** = der Vorschlag/Sitz-Name erscheint in einem **Commit** oder einer **gebauten
  Code-Stelle** (Beleg: sha).
- **Einzigartigkeit** = der Sitz brachte eine Position, die **kein/zwei** andere hielten, und sie
  ist **am Baum als Riss benannt** (nicht geglättet).

Quellen: die Verdikt-Dateien (`state/stimmen/2026-10-*`) + `git log` (Commit-Botschaften 05.–15.10.).
Was nicht belegbar ist, bleibt `pending` — nie geraten.

---

## 1 · Belegte Umsetzungen je Sitz (aus Commits + Code)

| Sitz | Was umgesetzt wurde | Beleg (sha / Stelle) |
|---|---|---|
| **Rat (5 Stimmen)** | `--night-sector` MLT 16–03 am Netz-Arm | `b9f48bff4` |
| **Rat** | „deny global_min" — Fusion leitet sich aus der Größe (dH→min, \|dB\|→union) | `0b3332c24` |
| **Rat** | Norm in die Kanal-Deklaration (`norm:<token>`, verweigert bei Kopf-Mismatch) | `01ea75373` |
| **Rat** | M4: SH-Synthese = **eigene Klasse**, kein achter Kernel | `80a4bff92` |
| **Rat** | Katalog-Verdrahtung: **eigenes Modul** > block-scoped (gewichtet, `roster2_gewichtet`) | (Verdikt, Weg offen) |
| **GLM-5.3 Deep Think Max** | quantitativ: Erwartungswert des Minimums ~−σ√(2 ln N) → Fusions-Einbruch | `netzfusion-rat-verdikt` Z. 22 |
| **Claude** | „Abwesenheit über Gültigkeits-Flag, keine Koordinate erfinden" (ENtsO-Form) | `entsoe_roster_gewichtet` Z. 19 |
| **Duck.ai (GPT-6 Luna)** | „eigener Ereignis-Record / EIC kein räumlicher Anker" (ENtsO) | `entsoe_roster_gewichtet` Z. 21 |
| **Sakana Namazu** | „kein (x,y,z)-Zwang, kein tau-Missbrauch" (ENtsO) | `entsoe_roster_gewichtet` Z. 22 |
| **Sakana Fugu** | Venus-Bild: „HEALPix trifft 75 m nicht — jede Venus-HiPS ist Resampling" | `venus-bild-rat-verdikt` Z. 43 |
| **Sakana Fugu** | Provenienz-Layer = Bedingung (die `0` des Basemaps ehren) | `venus-bild-rat-verdikt` Z. 67 |
| **Sakana Fugu** | F7′: „Energie gegen Lautstärke" — der schärfste Riss | `nachfrage-f7-f4-verdikt` Z. 44 |
| **Kimi** | M4.7: „der 5. Riss greift nicht auf der Koeffizienten-Zeile" (benannter Widerspruch) | `sh-kernel-verdikt` Z. 99 |
| **Gemini** | „Code-Bedingung `if sum(weights)==0` entfernen" (Form präzisiert) | `rat-fragenpaket-verdikt` Z. 106 |
| **MiniMax 10** | „die Min-Hülle zählt fehlende Station als ruhig" — am Baum widerlegt | `rat-fragenpaket-verdikt` Z. 103 |
| **Perplexity** | M4.2: Kopfweg **(a) Draht** | `sh-kernel-verdikt` Z. 45 |

**Belegte Umsetzungen insgesamt:** der **Rat** führt (4 harte Code-Umsetzungen), danach
**Sakana Fugu** (3 benannte Beiträge), **GLM-5.3** (1 quantitativer Mechanismus).

---

## 2 · Belegte Einzigartigkeit je Sitz (als Riss getragen)

| Sitz | Einzigartige Position | Wurde sie getragen oder verworfen? |
|---|---|---|
| **Kimi** | M4.7 widerspricht: der 5. Riss sitzt an der **Benennung**, nicht der Einheit | **getragen** (beide Lesungen stehen) |
| **Inkling** | „globales Dataset für einen Punkt = Über-Overhead" | **getragen** (als gleichwertige A-Alternative) |
| **Sakana Fugu** | „jede Venus-HiPS **ist** ein Resampling" | **getragen** → COG wurde Träger |
| **Lumo** | F7′: „**Stille**" bei Σω=0 | **Minderheit** (2 Sitze: Kimi, Lumo) — Bau tut es, Runde verwirft |
| **Kimi** | F7′: „**Stille.** Beide Kanäle 0." | **Minderheit** (2 von 12) |
| **MiniMax M3** | ENtsO: „(D) Ganzbild" (Venus) — Abweichler | **verworfen** (Venus bleibt W) |
| **DeepSeek 07b** | F5: „`pending`; FMI ist nicht GIC" (kein Verdikt) | **als Riss benannt** |
| **Claude** | „die Einzelstation als Benchmark kann ein Siegerfluch sein" | **getragen** (Kalibrierungs-/Test-Schnitt gefordert) |

**Einzigartig** (Position hielt nur 1–2 Sitze und wurde als Riss getragen): **Kimi** (2×),
**Lumo** (1×), **Sakana Fugu** (1×), **Inkling** (1×), **Claude** (1×), **MiniMax** (1×).

---

## 3 · Rohdaten-Landkarte (wo die Auswertung materialisiert ist)

- **Korpus:** `~/projects/omegaflow-work/stimmen/` (981 Dateien, 25.09.–14.10.) +
  `~/projects/omegaflow/state/stimmen/` (57 Dateien, 15.–16.10.).
- **Fertige Auswertungen (Gewichtung/Konvergenz):**
  - `2026-10-13_roster2_gewichtet.md` — Gewichtungsstufen A/B+/B je Sitz.
  - `2026-10-13_entsoe_roster_gewichtet.md` — **Sitz-Tabelle mit Verdikt je Seat** (Vorbild der Form).
  - `2026-10-15-*-verdikt.md` (8×) — Konvergenz + Risse je Frage.
- **Umsetzungs-Seite:** `git log --since=2026-10-05` — Commit-Botschaften nennen Sitz/Regel.

## 4 · Muster (gemessen, nicht interpretiert)

- **Der Rat** trägt die meisten **harten** Umsetzungen (4) — er ist die Struktur-Autorität.
- **Sakana Fugu** ist der stärkste **UI-Sitz** nach belegten Beiträgen (3), dicht gefolgt von
  **GLM-5.3**, **Claude**, **Kimi**.
- **Einzigartigkeit** ist selten: 5 Sitze hielten je **eine** nur-ihnen-eigene Position; sie
  wurden **als Riss getragen**, nie geglättet.
- **Lumo** und **Kimi** dissentieren bei F7′ **gemeinsam** (Stille) — nicht einzigartig, sondern
  eine **2er-Minderheit**.

## 5 · Nenner-Seite — wie oft war jeder Sitz dabei (gemessen über den vollen Korpus)

Die Erste Fassung nutzte nur die **13 Verdikte** (≈ 3 % der **441 auswertbaren .md-Runden-Dokumente**;
dazu **411 JSON** Rohantworten, 89 txt). Der Riss war: ohne Nenner kein Rang. Jetzt gefüllt —
dateibasiert („in wie vielen Runden-Dokumenten wird der Sitz befragt / trägt er einen Verdikt-Beleg").

| Sitz | befragt (Runden-Dateien) | mit Verdikt-Beleg | Umsetzungen (§1) |
|---|---|---|---|
| **Claude** | 56 | 45 | 1 |
| **Qwen** | 53 | 50 | — |
| **Duck.ai** | 42 | 28 | 1 |
| **Z.ai / GLM** | 39 / 17 | **65** | 1 |
| **DeepSeek** | 34 | 39 | — |
| **Gemini** | 14 | 17 | 1 |
| **Mistral** | 13 | 13 | — |
| **MiniMax** | 12 | 8 | 1 (widerlegt) |
| **Kimi** | 12 | 17 | 1 |
| **Perplexity** | 10 | 7 | 1 |
| **Lumo** | 8 | 11 | — |
| **Sakana (Namazu/Fugu)** | 6 | 7 | 3 (Fugu) |

**Korpus:** `~/projects/omegaflow-work/stimmen/` 981 Dateien (412 md · 411 json · 89 txt),
25.09.–14.10. · `~/projects/omegaflow/state/stimmen/` 58 Dateien (15.–16.10.).
**`fugu` erscheint in `work/stimmen` 0×** — der Sitz tritt erst ab 15.10. in `state/stimmen` auf
(F7′, Venus-Bild, Netz-Fusion). Das ist ein Befund, kein Fehler.

**Vorsicht (kein stiller Schluss):** die Zahl zählt **Nennungen je Dokument**, nicht die Zahl der
**Vorschläge je Sitz**. Ein Sitz, der in vielen Runden **bestätigt** (Claude, Qwen), sammelt Präsenz
ohne eigene Umsetzung; ein Sitz, der **selten, aber scharf** abreitet (Sakana Fugu), hat wenig
Präsenz, aber die höchste Umsetzungsdichte. **Präsenz ≠ Beitrag** — das ist der zweite Riss.

## 5c · Der harte Zähler — echte Runden-Antworten je Sitz (JSON-Rohmaterial)

Aus 411 JSON in `work/stimmen`: **89 sind `{"error":…}`** (API-Ausfall), **172 sind Förderungs-
Reviews** (`*-application`/`fellowship`/`credits`) — **keine Rat-Runden**. Bleiben **226 echte
Runden-Antworten**. Zusätzlich die **Dissent-Nennungen** („weicht ab / dissentiert / nur X"):

| Sitz | Runden-JSON (echt, gemessen) | Dissent-Nennungen |
|---|---|---|
| **Z.ai / GLM** | 35 / 21 | 2 |
| **Claude** | 22 | 3 |
| **Gemini** | 15 | 1 |
| **Mistral** | 12 | — |
| **Kimi** | 11 | 2 |
| **Qwen** | 7 | **4** |
| **Duck.ai** | 5 *(UI-Kanal: Browser, nicht voll in JSON)* | 1 |
| **DeepSeek** | 2 | 3 |
| **MiniMax** | **0** | 1 |
| **Lumo** | **0** *(UI)* | — |
| **Perplexity** | **0** | — |
| **Sakana** | **0** *(UI)* | 1 |
| **Fugu** | **0** *(UI, erst ab 15.10.)* | 1 |

(Auch `arena` 10 und `sonnet` 2 — Arena-UI-Sitze bzw. Claude-Sonnet-Reads, eigene Kanäle.)

**Riss, der die Zahl begrenzt (nicht geglättet):** Die **UI-Sitze** (Duck, Claude-UI, Lumo, Sakana,
Fugu) laufen über den **Browser** und legen **keine** API-JSON ab — die 226 JSON zählen also
**API-Sitze vollständig, UI-Sitze kaum**. Ein Ranking allein aus den JSON würde die UI-Sitze
systematisch unterschätzen. **Deshalb sind beide Beine nötig:** die **Präsenz-Zählung** (§5, aus den
.md-Runden) deckt die UI-Sitze, der **JSON-Zähler** (§5c) die API-Sitze.

## 5d · Der belegte Einzigartigkeits-Kern (Dissent, der getragen wurde)

**Qwen** führt die Dissent-Spur (4) — u. a. „Geokodier-Herkunft (unzulässige Annahme)", getragen.
Danach **Claude** (3: u. a. „Einzelstation als Siegerfluch"), **DeepSeek** (3: F5 „FMI ist nicht GIC").

## 5e · Der zweite Riss — Präsenz und Wirkung sind verschieden

- **Höchste Präsenz:** Claude (56) · Qwen (53) · Duck (42) — die „immer-dabei"-Sitze.
- **Höchste Wirkung je Auftritt:** **Sakana Fugu** (6 Auftritte → 3 Umsetzungen) und **Rat**
  (Struktur-Autorität). **GLM-5.3** trägt den dichtesten Verdikt-Beleg (65).
- **Lumo** (8) und **Kimi** (12): wenig Präsenz, aber als **2er-Minderheit** (F7′ Stille) gemeinsam
  scharf — Kimi zusätzlich einzigartig bei M4.7.
- **Folge für den Cost-Quality-Rat:** nicht die **häufigsten** Sitze sind die wertvollsten, sondern
  die mit **hoher Umsetzungsdichte** (Fugu, Rat) plus die **billig-direkten** (DeepInfra-Route).
  Die Präsenz-Zahl ist der Nenner, die Umsetzung der Zähler.

## 6 · Nächster Schritt (für den Cost-Quality-Rat)

1. Diese Tabelle als **Grundlage** — Zähler (§1, Umsetzungen) und Nenner (§5, Präsenz) stehen.
2. Die **Vorschlags-Zählung je Sitz** exakt nachziehen (nicht Nennung, sondern eigener Beitrag
   je Runde) — aus 2–3 dichten Runden (`roster2`, `fragenpaket`, `netzfusion`) als Stichprobe.
3. Die **API-Route** je Sitz messen (erreichbar? Kosten?) über `bin/rat_dispatch` + DeepInfra.
4. Die zwei UI-Schnell-Sitze (**Sakana Fugu**, **Lumo**) als Fragenpaket (5 Axiome + 5 Stimmen)
   anbinden — ohne internen Overhead.

## 7 · Korpus-Beleg (wie viele Dokumente wirklich vorliegen)

- **441 auswertbare Runden-Dokumente** (bedienbar): 412 `.md` (`work`) + 29 `.md` (`state`).
- **411 JSON** Rohantworten (je Sitz ein POST) · **89 txt** (Prompt-/Antwort-Dumps) · 15 sh · 14 log.
- **Erste Fassung nutzte 13** (die Verdikte) — **≈ 3 %**. Diese Fassung trägt den **vollen Nenner**
  über alle `.md`; die JSON-Rohantworten sind für die exakte Vorschlags-Zählung noch offen (§6.2).
