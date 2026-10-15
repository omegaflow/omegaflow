# Rat-/Roster-Fragebogen — Die global gemessene Gitter-Form (M5, Mountain, 2026-10-15)

*(Ein benannter Akt. Die **Forschungsschicht** ist gefahren (`bin/all_wide`, zwei Spills unten zitiert).
Der Operator sendet **dieses Paket** unverändert an jeden Sitz; die Sitz-Antwort trägt die Frage. Die
Session verifiziert jede Behauptung am Baum, nie die Stimme; das **Verdikt trägt der Rat** (fünf
Stimmen durch die fünf Axiome des Granits). Vorgänger: M1–M3 (`2026-10-15-mountain-rat-fragen.md`),
M4 (`2026-10-15-mountain-rat-fragen-sh-kernel.md`), Verdikte dazu.)*

Der M4-Rat sprach: die **Kugelflächen-Synthese** ist nicht achte Kernel-Form und keine `field`-Zeile,
sondern eine **eigene Klasse**; Gitter sind `derived`. **M5 stellt die nächste Tür derselben Familie:**
ein **global gemessenes, regelmäßig vergittertes Skalarfeld** — konkret das WGLC-Blitzdichte-Gitter —
hat *keinen Ort der Quelle*; es trägt seinen Wert an einer **Zelle**. Wie tritt es ein?

## So wird dieses Formular benutzt (für jeden, der es in die Hand nimmt)

1. **Operator:** den Block „Das Paket (verbatim an jeden Sitz)" **unverändert kopieren** und an jeden
   Sitz **einmal** senden. Kein Vorwort, keine Zusatzfrage.
2. **Operator:** je Sitz die Antwort in die `Antwort (M5):`-Zeile des passenden Abschnitts unter
   „Die Sitze" kleben und die `Zeit:`-Zeile füllen. Ein Sitz ohne Antwort erhält `pending` mit
   genanntem Grund (Tageslimit/Login), **nie** eine stille Leerzeile.
3. **Session:** verifiziert jede Behauptung **am Baum**, nie die Stimme; schreibt das Verdikt.
4. **Regel:** ein Sitz, der „denkt", ist **nicht** `pending`; erst nach einer echten Frist mit
   benanntem Grund. Das langsamste Reasoning-Sitz (GLM · Deep Think Max) zuerst stellen, zuletzt abholen.

## Welche Linie die Frage trägt

| Frage | Thema | Träger-Linie (Verdikt/Verifikation am Baum) |
|---|---|---|
| M5 | Die global gemessene Gitter-Form | **Mountain** (Register/Parser/Datenkontrakt) |

