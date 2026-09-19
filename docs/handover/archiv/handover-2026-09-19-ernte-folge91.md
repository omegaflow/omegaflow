<!--
  title: Handover — Ernte-Folge 91 (Stand 2026-09-19)
  session: Ernte-Folge 91
  class: handover
  date: 2026-09-19
  sha256: 30afb574c6366531eeeafb92edd87f1a423946b2a72749dd18617b17d48c9db9
  status: live
-->
# Handover — Ernte-Folge 91 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19)

- **HEAD** `715219db` (== origin/main). Safety-Net `refs/safety/1789801162`.
- **Postfach** — neuester `state/mail/mail_ledger.φ`-Eingang `1789795811`
  (Rubin Obs Rubin-Forum AGN-Lightcurves, informativ); keine Sonden-Antwort.
  `docs/zustand/external-state.md` Postfach- + CI-Zeile fortgeschrieben.
- **CI** — `harvest-long` `35425365015` **success** (lro_trk_2009.bin present);
  `harvest` `35426168639`/`35427594463` success; `planetary-odf-cdn` success.
  **Gefixt:** `harvest` `35426123577` failure — exofop_compiler ohne `--out`
  (harvest.yml-Pfad); `workflow exofop-cdn.yml` im Register ergänzt, der
  Dispatcher routet nun dorthin.

## Source-Port — offene Arme (härtester undatiert zuerst)

- **Cassini RSS raw** `phi/blocked_sources.φ:21-23` — gemessen 2026-09-19:
  `co-ss-rss-1-scc1..14/sce1` = **open-loop RSR** (4260 B SFDU I/Q, 1000×32 bit,
  DSN 820-013) + **ODF TRK-2-18** (36 B); `co-ssa-rss-1-digr1..5/engr1..8`.
  `cassini_tnf` deckt nicht; `odf::parse_odf` deckt den ODF-Arm. (Schritt: Arm
  `cassini_rsr` bauen — SFDU-Header → I/Q — **oder** zuerst den ODF-Arm über
  `odf::parse_odf`.) `pending` (Bau-Duty).
- **gedi_l2a** `phi/harvest.φ` — `blockiert` (Code). Hang gemessen:
  `src/archivar/hdf5.rs` `gather_messages` (v1-Cont-Loop `:652`, v2 `:715`);
  eager `parse_fetch` traversiert den ganzen Objektgraphen, globaler Range-Guard
  fehlt. Post an bau (`docs/handover/post.md`). (Schritt: bau-Fix.)
- **icesat2_atl03** `phi/harvest.φ` — `blockiert` (Budget). Gemessen: der
  **ci_watchdog** cancelt (1598 s > 2× Median 71 s), nicht das Job-Timeout 360;
  215 Granules à ~100 min, `--limit 2` seriell ohne Offset. Post an bau. (Schritt:
  `--limit 1` + `--skip <k>` in `icesat2_atl03_compiler.rs`.)
- **FUGIN Bulk** `phi/blocked_sources.φ:39-41` — die 269 Cube-Assets des
  Bulk-Laufs sind noch nicht einzeln verifiziert. (Schritt: `gh release view
  fugin.nao.ac.jp` / Sniff der Cube-Assets.) `wartend`.

## Wartend (kein Auswahlpunkt)

- **Lasair-LSST** `external-state.md:23` — Re-Messung 2026-09-19: direct + Proton
  502, Wayback 200 ohne Snapshot; nächste Wiedervorlage / Banner-Wechsel. `wartend`.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ` —
  Anfragen offen. `wartend`.
- **Limadou PI-Freigabe** `ledger.φ` — per-act consent (Operator). `operator-gebunden`.
- **Queue-Korpora** `ledger.φ` — `--port` braucht das Operator-Wort für den lokalen
  Release-Binär-Lauf. `operator-gebunden`.

## Benchmark

- **flash-first:** CI-Log-Lesung + gedi/icesat-Diagnose über `grind-flash`; Cassini
  (Format-/Route-Urteil) + rosetta (Shard-Kuration) über `grind-pro`. Kein
  Doppel-Lauf, keine pro/max-Konkurrenz nötig.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/sources.φ` (rosetta 6 Shard-Blöcke + sha256, lro_trk
  url+sha256), `phi/harvest.φ` (exofop workflow-route, rosetta present, lro present,
  gedi/icesat-Notes), `phi/blocked_sources.φ` (Cassini-Note),
  `phi/pipeline/ledger.φ` (lro_trk verifiziert), `docs/handover/post.md`
  (Post an bau), `docs/zustand/external-state.md` (Postfach + CI), die neue
  `handover-2026-09-19-ernte-folge91.md`, `docs/handover/archiv/handover-2026-09-19-ernte-folge90.md`
  (verschoben).
- **Fremd (nicht anfassen):** `src/archivar/weberin_verdicts.rs`,
  `src/mathematikerin/omega.rs`, `src/mathematikerin/tests.rs`,
  `tools/measure/src/bin/perm_target_probe.rs`, die entscheid/forschung-Handover-Moves
  in `docs/handover/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Kein neuer Workflow in
dieser Session gebaut → kein Post-Push-Dispatch (die rosetta-Registrierung wirkt beim
nächsten `planetary-odf-cdn`-Lauf).
