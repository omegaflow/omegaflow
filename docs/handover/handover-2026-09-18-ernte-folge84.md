<!--
  title: Handover — Ernte-Folge 84 (Stand 2026-09-18)
  session: Ernte-Folge 84
  class: handover
  date: 2026-09-18
  sha256: 2c242c7f374710a493e8ce9c1b4477cd152e3a798adec695bc4e2bdae981a707
  status: live
-->
# Handover — Ernte-Folge 84 (2026-09-18)

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

- **HEAD** `642ff12c` (Session-Start nach Neustart). Safety-Net `refs/safety/1789728113`.
- **Postfach** — `post.md` trug die bc_mpo_mag-Zeile (abgeholt, gefaltet, gelöscht);
  die forschung-Zeile (`bc_mpo_more`) bleibt fremd. `mail_ledger.φ` ohne Agenten-Eingang.
- **CI** — LRO `35325786893` cancelled; swot `35323857410` success (7679 records);
  rosetta `35313968728` (planetary-odf-cdn) weiter in_progress; fugin-cdn-Flut fremd.
  Zustand-Eintrag `docs/zustand/external-state.md` fremd — nicht angefasst.

## gedi_l2a / icesat2_atl03 — Wurzel `dataset absent` (härtester undatiert)

Messung 2026-09-18 (runs `35330768755` atl03 / `35330766271` gedi, head `21590a43`):
beide Compiler lesen die CMR-Granule-Größe korrekt (263453671/1228910100 B GEDI,
427819008/1912602624 B ATL03), aber **jedes** Beam-/gt-Dataset (`BEAM0000/delta_time`,
`gt1l/heights/delta_time`) meldet `dataset absent` → 0 records. Neue Stufe damit:
`root.links` bleibt leer.
Fix gebaut: `gather_messages` in `src/archivar/hdf5.rs` verarbeitet jetzt auch
**v1-Objekt-Header-Continuation-Blöcke** (`v1_messages` + `cont_target` mit
offset/length-Size), liefert `HeaderDiag` (version, msgs_initial/cont, cont_blocks,
symtab_found, link_info_found); `Hdf5File::root_header_diag()`. Beide Compiler geben
den HeaderDiag mit der absent-Zeile aus. Test
`v1_object_header_reads_symbol_table_from_continuation_block` (synthetisch).
`cargo check --all-targets` + `-p omegaflow-harvest --bins --tests` 0/0.
- **Offen:** Re-Dispatch nach dem Push (`harvest-dispatch` auf `tools/harvest/src/bin/**`).
  (Schritt: `ci_manage log <neuer gedi/atl03-run>` **einmalig** — die `HeaderDiag`
  nennt version/continuation/symtab; bleibt `dataset absent` bei v2-Header ohne
  `link_info`, ist die nächste Schicht `read_links_modern`/`parse_fractal_heap`.
  **Nie pollen.**)

## bc_mpo_mag — Compiler + Register + Workflow gebaut, Manifestation offen (wartend)

Quelle offen (`phi/blocked_sources.φ:29`): BepiColombo MPO-MAG derived. Asset via
PSA-TAP `data?retrieval_type=PRODUCT` anonym, ZIP 853331 B (sha256
**nicht-deterministisch** — pro Abruf neu erzeugt), `.tab` 4828824 B sha256
`8d7e6007…` 30954 rows, 11 Spalten kommasepariert (0 Zeit, 2–4 Pos km ECLIPJ2000,
5–7 B nT). Gebaut: `src/archivar/bc_mpo_mag.rs` (7×f64, MAGIC `BCM1`),
`tools/harvest/src/bin/bc_mpo_mag_compiler.rs` (ZIP→`unzip`→`.tab`),
`phi/sources.φ`-Eintrag (psa.esa.int), `phi/harvest.φ`-Block, `.github/workflows/bc-mpo-mag-cdn.yml`.
- **Offen:** Workflow nach dem Push dispatchen und Asset messen.
  (Schritt: `gh workflow run bc-mpo-mag-cdn.yml`; Run-ID registrieren; Asset
  size/sha256 des `.bin` per `gh release view psa.esa.int` einmalig lesen.)
  `field`/`at`-Semantik im sources.φ-Eintrag ist Entwurf (gegen `sources-v2-spec`
  finalisieren, falls der Loader die Feldnamen bindet).

## LRO utF harvest — Re-Dispatch (wartend)

`35332922040` (harvest-long, format lro_trk, args --year 2009) queued, dispatcht
2026-09-18T10:05:34Z. (Schritt: `ci_manage view 35332922040` einmalig; bei success
`shard` ins Register = gemessene Asset-Zahl, nächstes Jahr als eigenes Atom binden.)

## rosetta_odf (wartend)

`35313968728` (`planetary-odf-cdn`) in_progress, seit 06:13:44Z ohne Status-Update.
(Schritt: Verdikt einmalig `ci_manage view 35313968728`; bei failure
`gh workflow run planetary-odf-cdn.yml`.)

## HTTP-Reach-Zahl je Granule (Council, nicht blockierend)

Überlappende 64-B-then-Span-Fetches verfehlen den Cache per Containment —
langsamere Harvests, korrekte Records; Lauf-Zeit im nächsten CI-Ergebnis nennen.

## AGENTS.md-Wrapper-Satz (fremd)

`AGENTS.md` ist fremd-modifiziert; der Satz „`bin/archive_search` rebuilds only
when stale …" gehört der Linie, die `AGENTS.md` besitzt.

## Benchmark

Punkt 1 (`dataset absent`-Diagnose+Fix) lief `grind-max` (hartes Parser-Atom);
Punkt 4 (`bc_mpo_mag`-Compiler) `grind-pro` (Urteil Source-Port); Punkte 2/3
(swot-Verify, LRO-Dispatch) `grind-flash` (Routine). Kein Doppel-Lauf gegen flash
bei den harten Atomen, kein Benchmark-Eintrag; Schiedsrichter ist der nächste
CI-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/hdf5.rs`, `src/archivar/mod.rs`,
  `src/archivar/bc_mpo_mag.rs`,
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs`, `phi/harvest.φ`,
  `phi/sources.φ`, `.github/workflows/bc-mpo-mag-cdn.yml`,
  `docs/handover/post.md` (nur die bc_mpo_mag-Zeile),
  `docs/handover/handover-2026-09-18-ernte-folge84.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge83.md`).
- **Fremd (nicht anfassen):** `.github/workflows/ci-check.yml`, `static/index.html`,
  `static/radiator.js`, `static/radiator.test.mjs`, `static/sensorium.js`,
  `static/sensorium.test.mjs`, die `handover-2026-09-16-*`-Moves,
  `docs/zustand/external-state.md`, `opencode.json`, `phi/bindings/*.φ`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
