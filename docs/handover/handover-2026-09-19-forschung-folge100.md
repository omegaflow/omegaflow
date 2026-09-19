<!--
  title: Handover — Forschung-Folge 100 (Stand 2026-09-19)
  session: Forschung-Folge 100
  class: handover
  date: 2026-09-19
  sha256: 5c72b8c5a9dc300d04fff28c063a1bbe6fc076a7569b586e399053724b054ac9
  status: live
-->
# Handover — Forschung-Folge 100 (2026-09-19)

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
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 100)

- **HEAD** `bbf3cf3a` (forschung folge99) == `origin/main` bei Session-Beginn;
  FF stand. `git_safety` Snapshot `refs/safety/1789852858`.
- **Postfach** — kein neuer Eingang seit `1789795811` (Rubin-Forum AGN DP2,
  informativ); `post.md` trägt nur `An bau` (sniff-Partial-Hash), **keine**
  `An forschung`-Zeile. Eintrag zitiert (`external-state.md`), nicht kopiert.
- **CI** — die `external-state.md`-CI-Zeile auf HEAD `bbf3cf3a` fortgeschrieben
  (`ci_manage list` 2026-09-19 ~21:20Z): **failure** `openneuro-cdn` `35468606830`
  @`7aa5c23e` (ds007822 .set, parser-gap); **success** `measure-gates` `35468941451`
  @`4638d7f3`, `quake-feeds-cdn` `35468653918`, `ned-cdn` `35468572091`,
  `placebo-ave-cdn` `35468313922`, `openneuro-cdn` `35468312819` @`d1750fe0`,
  `auto-dispatch` `35468305429`, `harvest-dispatch` `35468305339`;
  **pending/in_progress** `ci-check` `35470085666` @`bbf3cf3a`,
  `silence-map-probe` `35468942740` @`4638d7f3`, `ci-check` `35468441157`,
  `hyperscanning-te` `35468144989` @`5219db7e`, `te-gate` `35462518676`
  @`3d2e6adb`; **cancelled** `ci-check` `35470066148`/`35469397475`/`35468939645`
  /`35468328112`/`35468305262`/`35468161696`/`35468134873` — das Familien-Max-FN-
  Verdikt @`2454de6e` liegt noch **nicht** vor (`35470066148` verdrängt).
- **Fremde uncommittete Arbeit (nicht angefasst):** `src/archivar/mod.rs`,
  `src/lib.rs`, `src/archivar/hdf5.rs`, `src/archivar/matfile.rs`,
  `src/archivar/brainvision.rs` (neu), `src/archivar/snirf.rs` (neu),
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `tools/harvest/src/bin/brainvision_compiler.rs` (neu),
  `tools/harvest/src/bin/snirf_compiler.rs` (neu),
  `tools/utils/src/bin/archive_search/net.rs`, die drei Handover-Moves
  `entscheid-folge24`/`forschung-folge44`/`forschung-folge51` → `archiv/`,
  `handover-2026-09-19-bau-folge95.md`.

## Riss 2 — zirkuläres FFT-Randartefakt — Messgate gebaut — `wartend` (CI)

- `gate_phase_surrogate_padding_edge` (`src/mathematikerin/te.rs` `#[cfg(test)]`)
  misst die Lag-Autokorrelations-Abweichung des Surrogats vom Input an einer
  wrap-diskontinuierlichen Rampe bei n=1000 (Pad auf 1024) gegen n=1024 (exakt);
  der Pad-Effekt wird isoliert (`dev_pad ≤ dev_exact + 0.15`). `cargo check
  -p omegaflow --all-targets` 0 Fehler/0 Warnungen. (Schritt: das `ci-check`-
  Verdikt auf dem eigenen HEAD lesen — grün ⇒ Pad unkritisch; rot ⇒ die gemessene
  Abweichung ist der Riss, dann Fix als eigenes Atom.)

## Riss 3 — τ-Instabilität — Messgate gebaut — `wartend` (CI)

- `gate_mi_lag_stability` (`src/mathematikerin/te.rs` `#[cfg(test)]`) misst
  None-Rate/mean/sd von `find_mi_lag` über 200 Phase-Surrogate je
  n∈{32,64,128,512,4096} (print-only, keine Behauptung); die τ-Freeze bleibt
  bewusst aus diesem Atom. (Schritt: das `ci-check`-Verdikt lesen — die gedruckte
  Serie ist die Messung; danach τ-Freeze + Re-Pass der vier Kalibrier-Gates.)

## Familien-Max-FN — Per-Zelle-Fix gebaut — `wartend` (CI-Verdikt)

