<!--
  title: Handover — Forschung-Folge 95 (Stand 2026-09-19)
  session: Forschung-Folge 95
  class: handover
  date: 2026-09-19
  sha256: 59fbb4fea291dd814a84e695fb7bcd0c365ad82d89c5049e6dbcf4bfc561824c
  status: live
-->
# Handover — Forschung-Folge 95 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatiert); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 95)

- **HEAD** `3d2e6adb` == `origin/main`; `git_safety` Snapshot `refs/safety/1789844667`.
  Seit folge94 (`a786e208`): `566d8ea4` (coherent-phase-null), `3d2e6adb`
  (Archiv-Move folge93). Fremde uncommittete Arbeit: `src/mathematikerin/te.rs`,
  `docs/handover/post.md`, die Handover-Moves (`entscheid24`, `forschung44/51`,
  `bau91`→`archiv/`, neu `bau92`) — nicht angefasst.
- **Postfach** — `post.md` leer (kein `An forschung`); jüngster `mail_ledger`-Eingang
  `1789795811` (Rubin-Forum AGN, informativ). NSE/Haug bleibt Trigger.
- **CI** — `te-gate` `35462518676` **pending** @`3d2e6adb` (deckt die neuen
  coherent-phase-Arme `gate_fpr_autocorrelation*` + n=1000 conditional null);
  `ci-check` churn (fmt-apply `35462507189` failure, viele cancelled). Die
  CI-Zeile in `docs/zustand/external-state.md` wird zitiert, nicht kopiert.

## Hyperscanning-TE — Screen-Verifikation (härtester undatiert)

- **Gebaut (dieses Atom):** Der Screen wurde vom Watchdog abgeschossen, weil der
  Median aus einem **leeren grünen Lauf** stammte: `35456288477` @`778c68db` (154 s)
  zeigte für alle 33 Probanden `absent — no readable [Fz] series (0 honored)` und
  endete trotzdem exit 0. Der Loader-Fix-Lauf `35457694014` @`9806bb52` lud alle 33
  Probanden (`n = 60000 srate = 300`) und wurde bei 3758 s vom 2×-154 s-Floor
  gecancelt (Watchdog-Log `2026-09-19T20:21:26`). Rat (council, pro/max):
  (c) + n≥2. Umgesetzt: `hyperscanning_group_te` exit(2), wenn **kein** Task eine
  vollständige Triade trug (Report bleibt ehrlich, nur die Conclusion wird rot);
  `bin/ci_watchdog.sh` `median_duration` erst ab n≥2; `AGENTS.md`-Regelzeile.
- **Loader-Fix verifiziert grün** — `35457694014`: alle 33 Probanden laden, die
  `[Fz]`-Serie ist lesbar. `silence_map` FN-Gate grün — Artefakt `measure-gates.txt`
  (`35456056999`): 6/6 silence_map + 4/4 betti0.
- **Screen erneut dispatchen** (Session-Duty, geändertes Werkzeug). (Schritt:
  `gh workflow run hyperscanning-te`; Artefakt `hyperscanning-te-report` = die
  Messung; der Lauf läuft nun ohne Ein-Proben-Floor durch.)
