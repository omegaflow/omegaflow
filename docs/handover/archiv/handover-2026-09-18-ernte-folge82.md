<!--
  title: Handover — Ernte-Folge 82 (Stand 2026-09-18)
  session: Ernte-Folge 82
  class: handover
  date: 2026-09-18
  sha256: 754e81fd0aec63f97226266b1ad494a312d675f49eee7205eca88cdb9ca03601
  status: live
-->
# Handover — Ernte-Folge 82 (2026-09-18)

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

- **HEAD** `427f029a` (Session-Start `fb6b62b4`; Fremdlinien committeten während
  der Session — forschung/bau). Safety-Net `refs/safety/1789719690`.
- **Postfach** — `post.md` leer; `state/mail/mail_ledger.φ` ohne Agenten-Eingang
  (letzte Eingänge 2026-08-25).
- **CI** gedi_l2a `35323850479` **failure**, icesat2_atl03 `35323854366`
  **failure** (beide 0 B granule, post Header-Fix); swot_l2_lr_ssh `35323857410`
  in_progress; rosetta über `planetary-odf-cdn` `35313968728` in_progress.
  Zustand-Eintrag `docs/zustand/external-state.md` fremd — nicht angefasst.

## gedi_l2a / icesat2_atl03 — neue Wurzel „0 B granule" (härtester undatiert)

Nach dem Header-Fix (16+header_size) liefern beide Compiler 0 Records, und die
gemeldete Granule-Größe `c.size` ist 0 (`0 B granule`) — eine Wurzel **upstream**
des HDF5-Readers (S3-Objektgröße / CMR-Metadaten), nicht die v1-Header-Trunkierung.
(Schritt: `c.size`-Herkunft im Compiler + S3-Listing/CMR-Metadaten mit EDL-Token
messen — `sfetch`/`curl` der Granule-URL mit Token, HTTP-Status + Content-Length;
Fehllog `ci_manage log 35323850479` / `35323854366`.)

## LRO utF harvest — erster jahr-gebundener Lauf dispatcht (wartend)

- Bindung gebaut: `workflow`-Feld in `harvest_reg.rs`; lro_trk-Block `workflow
  harvest-long`, `args --year 2009`, jahr-gebundenes Pattern, `timeout 8`; neue
  `.github/workflows/harvest-long.yml` (harvest.yml-Body); `harvest-dispatch.yml`
  routet per `workflow`-Feld (Default `harvest.yml`); `lro_trk_compiler --year
  <YYYY>` filtert am YYYYDDD-Verzeichnis, Asset-Name jahr-gebunden
  `lro_trk_<year>[_t<lo>_<hi>].bin`. Rat-Entscheid Option B (Jahr in `args`).
  Erstes Jahr = 2009 (kleinstes YYYY im Baum, `LRO_CO/2009169`).
- **Offen:** Verdikt des ersten `harvest-long`-Laufs `35325786893` (queued
  2026-09-18T08:43Z, format lro_trk, args --year 2009; dispatcht nach dem Push).
  (Schritt: `ci_manage view 35325786893` einmalig; **nie pollen**.) Bei success:
  `shard` im Register = gemessene Asset-Zahl; das nächste Jahr als eigenes Atom
  binden (`args --year <n>` ändern — die `phi/harvest.φ`-Änderung löst
  `harvest-dispatch` aus).
- Rat-Konfounder: Idempotenz-Pattern jahr-gebunden halten; `args`/`workflow`
  innerhalb 6 Zeilen von `format` (Dispatcher-`-U6`-Diff); Void-Jahre vor dem
  Binden messen (0 honored).

## swot_l2_lr_ssh — Verdikt ausstehend (wartend)

`35323857410` in_progress. (Schritt: `ci_manage view 35323857410` einmalig; bei
success `phi/harvest.φ` asset fehlt→present + `note` size/sha256.)

## rosetta_odf — Re-Dispatch (wartend)

Register-Befund gegen den Baum: rosetta wird von `planetary-odf-cdn.yml` gefahren,
**nicht** `harvest.yml` (Handover-81:101 falsch); Lauf `35313968728` in_progress.
(Schritt: Verdikt einmalig `ci_manage view 35313968728`; bei failure
`gh workflow run planetary-odf-cdn.yml`.)

## Council benannt, nicht blockierend

Die HTTP-Reach-Zahl je Granule wächst (überlappende 64-B-then-Span-Fetches
verfehlen den Cache per Containment) — langsamere Harvests, korrekte Records;
Lauf-Zeit im nächsten CI-Ergebnis benennen.

## Werkzeug-Wrapper — AGENTS.md-Satz (fremd)

`AGENTS.md` ist fremd-modifiziert; der Satz „`bin/archive_search` rebuilds only
when stale …" gehört der Linie, die `AGENTS.md` besitzt.

## Benchmark

Jahr-Bindungs-Architektur (hart): der Rat (pro/max) entschied Option B (Jahr in
`args`); die Umsetzung geteilt — `grind-flash` (Register/Workflows, mechanisch)
+ `grind-pro` (Compiler `--year`, Urteil). Keine Doppelung, keine neue
Benchmark-Zeile; die CI-Verdikte der drei Formate bleiben der Schiedsrichter.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/utils/src/bin/harvest_reg.rs`,
  `.github/workflows/harvest-long.yml`, `.github/workflows/harvest-dispatch.yml`,
  `tools/harvest/src/bin/lro_trk_compiler.rs`, `phi/harvest.φ`,
  `docs/handover/handover-2026-09-18-ernte-folge82.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge81.md`).
- **Fremd (nicht anfassen):** `src/archivar/galileo_odr.rs`,
  `src/archivar/las/mod.rs`, `src/archivar/voyager_odr.rs`,
  `tools/utils/src/bin/omega_sh.rs`, `docs/zustand/external-state.md`, die
  `handover-2026-09-16-*`-Renames/Deletes,
  `handover-2026-09-18-{entscheid-folge49,forschung-folge79}.md`,
  `opencode.json`, `phi/bindings/*.φ`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
