# Rat-/Roster-Fragebogen — Kraft-/Kernel-Zuordnung der offenen Messgrößen (Mountain, 2026-10-15)

*(Ein benannter Akt, Operator-Wort 2026-10-15: „… vor dem rat kommt die befragung des rosters
bitte erstelle ein dokument so wie rivers oder myzeliums letzte fragebögen (bedenke 5 stimmen,
5 axiome)". Die **Forschungsschicht** ist gefahren (`bin/all_wide`, 5 Spills unten zitiert).
Der Operator sendet **das Paket** unverändert an jeden Sitz; die Sitz-Antwort trägt **alle**
Fragen. Die Session verifiziert jede Behauptung am Baum, nie die Stimme; das Verdikt trägt der
Rat. Vorgänger-Formulare: `2026-10-15-netzfusion-formular.md` (River),
`2026-10-15-rat-fragenpaket.md` (Mycelium).)*

**Der Katalog führt die Fragen F1–F7.** Dieser Bogen stellt **die drei Kraft-/Kernel-Fragen der
Mountain-Linie** — sie sind **nicht** im Katalog (F2–F7) und werden hier **einmal** gestellt:
**M1** WDC-B-Messgrößen → Kraft/Kernel · **M2** Aurora-Grenzbreite als Feld · **M3**
OVATION-AACGM-Feld/Kernel/Format. Sie gehen in **eine** Nachricht, die der Operator **einmal je
Sitz** einreicht.

## So wird dieses Formular benutzt (für jeden, der es in die Hand nimmt)

1. **Operator:** den Block „Das Paket (verbatim an jeden Sitz)" **unverändert kopieren** und an
   jeden Sitz **einmal** senden. Kein Vorwort, keine Zusatzfrage.
2. **Operator:** je Sitz die Antwort **in die `Antwort (M1–M3)`-Zeile** des passenden Abschnitts
   unter „Die Sitze" kleben und die `Zeit:`-Zeile füllen. Ein Sitz ohne Antwort erhält `pending`
   mit genanntem Grund (Tageslimit/Login), **nie** eine stille Leerzeile.
3. **Session:** verifiziert jede Behauptung **am Baum**, nie die Stimme; schreibt das Verdikt.
4. **Regel:** ein Sitz, der „denkt", ist **nicht** `pending`; erst nach einer echten Frist mit
   benanntem Grund. Das langsamste Reasoning-Sitz (GLM · Deep Think Max) zuerst stellen, zuletzt
   abholen.

## Welche Linie welche Frage trägt (Vorschlag)

