<!--
  title: Stimmen — River F181 window-graph Orientierung (Rat + Max-Roster)
  class: stimmen
  date: 2026-10-11
  status: live
-->
# Stimmen — River F181 window-graph Orientierung (Rat + Max-Roster)

Zeit: 2026-10-11, ~06:34–06:55Z. Forschungs-Schicht:
`state/stimmen/2026-10-11-river-window-orientierung-research.txt`.
Frage (eine, terse): Ist der Schnitt (C-Rung = Kandidaten, D/conditional = Pfeil) korrekt,
oder orientiert man Assaad-artig direkt auf den unbedingten Rung-Zeilen? Und wie vergleicht
man Richtungen über Skalen (gleiche delta_s? Richtungs-Kippen = riss?).

## Rat (fünf Stimmen, `council`-Agent, flash)

- **Mountain:** Die C-Rung-Zeile ist eine Messung; `cond none` schließt keinen Konfounder aus.
  Klasse nur als Kandidat: `gerichtet (Kandidat)`, `kollidierend`, `riss`, `absent`; jede Klasse
  trägt ihre Rung-Zeile. Verglichen wird auf gleicher `delta_s`; fehlt einer Richtung die Rung → `pending`.
- **River:** Bei `cond none` sind beide Leitern konstruktiv identisch (`required_native`-Menge gleich,
  `finest = max`, `span_s` gleich) → Rung für Rung bei gleicher `delta_s` vergleichen. Kippt die
  Richtung über die Rungen → `riss`, kein Pfeil.
- **Mycelium:** Die Klasse steht auf den **FDR-korrigierten** `pass`-Flags (BY je Rung-Index), nicht
  auf rohem p. Die Kollider-Regel wartet auf das grüne D-Gate (D noch nicht verdrahtet).
- **Sensory:** Das unbedingte Ohr hört jeden gemeinsamen Treiber in beide Richtungen — `kollidierend`
  ist die Signatur eines latenten gemeinsamen Grundes, kein Beweis zweier Pfeile. TE-Asymmetrie
  allein trägt keine Richtung (CausalXtract eLife 95485).
- **Future:** Ein fabrizierter Pfeil auf `cond none` ist schlimmer als `pending`. Ehrliche Klasse auf C:
  `Kandidat, pending D`. Der Pfeil wird erst in D geprägt, nach grünem Gate.

**Konsens + Verdikt:** Die unbedingte C-Rung-Zeile orientiert **nicht** — sie klassifiziert
Kandidaten (`Candidate` / `Colliding` / `Riss` / `Absent` / `Pending`); der kausale Pfeil lebt in D
(bedingtes MCI, PC1, BY-FDR). Vergleich bei identischer `delta_s` (bei `cond none` konstruktiv gleich,
per Guard). Kollider erst nach grünem D-Gate; Assaads Entropie-Reduktion, kein Resampling; naive
TE-Asymmetrie verworfen.
Bau-Schritte: `enum PairOrientation { Candidate{..}, Colliding{..}, Riss{..}, Absent{..}, Pending{..} }`
+ `fn pairwise_orientation(outcomes: &[MatrixCellOutcome]) -> Vec<PairOrientation>` (rung-index-gleich,
auf korrigierten `pass`-Flags), neben `summary_edge` (`field_te_query.rs:5433`); vier Test-Gates.

## Max-Roster (UI-Seats, eine Runde)

