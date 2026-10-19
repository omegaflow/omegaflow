# Max-Roster — River: Window-Graph Δ (2026-10-10)

Operator-Wort „bitte --all max roster und rat" (River F174). Rat:
`state/stimmen/2026-10-10-river-window-graph-panel-rat.md`; Forschung:
`state/stimmen/2026-10-10-river-window-graph-panel-research.txt`.

Frage (an alle Seats identisch): Ist Δ = gröbstes gemeinsames natives Gitter
wissenschaftlich tragfähig; ist D NICHT „C gefiltert auf Δ"; was ist der größte
Fehler-/Bias-Risikopunkt (FDR/Selektion/Resampling)?

## Claude (Sonnet 5.5 Extra hoch) — GEANTWORTET
- (1) **Nur als deklarierte Konvention tragfähig**, nicht als „wahre" Skala. Zeitliche
  Aggregation verschiebt die Kausalstruktur: Lags kollabieren zu scheinbar instantanen
  Kanten; ohne Anti-Alias-Filter Aliasing; das Ergebnis hängt vom Operator (mean/sum/
  Subsampling) ab. Δ hängt zudem von der Kanalauswahl ab. Δ **samt Operator explizit
  deklarieren**, Graph als „Δ-skalenbedingt" etikettieren; paarweise Aussagen primär aus
  der Rungtabelle. Monatlich = Kalenderjitter + kleines N = wenig Power.
- (2) **Ja.** MCI konditioniert auf die Eltern von X und Y, also ist D keine Teilmenge von C.
  Divergenz nur lesbar, wenn der Schätzer fix bleibt: ParCorr(none) vs ParCorr(MCI) und
  TE(none) vs TE(MCI) **getrennt** vergleichen. ParCorr↔TE-Unterschied = Schätzer-, kein
  Konditionierungseffekt. C-only → Confounder/indirekte Pfade; D-only → Maskierung/Collider.
- (3) **Größtes Risiko: die Selektions-/Familienkette.** Skeleton, Screening und Test laufen
  auf denselben Daten (Post-Selection); BY nur über die Überlebenden ist zu optimistisch. Die
  Familie muss **alle (Paar, Lag, Rung) vor dem Screening** umfassen, oder das Screening muss
  datenunabhängig/gesplittet sein. ParCorr-Screening verwirft nichtlineare Eltern →
  unvollständige TE-Konditionierung. Phasen-Surrogate setzen Stationarität voraus →
  Saisonalität/Trend vorher entfernen; aggregierte Reihen brauchen ein korrigiertes `N_eff`.

## Z.ai (GLM-5.3 Deep Think Max) — GEANTWORTET
- (1) **Tragfähig — mit Auflagen.** Δ als **deklarierte Designkonstante mit Provenienz**
  (welcher Kanal bestimmte sie? `--scale`-Override? 86400-Notfall geloggt), nicht als
  Dateneigenschaft. Semantik: **reine Blockaggregation (Mittel) der feinen Kanäle, nie
  Interpolation der groben** — MCI erzwingt ohnehin ein Gitter (Bedingungsmengen umfassen alle
  Variablen); Per-Paar-Gitter nur für den Summary-Graph. `N_eff` und max-lag auf
  Δ-Einheiten neu ableiten; ganzzahlige Ratenverhältnisse prüfen.
- (2) **Ja, korrekt:** D ist kein Downstream-Filter von C, sondern eigener MCI-Test
  (Vollbedingung) auf demselben Gitter; C bleibt `cond-none`-Screen. **Aber:** testet D nur
  C-Überlebende, ist das ein **Trichter** — C-Falsch-Negative sind unwiederbringlich. Als
  Auswahlmechanismus deklarieren; Risse sind dann erwartbare Statistik-Divergenz, kein Bug.
- (3) **Der Trichter:** ParCorr filtert mit anderer Statistik vor, BY korrigiert nur auf
  D-Überlebenden → der Gesamtfehler der Pipeline ist unkontrolliert. Verstärkt durch den
  Surrogat-Floor: nach Aggregation quantisieren TE-p-Werte auf `1/(B+1)`, BY wird machtlos,
  Risse wirken wie Methodenkonflikt statt Machtverlust. Sekundär: Sub-Δ-Lags werden
  unsichtbar (nur die Rungtabelle fängt sie). Report: p-Floor, Stufen-Überlebende,
  Divergenzquote; divergente Paare nie still aus dem Summary streichen.

## Tryingopen / GLM 5.3 Flash (Z.ai, Cloud-Route) — GEANTWORTET
- (1) **Ja, tragfähig — mit Vorbehalt:** das größte gemeinsame Gitter ist das einzige ehrliche
  Panel-Raster (kein Upsampling-Flicker). Δ **als Auflösungsgrenze deklarieren, nicht als
  Kausalzeitkonstante**: schneller als Δ geschieht im Panel-Graphen per Konstruktion nichts;
  Kanten mit τ·Δ ≪ nativer dt des Paares sind artefaktgefährdet (Aggregations-Bias). Die
  Rungtabelle trägt die native Skala.
