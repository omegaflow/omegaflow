<!--
  title: Handover — Forschung-Folge 136 (Stand 2026-09-21)
  session: Forschung-Folge 136
  class: handover
  date: 2026-09-21
  sha256: 8b8636dd93930426569add694d9c31664f4f5eb1f05322883864ac507671eaab
  status: live
-->
# Handover — Forschung-Folge 136 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 136)

- **HEAD** `fe8ee746` == `origin/main` (folge135 `44e77da0` Vorfahr; die
  Übergabe 135 nannte `be8fe4d2` — seither ist `d1ffc82e` entscheid-folge81 und
  `fe8ee746` ernte-folge132 gelandet). Arbeitsbaum trägt fremde uncommittete
  Arbeit (`.github/workflows/tools-build.yml`,
  `tools/harvest/src/bin/glm_l2_compiler.rs`) — nicht angefasst, nicht committet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API
  „usage limit reached", informativ); kein neuer Eingang, kein
  handlungsbedürftiger Fall.
- **CI** — die drei @`44e77da0` dispatchten Läufe sind **nicht aufgelöst**:
  `hyperscanning-te` `35596009980` in_progress (`archive_search --playwright`:
  „hyperscanning-te #17 · 44e77da · In progress"), `te-gate` `35595896140`
  in_progress (seit 12:16), `ci-check` `35595889051` cancelled (von
  `be8fe4d2`/`fe8ee746` überholt). Nach dem Push @`bdd2cf1e` neu dispatcht:
  `hyperscanning-te` `35598791059` queued (`confirm`-Job), `ci-check`
  `35598781610` pending. Kein Poll.

## Riss — getragen als Naht (unverändert, gemessen 35584758519 @c7cb201f)

Der Riss trägt zwei gemessene Böden, die sich weigern zu konvergieren:
Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden
liegt weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet.
Die Naht trägt keine Schätzer-Aussage: der stochastic-driver-Arm (KSG τ=2,
excess +18.2 sd) trägt ihn getrennt; der rote Assert ist Fixture-/Gate-Frage
(Punkt 1). Mountain/River (weißer Boden ≈ Surrogat-Boden): **nicht bestätigt**.

## Punkt 1 — `family_fn_gate`: Fix A, CI-Verdikt ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A committet (`0f8bc1b2`); `35596009980` @`44e77da0` in_progress
  (live via `--playwright` bestätigt). Nach dem Push @`bdd2cf1e` neu dispatcht:
  `hyperscanning-te` `35598791059` queued.
- **Blockade:** Run-Abschluss (funktionaler Lauf nur in CI).
- **Braucht:** `ci_manage view 35598791059` einmal.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35595896140` @`44e77da0` pending; der `te_fn_probe`-Step steht
  (`te-gate.yml:57`, `te_scal`/`te_topo` im Bin, gemessen).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35595896140`; bei Bedarf `gh workflow run te-gate.yml`.

## Punkt 3 — `--dropped` Delta-Gate: Baseline auf 2158 nachgezogen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2158** (zweifach gemessen,
  stabil), `--persist 2` = 358; Baseline in `docs/zustand/dropped-baseline.md`
  auf 2158 gesetzt (delta 65 über 2093, cross-line, im annehmenden Commit).
- **Blockade:** nächster `ci-check`-Lauf.
- **Braucht:** `ci_manage view <ci-check-id>`; bei neuer Drift erneut messen und
  die Baseline im annehmenden Commit nachziehen.

## Punkt 4 — confirmation-Test: als eigener CI-Job verdrahtet

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Test
  `confirmation_confirms_the_strong_pair_against_its_own_null`
  (`hyperscanning_group_te.rs:1521`) war in **keinem** Workflow verdrahtet;
  `ci-check` testet `omegaflow-measure` gar nicht (`ci-check.yml:55-62`).
  Neu: eigener Job `confirm` in `hyperscanning-te.yml` (läuft **parallel** zum
  `screen`-Job, bricht den Screen nicht ab). Der Test misst `observed.te` gegen
  das p99 der 200 Surrogate; die Assert-Nachricht trägt die Zahl. Erwartung laut
  Register: rot. Alternative gemessen: **Kirkley 2025, „Transfer entropy for
  finite data" (arXiv 2506.16215, sha256 `d3c38657…`)** — reduced TE mit
  MDL-Signifikanz **ohne Simulation**, Referenz
  `github.com/aleckirkley/Reduced-Transfer-Entropy`.
