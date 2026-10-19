# Max-Roster — D2 Summary-Graph/PCMCI (River, 2026-10-10)

Frage wie im Rat (`state/stimmen/2026-10-10-river-d2-pcmci-rat.md`).
Runde: `river-ui` (Claude · Z.ai · Duck · Qwen) + geteilt `open-weight-ui`
(Qwen3.8 2.4T; tryingopen, Lock).

## Claude (Sonnet 5.5 Extra hoch) — GEANTWORTET
- **(1) ParCorr trägt den Algorithmus, TE bleibt außerhalb der Schleife.**
  Phasen-Surrogate sind als CI-Test **nicht gültig**: das Surrogat von X zerstört auch
  die X–Z-Abhängigkeit, die Null ist „X ⊥ (Y,Z)" statt „X ⊥ Y | Z" → bei korreliertem
  X,Z systematisch falsch-positiv; die Null ist ein linearer Gaußprozess mit gleichem
  Spektrum, eine Ablehnung kann bloße Nichtlinearität sein. Bei BY mit m≈300 liegt
  α_adj≈2,6·10⁻⁵ → rank-p bräuchte B ≳ 4·10⁴ Surrogate/Test; binned konditionierte TE
  hat bins^|Z| Zellen (bei |Z|≥3 datenleer). → **ParCorr (OLS-Residuen, t-Test
  df = n_eff−|Z|−2)** für PC1+Skeleton+MCI; binned TE (nur marginal oder |Z|≤1) als
  nachgelagerte Konsistenzprüfung, Label „TE-gestützt / nur linear"; Nichtlinearität
  bleibt erklärter Blindfleck von D.
- **(2) PCMCI+ statt PCMCI**, auf dem Window-Graph, dann kollabieren: PC1-Elternselektion
  → lagged MCI → kontemporäres Skeleton (PC-stable) → Kollider → Meek. Orientierung:
  lag>0 → i→j; kontemporär undirektiert bis Kollider. Summary-Graph erst nach dem
  Window-Graph: i→j wenn irgendein Lag signifikant, annotiert mit min-p-Lag; darf
  Zyklen (i⇄j) enthalten; PC nie direkt auf dem kollabierten Graph. max_lag =
  min(Cap≈3, ⌈τ_max aus C-Tabelle / scale⌉), ACF nur Plausibilitätsobergrenze.
  α: PC1 liberal ≈0,2 (Selektion, kein FDR); finaler MCI **BY**-Default, BH nur als
  ausdrücklich „explorativ" markierter Tier.
- (3)(4)(5): im vollen Thread `Summary-Graph und PCMCI für Transfer-Entropie-Analyse`.

## Qwen (Qwen3.7-Plus) — GEANTWORTET
- **(1) Zwingend Zweistufigkeit:** binned TE allein als Skeleton-CI-Test ist bei 15
  Feldern × Lags ein Todesurteil (Fluch der Dimensionalität/leere Bins). Stufe 1 (PC1):
  **ParCorr (Fisher-z)**, in Rust std mit reiner Matrixinversion (Gauß-Jordan/Cholesky
  auf der Kovarianzmatrix) ohne Crates. Stufe 2 (MCI): binned TE + Surrogat-p nur über
  die signifikanten PC1-Kanten (fängt Nichtlinearität).
- (2) **PC-stable zwingend** (Reihenfolge darf das Ergebnis nicht ändern); Konditionier-
  mengen-Größen d=0,1,2,… bis Abbruch; MCI `i_{t−τ} ⊥ j_t | Z`.
- (3)(4)(5): im vollen Thread.

