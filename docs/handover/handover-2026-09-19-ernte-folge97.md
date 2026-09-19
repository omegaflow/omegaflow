<!--
  title: Handover — Ernte-Folge 97 (Stand 2026-09-19)
  session: Ernte-Folge 97
  class: handover
  date: 2026-09-19
  sha256: 0ce3dbeab7ab431ca799b3237d27eb6b57324528eee1677ed9762134826b5ffa
  status: live
-->
# Handover — Ernte-Folge 97 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Folge 97)

- **HEAD** `f3745142` (== origin/main). Safety-Net `refs/safety/1789848085` (Start).
- **Postfach** — `post.md` leer (keine ernte-Zeile); `external-state.md:20`: jüngster
  Ledger-Eingang `1789795811` (Rubin-Forum, informativ); keine Nachricht an ernte.
- **CI** — Watchdog-Snapshot 21:25Z: active `ci-check` `35462513268`, `measure-gates`
  `35457737916`, `health-check` `35451454869`, `te-gate` `35451283398`. `ci_manage list`
  20:08Z: pending `hyperscanning-te` `35466436802`, `ci-check` `35466472052`,
  `te-gate` `35462518676`; in_progress `allwise-cdn` `35464786486`; success
  `fmt-apply` `35464780631`, `harvest-dispatch` `35463566653`, `mars-dust-cdn`
  `35461031672`; failure `ci-check` `35462513268`, `fmt-apply` `35462507189`;
  cancelled `cassini-odf-cdn` `35455835693` (Watchdog 5913 s > 2×Median 381 s).

## Source-Port — offene Arme

- **Cassini ODF + RSR** `phi/harvest.φ:10-29`, `phi/sources.φ:6919/6928`,
  `phi/blocked_sources.φ:21-23` — `asset fehlt`. Gemessene Ursache des Dauer-Cancel:
  der **serielle** Listing-Crawl brauchte 73 min (15 Volumes, 258 ODF-Dateien;
  `volumes` 17:07:59Z → `files` 18:21:19Z), der Watchdog cancelt bei 2×Median
  381 s/438 s aus den leeren Erstläufen. **Gebaut (2026-09-19):** beide Compiler
  crawlen parallel+geprunt (16 Threads, `visited`-Dedup, Skip von rsr/tlm/158/
  ancillary) und brechen bei jedem Listing-/Fetch-/Parse-Fehler hart ab (kein
  Teil-Asset); `cargo check -p omegaflow-harvest --bins` 0/0. **Schritt:** der
  `harvest-dispatch` beim Push dispatcht `cassini-odf-cdn`/`cassini-rsr-cdn`
  automatisch; Ergebnis aus Watchdog-Snapshot / `ci_manage view <id>`; grün →
  Größe/sha256 messen, `sources.φ`-`sha256` + `harvest.φ` `asset present`. `wartend`.
- **OpenNeuro-Ingest** `phi/pipeline/ledger.φ:122-136` (`ausstehend`) — ds007822
  (EEGLAB `.set` MAT-v5: `openneuro_compiler` trägt `DATASET` hart `ds005034` →
  `--dataset`-Argument, dann Probe), ds007471 (BrainVision `.vhdr/.eeg/.vmrk`-Arm),
  ds008192 (SNIRF/HDF5-Arm); ds004103 (BOLD-fMRI, kein TE-Pfad) → per Messung
  `declined`. **Schritt:** `openneuro_compiler --dataset ds007822` parametrisieren,
  dann `--probe` über `openneuro-cdn.yml`. `ernte`.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20` —
  `blockiert` (extern). Re-Messung 2026-09-19 abends: Root je 200, sync-QUERY beider
  VOTable-`ERROR` `connection to server at "localhost" port 5432 failed: Connection
  refused`. **Schritt:** Re-Check (`archive_search --verdict` + sync-QUERY), Trigger
  Backend-Erholung. `blockiert`.
- **gedi_l2a** `phi/harvest.φ:57-65` — `blockiert` (Code, hdf5.rs fremd) → an bau.
- **icesat2_atl03** `phi/harvest.φ:75-83` — `blockiert` (Budget); ein fremder Arm
  (`tools/harvest/src/bin/icesat2_atl03_compiler.rs`) ist im Baum in Arbeit → an bau.

## Wartend (kein Auswahlpunkt)

- **Cassini** (s. o.) — Re-Dispatch beim Push; Auslöser: Run-Abschluss.
- **allwise-cdn** `35464786486` (scheduled, stündlich) — `allwise_coverage.fp01`
  noch 404 (`archive_search --sniff`); bei Abschluss CDN-Manifestations-Pflicht
  (`sources.φ`-Registrierung fehlt). **Schritt:** Watchdog-Snapshot / `ci_manage view
  35464786486`. `wartend`.
- **Lasair-LSST** `external-state.md:23` — direct+Proton 502, Wayback 200 ohne Snapshot.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ` — Anfragen offen.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:30-33` — Freigabe ~April (PSA/Iess).
- **Limadou PI-Freigabe** `ledger.φ:26-28` — per-act consent. `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:82-120` — `--port` braucht das Operator-Wort.
  `operator-gebunden`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — maxTime unverändert
  `2026-07-30T23:30:00Z`. Nächste Re-Messung **2026-10-19**. `termin`.

## Benchmark

- Kein pro/max-Lauf. Die Cassini-Diagnose (Watchdog-Log `/tmp/opencode/ci_watchdog.log`
  + Compiler-Codepfad + `ci_manage log --all`) lief in-Session; `cargo check -p
  omegaflow-harvest --bins` lokal, 0/0.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/harvest/src/bin/cassini_odf_compiler.rs`,
  `tools/harvest/src/bin/cassini_rsr_compiler.rs`, `phi/harvest.φ`, `phi/sources.φ`,
  `phi/blocked_sources.φ`, die neue `handover-2026-09-19-ernte-folge97.md`, der Move
  `docs/handover/archiv/handover-2026-09-19-ernte-folge96.md`.
- **Fremd (nicht anfassen):** `tools/harvest/src/bin/icesat2_atl03_compiler.rs` (M),
  die drei `handover-2026-09-16-*`-Moves, `docs/zustand/external-state.md`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
