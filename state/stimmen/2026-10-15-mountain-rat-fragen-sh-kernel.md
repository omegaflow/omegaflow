# Rat-/Roster-Fragebogen — Die Kugelflächen-Synthese als Feld/Kernel (M4, Mountain, 2026-10-15)

*(Ein benannter Akt. Die **Forschungsschicht** ist gefahren (`bin/all_wide`, Spill unten zitiert).
Der Operator sendet **dieses Paket** unverändert an jeden Sitz; die Sitz-Antwort trägt die Frage.
Die Session verifiziert jede Behauptung am Baum, nie die Stimme; das **Verdikt trägt der Rat**
(fünf Stimmen durch die fünf Axiome des Granits). Vorgänger-Formulare:
`2026-10-15-mountain-rat-fragen.md` (M1–M3), `2026-10-15-mountain-rat-fragen-verdikt.md` (Verdikt).)*

Der Katalog (`2026-10-15-rat-fragen-katalog.md`) führt die Fragen F1–F7. **M4 ist dort nicht** —
sie ist die Fortsetzung der Mountain-Kraft-/Kernel-Familie (M1–M3). Der M1-Rat sprach: `gdata`/`mgg`
trägt als **Schwere-SH** (`gfc`/`vmf3_sh`). **Wie** ein Kugelflächen-Modell aber in die 26×f64-Welt
tritt, ist **ungestellt** — das ist M4.

## So wird dieses Formular benutzt (für jeden, der es in die Hand nimmt)

1. **Operator:** den Block „Das Paket (verbatim an jeden Sitz)" **unverändert kopieren** und an
   jeden Sitz **einmal** senden. Kein Vorwort, keine Zusatzfrage.
2. **Operator:** je Sitz die Antwort in die `Antwort (M4)`-Zeile des passenden Abschnitts unter
   „Die Sitze" kleben und die `Zeit:`-Zeile füllen. Ein Sitz ohne Antwort erhält `pending` mit
   genanntem Grund (Tageslimit/Login), **nie** eine stille Leerzeile.
3. **Session:** verifiziert jede Behauptung **am Baum**, nie die Stimme; schreibt das Verdikt.
4. **Regel:** ein Sitz, der „denkt", ist **nicht** `pending`; erst nach einer echten Frist mit
   benanntem Grund. Das langsamste Reasoning-Sitz (GLM · Deep Think Max) zuerst stellen, zuletzt
   abholen.

## Welche Linie die Frage trägt

| Frage | Thema | Träger-Linie (Verdikt/Verifikation am Baum) |
|---|---|---|
| M4 | Kugelflächen-Synthese als Feld/Kernel | **Mountain** (Register/Parser/Datenkontrakt) |

Der **Rat** (fünf Stimmen durch die fünf Axiome) trägt das Verdikt.

---

## Das Paket (verbatim an jeden Sitz)

