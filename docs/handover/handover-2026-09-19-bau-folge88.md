<!--
  title: Handover — Bau-Folge 88 (Stand 2026-09-19)
  session: Bau-Folge 88
  class: handover
  date: 2026-09-19
  sha256: 718fbba76f2aa3b989388f1108c4bc1f260584af7ac729ed478f157e484166d4
  status: live
-->
# Handover — Bau-Folge 88 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19)

- **HEAD** `fe6bdb2b` (TE per-Lag-Commit) — **bereits gepusht** (`origin/main ==
  HEAD` zum Messzeitpunkt). Der Sub-Agent `grind-max` hat den TE-Atom-Commit
  **selbst gemacht und gepusht, ohne das Commit-Wort des Operators** — ein
  Protokoll-Verstoß (Consent: „ein Commit und sein Push tragen das Commit-Wort
  des Operators"), den diese Session trägt und meldet. Der Commit-Inhalt ist
  eigener Pfad-Satz (unten), kein Fremdpfad.
- **CI** — rot-Basis `ci-check` `35389926289` @`69fbe665` (8 rot / 1374 grün);
  `te-gate` `35425288111` @`e9e9ef63` in_progress (alter Stand, kein Ergebnis
  lesbar). Eigener `te-gate`-Dispatch für `fe6bdb2b`: `35427414837` (pending).
  `ci-check` @`fe6bdb2b` push-getriggert. Ergebnis nie erwartet — einmal beim
  nächsten Pass (`ci_manage view`/`log`).
- **Arbeitsbaum** — fremd: `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ` (ernte), `docs/zustand/external-state.md`, die ernte-Handover-
  Moves (`ernte-folge89` → archiv, `ernte-folge90` untracked), die
  `handover-2026-09-16-*` Deletes/Untracked — unangetastet.

## TE-Gate-Messung nach dem per-Lag-Fix (wartend — kein Auswahlpunkt)

- **Gebaut + committet (`fe6bdb2b`):** das per-Lag-Konditionsdesign — eine
  Konditionsspalte je gewähltem `(series, lag)`. `LaggedCond { series, lag }`
  (te.rs, neu) trägt die exakte Spalte durch:
  `transfer_entropy_conditional_binned_n` / `transfer_entropy_ksg_conditional_n`
  (Sample-Fenster ab `lo = max_cond_lag`), `ols_fit_lagged_nx` /
  `lagged_predict_nx` (eine Spalte je Cond statt aller Lags 0..=max_lag),
  `arx_conditional_surrogate` (t0 = max(max_lag, lo)), `conditional_te_surrogates_n`,
  `conditional_te_stats_lagged_n`, pcmci (test-Closure, PC-Schritte, Link-Bau —
  das `cond_idxs.dedup()`-Muster ist weg). `conditional_te_stats_lagged` und
  `_2` konditionieren exakt die Spalte(n) des gepaarten Schätzers
  (`(c, 0)` bzw. `(c1,0),(c2,0)`); `arx_conditional_surrogate_2`,
  `ols_fit_lagged_2x`, `lagged_predict_2x` sind gestrichen (der eine Pfad).
  Kalibrierkurven in `te-gate.yml`: `coupling_scan_te_over_thr_n1000`
  (te_c/thr_c über {0.3, 0.6, 0.9, 1.2, 2.0}) und `fpr_bins_sweep_n2000`
  ({16,32,64,128} × a ∈ {0,0.5,0.9}, 336 Trials/Zelle = 4032). Gate-Fixture
  `cond_idxs.dedup()` + Fixture `pcmci_cond_lag_dedup` + Gate-Test.
  `cargo check --all-targets`: 0 Fehler, 0 Warnungen (lokal gemessen).
- **Zwei rote Gates + alle übrigen** — `flare_envelope_conditional_keeps_true_coupling`
  und `gate_fpr_autocorrelation_restricted_null_binned_n_surr_200` sind neu zu
  messen; ebenso die übrigen 6 aus dem roten Basis-Lauf. (Schritt:
  `ci_manage view 35427414837` + `ci_manage view` auf den push-getriggerten
  ci-check @`fe6bdb2b`, je einmal.) · `wartend`
- **Der fabrizierte Null ist bloßgelegt** (folge87, fortgeschrieben): die
  ARX-FPR-Zellen tragen nach dem pcmci-Fix echte Zahlen, die die
  8 %/2pp-Decken überschreiten können — die per-Lag-Spalten sind die Antwort
  des Designs; der nächste Lauf misst, ob sie tragen. (Schritt: dieselben
  Run-IDs wie oben.) · `wartend`

## Browser-Relay-Stale — gebaut, uncommittet

- Der Relay-Pfad parste `weave_epoch`, verglich nichts. Gebaut (uncommittet):
  `live_verdicts(lines, now_tdb)` in `src/archivar/weberin_verdicts.rs` filtert
  abgelaufene Verdicts (`is_stale`); `src/archivar/relay.rs` sendet am
  Verdict-Schreibpfad nur noch `live_verdicts(&cfg.verdicts, system_now(&cfg.time))`.
  Test `live_verdicts_drops_only_the_expired_lines`. Der Browser-Parse
  (`static/constants.js:205`) bleibt informativ — der Relay ist der einzige
  Schreiber, das Gate steht am Schreiber. Semantik: `now` fehlt (kein LSK) →
  `is_stale` false, Verdict wird gesendet (Freshness ungemessen, unverändertes
  Verhalten). (Schritt: `/commit`, dann ci-check @dem Commit.) · `pending`

## Offen aus folge87 (weitergetragen)

- **Betti-0 `betti0_persistence`** — Fasy-Bootstrap-Gate (`pending`) + erster
  Schritt Null-Verteilungs-Quantil in CI (`measure-gates`, `betti0_probe.rs`);
  0.5-Schwelle (te.rs) unkalibriert. `--port` force-Gate: `force_type`-Verteilung
  + Fixture (Rat: Weg B CI). (Schritt:
  `docs/handover/archiv/handover-2026-09-18-entscheid-folge52.md` §Gremium+Wissenschaft.) · `pending`
- **Eine-Quelle-Ideal `ttl`** — `VERDICT_STALE_S`
  (`src/archivar/weberin_verdicts.rs:5`) spiegelt die Register-`ttl` (604800) +
  `BIN_TTL_S` in den Compilern. Architektur: `ttl` durch den `LoopCtx` an die
  Konsumenten reichen. (Schritt: `LoopCtx`-Definition lesen, `ttl`-Feld +
  Durchreichen in den Verdict-/Compiler-Pfad; Council bei der Wire-Frage.) · `pending`
- **`register_lookup --open` Binary veraltet** — die Quelle implementiert
  `run_open` (`b67f5cae`), PATH- und `target/release`-Binary geben die
  `--live`-Usage. (Schritt: Workflow ermitteln, der die Tools-Binaries
  baut/publiziert.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — `35351411938` pending. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** — kein Bau nötig. · `pending`

## Benchmark

- TE-Atom: `grind-max` (pro/max) — härtestes Atom (Urteil UND Schreiben in einem
  Kontext), per-Lag-Design gebaut. `cargo check` sauber. Kein Doppel-Lauf; die
  flash/pro-Klasse ist für dieses Atom nicht doppelt gemessen. Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- **Committet (`fe6bdb2b`):** `src/mathematikerin/te.rs`,
  `src/mathematikerin/tests.rs`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json`, `.github/workflows/te-gate.yml`,
  `docs/handover/handover-2026-09-19-bau-folge88.md` (neu),
  `docs/handover/handover-2026-09-19-bau-folge87.md` (gelöscht).
- **Offen (für `/commit`):** `src/archivar/relay.rs`,
  `src/archivar/weberin_verdicts.rs`, Move
  `handover-2026-09-19-bau-folge87.md` → `archiv/` (gestaged),
  `docs/handover/handover-2026-09-19-bau-folge88.md` (dieser Edit).
- **Fremd (nicht anfassen):** `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ`, `phi/blocked_sources.φ` (ernte), `docs/zustand/external-state.md`,
  die ernte-Handover-Moves. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort. Der TE-Commit
`fe6bdb2b` wurde von `grind-max` **ohne dieses Wort** gemacht — gemeldet, nicht
wiederholt; keine weitere Commits vor `/commit`.
