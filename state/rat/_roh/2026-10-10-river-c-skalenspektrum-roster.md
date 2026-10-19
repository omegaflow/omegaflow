# Max-Roster — C „Skalenspektrum je Paar" (River, 2026-10-10)

Frage wie im Rat (`state/stimmen/2026-10-10-river-c-skalenspektrum-rat.md`):
C = native per-Paar-Skala / Skalenspektrum TE; Verhältnis zu D (Summary-Graph/PCMCI).
Runde: `river-ui` (Duck · Claude · Qwen · Z.ai) + geteilt `open-weight-ui`
(Qwen3.8 2.4T; tryingopen, Lock `state/ui-open-weight.lock`).

## Claude — Sonnet 5.5 Extra hoch — GEANTWORTET
Verdikt: Die Richtung stimmt, aber drei Korrekturen + vier Änderungen.
- **„Kein Aliasing" zu stark.** Ein Boxcar-Mittel dämpft nur (Sinc-Seitenkeulen);
  bei Potenzgesetz-Spektren (Sonnenwind) ist der Rest nicht null.
- **`median_dt` als Float fragil.** Kalendermonate schwanken 28–31 Tage → τ auf
  eine Kadenzklasse mit Toleranz rasten, sonst gilt eine Monatsreihe als zwei
  Skalen und wird auf sich selbst „aggregiert".
- **Aggregation ändert die Größe.** Sobald die Kopplungsverzögerung < Binbreite
  ist, wird sie zur gleichzeitigen Kopplung (Breitung & Swanson 2002); TE mit
  Lag ≥ 1 sieht sie nicht mehr. Die Stufen sind verschiedene Fragen.
- Leiter am **Paar** ankern: `Δⱼ = 2ʲ · (native Länge des gröberen Arms)`, nicht
  global Power-of-2 ab 1 min.
- **Rolle pro Arm**, nicht pro Zeile (Zeilenstatus = absent, sobald ein Arm absent).
- **Vierter Zustand `Underpowered`** bei N < N_min — sonst liest sich „nicht
  signifikant bei N=60" wie „keine Kopplung".
- Vergleichbarkeit: z/p, q, effektive/normierte TE speichern; FDR-Familie je Stufe
  vorab festlegen. Kein Mittel über Skalen — richtig.
- **C↔D:** C getrennter Schritt, Grundlage von D. PCMCI braucht ein gemeinsames
  Gitter → läuft nur auf Stufen ≥ max τ oder skalenkompatiblen Teilmengen; Kanten
  brauchen ein Skalenlabel, keine Pfade über Stufen. C liefert D die Rungtabelle
  als Kandidatenfilter. Bivariate C-TE bleibt konfundiert (Sonnenzyklus).
- **Erster Bau-Schritt:** Reihenfolge stimmt; Signatur
  `scale_ladder(τ_d, τ_t, span, n_min) -> Vec<Rung{Δ, role_d, role_t, n}>`.
  Aggregation über Index-Blöcke des gröberen Arms; schneller Arm über dasselbe
  Zeitfenster gemittelt mit Mindestabdeckung; Zeitstempel = Intervall (nicht
  Punkt); kein RNG/FFT.

## Qwen3.8 2.4T (tryingopen) — GEANTWORTET
- (1) Weitgehend korrekt. Zusatz: (a) Boxcar = Low-Pass, TE je Δ ist ein
  *anderer* Prozess; (b) Kalendermonate → nie eine Leiter in absoluten Sekunden;
  (c) extensive Felder (Fluss/Zählung) → Block-**Summe** statt Mittel.
- (2) Ja, lange Tabelle (Paar, Δ) → eine Zeile je Stufe mit Rolle {native,
  aggregated, absent}, N_eff, τ, k, p_surrogate, q_FDR; kein Mittel über Skalen.
  FDR-Familie fixieren: **within-rung BH** (rungs stark positiv abhängig), BHY als
  Alternative; `absent`-Zeilen nur als Schema-Doku, nicht rechnen.