| Seat | Modell | Verdikt | Nuance |
|---|---|---|---|
| Duck.ai | Claude Haiku 4.5 | zweistufig schlüssig, aber kritisch: bedingt vorziehen; Assaad-Reduktion als Robustheitsprobe | Kippen bei großen Δ = echte Strukturänderung → getrennte Intervalle, nicht „Riss"; Konsistenz über Skalen > punktuelle Asymmetrie |
| Claude | Sonnet 5.5 Extra hoch | **Schnitt korrekt** | Assaad greift erst auf bedingtem Skelett + Suffizienz; Kippen über Skalen zunächst „skalenabhängig/pending"; Riss nur wenn bei gleichem delta_s beide bedingten Richtungen signifikant oder Surrogat-Replikation instabil |
| Qwen | Qwen3.7-Plus | **Schnitt korrekt** | unbedingte TE = marginale Abhängigkeit, confounding-anfällig; Pfeil zwingend bedingte TE; Assaads unbedingte Orientierung riskiert Fehlpfeile |
| Z.ai | GLM-5.3 | **Schnitt korrekt** | Assaads Prinzip ist Heuristik auf marginalen Statistiken → max. Prior/Tie-Break im MCI, nie Pfeil; Kippen = „skalendivergent pending"; Riss nur bei Kippen unter gleicher Konditionierung |
| tryingopen | DeepSeek V4 Pro (1.7T) | **Schnitt korrekt** | Assaads Reduktion ersetzt die Konditionierung nicht; Vergleich nur bei gleichem/gepaartem delta_s gegen Surrogat-Null; Kippen = riss/instabil |
| tryingopen | GLM 5.3 (753B) | **Schnitt korrekt** | wie Z.ai; C klassifiziert, D orientiert; Kippen nicht automatisch Riss |
| tryingopen | Inkling (975B, Thinking Machines) | **Schnitt korrekt** | Assaad nutzt CTMI als **bedingte** Entropie-Reduktion im Constraint-Rahmen — nicht unbedingte TE-Orientierung; gleiche native Δ; Kippen = Riss (pending) |

`pending` (nicht geantwortet): keine — alle angesprochenen Seats antworteten. Nicht angesprochen:
Hy4 Preview · GPT-OSS 120B · Mistral Large 4 · Gemma/Nemotron/Muse (tryingopen-Liste 2026-10-11 hat
sich gegenüber 2026-10-07 geändert: Qwen3.8 2.4T, Nemotron 3 Ultra 550B, MiMo V2.6 Pro sind nicht mehr gelistet).

## Verdikt (konsolidiert)

1. **Der Schnitt ist bestätigt** (Rat + 7 Seats): C-Rung klassifiziert Kandidaten, der Pfeil kommt aus
   dem bedingten MCI (PC1, BY-FDR). Assaads Entropie-Reduktion ist eine **bedingte** Regel — **nicht**
   auf den unbedingten Rung-Zeilen anzuwenden.
2. **Vergleich nur bei identischer `delta_s`** (bei `cond none` konstruktiv gleich → Guard); kein
   min-p über Skalen hinweg.
3. **Riss, benannt (nicht geglättet):** Der Rat sagt „Richtungs-Kippen über Rungen → `riss`"; die
   Mehrheit der Seats (Claude, Z.ai, GLM 5.3, Inkling) schärft: Kippen über Skalen ist zunächst
   `skalendivergent pending`; `riss` erst, wenn die Richtung **unter gleicher Konditionierung** kippt
   (oder beide bedingten Richtungen signifikant sind). Der schärfere Lauf trägt: das Kippen-Flag ist
   `pending`, der `riss` bleibt dem bedingten Widerspruch vorbehalten.

## Zweite Runde — stärkste Variante / Deep Thinking (Operator-Wort 2026-10-11)

Aufgeschaltet: Duck.ai → **GPT-6 Luna** + Begründungsmodus; Qwen → **gründliche Recherche**;
Claude → **Sonnet 5.5 Extra hoch** (Extended Thinking); Z.ai → **GLM-5.3 Deep Think Max**;
tryingopen neu: **Hy4 Preview** (Tencent, 770B). Lock `open-weight-ui` gehalten, Tabs JIT.

