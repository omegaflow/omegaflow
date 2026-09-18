<!--
  title: Handover — Ernte-Folge 83 (Stand 2026-09-18)
  session: Ernte-Folge 83
  class: handover
  date: 2026-09-18
  sha256: 365d5f5a1bf71c422561a8da27c1caa4c11fee25ec2c5f5ae9945bb5ced772fe
  status: live
-->
# Handover — Ernte-Folge 83 (2026-09-18)

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

- **HEAD** `07c8d6a7` (Session-Start). Safety-Net `refs/safety/1789721053`.
- **Postfach** — `post.md` leer; `state/mail/mail_ledger.φ` ohne Agenten-Eingang.
- **CI** Snapshot 2026-09-18T10:10Z: aktiv ci-check `35321200805`, harvest
  `35316446908`, allwise-cdn `35315711090`, planetary-odf-cdn `35313968728`;
  zahlreiche harvest-Fehlläufe (35322422042 u. a.). Zustand-Eintrag
  `docs/zustand/external-state.md` fremd — nicht angefasst.

## gedi_l2a / icesat2_atl03 — 0 records, Wurzel neu verortet (härtester undatiert)

Messung 2026-09-18: `c.size` war im CMR-Pfad **hartkodiert 0** (gedi_l2a_compiler
:848, icesat2_atl03_compiler :764) — kein upstream-S3/CMR-Gap. CMR liefert
`granule_size` (HTTP 200), die Granule-Route mit EDL-Token HTTP 206
`…/263454103` B (GEDI) bzw. `…/1912602624` B (ATL03). `c.size` gate't keinen
Fetch → die Handover-82-Hypothese „Wurzel upstream des HDF5-Readers" ist
widerlegt; die 0-Records-Wurzel liegt in der Datenextraktion (stille `None`).
Fix gebaut: `CmrGranule.size: Option<u64>` + `granule_size`-Parse (plausibility
`is_finite && > 0`), eprintln trennt echte Größe / „size unread". Instrumentiert:
`ChunkReadDiag`/`read_chunk_diag` in `src/archivar/hdf5.rs` (read_chunk-Semantik
unverändert) + `first_values`/`rh98_column` in beiden Compilern benennen Dataset
+ Stufe. `cargo check` 0 Fehler / 0 Warnungen.
- **Offen:** Re-Dispatch läuft nach dem Push an (`harvest-dispatch` auf
  `tools/harvest/src/bin/**`; gedi + atl03 asset fehlt). (Schritt: `ci_manage log
  <neuer gedi/atl03-run>` **einmalig** — die Stufe (`chunk data fetch void` |
  `chunk index read` | `chunk not found` | `dataset absent` | …) benennt die
  Wurzel; **nie pollen**.) Bleibt `chunk data fetch void` ohne Transport-Ursache:
  `GranuleFetch::range`/`fetch_range` instrumentieren (HTTP-Status vs. veraltete
  B-Tree-Adresse) — eigene Datei der Ernte-Linie.

## LRO utF harvest — erster jahr-gebundener Lauf (wartend)

`35325786893` (harvest-long, args --year 2009) in_progress, head `4554c40d`.
(Schritt: `ci_manage view 35325786893` einmalig; bei success `shard` ins Register
= gemessene Asset-Zahl, nächstes Jahr als eigenes Atom binden.)

## swot_l2_lr_ssh (wartend)

`35323857410` in_progress. (Schritt: `ci_manage view 35323857410` einmalig; bei
success `phi/harvest.φ` asset fehlt→present + `note` size/sha256.)

## rosetta_odf (wartend)

`35313968728` (`planetary-odf-cdn`) in_progress, seit 06:13:44Z ohne
Status-Update. (Schritt: Verdikt einmalig `ci_manage view 35313968728`; bei
failure `gh workflow run planetary-odf-cdn.yml`.)

## HTTP-Reach-Zahl je Granule (Council, nicht blockierend)

Überlappende 64-B-then-Span-Fetches verfehlen den Cache per Containment —
langsamere Harvests, korrekte Records; Lauf-Zeit im nächsten CI-Ergebnis nennen.

## AGENTS.md-Wrapper-Satz (fremd)

`AGENTS.md` ist fremd-modifiziert; der Satz „`bin/archive_search` rebuilds only
when stale …" gehört der Linie, die `AGENTS.md` besitzt.

## Benchmark

0-Records-Wurzel (hart): `grind-pro` (Diagnose `c.size` + Wurzel-Verortung) und
`grind-pro` (Instrumentierung) — kein Doppel-Lauf gegen flash, kein
Benchmark-Eintrag. Die CI-Logs der re-dispatchten gedi/atl03-Läufe sind der
Schiedsrichter.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/hdf5.rs`,
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`, `phi/harvest.φ`,
  `docs/handover/handover-2026-09-18-ernte-folge83.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge82.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, die
  `handover-2026-09-16-*`-Renames/Deletes,
  `handover-2026-09-18-entscheid-folge49.md`,
  `src/archivar/galileo_odr.rs`, `src/archivar/las/mod.rs`,
  `src/archivar/voyager_odr.rs`, `tools/utils/src/bin/omega_sh.rs`,
  `opencode.json`, `phi/bindings/*.φ`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
