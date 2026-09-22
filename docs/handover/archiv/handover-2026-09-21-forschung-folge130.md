<!--
  title: Handover — Forschung-Folge 130 (Stand 2026-09-21)
  session: Forschung-Folge 130
  class: handover
  date: 2026-09-21
  sha256: 105fce7c70487c22c488029b863515613eb18c6579bc710c8994842378a67eb4
  status: live
-->
# Handover — Forschung-Folge 130 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 130)

- **HEAD** `2ae4978a` (ernte folge129) == `origin/main`, **grün**:
  `cargo check --lib -p omegaflow` OK. Der rote Vorgänger-HEAD `fb363dda` (bau
  folge120 `0475d67f` committete `hfrnet_rtv`-Referenzen ohne `mod.rs`/
  `hfrnet_rtv.rs`, E0433) ist durch ernte folge129 geschlossen; der Post an ernte
  wurde gefaltet und gelöscht.
- **CI** — `hyperscanning-te` `35578254642` @`2ae4978a` queued (neu dispatcht),
  `te-gate` `35578257445` @`2ae4978a` queued (neu dispatcht), `ci-check`
  `35577602067` @`2ae4978a` pending (Auto-Push). Kein Poll.

## Punkt 1 — Selbst-Null-Diskriminator: Verdikt (CI)

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der rote HEAD blockierte den Verdikt-Lauf (`35577125227` @`fb363dda`
  scheiterte an `E0433 hfrnet_rtv`, kein Testlauf). Am grünen HEAD `2ae4978a` neu
  dispatcht: `35578254642` queued; der Step fährt
  `family_fn_gate family_fp_gate coherent_null_fp_gate self_null_discriminator
  nominees_round_trip`.
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578254642` **einmal**; μ_W1/μ_W2/μ_W3/μ_K lesen
  und die Verdikt-Regel anwenden: `|μ_W1 − μ_S| ≤ 2σ_W1` → struktureller Boden
  (Mountain/River; nächstes Atom Bias-Kontrolle); `μ_S − μ_W1 > 2σ_W1` → Naht
  (Mycelium/Sensory; nächstes Atom Null-Konstruktion,
  `randomized_triad:262`/`coherent_phase_surrogates te.rs:1609`).

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Am grünen HEAD `2ae4978a` neu dispatcht: `35578257445` queued (misst
  `shift_sweep_n1000` + `gate_fpr_autocorrelation` am zurückgenommenen Schätzer).
  Der Alt-Lauf `35572569205` @`a70d20c7` blieb Geister-pending.
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35578257445` einmal.

## Punkt 3 — `--dropped` Delta-Gate: Baseline-Disziplin

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Gate gebaut (`register_lookup --dropped --count`), Baseline
  `docs/zustand/dropped-baseline.md` (`dropped 1760` @`ae822fe7`); `ci-check`
  `35577602067` @`2ae4978a` pending (Auto-Push) — trägt den ersten `dropped-gate`-Lauf.
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35577602067` einmal; bei `current > baseline` Baseline
  auf den gemessenen Wert setzen.

## Punkt 4 — confirmation-Test nach grünem Screen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null` bleibt aus
  dem Teststep; sie assertet `observed.te > p99`, am aktuellen Schätzer rot.
- **Blockade:** grüner `family_fn_gate` (Lauf `35578254642`).
- **Braucht:** nach grünem Lauf `confirmation_…` in `.github/workflows/hyperscanning-te.yml`.

## Punkt 5/6 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen `family_fn_gate`.
- **Blockade:** grüner Screen.
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Riss (getragen, nicht geglättet)

Der Rat trägt den Riss als **Vorhersage-Paar** bis der Diskriminator läuft:
Mountain/River — weißer Boden ≈ Surrogat-Boden (strukturell); Mycelium/Sensory —
weißer Boden < Surrogat-Boden, mindestens im KDE-Arm (Naht). Future trägt die
Verdikt-Regel (Arithmetik, nicht Abstimmung). Löst sich der Riss in zwei gemessene
Zahlen auf, tragen beide Linien ihren gemessenen Anteil.

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
| 1. Diskriminator-Verdikt | wartend | eigen | `35578254642` @`2ae4978a` queued | Run-Abschluss | `ci_manage view` einmal + Verdikt-Regel |
| 2. `te-gate` n=1000 | wartend | eigen | `35578257445` @`2ae4978a` queued | Run-Abschluss | `ci_manage view` einmal |
| 3. `--dropped` Baseline | wartend | eigen | `ci-check` `35577602067` @`2ae4978a` pending | Run-Abschluss | `ci_manage view` einmal, Baseline-Bump |
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

- Keine Dispatch-Klasse: Diagnose des roten HEAD (`git log -S`/`git show`/`cargo
  check`) + Re-Dispatch der zwei Workflows, flash-Level. Kein pro/max-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge130.md` (neu)
- Move `handover-2026-09-21-forschung-folge129.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Zeile @`2ae4978a`)
- `docs/handover/post.md` — unverändert (der ernte-Post wurde vom Empfänger gefaltet
  und gelöscht; kein eigener Hunk)

Fremde uncommittete Arbeit im selben Baum (entscheid folge76, bau folge120) wird
**nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