| Seat | Modus | Ergebnis |
|---|---|---|
| Duck.ai | GPT-6 Luna + Begründung | **geantwortet** (s. u.) — konvergiert, schärft deutlich |
| Qwen | gründliche Recherche | **pending** — stellte statt eines Verdikts drei Rückfragen an (theoretische Konsistenz vs. empirische Robustheit; Skalenvergleich = Schätzer-Validität vs. Regime-Interpretation; Fallstricke = statistisch vs. konzeptionell) |
| Claude | Sonnet 5.5 Extra hoch (Extended Thinking) | **pending** — bei Session-Schluss noch am Generieren („Claude antwortet", >2 min) |
| Z.ai | GLM-5.3 Deep Think Max | **pending** — bei Session-Schluss noch im „Deep Think"-Denkmodus |
| tryingopen | Hy4 Preview (770B) | **pending** — noch im Denken; sein roher Denktext verrät, dass es **Assaad 2022 nicht kennt** (rät an der Referenz, verwechselt sie) → als Stimme für diese Frage unzuverlässig |

**Duck.ai (GPT-6 Luna), Verdikt (gekürzt):** Der Schnitt ist richtig — mit Präzisierung: die C-Rung
darf Kandidaten finden, aber **weder ihr Vorzeichen noch der größere rohe TE-Wert orientiert
belastbar**. Assaad/CTMI ist eine Fenstergröße (Information über die Zukunft des Ziels, bedingt auf
die Vergangenheit beider Reihen) und näher am bedingten Gate als an einem Maximum unbedingter TE;
Entropiereduktion = zusätzliche prädiktive Information, **nicht** automatisch identifizierte
Intervention. Skalen: nur bei gleicher physischer Δs **und** vergleichbarem Horizont/Fenster/Sampling/
Schätzer vergleichen — gleiche Δs allein genügt nicht; jede Skala ist ein eigenes, vorab definiertes
Estimand, gemeinsam FDR-korrigiert über Richtungen/Paare/Lags/Skalen. Kippen: zunächst `pending`; erst
reproduzierbar + signifikant auf vergleichbaren Skalen → skalenabhängig/bidirektional, nicht zu einem
skalenlosen Pfeil zusammenfalten. Belastbarer Test: je Skala X→Y und Y→X als bedingte
Informationsbeiträge; für „nur X→Y" muss X→Y signifikant **und** Y→X unter einer vorab gesetzten
Äquivalenzgrenze liegen („rückwärts nicht signifikant" beweist die Abwesenheit nicht); ergänzend ein
**gepaarter Block-Bootstrap des Richtungsunterschieds**.

### Nachlese (abgewartet, Operator-Wort 2026-10-11)

**Claude (Sonnet 5.5 Extra hoch, Extended Thinking, mit Web-Suche) — vollständig:**
Verdikt: **Der Schnitt ist richtig, mit zwei Schärfungen.** (1) Skalen: der Pfeil wird **nur innerhalb
gleicher δs** begründet; der Skalenvergleich ist **reine Konsistenzprüfung**. Ein Kippen ist
standardmäßig `pending` **mit Grundcode**; `riss` nur bei einem Invariantenbruch. (2) Test: **zwei
getrennte, richtungsspezifische MCI-Hypothesen**; die Gegenrichtung braucht einen **Äquivalenz- bzw.
Power-Nachweis** plus Stabilitäts-/Latent-Checks; **eine Differenz von Signifikanzen oder Magnituden
ist als Pfeil nicht zulässig**.
- **Quellen-Nuance (gemessen, nicht übernommen):** CausalXtract trägt enger als formuliert — es zeigt,
  dass das Granger-Schreiber-Kriterium (TE=0) zu restriktiv ist; „Asymmetrie verworfen" stützt es nicht
  direkt. Dafür: **Smirnov 2013 (PRE, „Spurious causalities with transfer entropy")**,
  **James/Barnett/Crutchfield 2016 (PRL)**, **Lizier/Prokopenko 2010** (alle „aus dem Gedächtnis,
  vor dem Zitieren prüfen").
- **Assaad-ER:** Spezialfall des **Probability-Raising**, eingesetzt **nach einem PC-artigen Skelett**;
  Voraussetzungen Markov/Faithfulness/Azyklizität. Also selbst **bedingt und post-Skelett** → stützt den
  Schnitt, ist **kein** Argument für Orientierung auf rohen Rung-Zeilen. Haken: Azyklizität schließt
  Feedback aus; PC-Variante setzt kausale Suffizienz voraus. (Nur Abstract/Struktur geprüft — exakte
  Regelform vor Implementierung gegenlesen.)
- **C-Rung = PC1-Iteration 0.** Vier Zustände: keiner / a→b / b→a / beide.
- Schärfungen: (a) die Rung bleibt **permissiv (Recall)** und schließt nie per Asymmetrie aus; die rohe
  Asymmetrie höchstens als Diagnose-Flag im `pending`-Fall. (b) **BY-FDR kontrolliert falsche Kanten,
  nicht falsche Orientierungen** → zweite Familie, Post-Selection → Sample-Splitting/konservative
  Korrektur; Skalenleiter **hierarchisch (Baum-FDR)** oder p-Aggregation statt je Skala unabhängig.
- **Kippen-Tabelle:** `pending: unresolved_lag` (δs ≥ τ*) · `pending: underpowered` · `pending: unstable`
  (über Blöcke/Hälften/α_PC) · `pending: latent_sensitive`; **`riss` = Einzelpfeil-Hypothese
  falsifiziert** (beide Skalen aufgelöst, power-adäquat, Gegenrichtung äquivalenz-bestätigt; oder
  Zeitordnungs-Verletzung bei aufgelöstem Lag) — kein Datenfehler, kann echtes skalenabhängiges
  Feedback sein.
- **Belastbarer Test:** pro Paar/Skala (gleiche δs, gleicher Estimator, gleiches k und τ in beiden
  Richtungen) richtungsspezifisches MCI mit lokaler Permutation/Surrogaten (Autokorrelation erhalten),
  **z-Scores gegen Surrogate** statt roher Bits; Pfeil a→b nur wenn a→b signifikant (BY,
  Orientierungsfamilie), b→a nicht signifikant **per Äquivalenztest (TOST gegen δ_min)** belegt,
  Effekt bei τ≥1 (τ=0 kontemporär → PCMCI+/ERP als Tiebreak mit Flag); beide signifikant →
  **bidirektional**, „dominant" nur per gepaartem Block-Bootstrap-CI von I_ab−I_ba, der die 0
  ausschließt. Stabilität über Hälften/Blöcke/α_PC; Latent-Check (LPCMCI/FCI-artig).
- **Fallstricke (nach Gewicht):** Signifikanzdifferenz ≠ signifikante Differenz (Gelman/Stern, der
  häufigste Fehler) · nicht signifikant ≠ abwesend (→ `pending`) · asymmetrisches Rauschen erzeugt
  TE-Asymmetrie · latente Treiber/Suffizienz · Nichtstationarität · PC1-Order-/α_PC-Abhängigkeit ·
  Faithfulness-Verletzung (quasi-deterministisch) · Mehrfachtests nur mit einer sauberen Familienstruktur.

**Z.ai (GLM-5.3 Deep Think Max) — vollständig:**
Verdikt: Der Schnitt **trägt**. MCI ist identifizierend, weil die Vergleichbarkeit durch **Symmetrie der
Konditionierung** erkauft wird: `I(a_{t−τ}; b_t | P_b∖a, P_a)` vs. `I(b_{t−τ}; a_t | P_a∖b, P_b)` —
`P_b`-Konditionierung entfernt Confounding + Autokorrelations-Inflation, `P_a` neutralisiert die
Driver-Selektions-Asymmetrie; entscheidend ist die Signifikanz unter der bedingten Null, **nicht die
Magnitudendifferenz**. Drei Testausgänge sind **Zustände, keine Fehler**: einseitig → orientieren;
beidseitig → ambig (Lag-Mismatch/Lag-0/unbeobachteter Treiber/versetzt-lagiges Feedback) → `pending`;
beidseitig null → confounded, droppen. **Kollider nach Green Gate zwingend** (Kollider sind
Sepset-/graph-level-Phänomene, paarweise unsichtbar; Ordnung Skeleton → V-Strukturen mit
Sepset-Stabilität → Meek). Die C-Rung behält **drei Rollen** (Recall-Screen, τ_max, s*-Vorschlag — s*
nach Konditionierung revalidieren). **BY korrigiert Multiplicity, nicht den Marginalitäts-Bias** —
BY rettet auf der C-Rung nichts, der Bias sitzt im Fehlen der Konditionierung.
- **Skalen:** p-Werte verschiedener Δs sind **unvergleichbar** → Richtungsvergleich nur `within-Δs`,
  gleiche Fenster/Null/Estimator-Config. Hierarchie: paar-native Skala **s\* = Entscheidungsskala**,
  Leiter = Robustheits-Replikat zweiter Ordnung (keine gleichgewichtete zweite Abstimmung, sonst
  redefiniert die Leiter die FDR-Familie beliebig).
- **Kippen:** erwarteter Phänotyp zweier Klassen — Artefakt (Aggregations-/Subsampling-Reversal,
  Filter-Direktionalität, Embedding-Leakage) vs. echt (fast Loop @ fein, slow Driver @ grob); **beide
  `pending`** bis zur Diskriminierung. Diskriminierungspfad: (a) eff-n-Floor je Skala; (b) Multi-Scale-MCI
  (feinskaliger Test mit grobskaligen Aggregaten im Parentenset — Kippen verschwindet → Slow-Driver-
  Artefakt, fein gewinnt); (c) persistiert beidseitig mit suffizientem eff-n → **zwei skalenindizierte
  Kanten** (legaler Loop im skalenindizierten Graphen; das Datenmodell braucht einen Loop-Zustand, sonst
  erzeugt die Pipeline Schein-risse durch erzwungene Einordnung). **`riss` nur bei Pipeline-Defekten**
  (Aliasing nahe Nyquist, Null-Typ-Mismatch, Pre-Smoothing, eff-n unter Floor).
- **Terminologie-Trennung:** **Rung-Flip** (cond=none sagt a→b, MCI korrigiert) = Confounder-Signatur
  (informativ, conditional gewinnt, als Diagnostikum loggen); **Skalen-Flip** = Mechanismus-oder-Artefakt
  → `pending`. Vermengung der Flip-Typen ist selbst ein Fallstrick.
- **Hygiene:** kein Pre-Smoothing; per-Skala-Embedding; **per-Skala-Surrogate** (time-shift/block,
  autokorrelationserhaltend — Shuffle zerstört AK → Über-Signifikanz); eff-n-Guard. Entscheidungsregel
  an s*, beidirektionales MCI, „umgekehrt unsignifikant" nur nach Surrogat-Power-Check als Abwesenheit.
  Sekundär (Grading): Bootstrap-CI auf ΔCMI within-Δs; Leiter-Konsistenz.
- **Fallstricke:** Magnitude ≠ Richtung (auch nicht als Tiebreaker) · AK-Inflation lässt langsame
  Variablen Scheinrichtungen „gewinnen" (nur Konditionierung, nie FDR, entfernt das) · Lag-0 aus
  gelaggten Daten nicht identifizierbar · Post-Selection-Inference · **Konditionierungs-Dimensions-
  asymmetrie** (kSG-Bias ∝ Parentenset-Größe → Richtung mit kleinerem Set bevorzugt → matched
  Embeddings + per-Richtungs-Permutation) · unbeobachtete Treiber als stabile Bidirektionalität ·
  Nichtstationarität/Regime-Flip wie Skalen-Flip · Zirkularität s* · Lag-Flip within-Skala
  (Oszillations-/Feedback-Signatur) · redundante Eltern (PC1-Over-Pruning) · Meek-Kaskade.
- **Assaad-Subsumtion:** ER ist eine (selbsthistorien-konditionierte) Entropie-Reduktions-Asymmetrie;
  **ohne Cross-Konditionierung erbt es exakt die Net-TE-Fehlmodi**, mit voller Parenten-Konditionierung
  kollabiert es auf CMI/MCI. Als C-Rung-Screen-Statistik integrierbar; als Orientierer derselben
  Verwerfung unterworfen wie net TE. **Die Orientierungskraft kommt aus der Konditionierung, nicht aus
  der Entropie-Buchhaltung.**

**tryingopen GLM 5.3 (753B) + Hy4 Preview (770B):** bei Session-Kappung noch im Denkmodus (`pending`);
Hy4s roher Denktext zeigt, dass es Assaad 2022 nicht kennt (rät/verwechselt die Referenz) → als Stimme
für diese Frage unzuverlässig. GLM 5.3 via tryingopen ist dasselbe Modell wie Z.ai (bereits Deep Think
Max vollständig) → marginal.