> **An dich, Frontier-Modell.** Antworte als **Urteil, nicht als Erwartung.** Kein Aufsatz —
> Verdikt, Begründung, benannte Risse. Halte die Frage durch die **fünf Axiome** und sprich sie
> durch die **fünf Stimmen** hindurch.
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
> ### M4 · Die Kugelflächen-Synthese als Feld/Kernel
>
> **Frage.** Ein globales **Kugelflächen-Modell** (sphärische Harmonische) liegt gemessen vor — das
> Schwere-Modell ICGEM `gfc`. Es ist kein Punkt-Kernel: sein Wert an einem Ort ist die **Summe über
> alle Koeffizienten** bis zu einem Maximalgrad. **Wie tritt ein solches Modell in die
> 26×f64-Feld-/Kernel-Welt** — als **neue Kernel-Form**, als **eigene Feld-Klasse**, oder gar
> **nicht** als `field`?
>
> **Stand (am Baum gemessen, 2026-10-15).**
> · **Modellkopf XGM2019e** (`https://icgem.gfz.de/getmodel/gfc/…/XGM2019e.gfc`, HTTP-Range):
>   `product_type gravity_field`, `earth_gravity_constant 3.986004415E+14 m³/s²`,
>   `radius 6378136.3 m`, `max_degree 5540`, `norm fully_normalized`, `tide_system zero_tide`.
>   Das ICGEM-Format-Doc (16.02.2023) nennt genau zwei `norm`-Werte (fully_normalized Default ·
>   unnormalized); es liefert **keine** explizite 4π-Formel.
> · **Kern gebaut:** `src/archivar/gfc.rs` trägt `GfcHead`/`parse_head` (Kopf aus dem `.gfc`-Text),
>   `GfcModel` (Kopf + Koeffizienten aus Text/26×f64-Draht), die **fully-normalized 4π-Rekursion**
>   (gegen die Definition `P̄_nm = √((2−δ)(2n+1)(n−m)!/(n+m)!)·P_nm` bis Grad 14 geprüft),
>   `harmonic_sums`/`synthesize` (Geopotenzial `V`, `(n−1)`-gewichtete Anomalie `Δg`). 12/12 Tests grün.
> · **Die `field`-Zeile** ist `<key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`.
>   **Sieben** Kernel-Formen stehen (`kernel_id_of`, `extract.rs:1700`): `inverse-square` (0) ·
>   `gaussian-inverse-square` (1) · `point` (2) · `erfc` (3) · `exponential-decay` (4) ·
>   `patch-levy` (5) · `inverse-linear` (6). **Keine** davon ist eine globale
>   Koeffizienten-Synthese — alle sind Punkt-/Quell-Kernel. Ein Force-Gate verweigert eine
>   forceless Quelle (`extract.rs:1701`).
> · **Haus-Präzedenz:** `IgrfCoeffs::synthesize` (`igrf.rs:131`, Schmidt-Legendre) und
>   `vmf3_sh::b_c` (`vmf3_sh.rs`, Grad 12) sind **eigenständige Funktionen**, **nicht** als Kernel
>   verdrahtet; `gfc` deklariert heute nur seine Koeffizienten-Kanäle (`icgem_gfc_c`/`_s`, unit `1`).
> · Der **Modellkopf steht nicht im Draht**: der 26×f64-Record trägt `C`/`S` in `val`, `n` in
>   `FREQ`, `m` in `BIN_WIDTH`; GM/Radius/Normierung/Tide sind heute **nicht** im Record und **nicht**
>   in der `phi`-Zeile.
>
> **Forschungslage** (weite Leiter, Spill
> `…/recherche/spill/omegaflow_all_1792070842_spherical_harmonic_synthesis_gravity_fie.txt`, 921 Zeilen).
> · Die **„4pi"-Normierung ohne Condon-Shortley-Phase** ist der Geodäsie-Standard: die
>   Wieczorek/Zenodo-Formmodelle (Mond, Mars, Vesta, Ceres, Enceladus, Eros) wurden alle mit
>   **pyshtools `SHCoeffs.expand()`** als „4pi"-normalisiert expandiert — genau die Normierung, die
>   der Kern trägt.
> · Andere Modelle tragen **andere** Normierungen: die Galileischen Monde sind **unnormalized**
>   (Header mit `radius`, `GM`, `k2`; `Anderson2001_Callisto_gravity.sh`), das Mond-Magnetfeld ist
>   **Schmidt semi-normalized** (`T2015_449.sh.gz`). Die Normierung **muss** also je Modell aus dem
>   Kopf kommen, nie als Default.
> · Die Geodäsie exponiert ein SH-Modell als **Funktional-Synthese** (Geopotenzial, Schwereanomalie,
>   Geoidhöhe, Gradienten) — ICGEM-Calculation-Service, Barthelmes STR09/02; die
>   `gravity-toolkit` (Sutterley) rechnet über **Clenshaw-Summation**, der Kern über die
>   Legendre-Rekursion. **Keine** gefundene Schnittstelle führt einen Koeffizientensatz als
>   **Punkt-Kernel**.
> · Für schnelle globale Auswertung existieren **Alternativ-Basen** (cubed-sphere / MRQSphere,
>   Jones 2010 — effektiv konstante Rechenzeit je Grad; Slepian für regional). Das sind
>   **Repräsentations**-Alternativen, keine Feld-Kernel.
>
> **Trägt.** (1) Trägt die SH-Synthese als **`field`** — und wenn ja, welche **Kernel-Form/Signatur**
> (global, parameterisiert durch GM/Radius/Normierung, ausgewertet an `(r, colat, lon)`)? Oder ist
> sie eine **eigene Klasse** (wie die vom Rat benannten `model_output`/`derived`-Typen)? (2) Der
> **Weg des Modellkopfs** zum Auswerter: **(a)** in den 26×f64-Draht (Kopf-Record / erweiterter
> Frame), **(b)** in die `phi`-Zeile (neue Direktiven `gm`/`radius`/`norm`/`tide`), oder **(c)** als
> Kernel-Parameter? (3) Tragen die Records die **Koeffizienten** `(n,m,C,S)` **oder** die
> **synthetisierten Funktionalwerte** auf einem Gitter? (4) Welches Funktional ist die Feldgröße:
> `V`, `Δg`, `N = T/γ₀` — **oder mehrere Zeilen** (wie der M1-Rat für `gdata`/`mgg` „zwei Zeilen,
> nie verschmolzen" forderte)? (5) Die **Truncation** und die **Kosten-Grenze** (der volle
> 5540-Grad-Bau ist ein CI-Lauf; welcher Grad trägt das Feld?). (6) Der **nächste begrenzte Schritt**.
> (7) Der **5. Riss des M1-Verdikts** — „26×f64 fabriziert Gleichheit" — gilt er für die
> Koeffizienten-Zeile (Winkel, nT, mGal, km nebeneinander), und was folgt daraus?

