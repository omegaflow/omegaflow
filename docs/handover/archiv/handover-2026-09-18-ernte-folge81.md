<!--
  title: Handover — Ernte-Folge 81 (Stand 2026-09-18)
  session: Ernte-Folge 81
  class: handover
  date: 2026-09-18
  sha256: 46363ab82f3db0a8246c5485505607e2fecb6339d9161df5f46455cb6e0d3a3b
  status: live
-->
# Handover — Ernte-Folge 81 (2026-09-18)

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

- **HEAD** `0aaa34e1` (Session-Start `ca54266b`, Fix-Commits `91741691`/`ca54266b`
  nach Handover-80-Header `60c15b6d`; der Baum lief fremd weiter — Entscheid
  `0aaa34e1`, Renames, `handover-2026-09-18-entscheid-folge49.md`,
  `forschung-folge79.md`); Safety-Net `refs/safety/1789718691`.
- **Postfach** — `post.md` leer. Letzter Ledger-Eingang `1789715019` (Brave-API-
  Schwellen-Alert, **kein Agenten-Eingang**); kein Ernte-Eingang.
- **CI** — `gedi_l2a` `35322402803`, `icesat2_atl03` `35322405744`,
  `swot_l2_lr_ssh` `35322408617` **failure** (0 Records, gemessen 2026-09-18T08:04Z,
  `ci_manage log`). Auto-Nachläufer `35322418662`/`35322420350`/`35322422042`
  failure. `rosetta_odf` `35312992622` in_progress (wartend); sonst fremde Linien.
  Zustand-Eintrag `docs/zustand/external-state.md` (CI-Status) ist aktuell.

## harvest — drei Compiler-Fixes gebaut, Re-Dispatch ausstehend

Die drei roten Läufe hatten **eine gemeinsame Wurzel**, gemessen am Baum + an der
Quelle (`curl -r` + `od`):
- **gedi_l2a/icesat2_atl03**: der `Hdf5WindowReader`-Refactor las das v1-Object-
  Header-Fenster mit `header_size` (24 B) statt `16 + header_size` (40 B) →
  `EndAtByte { off: 120 }` (gedi) / `{ off: 5096 }` (atl03). Der 16-B-Präfix
  (Version/Reserved/NumMessages/RefCount/HeaderSize/Reserved) war nicht
  eingerechnet; die GEDI-Bytes (`header_size = le_u32@8 = 24`, erste Message ab
  relativ 16, Daten bis relativ 40) belegen es. Fix: `src/archivar/hdf5.rs:505`
  `r.read(addr, (16 + header_size) as u64)` + Regressionstest
  `parse_fetch_reads_a_v1_object_header_beyond_base`.
- **swot_l2_lr_ssh**: die Datei ist 893 155 007 B, ihr Objektgraph reicht bis
  892 581 824 B — jenseits des festen 96-MiB-Fensters. Der Compiler parste noch mit
  `Hdf5File::parse(&w)` (eager walk über das Fenster). Fix: `parse_fetch` mit
  `g.read_range` (on-demand) für w1 und w2.
- Council-Punkt 4 umgesetzt: `Hdf5Note::AbsentAtByte { off }` trennt „keine Bytes
  an dieser Adresse" vom überladenen `EndAtByte`; `Hdf5WindowReader::read` nutzt es
  beim `None`-Fetch; `tools/utils/src/bin/hdf5_reader.rs` trägt die Zeile.
- `cargo check --all-targets` sauber (0 Fehler, 0 Warnungen). Unabhängig
  verifiziert von `grind-flash` (v1-Arithmetik + eigener `curl`-GEDI-Fetch:
  HTTP 206, 128 B; keine weiteren range-fetch-HDF5-Aufrufer ohne `parse_fetch`).

Dispatcht @`63878834`: `gedi_l2a` `35323850479`, `icesat2_atl03` `35323854366`,
`swot_l2_lr_ssh` `35323857410` — **Verdikt ausstehend.** (Schritt: Verdikt einmalig
lesen `ci_manage view <id>`; bei success `phi/harvest.φ` `asset fehlt`→`present` +
`note` size/sha256. **Nie pollen.**)

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

## rosetta_odf — Re-Dispatch (wartend)

- `harvest.yml` Zwei-Job-Fix gepusht (`6295dd5e`); der Re-Dispatch wartet auf den
  Abschluss von `35312992622` (alter Workflow, in_progress). (Schritt: Re-Dispatch
  `gh workflow run harvest.yml -f format=rosetta_odf` ohne `-f timeout`; Run-ID
  registrieren.)

## Council benannt, nicht blockierend

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

- HDF5-Fetch-Reader-Wurzel (hart): main-Session (flash-Tier) maß sie mit `curl -r`
  + `od` und setzte den Fix; `grind-flash` verifizierte unabhängig (v1-Arithmetik +
  eigener Fetch, `session_burn` $0.0168). **Erster gemessener Flash-Vertreter** der
  Novel-Parser-Klasse (Handover 80 trug noch keinen; das frühere HDF5-Atom lief
  `grind-max` ohne Flash-Baseline). Die CI-Verdikte der drei Formate bleiben der
  Schiedsrichter.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/hdf5.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`,
  `tools/utils/src/bin/hdf5_reader.rs`, `phi/harvest.φ`,
  `docs/handover/handover-2026-09-18-ernte-folge81.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge80.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`,
  `phi/blocked_sources.φ`, die `handover-2026-09-16-*`-Renames/Deletes,
  `handover-2026-09-18-{entscheid-folge48,49,forschung-folge78,79}.md`,
  `tools/harvest/src/bin/fugin_skymap_compiler.rs`, `opencode.json`,
  `phi/bindings/*.φ`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
