<!--
  title: Handover — Ernte-Folge 85 (Stand 2026-09-18)
  session: Ernte-Folge 85
  class: handover
  date: 2026-09-18
  sha256: c79dfc77f804fbf86cc34e06c572b4ff9390ae534b92db40258f2eddbce81e05
  status: live
-->
# Handover — Ernte-Folge 85 (2026-09-18)

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

- **HEAD** `faa6c173` == `origin/main` (Fast-Forward, Push steht). Safety-Net
  `refs/safety/1789730290`.
- **Postfach** — neuer Ledger-Eingang `1789729151`: Luciano Iess (BepiColombo
  MORE) — Cruise-Daten werden zur Wissenschaftsphase freigegeben, kein
  Zwischenzugang. Gehört der `forschung`-Linie (`post.md` `bc_mpo_more`), nicht
  gefaltet. Kein eigener Post.
- **CI** — der Push löste `harvest-dispatch` `35338058101` (success) +
  `auto-dispatch` `35338058143` (success) aus; `bc-mpo-mag-cdn` `35338074317`
  **success**. Zustand-Eintrag `docs/zustand/external-state.md` fremd — nicht
  angefasst.

## gedi_l2a / icesat2_atl03 — Wurzel `dataset absent` (härtester undatiert, wartend)

Der Push hat den Re-Dispatch automatisch ausgelöst: gedi `35338097066`
(in_progress), atl03 `35338099137` (in_progress), beide @`faa6c173`. Der
harvest-Job-Log ist bis Job-Abschluss HTTP 404 — **HeaderDiag noch nicht lesbar**.
Der Fix (HDF5 v1 object-header continuation, `HeaderDiag` in beiden Compilern) ist
gebaut und gepusht. Status `wartend` (Trigger Run-Abschluss).
- **Schritt:** `ci_manage log 35338097066` / `ci_manage log 35338099137`
  **einmalig** — `HeaderDiag` (version/continuation/symtab/link_info) lesen.
  Bleibt `dataset absent` bei v2-Header ohne `link_info`, ist die nächste Schicht
  `read_links_modern`/`parse_fractal_heap`. **Nie pollen.**

## LRO utF harvest — Re-Dispatch (wartend)

`35332922040` war `completed cancelled` (Log: `The operation was canceled.`, mitten
im `lro_trk_compiler`; `cancel-in-progress: false` — keine Concurrency-Supersession).
Neu dispatcht: `35339151004` (`harvest-long`, format `lro_trk`, queued
2026-09-18T11:20:32Z). Status `wartend` (Trigger Run-Abschluss).
- **Schritt:** `ci_manage view 35339151004` **einmalig**; bei success `shard` ins
  Register = gemessene Asset-Zahl, nächstes Jahr als eigenes Atom binden.

## rosetta_odf (wartend)

`35313968728` (`planetary-odf-cdn`) in_progress, seit 06:13:44Z ohne Status-Update
(Messung Folge 84). Status `wartend` (Trigger Run-Abschluss).
- **Schritt:** Verdikt einmalig `ci_manage view 35313968728`; bei failure
  `gh workflow run planetary-odf-cdn.yml`.

## HTTP-Reach-Zahl je Granule (Council, nicht blockierend)

Überlappende 64-B-then-Span-Fetches verfehlen den Cache per Containment —
langsamere Harvests, korrekte Records; Lauf-Zeit im nächsten CI-Ergebnis nennen.

## Benchmark

Punkte 1/2/3 (Run-Log-Zuordnung, bc_mpo_mag-Asset-Messung, LRO-Re-Dispatch) liefen
`grind-flash` — Routine, kein hartes Atom, kein Doppel-Lauf, kein Benchmark-Eintrag.
Schiedsrichter ist der nächste CI-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/sources.φ` (bc_mpo_mag `sha256` + Note),
  `phi/harvest.φ` (bc_mpo_mag `asset present` + Note),
  `docs/handover/handover-2026-09-18-ernte-folge85.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge84.md`).
- **Fremd (nicht anfassen):** `.github/workflows/ci-check.yml`,
  `static/index.html`, `static/radiator.js`, `static/radiator.test.mjs`,
  `static/sensorium.js`, `static/sensorium.test.mjs`, `src/archivar/main_flow.rs`,
  `src/archivar/mod.rs`, `src/archivar/relay.rs`, `src/mathematikerin/te.rs`,
  `static/constants.js`, die `handover-2026-09-16-*`-Moves,
  `docs/zustand/external-state.md`, `opencode.json`, `phi/bindings/*.φ`, die
  `tools/measure/src/bin/{betti0_probe,weberin_verdicts_compiler}.rs`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