Der **Rat** (fünf Stimmen durch die fünf Axiome) trägt das Verdikt. Die **Engine-Format-/Membran**-Seite
(Zeitachse, `bin`/`cond`) ist Rivers; das Ablegen (CDN, `url`/`origin`/`compiler`) ist Myceliums.

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
> ### M5 · Die global gemessene Gitter-Form
>
> **Frage.** Ein **global gemessenes, regelmäßig vergittertes Skalarfeld** liegt gemessen vor — das
> WWLLN Global Lightning Climatology (WGLC): eine **tägliche** globale Dichte auf einem **0,5°-Gitter**
> (Zeitreihe `density [Tage, 360, 720]`; eine 5-Winkelminuten-Fassung daneben). Es ist **kein
> Punkt-Kernel**: der Wert an einem Ort ist der **Zellwert**, nicht die Antwort einer Quelle auf eine
> Distanz. **Wie tritt ein solches Gitter in die 26×f64-Register-/Feld-Welt** — als **neue
> Kernel-Form**, als **eigene Klasse/Register-Typ**, oder **gar nicht** als `field`?
>
> **Stand (am Baum gemessen, 2026-10-15).**
> · **Die Quelle WGLC** (Zenodo concept `10.5281/zenodo.4774528`, Version-Record `20277101`):
>   `CC-BY-4.0` laut Record-Metadaten, **Riss** mit dem README-Badge `CC BY-SA 4.0` (beide Zeilen,
>   nicht geglättet). README verbatim: „gridded at 0.5 degree 5 arc-minute and hourly resolution,
>   converted into density"; Variablen `density` (strokes km⁻² d⁻¹), `power_mean/median/SD` (MW);
>   Zeit `5479` (daily) / `180` (monthly) / `12` (climatology). SRS: geographic (WGS84), 2010–2025.
>   Format NetCDF4 **= HDF5-Container** (`--sniff`: `magic hdf5`, SHA-256 `98aafd8b…`). **Riss:**
>   README nennt „hourly", die Zeitreihe trägt **täglich** (die Dichte-Datei `[5844,360,720]`,
>   Schritt 1 Tag) — gemessen; benannt, nicht geglättet.
> · **Der Compiler steht** (`tools/harvest/src/bin/wglc_compiler.rs`, Mycelium).
> · **Die `field`-Zeile** ist `<key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`.
>   **Sieben** Kernel-Formen stehen (`kernel_id_of`, `src/archivar/extract.rs:1700`):
>   `inverse-square` (0) · `gaussian-inverse-square` (1) · `point` (2) · `erfc` (3) ·
>   `exponential-decay` (4) · `patch-levy` (5) · `inverse-linear` (6) — **alle Punkt-/Quell-Kernel**;
>   das Force-Gate verweigert eine forceless Quelle (`src/archivar/membrane.rs:456`). Die Zeile
>   verlangt τ/Absorption/Advection — für ein Gitter alle drei ohne Referenten.
> · **Haus-Präzedenz (M4):** die SH-Synthese trat **nicht** als `field` ein, sondern als eigene
>   Klasse (`model_output`/`derived`); Koeffizienten im Draht, Gitter `derived`. Der WDC-B/M1-Rat
>   benannte den **Absenz-Pfad** für Nicht-Feld-Typen (`scalar_constant`/`point_process`/
>   `planetary_index`/`wave_event`/`metadata`) als **Schema-Erweiterung (Rat)**. Die
>   Aurora-Grenzbreite (BAS-IMAGE/DMSP-SSJ) wurde als **Ort auf einer geschlossenen MLT-Achse**
>   verdiktet, nicht als Kraftfeld.
> · **Register-Präzedenz (gemessen):** heutige globale Gitter stehen teils als `field`-Zeilen mit
>   `inverse-square` (z. B. Wolkenbedeckung ISCCP-HGM/PATMOS-x im Fäden-Entwurf) — dieselbe
>   Kategorien-Frage.
>
> **Forschungslage** (weite Leiter, Spills
> `omegaflow-work/recherche/spill/omegaflow_all_1792080805_global_gridded_scalar_field_netCDF_CF_co.txt` (784 Z.)
> und `…_1792081383_particle_in_cell_grid_field_versus_point.txt` (718 Z.)).
> · **CF Metadata Conventions v1.11** (netCDF) tragen ein globales Gitter als: lat/lon-Achsen
>   (`units=degrees_north`/`degrees_east`, `axis=Y`/`X`), `grid_mapping_name="latitude_longitude"`,
>   eine Zeit-Achse (`units="… since …"`, `calendar`), **`bounds`** (Zellgrenzen) und
>   **`cell_measures="area: cell_area"`** (Zellfläche explizit, wo nicht ableitbar), und
>   **`cell_methods`** (`point`/`mean`/`sum (interval: …)`) — die Zelle ist ein **Wert mit
>   Ausdehnung**, kein Quellpunkt. Leerzellen sind **`_FillValue`/`missing_value`** (erste Klasse,
>   nie 0.0); Interpolation bewahrt `NaN` (scipy/xarray), glättet nicht.
> · **Alternative Basen:** HEALPix (flächentreue Pixel an `N_side`, Zelle = Index), EASE-Grid 2.0,
>   hexagonale DGGS. **Fernerkundungs-/Archiv-Formate:** GeoTIFF (`ModelTiepointTag`/`ModelPixelScale`/
>   `RasterPixelIsArea`), FITS-WCS (Pixelzentrum + WCS), SuperDARN-Grid als netCDF (AACGM v2).
> · **Engine-Präzedenz (gemessen):** in WarpX (PIC), Gkeyll (DG), Dedalus (spektral) ist das
>   **Gitterfeld ein erstklassiges Objekt**; ein Punkt/ein Partikel ist eine **Kopplung** (deposit/
>   interpolate) hinein, keine eigene Feld-Sorte. „Gitterfeld" und „Punktquelle" sind **getrennte
>   Register**.
> · **Narrow Schluss der Forschung:** jede gemessene Konvention behandelt das global vergitterte
>   gemessene Skalar als **andere Sorte** als den Punkt-Oszillator: kein Quellort, Wert-an-Zelle,
>   Zelle = Wert-mit-Ausdehnung (Fläche), Leerzelle = Gap. Der Weg der netCDF/CF-Seite ist ein
>   **Koordinaten+Bounds+cell_measures-Dreiklang**, nicht ein Pol/Kraft/τ-Satz.
>
> **Trägt.** (1) Trägt das Gitter als **`field`** — und wenn ja, welche **Kernel-Form/Signatur**
> (global, ausgewertet an `(colat, lon, t)`)? Oder ist es eine **eigene Klasse/Register-Typ** (wie
> die vom Rat benannten `model_output`/`derived`) — und braucht es einen **neuen Typ `grid`**?
> (2) **Welche minimale ehrliche Signatur** trägt die Klasse (Zelle + Achsen + Zeit + Provenienz)?
> (3) Der **Draht:** wird eine **Gitterzelle ein Record** im 26×f64-Frame, oder trägt die Klasse einen
> **eigenen Träger/Frame** (die WGLC-Tagesreihe hat 360×720 = 259 200 Zellen je Tag über 5844 Tage)?
> (4) **Takt/Zeitform:** wie trägt der Eintrag die tägliche Zeitform (und die monatliche
> Leistung) — ohne einen stündlichen Default (`bin 3600`)? (5) Der **Zellwert**: Dichte (strokes km⁻² d⁻¹)
> mit **Zell-Fläche** — ist die Fläche ein Pflicht-Träger, und gilt der **5. Riss** (Einheiten-Overload)
> für die Gitterzeile? (6) Die **Leerzelle/Gap**: `_FillValue`/`pending` — nie 0.0; wie trägt die
> Klasse das? (7) Der **nächste begrenzte Schritt** (WGLC konkret).

