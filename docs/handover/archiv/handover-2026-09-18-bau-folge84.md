<!--
  title: Handover — Bau-Folge 84 (Stand 2026-09-18)
  session: Bau-Folge 84
  class: handover
  date: 2026-09-18
  sha256: eb5f034d1f71f42c718f4c300395e76003c031886e2e155e212a10c27b02cfa1
  status: live
-->
# Handover — Bau-Folge 84 (2026-09-18)

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

## Weberin-Verdikt-Seitenkanal — offen (nächstes Atom)

- **`weberin_verdicts` → CDN.** Der Compiler (`weberin_verdicts_compiler`) schreibt
  `data/weberin_verdicts.bin`; die Manifestation ist eine Register-Pflicht.
  (Schritt: `--ci-mode`-Upload im Compiler + `url`-Zeile `format weberin_verdicts`
  in `phi/sources.φ` + CI-Workflow.) · `pending`
- **Native Tonung.** Das Weberin-Wort in die Tonungs-Optionen des nativen Pfads
  (Gremium Schritt 4; der Browser nennt den `riss`-Körper bereits).
  (Schritt: `src/mathematikerin/omega.rs` Diode/Tonung um das geladene
  `Vec<VerdictLine>` erweitern.) · `pending`
- **`stale`.** Ein Verdikt, dessen `weave_epoch` älter als das Fenster ist, als
  `stale` benennen (Register-Pflicht der Sonde, nie still weiterverwenden).
  (Schritt: `weberin_verdicts_compiler`-Alter gegen die Fensterbreite prüfen.) · `pending`
- **`riss` als vierter Zustand.** Im 0-Kanon benennen: `null-echt` / `absent` /
  `pending` / **`riss`** (anwesender Widerspruch, nie auf `absent`/`0.0` abgebildet).
  (Schritt: AGENTS.md Ethik-Abschnitt, mit Gate-Fixture.) · `pending`

## Bau-Folge 83 — übernommene Wartestellungen (gegen den Baum halten)

- **FUGIN-Bulk-Manifestation** — 270 Cubes; Abschluss, wenn die Assets auf
  `jvo.nao.ac.jp` liegen. (Schritt: `ci_manage list`/`view` einmal.) · `wartend`
- **Red main** — der 41-Test/clippy/rustfmt-Fix am HEAD. (Schritt: `ci_manage view`
  des jüngsten `ci-check`; rot → rote Zelle.) · `wartend`
- **TE-Gate** — Gate-Verdikt; grün → Rename `arx_restricted_surrogate_conditional`;
  rot → FN-Arm. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — Verdikt. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und `--pdf-text` Type0/Identity-H ohne
  ToUnicode — kein Bau nötig. · `pending`

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit:** `src/archivar/weberin_verdicts.rs` (neu),
  `src/archivar/relay.rs`, `src/archivar/main_flow.rs`, `src/archivar/mod.rs`,
  `tools/measure/src/bin/weberin_verdicts_compiler.rs` (neu),
  `static/{sensorium,radiator,sensorium.test,radiator.test,verdicts.test}.js|mjs`
  (neu), `static/constants.js`, `static/index.html`,
  `.github/workflows/ci-check.yml`, `docs/handover/handover-2026-09-18-bau-folge84.md`
  (neu), Move `handover-2026-09-18-bau-folge83.md` → `archiv/`.
- **Fremd (nicht anfassen):** `src/mathematikerin/te.rs`,
  `tools/measure/src/bin/{betti0_probe,silence_map_probe}.rs`, die
  ernte-/forschung-Handover-Moves, `phi/harvest.φ`, `phi/sources.φ`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
