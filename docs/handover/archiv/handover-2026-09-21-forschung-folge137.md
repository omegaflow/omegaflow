<!--
  title: Handover — Forschung-Folge 137 (Stand 2026-09-21)
  session: Forschung-Folge 137
  class: handover
  date: 2026-09-21
  sha256: 63784e55e9a249b32562d6591fa6de7064991f5bf08f4e1012faad8eafd9aa6f
  status: live
-->
# Handover — Forschung-Folge 137 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 137)

- **HEAD** `096ed904` == `origin/main` (fremde Commits seither: `f07dce4d`/
  `308c5bd1` bau-folge123, `096ed904` entscheid-folge82; folge136 nannte
  `fe8ee746`). Arbeitsbaum trägt fremde uncommittete Arbeit
  (`docs/specs/mantis-shrimp-bom.md`, `phi/pipeline/ledger.φ`, `phi/sources.φ`,
  `docs/specs/mantis-shrimp-build.md`) — nicht angefasst, nicht committet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", 100 % von $5.00 free credits, September 2026 — informativ);
  kein neuer Eingang seit folge136, kein handlungsbedürftiger Fall.
- **CI** — die drei Läufe sind **nicht aufgelöst**: `hyperscanning-te`
  `35598791059` @`bdd2cf1e` **pending**, `te-gate` `35595896140` @`44e77da0`
  **in_progress**, `ci-check` `35598781610` **pending** (`ci_manage view`,
  einmal gelesen, kein Poll).

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
- **Lage:** Fix A committet (`0f8bc1b2`); `35598791059` @`bdd2cf1e` pending
  (`ci_manage view`).
- **Blockade:** Run-Abschluss (funktionaler Lauf nur in CI).
- **Braucht:** `ci_manage view 35598791059` einmal.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35595896140` @`44e77da0` in_progress; der `te_fn_probe`-Step steht
  (`te-gate.yml:57`).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35595896140`.

## Punkt 3 — `--dropped` Delta-Gate: Baseline 2158 → 2219

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2219** (2026-09-21 folge137,
  Delta 61 über 2158); Baseline in `docs/zustand/dropped-baseline.md` im
  annehmenden Commit nachgezogen.
- **Blockade:** nächster `ci-check`-Lauf.
- **Braucht:** `ci_manage view <ci-check-id>`; bei neuer Drift erneut messen.

## Punkt 4 — confirmation-Test: Schätzer-Entscheid gemessen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Zwei unabhängige Taucher (flash `general`, `research-max`) lesen
  Kirkley 2025 „Transfer entropy for finite data" (`arXiv 2506.16215v3`, DOI
  `10.1103/tcss-5hn3`) + `RTE.py`
  (`github.com/aleckirkley/Reduced-Transfer-Entropy`). Verdikt: reduced TE ist
  ein **diskret/gebinnter** mikrokanonischer Schätzer (Korrektur Δ ≤ 0,
  MDL-Signifikanz R < 0 ⇔ kein Fluss, Bias ⟨T⟩ ≈ Cˡ(Cᵏ−1)(C−1)/(2N)) — er
  **misst ein anderes Objekt** als der kontinuierliche Takens-KSG des Tests
  (`hyperscanning_group_te.rs:1521`, n=800, dim 3, KSG k=4, 200 Phasen-Surrogate,
  p99). **Ergänzen, nicht ersetzen**; MDL-Konsistenz ist keine kalibrierte FPR
  bei n=800/1000. Rust-Port-Pfad: Kontingenztabellen (`HashMap`), log-Fakultät
  via Lanczos-`lgamma` (std-only), Ort `src/mathematikerin/te.rs`; Kanon ist das
  Paper, nicht `RTE.py` (verstümmelter Normalisierungsabschnitt). `confirm`-Job
  in `hyperscanning-te.yml` verdrahtet; `35598791059` pending.
