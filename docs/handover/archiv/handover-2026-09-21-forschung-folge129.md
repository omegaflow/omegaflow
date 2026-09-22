<!--
  title: Handover — Forschung-Folge 129 (Stand 2026-09-21)
  session: Forschung-Folge 129
  class: handover
  date: 2026-09-21
  sha256: d4bf2a985bdf78c7be149bfb41f0ae6587b959dca5742ba69555bb2b5216a65e
  status: live
-->
# Handover — Forschung-Folge 129 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 129)

- **HEAD** `4ca9a1dd` (== `origin/main`) bei Session-Ende; Session-Beginn
  `ae822fe7` (== `origin/main`), dazwischen `33c124d8` (ernte folge128). Baum
  geteilt: entscheid folge75/76, ernte folge129, bau folge120 parallel.
- **Postfach** — kein neuer Eingang; neuester Ledger-Eintrag `1789973288`.
- **CI** — `ci_manage view`/`log`: `hyperscanning-te` `35572559304` @`a70d20c7`
  **failure** (`family_fn_gate`); `te-gate` `35572569205` @`a70d20c7` pending.
  Watchdog-Snapshot 09:07: `ci-check` `35569256029`, `ps1-cdn` `35569486280`,
  `health-check` `35556807317` in_progress.

## Punkt 1 — Selbst-Null-Diskriminator: Verdikt (CI)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Die Skalen-Reparatur (folge128) ist gemessen gescheitert — CI
  `35572559304` @`a70d20c7`: `family_fn_gate` FAILED, `TE -3.4662e-4 | null mean
  9.4019e-2 sd 3.1688e-2 p95 1.4948e-1 | excess -9.4365e-2 (-2.98 sd) | fam-max
  3.4785e-1` (`nominees_round_trip`/`family_fp_gate`/`coherent_null_fp_gate` ok).
  Diagnose (research-max): die Invarianz-Begründung war ein Kategorie-Fehler —
  der Box-Zähl-KSG ist nur unter **gemeinsamer** Skalierung invariant, nicht
  unter anisotroper per-Koordinate-Reskalierung (`te.rs:2242–2287`); der
  CI-Lauf selbst widerlegt sie. Die C→D-Zelle ist nahezu deterministisch
  (Varianz-SNR ≈ 1100), die 7-dim Joint-Wolke liegt auf einem 6-dim Sheet
  (~2.6 % Dicke) → KSG ≈ 0; die phasenrandomisierte Null ist vollrangig →
  Eigenbias +9.4e-2. Das Signal (9.4e-3) saß 9.4e-2 **unter** der Null.
  Rat (4/5, Sensory ohne Gegenstimme): Standardisierung zurücknehmen — gebaut
  (Block `te.rs:2213–2240` entfernt). Diskriminator gebaut: `self_null_discriminator`
  (`tools/measure/src/bin/hyperscanning_group_te.rs`, C→D-Fixture, τ eingefroren)
  — Arme W1 (Treiber-weiß, Ziel echt) / W2 (beide weiß) / W3 (Ziel-weiß, Treiber
  echt) je N=100, KDE-Arme K1–K3 (`transfer_entropy_embedded_kde`, fbd0f153-Ära),
  τ-Sweep W1 bei τ∈{1,2,4,8,τ_c}; gedruckt μ/σ/p95 je Arm, assertiert nur
  Messbarkeit + Determinismus.
- **Blockade:** CI-Lauf (Dispatch nach Push).
- **Braucht:** nach Push `hyperscanning-te` dispatchen, `ci_manage view <id>`
  **einmal**; μ_W1/μ_W2/μ_W3/μ_K lesen und die Verdikt-Regel anwenden:
  `|μ_W1 − μ_S| ≤ 2σ_W1` → struktureller Boden (Mountain/River; nächstes Atom
  Bias-Kontrolle); `μ_S − μ_W1 > 2σ_W1` → Naht (Mycelium/Sensory; nächstes Atom
  Null-Konstruktion, `randomized_triad:262`/`coherent_phase_surrogates
  te.rs:1609`). μ_S liest der `family_fn_gate`-Druck.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `te-gate` `35572569205` @`a70d20c7` **pending** (misst die
  Kalibrier-Gates am zurückgenommenen Schätzer). Der n=1000-FPR-Boden bleibt
  ungemessen. Eintrag `docs/zustand/external-state.md` (TE-Gate n=1000).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35572569205` einmal.

## Punkt 3 — `--dropped` Delta-Gate: Baseline-Disziplin

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Gate gebaut: `register_lookup --dropped --count`
  (`tools/register/src/bin/register_lookup.rs`), Baseline
  `docs/zustand/dropped-baseline.md` (`dropped 1760`, `commit-resolved 164`,
  `pairs 435`, `candidates 3317` @HEAD `ae822fe7`), Job `dropped-gate` in
  `.github/workflows/ci-check.yml` — fail ⇔ `current > baseline` (Delta > 0,
  nie Absolutwert). Erster CI-Lauf bestätigt die Baseline noch nicht.
- **Blockade:** Baseline ist die zitierte Handover-Messung, nicht am
  Einführungs-SHA erneut gemessen.
- **Braucht:** erster `ci-check`-Lauf nach Push; bei Abweichung Baseline auf den
  gemessenen Wert setzen; wer einen offenen Punkt legitim fallen lässt, bumpt
  die Baseline im annehmenden Commit.

## Punkt 4 — confirmation-Test nach grünem Screen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null` bleibt
  aus dem Teststep; sie assertet die Stark-Paar-Erkennung (`observed.te > p99`),
  die am aktuellen Schätzer rot ist.
