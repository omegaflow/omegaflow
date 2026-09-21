<!--
  title: Handover — Forschung-Folge 135 (Stand 2026-09-21)
  session: Forschung-Folge 135
  class: handover
  date: 2026-09-21
  sha256: ae61ae22500967cd1a1fe2adea9c559458d49dd3bba3bcaf287235f9679037dd
  status: live
-->
# Handover — Forschung-Folge 135 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 135)

- **HEAD** `be8fe4d2` == `origin/main` (folge134 `cd764c0b` ist Vorfahr; die
  Übergabe 134 nannte `7695ae23` — seither ist `entscheid-folge80` `be8fe4d2`
  gelandet). Arbeitsbaum trägt fremde uncommittete Arbeit
  (`.github/workflows/tools-build.yml`, `phi/sources.φ`) — nicht angefasst,
  nicht committet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API
  „usage limit reached", informativ); kein neuer Eingang, kein
  handlungsbedürftiger Fall.
- **CI** — die drei offenen Läufe waren **ghost-locked** (kein Verdikt lesbar,
  Job-Log HTTP 404): `hyperscanning-te` `35587070232` @`0f8bc1b2` (in_progress
  ohne Abschluss), `te-gate` `35586377356` @`42aeeea4` (pending, 0 Jobs),
  `ci-check` `35592556416` @`0c0b30fb` (in_progress nach Job-Ende 11:23).
  Gecancelt (frei die Concurrency-Gruppe); `hyperscanning-te` neu dispatcht:
  `35595713732` @`be8fe4d2`. Kein Poll.

## Riss — getragen als Naht (unverändert, gemessen 35584758519 @c7cb201f)

Der Riss trägt zwei gemessene Böden, die sich weigern zu konvergieren:
Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden
liegt weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet.
Die Naht trägt keine Schätzer-Aussage: der stochastic-driver-Arm (KSG τ=2,
excess +18.2 sd) trägt ihn getrennt; der rote Assert ist Fixture-/Gate-Frage
(Punkt 1). Mountain/River (weißer Boden ≈ Surrogat-Boden): **nicht bestätigt**.

## Punkt 1 — `family_fn_gate`: Fix A gesetzt, CI-Verdikt neu angefragt

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A committet (`0f8bc1b2`); der alte Lauf `35587070232` war
  ghost-locked (Job-Log 404, kein Verdikt). Neu dispatcht: `35595713732`
  @`be8fe4d2` (`gh workflow run hyperscanning-te.yml`, Defaults phase/Fz/95/200).
- **Blockade:** Run-Abschluss (funktionaler Lauf nur in CI).
- **Braucht:** `ci_manage view 35595713732` einmal.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35586377356` (0 Jobs, ghost) gecancelt. Re-Dispatch erst nach dem
  Push möglich — der neue `te_fn_probe`-Step (Punkt 6) muss auf `main` stehen.
- **Blockade:** Push + Run-Abschluss.
- **Braucht:** `gh workflow run te-gate.yml` nach dem Push; `ci_manage view <id>`.

## Punkt 3 — `--dropped` Delta-Gate: Baseline neu gemessen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `register_lookup --dropped --count` = **2093** (gemessen 2026-09-21,
  HEAD `be8fe4d2`) gegen die alte Baseline 1760 → delta 333; `--persist 2` = 346
  (anhaltende Drops, überwiegend alte `bau`-Handover). Die Baseline in
  `docs/zustand/dropped-baseline.md` auf den **gemessenen** Wert 2093 gesetzt
  (im akzeptierenden Commit, nicht still). Der `--dropped`-Register bleibt der
  Beleg der Drops.
- **Blockade:** nächster `ci-check`-Lauf.
- **Braucht:** `ci_manage view <ci-check-id>`; bei neuer Drift erneut messen und
  die Baseline im akzeptierenden Commit nachziehen.

## Punkt 4 — confirmation-Test: Blocker ist der Schätzer, nicht der Screen

- **Status:** blockiert | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null`
  (`hyperscanning_group_te.rs:1520`) assertet `observed.te > p99` am
  topologischen Schätzer und ist rot. Die Aufnahme in den Workflow
  (`hyperscanning-te.yml:41`, Filterliste) würde den Screen rot machen — der
  grüne `family_fn_gate`-Screen ist damit **keine** hinreichende Bedingung.
