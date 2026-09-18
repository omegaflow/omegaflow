<!--
  title: Handover — Ernte-Folge 79 (Stand 2026-09-18)
  session: Ernte-Folge 79
  class: handover
  date: 2026-09-18
  sha256: 3e2fc1abb4aae0f851be530fbb68f8e74d2a1fa088e5b2ea84ccebb2b4229a95
  status: live
-->
# Handover — Ernte-Folge 79 (2026-09-18)

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

- **HEAD** `f6b696a7` bei Session-Start == `origin/main`; die Bau-Linie pushte
  `4dbe4805` (parser-def queue) mid-Session — `origin/main` steht auf `4dbe4805`,
  der eigene Commit folgt darauf. `git_safety`-Snapshot `refs/safety/1789714672`.
- **Postfach** — kein neuer Agenten-Eingang; `post.md` leer, Ledger-Tail ohne Eingang.
- **CI** — die drei Re-Dispatch-`harvest`-Läufe `35317116366`/`35317118656`/
  `35317120978` **failure** (drei Compiler-Gründe, s.u.); `EARTHDATA_EDL_TOKEN: ***`
  erreicht die Jobs (Secret seit 2026-09-16), die frühere „absent"-Ursache ist
  behoben. `rosetta_odf` `35312992622` in_progress (alter Workflow). Zustand-Ledger-CI-Zeile
  auf `f6b696a7` gezogen.

## harvest — drei Compiler-Gründe offen (härtester undatierter Punkt)

Der EDL-Token-Fix (`cadc738a`) + Legacy-cdn-Retirement sind gebaut und gepusht;
die Re-Dispatch-Welle am `4b01d600` **failure**, aber nicht mehr am Token. Drei
getrennte Compiler-Lücken, je ein hartes Atom:

- **gedi_l2a** — `Run 35317116366`: HDF5-`metadata extent 263439695 B >
  META_ESCALATION 100663296 B` (`gedi_l2a_compiler.rs:24-25`), 0 records.
  (Schritt: `sread src/archivar/hdf5.rs` — Superblock/Userblock-Metadaten-Extent
  lesen statt fester Fenster-Eskalation; `sgrep "META_ESCALATION"`.)
- **icesat2_atl03** — `Run 35317118656`: fetch + 32 MiB Header-Parse ok,
  `first_values` 0 Datasets (kein Fehler-Line). (Schritt: `first_values` in
  `icesat2_atl03_compiler.rs` instrumentieren; erwartete HDF5-Gruppe/Datasets
  messen.)
- **swot_l2_lr_ssh** — `Run 35317120978`: SigV4 GetObject auf
  `podaac-swot-ops-cumulus-protected` → 403, Creds geholt (`range.rs:230-247`
  podaac-* → `PODAAC_S3_CREDENTIALS_URL`). (Schritt: Creds-Antwort gegen Bucket
  messen; mit `sources.φ:1542` (GetObject 206 mit EDL-Token, `--granule`)
  vergleichen.)

Nach Fix: `gh workflow run harvest.yml -f format=<f>`, Run-ID registrieren, bei
success `phi/harvest.φ` `asset fehlt`→`present` + `note` (size/sha256).

## rosetta_odf — Re-Dispatch (wartend)

- `harvest.yml` Zwei-Job-Fix gepusht (`6295dd5e`); der Re-Dispatch wartet auf den
  Abschluss von `35312992622` (alter Workflow, in_progress). (Schritt: Re-Dispatch
  `gh workflow run harvest.yml -f format=rosetta_odf` ohne `-f timeout`; Run-ID
  registrieren.)

## Werkzeug-Wrapper — AGENTS.md-Satz (fremd)

- `AGENTS.md` ist fremd-modifiziert; der Satz „`bin/archive_search` rebuilds only
  when stale …" gehört der Linie, die `AGENTS.md` besitzt.

## Waiting (kein Auswahlpunkt)

- `ulysses_atdf_x` X-Ref-Fenster (Auslöser = nächstes `atdf`-Atom);
  `lro_utf`/`§4 census`/`bepicolombo`/`rosetta ungelaufene Pfade`/`auto-dispatch`/
  fünf Familien-Blöcke — Auslöser unverändert.

## Benchmark

- Routine (Log-Root-Cause, `ci_manage log`): `grind-flash`. Judgment
  (non-CMR-Disposition): `grind-pro`. Kein neuer Doppel-Lauf — beide Klassen tragen
  ihren registrierten Sieger.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/harvest.φ`, `docs/zustand/external-state.md` (CI-Zeile),
  `docs/handover/handover-2026-09-18-ernte-folge79.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge78.md`).
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/SOURCE_PORT.md`, `opencode.json`,
  `src/archivar/{extract,mod,tests}.rs`, `src/archivar/arpansa.rs`,
  `tools/register/src/bin/register_lookup.rs`, `docs/handover/post.md`,
  `handover-2026-09-18-entscheid-folge46.md`, `handover-2026-09-18-bau-folge78.md`,
  `handover-2026-09-18-forschung-folge76.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