- **Blockade:** Verdikt des `confirm`-Jobs.
- **Braucht:** `ci_manage view 35598791059`; bei Ergänzung Rust-Port reduced TE
  (`grind-pro`/`grind-max`, `src/mathematikerin/te.rs`).

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen (`35598791059`).
- **Blockade:** Run-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — Riss 4 (KSG↔KDE): B gebaut; Zahlen-Step + CI-Nachweis

- **Status:** wartend | **Bindung:** eigen
- **Lage:** B committet (Shader-Kommentar `shaders.rs:483,525`; Sprechort
  `solar.rs:445,449`, `matrix.rs:894,911,1059`; Gate-Fixture
  `riss_ksg_kde_estimator_split_is_measured` `te.rs:3596`). Kirkley löst den Riss
  **nicht** (dritter, diskreter Pfad — gemessen 2026-09-21): KSG (`te.rs:2165`)
  gegen KDE (`te.rs:2084`) bleiben die zwei kontinuierlichen Böden.
  `te_fn_probe` steht in `te-gate.yml:57`.
- **Blockade:** CI-Nachweis + die zwei Zahlen.
- **Braucht:** `ci-check` (push-getriggert); `ci_manage view <te-gate-id>`.
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
| 1. `family_fn_gate` Fix A | wartend | eigen | `35598791059` pending @`bdd2cf1e` | Run-Abschluss | `ci_manage view 35598791059` |
| 2. `te-gate` n=1000 | wartend | eigen | `35595896140` in_progress | Run-Abschluss | `ci_manage view 35595896140` |
| 3. `--dropped` Baseline | wartend | eigen | current 2219, Baseline 2219 | nächster ci-check | `ci_manage view <ci-check-id>` |
| 4. confirmation / Schätzer | wartend | eigen | Entscheid: ergänzen; `35598791059` pending | `confirm`-Verdikt | `ci_manage view 35598791059`; Rust-Port |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35598791059` |
| 6. Riss 4 KSG↔KDE | wartend | eigen | B committet; Kirkley löst nicht | CI-Nachweis + Zahlen | ci-check; `te-gate`-Lauf |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- **Schätzer-Entscheid (2026-09-21, folge137):** flash `general` **$0.0194** vs
  `research-max` **$0.0369** — identisches Verdikt („ergänzen, nicht ersetzen"),
  flash 1,9× billiger → **flash Sieger** der Routineklasse; pro bleibt Eskalation
  für das harte Atom.
- **`archive_search` über alle Funktionsklassen (2026-09-21, folge137):** content/
  paths/history/leads + `--all` (~35 Quellen) + Key=Value (`--entrez`/`--cod`/
  `--biomodels`/`--isc`/`--supermag`/`--ena`) + `--arxiv-src`/`--verdict`/`--sniff`/
  `--pdf-text`/`--playwright` gegen die 6 Punkte — ein Durchgang. Gated/leer
  (gemessen): `--brave` 402, `--librs` 403, `--mwmbl` 0, `--semanticscholar`
  rate-limit, `--pubchem` 503, `--interpro` 204, `--alphafold`/`--materialsproject`
  400, `--ena` 400, `--openalex`/`--zenodo` 429, `--wayback` 503. Nicht gefahren
  (Consent/Gerät): `--serve`, `--playwright --headed`, `--mft`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge137.md` (neu)
- Move `handover-2026-09-21-forschung-folge136.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/zustand/dropped-baseline.md` (2158 → 2219, gemessen)
- `docs/zustand/external-state.md` (CI-Zeile)

Fremde uncommittete Arbeit im selben Baum (`docs/specs/mantis-shrimp-bom.md`,
`phi/pipeline/ledger.φ`, `phi/sources.φ`, `docs/specs/mantis-shrimp-build.md`)
wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Diese Session ändert keinen
Workflow — kein Dispatch nötig; `ci-check` läuft push-getriggert. `/consent` ist
der session-weite Consent (Delegation), nie das Commit-Wort.
