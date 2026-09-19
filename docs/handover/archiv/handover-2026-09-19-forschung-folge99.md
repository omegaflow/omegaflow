<!--
  title: Handover — Forschung-Folge 99 (Stand 2026-09-19)
  session: Forschung-Folge 99
  class: handover
  date: 2026-09-19
  sha256: 88b1c27df4280e076b09b735111cc84644c5b62bcbd91789999154e905e9d613
  status: live
-->
# Handover — Forschung-Folge 99 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 99)

- **HEAD** `f6b10df0` (ernte folge100) == `origin/main` bei Session-Beginn;
  FF stand. `git_safety` Snapshot `refs/safety/1789851601`. Der eigene Commit
  dieser Session folgt (HEAD-Wechsel).
- **Postfach** — kein neuer Eingang seit `1789795811` (Rubin-Forum AGN DP2,
  informativ); `state/mail/mail_ledger.φ` letzte Zeile `1789795811`; `post.md`
  trägt nur `An bau` (sniff-Partial-Hash), **keine** `An forschung`-Zeile.
  Eintrag zitiert (`external-state.md`), nicht kopiert.
- **CI** — zitiert aus `docs/zustand/external-state.md` (Zeile CI-Status,
  Ernte-Folge 100 @`4638d7f3`; nicht kopiert). Eigene Messung: `measure-gates`
  `35468941451` **success** @`4638d7f3`; `silence-map-probe` `35468942740`
  in_progress @`4638d7f3`; `ci-check` `35468939645` **cancelled** (Verdrängung),
  `35469397475` pending @`4638d7f3`; `hyperscanning-te` `35468144989` **pending**
  @`5219db7e`; `te-gate` `35462518676` **pending** @`3d2e6adb`.
- **Fremde uncommittete Arbeit (nicht angefasst):** `src/archivar/mod.rs`,
  `src/lib.rs`, `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `tools/utils/src/bin/archive_search/net.rs`, `src/archivar/brainvision.rs`
  (neu), die drei Handover-Moves `entscheid-folge24`/`forschung-folge44`/
  `forschung-folge51` → `archiv/`, `handover-2026-09-19-bau-folge95.md`.

## Familien-Max-FN — Per-Zelle-Fix gebaut — `wartend` (CI-Verdikt)

- **Riss 1, gebaut** (Rat-Verdikt „build now"): `surrogate_family_maxima`
  (`tools/measure/src/bin/hyperscanning_group_te.rs`) liefert jetzt die
  Familien-Maxima **und** je Zelle eine Surrogat-Verteilung (eine RNG-Ableitung
  pro Replikation/Triade/Member — die Familien-Max-Zeile bleibt byte-reproduzierbar);
  `main` entscheidet zweistufig: Familien-Max-Survivor (strengere FWER-Linie) +
  Per-Zelle-Survivor (die vom Familien-Maximum maskierte schwächere Transfer-Zelle
  wird benannt). Schätzer und Null unverändert → die vier Kalibrier-Gates laufen
  unberührt. Neue Tests `family_fn_gate` (starkes lineares Paar A→B + schwächeres
  nichtlineares C→D, `coherent=true`: Per-Zelle findet C→D, Familien-Max nicht —
  die entfernte FN) und   `family_fp_gate` (unabhängige strukturierte Familie,
  Per-Zelle nahe Zufall). (Schritt: das Verdikt des `ci-check` `cargo test
  --release` auf dem eigenen HEAD `2454de6e` lesen — dispatcht `35470066148`;
  grün ⇒ Fix hält; rot ⇒ `family_fn_gate`-Power/Tuning nachziehen — die neuen
  Gates sind lokal ungemessen, CI ist die Messung.)
- **Riss 2 — zirkuläres FFT-Randartefakt — offen:** `phase_randomized_surrogate`
  (`te.rs:1577–1601`) nullt auf `next_power_of_two`, rotiert zirkulär, schneidet ab;
  DC/Nyquist unrotiert. Richtung plausibel FN, **ungemessen** (kein Rand-/
  Nicht-Zweierpotenz-Test; nur Roundtrip). (Schritt: erst messen — `te.rs`-Testgate
  n=1000 vs n=1024 + Rand-/Step-Serie, Surrogat-TE-Mittel/sd vergleichen; Fix erst
  nach der Messung.)
- **Riss 3 — τ-Instabilität auf kurzen Segmenten — offen (eigenes Atom):**
  `find_mi_lag` (`te.rs:1825–1893`) erstes lokales Minimum ab Lag 3; der
  Surrogat-Pfad schätzt τ je randomisierter Serie neu (`te.rs:2277`), τ-Varianz
  weitet die Null (konservativ, FN); ein `None` verwirft die Zelle (`?` `2336`).
  (Schritt: τ-Stabilitäts-Gate — None-Rate/mean/sd über n∈{32,64,128,512,4096};
  danach τ-Freeze + Re-Pass der vier Gates; der Rat hält die τ-Freeze bewusst aus
  diesem Atom heraus, weil sie die Null-Verteilung ändert.)
- **Riss 4 — Ksg off-path — offen (Operator-Entscheid):** der Familien-Screen ruft
  nur `topological_te_estimate`, nie Ksg; die n=1000-Ksg-FPR-Sweeps sind
  print-only/ignored. (Schritt: Operator entscheidet verdrahten oder descopen mit
  gemessenem „nicht auf dem Pfad".)

## TE-Kalibrier-Gate — CoherentPhase-Tests — `wartend`

- Fix aus folge98 (fünf Aufrufstellen `(target, driver)` getauscht) ist committed;
  das Verdikt steht aus. (Schritt: das Verdikt des `ci-check` auf dem eigenen HEAD
  lesen; grün ⇒ das Kalibrier-Gate hält, rot ⇒ Fixture/Schätzer nachziehen.)

## measure-gates Budget — CDN-Probe abgetrennt — `wartend`

- Die Abtrennung ist gemessen: `measure-gates` `35468941451` @`4638d7f3` **success**
  (der Lauf, der zuvor am Probe-Budget starb), `silence-map-probe` `35468942740`
  eigenes Workflow. (Schritt: das Verdikt des `silence-map-probe`-Laufs lesen;
  danach ist der Punkt geschlossen.)

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

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (Per-Zelle-Fix + 2 Tests)
- `docs/handover/handover-2026-09-19-forschung-folge99.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge98.md` (Move)
- **Fremd (nicht angefasst):** `src/archivar/mod.rs`, `src/lib.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `tools/utils/src/bin/archive_search/net.rs`, `src/archivar/brainvision.rs`,
  die drei Handover-Moves `entscheid-folge24`/`forschung-folge44`/
  `forschung-folge51` → `archiv/`, `handover-2026-09-19-bau-folge95.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
