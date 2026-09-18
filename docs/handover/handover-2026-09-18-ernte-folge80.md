<!--
  title: Handover — Ernte-Folge 80 (Stand 2026-09-18)
  session: Ernte-Folge 80
  class: handover
  date: 2026-09-18
  sha256: 8a136234504b736de8000a491706a1006843c032e0b582d47feebaae60448850
  status: live
-->
# Handover — Ernte-Folge 80 (2026-09-18)

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

- **HEAD** `60c15b6d` bei Session-Ende (Session-Start `d8a8ed3e`; der Baum lief
  weiter — bau `1a09d8e9`/`1d38b04e`/`60c15b6d`, entscheid `b67f5cae`/`65cf5341`,
  forschung `3471b051`, chore `c7af0827`). Safety-Net `refs/safety/1789718238`.
- **Postfach** — `post.md` trug die LRO-Übergabe der Forschung-Linie (gefaltet,
  Zeile gelöscht). Neuester Ledger-Eingang `1789715019` (Brave-API-Schwellen-Alert,
  **kein Agenten-Eingang**), davor `1789689115` (Rubin-Forum-Migration).
- **CI** — `harvest` `35317120978`/`35317118656`/`35317116366` failure (Ernte-Linie);
  `rosetta_odf` `35312992622` in_progress (wartend); sonst fremde Linien.

## LRO utF harvest — Fix in einem Atom (härtester undatierter Punkt)

Gefaltet aus `post.md` (Forschung-Folge 78; Rat-Verdikt A+D, 2026-09-18). Wurzel:
`harvest.yml` fährt 74 Formate in einem Workflow (erfolgreiche Läufe 46–142 s);
der ci_watchdog killt jeden Lauf > 2× Median (48 s) — lro_trk ist ein ungebundener
Voll-Crawl (50 790 .TRK in ~61 min, `FILE_CAP 100_000`, kein `args`/`timeout`),
selbstverriegelnd. Fix (1)–(5):
1. `workflow` als bekanntes Feld in `tools/harvest/src/bin/harvest_reg.rs` +
   `workflow harvest-long` im lro_trk-Block (`phi/harvest.φ:45`) — Werkzeug und
   Register im selben Commit.
2. `.github/workflows/harvest-long.yml` mit dem harvest.yml-Body.
3. `harvest-dispatch.yml` routet Formate mit `workflow`-Feld dorthin.
4. Compiler `--year <n>` — Baum `LRO_<ST>_<n>/YYYYDDD/LSUTDF_<st>_<f3>_<YYYY>_<DDD>_<hhmm>.TRK`,
   Filter am YYYYDDD-Verzeichnis.
5. Ersten gebundenen Lauf dispatchen, nie pollen. Volle Reihe, Jahr als Bindeeinheit;
   E (Parallel-Fetch) verweigert. `bin/ci_watchdog.sh` unverändert.
Nach dem ersten Erfolg: `shard` im Register = gemessene Asset-Zahl.
(Schritt: `harvest_reg.rs` lesen, Feld ergänzen, dann (2)–(5).)

## harvest — drei Compiler-Lücken: Fix gebaut, CI-Verdikt ausstehend

- **gedi_l2a + icesat2_atl03** — Wurzel gemessen: beide Compiler parsten HDF5 aus
  einem festen Prefix-Fenster (32 MiB, eine Eskalation auf 96 MiB), während der
  Parser bei absoluten Datei-Adressen indizierte; GEDI-Root-Object-Header bei
  263 439 695 B → `EndAtByte` → 0 records; ATL03-Chunk-Btrees > 32 MiB → still
  0 Datasets. Fix: `Hdf5WindowReader` + `Hdf5File::parse_fetch` (`src/archivar/hdf5.rs`);
  Object-Header, Continuation-Chunks, Fractal-Heaps, Link-Btrees, Symtabs, Dense-Attrs
  und Chunk-Btrees lesen Fenster an ihrer Adresse; `parse` delegiert mit No-op-Closure
  (Semantik unverändert). GEDI/ATL03 nutzen `PREFIX_WINDOW = 1<<9` (512 B,
  Superblock ≤ 72 B). Council-Revisionen eingebaut: fetch-Längenvertrag,
  symtab-NUL-Pflicht, chunked-Bounds-Check, Test
  `parse_fetch_resolves_object_header_beyond_base`.
