<!--
  title: Handover — Forschung-Folge 105 (Stand 2026-09-20)
  session: Forschung-Folge 105
  class: handover
  date: 2026-09-20
  sha256: 7cf864a08912c8fff0e30e6bd117da747d0ffa0288fd562076f17f9f6a83d397
  status: live
-->
# Handover — Forschung-Folge 105 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 105)

- **HEAD** `a89fb434` (== `origin/main`, „research: add fifteen science/literature
  search modes …") am Session-Ende; `git_safety` Snapshot `refs/safety/1789894985`
  bei Session-Beginn. Der Baum trug bei Beginn nur die zwei forschung-eigenen
  Archiv-Moves `folge44/51` → `archiv/` — in diesem Atom mitcommittet.
- **Postfach** — neue Eingänge: `1789899110` „Your Semantic Scholar API Key",
  `1789899008` „CORE API Key registration", `1789898868` „CORE API Key
  registration (verify)", `1789898286` „We've received your key request",
  `1789898153` „[Materials Project] Welcome …". Beide Keys (CORE, S2) sind da;
  der CORE-Verify-Link wurde gelöst (1. Versuch HTTP 500 = CORE-Backend-Bug,
  2. Versuch 200).
- **CI** — `ci_manage list` (2026-09-20 ~10:18Z): **in_progress** `ci-check`
  `35504694962` @`a89fb434` (der Push dieser Session), `allwise-cdn`
  `35504050725`; **failure** `ci-check` `35501198690`, `health-check`
  `35486205691`; **success** `harvest-dispatch` `35497506204`, `quake-feeds-cdn`,
  `ned-cdn`, `ps1-cdn`, `swpc-mirror-cdn`; **cancelled** die ci-check-Kette
  (per-ref-Concurrency). Der `ci-check`-Failure bleibt offen (s. u.).

## Flyby Path 2 — Feldfüllung am Perigäum-Tubus vor dem 28.09. — `termin`

Das versiegelte Blatt (`docs/paper/flyby-path-2-preregistration.md`, Seal
2026-08-22; `-falsification-metric-addendum.md`, Seal 2026-08-28) ist kein
Auftrag. Die präregistrierte Kette (plasma-pressure gradient, IMF-Bz, Kp, Swarm)
muss transit-time-korrigiert am Perigäum-Tubus gefüllt und als Auftrag
registriert werden — nach dem Ereignis (28./29.09.) ist die Vorhersage post-hoc.
Kanäle leben: `phi/sources.φ` RTSW mag/wind (`:158,164`), Kp (`:179`), Swarm
(`:6062-6078`). (Schritt: Auftrag in `docs/auftrag/`, dann die Kette füllen —
`research-max`.)

## CI-Verdikte am Gate-Fluss — `wartend`

- n=1000 FPR-Gate (3σ-Fix `5b406e16`): `te-gate` `35475226890`. (Schritt:
  `ci_manage view 35475226890`.)
- Flare-Gate Power-Probe (`9f48b5bd`): Lauf `35473941500`. (Schritt:
  `ci_manage log 35473941500`.)
- `ci-check` `35501198690` failure; der Lauf `35504694962` @`a89fb434`
  entscheidet neu. (Schritt: `ci_manage view 35504694962`; bei rot
  `ci_manage log`.)
- `hyperscanning-te` `35472171277` pending. (Schritt: Verdikt lesen.)

## API-Keys ablegen — `operator-gebunden`

Die Modi `--core` (keyless nutzbar), `--semanticscholar`, `--materialsproject`
sind gebaut und committet; ohne Key melden sie `pending` bzw. den 429.
(Schritt: `CORE_API_KEY` und `S2_API_KEY` aus `state/mail/mail_ledger.φ`,
`MP_API_KEY` aus `next-gen.materialsproject.org/api` in `.secrets.local`; dann
`archive_search --core/--semanticscholar/--materialsproject <q>` messen.)

## Datenbank-Ausbau — offene Kandidaten

Die 15 gebauten Modi sind committet; aus der Messliste (2026-09-20) fehlen noch
`--reactome`, `--interpro`, `--kegg` (Lizenz), `--alphafold`, `--entrez`
(GenBank/SRA/GEO), `--ena`, `--biomodels`, `--nist`, `--cod` — alle HTTP 200
außer KEGG (Lizenz). (Schritt: nächsten Kandidaten nach `docs/SOURCE_PORT.md`
portieren, `grind-flash`.)

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09., s. o.).

## Benchmark (dieses Atom)

- Kein Doppellauf: das Atom war Bau (15 Modi + Wartung), kein Architektur-Urteil.
  Der `research-max`-Lauf (Datenbank-Landschaft, Top-10 + Kandidatenliste) lieferte
  die Messliste; die Klasse „Datenbank-Landschaft" ist damit erstmals gemessen.

## Geteilter Baum — eigener Pfad-Satz

- `tools/utils/src/bin/archive_search{,/*.rs}` (15 neue Module + Verdrahtung),
  `tools/utils/src/bin/hdf5_reader.rs`, `docs/concepts/tools-map.md`,
  `docs/handover/handover-2026-09-20-forschung-folge105.md` (neu),
  `docs/handover/archiv/handover-2026-09-20-forschung-folge104.md` (Move),
  `docs/zustand/external-state.md`, `docs/handover/post.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