- **Blockade:** grüner `family_fn_gate`.
- **Braucht:** nach grünem Lauf `confirmation_…` in die Workflow-Zeile
  (`.github/workflows/hyperscanning-te.yml`).

## Punkt 5/6 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen `family_fn_gate`.
- **Blockade:** grüner Screen.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Riss (getragen, nicht geglättet)

Der Rat trägt den Riss als **Vorhersage-Paar** bis der Diskriminator läuft:
Mountain/River — weißer Boden ≈ Surrogat-Boden (strukturell); Mycelium/Sensory —
weißer Boden < Surrogat-Boden, mindestens im KDE-Arm (Naht). Future trägt die
Verdikt-Regel (Arithmetik, nicht Abstimmung). Löst sich der Riss in zwei
gemessene Zahlen auf, tragen beide Linien ihren gemessenen Anteil.

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path (WGSL-KSG-Spiegel) — `operator-gebunden` (entscheid-Post).
- Cookie-Editor-Export — `wartend`/`operator` (Host fehlt).
- Flyby-Path-2-Kette — `termin:2026-09-28`.
- NSE/Haug — `wartend`/`dritter` (Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04`.
- Buster-Store-„Updated"-Datum — `wartend`/`operator` (CWS nur im echten Browser).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. Diskriminator-Verdikt | wartend | eigen | gebaut, CI ausstehend | CI-Lauf | `ci_manage view` einmal + Verdikt-Regel |
| 2. `te-gate` n=1000 | wartend | eigen | `35572569205` pending | Run-Abschluss | `ci_manage view 35572569205` |
| 3. `--dropped` Baseline | wartend | eigen | Gate gebaut, Baseline zitiert | erster Lauf | `ci-check`-Lauf, Baseline-Bump-Disziplin |
| 4. confirmation-Test | wartend | eigen | rot am Schätzer | grüner Screen | Workflow-Zeile |
| 5. F3/F4 | wartend | eigen | — | grüner Screen | getrennte Läufe |
| 6. Takens-Wandzeit | wartend | eigen | — | grüner Screen | Wandzeit im Lauf |
| 7. Riss 4 Ksg (WGSL) | operator-gebunden | operator | — | Operator | entscheid-Post |
| 8. Cookie-Editor | wartend | operator | Host fehlt | Operator | Host nennen |
| 9. Flyby-Path-2 | termin:2026-09-28 | termin | — | Datum | Zellen ab Perigäum |
| 10. NSE/Haug | wartend | dritter | — | Dateieingang | Trigger |
| 11. BepiColombo MORE | termin:2027-04 | termin | — | Freigabe | Wissenschaftsphase |
| 12. Buster-Store | wartend | operator | — | echter Browser | im Browser lesen |

## Benchmark

- **Rat (pro/max, `council`) + research-max (pro/max):** Diagnose Skalen-Kollaps
  (Kategorie-Fehler der Invarianz, Sheet-Geometrie) + Diskriminator-Design
  (drei Arme + KDE + τ-Sweep) + Rücknahme-Entscheid — Architektur/Diagnose, kein
  flash-Doppellauf (harte Atomklasse). research-max und Rat unabhängig konvergent:
  der Boden ist Schätzer-Struktur, nicht Skala.
- **grind-max (pro/max):** Diskriminator-Bau (novel Estimator-Test) — hartes Atom.
- **grind-flash (flash):** `--dropped`-Delta-Gate (Routine-Bau, Rat-Verdikt lag) —
  flash-first, kein pro-Doppellauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (Rücknahme `te.rs:2213–2240` + Kalibrier-Gate)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (`self_null_discriminator`)
- `.github/workflows/hyperscanning-te.yml` (Teststep)
- `tools/register/src/bin/register_lookup.rs` (`--dropped --count`)
- `.github/workflows/ci-check.yml` (Job `dropped-gate`)
- `docs/zustand/dropped-baseline.md` (neu)
- `docs/zustand/external-state.md` (CI-Zeile + TE-Gate-Zeile)
- `docs/handover/handover-2026-09-21-forschung-folge129.md` (neu)
- Move `handover-2026-09-21-forschung-folge128.md` → `archiv/` (eigene Linie, atomar)

Fremde uncommittete Arbeit im selben Baum (entscheid folge76, ernte folge129,
bau folge120, hfrnet/glm-l2) wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
