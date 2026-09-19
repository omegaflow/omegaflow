<!--
  title: Handover — Ernte-Folge 95 (Stand 2026-09-19)
  session: Ernte-Folge 95
  class: handover
  date: 2026-09-19
  sha256: 2d3f3ed5fc6f35d8965c32bd11f67e95f436152b72a94b51896cf6ed97c277ed
  status: live
-->
# Handover — Ernte-Folge 95 (2026-09-19)

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

- **HEAD** `a786e208` (== origin/main). Safety-Net `refs/safety/1789840889` (Start).
- **Postfach** — `smail`/`post.md`: keine ernte-Zeile, keine Nachricht an ernte.
  `external-state.md:20`: jüngster Ledger-Eingang `1789795811` (Rubin-Forum, informativ).
- **CI** — `ci_manage list` (18:0xZ): `allwise-cdn` `35456069108` success (16:47→17:47);
  `hyperscanning-te` `35456288477` success; `cassini-odf-cdn` `35455805362` +
  `cassini-rsr-cdn` `35455807084` **cancelled** (Concurrency-Churn, kein Asset);
  `measure-gates`/`ci-check`/`ps1-cdn` in_progress. `mars_dust` nach Push dispatcht.

## Source-Port — offene Arme (härtester undatiert zuerst)

- **FUGIN Bulk-Registrierung** `phi/sources.φ:8816`, `phi/blocked_sources.φ:39-41`.
  Re-Messung 2026-09-19: TAP `fugin.cube` 810 Zeilen, 270 Cubes (`access_estsize>65536`),
  540 2D-Maps descoped; **alle 270 `fgn*.sky1` auf CDN `jvo.nao.ac.jp` present**.
  `sources.φ` trägt nur den Pilot. **Schritt:** `gh release view jvo.nao.ac.jp --repo
  omegaflow/sources --json assets` → 269 `url`-Blöcke in `sources.φ` (Arm + Pilot stehen).
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-12,18-20`.
  Re-Messung 2026-09-19: Root 200, sync-QUERY VOTable-Error `localhost:5432 connection
  refused` — unverändert. **Schritt:** Re-Check (Backend-Erholung) via
  `archive_search --verdict` + sync-QUERY. `blockiert` (extern).
- **lmd.jussieu vcd + mcd** `phi/pipeline/ledger.φ:14-16` — declined (GCM-Modell);
  nur `mars_dust` akzeptiert (em). Kein Schritt.
- **gedi_l2a** `phi/harvest.φ` — `blockiert` (Code, hdf5.rs fremd) → an bau. Kein Schritt.
- **icesat2_atl03** `phi/harvest.φ` — `blockiert` (Budget, bin fremd) → an bau. Kein Schritt.

## Wartend (kein Auswahlpunkt)

- **mars_dust CDN-Manifestation** — Compiler `tools/harvest/src/bin/mars_dust_compiler.rs`
  + `.github/workflows/mars-dust-cdn.yml` gebaut (cargo check sauber), in
  `phi/sources.φ`/`phi/harvest.φ` registriert, Workflow nach Push dispatcht.
  Auslöser: Run-Abschluss. **Schritt:** Ergebnis aus Watchdog-Snapshot /
  `ci_manage view <id>`; bei Erfolg Größe/sha256 messen und `sources.φ`-`sha256`-Zeile +
  `harvest.φ` (`asset present`) fortschreiben. `wartend`.
- **Cassini ODF+RSR** `phi/harvest.φ:10-29`, `phi/sources.φ:6919-6936` — Runs
  `35455805362`/`35455807084` **cancelled**, kein Asset. **Schritt:** Re-Dispatch nach
  Push (`gh workflow run cassini-odf-cdn.yml` / `cassini-rsr-cdn.yml`), Ergebnis lesen.
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

- **flash-first, Klasse geschlossen:** die Routine-Messungen (TAP-Re-Check dachs/pithia,
  FUGIN-Bulk-Verifikation) liefen über `grind-flash`; der harte Port-Bau (novel
  FITS-Cube-Parser + TAP-Route) über `grind-max`. Kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/harvest/src/bin/mars_dust_compiler.rs`,
  `.github/workflows/mars-dust-cdn.yml`, `src/archivar/motion.rs`,
  `phi/harvest.φ`, `phi/sources.φ`, `phi/pipeline/ledger.φ`,
  `phi/blocked_sources.φ`, die neue `handover-2026-09-19-ernte-folge95.md`,
  `docs/handover/archiv/handover-2026-09-19-ernte-folge94.md` (verschoben).
- **Fremd (nicht anfassen):** `src/archivar/fugin.rs`, `src/archivar/tests.rs`,
  `src/archivar/zarr.rs`, `src/mathematikerin/te.rs`, `src/mathematikerin/omega.rs`,
  `tools/measure/src/bin/*`, `tools/utils/src/bin/hdf5_reader.rs`,
  `docs/zustand/external-state.md`, die entscheid/forschung/bau-Handover-Moves.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