---

## Die Sitze (alle — ein Paket je Sitz)

Der **Roster ist variabel**; die volle Bedienung steht in `~/projects/omegaflow-work/ui-seats.md`
(Suche/Max setzen **und verifizieren** vor dem Senden). Das **langsamste Reasoning-Sitz zuerst**
stellen, zuletzt abholen (GLM · Deep Think Max, Perplexity/Sakana Deep Research). Ein Sitz ohne
Antwort wird als `pending` mit genanntem Grund verbucht — nie als stille Null.

**Je Sitz die Antwort unten unter `Antwort (M4):` einkleben.** Nummerierung ist fest — daran hängt
die Zuordnung.

### 01 · Z.ai / GLM — Deep Think Max
- **Site:** `https://chat.z.ai/`
- **Einstellung:** GLM 5.3 · **Deep Think Max** · **Advanced Search** (Globus, `data-active="true"`)
- **Antwort (M4):**
- **Zeit:**

### 02 · Perplexity — Deep Research
- **Site:** `https://www.perplexity.ai/`
- **Einstellung:** Deep Research · Modell max · Suche (`aria-pressed=true`)
- **Antwort (M4):**
- **Zeit:**

### 03 · Sakana — Deep Research (Namazu)
- **Site:** `https://chat.sakana.ai/`
- **Einstellung:** Namazu (rechnet von sich aus; langsam)
- **Antwort (M4):**
- **Zeit:**

### 04 · Claude — Anthropic
- **Site:** `https://claude.ai/new`
- **Einstellung:** Modell **„Sonnet 5.5 · Extra hoch"** (Extended Thinking max)
- **Antwort (M4):**
- **Zeit:**

### 05 · Qwen — Alibaba
- **Site:** `https://chat.qwen.ai/`
- **Einstellung:** Modell **`Qwen3.8-Max`** · **Denken** (max) · **Web-Suche** an
- **Antwort (M4):**
- **Zeit:**

### 06 · Mistral — EU
- **Site:** `https://chat.mistral.ai/`
- **Einstellung:** **„Denken"** · Suchen (`aria-label="Suchen"`)
- **Antwort (M4):**
- **Zeit:**

### 07 · Gemini AI Studio — Google
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Einstellung:** **„Gemini 3.1 Pro"** · **Grounding with Google Search** an
- **Antwort (M4):**
- **Zeit:**

### 08 · MiniMax — M3
- **Site:** `https://agent.minimax.io/`
- **Einstellung:** **„MaxHermes"/„MaxClaw"** · **Search** an
- **Antwort (M4):**
- **Zeit:**

### 09 · DeepSeek — Chat
- **Site:** `https://chat.deepseek.com/`
- **Einstellung:** **DeepThink** (`aria-pressed=true`) · **Search** (`aria-pressed=true`)
- **Antwort (M4):**
- **Zeit:**

### 10 · Duck.ai — GPT (OpenAI, primär)
- **Site:** `https://duck.ai/`
- **Einstellung:** Modell-Chip „6 Luna" · **Begründung** · **Websuche** an
- **Antwort (M4):**
- **Zeit:**

### 11 · Kimi — Moonshot
- **Site:** `https://kimi.ai/`
- **Einstellung:** Standard (kein K3; K3 nur über tryingopen)
- **Antwort (M4):**
- **Zeit:**

### 12 · Lumo — Proton
- **Site:** `https://lumo.proton.me/guest`
- **Einstellung:** anonym, kein Login
- **Antwort (M4):**
- **Zeit:**

### 13 · Tryingopen — geteilte Open-Weight-Sitze
- **Site:** `https://www.tryingopen.com/`
- **Einstellung:** **ein Tab je Modell** (Modell-Button `[aria-label^="Model: "]`); Zugang nur über
  das Lock `~/projects/omegaflow-work/ui/open-weight.lock` (Halter-Linie + Zeit setzen/löschen)
- **Antwort (M4):**
- **Zeit:**

*(Sitze, die in dieser Runde nicht gestellt wurden — Tageslimit/Login-Wall/Rate-Limit — hier
`pending` mit benanntem Grund eintragen; eine stille Leerzeile ist keine Messung.)*