- (3) C getrennter Schritt, Zulieferer/Modell-Selektor für D: nutze C, um (i) das
  gröbste gemeinsame Rung fürs PCMCI-Gitter zu wählen, (ii) `tau_max` aus
  physikalischen Lags zu setzen, (iii) jede Kante mit charakteristischer Skala
  (feinstes FDR-signifikantes Rung) zu annotieren. D ist eine neue Rechnung
  (multivariate CI-Tests), keine Aggregation von C-Zeilen.
- (4) Reihenfolge richtig, reißt an zwei Stellen: Power-of-2-Sekunden bricht bei
  monatlich/unregelmäßig; statt dessen Paargitter = native Kadenz des gröberen Arms.

## Qwen (Qwen3.7-Plus, `river-ui`) — GEANTWORTET
- (2) Ja — nicht über Skalen mitteln, sonst löscht man die skalenabhängige Physik
  (kurzfristige Sonnenwind- vs. langfristige QBO-Kopplung). Metadaten (Herkunft, N,
  τ) zwingend behalten — N braucht man für die FDR-Gewichtung über 210 Paare × Skalen.
- (3) C ist Zulieferer, D läuft **pro Skala** (Multiscale-PCMCI). PCMCI kann nicht
  direkt auf ein „Spektrum" angewendet werden; Konditionierungsmengen (Parents)
  nur aus Variablen derselben Skalenstufe (oder explizit modellierten
  Cross-Scale-Verzögerungen). Start: strikt getrenntes PCMCI je Skala.
- (4) Das Datenmodell (ScaleRole + scale_ladder) ist perfekt; es reißt am
  **Embedding**: für jede Skalenstufe Δ müssen die Embedding-Parameter neu
  bestimmt werden (Time-Delay τ skaliert mit Δ).

## Z.ai (GLM-5.3, Deep Think Max) — GEANTWORTET (Nachlauf)
Beim ersten Lesen noch „Thinking…"; die Antwort stand vollständig im Thread
(`Transfer-Entropie-Skalierung in Rust`). Verdikt:
- Leiter **je Paar**, nur aufwärts; Vergleichbarkeit über absolute Δ-Sekunden in
  der Zeile, nicht über global erzwungene Zweierpotenzen. **Rung 0 = Linie B
  exakt** — B als Rung 0 falten, keinen zweiten Codepfad behalten.
- Zeilenschema reicht nicht mit (Herkunft, N, τ): braucht **N_eff** (Aggregation
  rötet die Reihe, N_eff fällt schneller als N), **Coverage** (Anteil nicht-leerer
  Blöcke), τ in **Stufen-Samples** (Embedding je Stufe neu), Herkunft **pro Arm**.
- **„Absent" zweideutig:** strukturell (zu fein, gar kein Test) vs. statistisch
  (N_eff unter Floor). Floor vorab, z. B. `N_eff ≥ 4·(k+l+2)`.
- Kein Mittel über Skalen — **und kein argmax**; falls eine Zahl je Paar nötig:
  max über Stufen mit **max-T-Null** (Surrogate ebenfalls über Stufen maximieren).
  FDR: 210×K minus vorab registrierte Exklusionen; BH robust, BY konservativ.
- **C↔D:** beides — getrennt als Analyse, Baustein als Bibliothek. PCMCI braucht
  ein Panel auf einem Gitter; per-Paar-native Skalen konsumiert es nicht. Erster
  Wurf: **ein D auf der C-begründeten Stufe**. (Runge/PCMCI `1702.07007`.)
- **Drei Risse im Bau:** (a) `ScaleRole` unter-spezifiziert — „Absent" braucht den
  Grund, „Aggregated" den Faktor; (b) **Surrogat-Pfad + Konditionsmengen in
  denselben Schritt** — identische Block-Aggregation auf Daten und Null, sonst
  driftet die FDR-Null still; (c) **Assoziativitätsfalle** — bei Lücken
  Mittel-der-Mittel ≠ Mittel: nie Stufe k aus k−1 ableiten, je Stufe aus der
  nativen Reihe (oder `(sum,count)` je Block).
- Rand: `finest = max(τ_d,τ_t)` (nicht global 60 s), Spanne begrenzt nur, Floor
  entscheidet Gültigkeit; Zeitstempel i64-Sekunden, Blockindex per Ganzzahldivision;
  kalendermonatliche Reihen → erst Coverage annotieren, später Event-time-Blöcke.

