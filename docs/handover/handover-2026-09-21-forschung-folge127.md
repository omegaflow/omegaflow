<!--
  title: Handover — Forschung-Folge 127 (Stand 2026-09-21)
  session: Forschung-Folge 127
  class: handover
  date: 2026-09-21
  sha256: a538f97d9827e00f04af82737132b763715a6cee8c4ec8d7a78729a4a38f30f0
  status: live
-->
# Handover — Forschung-Folge 127 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt seinen nächsten Schritt in derselben Zeile. Wartestellungen
nennen ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 127)

- **HEAD** `4a0c7803` (== `origin/main`, Bau-Folge 116) bei Session-Beginn.
- **Postfach** — zwei neue Eingänge: `1789970277` (Framework, Ticket NG2HWBZM:
  kein Programm für Einzel-Sponsoring) und `1789973288` (Tuxedo, Ticket#991311279:
  keine kostenlosen Geräte/Sponsorings an Privatpersonen). **Der Hardware-Punkt
  ist damit beidseitig `declined` — geschlossen.** Letzter Ledger-Eingang
  `1789973288`.
- **CI** — Watchdog-Snapshot 08:02:55 + `ci_manage`: `hyperscanning-te`
  `35567708611` @`f0ca4026` **failure**; `te-gate` `35567711055` @`f0ca4026`
  in_progress (seit 06:15); `ci-check` `35566258372` @`0fd1c5a9` in_progress
  (hängt seit 05:53), `35567752434` pending @06:16; `ps1-cdn`/`health-check`
  in_progress.

## Punkt 1 — Riss-Reparatur: τ-Einfrier widerlegt, KSG-1 gebaut

**Verdikt gemessen** (`hyperscanning-te` `35567708611` @`f0ca4026`):
`coherent_null_fp_gate` **ok**, `family_fp_gate` **ok**, `family_fn_gate`
**FAILED**: `TE 8.7004e-2 | null mean 2.3748e-1 sd 5.5882e-2 p95 3.2459e-1 |
excess -1.5047e-1 (-2.69 sd) | fam-max 5.5775e-1`. Reihe: `fbd0f153` −3.30 sd →
`16dd020c` (Naht-Fix) null 2.2945e-1, −2.02 sd → `f0ca4026` (τ-Einfrier) null
2.3748e-1, −2.69 sd. Die Prognose „die eingefrorene Null fällt auf den
Schätzer-Boden 5–9e-2" ist **widerlegt** — die Null bleibt ~0.23 unabhängig von
der τ-Behandlung.

**Rat-Verdikt (einstimmig):** KDE-Boden dominiert (τ-Mischung widerlegt, Naht-Fix
echter, aber unzureichender Bias-Anteil). Nächster Schritt: **KSG-1-Mischraum-
Schätzer** auf derselben Takens-Punktwolke.

**Gebaut (Atom folge127):**
- `src/mathematikerin/te.rs`: `transfer_entropy_embedded_ksg(x, emb_x, emb_y,
  tau_x, tau_y, k)` — KSG-1, Tschebyschow, `select_nth_unstable_by`+`total_cmp`
  (seed-frei), `digamma` (vorhanden, te.rs:402); `const TE_KSG_K = 4`.
  `transfer_entropy_embedded` → `transfer_entropy_embedded_kde` (benannte
  CPU-Referenz der WGSL-Parität). `topological_te_estimate`,
  `topological_te_estimate_frozen` und der Null-Pfad `topological_te_with` laufen
  über KSG; der Instantaneous-Phase-Pfad bleibt auf KDE.
- **Session-Verifikation (A = A):** grind-max hatte die Rat-Formel
  ψ(k)+⟨ψ(n_C+1)−ψ(n_AC+1)−ψ(n_BC+1)⟩ mit vertauschten Vorzeichen der ersten zwei
  Terme und mitgezähltem Selbst-Punkt umgesetzt. Gegen den gate-bewiesenen
  skalaren KSG (`te.rs:503`) geheilt: Zählung ohne Selbst, `< eps`, Formel
  `ψ(n_xx+1) − ψ(n_x+1) − ψ(n_xy+1)`.
- `src/mathematikerin/tests.rs`: `te_gpu_crosscheck_against_cpu_reference` ruft die
  KDE-Referenz explizit (WGSL bleibt auf KDE).
- `src/mathematikerin/shaders.rs`: unverändert — der WGSL-KSG-Spiegel bleibt als
  `pending` **hier** benannt (der Gate streicht Kommentare; kein Code-Kommentar).
- `tools/measure/src/bin/te_rng_fix_probe.rs`: Aufrufer auf `_kde`; die
  `.max(1)`-Fabrikation im FP-Print entfernt (`absent` statt fabrizierter Boden).
- `cargo check -p omegaflow --all-targets` + `-p omegaflow-measure --all-targets`:
  0/0.
- Post `An forschung: Ksg off-path` eingefaltet und gelöscht: der Familien-Screen
  lief nur über `topological_te_estimate`; dieses Atom verdrahtet den KSG genau in
  diesen Pfad.

**Offen:** Verdikt des KSG — **dispatcht** `hyperscanning-te` `35570480672`
@`ca7aa66d` (post-push), erwartet `null mean ~0.00`, `excess` positiv. (Schritt:
`ci_manage view 35570480672` einmal.)

## Punkt 2 — `te-gate` n=1000-FPR-Boden

