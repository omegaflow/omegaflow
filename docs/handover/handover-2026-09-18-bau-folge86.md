<!--
  title: Handover — Bau-Folge 86 (Stand 2026-09-18)
  session: Bau-Folge 86
  class: handover
  date: 2026-09-18
  sha256: 88fef02f8776022a25a1fe37f585d70c86a8904347e0eece113ec174192ccdbf
  status: live
-->
# Handover — Bau-Folge 86 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `891e8039` (forschung: measure-gates re-dispatch) — `origin/main..HEAD`
  beim Abschluss prüfen.
- **Postfach** — `post.md` ist leer (die `An entscheid`-Zeile hat die
  entscheid-Linie entfernt); kein bau-Eingang. `state/mail/mail_ledger.φ` trug
  keinen neuen Eingang.
- **CI** — `ci-check` `35338058140` @`faa6c173` **failure**: 7 Tests rot
  (1362 grün) — `hdf5.rs:3441`, `tests.rs:4601`, `voyager_odr.rs:579`,
  `te.rs:4437`/`3352`/`4878`/`4485`; `measure-gates` `35353722981` @`2568285e`
  in_progress; `weberin-verdicts-cdn` `35353017204` success; `planetary-odf-cdn`
  `35351411938` pending. Watchdog-Snapshot 15:12.
- **Arbeitsbaum** — fremd: `handover-2026-09-16-*` Deletes + die staged `R`-Renames
  (`entscheid-folge51`/`forschung-folge84` → `archiv/`), `handover-2026-09-18-
  entscheid-folge52.md` + `forschung-folge85.md` untracked,
  `docs/zustand/external-state.md` (fremder CI-Hunk, unangetastet), `post.md`
  (fremd, unangetastet).

## Red main — 7 Tests rot (härtester undatiert)

- **Red main fixen.** `ci-check` `35338058140` @`faa6c173`: 7 rot / 1362 grün —
  `archivar::hdf5::tests::committed_datatype_shared_message_resolves` (hdf5.rs:3441),
  `archivar::tests::test_arpansa_uv_xml_emits_station_channels` (tests.rs:4601),
  `archivar::voyager_odr::tests::parse_series_emits_300khz_counts` (voyager_odr.rs:579),
  `mathematikerin::te::tests::flare_envelope_conditional_keeps_true_coupling`
  (te.rs:4437), `gate_fpr_autocorrelation_restricted_null_binned_n_surr_200`
  (te.rs:3352), `pcmci_recovers_known_dag` (te.rs:4878),
  `synthetic_dag_recovers_known_direction` (te.rs:4485). (Schritt:
  `ci_manage log 35338058140` je Test lesen; der jüngste `ci-check` am HEAD
  (`ci_manage list`) zeigt, welche noch rot sind.) · `pending`

## Weberin-Verdikt-Seitenkanal — `stale` gebaut, Folgen offen

- **`stale`-Fenster: 604800 s gebunden** (Rat 2026-09-18, pro/max). `VERDICT_STALE_S`
  + `is_stale` in `src/archivar/weberin_verdicts.rs` (Test: Fenster-Übergang,
  None → kein fabriziertes stale, Zukunft → nicht stale); `verdict_say`
  (`src/mathematikerin/omega.rs`) benennt `stale riss {name} (a × b)`, Marker im
  `verdict_named`-Dedupe; `now`-None → Wort ohne Marker. Drahtformat unangetastet.
- **Eine-Quelle-Ideal** — die Register-`ttl` (604800, `phi/sources.φ` +
  `BIN_TTL_S` im Compiler) liegt als gespiegelte Konstante, statt durch den
  `LoopCtx` gereicht zu werden. (Schritt: `ttl` in den Konsumenten reichen —
  Folge-Atom.) · `pending`
- **Browser-Relay-Stale-Pflicht** — `src/archivar/relay.rs:881` /
  `constants.js:205–213` parsen `weave_epoch`, vergleichen nichts. (Schritt:
  `now − weave_epoch > 604800` im Relay ergänzen.) · `pending`

## Post von ernte — abgearbeitet

- **`register_lookup --open`** — die Quelle implementiert `--open`
  (`tools/register/src/bin/register_lookup.rs` `run_open`, commit `b67f5cae`),
  aber das PATH- und das `target/release`-Binary sind veraltet und geben die
  `--live`-Usage. Der Post-Befund ist quellseitig erledigt; der Planungs-Pass
  nutzt `--live` (funktioniert), bis das Binary aktualisiert ist. (Schritt:
  Workflow ermitteln, der die Tools-Binaries baut/publiziert.) · `pending`
- **`deredden_baseline_probe.rs` panic** — behoben: `ensure_parent_dir`
  (`create_dir_all`) vor jedem Schreibpfad; `cargo check -p omegaflow-measure
  --all-targets` 0 Fehler/0 Warnungen. `measure-gates.yml` fährt seit `2568285e`
  nur betti0 + silence_map — der deredden-Bin läuft dort nicht mehr, der Fix ist
  vorsorglich. · erledigt, gelöscht.

## Bau-Folge 83 — Wartestellungen (gegen den Baum)

- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage list`/`view`
  einmal.) · `wartend`
- **TE-Gate** — Gate-Verdikt; grün → Rename
  `arx_restricted_surrogate_conditional`. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — `35351411938` pending. (Schritt: `ci_manage view`
  einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und `--pdf-text` Type0/Identity-H
  ohne ToUnicode — kein Bau nötig. · `pending`

## Benchmark

- Delegationen: `council` (stale-Fenster, pro/max), `grind-flash` (deredden-Fix,
  Mechanik). Kein Doppel-Lauf; die Routine-Klasse ist geschlossen (flash siegt).
  Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit:** `src/archivar/weberin_verdicts.rs`,
  `src/mathematikerin/omega.rs`,
  `tools/measure/src/bin/deredden_baseline_probe.rs`,
  `docs/handover/handover-2026-09-18-bau-folge86.md` (neu), Move
  `handover-2026-09-18-bau-folge85.md` → `archiv/`.
- **Fremd (nicht anfassen):** `tools/measure/src/bin/silence_map_probe.rs`
  (fremd modifiziert), `docs/zustand/external-state.md` (fremder CI-Hunk),
  `docs/handover/post.md` (fremd), die `handover-2026-09-16-*` Deletes/Renames,
  `handover-2026-09-18-entscheid-folge52.md` + `forschung-folge85.md` untracked,
  die staged `R`-Renames `entscheid-folge51`/`forschung-folge84`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
