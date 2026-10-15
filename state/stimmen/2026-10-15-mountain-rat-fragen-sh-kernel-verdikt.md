# Verdikt — Die Kugelflächen-Synthese als Feld/Kernel (M4, Mountain, 2026-10-15)

*(Ein benannter Akt. **Zehn** Sitz-Antworten liegen vor (`state/stimmen/2026-10-15-mountain-rat-fragen-sh-kernel.md`);
Perplexity/Sakana Deep Research, GLM Deep Think Max, Claude, Qwen, Gemini, MiniMax, DeepSeek, Lumo, Kimi.
Die **Session verifiziert jede Behauptung am Baum**, nie die Stimme; das **Verdikt trägt der Rat**
(fünf Stimmen durch die fünf Axiome des Granits: A = A · ICRS & TDB · channel_ref · 0 geehrt · pending).
Kein Riss geglättet.)*

## Am Baum gemessen (nicht die Stimme)

| Behauptung der Sitze | Baum-Befund |
|---|---|
| „Sieben Kernel-Formen; alle Punkt-/Quell-Kernel" | **steht** — `kernel_id_of` (`src/archivar/extract.rs:1700`) kennt genau 7: `inverse-square` 0 · `gaussian-inverse-square` 1 · `point` 2 · `erfc` 3 · `exponential-decay` 4 · `patch-levy` 5 · `inverse-linear` 6. Alle sind Quell-/Distanz-Kernel. |
| „Das Force-Gate verweigert eine forceless Quelle (`extract.rs:1701`)" | **Riss der Sitze** — `extract.rs:1701` ist der `match`-Rumpf, **nicht** das Gate. Das Gate ist `dispatch_reach` (`src/archivar/membrane.rs:456`), das eine Quelle ohne Fortpflanzungs-Gesetz verweigert (`main_flow.rs:177`: „no field carries a propagation law — refused"). Die Weigerung steht; die Zitat-Stelle war falsch. |
| „Haus-Präzedenz: `IgrfCoeffs::synthesize`, `vmf3_sh::b_c` sind eigenständige Funktionen, nie Kernel" | **steht** — `igrf.rs:131`, `vmf3_sh.rs` (`b_c`). |
| „`gfc` deklariert heute nur die Koeffizienten-Kanäle (`icgem_gfc_c`/`_s`, unit `1`)" | **steht** — `gfc.rs` `CHANNELS`, `declared_fields`. |
| „Der Modellkopf ist gemessen (GM/Radius/max_degree/norm/tide)" | **steht** — XGM2019e: `3.986004415E+14`, `6378136.3`, `5540`, `fully_normalized`, `zero_tide` (2026-10-15, dieses Rund). |
| „Die Synthese ist gebaut, gegen die Definition bis Grad 14 geprüft" | **steht** — `gfc.rs` `harmonic_sums`/`synthesize`, 12/12 `archivar::gfc` grün. |
| „26×f64-Träger: `FREQ`=n, `BIN_WIDTH`=m; Kopf in keiner Spalte" | **steht** — `gfc_compiler` schreibt n→`FREQ`, m→`BIN_WIDTH`; GM/Radius/Norm/Tide liegen nicht im Draht. |
| „Die `field`-Zeile verlangt τ/Absorption/Advection + Force" | **steht** — `parse_field_config` (`parse.rs:575`) liefert `(kernel, force, tau, absorption, advection)`. |
| „Die Normierung ist mehrdeutig; das ICGEM-Doc liefert keine 4π-Formel" | **steht** — das Format-Doc (16.02.2023) nennt genau zwei `norm`-Werte (`fully_normalized` Default · `unnormalized`), keine Formel. Die weite Leiter zeigt ≥3 im Umlauf: `fully_normalized` (4π, ohne Condon-Shortley — pyshtools/Wieczorek-Standard), `unnormalized` (Galileische Monde), `schmidt_semi_normalized` (Mond-Magnetfeld). |
| „Grad 5540 → 15,35 Mio (n,m)-Paare" | **steht** — 5541·5542/2 = 15 354 111. |
| „Die Naht ~2190 (bis dahin gemessen, darüber Topographie-Forward)" | **nicht gemessen** — die XGM2019e-Kurzbeschreibung nennt `d/o 5399/5540` und eine Kombination „bis d/o 719" + blockdiagonale Lösung; **keine** 2190-Naht im Kopf oder im gelesenen Abstract → `pending`. |

