<!--
  title: Handover — Ernte-Folge 89 (Stand 2026-09-19)
  session: Ernte-Folge 89
  class: handover
  date: 2026-09-19
  sha256: 02747320b8e1f17f33c4b7bfc38d1ebc4c894d7e1644415896ff5cf09c4c567b
  status: live
-->
# Handover — Ernte-Folge 89 (2026-09-19)

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

- **HEAD** `4e43856e` (während der Session von `69fbe665` bewegt — research folge86
  committete `te-gate`-Re-Dispatch). Safety-Net `refs/safety/1789762240`.
- **Postfach** — `state/mail/mail_ledger.φ` trägt neue Eingänge bis `1789795811`
  (Rubin-Forum-Liste, Brave-Quota-Alert, eigene SuperDARN-Anfrage `1789718159`);
  keine Sonden-Antwort. `docs/zustand/external-state.md` fremd-modifiziert
  (Entscheid-Folge 52) → zitiert, nicht angefasst.
- **CI** — Watchdog-Snapshot gelesen; `gedi`/`atl03`/`lro` waren zu Session-Beginn
  nicht mehr aktiv. Gemessen per `ci_manage view`/`log`: alle drei `cancelled` am
  eigenen `timeout-minutes`, kein Watchdog-Cancel (Watchdog-Log nennt nur die
  älteren `35338097066`/`35338099137`). `rosetta` (`planetary-odf-cdn`
  `35351411938`) lief noch.

## Source-Port — offene Arme (härtester undatierter zuerst)

- **Cassini RSS Compiler-Arm** `phi/blocked_sources.φ:17-19` — CORSS_8xxx,
  Bahnverfolgung em/Range-Rate; Arm ungebaut. (Schritt: Arm in `tools/harvest` +
  `src/archivar` bauen, dann Register.) `blockiert` (Arm).
- **ExoFOP TOI** `phi/pipeline/ledger.φ:66-68` — Arm gebaut 2026-09-19
  (`tools/harvest/src/bin/exofop_compiler.rs`, `extract.rs` `exofop_toi`, Format
  EXF1 44 B/Record, 1770 Zeilen/63 Spalten gemessen); `sources.φ`+`harvest.φ`
  registriert, Force `em`. CDN offen: nach Push `gh workflow run exofop-cdn.yml`
  → `archive_search --sniff …/exofop.ipac.caltech.edu/exofop_toi.bin`, dann
  `harvest.φ`-Block auf `asset present`. (Schritt: CI-Dispatch + Sniff.) `offen`.
- **KCDC** `phi/blocked_sources.φ:47-50` — `phi/sources.φ` jetzt clean, Block
  entblockt. (Schritt: `kcdc_kascade`-Block + CDN-Manifestation.) `offen`.
- **gedi_l2a** `phi/harvest.φ:26-34` — Run `35351460528` `cancelled` am timeout 180
  ohne Fortschritt: `gather_messages`-Hang besteht trotz `c4533fe9`
  (HashSet+MAX_CONT_BLOCKS) fort. (Schritt: GEDI-HDF5-Pfad messen, nicht erneut
  dispatchen.) `blockiert` (Code).
- **GHRC GLM L1B + TRMM LIS** `phi/blocked_sources.φ:35-37` — CDN dispatcht
  2026-09-19 (`glm-l1b-cdn` `35425276060`, `trmm-lis-cdn` `35425276991`); bei
  Abschluss sha256/Größe + `asset present`. `wartend`.
- **FUGIN Bulk** `phi/blocked_sources.φ:39-41` — `fugin-cdn-dispatch`
  `35425278304` (269 Cubes, Pilot idempotent übersprungen). Bei Abschluss
  Assets prüfen. `wartend`.
- **icesat2_atl03 / lro_trk** `phi/harvest.φ:44-64` — Timeouts korrigiert
  (360/120), re-dispatcht `35425364754`/`35425365015` (beide `in_progress`). Bei
  Abschluss sha256/Größe + `asset present`. `wartend`.
- **rosetta_odf** `phi/harvest.φ:92-99` — `planetary-odf-cdn` `35351411938`
  noch aktiv. `wartend`.

## Wartend (kein Auswahlpunkt)

- **Lasair-LSST** `external-state.md:23` — Re-Messung nach at-risk-Fenster
  (2026-09-19). `termin`.
- **Voyager/Mariner/Viking/Juno-Antworten** `blocked_sources.φ:52-66` — Anfragen
  offen. `wartend`.
- **Limadou PI-Freigabe** `ledger.φ:26-28` — per-act consent (Operator). `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:90-128` — `--port` braucht das Operator-Wort für den
  lokalen Release-Binär-Lauf. `operator-gebunden`.

## Benchmark

- **flash-first:** die Cancel-Diagnose (3 Runs, `ci_manage view`/`log`) lief über
  `grind-flash` — kein Doppel-Lauf, keine pro/max-Konkurrenz nötig; der ExoFOP-Arm
  (neuer Parser) über `grind-pro`. Kein Regressions-Paar.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/harvest/src/bin/exofop_compiler.rs` (neu),
  `.github/workflows/exofop-cdn.yml` (neu), `src/archivar/extract.rs` (exofop-Dispatch),
  `phi/harvest.φ` (exofop-Block + Timeout-Korrekturen icesat2/lro + gedi-Note),
  `phi/sources.φ` (**nur** der gestagte exofop-Hunk), `phi/pipeline/ledger.φ`
  (ExoFOP → kompiliert), `phi/blocked_sources.φ` (GLM/TRMM/FUGIN-Notes),
  `docs/handover/handover-2026-09-19-ernte-folge89.md` (neu),
  `docs/handover/archiv/handover-2026-09-18-ernte-folge88.md` (verschoben).
- **Fremd (nicht anfassen):** `phi/sources.φ`-Fremd-Hunks (ARPANSA/MAXI/Restruktur),
  `src/archivar/hdf5.rs`, `src/archivar/tests.rs`, `src/archivar/voyager_odr.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `src/mathematikerin/te.rs`, die `handover-2026-09-16-*`-Moves,
  `docs/handover/post.md`, `docs/zustand/external-state.md` (Entscheid-Folge 52).
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Nach dem Push:
`exofop-cdn.yml` dispatchen (der Workflow existiert erst auf dem Default-Branch).
