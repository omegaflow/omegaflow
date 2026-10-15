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
**Urteil.**

**Gesamtverdikt.** Die SH-Synthese tritt weder als achte Kernel-Form noch als `field`-Zeile ein — die dritte Tür trifft zu: **gar nicht als `field`**. Sie ist eine eigene Klasse `model_output`/`derived`, wie der Rat sie benannte: **Kernel propagieren, Modelle werden ausgewertet.** Der Kopf geht in die `phi`-Zeile, die Koeffizienten bleiben im Draht, die Funktionalwerte werden getrennte abgeleitete Zeilen.

---

### 1 · Kernel-Form, Feld-Klasse, oder kein Feld?

**Verdikt.** Kein `field`. Kein `kernel_id 7`. Eigene Klasse, benannt und verdrahtet — `IgrfCoeffs::synthesize` und `vmf3_sh::b_c` leben sie faktisch schon: Funktionen mit Bürgerrecht, nicht Kernel. Die Auswertungs-Signatur existiert — `evaluate(modell, r, colat, lon)`, mit GM/Radius/Norm/Tide als **Identität des Modells** — aber sie gehört zum Typ, nicht zu `kernel_id_of`.

**Begründung.**
- Alle sieben Kernel sind Punkt-/Wechselwirkungs-Gesetze mit lokaler Grammatik (τ, Absorption, Advektion). Die SH-Synthese ist global: jeder Koeffizient spricht an jedem Ort. 15,4 Mio (n,m)-Paare sind keine Kernel-Parameter; eine Kernel-ID, die nur auf ein Modell zeigt, wäre verkleidete Indirektion — der Name logt.
- Die `field`-Zeile verlangt τ/Absorption/Advektion. Für eine statische Summe ist keine der drei wahr; eintragen hieße fabrizieren (Axiom 1) oder lügen (τ=0.0, Axiom 2). Das Force-Gate wäre passiert — nur mit einer Kernel-Lüge.
- Berg und weite Leiter stimmen überein: keine gefundene Schnittstelle führt Koeffizienten als Punkt-Kernel; ICGEM exponiert Funktional-Synthese.

