<!--
  title: Handover — Forschung-Folge 132 (Stand 2026-09-21)
  session: Forschung-Folge 132
  class: handover
  date: 2026-09-21
  sha256: 86c3e9172a42353d7d853a1637e8cc3b942739641abc7e902321a10eefb3b70c
  status: live
-->
# Handover — Forschung-Folge 132 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 132)

- **HEAD** `d78c95cc` == `origin/main`; Arbeitsbaum trägt fremde uncommittete
  Arbeit: der `upload_asset`→`upload_release(tag, path)`-Umbau (~76 Dateien) und
  ein fremder Nachtrag in `docs/surveys/survey-funding-erkundung.md` — nicht
  angefasst, nicht committet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", 100 % von $5.00 free credits, informativ); kein neuer Eingang,
  kein handlungsbedürftiger Fall.
- **CI** — `hyperscanning-te` `35584758519` @`c7cb201f` **failure**: 5/6 Tests
  grün, nur `family_fn_gate` rot (die bekannte FN); **`fn_gate_sweep` gelaufen
  und grün**, die Response-Tafel gedruckt (Verdikt s. Punkt 1). `te-gate`
  `35578257445` @`2ae4978a` **pending** (Ghost-Verdacht); `ci-check`
  `35578412609` @`a476ccfb` **failure** (dropped-gate: baseline 1760 | current
  1819 | delta 59); nach dem Push dieser Session: `ci-check` `35585642642`
  @`e9556f86` pending + `tools-build` `35585642669` queued.

## Punkt 1 — `family_fn_gate`: Gate/Fixture ehrlich schneiden (Sweep-Verdikt gefallen)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Sweep `35584758519` @`c7cb201f` ist gelandet: **5/6 Tests grün**,
  nur `family_fn_gate` rot (die bekannte FN, Assert
  `tools/measure/src/bin/hyperscanning_group_te.rs:1056`); `fn_gate_sweep` selbst
  grün. Die Response-Tafel (KSG/KDE × τ∈{mi,2,1}; n∈{600,1200,2400} bei s=0.35;
  s∈{0.05…0.5} bei n=1200; plus stochastic-driver-Arm):
  - **stochastic-driver** (n=600, s=0.35) **KSG τ=2: excess +18.2 sd** (TE
    3.51e-1, null_mu 1.82e-2) → der KSG-Schätzer findet den vollrangigen
    AR(1)-Treiber. **Kein Schätzer-Bug.**
  - Sheet-Arme (KSG) negativ über alle n/s: mi −2.55/−1.51/−2.75,
    τ=2 −3.18/−1.91/−2.46, τ=1 −1.11/+0.18/−0.17 → TE≈0 ist die physikalisch
    wahre Antwort der quasi-deterministischen Sheet-Fixture.
  - **Konfundierung:** KDE τ=1 zeigt einen fremden positiven Excess
    (n-sweep +1.04/+1.31/+3.30; s-sweep +0.17…+1.41) — der KDE-Pfad bei τ=1,
    nicht der kanonische KSG (`src/mathematikerin/te.rs`); der KDE-stochastic-Arm
    ist wegen des großen KDE-Nulls (0.78) blind (excess −1.2).
  - **Verdikt nach der Regel** (stochastischer Arm grün + Sheet-Arme rot):
    **Fix = Fixture/Gate**, nicht Schätzer.
- **Blockade:** keine.
- **Braucht:** Fix A — den Assert `hyperscanning_group_te.rs:1056` auf die
  gemessene Wahrheit schreiben (die Sheet-Fixture trägt keine bedingte
  Transfer-Entropie) — oder Fix B — den stochastischen Treiber ins Gate nehmen;
  den KDE-τ=1-Ausreißer als Konfundierung benennen.

## Punkt 2 — Diskriminator-μ-Werte (W1/W2/W3/K1/K2/K3) — gemessen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** gemessen `35584758519` @`c7cb201f`. μ_S (KSG per-cell, aus
  `family_fn_gate`) = 1.0335e-1. **KSG:** W1 driver-white −1.4616e-2
  (sd 5.175e-3, p95 −6.005e-3), W2 both-white +1.278e-3, W3 target-white
  +3.335e-3. **KDE:** K1 1.1134e-1 (sd 5.430e-3), K2 1.0702e0, K3 6.3224e-1.
  **Riss-Regel:** μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht** (der
  Weiß-Treiber-Null liegt weit unter dem Sheet-Null).
- **Blockade:** keine.
- **Braucht:** die Naht-Aussage tragen (Rat bei der Formulierung); die
  μ-Zeilen sind die gemessenen Zahlen des aufgelösten Risses.

