<!--
  title: Handover — Forschung-Folge 128 (Stand 2026-09-21)
  session: Forschung-Folge 128
  class: handover
  date: 2026-09-21
  sha256: bdc125613eda6acce2bebf3c3e9311fa73f3cd88005276fbfb3e9e68205cf797
  status: live
-->
# Handover — Forschung-Folge 128 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt seinen nächsten Schritt in derselben Zeile. Wartestellungen
nennen ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 128)

- **HEAD** `da03beb4` (== `origin/main`) bei Session-Beginn.
- **Postfach** — kein neuer Eingang; letzter Ledger-Eingang `1789973288`
  (Tuxedo-decline). Eintrag `docs/zustand/external-state.md` zitiert.
- **CI** — Watchdog-Snapshot 08:02:55 + `ci_manage view`: `hyperscanning-te`
  `35570480672` @`ca7aa66d` **failure**; `te-gate` `35570482875` @`ca7aa66d`
  pending; `ci-check` `35570485286` @`ca7aa66d` cancelled; `ps1-cdn`/`health-check`
  in_progress.

## Punkt 1 — KSG-Verdikt gemessen: Skalen-Kollaps, Skalen-Reparatur gebaut

**Verdikt gemessen** (`hyperscanning-te` `35570480672` @`ca7aa66d`):
`coherent_null_fp_gate` ok, `family_fp_gate` ok, `family_fn_gate` **FAILED**:
`TE 9.4083e-3 | null mean 1.0335e-1 sd 3.1499e-2 p95 1.4751e-1 | excess
-9.3939e-2 (-2.98 sd) | fam-max 3.6708e-1`. Reihe der Null-Mittel: KDE `fbd0f153`
−3.30 sd → Naht `16dd020c` 2.2945e-1 −2.02 sd → τ-Einfrier `f0ca4026` 2.3748e-1
−2.69 sd → KSG `ca7aa66d` 1.0335e-1 −2.98 sd. Die Prognose „KSG → Null ~0.00,
Excess positiv" ist **widerlegt**; der Excess bleibt negativ.

**Verdikt (Rat einstimmig + research-max konvergent):** der Kollaps ist der
`−1/k`-Sättigungsterm — der kleinere Partnerblock (d ~0.35 gegen c ~1.0) ertrinkt
in der gemeinsamen Tschebyschow-Norm (`te.rs:2206–2212` roh verkettet), `n_xy`
sättigt bei k=4, `ψ(k)−⟨ψ(n_xy+1)⟩ ≈ −1/k = −0.25`. Der ~0.10-Rest-Boden ist
KSG-Eigenbias (Digamma kleiner Randzählungen, k=4 in 7D). Formel/Vorzeichen/
Selbst-Punkt-Heilung gegen den skalaren KSG (`te.rs:503`) sind vollständig.

**Gebaut (Atom folge128):** `src/mathematikerin/te.rs`
`transfer_entropy_embedded_ksg` — per-Koordinate-Standardisierung der
Mischraum-Punktwolke vor der Tschebyschow-Metrik (jede Spalte auf Mittel 0, sd 1;
var ≤ 0 → `None`, absent statt fabriziert). MI ist unter invertiblen
per-Koordinate-Abbildungen invariant; der Schätzer wird skalen-invariant, der
kleinere Block ertrinkt nicht mehr. `cargo check -p omegaflow --all-targets` +
`-p omegaflow-measure --all-targets`: 0/0.

**Offen:** CI-Verdikt der Reparatur. Nach Push `hyperscanning-te` + `te-gate`
@neuer SHA dispatcht; erwartet: Excess ≥ 0 (TE kehrt von 9.4e-3 auf ~0.1–0.26
zurück, Null ≤ 0.10). (Schritt: `ci_manage view <id>` einmal nach Push.)

## Punkt 2 — `te-gate` n=1000-FPR-Boden

`te-gate` `35570482875` @`ca7aa66d` pending — der n=1000-FPR-Boden bleibt
ungemessen. Eintrag `docs/zustand/external-state.md` (TE-Gate n=1000). (Schritt:
`ci_manage view 35570482875` einmal.)

## Punkt 3 — nominees-Tests: Entscheidung + Workflow-Zeile (gebaut)

