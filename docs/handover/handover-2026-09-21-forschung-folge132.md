<!--
  title: Handover — Forschung-Folge 132 (Stand 2026-09-21)
  session: Forschung-Folge 132
  class: handover
  date: 2026-09-21
  sha256: a275ce653279f5511c98c130b73236d5dcfcf69256cac90948b684e79f77bd2d
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
- **CI** — `hyperscanning-te` `35584758519` @`c7cb201f` **in_progress** (trägt
  `fn_gate_sweep` + `--nocapture`; `updated_at` seit `09:42:19` unverändert —
  Ghost-Verdacht wie `te-gate`); `te-gate` `35578257445` @`2ae4978a` **pending**
  (Ghost-Verdacht); `ci-check` `35578412609` @`a476ccfb` **failure** (dropped-gate:
  baseline 1760 | current 1819 | delta 59); `ci-check` `35584795196` @`d78c95cc`
  in_progress; `tools-build` `35584750291` success; `hfrnet-cdn` `35581429287`
  success.

## Punkt 1 — `family_fn_gate` FN: Response-Kurve messen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Sweep-Test `fn_gate_sweep` ist gebaut und committet (`c7cb201f`);
  der Lauf `35584758519` @`c7cb201f` steht auf **in_progress** (seit `09:42:19`
  ohne Fortschritt — Ghost-Verdacht). Der FN selbst ist diagnostiziert: die
  Fixture C→D legt die 7-dim Joint-Wolke auf ein 6-dim Sheet (`d_eff=6`), TE≈0
  ist die physikalisch wahre Antwort; die per-cell-Null trägt nur den
  KSG-Eigenbias.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage log 35584758519` **einmal**, sobald der Lauf beendet ist
  → Verdikt-Regel (stochastischer Arm grün + Sheet-Arme rot → Fix = Fixture/Gate;
  τ-fest-2 grün → Fix = τ-Ausrichtung; Null-Mittel n=2400 unter dem beobachteten
  TE → n-Floor; stochastischer Arm rot → Fix = Schätzer).

## Punkt 2 — `family_fn_gate`: Gate/Fixture ehrlich schneiden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** hängt am Sweep-Verdikt; danach Fix A (Assert `hyperscanning_group_te.rs:1035`
  auf die gemessene Wahrheit) oder Fix B (stochastischer Treiber ins Gate).
- **Blockade:** Sweep-Verdikt (Punkt 1).
- **Braucht:** Punkt 1.

## Punkt 3 — Diskriminator-μ-Werte (W1/W2/W3/K1/K2/K3)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `self_null_discriminator` grün; die Workflow-Zeile trägt jetzt
  `--nocapture` (committet `c7cb201f`). μ_S = 1.0335e-1 (aus dem
  `family_fn_gate`-Druck).
- **Blockade:** Sweep-Lauf.
- **Braucht:** `ci_manage log` aus Punkt 1; Riss-Regel anwenden
  (`|μ_W1 − μ_S| ≤ 2σ_W1` → struktureller Boden; `μ_S − μ_W1 > 2σ_W1` → Naht).

## Punkt 4 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35578257445` @`2ae4978a` **pending** (Ghost-Verdacht wie der
  Vorgänger).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578257445` einmal.

## Punkt 5 — `--dropped` Delta-Gate: Klassifikator statt Baseline-Bump

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

Der Rat trägt den Riss als **Vorhersage-Paar** bis der Sweep läuft:
Mountain/River — weißer Boden ≈ Surrogat-Boden (struktureller Boden, Sheet);
Mycelium/Sensory — weißer Boden < Surrogat-Boden, mindestens im KDE-Arm (Naht).
Die Diagnose stützt den strukturellen Boden, aber die `self_null_discriminator`-μ-Zeilen
sind noch nicht gelesen (Punkt 1/3). Löst sich der Riss in zwei gemessene Zahlen
auf, tragen beide Linien ihren gemessenen Anteil.

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
| 1. `fn_gate_sweep` Kurve | wartend | eigen | Lauf `35584758519` in_progress (Ghost) | Lauf-Abschluss | `ci_manage log` einmal |
| 2. Gate/Fixture schneiden | wartend | eigen | hängt am Verdikt | Sweep-Verdikt | Punkt 1 |
| 3. Diskriminator-μ | wartend | eigen | `--nocapture` committet | Sweep-Lauf | `ci_manage log` aus P1 |
| 4. `te-gate` n=1000 | wartend | eigen | `35578257445` pending (Ghost) | Run-Abschluss | `ci_manage view` einmal |
| 5. `--dropped` Klassifikator | wartend | eigen | Fix gebaut (uncommittet) | CI-Messung | `ci-check`-Zähler → Baseline |
| 6. confirmation-Test | wartend | eigen | rot am Schätzer | grüner Screen | Workflow-Zeile |
| 7. F3/F4 + Takens | wartend | eigen | — | grüner Screen | getrennte Läufe |
| 8. Riss 4 Ksg (WGSL) | operator-gebunden | operator | Post-Zeile gelegt | Operator | entscheid-Antwort |
| 9. Cookie-Editor | wartend | operator | kein Host benannt | Operator | Host nennen |
| 10. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 11. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 12. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

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