- (2) **Ja.** C = PC1-Skeleton (unbedingt, ParCorr-Screening), D = MCI-Bestätigung (bedingt auf
  Nachbarschaften) — zwei verschiedene Inferenzstufen auf demselben Gitter. C ist kein
  „gefiltertes D"; die Teilmengenbeziehung ist empirisch, nicht definitorisch. Skeleton-Kanten,
  die D fallen lässt, sind genau die indirekten Verbindungen, die MCI eliminieren soll.
- (3) **Größtes Risiko: Selektionsbias** durch bedingte TE auf PC1-Nachbarschaften — die
  Bedingungsmenge kommt vom selben Screening wie der Test; BY über die **falsche Testmenge**
  (nur bestätigte statt aller getesteten Hypothesen) verzerrt FDR nach unten. Sekundär:
  Phasen-Surrogate zu „leicht" unter Autokorrelation → überempfindliches D. Kein Resampling →
  keine Konfidenzintervalle → **Stabilität über Fenster reporten, nicht p-Werte als Wahrheit**.

## Duck.ai — NICHT GEANTWORTET (gemessen)
Tageslimit erreicht, „Wird in 3h zurückgesetzt" — die Sende-Seite nimmt nichts an. `pending`,
gemessener Grund (Tageslimit). Zweiter Kanal (Claude Haiku 4.5).

## Qwen (chat.qwen.ai) — NICHT GEANTWORTET (gemessen)
Tab lädt nicht (Composer-Query und Text-Read laufen in den 30-s-Timeout; Tab-Titel/URL bleiben
leer). `pending`, gemessener Grund (Seite nicht geladen).

## Konvergenz
Rat + Claude + Z.ai + GLM 5.3 Flash konvergieren: **Δ ist eine deklarierte Designkonstante
(Auflösungsgrenze) mit Provenienz, keine Dateneigenschaft**; **D ist ein eigenständiger
bedingter MCI-Test auf demselben Gitter, kein Filter von C**; **größtes Risiko = die
Post-Selection-/Familienkette** (BY über die falsche Testmenge; ParCorr-Screen verwirft
nichtlineare Eltern; Surrogat-Floor nach Aggregation). Divergenz ist `riss`, nie still
gestrichen. Verdikt = Rat (`…-rat.md`).

## Nachtrag — Operator-Wort 2026-10-10: Z.ai = GLM 5.3 **Max**; tryingopen führt GLM 5.3 Max + Qwen

Auf das Operator-Wort („z.ai immer glm 5.3 max gibt es auch in tryong open genauso wie qwen")
wurde die Runde wiederholt: die tryingopen-Seats **explizit** auf die stärksten Modelle
gestellt (nicht die Cloud-Route „GLM 5.3 Flash"), Z.ai mit GLM 5.3.

### Tryingopen / **GLM 5.3** (753B, Z.ai) — GEANTWORTET (2. Runde)
- (1) **Tragfähig — wenn sauber deklariert:** Δ ist die Analyse-/Panel-Skala (gröbstes
  natives Median-dt), **keine physikalische Aussage über Kopplungsstärken**. Die native
  Information bleibt in der Rungtabelle; das Gitter ist nur ein gemeinsamer Operativraum.
  Keine Interpolation als „Messung" ausgeben, fehlende Werte als Missing deklarieren,
  `--scale` als reine Convenience-Override (86400 = expliziter Notwert, nicht Default-Pfad).
- (2) **Ja, genau richtig.** D ist nicht „C auf Δ gefiltert", sondern C = bedingungslose
  Assoziation (ParCorr, `cond none`) auf dem Gitter; D = bedingte Abhängigkeit (MCI,
  konditioniert auf Elternmengen) auf demselben Gitter. C ist Screen/Proxy, D ist das
  Verdikt nach Conditioning. Die Asymmetrie C≠D ist Signal (Kollision/Unterdrückung), kein
  Bug — „Divergenz = riss" ist als Warnung, nicht als Fehlerklasse, richtig modelliert.
- (3) **Größtes Risiko: Selektion vor FDR.** Läuft die TE-Bestätigung nur auf
  ParCorr-überlebenden Paaren, ist die Testfamilie bereits beschnitten (Post-Selection).

### Tryingopen / **Qwen3.8 27B** (Alibaba) — NICHT GEANTWORTET (gemessen)
Composer entgegengenommen, Senden geklickt; die Seite bleibt bei „Still working. The biggest
models take a moment to start" / „writing a reply…" stehen (mehrfach gelesen). `pending`,
gemessener Grund: Seat antwortet nicht innerhalb der Fenster (kein stiller Null).

### Z.ai (chat.z.ai) — GEANTWORTET (1. Runde, GLM-5.3, „Thought Process" = Deep Think)
Antwort siehe oben (Z.ai-Abschnitt); das Modell war GLM-5.3 mit gezeigtem Thought Process.
Das Z.ai-UI führt zusätzlich die Modus-Schalter **Deep Think** und **Max** — auf das
Operator-Wort hin ist für künftige Runden **GLM 5.3 Max explizit** zu setzen.
