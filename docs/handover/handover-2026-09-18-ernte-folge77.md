<!--
  title: Handover — Ernte-Folge 77 (Stand 2026-09-18)
  session: Ernte-Folge 77
  class: handover
  date: 2026-09-18
  sha256: 2167fd9b9920a58d408201e6a32275669d09a3dbfd70853148a3ba07b3ac6d15
  status: live
-->
# Handover — Ernte-Folge 77 (2026-09-18)

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

- **HEAD** `627ccea5` == `origin/main` (forschung: odyssey_odf close) beim Schreiben;
  die Linie startete auf `0dc09172` (ernte folge76). Der Baum war stark umkämpft:
  parallele `forschung`/`bau`/`entscheid`-Linien committen und arbeiten uncommittet
  an `src/archivar/*.rs`, `tools/register/src/bin/register_lookup.rs`, `AGENTS.md`,
  `docs/SOURCE_PORT.md`, `opencode.json`.
- **Postfach** — letzter Ledger-Eingang `1789689115`, **kein neuer Agenten-Eingang**
  (`state/mail/mail_ledger.φ` gelesen; `smail`-Eingänge nur Registrierungs-/Forum-Mails).
- **CI** — `ci_manage`: `ci-check` `35315678449` pending @627ccea5; `rosetta_odf`
  `harvest` `35312992622` in_progress (Ernte-Folge-77-Dispatch, head_sha `627793c4`
  = deployter alter Workflow); `te-gate` `35314110831` pending; `allwise-cdn`/
  `planetary-odf-cdn`/`ned-cdn` in_progress; `paper-check` success; sonst fremde Linien.
  Der Zustand-Ledger `docs/zustand/external-state.md` ist auf `627ccea5` gezogen
  (Postfach- + CI-Zeile).

## gedi/icesat2/swot — Compiler-Arm gebaut, Harvest-Dispatch offen (härtester undatiert)

- `grind-pro`: CMR→GetObject in `tools/harvest/src/bin/gedi_l2a_compiler.rs` und
  `icesat2_atl03_compiler.rs` (`--cmr`), CMR-Key-Beschaffung in
  `swot_l2_lr_ssh_compiler.rs`; `cargo check -p omegaflow-harvest` null Fehler/null
  Warnungen. CMR `short_name=GEDI02_A` liefert v002+v003, `--version` pinnt Default 002.
  `phi/harvest.φ` trägt die drei Blöcke (`asset fehlt`, `--cmr --day … --version …`,
  `tag`/`pattern` aus dem Compiler-Code). Dispatch 2026-09-18 nach Push (neue
  Zwei-Job-Form): `gedi_l2a` `35316395857`, `icesat2_atl03` `35316398029`,
  `swot_l2_lr_ssh` `35316400165` (alle queued). (Schritt: Asset via
  `ci_manage view <id>` prüfen; Idempotenz greift nach dem ersten Upload.)

## rosetta_odf — Workflow-Fix, Lauf offen (wartend)

- `harvest.yml` von Ein-Job auf Zwei-Job (`register` → `harvest`) umgebaut, damit das
  Job-Timeout den Block-`timeout` (350) liest (`needs.register.outputs.timeout`; Council
  bestätigt die Bauform: Job-Timeout wird vor dem ersten Step gewertet, nur `needs.*`
  ist zu diesem Zeitpunkt gefüllt). Dispatch `35312992622` (`-f timeout=350`) lief noch
  auf dem deployten alten Workflow. Der Zwei-Job-Fix ist gepusht (`6295dd5e`).
  (Schritt: Re-Dispatch `gh workflow run harvest.yml -f format=rosetta_odf` ohne
  `-f timeout` auf der neuen Form, sobald `35312992622` durch ist; Ergebnis via
  `ci_manage view`; Run-ID registrieren.)

## Werkzeug-Wrapper — lokaler Build entfernt

- Council-Verdikt C: `bin/{omega_sh,git_safety,sfetch,sgrep,ci_manage}` bauen nicht mehr
  lokal (kein `cargo build` im Session-Pfad; Fallback aufs vorhandene Binary + stderr-
  Stale-Notiz); `bin/archive_search` bleibt der sanktionierte Nachbau. Ursache der
  Hang-Notiz in folge76 (gemessen, nicht geraten): der erste Werkzeugaufruf startete
  einen Release-Build über dem Session-Timeout. (Schritt: AGENTS-Satz
  „`bin/archive_search` rebuilds only when stale …" auf die Wrapper-Menge erweitern —
  `AGENTS.md` ist fremd-modifiziert; der Schritt gehört der Linie, die `AGENTS.md` besitzt.)

## Waiting (kein Auswahlpunkt)

- `ulysses_atdf_x` X-Ref-Fenster/Halbwertsbreiten (Auslöser = nächstes `atdf`-Atom);
  `lro_utf`/`§4 census`/`bepicolombo`/`rosetta ungelaufene Pfade`/`auto-dispatch`/
  fünf Familien-Blöcke — Auslöser unverändert.
- Lasair-LSST + Pipeline-Port force-Gate: operator-gebunden, von `entscheid` konsumiert
  (die zwei `An entscheid`-post-Zeilen sind gelöscht) — nicht mehr in dieser Linie.

## Benchmark

- Hard atom (CMR→GetObject-Parser): `grind-pro`. Routine (rosetta-Workflow-Umbau,
  Wrapper-Edit): `grind-flash`. Workflow-Architektur: `council`. Kein neuer Doppel-Lauf —
  die Routineklassen tragen registrierte Sieger (`grind-flash`).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/harvest.yml`, `bin/{ci_manage,git_safety,
  omega_sh,sfetch,sgrep}`, `phi/harvest.φ` (drei Blöcke), `phi/sources.φ` (drei Noten),
  `tools/harvest/src/bin/{gedi_l2a,icesat2_atl03,swot_l2_lr_ssh}_compiler.rs`,
  `docs/zustand/external-state.md` (Postfach- + CI-Zeile),
  `docs/handover/handover-2026-09-18-ernte-folge77.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge76.md`).
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/SOURCE_PORT.md`, `opencode.json`,
  `src/archivar/{extract,mod,tests}.rs`, `src/archivar/arpansa.rs`,
  `tools/register/src/bin/register_lookup.rs`, `docs/handover/post.md`, die drei
  `handover-2026-09-16-*`-Renames, `handover-2026-09-18-entscheid-folge46.md`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
