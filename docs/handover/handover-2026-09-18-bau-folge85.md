<!--
  title: Handover — Bau-Folge 85 (Stand 2026-09-18)
  session: Bau-Folge 85
  class: handover
  date: 2026-09-18
  sha256: 12ebb22d81f9da76b7b145d11af2d469c9191ef77d9a39bff7b97e33a2c4c800
  status: live
-->
# Handover — Bau-Folge 85 (2026-09-18)

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

- **HEAD** `14ccc152` (bau folge84) — `origin/main..HEAD` leer, folge84 gepusht.
- **Postfach** — `post.md` trägt nur die forschung-Zeile (BepiColombo MORE), kein
  bau-Eingang.
- **CI** — `ci-check` 3× failure, `harvest`-Serie failure, `ci-check`/`allwise-cdn`/
  `harvest`/`health-check` in_progress (Watchdog-Snapshot 15:12). Fremder Ledger.
- **Arbeitsbaum** — forschung committete `src/mathematikerin/te.rs` +
  `betti0_probe.rs` während der Session; `silence_map_probe.rs` bleibt fremd
  modifiziert.

## Weberin-Verdikt-Seitenkanal — `stale` offen (härtester undatiert)

- **`stale`-Fenster binden.** Ein Verdikt, dessen `weave_epoch` älter als das
  Fenster ist, als `stale` benennen — nie still weiterverwenden. Die Bindung ist
  gemessen, aber **nicht eindeutig**: drei Fenster-Kandidaten — `ttl 604800`
  (7 d, eigene Quellen-`ttl` in `phi/sources.φ` + Compiler `BIN_TTL_S`),
  `ttl/Φ ≈ 373781` (4,3 d, `src/archivar/fetch.rs:313–315`), Eingangs-`ttl 86400`
  (1 d). Kein Konsument vergleicht heute `weave_epoch` gegen `now`.
  (Schritt: Rat/Operator bindet das Fenster, dann `verdict_say` in
  `src/mathematikerin/omega.rs` um `now − weave_epoch > Fenster` erweitern; das
  Drahtformat bleibt unangetastet.) · `pending`
- **`weberin-verdicts-cdn` — erster Lauf.** Workflow neu
  (`.github/workflows/weberin-verdicts-cdn.yml`), `url`-Zeile in `phi/sources.φ`,
  `--ci-mode` im Compiler. (Schritt: nach dem Commit
  `gh workflow run weberin-verdicts-cdn.yml`; Verdikt einmalig
  `ci_manage view <id>`.) · `wartend`

## Bau-Folge 83 — übernommene Wartestellungen (gegen den Baum halten)

- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage list`/`view`
  einmal.) · `wartend`
- **Red main** — der 41-Test/clippy/rustfmt-Fix am HEAD. (Schritt:
  `ci_manage view` des jüngsten `ci-check`; rot → rote Zelle.) · `wartend`
- **TE-Gate** — Gate-Verdikt; grün → Rename
  `arx_restricted_surrogate_conditional`. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — Verdikt. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und `--pdf-text` Type0/Identity-H
  ohne ToUnicode — kein Bau nötig. · `pending`

## Benchmark

- Delegationen: Gremium (`riss`-Verdikt + Tonungs-Bindung, pro/max), `grind-flash`
  (Punkt 1 CDN, Mechanik), `grind-pro` (Punkt 2 Native Tonung + Punkt 3 Messung).
  Kein Doppel-Lauf: die Routine-Klasse ist geschlossen (flash siegt), der harte
  Tonungs-Atom ging an pro. Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit:** `AGENTS.md` (0-Kanon-Hunk), `phi/sources.φ`
  (`weberin_verdicts`-Zeile), `src/archivar/main_flow.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `src/mathematikerin/omega.rs`, `src/mathematikerin/tests.rs`,
  `tools/measure/src/bin/weberin_verdicts_compiler.rs`,
  `.github/workflows/weberin-verdicts-cdn.yml` (neu),
  `docs/handover/handover-2026-09-18-bau-folge85.md` (neu), Move
  `handover-2026-09-18-bau-folge84.md` → `archiv/`.
- **Fremd (nicht anfassen):** `tools/measure/src/bin/silence_map_probe.rs` (fremd
  modifiziert), die `handover-2026-09-16-*`-Renames/Deletes,
  `src/mathematikerin/te.rs` (von forschung committet). Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