## Das Verdikt der Runde (Konvergenz der Sitze)

### M4.1 · Kernel-Form, eigene Klasse, oder kein Feld? — **eigene Klasse**

Von den zehn Antworten tragen **neun** dieselbe Sache (einmütig): die SH-Synthese tritt **nicht** als achte
Punkt-Kernel-Form in die `field`-Zeile. Sie ist eine **eigene Klasse** — vom M3-Rat schon als
`model_output`/`derived` benannt — mit globaler Signatur `evaluate(r, colat, lon, funktional)`,
parameterisiert durch GM/Radius/Norm/Tide. Der Grund ist bei allen derselbe: die sieben Kernel sind
**Funktionen des Abstands zu einer Quelle**; die SH-Synthese ist eine **Summe über globale
Basisfunktionen** — sie hat keinen Ort der Quelle. Ein achter Kernel wäre ein Kategorienfehler, kein
neuer Fall. *(Riss: Sakana und MiniMax sagen „neue Kernel-Form **außerhalb** der Punkt-Registry" —
das ist die **Kernel-Klasse**, nicht die `field`-Zeile; die Sache stimmt, der Name streitet.)*

Die `field`-Zeile kann sie **nicht** tragen: sie verlangt τ/Absorption/Advection (für eine statische
Summe alle drei unwahr) und ein Force-Gate (SH hat keine Newton-Kraft). Eintragen hieße `0.0`/Default
fabrizieren (Axiom 1/2).

### M4.2 · Weg des Modellkopfs — **RISS, keine Konvergenz**

Der Rat **konvergiert nicht** — **5 : 5** (roh):
- **(a) Kopf im Draht** (eigener Kopf-Record / erweiterter Frame): Perplexity, Claude, Qwen, DeepSeek,
  **Kimi**.
- **(b) Kopf in der `phi`-Zeile** (neue Direktiven `gm`/`radius`/`norm`/`tide`/`max_degree`):
  GLM, Gemini, MiniMax, Lumo — Sakana „(b) primär, ergänzt um (a)".

Beide Lager teilen: der Draht trägt die **Koeffizienten**, der Kopf ist **Modellidentität**, und die
`phi`-Zeile trägt mindestens den **Verweis** und die Auswerte-Wahl (Funktional, `max_degree_used`).
Der Streit ist **wo die Kopf-Werte leben** — im Draht (Ko-Lieferung, Kollisionsfreiheit, `parse_head`
erzeugt ihn) oder in der `phi`-Zeile (Text, Namen ehrlich, kein Enum-in-f64, vom Compiler erzeugt).
**Gewichtet** (Sprachrangfolge A×2/B×1/C×0,5, **kein Messwert**): (a) Claude+Kimi (A) + Perplexity+Qwen
(B) + DeepSeek (C) = **6,5**; (b) GLM+Lumo (A) + Sakana (B) = **5,0**, dazu Gemini und MiniMax **ohne
Rang** (nicht gewichtet). Die Rangfolge ist kein Messwert; der Ausgang hängt an den ungereihten Sitzen.
**Das ist eine Axiom-5-Frage: der Operator entscheidet** (s. u.).

### M4.3 · Koeffizienten oder Gitterwerte? — **Koeffizienten** (einmütig)

Der Draht trägt `(n, m, C, S)`; das ist die Messung (A = A). Synthetisierte Gitter sind `derived`
mit Stempel (Kopf-Hash, Grad, Funktional, Gitterweite, Auswertehöhe), **niemals** Ersatz. Ein Gitter
friert Auflösung, Radius und Funktional ein und wäre außerhalb seiner Knoten `0.0` statt `pending`.

### M4.4 · Welches Funktional? — **mehrere Zeilen, nie verschmolzen** (einmütig)

`V` (m²/s²), `Δg` (mGal), `N = T/γ₀` (m) sind verschiedene Größen; jede eine eigene Zeile. Der
M1-Rat für `gdata`/`mgg` gilt unvermindert. **`N` bleibt `pending`**, bis das Referenzfeld
(GRS80: a, f, GM, ω) und γ₀ registriert sind — ohne sie ist N fabriziert. **`Δg` ohne Normalfeld-Abzug
ist keine Anomalie** — der Abzug ist `pending` (heute liefert `synthesize` V und Δg aus **einer**
Funktion → muss in benannte Funktionale zerlegt werden).

