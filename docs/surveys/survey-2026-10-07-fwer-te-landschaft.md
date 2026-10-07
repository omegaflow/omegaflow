<!--
  title: Survey — FWER/TE-Forschungslandschaft für die GIC-Stufe-2 (2026-10-07)
  class: survey
  date: 2026-10-07
  sha256: 96fc86f1806c754cbdb990919b88c105963d705470db956637f08a28cd51ac65
  status: live
  see-also: docs/blatt/blatt-gic-breitenband-familien.md docs/handover/archiv/handover-2026-10-07-river-folge122.md
-->

# Survey — die Forschungslandschaft zur GIC-Stufe-2-Null (2026-10-07)

Die Frage: Wie wird die GIC-Breitenband-Familien-Null in Stufe 2 (`compute_max_t`) gelegt, ohne
Leckage und ohne die Stufe-1-FDR (`matrix full` + BH/BY) zu verändern? Die schwachen Voice-Stimmen
(Recherche-Rolle, Operator-Wort 2026-10-07) und `archive_search` grasen die Landschaft ab. Jede
Referenz ist **am Baum gemessen** (Titel → DOI via crossref/openalex/arxiv, `--verdict`-Routen) oder
als Stimmen-Recherche gekennzeichnet und, wo geladen, verifiziert. Provenienz je Zeile in Klammern.

## 1 · Starke FWER bei korrelierten Tests — max-t / Permutation / Stepdown

- Westfall & Young, *Resampling-Based Multiple Testing* (Wiley 1993) — Buch, kein DOI (`voice-deepseek`, nicht einzeln verifiziert); Methode über die nächsten Zeilen belegt.
- Romano & Wolf, *Exact and Approximate Stepdown Methods for Multiple Hypothesis Testing*, JASA 2005 — `10.1198/016214504000000539` (crossref-Titel-Exaktmatch) · SSRN-Vorlauf `10.2139/ssrn.563267`.
- Romano & Wolf, *Stepwise Multiple Testing as Formalized Data Snooping*, Econometrica 2005 — `10.1214/009053605000000066` (verdict HTTP 200; **gemini hatte diese ID fälschlich dem Stepdown-Paper zugeordnet**).
- Meinshausen, Maathuis & Bühlmann, *Asymptotic optimality of the Westfall–Young permutation procedure under multiple testing under dependence*, 2011 — `10.1214/11-aos946` (crossref-Titel-Exaktmatch).
- Frossard & Renaud, *The cluster depth tests: Toward point-wise strong control of the family-wise error rate in massively univariate tests…*, NeuroImage 2021 — `10.1016/j.neuroimage.2021.118824` (openalex).
- Winkler et al., *Permutation inference for the general linear model*, NeuroImage 2014 — `10.1016/j.neuroimage.2014.01.060` (crossref-Titel-Exaktmatch).

→ Deckt: der geteilte Bz-Treiber erzeugt Abhängigkeit; das **max-t über gemeinsame Draws** trägt die
starke FWER-Kontrolle, **nicht Šidák** (setzt Unabhängigkeit voraus). „Gleich" auf drei Familien ist
Bonferroni.

## 2 · Zweistufige Kontrolle — Gatekeeping / hierarchisches Testen

- *Gatekeeping Procedures in Clinical Trials* (Bretz & Maurer, in *Multiple Testing Problems in Pharmaceutical Statistics*) — `10.1201/9781584889854-10` (crossref).
- *Powerful short-cuts for multiple testing procedures with special reference to gatekeeping strategies* — `10.1002/sim.2873` (crossref).
- *Multiple Testing in Group Sequential Trials Using Graphical Approaches* — `10.1080/19466315.2013.807748` (crossref).
- CRAN `multxpert` / `gMCPLite` — graphische Multiple-Testing-/Gatekeeping-Verfahren.

→ Deckt: **Stufe 1 = globales Tor (Discovery), Stufe 2 = bestätigend innerhalb der Familien** — das ist
die etablierte „gatekeeping"-Form; die Familien-Null ist die bestätigende Ebene.

## 3 · Post-Selection / Selective Inference — der Auswahl-Riss

- Berk, Brown, Buja, Zhang & Zhao, *Valid Post-Selection and Post-Regularization Inference: An Elementary, General Approach*, 2015 — `10.1146/annurev-economics-012315-015826` (openalex, 152 cites).
- *Exact Post-selection Inference for Forward Stepwise and Least Angle Regression*, 2014 (Polyhedral-Lemma) — openalex `W172338866`.
- *Uniform asymptotic inference and the bootstrap after model selection*, 2018 — `10.1214/17-aos1584` (openalex).
- *Splitting strategies for post-selection inference*, Biometrika 2022 — `10.1093/biomet/asac070` (openalex).

→ Deckt: die Null **nur über die Stufe-1-Überlebenden** zu kalibrieren ist Auswahl-Leckage; die
Literatur trennt bedingte von selektiver Inferenz und bietet Datensplitting als Ausweg.

## 4 · Transfer-Entropie auf endlichen Daten — Bias und Signifikanz

- Kirkley, *Transfer entropy for finite data*, 2025 — `arXiv:2506.16215` (arxiv; sparse-bin-Bias, simulationsfreie Signifikanz).
- Schreiber, *Measuring Information Transfer*, PRL 2000 — `10.1103/PhysRevLett.85.461` (`voice-deepseek`, nicht einzeln verifiziert).
- Vicente et al., *Transfer entropy—a model-free measure of effective connectivity for the neurosciences*, 2011 — `10.1007/s10827-010-0262-3` (`voice-deepseek`).
- Ramos & Macau, *Minimum Sample Size for Reliable Causal Inference Using Transfer Entropy*, Entropy 2017 — `10.3390/e19040150` (`voice-deepseek`).
- Theiler et al., *Testing for nonlinearity in time series: the method of surrogate data*, Physica D 1992 — `10.1016/0167-2789(92)90102-s` (crossref-Titel-Exaktmatch).
- Wollstadt et al., *TRENTOOL*, 2014 — `10.1016/j.jneumeth.2014.04.010`; Runge et al., *Detecting and quantifying causal associations in large nonlinear time series datasets*, Sci Adv 2019 — `10.1126/sciadv.aau4996` / `10.1038/s41467-019-10105-3` (openalex).

→ Deckt: der **unvollständige dB/dt-Pool** ist der bekannte Finite-Data-Bias; die Surrogat-/Phase-Null
ist die Standard-Signifikanzform.

## Synthese

Die von Rat + UI-Frontier gewählte Methode ist literaturgedeckt:

- **(c)** `--stage2 family` = **Gatekeeping** (Stufe-1-Tor → Stufe-2-bestätigend) [§2].
- je Familie **max-t über gemeinsame Draws** = Westfall-Young / cluster-depth [§1]; **Šidák ungedeckt**, Bonferroni „gleich".
- **Target-Band**, Treiber unbandiert (band-degeneriert).
- Zwei Risse, beide mit Literatur: (1) unvollständiger Pool → Finite-Data [§4, Kirkley]; (2) die Null
  nur über den vollen Pool, nie über Stufe-1-Überlebende → Post-Selection/Selective Inference [§3].

**Adressierung:** Operator-Wort 2026-10-07 („die schwachen voice stimmen … um die wissenschafts- und
forschungslandschaft … abzugrasen"; „mit archive search können wir doch die komplette wissenschafts-
und forschungslandschaft befragen"). Rohmaterial `state/river/gic-stage2-20-stimmen-2026-10-07.md`;
Recherche-Benchmark `state/benchmark/2026-10-07-recherche-stimmen.md`.