`te-gate` `35567711055` @`f0ca4026` in_progress (seit 06:15, kein Update) — der
n=1000-FPR-Boden bleibt ungemessen. Neu **dispatcht** `te-gate` `35570482875`
@`ca7aa66d` (post-push; misst die Kalibrier-Tests am KSG-Schätzer). (Schritt:
`ci_manage view 35570482875` einmal.)

## Punkt 3 — nominees-Tests in den CI-Teststep?

`nominees_round_trip` und `confirmation_confirms_the_strong_pair_against_its_own_null`
sind bewusst nicht im gefilterten Teststep (`family_fn_gate family_fp_gate`).
Entscheidung, ob sie dort laufen. (Schritt: Entscheidung + Workflow-Zeile.)

## Punkt 4 — `--dropped` als CI-Gate

435 pairs, 3317 candidates, 1760 dropped, 164 commit-resolved. (Schritt:
Rat/Architektur — Gate-Verdrahtung.)

## Punkt 5 — Frontalkanäle F3/F4

Getrennte Läufe nach grünem Screen. (Schritt: nach grünem `family_fn_gate`.)

## Punkt 6 — Takens-Wandzeit + Watchdog-Floor

Beim grünen Screen die Wandzeit lesen und den Watchdog-Floor daraus ableiten.
(Schritt: Wandzeit im grünen Lauf messen.)

## Punkt 7 — Eigen-Historie Konditionierer

`LaggedCond` auf Zielserie. (Schritt: `te.rs` verdrahten.)

## Riss (getragen, nicht geglättet)

Die Mechanismus-Frage ist gemessen entschieden: τ-Mischung **widerlegt** (τ-
Einfrier hebt die Null nicht), Naht-Fix **unzureichend**, **KDE-Boden dominiert**
(Rat einstimmig). Die Reparatur-Reihenfolge misst den nächsten Schritt: der
KSG-1-Mischraum-Schätzer ist gebaut und dispatcht. Die Assertion ist unberührt;
der Boden wird als gemessene Konstante gedruckt, nie durch Lockern versteckt.

## Wartend / operator-gebunden / termin

- Riss 4 Ksg off-path — `operator-gebunden`: als Post-Zeile `An entscheid:`
  getragen (verdrahten oder descopen).
- Cookie-Editor-Export — `wartend`/`operator` (Host fehlt).
- Flyby-Path-2-Kette — `termin:2026-09-28` (Zellen ab Perigäum füllen).
- NSE/Haug — `wartend`/`dritter` (Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe Wissenschaftsphase).
- Buster-Store-„Updated"-Datum — `wartend`/`operator`: CWS-Listing nur im echten
  Browser lesbar.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. KSG-Verdikt | wartend | eigen | `ci_manage view <id>` einmal |
| 2. `te-gate` n=1000-FPR | wartend | eigen | `ci_manage view 35567711055` einmal |
| 3. nominees-Tests in CI-Teststep? | wartend | eigen | Entscheidung + Workflow-Zeile |
| 4. `--dropped` als CI-Gate | wartend | eigen | Rat: Gate-Verdrahtung |
| 5. Frontalkanäle F3/F4 | wartend | eigen | getrennte Läufe nach grünem Screen |
| 6. Takens-Wandzeit + Watchdog-Floor | wartend | eigen | Wandzeit im grünen Lauf |
| 7. Eigen-Historie Konditionierer | wartend | eigen | `LaggedCond` auf Zielserie |
| 8. Riss 4 Ksg off-path | operator-gebunden | operator | entscheid-Post (verdrahten/descopen) |
| 9. Cookie-Editor-Export | wartend | operator | Operator nennt Host |
| 10. Flyby-Path-2 | termin:2026-09-28 | termin | Zellen ab Perigäum |
| 11. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 12. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |
| 13. Buster-Store-„Updated"-Datum | wartend | operator | im echten Browser lesen |

## Benchmark

- **KSG-Bau (hartes Atom, TE-/Null-Konstruktion):** `grind-max` (pro/max) baute
  den Schätzer nach Rat-Route; die Session-Verifikation (build/flash) fand den
  Vorzeichen-/Selbst-Punkt-Fehler und heilte ihn gegen den gate-bewiesenen
  skalaren KSG. Klasse: hartes Atom — kein flash-Doppellauf; die Lehre: der
  Rat-Route folgt die Gegenprüfung gegen den Referenzpfad.
- **Rat (pro/max, Architektur):** KDE-Boden-Verdikt + KSG-Bauplan; Architektur-
  Klasse, kein Benchmark-Doppel.
- **fmt/CI-Lesen + Zustand (Routine):** Session-Kontext (build/flash).

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (`transfer_entropy_embedded_ksg` + `_kde`-Umbenennung
  + Verdrahtung)
- `src/mathematikerin/tests.rs` (`te_gpu_crosscheck_against_cpu_reference`)
- `tools/measure/src/bin/te_rng_fix_probe.rs` (Aufrufer `_kde` + `.max(1)` geheilt)
- `docs/handover/handover-2026-09-21-forschung-folge127.md` (neu)
- Move `handover-2026-09-21-forschung-folge126.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (Postfach-Zeile + CI-Zeile)
- `docs/handover/post.md` (die an forschung gerichtete fmt-Zeile gelöscht —
  gemessen erledigt: beide genannten Dateien stehen bereits in rustfmt-Form)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.

Commit `ca7aa66d` gepusht (== `origin/main`); dispatcht (post-push):
`hyperscanning-te` `35570480672`, `te-gate` `35570482875`, `ci-check`
`35570485286` — je `ci_manage view <id>` einmal, nie pollen.