---

## Die Sitze (alle — ein Paket je Sitz)

Der **Roster ist variabel**; die volle Bedienung steht in `~/projects/omegaflow-work/ui-seats.md`
(Suche/Reasoning/Max setzen **und verifizieren** vor dem Senden). Das **langsamste Reasoning-Sitz
zuerst** stellen, zuletzt abholen (GLM · Deep Think Max, Perplexity/Sakana Deep Research). Ein Sitz
ohne Antwort wird als `pending` mit genanntem Grund verbucht — nie als stille Null.

**Je Sitz die Antwort unten unter `Antwort (M5):` einkleben.** Nummerierung ist fest — daran hängt die Zuordnung.

### 01 · Z.ai / GLM — Deep Think Max
- **Site:** `https://chat.z.ai/`
- **Einstellung:** GLM 5.3 · **Deep Think Max** · **Advanced Search** (Globus, `data-active="true"`)
- **Antwort (M5):**
- **Zeit:**

### 02 · Perplexity — Deep Research
- **Site:** `https://www.perplexity.ai/`
- **Einstellung:** Deep Research · Modell max · Suche (`aria-pressed=true`)
- **Antwort (M5):**
- **Zeit:**

### 03 · Sakana — Deep Research (Namazu)
- **Site:** `https://chat.sakana.ai/`
- **Einstellung:** Namazu (rechnet von sich aus; langsam)
- **Antwort (M5):**
- **Zeit:**

### 04 · Claude — Anthropic
- **Site:** `https://claude.ai/new`
- **Einstellung:** Modell **„Sonnet 5.5 · Extra hoch"** (Extended Thinking max)
- **Antwort (M5):**
- **Zeit:**

### 05 · Qwen — Alibaba
- **Site:** `https://chat.qwen.ai/`
- **Einstellung:** Modell **`Qwen3.8-Max`** · **Denken** (max) · **Web-Suche** an
- **Antwort (M5):**
- **Zeit:**

### 06 · Mistral — EU
- **Site:** `https://chat.mistral.ai/`
- **Einstellung:** **„Denken"** · Suchen (`aria-label="Suchen"`)
- **Antwort (M5):**
- **Zeit:**

### 07 · Gemini AI Studio — Google
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Einstellung:** **„Gemini 3.1 Pro"** · **Grounding with Google Search** an
- **Antwort (M5):**
- **Zeit:**

### 08 · MiniMax — M3
- **Site:** `https://agent.minimax.io/`
- **Einstellung:** **„MaxHermes"/„MaxClaw"** · **Search** an
- **Antwort (M5):**
- **Zeit:**

### 09 · DeepSeek — Chat
- **Site:** `https://chat.deepseek.com/`
- **Einstellung:** **DeepThink** (`aria-pressed=true`) · **Search** (`aria-pressed=true`)
- **Antwort (M5):**
- **Zeit:**

### 10 · Duck.ai — GPT (OpenAI, primär)
- **Site:** `https://duck.ai/`
- **Einstellung:** Modell-Chip „6 Luna" · **Begründung** · **Websuche** an
- **Antwort (M5):**
- **Zeit:**

### 11 · Kimi — Moonshot
- **Site:** `https://kimi.ai/`
- **Einstellung:** Standard (kein K3; K3 nur über tryingopen)
- **Antwort (M5):**
- **Zeit:**

### 12 · Lumo — Proton
- **Site:** `https://lumo.proton.me/guest`
- **Einstellung:** anonym, kein Login
- **Antwort (M5):**
- **Zeit:**

### 13 · Tryingopen — geteilte Open-Weight-Sitze
- **Site:** `https://www.tryingopen.com/`
- **Einstellung:** **ein Tab je Modell** (Modell-Button `[aria-label^="Model: "]`); Zugang nur über
  das Lock `~/projects/omegaflow-work/ui/open-weight.lock` (Halter-Linie + Zeit setzen/löschen)
- **Antwort (M5):**
- **Zeit:**

*(Sitze, die in dieser Runde nicht gestellt wurden — Tageslimit/Login-Wall/Rate-Limit — hier
`pending` mit benanntem Grund eintragen; eine stille Leerzeile ist keine Messung.)*
