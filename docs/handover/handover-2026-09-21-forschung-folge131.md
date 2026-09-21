<!--
  title: Handover — Forschung-Folge 131 (Stand 2026-09-21)
  session: Forschung-Folge 131
  class: handover
  date: 2026-09-21
  sha256: a0208a7aa3ee38bb6480d35941ec68c078b19e69cf7b84d1c120dfe0e9af5a30
  status: live
-->
# Handover — Forschung-Folge 131 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 131)

- **HEAD** `a476ccfb` (forschung folge130) == `origin/main`; Arbeitsbaum zu
  Session-Beginn sauber (`git_safety --snapshot`: „working tree equals HEAD").
  **Fremde uncommittete Arbeit** einer anderen Linie liegt jetzt im Baum: der
  `upload_asset`→`upload_release(tag, path)`-Umbau (76 Dateien, ~74
  Compiler-Sites + `src/gate/commit_gate_vocab.json`) — nicht angefasst, nicht
  committet.
- **Postfach** — neuer Eingang `1789978555` (Brave Search API „usage limit
  reached", 100 % von $5.00 free credits, 2026-09-21, informativ); davor
  `1789973288` (Tuxedo Ticket#991311279: keine kostenlosen Geräte/Sponsorings an
  Privatpersonen — Hardware-Punkt beidseitig declined), `1789970277`
  (Framework-Sponsoring abgelehnt, Ticket NG2HWBZM), `1789930255` (Pine64:
  Ox64-Hardware zugesagt). Kein handlungsbedürftiger Eingang.
- **CI** — `hyperscanning-te` `35578254642` @`2ae4978a` **failure**: Teststep
  4/5 grün (`nominees_round_trip`, `coherent_null_fp_gate`, `family_fp_gate`,
  `self_null_discriminator`), **`family_fn_gate` rot** (Punkt 1); die Folgesteps
  (Manifest, Screen, Confirmation) fielen wegen fehlendem `manifest.txt`. `te-gate`
  `35578257445` @`2ae4978a` pending (Ghost-Verdacht); `ci-check` `35578412609`
  @`a476ccfb` pending. Neu dispatcht (diese Session): `hyperscanning-te`
  `35584758519` @`c7cb201f` queued (trägt `fn_gate_sweep` + `--nocapture`).

## Punkt 1 — `family_fn_gate` FN: Response-Kurve messen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `family_fn_gate` rot (`tools/measure/src/bin/hyperscanning_group_te.rs:1035`):
  TE 9.4083e-3 | per-cell-Null mean 1.0335e-1 sd 3.1499e-2 p95 1.4751e-1 |
  excess −2.98 sd | fam-max 3.6708e-1. Rat + research-max (2026-09-21): der FN
  ist **kein Schätzer-Bug** — die Fixture C→D (`d = 0.5·d[t−1] + 0.35·c[t−2]²`,
  quasi-deterministischer Sinus-Treiber) legt die 7-dim Joint-Wolke auf ein
  6-dim Sheet (`d_eff=6`), der KSG-ε-Ball ist größer als die Sheet-Dicke → TE≈0
  ist für dieses Ziel die physikalisch wahre Antwort (der Treiber fügt über die
  eigene Vergangenheit des Ziels nichts hinzu); die per-cell-Null ist korrekt
  (richtige Zelle, eingefrorene τs) und trägt nur den KSG-Eigenbias 1.03e-1. Der
  Assert behauptet eine bedingte Fähigkeit, die die Fixture-Physik nicht liefert.
  n-Sweep rettet das Gate nicht (Kollaps-Grenze n≈1.3e10). Die Mess-Instrumentierung
  ist gebaut (uncommittet): Test `fn_gate_sweep` (n∈{600,1200,2400} bei s=0.35;
  s∈{0.05,0.1,0.2,0.35,0.5} bei n=1200; Schätzer KSG/KDE × τ∈{mi,2,1}; plus
  `stochastic-driver`-Arm: AR(1)-Treiber ohne Sinus → vollrangige Wolke) — assertet
  nur Messbarkeit + Seed-Determinismus, druckt die Tafel.
- **Blockade:** die Kurve ist noch nicht gelaufen (braucht den gepushten Commit).
- **Braucht:** der Lauf ist dispatcht: `hyperscanning-te` `35584758519` @`c7cb201f`
  queued → `ci_manage log 35584758519` **einmal**. Verdikt-Regel (nicht assertet): stochastischer
  Arm grün + Sheet-Arme rot → Fix = Fixture/Gate (stochastischer Treiber bzw.
  Assert auf die gemessene Wahrheit); τ-fest-2 grün → Fix = τ-Ausrichtung im
  Schätzer-Pfad; Null-Mittel bei n=2400 unter dem beobachteten TE → n-Floor;
  stochastischer Arm auch rot → Fix = Schätzer (`src/mathematikerin/te.rs`).

## Punkt 2 — `family_fn_gate`: Gate/Fixture ehrlich schneiden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** hängt am Sweep-Verdikt; erst danach Fix A (Assert `:1035` auf die
  gemessene Wahrheit umschreiben) oder Fix B (stochastischer Treiber ins Gate).
- **Blockade:** Sweep-Verdikt (Punkt 1).
- **Braucht:** Punkt 1.

## Punkt 3 — Diskriminator-μ-Werte (W1/W2/W3/K1/K2/K3)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `self_null_discriminator` grün, aber sein `println!` war von
  `cargo test` gecaptured; die Workflow-Zeile trägt jetzt `--nocapture`
  (uncommittet). μ_S = 1.0335e-1 (aus dem `family_fn_gate`-Druck).
- **Blockade:** Sweep-Lauf.
- **Braucht:** `ci_manage log <id>` aus Punkt 1; Riss-Regel anwenden
  (`|μ_W1 − μ_S| ≤ 2σ_W1` → struktureller Boden; `μ_S − μ_W1 > 2σ_W1` → Naht).

## Punkt 4 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35578257445` @`2ae4978a` pending (Ghost-Verdacht wie der Vorgänger).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578257445` einmal.

## Punkt 5 — `--dropped` Delta-Gate: Baseline-Disziplin

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Gate gebaut (`register_lookup --dropped --count`), Baseline
  `docs/zustand/dropped-baseline.md` (`dropped 1760` @`ae822fe7`); `ci-check`
  `35578412609` @`a476ccfb` pending.
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578412609` einmal; bei `current > baseline` die
  Baseline auf den gemessenen Wert setzen.

## Punkt 6 — confirmation-Test nach grünem Screen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null` bleibt aus
  dem Teststep; sie assertet `observed.te > p99`, am aktuellen Schätzer rot.
- **Blockade:** grüner `family_fn_gate` (Punkt 1/2).
- **Braucht:** nach grünem Lauf `confirmation_…` in
  `.github/workflows/hyperscanning-te.yml` aufnehmen.

## Punkt 7 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen.
- **Blockade:** grüner Screen.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Riss (getragen, nicht geglättet)

Der Rat trägt den Riss als **Vorhersage-Paar** bis der Sweep läuft: Mountain/River
— weißer Boden ≈ Surrogat-Boden (struktureller Boden, Sheet); Mycelium/Sensory —
weißer Boden < Surrogat-Boden, mindestens im KDE-Arm (Naht). Die Diagnose stützt
den strukturellen Boden (die Null ist korrekt, der Kollaps physikalisch), aber die
`self_null_discriminator`-μ-Zeilen sind noch nicht gelesen. Löst sich der Riss in
zwei gemessene Zahlen auf, tragen beide Linien ihren gemessenen Anteil.

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
| 1. `fn_gate_sweep` Kurve | wartend | eigen | Instrumentierung gebaut (uncommittet) | Commit+Push fehlt | `/commit` → `gh workflow run` → `ci_manage log` einmal |
| 2. Gate/Fixture schneiden | wartend | eigen | hängt am Verdikt | Sweep-Verdikt | Punkt 1 |
| 3. Diskriminator-μ | wartend | eigen | `--nocapture` jetzt in Workflow | Sweep-Lauf | `ci_manage log` aus P1 |
| 4. `te-gate` n=1000 | wartend | eigen | `35578257445` pending | Run-Abschluss | `ci_manage view` einmal |
| 5. `--dropped` Baseline | wartend | eigen | `35578412609` pending | Run-Abschluss | `ci_manage view` einmal, Bump |
| 6. confirmation-Test | wartend | eigen | rot am Schätzer | grüner Screen | Workflow-Zeile |
| 7. F3/F4 + Takens | wartend | eigen | — | grüner Screen | getrennte Läufe |
| 8. Riss 4 Ksg (WGSL) | operator-gebunden | operator | — | Operator | entscheid-Post |
| 9. Cookie-Editor | wartend | operator | Host fehlt | Operator | Host nennen |
| 10. Flyby-Path-2 | termin:2026-09-28 | termin | — | Datum | Zellen ab Perigäum |
| 11. NSE/Haug | wartend | dritter | — | Dateieingang | Trigger |
| 12. BepiColombo MORE | termin:2027-04 | termin | — | Freigabe | Wissenschaftsphase |
| 13. Buster-Store | wartend | operator | — | echter Browser | im Browser lesen |

## Benchmark

- Rat (pro/max) + research-max (pro/max) für die FN-Diagnose, grind-pro für die
  Instrumentierung — kein Doppellauf, kein Flash-first-Verstoß (das Verdikt war
  das Urteil, nicht die Routine).

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (Fixture-Parametrisierung
  `fn_fixture_quad`/`quad_target`, neuer Test `fn_gate_sweep`)
- `.github/workflows/hyperscanning-te.yml` (Testname + `--nocapture`)
- `docs/handover/handover-2026-09-21-forschung-folge131.md` (neu)
- Move `handover-2026-09-21-forschung-folge130.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (Postfach-/CI-/TE-Gate-Zeile, neue Brave-Zeile)
- `docs/handover/post.md` — unverändert

Fremde uncommittete Arbeit im selben Baum (der `upload_asset`→`upload_release`-Umbau,
76 Dateien) wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