- **Riss 1, gebaut** (Rat-Verdikt „build now"): `surrogate_family_maxima`
  (`tools/measure/src/bin/hyperscanning_group_te.rs`) liefert die
  Familien-Maxima **und** je Zelle eine Surrogat-Verteilung; `main` entscheidet
  zweistufig (Familien-Max-Survivor + Per-Zelle-Survivor). Schätzer und Null
  unverändert → die vier Kalibrier-Gates unberührt. Neue Tests `family_fn_gate`
  und `family_fp_gate`. (Schritt: das Verdikt des `ci-check` `cargo test
  --release` auf `2454de6e` lesen — `35470066148` **cancelled**, Nachfolger
  `35470085666` pending @`bbf3cf3a`; grün ⇒ Fix hält; rot ⇒
  `family_fn_gate`-Power/Tuning nachziehen.)
- **Riss 4 — Ksg off-path — offen (Operator-Entscheid) — `operator-gebunden`:**
  der Familien-Screen ruft nur `topological_te_estimate`, nie Ksg; die
  n=1000-Ksg-FPR-Sweeps sind print-only/ignored. (Schritt: Operator entscheidet
  verdrahten oder descopen mit gemessenem „nicht auf dem Pfad".)

## TE-Kalibrier-Gate — CoherentPhase-Tests — `wartend`

- Fix aus folge98 (fünf Aufrufstellen `(target, driver)` getauscht) ist committed;
  das Verdikt steht aus. (Schritt: das Verdikt des `ci-check` auf dem eigenen HEAD
  lesen; grün ⇒ das Kalibrier-Gate hält, rot ⇒ Fixture/Schätzer nachziehen.)

## measure-gates Budget — CDN-Probe abgetrennt — `wartend`

- Die Abtrennung ist gemessen: `measure-gates` `35468941451` @`4638d7f3` **success**
  (der Lauf, der zuvor am Probe-Budget starb), `silence-map-probe` `35468942740`
  eigenes Workflow (in_progress). (Schritt: das Verdikt des `silence-map-probe`-
  Laufs lesen; danach ist der Punkt geschlossen.)

## Takens-Screen — Wandzeit-Messung — `wartend`

- Lauf `35468144989` @`5219db7e` pending (`null_model=phase`, `percentile=95`,
  `surrogates=200`) — erster Lauf auf dem Takens-Stand, zugleich die
  **Wandzeit-Messung** (O(n²)-Zelle gegen das alte O(n)-Gitter). (Schritt:
  `ci_manage view 35468144989` einmal nach Abschluss; grün mit Survivor/Pending ⇒
  Wandzeit lesen; Exit-2-Gate ⇒ das ist die Messung, kein Defekt.) Erst diese
  Messung entscheidet den Watchdog-Floor-Punkt.

## Bestätigungsstufe — `wartend`

- Screening p95/200, Überlebende p99/1000; Workflow-Inputs stehen. (Schritt: nach
  dem Screen-Lauf `gh workflow run hyperscanning-te -f null_model=coherent-phase
  -f percentile=99 -f surrogates=1000`; die Differenz der Survivor-Zellen ist die
  Messung „über das Lineare hinaus".)

## Mehrere Frontalkanäle — `wartend`

- `--channel` trägt der Workflow. (Schritt: `gh workflow run hyperscanning-te
  -f channel=F3` (und F4) als getrennte Läufe, nach der Bestätigungsstufe.)

## te-gate `35462518676` — `wartend`

- pending @`3d2e6adb`; deckt `gate_fpr_autocorrelation*` (inkl. coherent-phase) +
  `gate_conditional_arx_fpr_fn_n1000`. (Schritt: `ci_manage view 35462518676`;
  grün ⇒ die Arme frei, `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.) Kein Re-Dispatch.

## Abgeleiteter Watchdog-Floor — `wartend`

- Für den Fall „ehrlicher schneller Median → legitime Verlangsamung" jenseits 2×.
  (Schritt: die Wandzeit-Messung des Takens-Screens oben; nur wenn ein legitimer
  Lauf 2× den ehrlichen Median übersteigt, `timeout/2²`-Floor bauen.)

## vC-Permeabilitäts-Karte — `operator-gebunden` (an entscheid gepostet)

- **Messakt**: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>
  ./target/release/omegaflow` lokal nach Core-Release-Build; danach
  `perm_target_probe --live <pfad>`. (Schritt: Operator-Wort + Release-Build.)
- **Richtungssemantik**: Legacy `exp(-vC/(g+1/C))` vs. heute `tanh(v_c/(g+PERM_GROUND))`.
  (Schritt: Operator-Wort.)

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

- **Klasse „TE-/Null-Konstruktion" (Diagnose), geschlossen:** `general` (flash)
  vs. `research-max` (pro/max, folge98 `$0.0565`) — die flash-Diagnose der vier
  Risse war vollständig und zeilengenau; alle tragenden Aussagen am Baum
  verifiziert (`te.rs:1577–1601`, `1603–1636`, `1825–1893`, `2277`, `2336–2337`,
  `2328`; `hyperscanning_group_te.rs:110–125`). Sieger `general`/flash,
  `$0.0265` (`session_burn`) — ~2.1× günstiger als `research-max`, identisches
  Ergebnis. Kein pro/max-Vorlauf nötig.
- **Rat** (Architektur, Riss-1-Fix): `$0.0283`; **Bau** (Per-Zelle-Fix): `grind-flash`.
- **Bau** (Riss-2/3-Messgates): `build` (dieses Atom, flash) — kein Sub-Agent nötig.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (`gate_phase_surrogate_padding_edge` +
  `gate_mi_lag_stability`)
- `docs/zustand/external-state.md` (CI-Zeile auf `bbf3cf3a` fortgeschrieben)
- `docs/handover/handover-2026-09-19-forschung-folge100.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge99.md` (Move)
- **Fremd (nicht angefasst):** `src/archivar/{mod.rs,lib.rs,hdf5.rs,matfile.rs,brainvision.rs,snirf.rs}`,
  `tools/harvest/src/bin/{icesat2_atl03,brainvision,snirf}_*.rs`,
  `tools/utils/src/bin/archive_search/net.rs`,
  die drei Handover-Moves `entscheid-folge24`/`forschung-folge44`/`forschung-folge51` → `archiv/`,
  `handover-2026-09-19-bau-folge95.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