- **Kohärenten Paar-Null auf den Screen anwenden** — nach grünem Screen-Lauf.
  (Schritt: `hyperscanning_group_te --null coherent-phase` als zweiter Dispatch;
  die Differenz der Survivor-Zellen ist die Messung „über das Lineare hinaus".)
- **Risse (Rat, ungeglättet):** Gauss-Blindheit (CoherentPhase kann Gauss-linearen
  Transfer prinzipbedingt nie detektieren — zwei Nulls, zwei Fragen); nicht-Gauss-
  Marginale; Familien-Max-Interaktion unter CoherentPhase (stärkstes lineares Paar
  verdeckt schwächere nichtlineare → FN-Risiko); gemeinsamer zirkulärer
  Randartefakt; Ksg-Verhalten `pending`.
- **Bestätigungsstufe** — Screening p95/200, Überlebende p99/1000. (Schritt:
  `hyperscanning_group_te --percentile 99 --surrogates 1000` auf den Survivor-Zellen.)
- **Eigen-Historie als Konditionierer** — Apparat existiert (`conditional_te_*_n`).
  (Schritt: `LaggedCond` auf die Zielserie statt `&[]`.)
- **Mehrere Frontalkanäle** — `--channel` existiert. (Schritt: F3/F4 als getrennte
  Workflow-Matrix-Läufe.)
- **Topologische TE** (Takens, `te_compute`) als Upgrade. (Schritt:
  `topological_te_phase` statt `transfer_entropy_binned` im Gruppen-Werkzeug.)

## te-gate `35462518676` — `wartend`

- pending @`3d2e6adb`; deckt `gate_fpr_autocorrelation*` (inkl. coherent-phase) +
  `gate_conditional_arx_fpr_fn_n1000`. (Schritt: `ci_manage view 35462518676`;
  grün ⇒ die Arme frei, `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.) Kein Re-Dispatch.

## Abgeleiteter Watchdog-Floor — benannter offener Punkt (Rat)

- Für den Fall „ehrlicher schneller Median → legitime Verlangsamung" jenseits 2×.
  (Schritt: messen, ob ein legitimer `hyperscanning-te`-Lauf 2× den ehrlichen Median
  (~126 min nach Erholung) übersteigt; nur dann `timeout/2²`-Floor bauen — nicht
  ohne diese Messung.)

## measure-gates Budget-Defekt

- `35456056999` stirbt im `timeout-minutes: 120` während `silence_map_probe` gegen
  den CDN-Katalog; das FN-Gate selbst ist grün (Artefakt `measure-gates.txt`).
  (Schritt: schweren CDN-Probe aus dem Gate-Lauf lösen oder Budget trennen —
  eigenes Atom, kein Watchdog-Defekt.)

## vC-Permeabilitäts-Karte — `operator-gebunden` (an entscheid gepostet)

- **Messakt**: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>
  ./target/release/omegaflow` lokal nach Core-Release-Build; danach
  `perm_target_probe --live <pfad>`. (Schritt: Operator-Wort + Release-Build.)
- **Richtungssemantik**: Legacy `exp(-vC/(g+1/C))` vs. heute `tanh(v_c/(g+PERM_GROUND))`.
  (Schritt: Operator-Wort.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- Läuft im `te-gate` `35462518676` (siehe oben). (Schritt: dessen Verdikt.)

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin`

- Cruise-Daten erst zur Wissenschaftsphase (~April 2027) freigegeben. (Schritt:
  kein TAP-Abruf vorher.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.): „in einigen Tagen"; er sendet die TRISP-NSE-Daten
  selbst. Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle +
  Compiler + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Der Rat (council, pro/max) entschied das Watchdog-/Null-Atom — ein benanntes
  hartes Atom, kein flash-Mirror. `grind-flash` sichtete den Exit-Pfad
  (`hyperscanning_group_te.rs`) und den fehlenden Watchdog-Test — Routine.
  Burn (gemessen, `session_burn`): council `deepseek-v4-pro` $0.0449 (1 Lauf),
  `grind-flash` `deepseek-v4-flash` $0.0034 (1 Lauf) — pro/max 13× für das
  Urteil, flash für die Routine; beide korrekt/ vollständig.

## Geteilter Baum — eigener Pfad-Satz

- `AGENTS.md` (Watchdog-Regelzeile)
- `bin/ci_watchdog.sh` (Median erst ab n≥2)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (Exit-Gate + Test)
- `docs/zustand/external-state.md` (CI-Zeile auf `3d2e6adb`)
- `docs/handover/handover-2026-09-19-forschung-folge95.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge94.md` (Move)
- **Fremd (nicht angefasst):** `src/mathematikerin/te.rs`, `docs/handover/post.md`,
  die Handover-Moves (`entscheid24`, `forschung44/51`, `bau91`→`archiv/`, `bau92`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