`nominees_round_trip` (Parser-Roundtrip, deterministisch, schätzer-unabhängig)
läuft jetzt im gefilterten Teststep (`.github/workflows/hyperscanning-te.yml:41`).
`confirmation_confirms_the_strong_pair_against_its_own_null` bleibt draußen: sie
assertet die Stark-Paar-Erkennung des Schätzers (`observed.te > p99`), die gerade
rot ist. Trigger: nach grünem `family_fn_gate` aufnehmen. (Schritt: nach grünem
Lauf `confirmation_…` in die Zeile.)

## Punkt 4 — `--dropped` als CI-Gate

Rat-Verdikt: Gate ja, als **Delta-Gate mit gemessener Baseline** (Schwelle null;
nie ein Absolutwert-Gate auf dem Rücken der Vergangenheit). 435 pairs, 3317
candidates, 1760 dropped, 164 commit-resolved. (Schritt: Baseline-Register +
Workflow-Bedingung bauen — `register_lookup --dropped`.)

## Punkt 5/6 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

Beide `wartend` auf grünen `family_fn_gate`. (Schritt: nach grünem Lauf Wandzeit
lesen; F3/F4 getrennt fahren.)

## Punkt 7 — Eigen-Historie Konditionierer (gemessen stale)

Gemessen: die Eigen-Historie des Ziels ist im eingebetteten Schätzer bereits der
Konditionierer (`te.rs:2250`, `n_xx = sx` = eigene Vergangenheit; `n_x = fut ∧ sx`);
der Rat hat einen separaten LaggedCond-Bau im Binning-Pfad descoped und die
Eigen-Historie in die Einbettung gefaltet (folge97:160–162). Der Punkt ist für den
Screen-Pfad stale — geschlossen mit dieser Messung.

## Riss (getragen, nicht geglättet)

Der Rat trägt einen Riss: Mountain/River lesen den negativen Excess aller vier
Ären als strukturell (Skalierung/Konditionierung); Mycelium/Sensory/Future halten
offen, dass die KDE-Ära einen Naht-Anteil trug. Der Selbst-Null-Diskriminator
(weißes Rauschen als Treiber — bleibt der Boden?) entscheidet; er ist als nächster
Messschritt benannt, nicht gebaut.

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path (WGSL-KSG-Spiegel) — `operator-gebunden` (entscheid-Post).
- Cookie-Editor-Export — `wartend`/`operator` (Host fehlt).
- Flyby-Path-2-Kette — `termin:2026-09-28`.
- NSE/Haug — `wartend`/`dritter` (Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04`.
- Buster-Store-„Updated"-Datum — `wartend`/`operator` (CWS nur im echten Browser).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. Skalen-Reparatur-Verdikt | wartend | eigen | `ci_manage view <id>` einmal nach Push |
| 2. `te-gate` n=1000-FPR | wartend | eigen | `ci_manage view 35570482875` einmal |
| 3. confirmation-Test nach grünem Screen | wartend | eigen | Workflow-Zeile ergänzen |
| 4. `--dropped` Delta-Gate bauen | wartend | eigen | Baseline-Register + Workflow-Bedingung |
| 5. Frontalkanäle F3/F4 | wartend | eigen | getrennte Läufe nach grünem Screen |
| 6. Takens-Wandzeit + Watchdog-Floor | wartend | eigen | Wandzeit im grünen Lauf |
| 7. Riss 4 Ksg off-path (WGSL) | operator-gebunden | operator | entscheid-Post (verdrahten/descopen) |
| 8. Cookie-Editor-Export | wartend | operator | Operator nennt Host |
| 9. Flyby-Path-2 | termin:2026-09-28 | termin | Zellen ab Perigäum |
| 10. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 11. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |
| 12. Buster-Store-„Updated"-Datum | wartend | operator | im echten Browser lesen |

## Benchmark

- **Rat (pro/max, `council`) + research-max (pro/max):** KSG-Rest-Boden-Verdikt
  (Skalen-Kollaps, `−1/k`-Sättigung) + `--dropped`-Gate-Form — Architektur/
  Konstruktion, kein flash-Doppellauf. Beide unabhängig konvergent auf denselben
  Mechanismus — die Gegenprobe war der Wert.
- **Session (build/flash):** Skalen-Reparatur + Workflow-Zeile + Handover —
  Routine-Bau.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (per-Koordinate-Standardisierung in
  `transfer_entropy_embedded_ksg`)
- `.github/workflows/hyperscanning-te.yml` (Teststep + `nominees_round_trip`)
- `docs/handover/handover-2026-09-21-forschung-folge128.md` (neu)
- Move `handover-2026-09-21-forschung-folge127.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Zeile)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