## Punkt 3 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35578257445` @`2ae4978a` **pending** (Ghost-Verdacht wie der
  Vorgänger).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578257445` einmal.

## Punkt 4 — `--dropped` Delta-Gate: Klassifikator statt Baseline-Bump

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `ci-check` `35578412609` @`a476ccfb` **failure**: `dropped-gate:
  baseline 1760 | current 1819 | delta 59`. Lokal (HEAD `d78c95cc`) ist `current
  = 1864`. Die Delta ist von **umnummerierten/umformulierten Tafel-Zeilen**
  dominiert — `register_lookup --dropped --persist 2` meldet forschungs-eigene
  Zeilen als `DROPPED`, obwohl derselbe Punkt weitergetragen wird (z. B.
  `folge130:110` „7. Riss 4 Ksg (WGSL)" → folge131 trägt „8. Riss 4 Ksg
  (WGSL)"). Ursache gemessen: der Schlüssel ist `match_prefix(point_key_tokens)`
  — die führende Aufzählungsnummer sitzt im Schlüssel
  (`tools/register/src/bin/register_lookup.rs:1621`), sodass jede Umnummerierung
  als Drop zählt. Ein Baseline-Bump würde damit echte Drops maskieren.
- **Blockade:** der Klassifikator kann lokal nicht gemessen werden (das Werkzeug
  ist vorgebaut; funktionale Läufe nur in CI).
- **Braucht:** der Fix ist **gebaut (uncommittet)**: `point_key_tokens` verwirft
  führende reine Zahl-Token (`is_enumeration_token`), plus Test
  `point_key_ignores_a_leading_enumeration_number`; `cargo check -p
  omegaflow-register --all-targets` 0/0. → `ci-check` am neuen HEAD misst den
  korrigierten Zähler; danach die Baseline auf den **gemessenen** Wert setzen
  (in dem Commit, der den legitimen Drop trägt).

## Punkt 5 — confirmation-Test nach grünem Screen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null` bleibt aus
  dem Teststep; sie assertet `observed.te > p99`, am aktuellen Schätzer rot.
- **Blockade:** grüner `family_fn_gate` (Punkt 1).
- **Braucht:** nach grünem Lauf `confirmation_…` in
  `.github/workflows/hyperscanning-te.yml` aufnehmen.

## Punkt 6 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen.
- **Blockade:** grüner Screen.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Riss (aufgelöst in gemessene Zahlen)

Der Sweep ist gelaufen; der Riss löst sich in zwei gemessene Zahlen auf
(Punkt 2): μ_S = 1.0335e-1 (Sheet-Null, KSG) gegen μ_W1 = −1.4616e-2
(Weiß-Treiber-Null, KSG), 2σ_W1 = 0.0104 → μ_S − μ_W1 = 0.1180 > 2σ_W1 →
**Naht** (Mycelium/Sensory: der Weiß-Treiber-Boden liegt unter dem Sheet-Null).
Mountain/River (weißer Boden ≈ Surrogat-Boden) ist damit nicht bestätigt.
Der stochastic-driver-Arm (KSG τ=2, excess +18.2 sd) trägt den Schätzer-Befund
getrennt: der rote `family_fn_gate`-Assert ist eine Fixture-/Gate-Frage, kein
Schätzer-Boden (Punkt 1).

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path (WGSL-KSG-Spiegel) — `operator-gebunden` — jetzt als
  Post-Zeile `An entscheid:` in `post.md` gelegt (Forschung-Folge 132).
- Cookie-Editor-Export — `wartend`/`operator` (kein Host benannt).
- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).
- Buster-Store-„Updated"-Datum — **erledigt**: gemessen 2026-09-20
  (`docs/surveys/survey-2026-09-20-browser-anbindung.md:95-97`: v3.4.0,
  20. Juni 2026); aus dem Register gestrichen.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. Gate/Fixture schneiden | wartend | eigen | Verdikt: Fix = Fixture/Gate | keine | Assert `:1056` / stochastischer Treiber |
| 2. Diskriminator-μ | wartend | eigen | Naht (μ_S−μ_W1 > 2σ_W1) | keine | Naht tragen (Rat) |
| 3. `te-gate` n=1000 | wartend | eigen | `35578257445` pending (Ghost) | Run-Abschluss | `ci_manage view` einmal |
| 4. `--dropped` Klassifikator | wartend | eigen | Fix gebaut (committet) | CI-Messung | `ci-check`-Zähler → Baseline |
| 5. confirmation-Test | wartend | eigen | rot am Schätzer | grüner Screen | Workflow-Zeile |
| 6. F3/F4 + Takens | wartend | eigen | — | grüner Screen | getrennte Läufe |
| 7. Riss 4 Ksg (WGSL) | operator-gebunden | operator | Post-Zeile gelegt | Operator | entscheid-Antwort |
| 8. Cookie-Editor | wartend | operator | kein Host benannt | Operator | Host nennen |
| 9. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 10. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 11. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- `grind-flash` für den `--dropped`-Klassifikator-Fix (Routine-Edit + Test) —
  kein Doppellauf; Rat/research-max nicht neu dispatcht (die FN-Diagnose steht
  seit Folge 131).

## Geteilter Baum — eigener Pfad-Satz

- `tools/register/src/bin/register_lookup.rs` (Klassifikator-Fix, grind-flash)
- `docs/handover/handover-2026-09-21-forschung-folge132.md` (neu)
- Move `handover-2026-09-21-forschung-folge131.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/handover/post.md` (`An entscheid:`-Zeile)
- `docs/zustand/external-state.md` (Postfach-/CI-/TE-Gate-Zeile)

Fremde uncommittete Arbeit im selben Baum (`upload_asset`→`upload_release`-Umbau,
`survey-funding-erkundung.md`-Nachtrag) wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