- **swot_l2_lr_ssh** — Wurzel gemessen: der protected Bucket
  `podaac-swot-ops-cumulus-protected` verweigert S3-SigV4 GetObject/ListBucket 403
  (kein Signing-/Scope-Bug). Route: CMR-`/data#`-Link →
  `archive.swot.podaac.earthdata.nasa.gov/<bucket>/<key>.nc` + EDL-Bearer →
  303 CloudFront → 206. Fix: `fetch_bearer_range` (`range.rs`), `data_url` (Option)
  aus CMR, Bearer-Route für protected Granules; public/Listing bleiben SigV4.
  Register: `phi/sources.φ:1542`-note um die S3-vs-HTTPS-Messung ergänzt.
- **Verdikt ausstehend:** `cargo check --all-targets` sauber (0 Fehler, 0 Warnungen);
  `cargo test` des neuen Tests grün. Dispatcht @`91741691`:
  `gedi_l2a` `35322402803`, `icesat2_atl03` `35322405744`,
  `swot_l2_lr_ssh` `35322408617` — Ergebnis liest die nächste Session einmalig
  (`ci_manage view <id>`), bei success `phi/harvest.φ` `asset fehlt`→`present` +
  `note` (size/sha256).

## rosetta_odf — Re-Dispatch (wartend)

- `harvest.yml` Zwei-Job-Fix gepusht (`6295dd5e`); der Re-Dispatch wartet auf den
  Abschluss von `35312992622` (alter Workflow, in_progress). (Schritt: Re-Dispatch
  `gh workflow run harvest.yml -f format=rosetta_odf` ohne `-f timeout`; Run-ID
  registrieren.)

## Council benannt, nicht blockierend

- `EndAtByte` ist überladen — es sagt „Datei endet", wo die Fetch-Wahrheit „keine
  Bytes an dieser Adresse" ist; ein eigener `AbsentAtByte`-Variant wäre der ehrliche
  Diagnose-Name. (Schritt: Variant einführen, Aufrufer umstellen.)
- Die HTTP-Reach-Zahl je Granule wächst (überlappende 64-B-then-Span-Fetches
  verfehlen den Cache per Containment) — langsamere Harvests, korrekte Records;
  Lauf-Zeit im nächsten CI-Ergebnis benennen.

## Werkzeug-Wrapper — AGENTS.md-Satz (fremd)

- `AGENTS.md` ist fremd-modifiziert; der Satz „`bin/archive_search` rebuilds only
  when stale …" gehört der Linie, die `AGENTS.md` besitzt.

## Waiting (kein Auswahlpunkt)

- `ulysses_atdf_x` X-Ref-Fenster (Auslöser = nächstes `atdf`-Atom);
  `lro_utf`/`§4 census`/`bepicolombo`/`rosetta ungelaufene Pfade`/`auto-dispatch`/
  fünf Familien-Blöcke — Auslöser unverändert.

## Benchmark

- HDF5-Parser-Atom (hart): `grind-max` — der billigste tragende Vertreter des
  bestätigten Plans. **Kein Flash-Baseline-Lauf** — die Novel-Parser-Klasse trägt
  damit noch keinen gemessenen Sieger; ein künftiges vergleichbares Atom zieht den
  Flash-Doppellauf nach.
- SWOT-Route (Judgment): `grind-pro`. Routine (Log-Root-Cause): `grind-flash`.
- Architektur (HDF5 fetch-reader): `council` — Verdikt revise→commit, vier
  Revisionen im selben Atom.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/hdf5.rs`, `src/archivar/range.rs`,
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`, `phi/sources.φ`,
  `docs/zustand/external-state.md`, `docs/handover/post.md`,
  `docs/handover/handover-2026-09-18-ernte-folge80.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge79.md`).
- **Fremd (nicht anfassen):** `opencode.json`, `phi/bindings/*.φ`,
  `docs/concepts/bindings-*.md`, die `handover-2026-09-16-*`-Renames/Deletes,
  `handover-2026-09-18-{entscheid-folge46,bau-folge78,forschung-folge78}.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