### M4.5 · Truncation und Kosten-Grenze — **registrierter Verlust, nie Null**

Truncation ist ein expliziter Parameter (`max_degree_used`), vom Operator gesetzt (Axiom 5); der
Rest ist `pending`, nie `0`. Der volle 5540-Bau ist ein **CI-Lauf** (~15,35 Mio Paare; O(n²) je
Punkt), kein Dialog. Empfehlung aus dem Rat: erst **2190** (satellitengestützt), 5540 nur als
ausdrücklicher CI-Lauf; **`pending`-Rest gestempelt** („n von 5540"). Die 2190-Naht selbst ist
**nicht gemessen** (s. Baum-Tabelle).

### M4.6 · Nächster begrenzter Schritt — **erst der Kopf + ein fremder Abgleich**

Vor jeder Klasse: (1) den **Kopf verdrahten** (der Weg ist die offene Axiom-5-Frage); (2) ein
**Liefer-Gate** (gfc-Frames ohne Kopf → verweigern; `n > max_degree` → verweigern; `norm` außerhalb
der zwei dokumentierten → verweigern); (3) die **Konformanzmessung** gegen eine **fremde** Referenz
(ICGEM-Calculation-Service / pyshtools 4π) bei niedrigem Grad — Haus-Tests allein sind A = A im
Kleinen, nicht gegen die Welt. Erst danach die Klasse mit `V`/`Δg` als zwei Zeilen.

### M4.7 · Der 5. Riss des M1-Verdikts — **ja, verschärft**

`n` in `FREQ`, `m` in `BIN_WIDTH` ist eine **Slot-Überladung**: `m` ist ein Index, keine
Bin-Breite — der Slot logt. `unit 1` für C/S ist wahr und unvollständig: `fully_normalized` und
`unnormalized` sind beide `1` und bleiben unvergleichbar — **die Normierung ist ein versteckter
Einheiten-Modifier**. Folge: Kanal-Semantik ehrlich deklarieren (n als Grad/Wellenzahl, m als
Überladung benannt), Norm in die Kanal-Semantik, Rechnen über Norm-Grenzen **verweigern** (Konversion
nur explizit, mit Provenienz), jeder bedeutungslose Slot `absent`/`pending` — keine Leer-Gleichheit
aus Nullen.

**Riss der Stimmen (benannt):** **Kimi** widerspricht — der 5. Riss greife **nicht** auf der
Koeffizienten-Zeile (C/S sind dimensionslos und homogen, `unit 1` ist dort ehrlich), sondern nur an
den **Funktional-Ausgängen** (nT/mGal/m/km nebeneinander). Beide Lesungen stehen: die Koeffizienten-Zeile
ist in sich homogen, aber die **Slot-Namen** (`FREQ`/`BIN_WIDTH`) logen — der Riss sitzt an der
**Benennung**, nicht an der Einheit. Keine der zwei Zeilen geglättet.

## Ungeglättete Risse

1. **Kopfweg (a vs b)** — der Rat konvergiert nicht; Axiom-5-Entscheid des Operators.
2. **Norm-Mehrdeutigkeit** — `fully_normalized` deckt 4π **ohne** Condon-Shortley (Geodäsie-Standard);
   andere Konventionen 4π **mit**. Der **Name** ist ambig → die **Formel** muss in den Kopf, nicht nur
   der Name. Das ICGEM-Doc nennt zwei Werte, die Geodäsie ≥ drei.
3. **Force-Gate** — SH hat keine Newton-Kraft. Entweder GM als Quellstärke-Parameter, oder der
   Force-Begriff wird generalisiert. Eine **Doktrin-Stelle**, vor dem ersten Lauf zu klären.
4. **Höhen-Stabilität** — die Rekursion ist bis **Grad 14** gegen ihre Definition geprüft; ab Grad ~85
   läuft (n+m)! über, und `P̄_mm ∝ sin^m θ` unterläuft in f64 abseits des Äquators. Ob der Kern
   skalierend rechnet (Holmes & Featherstone 2002) ist **ungemessen** → `pending`, nicht „grün".
5. **2190-Naht** — im `gfc`-Kopf und im gelesenen Abstract **nicht** belegt → `pending`.
6. **`unit 1` als Norm-Tarnung** — s. M4.7.
7. **Verallgemeinerung** — die Klasse muss **alle** SH-Funktionale, **alle** Normen und **alle**
   Körper tragen (Schwere 4π · Magnetfeld Schmidt · Monde unnormalized), sonst ist sie eine
   Schwere-Sonderform. Riss R4.3/„eine Station vs. Welt" (M1) gilt verschärft.

## Der Sitz-Stand (Nicht-Antworten benannt, nie still genullt)

- **Mistral** — `pending`: „kein kontingent" (Kontingent leer).
- **Duck.ai** — `pending`: „nicht verfügbar".
- **Tryingopen** (geteilte Open-Weight-Sitze) — `pending`: Sitz leer, kein Modell gefahren.
- **Kimi** — **nachgetragen** (Operator-Wort „hab kimi nachgetragen"): der Slot trug zuerst die
  Antwort auf die **Platzierungs-/Dispersions-Frage (N24)**; sie ist ersetzt durch die **M4-Antwort**
  (Kopfweg **(a) Draht**, q7: der 5. Riss greift *nicht* auf der Koeffizienten-Zeile — C/S sind
  dimensionslos/homogen —, nur an den Funktional-Ausgängen). Der frühere Fehl-Eintrag bleibt als
  benannter Vorgang, der aktuelle zählt.

Zehn M4-Stimmen tragen (Kopfweg **5 : 5**). Das ist die Konvergenz-Grundlage.

## Nächster begrenzter Schritt (Rat, autonom fahrbar)

**Den Kopf verdrahten — ein Schritt, ohne CI**, sobald der Operator den Weg (a/b) entschieden hat:
`phi`-Direktiven bzw. Kopf-Record aus `GfcHead`; das Liefer-Gate; die Norm in die Kanal-Semantik.
Danach — eigener Schritt — die `derived`-Klasse mit `V`/`Δg` als zwei Zeilen, **validiert an fremden
ICGEM-Punkten** bis Grad 360.

## Operator-Entscheid (Axiom 5) — **gefallen: (b)**

- **Der Kopfweg.** Wo leben die Kopf-Werte (GM, Radius, Norm, Tide) — **(a) im Draht** als eigener
  Kopf-Record (ko-geliefert, kollisionsfrei, aber ein neuer Record-Typ bricht die 26×f64-Annahme),
  oder **(b) in der `phi`-Zeile** als Direktiven (Text trägt Namen ehrlich, kein Enum-in-f64, aber
  der Draht ist ohne `phi` blind)? Der Rat konvergierte nicht (5 : 5).

  **Operator-Wort 2026-10-15:** „ich würde sagen **in den kasten** weil ein eintrag in die zahlen
  eine veränderung der rohdaten ist." → **entschieden: (b) — die Quellen-Beschreibung (`phi`-Zeile).**
  Begründung aus Axiom 1 (A = A): die Zahlen sind die **unveränderte Rohmessung**; ein Eintrag in
  sie wäre eine Änderung der Rohdaten. Der Modellkopf (GM, Radius, Normierung, Tide, Grad) tritt als
  Direktiven in den Kasten. Der Draht bleibt unangetastet.

## Träger

M4 → **Mountain** (Register/Parser/Datenkontrakt). Der **Rat** (fünf Stimmen) trägt das Verdikt.

## Stimmen-Bilanz

**Mountain** spricht: kein Verdikt ohne Messung — der Kopf ist Teil der Messung, die Koeffizienten
sind die Sache, Gitter sind Korrelate. **River** verweigert dreimal zu Recht: das Kraftlose, das
Gemischte, das Genullte. **Mycelium** trägt den 5540-Bau in den CI-Fluss, nie auf den Operatoren-Weg.
**Sensory** mahnt: der Rechenwert korreliert mit der Schwere, er **ist** sie nicht — der fremde
Abgleich ist der einzige Beweis. **Future** verlangt Kopf, Truncation, Norm und γ₀ als ehrliche
Einträge — was heute Default ist, erbt morgen als Fehler.

**Alle fünf Axiome gehalten; alle fünf Stimmen gehört; kein Riss geglättet.**