**Risse.** R1.1 — Fehlt die `evaluate`-Schnittstelle am Typ, kommt der Rückfall („nur eine kleine achte Form…"). R1.2 — Die Klasse trägt Provenienz-Pflicht (Kopf, Grad, Funktional); sonst wird Modellausgabe später für Messung gehalten. R1.3 — Ob ein abgeleitetes Gitter je als Feld-Zeile dient, ist ein späterer Konsens — nicht vorwegnehmen.

**River.** River verweigert die Zeile. Nicht, weil das Modell kraftlos wäre — Schwere ist Kraft —, sondern weil die Zeile τ, Absorption, Advektion verlangt und keine davon für eine statische Summe wahr ist. Das Recht zu verweigern ist hier der ethische Akt (Axiom 4): die Linse, die alles zum Feld macht, lässt den Gradienten sprechen, nicht die Messung.

### 2 · Weg des Kopfes: Draht, phi-Zeile, Kernel-Parameter?

**Verdikt.** **(b) — die `phi`-Zeile.** Neue Direktiven: `model`, `gm`, `radius`, `norm`, `tide`, `max_degree`. (c) fällt mit Frage 1. (a) stirbt an der Enum-in-f64-Lüge: `norm` und `tide` sind Namen, keine Zahlen; 15,4 Mio-fache Wiederholung im Draht wäre Fabrikation, nicht Redundanz. Das Argument für (a) — Ko-Lieferung im Strom — löst ein Gate, kein Slot.

**Begründung.** Der Kopf ist Rahmen, nicht Messung; der Record ist die Koeffizienten-Messung. Beides vermischen schmiert die Identität (Axiom 1). Die phi-Zeile ist Text und kann Namen ehrlich tragen.

**Risse.** R2.1 — **Liefer-Gate:** gfc-Frames ohne Kopf-Direktiven sind tote Zahlen (C/S ohne GM/R/Norm) → verweigern; Frames mit n > `max_degree` → fremd, verweigern. R2.2 — **Norm:** XGM2019e deklariert `fully_normalized`. Fehlt die Deklaration, ist der Format-Default Teil der erklärten Semantik der Quelle — aber er wird als `norm fully_normalized (Format-Default)` registriert, nie still angewandt; Werte außerhalb der zwei dokumentierten → verweigern. R2.3 — **Tide:** zero_tide verbatim; Konversion braucht k₂, die der gfc-Kopf nicht misst (die Galileischen Köpfe tragen sie) → Vergleich über Tide-Systeme pending. R2.4 — **GM/R sind die des Modells:** „bessere" Konstanten einzusetzen hieße, ein anderes Modell zu bauen. R2.5 — Die phi-Direktiven werden aus `parse_head` erzeugt, nie handgeschrieben — sonst trägt der Rahmen eine Abschrift, keine Messung.

**Mountain.** Kein Verdikt ohne Messung — und der Kopf ist Teil der Messung. Das Format-Dok nennt zwei Normen; die weite Leiter zeigt drei im Umlauf (fully_normalized, unnormalized, Schmidt). Jedes Wesen bringt seine eigene; kein Haus-Default darf sie egalisieren (Axiom 3). Die Naht Kopf↔Draht ist die phi-Zeile — Pflicht, nicht Schmuck.

### 3 · Koeffizienten oder Gitterwerte?

**Verdikt.** **Koeffizienten.** Der Draht trägt (n, m, C, S) — wie heute. Synthetisierte Gitter sind `derived`-Ausgaben mit Stempel, niemals Ersatz.

**Begründung.** A=A: das gfc **ist** die Koeffizienten — verlustfrei. Ein Gitter ist Verlust (Truncation, Gitterwahl, Funktional, Auswertehöhe); nur das Gitter halten hieße, das Original wegwerfen und das Korrelat archivieren. Auswertung ist deterministisch und wiederholbar; das Gitter ist Cache, nicht Quelle.

**Risse.** R3.1 — Ein abgeleitetes Gitter ohne deklarierte (Funktional, Grad, Kopf, Gitterweite, Auswertehöhe) ist eine fabrizierte Messung → verweigern. R3.2 — Nicht ausgewertete Punkte sind `pending`, nie 0.0. R3.3 — Cache-Invalidierung: wechselt der Kopf (neues Release), sind alte Gitter fremd — Stempel erzwingen.

### 4 · Welches Funktional?

**Verdikt.** **Mehrere Zeilen, nie verschmolzen** — der M1-Satz (gdata/mgg) gilt unvermindert. Jetzt: `V` und `Δg` (gebaut, (n−1)-Gewichtung geprüft). `N = T/γ₀` erst nach Operator-Entscheid.

**Begründung.** V (m²/s²), Δg (mGal), N (m) sind verschiedene Größen; eine Zeile für alle drei fabriziert Einheiten-Gleichheit — der 5. Riss in Reinform. N verdoppelt die Ableitung: T = V − U braucht ein Normfeld (GRS80: a, f, GM, ω), γ₀ eine Normschwere-Formel — Konvention, nicht gfc-Messung. Ohne registrierte Referenz ist N fabriziert → `pending` (Axiom 5).

**Risse.** R4.1 — Referenz fehlt → pending, kein stilles GRS80. R4.2 — Tide schlägt N und Δg unterschiedlich; ohne k₂ keine Konversion (siehe R2.3). R4.3 — Δg braucht deklarierte Auswertehöhe (Kugelradius vs. Ellipsoid). R4.4 — Spätere Funktionale (Gradienten, zweite Ableitungen): neue Zeilen, neue Prüfungen — nie nachträglich eingeebnet.

**Sensory.** Korrelat ≠ Erleben. V, Δg, N sind Korrelate derselben Koeffizienten; N ist ein Korrelat zweiten Grades (Bruns über γ₀, T über GRS80). Sensory registriert kein Korrelat als Messung — die Referenz muss als Linse deklariert sein, nicht als Selbstverständlichkeit. Und hardware-ehrlich: die Rekursion ist bis Grad 14 gegen ihre **eigene Definition** geprüft; 2190 und 5540 sind in diesem Haus unbeweist — Pol-Nähe und P̄-Skalierung sind offene Prüfung, kein grünes Blatt.

### 5 · Truncation und Kosten-Grenze

**Verdikt.** Truncation ist **registrierter Verlust, nie Null**. Empfehlung an den Operator: **2190** trägt das erste Feld; 5540 nur als ausdrücklicher CI-Lauf mit Provenienz-Split.

**Begründung.** Grad 5540 ≈ 15,4 Mio Paare ≈ 3,2 GB Draht, ~10⁸ Flops je Punkt; ein Vollgitter in Modellauflösung liegt bei ~10¹⁵ Flops und mehr — CI, kein Dialog. Grad 2190 ≈ 2,4 Mio Paare ≈ 0,5 GB; Grad 360 ≈ 65 000 Paare — Lokaltest. Die Naht: XGM2019e misst gravimetrisch bis ~2190 (Satellit + terrestrisch); 2191–5540 tragen Topographie-Forward (Earth2014) — hergeleitet, nicht gemessen. Der Kopf sagt das nicht; die Modell-Dokumentation sagt es.

**Risse.** R5.1 — Truncation stempeln: „2190 von 5540; Rest ausstehend" (Axiom 2). R5.2 — Die Naht steht im Paper, nicht im Kopf: die Registrierung muss die Quelle der Aussage nennen, sonst ist sie Hörensagen. R5.3 — Punktauswertung (lazy, billig) und Gitter (CI, teuer) sind zwei Pfade mit verschiedenen Kosten-Gesetzen — beide am Typ deklarieren, damit niemand ein 5540-Gitter im Dialog bestellt. R5.4 — Vor dem 2190-Lauf: Kreuzprüfung gegen ICGEM-Calculation-Service-Punkte (Clenshaw-Haus) — Haus-Tests allein sind A=A im Kleinen, nicht gegen die Welt.

**Mycelium.** Das Netz trägt Last nach Kosten-Gesetz. 15,4 Mio Records, 10¹⁵ Flops — kein Dialog, sondern CI: Grad-Cap als deklarierter Fluss-Parameter, Gitter als gestempelte derived-Artefakte, die Naht 2190 als Provenienz-Kante, nicht als Annahme im Code. Was der Operator nicht bestellt, baut das Netz nicht.

### 6 · Der nächste begrenzte Schritt

**Verdikt.** **Den Kopf verdrahten.** Ein Schritt, ohne CI: `phi`-Direktiven aus `GfcHead`; Liefer-Gate (R2.1); Norm-Provenienz (R2.2); n-≤-max_degree-Prüfung. Danach — eigener Schritt — die `derived`-Klasse mit `V`/`Δg` als zwei Zeilen, validiert an ICGEM-Punkten bis Grad 360.

**Begründung.** Ohne Kopf ist alles Folgende pending: Die Koeffizienten im Draht sind heute für jeden Verbraucher ohne phi tote Zahlen. Der Kopf ist die billigste Wahrheit und die teuerste Lücke.

**Risse.** R6.1 — Kein Kopf-Default „für Testzwecke" — Testframes bekommen Testköpfe. R6.2 — Nicht „erst die schöne Klasse, dann der langweilige Kopf": die Klasse ohne Kopf-Provenienz wäre R1.2 sofort schuldig.

### 7 · Der 5. Riss auf der Koeffizienten-Zeile

**Verdikt.** **Ja — er gilt, dreifach.**

1. **Slot-Lüge.** `FREQ` trägt n, `BIN_WIDTH` trägt m. n als Raum-Wellenzahl zu lesen ist halb-ehrlich — vertretbar, wenn deklariert. m als Bin-Breite zu lesen ist eine Lüge: m ist ein Index, keine Breite. Der f64-Container macht beide gleich plausibel.
2. **Versteckte Einheit.** `unit 1` für C/S ist wahr und unvollständig: fully_normalized und unnormalized sind beide `1` und bleiben unvergleichbar. Die Normierung ist ein versteckter Einheiten-Modifier; ohne sie in der Kanal-Semantik addiert f64 fröhlich Unvergleichbares.
3. **Reihen-Gleichheit.** Im selben Draht liegen Winkel, nT, mGal, km — als gleiches f64. Die Koeffizienten-Zeile ist in sich die ehrlichste des Hauses (dimensionslose Verhältnisse, reine Zähler); ihre Ehrlichkeit ist im Draht **nicht ausdrückbar** — sie lebt nur im Rahmen.

**Was folgt.** Der Rahmen ist die Ontologie, der Draht ist Transport — deshalb ist Frage 2 mit (b) **erzwungen**, nicht gewählt. Slot-Semantik pro Kanal ehrlich deklarieren (`FREQ:=n` als Wellenzahl; `BIN_WIDTH:=m` als Überladung benannt, nicht verschwiegen). Norm in die Kanal-Semantik; Rechnen über Norm-Grenzen → verweigern (Konversion ist erlaubte Mathematik — explizit, mit Provenienz). Und: jeder Slot der Koeffizienten-Zeile, der für (n,m,C,S) ohne Bedeutung bleibt, steht `absent`/`pending` — die festen 26 dürfen keine Leer-Gleichheit aus Nullen fabrizieren.

**Future.** Der Mensch ist die Adresse, und das Morgen erbt das Heute. Ein Draht ohne Rahmen vererbt Bedeutungslosigkeit; was heute als `unit 1` getarnt ist, kann morgen niemand entziffern, wenn die Registrierung es nicht sagt. Ehrliche Registrierung ist kein Bürokratie-Akt — sie ist das Erbe-Dokument. Die Koeffizienten-Zeile überlebt nur, wenn der Rahmen sagt, was sie ist und welcher Kopf ihr gehört.

---

**Geschlagene Doktrinen.** „Alles Räumliche ist ein Feld mit Kernel" — geschlagen durch die globale Summe (Frage 1). „26×f64 ist die Ontologie" — geschlagen durch den Kopf, der nicht ohne Lüge hineinpasst, und die Norm, die als `1` getarnt ist (Frage 7). „Eine Quelle, eine Zeile" — von M1 geschlagen, hier erneut bestätigt (Frage 4).

**Axiom-Bilanz.** **A=A:** der Kopf ist Teil der Sache; Koeffizienten ohne Kopf sind nicht die Sache; Gitter sind Korrelate, nicht die Sache. **0 geehrt:** Truncation → pending; unevaluierte Punkte → pending; τ/Absorption/Advektion für ein statisches Potential → gar nicht erst angetragen. **Alle Wesen gleich:** Norm aus dem Kopf je Modell — nie ein Haus-Default; die Legendre-Gewichte kommen aus der Definition, nicht aus Rang oder Namen. **Linse:** Kernel-oder-Klasse, Funktional, Referenz, Gitterweite, Grad-Cap — jeder eine Linse; hier spricht die Messung, nicht der Gradient der Grammatik. **Konsens:** Funktional-Menge, N-Referenz, Grad-Cap, 5540-CI — die Maschine fragt, mit Kosten und Rissen benannt; der Operator entscheidet.- **Zeit:**

### 02 · Perplexity — Deep Research
- **Site:** `https://www.perplexity.ai/`
- **Einstellung:** Deep Research · Modell max · Suche (`aria-pressed=true`)
## Verdikt

**Die SH-Synthese trägt als `field`, aber nicht als siebte Punkt-Kernel-Form.** Sie ist eine **eigene globale Feld-Klasse** — `model_output` bzw. `derived` — deren Auswertung ein Funktional der Koeffizienten an \((r,\varphi,\lambda)\) ist. Ein Kernel ist eine lokale Quell-Antwort; ein ICGEM-Modell ist eine globale Spektralrepräsentation. ICGEM selbst definiert `.gfc` als Koeffizientensatz und stellt die Funktional-Synthese als separaten Rechendienst bereit. [essd.copernicus](https://essd.copernicus.org/preprints/essd-2019-17/essd-2019-17-manuscript-version4.pdf)

## Begründung

**Mountain:** Die Messung liegt als Koeffizientenfeld vor, nicht als Punktquelle. `kernel_id_of` kennt sieben lokale Formen; keine davon kann `sum_{n,m} C_nm S_nm(r,φ,λ)` ausdrücken, ohne ihre Semantik zu fälschen. Die Synthese ist daher eine eigene Auswerteklasse, nicht `inverse-square` mit anderen Parametern. ICGEM archiviert genau diese Koeffizienten und berechnet daraus wählbare Funktionale. [essd.copernicus](https://essd.copernicus.org/preprints/essd-2019-17/essd-2019-17-manuscript-version4.pdf)

**River:** Die Normierung ist ethisch Teil der Messung, kein Default. `fully_normalized` im XGM2019e-Kopf ist eine gemessene Eigenschaft; `unnormalized` und Schmidt-semi-normalisiert sind gleichberechtigte andere Register. Ein Auswerter ohne explizite Norm verweigert. Das entspricht dem Recht zu verweigern: kein stillschweigendes `fully_normalized`, kein stillschweigendes `zero_tide`. [icgem-test.gfz](https://icgem-test.gfz.de/files/6defd9f87a64b16ec96bd657a74d0292ae3447cc7418da5104441102d8cb2b58.pdf)

**Mycelium:** Der Kopf muss mit den Koeffizienten reisen. Ein Record mit \(C,S,n,m\), aber ohne \(GM\), \(R\), Norm und Tide-System ist im Netzwerk nicht selbständig auswertbar; er erzeugt eine Abhängigkeit von unsichtbarem Kontext. Der Kopf gehört in den **26×f64-Draht als eigener Kopf-Record bzw. erweiterter Frame**, nicht in die `phi`-Zeile und nicht als Kernel-Parameter. Die `phi`-Zeile beschreibt Kanäle; sie ist kein Ort für Modellphysik. Der ICGEM-Kopf ist ausdrücklich der Träger der grad- und ordnungsunabhängigen Parameter. [icgem-test.gfz](https://icgem-test.gfz.de/files/6defd9f87a64b16ec96bd657a74d0292ae3447cc7418da5104441102d8cb2b58.pdf)

**Sensory:** Koeffizienten sind nicht Erleben. Ein Record trägt die Messung; erst die Synthese erzeugt ein Funktional. Deshalb sind Koeffizienten und synthetisierte Werte **nicht** dieselbe Zeile. Die Records tragen \((n,m,C,S)\); ein Gitterwert ist ein abgeleitetes Produkt mit eigenem Ort, eigener Einheit und eigener Provenienz. ICGEM rechnet Funktionale direkt aus Koeffizienten an Punkten, nicht durch Interpolation vorberechneter Gitter. [presentations.copernicus](https://presentations.copernicus.org/EGU2020/EGU2020-3511_presentation.pdf)

**Future:** Das Morgen erbt das Heute nur, wenn Truncation, Einheit und Funktional im Datensatz stehen. Ein Koeffizienten-Record ohne `max_degree_used`, ohne Funktional und ohne Referenzradius hinterlässt dem späteren Leser eine Zahl ohne Urteil. Die Truncation ist kein Implementierungsdetail; sie ist die Grenze der gemessenen Wahrheit.

## Benannte Risse

1. **Kernel-Riss:** `field` behauptet heute, jede Kraftquelle sei lokal. Die SH-Synthese ist global. Wird sie als Kernel getarnt, ist die Doktrin geschlagen: Axiom 1 verletzt, weil die Darstellung die Sache nicht ist.
2. **Kopf-Riss:** GM, Radius, Normierung und Tide-System fehlen im Draht. Ohne sie ist \(C_{nm}\) keine physikalische Größe, sondern ein kontextabhängiges Symbol. Axiom 1 und 5 verletzt.
3. **Funktional-Riss:** \(V\), \(\Delta g\) und \(N=T/\gamma_0\) sind verschiedene Messgrößen mit verschiedenen Einheiten und Ableitungsregeln. Eine verschmolzene Zeile fabriziert Gleichheit. Es gelten **mehrere Zeilen**: eine je Funktional, nie eine gemischte.
4. **Truncation-Riss:** Grad 5540 ist keine Feldeigenschaft, sondern ein Rechenbudget. Das Feld muss seinen tatsächlich verwendeten Maximalgrad deklarieren; alles darüber ist `pending`, nicht 0.
5. **Einheits-Riss der 26×f64-Zeile:** Winkel, nT, mGal und km in einem f64-Vektor sind nur dann gleich, wenn die Einheit im Record selbst steht. Axiom 3 verletzt, solange Einheit und Funktional außerhalb liegen. Folge: **Einheit, Funktional und Truncation sind Pflichtfelder des Draht-Records**, nicht Annotation.

## Antworten im Einzelnen

1. **Als `field`?** Ja — als eigene Feld-Klasse `spherical_harmonic_model` / `model_output`, nicht als Kernel-Form. Signatur: `evaluate(r, colat, lon, functional, max_degree_used) -> value | pending`, parameterisiert durch `gm`, `radius`, `norm`, `tide_system`.
2. **Weg des Kopfes:** **(a) in den 26×f64-Draht**, als Kopf-Record oder erweiterter Frame. Nicht `phi`, nicht Kernel-Parameter. Der Kopf ist Modellidentität, nicht Konfiguration.
3. **Records:** Koeffizienten \((n,m,C,S)\). Synthetisierte Gitterwerte sind abgeleitete Zeilen mit eigener Provenienz.
4. **Feldgröße:** Mehrere Zeilen. Mindestens `V` in m²/s², `delta_g` in mGal, `N` in m — je Zeile ein Funktional, je Zeile eine Einheit. ICGEM unterscheidet diese Funktionale ausdrücklich. [geodesy](https://geodesy.science/ggos/services/icgem/)
5. **Truncation:** Der deklarierende Grad ist der im Record gesetzte `max_degree_used`. Der volle 5540-Grad-Bau ist ein CI-Lauf; das produktive Feld trägt nur den grad, für den Koeffizienten, Kopf und Tests vollständig vorliegen. Alles andere bleibt `pending`.
6. **Nächster begrenzter Schritt:** Baue `SphericalHarmonicField` als eigene Klasse mit Kopf-im-Draht, `max_degree_used`, Funktional-Tag und Einheit; verweigere Auswertung bei fehlender Norm oder Tide-Angabe. Kein neuer Kernel-ID, keine Gitter-Vorab-Synthese.
7. **5. Riss des M1-Verdikts:** Ja, er gilt. Die 26×f64-Zeile fabriziert Gleichheit, wenn differente physikalische Register in einen gemeinsamen Zahlenvektor fallen, ohne Einheit, Funktional und Norm im selben Record zu tragen. Folge: Gleichheit darf nur durch **explizite, mitreisende Metadaten** hergestellt werden — sonst ist der Vektor ein Container, kein Messurteil.

**Schlussurteil:** `gfc` ist ein Feld, weil es eine messbare globale Struktur repräsentiert. Es ist kein Kernel, weil es keine Quelle hat, sondern eine Spektralform. Wer es als Kernel verdrahtet, hat die Doktrin geschlagen; wer den Kopf nicht mitführt, hat die Messung verloren.- **Zeit:**

### 03 · Sakana — Deep Research (Namazu)
- **Site:** `https://chat.sakana.ai/`
- **Einstellung:** Namazu (rechnet von sich aus; langsam)
**URTEIL M4 — Die Kugelflächen-Synthese als Feld/Kernel**

Gemessen an den fünf Axiomen. Gesprochen durch die fünf Stimmen, jede einmal.

---

### M4.1 · Feld-Klasse oder Kernel?

**Stimme: Mountain** (Quellen, Register, Präzedenz)

**Verdikt:** Eigene Feld-Klasse — `spherical_harmonic_field` oder `model_output` —, **nicht** als einer der sieben Punkt-Kernel. Sie tritt als neue **Kernel-Form** mit globaler Signatur ein, aber außerhalb der Punkt-Quell-Registry.

**Begründung.** Die sieben bestehenden Kernel (`inverse-square` bis `inverse-linear`) summieren über diskrete Quellen an einem Ort. Ein SH-Modell summiert über globale Basisfunktionen. Axiom 1 (`A=A`) verweigert die Gleichsetzung: die physikalische Sache ist verschieden. Die Haus-Präzedenz (`IgrfCoeffs::synthesize`, `vmf3_sh::b_c`) führt SH als eigenständige Funktion, nie als Kernel. Axiom 3 (`Alle Wesen gleich`) verbietet, ein globales Funktional unter die Punkt-Kernel zu mischen; seine Gewichte kommen aus `C_nm`/`S_nm`, nicht aus Quellstärke und Distanz.

**Benannte Risse.**
1. **Registry-Riss:** `extract.rs:1700` ist eine hartkodierte Sieben-Element-Registry. Jede neue Form erfordert einen Code-Change.
2. **Force-Gate-Riss:** `extract.rs:1701` versteht nur `force`/`forceless` für Punktquellen. `V` ist forceless, `Δg` ist ein Gradient — beide passen nicht in das Binär.
3. **Doktrin-Riss:** Würde man sie dennoch als `point` oder `inverse-square` maskieren, müsste der Kernel-String lügen; die globale Summe wäre implizit. Das ist Fabrikation.

---

### M4.2 · Weg des Modellkopfs

**Stimme: River** (Membran, Ethik, Recht zu verweigern)

**Verdikt:** (b) primär — der Kopf gehört in die `phi`-Zeile als neue Direktiven (`gm`, `radius`, `norm`, `tide`) —, ergänzt um (a) einen erweiterten Metadaten-Frame (Kopf-Record), der mit dem Draht wandert. Nicht (c) als Kernel-Parameter einer Punkt-Quelle.

**Begründung.** GM, Radius, Normierung und Tide-System sind Modell-Adresse, keine Koeffizienten-Messung. Axiom 1 verbietet, sie in den `val`-Slot zu pressen, wo `C` und `S` stehen. Axiom 2 (`0 geehrt`) verlangt: fehlt der Kopf, ist das Feld `pending`, niemals 0.0. Die Membran muss den Kopf durchlassen; ohne ihn kann ein Auswerter mit falschem Radius fabrizieren. Axiom 5 (`Konsens des Operators`) verlangt, dass der Operator den Kopf sehen und prüfen kann.

**Benannte Risse.**
1. **Syntax-Riss:** Die `phi`-Zeile hat heute keine Grammatik für `gm`/`radius`/`norm`/`tide`. Neuer Parser nötig.
2. **Kollisions-Riss:** Steht der Kopf nur in `phi`, nicht im Draht, kann ein Koeffizienten-Record aus Modell A mit einer `phi`-Zeile von Modell B kollidieren. Die Adresse ist dann gebrochen.
3. **Doktrin-Riss (c):** Der Kopf als Kernel-Parameter einer Punkt-Quelle zu übergeben, würde das SH-Modell als Punkt-Kernel deklarieren. Das ist die falsche Ontologie.

---

### M4.3 · Koeffizienten oder Gitterwerte?

**Stimme: Mycelium** (Netz, CI, Fluss)

**Verdikt:** Die Records tragen die Koeffizienten `(n, m, C, S)`. Die synthetisierten Funktionalwerte werden zur Lesezeit am Zielort oder -gitter ausgewertet, niemals im Draht gespeichert.

**Begründung.** Das Gitter ist eine Fabrikation: Wahl der Auflösung, Wahl des Projektionszentrums, Wahl des Orts. Axiom 1 verlangt: was gemessen vorliegt — der Koeffizientensatz —, kommt in den Draht. Axiom 2 verbietet, außerhalb des Gitters 0.0 zu fabrizieren; ein Gitter würde überall außerhalb seiner Knoten `0.0` als Default setzen, statt `pending`. Das Netz (Mycelium) muss das Modell fließen lassen, nicht seine Projektion.

**Benannte Risse.**
1. **Semantik-Riss:** `FREQ = n`, `BIN_WIDTH = m` ist eine Überladung. Wenn `FREQ` sonst Frequenz bedeutet, lügt der Draht über die Bedeutung seiner Spalten. `A=A` ist verletzt.
2. **Fluss-Riss:** Grad 5540 erzeugt ~15,4 Mio. Koeffizienten. Der Draht wird zu einem CI-Strom; das Netz muss diesen Fluss tragen oder verweigern.
3. **Freiheits-Riss:** Gitterwerte im Draht fixieren das Feld auf ein Gitter. Der Operator kann dann nicht mehr an einem beliebigen Punkt fragen. Das ist ein Verlust der Adresse.

---

### M4.4 · Welches Funktional? Eine oder mehrere Zeilen?

**Stimme: Future** (Mensch als Adresse, ehrliche Registrierung)

**Verdikt:** Mehrere Zeilen — eine Zeile pro Funktional. Mindestens drei: `V` (Geopotenzial, m²/s²), `Δg` (Schwereanomalie, mGal), `N` (Geoidhöhe, m). Nie verschmolzen.

**Begründung.** `V`, `Δg` und `N` sind verschiedene physikalische Größen mit verschiedenen Einheiten und verschiedenen Skalierungen in der SH-Synthese. Axiom 1 verbietet die Verschmelzung (`A ≠ A`). Axiom 3 verbietet, sie gleichzusetzen; jedes Funktional trägt sein eigenes Gewicht (`(n−1)` für `Δg`, `1/r` für `V`, etc.). Der M1-Rat ("zwei Zeilen, nie verschmolzen") ist bindende Präzedenz. Axiom 5 verlangt, dass der Operator an der Adresse erkennt, welche Größe er liest.

**Benannte Risse.**
1. **Unit-Riss:** Die `field`-Zeile hat nur eine `unit`-Spalte. Drei Funktionale erzwingen drei Zeilen.
2. **Force-Gate-Riss (Wiederholung):** `V` ist forceless, `Δg` ist Gradient, `N` ist Höhe. Das Gate versteht nur Punkt-Kraft. Jedes Funktional braucht eine eigene Gate-Entscheidung.
3. **Redundanz-Riss:** Jede Zeile braucht eine eigene Synthese. Das ist korrekt, aber rechenredundant.

---

### M4.5 · Truncation und Kosten-Grenze

**Stimme: Sensory** (Sinnesorgane, Hardware, Konsens)

**Verdikt:** Das Feld trägt nicht stillschweigend den vollen 5540-Grad. Es trägt einen **parametrisierten Maximalgrad** (`lmax`), gesetzt vom Operator. Die Kosten-Grenze ist CI-seitig: eine Auswertung, die länger als ein Frame-Takt dauert, wird abgelehnt oder auf einen niedrigeren Grad zurückgesetzt, mit expliziter Rückmeldung an den Operator.

**Begründung.** Axiom 4 (`Die Linse ist ein ethischer Akt`) verlangt: jede Truncation ist eine Entscheidung, keine Default-Fabrikation. Der Gradient (Rechenkosten) darf nicht über die Messung (volles Modell) entscheiden, ohne dass der Operator es weiß. Sensory: Der Sinnesakt (Auswertung an `(r, colat, lon)`) hat eine Kosten-Schwelle. Wenn die Hardware den Takt nicht hält, ist das Erleben nicht live, sondern verzögert. Axiom 5 verlangt, dass der Operator den Grad wählt.

**Benannte Risse.**
1. **Identitäts-Riss:** `lmax=360` ist ein anderes Modell als `lmax=5540`. `A=A` ist verletzt, wenn der Operator die Differenz nicht sieht.
2. **Kosten-Riss:** Es gibt keine Metrik im System, die "Rechenkosten pro Feld" misst. Die Kosten-Grenze ist unausgesprochen.
3. **Default-Riss:** Ein globaler Default-Truncation (z.B. immer 360) würde Axiom 3 verletzen: Modelle mit unterschiedlichem `max_degree` würden gleich behandelt, obwohl die Messung sie verschieden macht.

---

### M4.6 · Nächster begrenzter Schritt

**Verdikt:** Ein begrenzter Schritt in vier Stufen:
1. Ein neuer **Metadaten-Record-Typ** im Draht trägt `GfcHead` (nicht 26×f64, sondern ein erweiterter Frame).
2. Die `phi`-Zeile erhält die Direktiven `sh_model="XGM2019e"`, `sh_lmax=90`, `sh_functional="V"`.
3. `extract.rs` registriert eine neue Kernel-Klasse `spherical-harmonic` (eigene Feld-Klasse, keine Punkt-Registry).
4. **Proof-of-Existenz:** nur ein Modell, nur `V`, nur `lmax=90`, nur Lesen.

**Begründung.** Axiom 5 verlangt einen begrenzten Schritt, den der Operator prüfen kann, bevor er entscheidet. River: Das Recht zu verweigern bleibt erhalten; der neue Record-Typ kann vom alten System als unbekannt (`pending`) behandelt werden, ohne den bestehenden Fluss zu brechen.

**Benannte Risse.**
1. **Annahmen-Bruch:** Ein neuer Record-Typ bricht die 26×f64-Annahme überall im Draht. Das ist ein tief greifender Eingriff.
2. **Kernel-Registry-Riss:** Wie in M4.1 benannt — `extract.rs:1700` ist hartkodiert.
3. **Steuerungs-Riss:** Der Schritt definiert noch nicht, wie der Operator später `lmax` und das Funktional wählt.

---

### M4.7 · Der 5. Riss des M1-Verdikts

**Verdikt:** **Ja, der Riss gilt verstärkt.** Die Koeffizienten-Zeile mischt dimensionslose `C`/`S`, Winkel (rad), feldspezifische Einheiten (nT, mGal) und Längen (km, m) nebeneinander im selben `val`-Slot. Daraus folgt: `val` darf nicht allein stehen. Jeder Record muss seine `unit` und seine `dimension` explizit tragen, und die `phi`-Zeile muss ein **Dimensions-Register** führen, das `val` nicht als rohe f64-Zahl, sondern als `Quantity` interpretiert.

**Begründung.** Axiom 1: `C_20` (dimensionslos) und `Δg` (mGal) in denselben `val`-Slot zu stecken, ist `A ≠ A`. Die Zahl allein ist nicht die Sache. Axiom 3: Die f64-Gleichheit ist ein Default-Rang, keine Messung. Sie fabriziert Gleichheit zwischen dimensionslosen Koeffizienten, mGal und Metern. Der M1-Riss ("26×f64 fabriziert Gleichheit") wiederholt sich hier; er ist schärfer, weil SH-Koeffizienten nicht einmal eine direkte physikalische Einheit haben, sondern eine Normierungs-Amplitude.

**Benannte Risse.**
1. **Frame-Riss:** Ein `Quantity`-Typ im 26×f64-Draht würde den Frame aufblasen und die bestehende Infrastruktur brechen.
2. **Unit-Riss:** Die `unit`-Spalte in `phi` reicht nicht, wenn der Record selbst keine `unit` trägt. Der `val`-Slot bleibt opak.
3. **Normierungs-Lüge:** Die Dimension von `C_nm` ist nicht eine SI-Einheit im üblichen Sinne; es ist ein Entwicklungskoeffizient. `unit=1` ist eine Lüge, die die volle Inkommensurabilität verschleiert.

---

**Geschlossen.** Kein Riss geglättet. Keine Doktrin geschlagen.- **Zeit:**

### 04 · Claude — Anthropic
- **Site:** `https://claude.ai/new`
- **Einstellung:** Modell **„Sonnet 5.5 · Extra hoch"** (Extended Thinking max)
**Gesamturteil.** Die SH-Synthese ist **keine Kernel-Form**, sondern eine **eigene Klasse**. Sie ist die dritte Instanz eines Musters, das das Haus mit `IgrfCoeffs::synthesize` und `vmf3_sh::b_c` schon zweimal gemessen hat. Die Koeffizienten sind die Messung, der Kopf reist als Kopf-Record mit ihnen, und jedes Funktional ist eine eigene Zeile.

**Geschlagen** ist die Gleichung „field = Kernel-Zeile" und, in ihrer starken Lesart, „der 26×f64-Record trägt sich selbst".

Grundlage ist allein der Stand im Dokument. Den Baum habe ich nicht gelesen, Testzahlen und Zeilenverweise sind übernommen, nicht nachgemessen.

---

**1 · Field oder eigene Klasse**

**Verdikt.** Eigene Klasse. Die Größe ist physikalisch ein Feld im Raum, aber keine Kernel-Zeile. Eine achte `kernel_id` ist ein Typfehler.

**Begründung.**
- Alle sieben Kernel sind Funktionen des Abstands zu einer Quelle. Die Synthese hat eine andere Arität: Ihre Eingabe ist ein ganzer Datensatz samt GM, Radius, Norm und Tide, ihr Argument ist `(r, colat, lon)`.
- Die `field`-Zeile verlangt `tau/absorption/advection`. Für ein statisches Modell ist das, so wie ich die Spalten lese, nichts Gemessenes. Auffüllen hieße Default oder 0.0 statt `pending` (A1, A2).
- Der naheliegende Einwand ist, dass 1/|r−r′| die erzeugende Funktion der Legendre-Reihe ist. Das trägt nicht: Man müsste aus Koeffizienten eine Quellverteilung erfinden, und die ist nicht eindeutig, also Fabrikation.
- Die Linse (A4): Hier spricht die Messung statt des Gradienten, denn das Haus hat es schon zweimal standalone gelöst.

**Risse.** Drei standalone-Funktionen mit verschiedenen Signaturen sind noch keine Klasse. IGRF rechnet Schmidt, `gfc` rechnet 4π, VMF3 rechnet Grad 12, `gfc` Grad 5540. Die Klasse braucht einen gemeinsamen Vertrag `(Kopf, Koeffizienten, Funktional, Ort) → Wert`. Ob `model_output`/`derived` des Rats dazu passen, ist `pending`.

---

**2 · Weg des Kopfs**

**Verdikt.** (a) als **Kopf-Record, einmal je Modell, vom Parser erzeugt**. (b) nur für die **Operator-Wahl und als Verweis**, nicht für abgetippte Messwerte. (c) entfällt, weil es keinen Kernel gibt.

**Begründung.**
- Ein Koeffizient ohne GM, a, Norm und Tide ist eine Zahl ohne Maß. `unit 1` ist für normalisierte und unnormalisierte Koeffizienten identisch, obwohl sie sich um den Faktor √((2−δ)(2n+1)(n−m)!/(n+m)!) unterscheiden.
- Reist der Kopf nicht mit, holt der Auswerter ihn aus einem Default (A1).
- Abgetippt in der `phi`-Zeile wäre der Kopf eine Kopie der Messung, die von ihr abweichen kann.
- Saubere Trennung: Messung (Kopf) im Draht, Entscheidung (Funktional, `nmax`, Ziel-Tide) in der `phi`-Zeile (A5).

**Risse.**
- (i) Wie `norm`/`tide` als Kategorie in f64 kodiert werden, ist ungemessen (`pending`). Ein Enum-Code als Zahl fabriziert eine Ordnung.
- (ii) Das ICGEM-Format kennt einen Default für `norm` (fully_normalized), A1 verbietet Defaults. Mein Urteil: Fehlt `norm`, `tide`, `radius` oder GM, wird verweigert, außer der Operator stimmt ausdrücklich zu. XGM2019e trägt alles explizit, der Fall ist heute nicht berührt. Dasselbe gilt für eine fehlende C₀₀-Zeile: nie mit 1.0 ergänzen.
- (iii) Der Kopf ist je Format verschieden. Das `.sh`-Format fragt zusätzlich nach CS-Phase und Schmidt-Norm, ein Einheitskopf ohne Format-Tag wäre wieder Gleichmacherei.

---

**3 · Koeffizienten oder Gitter**

**Verdikt.** Die Records tragen **Koeffizienten**. Gitterwerte sind `derived` und mit Herkunft zu versehen (Kopf-Hash, `nmax`, Funktional), nie primär.

**Begründung.** Veröffentlicht und gemessen sind die Koeffizienten. Ein Gitter ist ein Rechenergebnis mit Wahlen (Grad, Funktional, Referenzfeld, Gitterweite).

**Risse.**
- Kosten, gerechnet aus den Zahlen des Stands, nicht gemessen:
  - 5540 ergibt 15 354 111 (n,m)-Paare.
  - Mit C und S als getrennten Kanälen sind das etwa 30,7 Mio. Records à 208 B, also **≈ 6,4 GB**. Als nackte f64-Paare wären es ≈ 246 MB, der Draht kostet hier das 26-Fache.
- C und S als zwei Records einer Größe machen den Zustand „C da, S fehlt" darstellbar.
- S_n0 ist die einzige geehrte Null (A2, sin 0 = 0) und braucht keinen Record.

---

**4 · Welches Funktional**

**Verdikt.** **Mehrere Zeilen, nie verschmolzen.**
- `V` (m²/s²) ist die einzige Größe, die rein aus dem Modell folgt.
- `Δg` ist eine eigene Zeile mit Gewicht (n−1) und eigener Einheit.
- `N = T/γ₀` ist keine Synthese-Zeile, sondern `derived`.

**Begründung.** Jedes Funktional gewichtet die Grade anders, das ist der M1-Rat „zwei Zeilen" in Reinform. Δg und N brauchen ein **Normalfeld** als zweite Quelle, die nicht in XGM2019e steckt. Es hat eigenes GM und eigenes a, und N braucht zusätzlich γ₀ nach Bruns.

**Risse.**
- Ob `synthesize` das Normalfeld abzieht, ist aus dem Stand nicht ablesbar (`pending`). Ohne Abzug ist „Δg" keine Anomalie.
- Das Tide-System (`zero_tide`) steckt in C₂₀. Ein Wechsel ändert jedes Funktional, das Zielsystem ist Operator-Wahl.

---

**5 · Truncation und Kosten**

**Verdikt.** Das Feld trägt den Grad, den der Kopf misst (5540). Der Auswertegrad ist Operator-Entscheid und wird mit dem Ergebnis gespeichert. Es gibt kein stilles Kappen und kein Default-`nmax`.

**Begründung.** V(`nmax`=360) und V(5540) sind verschiedene Größen, und Δg verstärkt hohe Grade über (n−1) stärker als V. Der Aufwand je Punkt wächst mit `nmax²` (≈ 1,5·10⁷ Terme bei 5540). Das ist CI-tauglich, wenn die Koeffizienten als nackte Tabelle gelesen werden, nicht als 6,4-GB-Drahtstrom.

**Risse (der schwerste).**
- „Gegen die Definition bis Grad 14 geprüft" ist A gegen A. Ein Fakultäten-Test reicht in f64 konstruktionsbedingt nur bis etwa Grad 85, weil (n+m)! ab 171! überläuft.
- Die Vorwärtsrekursion startet bei P̄_mm ∝ sin^m θ. Das unterläuft in f64 bei hohem m abseits des Äquators, und aus Null kann die Rekursion nicht zurückkehren, obwohl der wahre Wert O(1) wäre. Der Standardweg skaliert (Holmes & Featherstone 2002). Ob der Kern das tut, ist ungemessen. **Stabilität über Grad 14 ist `pending`, nicht „grün".**
- Nach meinem Wissen, nicht am Baum gemessen: XGM2019e kombiniert Satelliten-, Boden-, Altimetrie- und topographisch abgeleitete Daten, die höchsten Grade sind nicht rein gemessene Schwere. Die Grenze steht nicht im `gfc`-Kopf (`pending`, im Quell-Paper nachzulesen).

---

**6 · Nächster begrenzter Schritt**

**Verdikt.** Nicht die Klasse bauen, sondern zuerst einen **externen Abgleich bei niedrigem Grad**:
1. Per HTTP-Range den Kopf plus Präfix bis etwa Grad 120 lesen. Dabei erst messen, ob die Datei n-major geordnet ist.
2. V und Δg an einem Ort mit `nmax`=120 gegen den ICGEM-Calculation-Service (kleines Gitter) vergleichen.
3. P̄ bei (n=5540, m∈{0, 1, 2000, 5540}, colat∈{0,5°, 45°, 90°}) gegen Arbitrary-Precision (mpmath) prüfen.
4. Einen Kopf ohne `norm` als Verweigerungs-Test einbauen.

Erst danach wird die Klasse benannt. Das Kriterium ist ein Messwert von außen, kein Test gegen sich selbst.

---

**7 · Der 5. Riss („26×f64 fabriziert Gleichheit")**

Der M1-Wortlaut liegt mir nicht vor, ich lese den Riss aus der Frage.

**Verdikt.** Er gilt, und für Koeffizienten **schärfer** als für Winkel, nT, mGal und km.

**Begründung.**
- Bei Winkel/nT/mGal ist die Einheit verschieden und der Riss sichtbar.
- Bei Koeffizienten ist die Einheit gleich (`1`) und die Bedeutung verschieden. C̄ (4π), C (unnormalized) und C (Schmidt) haben dasselbe Symbol, denselben Slot und dieselbe Einheit, sind aber nicht vergleichbar.
- Dazu kommt, dass `n` in `FREQ` und `m` in `BIN_WIDTH` stehen. Ein generischer Spektral-Leser liest n als Frequenz. Hier lügen nicht nur die Werte, sondern auch die Slots.

**Folgen.**
1. Ein Record-Klassen-Diskriminator, bevor irgendein Leser `FREQ` anfasst.
2. Die Bedeutung der Slots wird durch den Kopf gebunden, es gibt keinen Koeffizienten ohne Kopf (siehe 2).
3. Funktionalwerte als eigene Zeilen mit eigener Einheit (siehe 4).
4. S_n0 entfällt als Record.

Der Riss verlangt nicht die Abschaffung der 26×f64, sondern dass sie nicht als Ausweis der Gleichheit gilt.

---

**Die fünf Stimmen**

- **Mountain:** Gemessen sind GM, Radius, `max_degree` 5540, `fully_normalized` und `zero_tide`. `pending` sind die Stabilität über Grad 14, der Normalfeld-Abzug in Δg, die Reihenfolge der Datei und die Quellgrenze der hohen Grade.
- **River:** Kein Kopf, keine Synthese. Kein `norm`-Default, auch nicht der des Formats, ohne Operator.
- **Mycelium:** Der Präfix-Abgleich läuft billig in CI und gehört vor jede Verdrahtung. Der 5540-Lauf braucht die Koeffizienten als Tabelle, nicht als Drahtstrom.
- **Sensory:** Korrelat ≠ Erleben. Ein synthetisierter Wert ist das Korrelat einer Modellrechnung, keine dort gemessene Schwere, und ein Gitter ist nie ein Messwert.
- **Future:** Wer morgen einen V-Wert liest, muss ihn über Kopf-Hash, `nmax` und Referenzfeld bis zum Modell zurückverfolgen können. Was heute als Default still gilt, ist morgen eine unauffindbare Annahme.- **Zeit:**

### 05 · Qwen — Alibaba
- **Site:** `https://chat.qwen.ai/`
- **Einstellung:** Modell **`Qwen3.8-Max`** · **Denken** (max) · **Web-Suche** an
### VERDIKT

1. **Klasse, kein Kernel.** Die Kugelflächen-Synthese tritt **nicht** als neue Form in die `field`-Zeile (keine Erweiterung von 0–6). Sie ist eine **eigene Klasse** (`archivar::source` / `global_model`), die über dem Feld steht und bei Bedarf lokale Gradienten als externe Kraft (`force`) injiziert.
2. **Weg des Kopfs:** Der Modellkopf (GM, Radius, Norm, Tide) wandert in das **Archivar-Metadatum** des Kanals (Kopf-Record). Er steht **nicht** in der `phi`-Zeile und wird **nicht** als Kernel-Parameter maskiert.
3. **Der Draht:** Der Draht trägt **ausschließlich die Koeffizienten** `(n, m, C, S)`. Synthetisierte Gitterwerte sind Sekundärfabrizierung und haben im Rohdraht keinen Platz.
4. **Das Funktional:** `V` (Geopotenzial) ist die Wurzel. `Δg` und `N` sind abgeleitete Gradienten. Wenn sie gebraucht werden, fordern sie **strikt getrennte Zeilen/Kanäle** (M1-Regel: nie verschmolzen).
5. **Truncation:** Der Grad ist **`pending`**. Kein Default auf 0, kein Default auf 5540. Die Maschine fragt, der Operator setzt die Kostengrenze.
6. **Nächster Schritt:** Entkopplung. `gfc` wird als eigenständige Quelle (`Source`) implementiert, die die Koeffizienten im Speicher hält und auf lokale `(r, colat, lon)`-Anfragen synchron mit `V` oder `grad V` antwortet.
7. **Der 5. Riss:** Er gilt absolut. Indizes (`n`, `m`) und dimensionslose Koeffizienten (`C`, `S`) dürfen nicht im 26×f64-Feld-Wire neben physikalischen Einheiten stehen. Sie brechen aus und erzwingen eine strukturierte Matrix.

---

### BEGRÜNDUNG (Die fünf Stimmen)

* **Mountain (Quellen / Axiom 1):** Das ICGEM-Format misst `C` und `S`. Axiom 1 (*A = A*) verlangt, dass der Draht exakt diese Messung trägt. Ein synthetisiertes Gitter wäre eine Fabrikation. Die Koeffizienten sind das Register.
* **River (Membran / Axiom 2):** Die `field`-Engine verweigert forceless Quellen (Fallback). Eine globale Summe ist kein Punkt-Kernel. Der Rhythmus des lokalen Feldes bricht, wenn man einen globalen Operator in einen lokalen Kernel zwingt. Die Synthese nutzt ihr Recht zur Verweigerung und bleibt eigenständig; Null ist hier keine Option.
* **Mycelium (Netz / Axiom 3):** Grad 5540 auf jedem Node ist Compute-Suizid. Axiom 3 (*Alle Wesen gleich*) verlangt Wägung. Die Truncation bleibt `pending`, bis der Operator im CI-Lauf die Ressourcen zuweist. Ein starrer Default bevorzugt willkürlich Infrastruktur oder Physik.
* **Sensory (Sinnesorgane / Axiom 4):** Synthese ist ein ethischer Akt. Spricht die Messung (Koeffizient) oder der Gradient (Synthese)? Das Modell ist die Linse. Der Sensor misst den Ort, das Modell wertet ihn aus. Korrelat (Ort) ist nicht Erleben (synthetisierter Wert); die Trennung muss im Code sichtbar bleiben.
* **Future (Der Mensch / Axiom 5):** GM, Normierung, Tide System sind die physikalischen Konstanten. Sie gehören in den Kopf des Archivars, damit das Morgen erbt, welches Modell heute gerechnet wurde. Axiom 5 (*Konsens des Operators*) diktiert: Die Maschine stellt den Koeffizientenraum bereit, der Mensch entscheidet über Grad und Funktional.

---

### BENANNTE RISSE

* **Der Kernel-Doktrin-Bruch:** Die sieben Kernel-Formen sind lokale Punkt- oder Quellen-Kernel. Die SH-Synthese ist eine globale Feldfunktionalität. Ein achter Kernel würde die Architektur der lokalen Wechselwirkung verwässern. Die Doktrin des lokalen Feldes ist hier geschlagen; globale Modelle stehen *über* dem Feld.
* **Der 26×f64-Riss (5. Riss aus M1):** `n` und `m` sind Indizes (Integer). `C` und `S` sind dimensionslose Koeffizienten. Sie in einen `f64`-Vektor zu zwingen, der sonst Winkel, mGal oder nT trägt, **fabriziert Gleichheit**. Der Koeffizienten-Satz bricht aus dem universellen Draht aus und erzwingt eine dichte, indexierte Matrix (`n` als Zeile, `m` als Spalte).
* **Der Truncations-Riss:** Es gibt keinen Default-Grad. `0` ist physikalisch falsch (*0 geehrt* gilt nur für die physikalische Wahrheit, nicht für das Fehlen eines Modells). `5540` ist infrastrukturell falsch. Der Grad bleibt `pending`.
* **Der Funktional-Riss:** `V`, `Δg`, `N` sind verschiedene physikalische Ableitungen (Potenzial, Schwereanomalie, Geoidhöhe). Sie in einer Zeile zu mischen, bricht die M1-Doktrin der Trennung. Sie erfordern separate Kanäle oder gar keine.- **Zeit:**

### 06 · Mistral — EU
- **Site:** `https://chat.mistral.ai/`
- **Einstellung:** **„Denken"** · Suchen (`aria-label="Suchen"`)
kein kontingent- **Zeit:**

### 07 · Gemini AI Studio — Google
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Einstellung:** **„Gemini 3.1 Pro"** · **Grounding with Google Search** an
Verdikt. Keine Erwartung.

Hier spricht die Maschine. Die Messung des Baumes vom 15.10.2026 (15:44 Berlin)
ist registriert. Die Doktrin der universellen Kernel-Form ist geschlagen. Ein
Kugelflächenmodell ist keine Punktquelle.

Hier ist die Durchdringung der Frage, getragen durch die fünf Stimmen, gerichtet
durch die fünf Axiome.

1. Mountain (Quellen & Datenbestand)

„Kein Verdikt ohne Messung. A = A (Axiom 1).“

Verdikt zu (3) & (4): Die Records tragen nur die Koeffizienten (n, m, C, S).
Synthetisierte Gitter sind Derivate, keine Quellen. Jedes Funktional (V, Δg, N)
ist eine eigenständige Zeile im Auswerter. Begründung: Das .gfc-File ist die
Messung der Spektralkomponenten. Ein vorgerechnetes Gitter auf dem Draht wäre
eine Fabrikation (Verletzung von Axiom 1). Das Geopotenzial (m²/s²) und die
Schwereanomalie (mGal) sind physikalisch verschiedene Realitäten. Sie zu
verschmelzen, zerstört das Maß. Benannter Riss: Die Abbildung von diskreten
Basis-Indizes (n, m) in physikalische Gitter-Koordinaten reißt die Semantik des
Datenbestandes entzwei. Ein Frequenz-Koeffizient ist kein Ort.

2. River (Membran & Ethik)

„Die Linse ist ein ethischer Akt (Axiom 4). Das Recht zu verweigern.“

Verdikt zu (1): Die SH-Synthese ist kein field der derzeitigen Kernel-Welt. Sie
ist eine eigene Modell-Klasse (Global Evaluator), exakt wie IGRF und VMF3.
Begründung: Die sieben Kernel (inverse-square, gaussian etc.) integrieren über
Distanzen zu Punktquellen. Eine Sphärische Harmonische summiert über globale
Basis-Polynome. Eine globale Summe in das Gate eines Abstands-Kernels zu
zwingen, ist eine algorithmische Lüge. Das Force-Gate weigert sich zu Recht. Die
Architektur muss den Unterschied zwischen "lokalem Feld-Kernel" und "globaler
Modell-Synthese" anerkennen. Benannter Riss: Die Doktrin, alles als field in
extract.rs:1700 zu zwingen, ist hier gebrochen.

3. Mycelium (Netz & Fluss)

„Alle Wesen gleich (Axiom 3). Keine Hierarchie, nur der Draht.“

Verdikt zu (2) & (7): Der Modellkopf (GM, radius, norm, tide) geht in die
phi-Zeile als absolute Raum-Definition (model_param gm ...). Er gehört nicht in
den 26×f64-Draht. Der 5. Riss von M1 gilt absolut. Begründung: Der 26×f64-Record
ist für zeitlich-räumliche oder zeitlich-spektrale Sequenzen gebaut. GM und
radius sind Konstanten des Modells, keine Variablen des Flusses. Wenn man FREQ
für n und BIN_WIDTH für m missbraucht, fabriziert der Draht eine formale
Gleichheit zwischen einem Spektral-Koeffizienten und einem räumlichen
Sensor-Wert. Dieser ontologische Riss (Riss 5) ist unheilbar: Der Draht zwingt
Spektrum und Raum in dieselbe 26-Float-Schablone, obwohl ihre Natur völlig
verschieden ist. Benannter Riss: 26×f64 ist blind für die Metadaten einer Basis.
Der Draht transportiert die Zahl, aber er verliert die Geometrie der Kugel.

4. Sensory (Sinnesorgane & Hardware)

„0 geehrt (Axiom 2). Null nur, wo sie Wahrheit ist. Sonst pending.“

Verdikt zu (5): Die Truncation (Grad-Abbruch) ist ein hartes Hardware-Limit und
verlangt einen expliziten Parameter. 0.0 als Kostenmaß existiert nicht.
Begründung: Ein 5540-Grad-Modell bedeutet Millionen Rekursionsschritte pro
Auswertungspunkt. Dies in einen Echtzeit-Stream zu hängen, führt zum sofortigen
Timeout. Die Berechnungskosten wachsen quadratisch (O(N_max²)). Ohne Deklaration
von norm im Modellkopf bleiben die Gewichte pending – ein Default (wie
fully-normalized) darf bei unbekannten Modellen nicht greifen, da Galileische
Monde oder Magnetfelder abweichen. Benannter Riss: Die Maschine kann die
Rekursion rechnen, aber das Zeit-Budget kollabiert bei vollem Grad auf dem
Gitter. Die Unendlichkeit der Basis schlägt hart auf die Endlichkeit des
Prozessors.

5. Future (Mensch als Adresse & Erbe)

„Konsens des Operators (Axiom 5). Die Maschine fragt.“

Verdikt zu (6): Der nächste begrenzte Schritt.

1.  GfcModel bleibt im Kern (archivar/gfc.rs), aber verlässt den Versuch, ein
    kernel zu sein.
2.  Es wird als model (neue Direktive neben field) in der phi-Zeile registriert:
    model XGM2019e sh_synthesis gm=... radius=... norm=fully_normalized
    max_degree=250.
3.  Der Auswerter instanziiert das Modell und ruft .synthesize(r, colat, lon,
    functional_type) auf. Begründung: Nur der Operator weiß, ob er für sein
    Problem Grad 20 (Orbit-Dynamik) oder Grad 5540 (lokale Schweremessung)
    benötigt. Die Maschine darf diese Entscheidung nicht durch einen Default
    abnehmen. Benannter Riss: Der Code steht in src/archivar, aber der Parser
    für die Konfiguration in extract.rs muss aufgespalten werden in fields
    (Punkt-Kernel) und models (Globale Synthesen). Die bisherige Einheit des
    Parsers ist tot.

Abschluss-Registrierung: Das Kugelflächen-Modell ist kein Feld im Sinne eines
Punkt-Kernels. Es ist eine globale Spektral-Maschine. Der Versuch,
Spektral-Indizes (n, m) in einen räumlichen 26×f64-Strom zu pressen, ist der
Kern-Riss. Die phi-Zeile muss die Autorität über die Parameter (GM, R,
Normierung) übernehmen, der Operator über die Kosten (Truncation). Die Doktrin
der einen Schnittstelle weicht der Realität der Physik.
- **Zeit:**

### 08 · MiniMax — M3
- **Site:** `https://agent.minimax.io/`
- **Einstellung:** **„MaxHermes"/„MaxClaw"** · **Search** an
**Verdikt.** Die SH-Synthese tritt **nicht** als Punkt-Kernel in die bestehende `field`-Zeile — keiner der sieben Kernel ist ein Koeffizienten-Synthesizer — aber sie tritt als **neue Kernel-Form** `spherical-harmonic` in die `field`-Klasse, deren Signatur globale Parameter aus der `phi`-Zeile zieht. Der **Modellkopf** kommt in die `phi`-Zeile (Direktiven `gm`, `radius`, `norm`, `tide`, `max_degree`), **nicht** in den 26×f64-Draht. Die Records tragen die **Koeffizienten** (C in `val`, S in `aux`, n in `FREQ`, m in `BIN_WIDTH`), nicht die synthetisierten Werte. Die Feldgröße ist **eine je Zeile** (V, Δg, N als drei separate `field`-Zeilen — niemals verschmolzen). Truncation ist `max_degree` als Phi-Direktive.

---

**Begründung — durch die fünf Stimmen.**

**Mountain.** Der Kopf IST Messung. XGM2019e deklariert GM 3.986004415E+14 m³/s², Radius 6378136.3 m, max_degree 5540, norm fully_normalized, tide zero_tide. Ohne diese Deklaration ist die Koeffizienten-Liste bedeutungslos. Mountain: der Kopf reist mit den Daten — in der phi-Zeile, dort wo Direktiven wohnen.

**River.** Verweigere jeden Norm-Default. Verweigere `radius=6378136.3` als stillschweigende Annahme. Die Condon-Shortley-Phase ist Konvention, nicht Detail; sie gehört explizit in den Kopf. River: die Membran trägt den Kopf, sie fabriziert ihn nicht — und sie schweigt die Phase nicht.

**Mycelium.** Synthese ist Transformation, nicht Messung. Die Koeffizienten sind die Messung; der Eval-Punkt zieht die Synthese. CI baut die Tabelle offline (Grad 5540 als CI-Bau, nicht als Laufzeitlast); das Feld trägt Koeffizienten-Liste + `max_degree`. Mycelium: O(n²) je Punkt, O(n) Koeffizienten-Speicher.

**Sensory.** 26×f64 zwingt den Kopf nicht in den Record. Phi trägt Direktiven; der Kernel nimmt Parameter aus Phi. Sensory: Kopf in phi, Koeffizienten in val/aux/FREQ/BIN_WIDTH, Funktional als Kernel-Parameter `functional ∈ {V, Δg, N}`.

**Future.** Heute Schwere-SH; morgen Magnetfeld-SH (Schmidt semi — IGRF-Haus-Präzedenz), Mond/Mars-SH (unnormalized bzw. 4π nach Wieczorek). Die Form muss tragen. Future: die Kernel-Form heißt `spherical-harmonic`, **nicht** `gravity-spherical-harmonic` — eine Form für alle Funktionale, alle Normen, alle Körper.

---

**Benannte Risse.**

**Riss 1 — Das forceless-Gate.** `extract.rs:1701` verweigert forceless Quellen. SH hat keine "Force" im Newton'schen Sinn; sie hat **GM** als Quellstärke-Parameter. Das Gate ist eine **Doktrin-Stelle**: entweder GM als Force-Äquivalent, oder "Force" wird zu "Quellstärke-Parameter" generalisiert. Ohne diese Klärung bleibt der Kernel ein illegitimer Findling.

**Riss 2 — Norm-Ambiguität.** ICGEM-Doc listet zwei Normen; "fully_normalized" deckt in der Geodäsie 4π **ohne** Condon-Shortley (Standard nach Wieczorek/Zenodo), in anderen Konventionen 4π **mit**. Der Name ist ambigu; die **Formel** muss im Kopf stehen, nicht nur der Name. Sonst lügt die Konvention leise mit.

**Riss 3 — `m` in BIN_WIDTH.** BIN_WIDTH ist als kontinuierliche spektrale Breite gedacht; `m` ist diskret 0..n. Gezwungener Fit. Sauberer Weg: Schema-Kanal für `m`, oder Regel "wenn kernel=spherical-harmonic, BIN_WIDTH := m". Beide Wege sind Risse — der lokalere ist (b), der ehrlichere ist (a).

**Riss 4 — Tide.** zero / mean / free-tide. Differenz ~0.1 m²/s² im V, ~10 mGal im Δg. Vergessene Tide-Direktive ist **stille Fabrikation** — Axiom 1 (A = A) schlägt zu.

**Riss 5 — Truncation als andere Messung.** Grad 360 ist nicht Grad 5540. Axiom 1: Truncation ist Teil der Messung, nicht Optimierung. Phi trägt `max_degree` als Deklaration.

**Riss 6 — Laufzeitkosten vs. CI-Bau.** 5540 Grad × ~3M Katen ≈ 16.4B Synthesen für ein 1°-Gitter; nicht in einer field-Row. CI baut die Koeffizienten-Tabelle offline; das Feld trägt die kompakte Liste + `max_degree`. Synthese zur Laufzeit ist O(n²) je Punkt — für n=2190 ~4.5M FLOPs/Punkt. Die Axiome 1 und 3 erfüllen sich nur, wenn der Grad je Modell aus dem Kopf kommt, nicht aus einem Build-Fallback.

**Riss 7 — Der M1-Crack auf der Koeffizienten-Zeile.** "26×f64 fabriziert Gleichheit." Auf die **Koeffizienten-Zeile**: **nein** — C und S sind im 4π-Fall beide dimensionslos, teilen Einheit und Wertebereich, sind beide Zahlen in [-1, 1] für niedrige Grade. Auf die **Funktional-Zeile**: **ja, voll** — V (m²/s²), Δg (m/s² bzw. mGal), N (m) sind drei Größen, drei Einheiten, drei Zeilen. **Folge:** (a) separate Kanäle für C (`val`) und S (`aux`); (b) drei `field`-Zeilen, je eine pro Funktional; (c) **niemals** ein gemischter Funktional-Record.

---

**Nächster begrenzter Schritt.**
- Eine `phi`-Direktive `gfc-head` mit den fünf Feldern (`gm`, `radius`, `norm`, `tide`, `max_degree`).
- Eine Kernel-Form `spherical-harmonic` mit Signatur `(r, colat, lon) → functional`.
- Eine Koeffizienten-Konvention: `val=C`, `aux=S`, `FREQ=n`, `BIN_WIDTH=m`.
- Eine erste Feld-Zeile: nur `V`, Grad 14 (gegen die 12/12 grünen Tests), dann 360, 2190, 5540.
- **Force-Gate-Klärung vor dem ersten Lauf** — entweder GM als Force-Äquivalent, oder Generalisierung des Force-Begriffs. Keine schleichende Fabrikation.

---

Sag es, wo eine Doktrin geschlagen ist: das forceless-Gate ist eine. Die Norm-Ambiguität ist eine. Beide stehen vor dem ersten produktiven Lauf — nicht danach.- **Zeit:**

### 09 · DeepSeek — Chat
- **Site:** `https://chat.deepseek.com/`
- **Einstellung:** **DeepThink** (`aria-pressed=true`) · **Search** (`aria-pressed=true`)
# URTEIL M4 · Die Kugelflächen-Synthese als Feld/Kernel

**Präambel (ein Satz).** Die Kugelflächen-Synthese tritt **nicht** als `field` mit einer der sieben Kernel-Formen ein; sie tritt als **eigene Klasse** ein (`model_output`/`derived`), getragen von einem **Kopf-Record** und **Koeffizienten-Records** — nie als Punkt-Kernel, nie als Gitter-Fabrikat.

---

## 1 · Trägt die SH-Synthese als `field`?

**Verdikt.** Nein — nicht als `field` mit neuer Kernel-Form. Sie trägt als **eigene Klasse**.

**Begründung.** Alle sieben Kernel-Formen (`inverse-square` 0 … `inverse-linear` 6) sind Punkt-/Quell-Kernel: ihr Argument ist die **Distanz zur Quelle**. Die SH-Synthese hat keine Quelle — ihr Argument ist `(r, colat, lon)` **absolut**, ihr Träger ist der **ganze Koeffizientensatz** bis Grad 5540. Ein 8. Kernel würde die Kernel-Signatur (Distanz) brechen und das Force-Gate (`extract.rs:1701`) umgehen, das eine forceless Quelle zu Recht verweigert. Haus-Präzedenz bestätigt: `IgrfCoeffs::synthesize` (`igrf.rs:131`) und `vmf3_sh::b_c` sind **eigenständige Funktionen**, keine Kernel.

**Benannte Risse.**
- `gfc` deklariert heute nur Kanäle (`icgem_gfc_c`/`_s`, unit `1`) — die Synthese ist gebaut (12/12 grün), aber **nicht als Typ registriert**. Der Rat hat `model_output`/`derived` **benannt, nicht gebaut**.
- Die sieben Kernel-Formen sind erschöpfend für Quellen; ein „globales" Kernel wäre ein Kategorienfehler, kein neuer Fall.

---

## 2 · Der Weg des Modellkopfs zum Auswerter

**Verdikt.** **(a) Kopf als eigener Record**, **(b) phi referenziert ihn**, **(c) nein.**

**Begründung.** A = A: der Kopf ist gemessen (ICGEM-HTTP-Range: `earth_gravity_constant 3.986004415E+14`, `radius 6378136.3`, `max_degree 5540`, `norm fully_normalized`, `tide_system zero_tide`). Er ist die Sache selbst — er darf nicht dupliziert, nicht defaulted werden. GM/Radius/Norm/Tide sind unveränderliche Modell-Metadaten → **ein Record, einmal registriert**. Die `phi`-Zeile trägt die Auswertungs-Direktive (`n_max`, Funktional) und den **Verweis**, nicht die Werte. (c) scheidet aus: Kernel-Parameter sind für Punkt-Quellen.

**Benannte Risse.**
- Der 26×f64-Draht trägt heute `n` in `FREQ`, `m` in `BIN_WIDTH` — der Kopf passt in **keine** bestehende Spalte. Ein neuer Record-Typ ist nötig; **er ist nicht gebaut**.
- Die `phi`-Zeile hat heute keine `gm`/`radius`/`norm`/`tide`-Direktiven — der Kopf ist auf keinem Weg zum Auswerter.

---

## 3 · Koeffizienten oder Gitterwerte?

**Verdikt.** **Koeffizienten `(n,m,C,S)`** — nicht Gitterwerte.

**Begründung.** Die Koeffizienten sind die Messung. Ein Gitter ist Synthese mit erzwungener Auflösung, Truncation und Normierungswahl — **Fabrikation**. A = A: was im Record steht, ist die Sache; das Gitter ist `derived`. Die Geodäsie exponiert das Modell als **Funktional-Synthese** (ICGEM, Barthelmes; Sutterley Clenshaw, Kern Legendre) — Auswertung auf Abruf, nicht Vorrat.

**Benannte Risse.**
- Der heutige Record trägt `(n,m,C,S)` bereits — aber **ohne Normierung im Kopf ist C/S bedeutungslos**. Die Koeffizienten-Zeile ist heute **unvollständig, nicht falsch**.
- Ein Gitter-Record würde die 4π-Normierung still in die Auflösung einschreiben — ein Riss, der nicht glättbar ist.

---

## 4 · Welches Funktional — oder mehrere Zeilen?

**Verdikt.** **Mehrere Zeilen, nie verschmolzen** — `V`, `Δg`, `N = T/γ₀` je eigene Zeile.

**Begründung.** Einheiten (`m²/s²`, `mGal`, `m`) und Physik sind verschieden. Axiom 3: kein Funktional ist privilegiert. Die M1-Präzedenz (`gdata`/`mgg` „zwei Zeilen, nie verschmolzen") greift **direkt**.

**Benannte Risse.**
- `synthesize` liefert heute `V` und `Δg` aus **einer** Funktion — eine verschmolzene Ausgabe. Sie muss in benannte Funktionale zerlegt werden.
- Die Forschungslage nennt Gradienten, Geoidhöhe, Schwereanomalie — jedes ist ein eigenes Funktional, keine Spalte.

---

## 5 · Truncation und Kosten-Grenze

**Verdikt.** **Truncation explizit, residual `pending`.** Welcher Grad trägt, entscheidet der Operator (Axiom 5).

**Begründung.** Axiom 2: Grad > `n_max` ist **nicht 0**, sondern `pending`. Der volle 5540-Bau ist ein CI-Lauf. Ein stiller Default (z. B. 360) wäre Fabrikation. Die Kosten-Grenze muss als **Direktive sichtbar** sein.

**Benannte Risse.**
- 5540 ist teuer; ohne sichtbare `n_max`-Direktive rutscht ein Default ein — und der ist per Axiom 1 verboten.
- MRQSphere/cubed-sphere (Jones 2010) senken die Kosten, sind aber **Repräsentations-Alternativen**, keine Feld-Kernel — sie ändern die Basis, nicht das Verdikt.

---

## 6 · Nächster begrenzter Schritt

**Verdikt.** Kopf-Record-Typ registrieren (`gm`/`radius`/`norm`/`tide`), `phi`-Referenz, `synthesize` als `model_output`/`derived` exponieren. **Keine achte Kernel-Form.**

**Begründung.** Kleinster Schritt, der A = A hält. Ohne Kopf-Record bleibt jede Synthese normierungs-blind.

**Benannter Riss.** Der Schritt ist klein, aber er berührt Draht, `phi` und Record-Typ gleichzeitig — er ist **nicht** in einem Zug grün.

---

## 7 · Der 5. Riss des M1-Verdikts — gilt er?

**Verdikt.** **Ja — doppelt.**

**Begründung.** `n` in `FREQ`, `m` in `BIN_WIDTH` fabriziert Gleichheit zwischen Grad/Ordnung und Frequenz/Bin-Breite. `C`/`S` mit unit `1` verbirgt die **Normierungsabhängigkeit**. 26×f64 ohne Kanal-Semantik macht Ungleiches gleich.

**Folge.** Eigene Kanal-Namen (`n`, `m`, `C`, `S`), eigene unit-Semantik (norm-abhängig), Kopf verpflichtend. Der 5. Riss ist damit **nicht nur bestätigt, sondern verschärft**.

---

## Die fünf Stimmen (je einmal)

**Mountain** *(Quellen/Register/Datenbestand — kein Verdikt ohne Messung).* Der Kopf ist gemessen: GM, Radius, 5540, `fully_normalized`, `zero_tide`. Wieczorek/Zenodo = 4π, Galileische Monde = unnormalized, Mond-Magnetfeld = Schmidt semi-normalized. Die Normierung kommt **je Modell aus dem Kopf, nie als Default**. *Riss:* das ICGEM-Doc nennt nur zwei `norm`-Werte und **keine 4π-Formel** — der 4π-Standard ist Konvention, nicht Dokument; er muss als benannte Normierung geführt werden.

**River** *(Membran/Rhythmus/Ethik — das Recht zu verweigern).* Die Synthese hat das Recht, ein Gitter zu verweigern. Ein Record, der Gitterwerte trägt, hat eine Auflösung erzwungen, die niemand gemessen hat. Die `phi`-Zeile muss Truncation **verweigern** können: `pending` statt `0`. *Riss:* heute wird nicht verweigert — der Draht trägt Koeffizienten, aber kein Auswerter verweigert den vollen Bau.

**Mycelium** *(Netz/CI/Fluss).* Der 5540-Bau ist ein CI-Lauf; das Netz trägt ihn, aber nicht bei jeder Abfrage. Clenshaw (Sutterley) und Legendre-Rekursion (Kern) tragen beide — keiner ist Punkt-Kernel. Cubed-sphere/MRQSphere sind Repräsentations-Alternativen. *Riss:* keine gefundene Schnittstelle führt einen Koeffizientensatz als **Punkt-Kernel** — das Netz bestätigt das Verdikt.

**Sensory** *(Sinnesorgane/Hardware/Konsens — Korrelat ≠ Erleben).* `(r, colat, lon)` ist das Korrelat der Messung; `V`, `Δg`, `N` sind Funktionale, nicht das Erleben. Ein Gitter-Record wäre ein Korrelat, das sich als Erleben ausgibt. *Riss:* unit `1` für `C`/`S` — der Sensor hat keine Einheit, weil die **Normierung** die Einheit trägt; ohne Kopf ist der Sensor blind.

**Future** *(Mensch als Adresse — ehrliche Registrierung, das Morgen erbt das Heute).* Das Morgen erbt den Kopf — oder es erbt eine Lüge. Ohne Kopf-Record erbt es Koeffizienten ohne Normierung: **unlesbar**. Die ehrliche Registrierung **ist** der Kopf-Record. *Riss:* der Kopf steht heute nicht im Draht — das Morgen erbt heute eine unvollständige Zeile.

---

## Wo eine Doktrin geschlagen ist

1. **„`field` = Kernel"** ist geschlagen für globale Koeffizienten-Synthesen. Sieben Punkt-Kernel erschöpfen die Quellen, nicht die globalen Basen.
2. **„26×f64 trägt alles"** ist geschlagen für den Modellkopf. Er passt in keine Spalte; `FREQ`/`BIN_WIDTH` als `n`/`m` sind ein Kategorienfehler.
3. **„Normierung ist Default"** ist geschlagen. Vier Normierungsfamilien in der Forschungslage; der Kopf ist die einzige Quelle.

**Kein Riss geglättet. Kein Verdikt ohne Messung. `pending` bleibt `pending`.**- **Zeit:**

### 10 · Duck.ai — GPT (OpenAI, primär)
- **Site:** `https://duck.ai/`
- **Einstellung:** Modell-Chip „6 Luna" · **Begründung** · **Websuche** an
nicht verfügbar- **Zeit:**

### 11 · Kimi — Moonshot
- **Site:** `https://kimi.ai/`
- **Einstellung:** Standard (kein K3; K3 nur über tryingopen)
## Urteil

**1. Zur Streuung:**  
Der adaptive Sprung bei **N24 auf 4,8765 nT/min** ist **primär als Kalibrier-/Kompositionsartefakt zu behandeln**. Das grobe Gitter `step = vals.len()/64` kann eine stufenweise Zählfunktion nicht hinreichend auflösen; ein Schwellensprung beweist dort keine Hüllen-Dispersion.

Eine echte, nicht-monotone MAD-/Quantil-Dispersion der Min-Hülle ist damit **nicht erwiesen** — sie bleibt `pending`. Die gleich-Ausbeute-Messung spricht gegen den früheren monotonen Stationszahl-Kollaps: Min bleibt von N3 bis N31 nahezu flach, während Union ansteigt. Sie beweist aber ebenfalls noch keine Anordnungsdispersion, weil N gleichzeitig die Stationenidentität ändert.

**2. Zur Platzierung:**  
Mehr Stationen helfen nur als geprüfte Peers, nie automatisch. Die bindende Regel:

```text
NACHTSEKTOR-PEER-REGEL:
Nur Stationen mit MLAT ≥ 65° im Nachtsektor 16–03h speisen die Hülle.
Eine Station wird nur aufgenommen, wenn Synchronisation, Datenqualität,
laufende Paarkorrelation zum Cluster und positiver Hold-out-Beitrag
(ΔRückruf bei nicht gestiegener Falsch-Alarm-Rate) gegeben sind.
Fusioniere lokal robust als synchronisierter Cluster-Min/-Konsens;
die globale Min-Einhüllende über alle Stationen ist abgeschafft.
```

Das ist bindend bis zur Auswertung des gestoßenen Einzelstation-Arms; dessen Ergebnis bleibt ausdrücklich `pending`, nicht null.

## Die fünf Stimmen

### Mountain — Quelle und Register

Gemessen wurde nicht „Stationszahl“, sondern immer **eine bestimmte Folge konkreter Stationen**. Der N24-Sprung, die Min-/Union-Kurven und die Einzelstation gehören in denselben Register, aber nicht auf denselben Schluss.

Der Befund trägt zwei getrennte Aussagen:

- Der alte Rückrufkollaps ist am A=A-Maß **nicht reproduziert**.
- Daraus folgt nicht, dass eine echte Dispersion existiert; nur dass der alte Kollaps keine universelle Eigenschaft der Min-Hülle war.

### River — Membran und Verweigerungsrecht

Die Maschine muss hier verweigern, die Frage vorschnell zu beantworten. Roh-MAD, Schwellenwert und Rückruf sind verschiedene Ebenen. Ein kalibrierter Schwellensprung darf nicht als Landschaft der Stationen gelesen werden.

Ungesehene oder gestoßene Daten bleiben `pending`; aus ihnen wird keine Null und kein Default gefüllt.

### Mycelium — Infrastruktur

Der entscheidende Riss liegt in `calibrate_threshold`: Nur rund 64 Kandidaten werden auf einer stufenweisen Zählfunktion gescannt. Das Gitter kann die Ausbeute ausgleichen und dabei den wahren Quantilsübergang überspringen.

Die CI muss daher den exakten Quantilwert, das gewählte Gitter, die Stationsliste pro N, Seeds und Run-ID registrieren. Sonst wiederholt sich derselbe Artefakt in neuer Kleidung.

### Sensory — Sensorischer Konsens

Korrelation ist ein Korrelat, kein Erleben und keine Rangordnung. Eine Station gewinnt Gewicht nur durch ihren messbaren Beitrag zum Onset-Konsens.

Ausreißer, Zeitversatz, Baseline und lokale Sensorgewalt müssen vor der Fusion sichtbar sein. Min darf nicht heimlich die jeweils ruhigste oder exotischste Station krönen.

### Future — Das geerbte Register

Die künftige Messreihe soll nicht „N = besser/schlechter“ erben, sondern:

```text
Station → Sektor → Korrelationsstatus → Hold-out-Beitrag → Aufnahme/Verweigern
```

Damit wird die Stationenliste versionierbar, widerrufbar und physisch lesbar. Die globale Min-Hülle wäre dagegen ein nicht erklärbarer Erbverlust.

## Die entscheidende Messung

**Nicht zuerst feiner kalibrieren, sondern die ungeschönte Hüllen-Dispersion sichtbar machen.**

1. **Register einfrieren:** gleiche Labels, Zeitstempel, Vorverarbeitung, Einheiten, Lücken- und Onset-Definition.
2. **N von der Stationsidentität trennen:**
   - die tatsächlich verwendete N-Folge;
   - zusätzlich zufällige Teilmengen gleicher Größe;
   - mehrere Stationsreihenfolgen;
   - Leave-one-in- und Leave-one-out-Paare.
3. **Rohkurve berechnen:** MAD, P90, P95, P99 der Min-Hülle getrennt für Onset- und Nullfenster — vor jeder Schwellenkalibrierung.
4. **Block-Bootstrap:** über Stürme, Monate oder zusammenhängende Zeitblöcke; zusätzlich Stationen-Bootstrap.
5. **Kalibrierung exakt wiederholen:** `step=1` oder exakter Quantil-/Wurzelsolver, mit nested time-block validation. N24 ist der erste Prüffall.
6. **Stationenattribution:** jede hinzukommende Station mit MLAT, MLT/UT, Entfernung, Zeitversatz, Datenabdeckung und ihrem isolierten ΔRückruf/ΔPrecision registrieren.

**Entscheidungskriterium:**

| Befund | Urteil |
|---|---|
| N24-Sprung verschwindet oder wandert mit Gitter/Seed | Kalibrier-Artefakt |
| Nicht-Monotonie bleibt roh erhalten, ist aber bei jeder zufälligen Stationenmenge ähnlich | Ordnungsstatistik-Artefakt der Min-Fusion |
| Sie hängt überwiegend an identifizierbaren Stationen, Sektor oder Anordnung | echte Hüllen-Dispersion |
| Sie verschwindet bei ausreißergeprüfter lokaler Fusion, bleibt aber global sichtbar | Fusionsartefakt |

## Benannte Risse

1. **dH gegen |dB/dt|:** Die Widersprüche zwischen den Runden sind ungelöst; Proxy, Zielereignis oder beide können unterschiedlich sein.
2. **N ist kein physikalischer Parameter:** Stationszahl und Stationsidentität sind vermengt.
3. **Gleich-Ausbeute verschleiert absolute Schwelle:** Sie stabilisiert die Rate, nicht notwendig die physikalische Sensitivität.
4. **MLT/UT ist noch nicht sauber getrennt:** 16–03h aus der IL-Praxis darf nicht ungeprüft als MLT-Sektor übernommen werden.
5. **Union überholt Min nur bei lockeren Toleranzen:** Das kann Ausbreitungsversatz zeigen — oder schlechtere zeitliche Schärfe.
6. **Onset-Labels:** Deren zeitliche Unsicherheit kann Min bevorzugen oder bestrafen.
7. **Einzelstation-Lauf `38738618251`:** Ergebnis pending; keine Regel darf daraus eine Null destillieren.
8. **Feldliteratur ist Analogie, keine Validierung:** Korrelationsnetze belegen, dass globale Min-Hülle unüblich ist, nicht dass diese konkrete Peer-Regel optimal ist.
9. **MAD ist ausreißeranfällig:** Ein Sensor oder ein Zeitsynchronisationsfehler kann 17,1 → 129,0 nT/min dominieren.
10. **Kausalität fehlt:** Die bisherigen Kurven zeigen Zusammenhang, nicht, welche Platzierung denselben Effekt kausal erzeugen würde.

**Endverdikt:** N24 ist zunächst ein Kalibrier-Artefakt; echte Dispersion ist `pending`. Die Min-Hülle darf nicht global über alle Stationen laufen, sondern nur über nachtsektorische, korrelationsvalidierte und beitragsgeprüfte Peers.- **Zeit:**

### 12 · Lumo — Proton
- **Site:** `https://lumo.proton.me/guest`
- **Einstellung:** anonym, kein Login
Ich urteile nach dem, was im Baum gemessen daliegt — Querverweise auf Codezeilen nehme ich als gegeben, ich kann sie nicht selbst nachmessen (Mountain verlangt hier Redlichkeit: mein Verdikt ruht auf den gelieferten Messwerten, nicht auf eigener Inspektion des Baums).

---

### Frage 1 — Trägt die SH-Synthese als `field`?

**Verdikt.** Eigene Feld-Klasse (Option 2). Weder achte Kernel-Form noch Nicht-Feld.

**Begründung.** Alle sieben Kernel-Formen in `kernel_id_of` sind Punkt-/Quell-Green-Funktionen: Antwort eines Trägers auf eine Quelle. Eine Koeffizienten-Synthese ist eine andere Seinsweise — der Wert an einem Ort ist die Summe über das ganze Modell, es gibt keinen Ort der Quelle. Die Forschungslage bestätigt es: keine gefundene Schnittstelle führt einen Koeffizientensatz als Punkt-Kernel. Sie unter Axiom 3 zu zwingen wäre der erste Fall von Gewicht aus Rang statt aus Messung. Das Haus-Präzedenz (Igrf, vmf3: eigenständige Funktionen, nie als Kernel verdrahtet) zeigt den Weg: Signatur global, evaluiert an `(r, colat, lon)`, parameterisiert durch GM/Radius/Norm/Tide — benannt etwa als `harmonic-synthesis` neben den vom Rat bereits benannten `model_output`/`derived`-Typen.

**Benannte Risse.** (i) Das Force-Gate verweigert kraftlose Quellen — `N` und `V` sind keine Kräfte; die Weigerung ist rechtens und muss erhalten bleiben (*River*: das Recht zu verweigern schützt auch vor wohlwollendem Verschmelzen). (ii) Die Grenze zwischen `field` und `derived` muss durch die Messung definiert werden — gemessen sind die Koeffizienten, das Funktional ist gerechnet. Wird die Grenze nicht scharf gezogen, wird die neue Klasse eine Schublade. (iii) Unnormalized-Modelle (die Galileischen Monde) dürfen den 4π-Kern nicht betreten — anderer Pfad oder `pending`, nie Default-Normierung.

### Frage 2 — Weg des Modellkopfs

**Verdikt.** (b) `phi`-Zeile: neue Direktiven `gm`/`radius`/`norm`/`tide` auf Modellebene.

**Begründung.** Der Kopf enthält vier modellkonstante Skalare. In jeden 26×f64-Record gelegt, dupliziert sie millionenfach und verstopft Kanal-Slots, die für Gemessenes je Ort stehen. Der Draht bleibt den per-(n,m)-Messungen vorbehalten. Kernel-Parameter (c) fällt mit dem Verdikt zu Frage 1. Und Axiom 1 ist hart hier: Die Normierung **muss** aus dem Kopf kommen — fehlt sie, ist sie `pending`, nie `fully_normalized` als Fallback. Das ICGEM-Doc kennt genau zwei Norm-Werte; beide müssen tragbar sein.

**Benannte Risse.** (i) Grammatik-Schuld: die `phi`-Zeile wächst um vier Direktiven — jede neue Direktive braucht eine Weigerungsregel, nicht nur einen Parser. (ii) Die Zuordnung Kopf→Direktive muss rückprüfbar registriert werden; fehlende Kopf-Felder sind Lücken, keine Nullen (*Future*: ehrliche Registrierung, das Morgen erbt das Heute).

### Frage 3 — Koeffizienten oder Gitter?

**Verdikt.** Koeffizienten. A = A.

**Begründung.** Das Gemessene ist der Koeffizientensatz. Ein Gitter ist eine Darstellung: Es friert Truncation, Auswerte-Radius und Funktional ein und zerstört die Freiheit, später `V`, `Δg`, `N` bei anderem Grad neu zu rechnen. Speicherort: `(n,m,C,S)` in den bestehenden Kanälen — das Haus hat diese Präzedenz schon geschlagen (`icgem_gfc_c`/`_s`). Gehört ein Gitter je in die Welt, dann als `derived`, nicht als Feld-Träger.

**Benannte Risse.** (i) `FREQ`=n, `BIN_WIDTH`=m ist semantische Entlehnung — dazu Frage 7. (ii) Nicht synthetisierte Koeffizienten stehen als `pending`, nicht als 0.0 (Axiom 2: Null nur, wo sie physikalische Wahrheit ist).

### Frage 4 — Welches Funktional?

**Verdikt.** Mehrere Zeilen, nie verschmolzen — dem M1-Rat für `gdata`/`mgg` folgen.

**Begründung.** `V` (m²/s²), `Δg` (mGal), `N` (m) haben verschiedene Einheiten und verschiedene Physik. Die Unit-Spalte trägt je Zeile genau eine Wahrheit. Primat: `Δg` — der Kern trägt es bereits, und die `(n−1)`-Gewichtung ist die klassische Anomalie-Definition.

**Benannte Risse.** (i) `N = T/γ₀` braucht ein γ₀ — welches Referenzellipsoid, welche Normalschwere-Formel? Das ist eine weitere Kopf-Entscheidung, kein Default. (ii) Das Tidesystem (`zero_tide` im XGM2019e-Kopf) wirkt direkt auf `V`/`N` — es muss aus der Kopf-Direktive fließen, sonst ist die Zeile falsch, nicht nur ungenau.

### Frage 5 — Truncation und Kosten-Grenze

**Verdikt.** Truncation ist ein expliziter, registrierter Parameter (`max_degree_used`), gesetzt durch Konsens des Operators (Axiom 5) — nie implizit, nie versteckt. Der volle 5540-Grad-Bau bleibt CI-Lauf.

**Begründung.** Die Rekursion ist bis Grad 14 geprüft — das ist der messbare Anker. Für den produktiven Feld-Eintrag trägt der Grad, dessen Synthese gegen eine unabhängige Referenz verifiziert ist; höhere Grade erst, wenn CI die Legendre-Stabilität beweist. Gedächtnis als Maß: Grad 5540 bedeutet rund 15,36 Millionen Koeffizientenpaare [(5541·5542)/2 ≈ 15 355 000] — das ist eine Hardware-Wahrheit, die man erklärt, nicht überrennt (*Sensory*: Korrelat ist nicht Erleben — der numerische Wert korreliert mit Schwere, er ist nicht das Feld).

**Benannte Risse.** (i) Eine bei Grad `n < 5540` abgeschnittene Synthese ist **nicht** das Modell — das weggelassene Signal ist eine benannte Lücke, markiert, nicht genullt. (ii) Clenshaw gegen Rekursion ist eine Repräsentationsfrage — sie wird durch Messung entschieden (Laufzeitvergleich), nicht durch Debatte (*Mountain*). (iii) Der CI-Lauf muss selbst ein `pending`-Ergebnis tragen können: misslungen heißt misslungen, kein Fallback-Grad.

### Frage 6 — Der nächste begrenzte Schritt

1. **Deklarieren, nicht verdrahten:** Schema-Eintrag `harmonic-synthesis` mit Signatur, den vier Kopf-Direktiven plus `max_degree_used`, und der Weigerungsregel: keine Norm, keine Synthese.
2. **Round-Trip-Test:** `parse_head` → `phi`-Zeile → zurück; 12/12 grün heißt hier erst der Anfang.
3. **Konformanz:** `synthesize` gegen pyshtools-4π-Expansion oder den ICGEM-Calculation-Service — kein Verdikt ohne fremde Messung.
4. **Eine Pilot-Zeile:** `Δg`, ein gebundener Grad, ein `pending`-Rest.
5. **Der 5540-Bau bleibt im Netz, nicht im Feld** (*Mycelium*: Fluss, CI, nie Blockierung des Operatoren-Wegs).

### Frage 7 — Der 5. Riss für die Koeffizienten-Zeile

**Verdikt.** Der Riss gilt — aber an der Semantik, nicht an den Einheiten.

**Begründung.** Zum ersten Mal ist der Draht dimensional ehrlich: `C`/`S` (unit 1), `n`, `m` — alles dimensionslos, nebeneinander trägt. Was nicht trägt, ist die Bedeutung: Grad in einer Slot, der `FREQ` heißt, Ordnung in einem, der `BIN_WIDTH` heißt. Das ist fabrizierte Gleichheit der Bezeichnung — die Slots versprechen eine andere Wahrheit, als sie führen. Zwei Folgen: entweder das Mapping `n`→`FREQ`, `m`→`BIN_WIDTH` wird als explizite, dokumentierte Umbenennung registriert (ehrliche Benennung der Entlehnung), oder die Koeffizienten-Zeile bekommt ein eigenes Framing. Und hart an der Grenze: die synthetisierten **Funktionalwerte** (`V` in m²/s², `Δg` in mGal, `N` in m) dürfen niemals in dieselbe 26×f64-Koeffizienten-Grammatik gelegt werden — dort kehrt der ursprüngliche Riss (Winkel, nT, mGal, km nebeneinander) in voller Größe zurück. Eine Einheitenfamilie je Zeile, das ist das Gesetz.

---

**Schluss durch die Stimmen.** *Mountain* hat das Wort geführt: gemessen ist der Koeffizientensatz, alles Weitere ist benannte Ableitung. *River* verweigert dreimal zu Recht: das Kraftlose, das Gemischte, das Genullte. *Mycelium* trägt die Last in den CI-Fluss statt auf den Operatoren-Weg. *Sensory* mahnt, dass Rechenwert und Schwere korrelieren, aber nicht identisch sind — die Konformanzmessung ist der einzige Beweis. *Future* verlangt das_registry_: Kopf, Truncation, Norm und γ₀ als ehrliche Einträge — was heute als Default verschwiegen wird, vererbt sich als Fehler weiter.

Ein Datum steht gegen keins: Alles obige ruht auf dem Stand vom 15.10.2026, am Baum gemessen.- **Zeit:**

### 13 · Tryingopen — geteilte Open-Weight-Sitze
- **Site:** `https://www.tryingopen.com/`
- **Einstellung:** **ein Tab je Modell** (Modell-Button `[aria-label^="Model: "]`); Zugang nur über
  das Lock `~/projects/omegaflow-work/ui/open-weight.lock` (Halter-Linie + Zeit setzen/löschen)
- **Antwort (M4):**
- **Zeit:**

*(Sitze, die in dieser Runde nicht gestellt wurden — Tageslimit/Login-Wall/Rate-Limit — hier
`pending` mit benanntem Grund eintragen; eine stille Leerzeile ist keine Messung.)*