## Duck.ai (Claude Haiku 4.5) — GEANTWORTET; GPT-6-Umstellung blockiert
**Operator-Wunsch (2026-10-10): Duck.ai auf GPT-6 umstellen → blockiert:**
Duck.ai zeigt „**Tageslimit erreicht · Wird in 4h zurückgesetzt**"; das Modell-
Menü (Button „Haiku 4.5") öffnet bei erreichtem Limit keine Liste. Gemessener
Zustand, kein stiller Null — die Umstellung ist nach dem Limit-Reset (4h) möglich.
- ParCorr-Screen unverzichtbar; TE nur in der MCI-/Kandidaten-Tripel-Phase; nichtlineare
  ParCorr-Gegenkante → undirektiert belassen. Faustregel: nur 5–10 % der O(d²)-Paare
  überleben den Skeleton → TE bezahlbar.
- PC-stable Ordnung 0,1,2,… (bei 15 Feldern praktisch bis Order 5–6); R0 v-Struktur
  (lag>0 → Kollider), R1–R4 Meek; MCI: TE(i→j|eltern(j)) vs TE(j→i|eltern(i)), höhere
  gewinnt. max_lag aus ACF-95%-KI oder empirisch 12, minimal ≥ 3×Periode; α_skeleton
  0.05–0.1; FDR pro Phase BH, globales α_mci 0.05.

## Z.ai (GLM-5.3 Deep Think Max) — GEANTWORTET (voll)
- **(2) Reihenfolge:** Complete-Case-n fixieren, `n_eff = n_grid − τ_max − Alignierungsverluste`;
  pro Feld Rang→Normal-Scores (Midranks; massive Ties aus gebinnten Rohdaten = Warnung).
  τ_max je Feld via ACF/PACF (bis ACF im 2/√n-Band), Deckel n/10, Untergrenze aus C-Lags,
  global τ_max = max. **PC1** pro Ziel j: Kandidaten (i,τ) τ∈1..=τ_max inkl. Selbst-Lags;
  Konditionierung **nur auf bereits selektierte Eltern**; PC-stable (Adjazenz-Snapshot,
  deklarationsfixe Reihenfolge → Determinismus); α_PC1=0.05 nominal, unkorrigiert (Filter,
  keine Inferenz). **MCI:** `X_{i,t−τ} ⊥ X_{j,t} | Z = P_j∖{(i,τ)} ∪ P_i`. τ=0 eigener
  Durchgang, undirektiert (gestrichelt); Kollider nur optional als Heuristik, kein Meek.
  Ergebnis aggCPDAG-artig, τ>0 per Zeitordnung, Kantenlabel = {τ: signifikant}.
  **FDR:** getrennte Familien je Testtyp; **BH Default + BY-p_adj immer**, Tiers
  „bestätigt" = BY<α, „angedeutet" = BH<α.
- **(3)** PC1 konditioniert nur auf Elternmengen (kein Subset-Search); `|Z| ≤ 2·k_max=16`
  via Eltern-Cap k_max=8 (top-k nach |r|, dokumentiert); MCI ~10²–10³ Tests, Surrogate nur
  auf ParCorr-Überlebende. **n_min:** `n_eff ≥ 10·(|Z|+3)` hart, `≥30·(|Z|+3)` Komfort
  (k_max=8 → 110/330). **Pending-Regeln:** n_eff < 10(k_max+3); |Z| > n_eff/10−3;
  τ_max<1/sub-grid; Complete-Case < 50 % des Gitters; PC1 überall leer aber C-Rung nicht →
  „Scale-Mismatch"; MCI-Familie < 10 Tests → „kleine Familie".
- **(4) Erster Bau-Schritt:** Primitive `erfc` (A&S 7.1.26), Invers-Normal (Acklam),
  `normal_scores`, `parcorr(x,y,z)->(r,p)` via Gram/QR + Unittests gegen Tabellen. **Dann
  das Scharnier `trait CiTest { fn p(&self,x,y,z)->f64 }`** mit ParCorr als erster Impl;
  PC1/MCI generisch darüber, TE später als zweiter Adapter. Lagged-Views ohne Kopie;
  ACF/PACF+τ_max als eigener Schritt; PC1 (~100 Zeilen), MCI dünn; Harness VAR(2) mit
  eigenem xorshift/LCG, Seed fix + in die Rungtabelle. „Nicht zuerst Skeleton oder MCI:
  ParCorr ist das meistbenutzte, fehleranfälligste Stück; der Rest ist Komposition."

## Qwen3.8-Max (`river-ui`, GEANTWORTET)
- (1) Wie Rat/Claude: ParCorr-first, TE als zweite Stufe.
- (2) Orientierung: τ>0 streng gerichtet (Zeitpfeil); τ=0 erst ungerichtet, dann
  Collider-/Meek-Regeln; unentschiedene Kanten als `---` (latent confounded). **max_lag
  nicht blind aus ACF** (ACF misst nur lineare Autokorrelation) — domänenspezifisches
  Hard-Limit τ_max = 3…5.
- (3) **Hartes `p_max` (max degree) = 3–4**; findet ein Knoten mehr Parents in PC1, Suche
  für den Knoten abbrechen → „hub/unresolved"; n_min-Formel für binned TE.

## Konvergenz
- **ParCorr (Fisher-z) als erster CI-Test für PC1/Skeleton — alle starken Seats + Rat.**
  Die binned TE + Phasen-Surrogat ist **kein gültiger konditionaler CI-Test** (Claude:
  falsche Null → Falsch-Positive; bins^|Z| datenleer). TE nur als **zweite MCI-Stufe**
  (bzw. Konsistenzprüfung) über den PC1-Kandidaten, mit Label „TE-gestützt / linear".
- **PC-stable** (ordnungs-unabhängig), Orientierung lag>0 → i→j, kontemporär undirektiert
  bis Kollider/Meek; **Summary-Graph erst nach dem Window-Graph** (Claude).
- **max_lag** aus τ_max/Skala gedeckelt (Claude) bzw. ACF-Obergrenze (Rat/Duck).
- **FDR:** PC1 liberal (kein FDR); finaler MCI **BY** (Rat/Claude) bzw. pro Phase BH +
  global α_mci (Duck).
- **Erster Bau-Schritt:** die **ParCorr-Primitive** in `src/mathematikerin/parcorr.rs`
  (Fisher-z, t-Test, Matrixinversion) + Test-Gate — dann Skeleton, dann MCI.
