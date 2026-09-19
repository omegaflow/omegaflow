<!--
  title: Handover — Bau-Folge 88 (Stand 2026-09-19)
  session: Bau-Folge 88
  class: handover
  date: 2026-09-19
  sha256: 9d56b23c17d435b113e69a28d2fa43fbb1d9bfa0c5cdb1c5e73a3f7c59eb37b1
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

- **HEAD** `3c1fb76c` (research folge87) bei Session-Beginn; `origin/main == HEAD`.
  Der eigene Commit dieser Session folgt darauf.
- **CI** — `te-gate` `35425288111` @`e9e9ef63` in_progress (queued 05:56, alter
  Stand); `ci-check` rot-Basis `35389926289` @`69fbe665` (8 rot). Die eigenen
  Läufe starten nach dem Push: `ci-check` (push-getriggert, `src/**`) + der
  eigene `te-gate`-Dispatch. Ergebnis nie erwartet — `ci_manage view`/`log`
  einmal beim nächsten Pass.
- **Arbeitsbaum** — fremd: `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ` (ernte), `docs/zustand/external-state.md`, die ernte-Handover-
  Moves — unangetastet.

## TE-Gate-Messung nach dem per-Lag-Fix (wartend — härtester Punkt, kein Auswahlpunkt)

- **Gebaut in dieser Session (Rat-Verdikt 2026-09-19, verbindlich):** das
  per-Lag-Konditionsdesign — eine Konditionsspalte je gewähltem `(series, lag)`.
  `LaggedCond { series, lag }` (te.rs, neu) trägt die exakte Spalte durch:
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
- **Zwei rote Gates + alle übrigen** — `flare_envelope_conditional_keeps_true_coupling`
  (te.rs) und `gate_fpr_autocorrelation_restricted_null_binned_n_surr_200`
  (te.rs) sind neu zu messen; ebenso die übrigen 6 aus dem roten Basis-Lauf.
  (Schritt: `ci_manage view` auf den push-getriggerten ci-check @dem eigenen
  Commit + den eigenen te-gate-Dispatch, je einmal.) · `wartend`
- **Der fabrizierte Null ist bloßgelegt** (folge87, fortgeschrieben): die
  ARX-FPR-Zellen tragen nach dem pcmci-Fix echte Zahlen, die die
  8 %/2pp-Decken überschreiten können — die per-Lag-Spalten sind die Antwort
  des Designs; der nächste ci-check/te-gate-Lauf misst, ob sie tragen.
  (Schritt: dieselben Run-IDs wie oben.) · `wartend`

## Offen aus folge87 (weitergetragen)

- **Betti-0 `betti0_persistence`** — Fasy-Bootstrap-Gate (`pending`) + erster
  Schritt Null-Verteilungs-Quantil in CI (`measure-gates`, `betti0_probe.rs`);
  0.5-Schwelle (te.rs) unkalibriert. `--port` force-Gate: `force_type`-Verteilung
  + Fixture (Rat: Weg B CI). (Schritt:
  `docs/handover/archiv/handover-2026-09-18-entscheid-folge52.md` §Gremium+Wissenschaft.) · `pending`
- **Browser-Relay-Stale** — `src/archivar/relay.rs:881` /
  `static/constants.js:205–213` parsen `weave_epoch`, vergleichen nichts.
  (Schritt: `now − weave_epoch >= VERDICT_STALE_S` (604800) im Relay-/Browser-Pfad.) · `pending`
- **Eine-Quelle-Ideal `ttl`** — `VERDICT_STALE_S`
  (`src/archivar/weberin_verdicts.rs:5`) spiegelt die Register-`ttl` (604800) +
  `BIN_TTL_S` in den Compilern. (Schritt: `ttl` durch den `LoopCtx` an die
  Konsumenten reichen.) · `pending`
- **`register_lookup --open` Binary veraltet** — die Quelle implementiert
  `run_open` (`b67f5cae`), PATH- und `target/release`-Binary geben die
  `--live`-Usage. (Schritt: Workflow ermitteln, der die Tools-Binaries
  baut/publiziert.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — `35351411938` pending. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** — kein Bau nötig. · `pending`

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit:** `src/mathematikerin/te.rs`, `src/mathematikerin/tests.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `.github/workflows/te-gate.yml`, `docs/handover/handover-2026-09-19-bau-folge88.md`
  (neu), Move `handover-2026-09-19-bau-folge87.md` → `archiv/`.
- **Fremd (nicht anfassen):** `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ` (ernte), `docs/zustand/external-state.md`, die
  ernte-Handover-Moves. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