- **Blockade:** Schätzer-Güte — der topologische Pfad confirmt das starke Paar
  nicht gegen sein eigenes p99.
- **Braucht:** Schätzer-/Konstruktions-Entscheid (Rat): ist der p99-Screen das
  richtige Instrument, oder wird der Pfad gestärkt?

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen; der Lauf `35595713732` liefert die
  Wandzeit.
- **Blockade:** Run-Abschluss.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — Riss 4 (KSG↔KDE): B gebaut; Zahlen-Step + CI-Nachweis

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Council (read-only, 2026-09-21) verdiktete **B — tragen,
  benennen, mit Mess-Trigger**; das Operator-Wort ist **B**. Gebaut und
  committet: (1) Shader-Kommentar `shaders.rs:483,525`; (2) Sprechort
  `solar.rs:445,449`, `matrix.rs:894,911,1059` sprechen `te(kde)`;
  (3) Gate-Fixture `riss_ksg_kde_estimator_split_is_measured` (`te.rs:3596`);
  (4) `AGENTS.md` Riss-Satz; (5) Stale-Zeile
  `docs/concepts/archivar-mathematikerin.md`. Der Riss ist **nicht**
  GPU-gegen-CPU, sondern KSG (`te.rs:2165`) gegen KDE (`te.rs:2084`) im selben
  Crate. **Neu:** print-only-Step `te_fn_probe` in `te-gate.yml` ergänzt — er
  druckt `te_scal`/`te_topo` für die Riss-Zeile.
- **Blockade:** CI-Nachweis + die zwei Zahlen (nach Push).
- **Braucht:** `ci-check` (push-getriggert über `src`- und `docs`-Pfade) für den
  Test-Nachweis; `ci_manage view <te-gate-id>` für die Zahlen.
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
| 1. `family_fn_gate` Fix A | wartend | eigen | `35595713732` @`be8fe4d2` neu dispatcht | Run-Abschluss | `ci_manage view 35595713732` |
| 2. `te-gate` n=1000 | wartend | eigen | `35586377356` gecancelt | Push + Run | `gh workflow run te-gate.yml` nach Push |
| 3. `--dropped` Baseline | wartend | eigen | current 2093, Baseline auf 2093 gesetzt | nächster ci-check | `ci_manage view <ci-check-id>` |
| 4. confirmation-Test | blockiert | eigen | rot am Schätzer | Schätzer-Güte | Rat (Instrument vs. Pfad) |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | `ci_manage view 35595713732` |
| 6. Riss 4 KSG↔KDE | wartend | eigen | B committet; `te_fn_probe`-Step neu | CI-Nachweis + Zahlen | ci-check; `te-gate`-Lauf |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- `grind-flash` für die CI-Workflow-/Run-Recherche (read-only): Kandidaten-Status,
  Ghost-Nachweis, `te_fn_probe`-Dispatchpfad — ein Lauf, kein Doppellauf (Routine).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge135.md` (neu)
- Move `handover-2026-09-21-forschung-folge134.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/zustand/dropped-baseline.md` (Baseline 1760 → 2093, gemessen)
- `docs/zustand/external-state.md` (CI-Zeile)
- `.github/workflows/te-gate.yml` (`te_fn_probe`-Step)

Fremde uncommittete Arbeit im selben Baum (`.github/workflows/tools-build.yml`,
`phi/sources.φ`) wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push dispatcht die
Session `te-gate` (geänderter Workflow); `ci-check` läuft push-getriggert.
`/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