| Frage | Thema | Träger-Linie (Verdikt/Verifikation am Baum) |
|---|---|---|
| M1 | WDC-B-Messgrößen → Kraft/Kernel | **Mountain** (Register/Parser/Datenkontrakt) |
| M2 | Aurora-Grenzbreite als Feld/Kernel | **Mountain** (Parser/Register) + **Sensory** (Aurora) |
| M3 | OVATION-AACGM Feld/Kernel/Format | **Mountain** (+ Querverweis auf Katalog-F3 „Aurora als Stationsschalter") |

Der **Rat** (fünf Stimmen durch die fünf Axiome) trägt das gemeinsame Verdikt über alle Linien.

---

## Das Paket (verbatim an jeden Sitz)

> **An dich, Frontier-Modell.** Antworte als **Urteil, nicht als Erwartung.** Kein Aufsatz —
> je Frage: Verdikt, Begründung, benannte Risse. Halte jede Frage durch die **fünf Axiome** und
> sprich sie durch die **fünf Stimmen** hindurch.
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
> ### M1 · WDC-B-Messgrößen — welche Kraft, welcher Kernel?
>
> **Frage.** Die WDC-B-Rettung (Obninsk) hat **205 ladbare Payload-Dateien** ins Register gezogen.
> Jetzt fehlt die `field`-Zeile je Datensatz. Welche **Kraft** und welchen **Kernel** trägt jede
> der sieben gemessenen Messgrößen in der 26×f64-Welt — und welche trägt **gar kein Feld** (dann
> ehrlich absent, keine erfundene Zeile)?
>
> **Stand (gemessen, Belege je `datei:zeile`).**
> · `sep/GravConst/data` → Gravitationskonstante G, CGS `cm³ g⁻¹ s⁻²` (`G_lib.h:31,41`).
> · `stp/geo_min.val` → 1-Minuten-Geomagnetik D,F,H,P,R,X,Y,Z; nT (D in Zehntel-Bogenminuten).
> · `sep/data/gdata` → Freiluft-Schwereanomalie (mGal) + Geoidhöhe (m); `mgg/*` Meeresgravimetrie (0,1 mGal).
> · `sep/data/sdata` → Erdbeben-Herdparameter (Breite/Länge, Tiefe km, Magnitude).
> · `stp/geomagni.ind/aa` → 3-stündlicher aa-Index, nT (gamma).
> · `stp/catal_pc` → Pc1-Perlenpulsationen (Begin/End/T/Amp/Type).
> · `gps` → GNSS-Metadaten (RINEX v2, GAMIT-Tabellen).
> Haus-Präzedenz: SH-Modelle (`IgrfCoeffs::synthesize`, `vmf3_sh::b_c`, `gfc`/ICGEM),
> `Position::Surface` für ortsgebundene Punktmessungen.
>
> **Forschungslage** (weite Leiter, Spill `…1792061923_geomagnetic_field_components_spherical_h.txt`,
> `…1792061927_spherical_harmonic_gravity_model_kernel_.txt`). Geomagnetik trägt als
> Kugelflächen-Modell (IGRF-13; pfm9k; „Fast and accurate evaluation of geomagnetic field elements",
> GJI 2020). Gravitation trägt als Geopotenzial-SH-Modell. Pc1-Perlen sind **EMIC-Wellen** (Bortnik
> 2007 automatische Wellendetektion; Prikner 2004 IAR-Resonator; Guglielmi/Potapov 2026 „Pearls") —
> also ein **Wellenfeld**, an Stationen gemessen. Erdbeben-Herde sind ein **Punkt-Prozess-Katalog**
> (Ort/Tiefe/Magnitude), kein Kraftfeld. **Keine** gefundene Zuordnung sagt, wie eine **einzelne
> Stations-Zeitreihe** (nT, 1-min) oder eine **Konstante** (G) als Feld zu tragen ist.
>
> **Trägt.** (1) Je Messgröße: **Kraft + Kernel + SI-Einheit**. (2) Die `field`-Zeile — **oder** die
> begründete Absenz (welche Größe ist kein Feld: z. B. GNSS-Metadaten, Erdkonstante G). (3) Die
> Geometrie: `Position::Surface` je Station (Punktmessung) vs. globales Feld. (4) Der Riss, wo die
> Doktrin „alles ist ein Feld" an einer Messgröße geschlagen ist. (5) Der nächste begrenzte Schritt.
>
> ### M2 · Aurora-Grenzbreite als Feld/Kernel
>
> **Frage.** BAS IMAGE (SI12) und DMSP-SSJ (FOM) liefern die **equatorwärtige/polwärtige
> Aurora-Oval-Grenze** je MLT-Sektor. Wie wird diese Grenzbreite als **Zeuge/Feld** in die Welt
> getragen — als **S²-Ort** (Richtung, wie die Astrometrie), als **Leitfähigkeits-/Strom-Skalar**,
> oder als **geschlossene Level-Achse je MLT-Bin** (wie SABERs Höhenform (C))?
>
> **Stand (gemessen 2026-10-15, Taucher).**
> · **BAS IMAGE SI12** (`ramadda.nerc-bas.ac.uk`, OGL v3.0): 30 Monats-`.txt`, **50 Spalten** —
>   Zeit UT, Instrument-Code (0 WIC/1 SI12/2 SI13), 3–26 equatorwärtige AACGM-Breiten für die 24
>   einstündigen MLT-Bins `00:30…23:30`, 27–50 polwärtige, gleiche Reihenfolge; `NaN` = fehlender Bin.
>   Takt ~2 min. sha256 der Zip `ceb8d697…`.
> · **DMSP-SSJ FOM** (Zenodo `10.5281/zenodo.3373812`, CC-BY-4.0): `fom_boundaries_…zip`,
>   sha256 `aae69e9a…`; **16-Spalten-CSV** mit `#`-Glossar; eine Zeile = **ein Polar-Pass** mit vier
>   Grenzpunkten EQ1/PO1/PO2/EQ2 (mlat + mlt), `hemisphere` ±1, `FOM`. Satellit/Datum nur im Dateinamen.
>   **Riss:** die CSV-Bytes waren nicht entpackbar (kein unzip im Sandkasten) → 16-vs-24-Spalten offen
>   (Hersteller-Default `['mlat','mlt']`).
>
> **Forschungslage** (weite Leiter, Spill `…1792061917_auroral_oval_equatorward_poleward_bounda.txt`).
> Die Grenze ist an **Leitfähigkeit/Feld-aligned current** gebunden (Ahn 2020, „Statistical Relations
> Between Auroral Electrical Conductances and Field-Aligned Currents"; CHAMP-FAC-Grenze 2014);
> Lunyushkin/Penskikh diagnostizieren die Oval-Grenzen per **Magnetogramm-Inversion** (kein direktes
> Feld). Der Grenzort trägt als **geometrischer Locus** (MLT × Breite × Zeit), nicht als Kraftwert.
>
> **Trägt.** (1) Trägt die Grenzbreite als **S²-Ort** (Richtung) — oder als **Skalar auf einer
> geschlossenen MLT-Achse** — oder beides (Ort + Konduktanz-Begleiter)? (2) Kraft/Kernel je Spur.
> (3) Die Achse: MLT-Bin **geschlossen diskret** (wie SABER Level) — gehört der Bin in den Schlüssel?
> (4) `NaN`/fehlender Bin als **`None`**, nie als 0.0. (5) Der nächste begrenzte Messschritt.
>
> ### M3 · OVATION-AACGM — Feld/Kernel/Format
>
> **Frage.** OVATION-AACGM (Zenodo `10.5281/zenodo.19651247`, CC-BY-4.0, **eine** Datei
> `OVATION_AACGM.zip`, **eine Station Mohe**) ist ein operativer Aurora-Nowcast. Welcher
> **Feld-/Kernel-/Format-Kontrakt** trägt ihn — als **Partikel-Prezipitations-/Energiefluss-Feld**
> oder als **Stationsschalter** (Katalog-F3)?
>
> **Stand (gemessen 2026-10-15).** Eine Station, nicht „je Station" (Riss gegen river-214). Span
> und Takt nicht im Zip-Kopf gelesen.
>
> **Forschungslage** (weite Leiter, Spill `…1792061915_OVATION_Prime_auroral_precipitation_ener.txt`).
> OVATION Prime / Prime-2013 (Newell) und PrecipNet (McGranaghan 2021) liefern die
> **Partikel-Prezipitation/hemisphärische Leistung** als Nowcast (NOAA SWPC „aurora 30-minute
> forecast"; CCMC „Ovation-Prime 2.3"; `10.1029/2014SW001056`). Das Oval ist Funktion von
> Kp/Bz/Sonnenwind.
>
> **Trägt.** (1) Trägt als **Energiefluss-Feld** — welche Kraft, welcher Kernel? (2) Format/Record
> für **eine** Station. (3) Der Riss „eine Station vs. globales Modell" — was ist der Zeuge wirklich?
> (4) Der Zusammenhang/Unterschied zu Katalog-F3. (5) Nächster Schritt.

---

## Die Sitze (alle — ein Paket je Sitz)

Das langsamste Sitz (GLM · Deep Think Max) zuerst stellen, zuletzt abholen. Der Roster ist
variabel; gefahren werden die bekannten Einstellungen (`~/projects/omegaflow-work/ui-seats.md`).
Ein Sitz ohne Antwort wird als `pending` mit genanntem Grund verbucht — nie als stille Null.

### 01 · Z.ai / GLM
- **Site:** `https://chat.z.ai/`
- **Modell/Einstellung:** GLM 5.3 · Deep Think Max · Advanced Search
- **Zeit:**
- **Antwort (M1–M3):**

### 02 · Perplexity (Deep Research)
- **Site:** `https://www.perplexity.ai/`
- **Modell/Einstellung:** Deep Research
- **Zeit:**
- **Antwort (M1–M3):**

### 03 · Sakana Chat (Deep Research)
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Deep Research
- **Zeit:**
- **Antwort (M1–M3):**

### 04 · Duck.ai
- **Site:** `https://duck.ai/`
- **Modell/Einstellung:** GPT (primär OpenAI) · Websuche
- **Zeit:**
- **Antwort (M1–M3):**

### 05 · Claude
- **Site:** `https://claude.ai/new`
- **Modell/Einstellung:** Claude Sonnet 5.5
- **Zeit:**
- **Antwort (M1–M3):**

### 06 · Qwen
- **Site:** `https://chat.qwen.ai/`
- **Modell/Einstellung:** Qwen (Default)
- **Zeit:**
- **Antwort (M1–M3):**

### 07 · Gemini AI Studio
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Modell/Einstellung:** Gemini 3.1 Pro
- **Zeit:**
- **Antwort (M1–M3):**

### 08 · DeepSeek Chat
- **Site:** `https://chat.deepseek.com/`
- **Modell/Einstellung:** DeepSeek Chat · V4 Pro (tryingopen)
- **Zeit:**
- **Antwort (M1–M3):**

### 09 · Mistral
- **Site:** `https://chat.mistral.ai/`
- **Modell/Einstellung:** Mistral (EU)
- **Zeit:**
- **Antwort (M1–M3):**

### 10 · Kimi
- **Site:** `https://www.kimi.ai/`
- **Modell/Einstellung:** Kimi (schneller Modus)
- **Zeit:**
- **Antwort (M1–M3):**

### 11 · MiniMax M3
- **Site:** `https://agent.minimax.io/`
- **Modell/Einstellung:** MiniMax M3
- **Zeit:**
- **Antwort (M1–M3):**

### 12 · Lumo
- **Site:** `https://lumo.proton.me/`
- **Modell/Einstellung:** Proton Lumo
- **Zeit:**
- **Antwort (M1–M3):**

---

## Die Forschungsschicht (belegt)

| Frage | weite Leiter (`bin/all_wide`) | Spill (vollständig lesen) |
|---|---|---|
| M1 Geomagnetik | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061923_geomagnetic_field_components_spherical_h.txt` |
| M1 Gravitation | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061927_spherical_harmonic_gravity_model_kernel_.txt` |
| M1 Pc1-Pulsationen | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792062199_Pc1_pearl_pulsation_ionospheric_waveguid.txt` |
| M1 Erdbeben-Herde | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792062208_earthquake_hypocenter_catalog_point_fiel.txt` |
| M2 Aurora-Grenze | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061917_auroral_oval_equatorward_poleward_bounda.txt` |
| M3 OVATION | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061915_OVATION_Prime_auroral_precipitation_ener.txt` |
| (Neben) Celestrak-Terms | ✅ | `omegaflow-work/recherche/spill/omegaflow_all_1792061919_Celestrak_GP_data_terms_of_use_Space_Tra.txt` |

**Neben-Befund (kein Rat, eine Messung):** der Celestrak-GP-Lizenz-Token ist über die
**Usage-Policy** erreichbar — die Seite `http://www.celestrak.org/usage-policy.php` existiert
(Wayback-Snapshot `20260516161024`, HTTP 200; Direkt-Abruf pending, kein Proton-Transit). Der
Token wird als `celestrak-usage-policy https://celestrak.org/usage-policy.php` vorgeschlagen;
der volle Policy-Text ist noch zu lesen (Mountain/Mycelium), dann setzt Mycelium den Block.