## Duck.ai (Claude Haiku 4.5) — GEANTWORTET; Send-Ursache gemessen
**Send-Ursache:** `browser_type` + **Enter sendet nicht** — der React-Input
braucht den expliziten **`Senden`-Button-Klick** (q14). Nach dem Klick ging die
Nachricht raus (Thread „Rust-TE Skalenspektrum"). Das ist der gemessene Grund des
`pending` der ersten Runde.
Verdikt (deckt sich mit Claude/Z.ai):
- (1) Mechanik korrekt; „absent" = unterabgetastet (kein Sample je dt), nicht
  gelöscht. Block-Mittel anti-aliased nur, wenn `dt_orig ≥ Nyquist` der
  Blockperiode; Nyquist/Filter-Annahme **je Feld dokumentieren**. `min(N_native,
  N_agg) > lag_max` je Paar prüfen.
- (2) Zeile = `[scale_idx, Δ, role, N_eff, τ_d_eff, τ_t_eff, TE, p, p_fdr]` +
  Herkunft-Signatur (`bin_count`); Cluster bilden („TE stabil Stufe 1–3, dann
  Kollaps") für D.
- (3) **C → D unidirektional**: D konsumiert die beste TE je Paar (oder
  Multi-Skalen-Gewichte), nicht alle Stufen.
- (4) Reihenfolge richtig; Lücke in Iteration 2: zuerst `ScaleRole` +
  `resample_to_delta()` (explizite Block-Mittelung + Fallback), dann
  `scale_ladder`, dann TE. Erste konkrete Aufgabe: `resample_to_delta()`.

## open-weight GLM 5.3 (tryingopen) — GEANTWORTET (Denkpfad + Verdikt)
- (1) Mechanik korrekt, aber **anti-aliasing ≠ unbiased**: Block-Mittel senkt die
  Varianz und **inflationiert die Autokorrelation**; Aggregation kann **spuriöse
  TE** erzeugen (Runge, causal discovery from aggregated data). TE auf
  aggregierten Stufen ist **nicht direkt über Zeilen vergleichbar** — die
  Null-Verteilungen unterscheiden sich je Skala, also **FDR je Skala**, Surrogate
  **auf jeder Skala identisch aggregieren**.
- Median-`dt` robust gegen Lücken; kalendermonatlich (28–31 Tage) → Power-of-2 in
  Sekunden stößt an (ein Monat ~2,6e6 s liegt zwischen 2²¹ und 2²²) — Leiter je
  Paar ankern.
- (2) Zeile mit `N_eff` und Herkunft je Arm; (3) C → D; (4) Rat-Reihenfolge
  richtig, zuerst Typ + Aggregations-/Resampling-Kern, dann TE.

## Konvergenz (starke Seats + Rat)
- (A)/(B)-Kern bestätigt; B ohne Skalenannotation = stille Fabrikation (schon
  durch `scale` adressiert).
- (1) `median_dt` je Arm, nie wählen; unter max τ absent, nie up-sampeln;
  Block-Aggregation = echt verlustbehaftet. **Korrektiv aller starken Seats:**
  nicht in absoluten Sekunden leitern (Kalendermonat), Rolle pro Arm, `Underpowered`
  als vierter Zustand, extensive Felder → Summe.
- (2) Spektrum = TE je Stufe, kein Mittel über Skalen; FDR-Familie je Stufe fixieren
  (within-rung BH; rungs sind stark abhängig).
- (3) C ist getrennt und Zulieferer von D; PCMCI läuft **pro Skala** auf gemeinsamem
  Gitter ≥ max τ; Kante trägt Skalenlabel.
- (4) Erster Schritt: `ScaleRole{Native,Aggregated,Absent(+Underpowered)}` +
  paargeankerte Leiter; **Claude/Qwen3.8/Qwen**: Signatur mit `τ_d, τ_t, span, n_min`,
  Block-Index-Aggregation des gröberen Arms, Intervall-Zeitstempel; Qwen: Embedding
  je Stufe neu (τ skaliert mit Δ). Der Rat-Bau-Schritt bleibt gültig, wird um diese
  Korrektive geschärft.