- **Blockade:** Verdikt des `confirm`-Jobs.
- **Braucht:** `ci_manage view 35598791059` (Verdikt des `confirm`-Jobs); danach
  Schätzer-Entscheid (p99-Screen vs. reduced TE).

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen; der Lauf `35596009980` liefert die
  Wandzeit.
- **Blockade:** Run-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — Riss 4 (KSG↔KDE): B gebaut; Zahlen-Step + CI-Nachweis

- **Status:** wartend | **Bindung:** eigen
- **Lage:** B committet (Shader-Kommentar `shaders.rs:483,525`; Sprechort
  `solar.rs:445,449`, `matrix.rs:894,911,1059`; Gate-Fixture
  `riss_ksg_kde_estimator_split_is_measured` `te.rs:3596`; AGENTS.md-Riss-Satz;
  Stale-Zeile in `archivar-mathematikerin.md`). Der print-only-Step
  `te_fn_probe` steht in `te-gate.yml:57`. Der Riss ist KSG (`te.rs:2165`) gegen
  KDE (`te.rs:2084`) im selben Crate.
- **Blockade:** CI-Nachweis + die zwei Zahlen.
- **Braucht:** `ci-check` (push-getriggert); `ci_manage view <te-gate-id>` für
  `te_scal`/`te_topo`.
- **Offene Confounder (Council, unverändert):** FN läuft auf dem skalaren KDE,
  kein topologischer Pfad trägt eine topologische FN; der KSG-Kanon speist
  keinen Produktions-Konsumenten (Verdrahtungs-Loch); f32/f64 bleibt ein
  Rest-Toleranzband; der Instantaneous-Phase-Pfad bleibt KDE.

## Wartend / operator-gebunden / termin

- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35598791059` queued @`bdd2cf1e` | Run-Abschluss | `ci_manage view 35598791059` |
| 2. `te-gate` n=1000 | wartend | eigen | `35595896140` pending | Run-Abschluss | `ci_manage view 35595896140` |
| 3. `--dropped` Baseline | wartend | eigen | current 2158, Baseline 2158 | nächster ci-check | `ci_manage view <ci-check-id>` |
| 4. confirmation-Test | wartend | eigen | `confirm`-Job verdrahtet; `35598791059` queued | Job-Verdikt | `ci_manage view 35598791059` |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35596009980` |
| 6. Riss 4 KSG↔KDE | wartend | eigen | B committet; `te_fn_probe`-Step | CI-Nachweis + Zahlen | ci-check; `te-gate`-Lauf |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- `archive_search` über alle Funktionsklassen (content/paths/history/leads/`--all`
  16 Modi/`--ads`/`--arxiv`/`--crossref`/`--openalex`/`--semanticscholar`/`--core`/
  `--wiki`/`--brave`/`--mwmbl`/`--github`/`--zenodo`/`--datacite`/`--ntrs`/
  `--wayback`/`--crates`/`--librs`/`--heasarc`/`--supermag`/`--isc`/`--sniff`/
  `--pdf-text`/`--pdf-image`/`--arxiv-src`/`--serve`/`--verdict`/`--playwright`)
  gegen die offenen Punkte — ein Durchgang; `--brave` 402 (Limit), `--supermag`/
  `--isc`/`--librs` gated (gemessen), `--pdf-image` leer (Vektor-PDF).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge136.md` (neu)
- Move `handover-2026-09-21-forschung-folge135.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/zustand/dropped-baseline.md` (Baseline 2093 → 2158, gemessen)
- `docs/zustand/external-state.md` (CI-Zeile)
- `.github/workflows/hyperscanning-te.yml` (`confirm`-Job)

Fremde uncommittete Arbeit im selben Baum (`.github/workflows/tools-build.yml`,
`tools/harvest/src/bin/glm_l2_compiler.rs`) wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push dispatcht die
Session `hyperscanning-te` (geänderter Workflow); `ci-check` läuft
push-getriggert. `/consent` ist der session-weite Consent (Delegation), nie das
Commit-Wort.
