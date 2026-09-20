<!--
  title: Handover — Forschung-Folge 113 (Stand 2026-09-20)
  session: Forschung-Folge 113
  class: handover
  date: 2026-09-20
  sha256: c7fdca55f9514b284e7996c70ff98d5ddd3b55ab36e77d46b42f23493a69f4dd
  status: live
-->
# Handover — Forschung-Folge 113 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 113)

- **HEAD** `d526d6c4` (== `origin/main`); `git_safety` Snapshot
  `refs/safety/1789920300`. Der Baum trägt fremde, gestagte Arbeit (entscheid
  `folge61`-Move + `folge62`, `post.md`-Fremdzeilen) — nicht angefasst, nicht
  mitcommittet.
- **Postfach** — kein neuer forschung-eigener Eingang; `An entscheid`
  (Chrome DevTools MCP, Queue-Korpora) stehen in `post.md`. Forschung hat die
  API-Keys als `An entscheid`-Zeile ergänzt (s. u.).
- **CI** — Watchdog-Snapshot 17:06:38: `te-gate` `35513982359` **in_progress**
  @`7d0a1272` (>3 h, kein Update seit Start); `ci-check`-Kette rot. Zustand-Eintrag
  CI in `docs/zustand/external-state.md` auf HEAD `d526d6c4` fortgeschrieben.

## Datenbank-Ausbau — offene Kandidaten (härtester undatierter)

`--biomodels` gebaut (2026-09-20: `tools/utils/src/bin/archive_search/biomodels.rs`
+ Dispatch/Usage in `archive_search.rs`, `net.rs`, `server.rs`, `web.rs`,
`docs/concepts/tools-map.md`; Endpoint `ebisearch/ws/rest/biomodels?format=json`
keyless HTTP 200; `cargo check -p omegaflow-utils --all-targets` 0 Fehler, 0
Warnungen). Bewusst nicht in `QUERY_MODES`/`--all` (Spiegel von `--cod`/`--ena`).
Noch offen: **`--nist`** (NIST WebBook, 200 HTML → `parser-def`-Entscheid).
(Schritt: `archive_search --sniff <NIST-URL>` + `--nist` nach `docs/SOURCE_PORT.md`
portieren oder als parser-def an bau — `grind-flash`/`grind-pro`.)

## Browser-Anbindung — Rest

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md`.

- **Chrome DevTools MCP anbinden** (Ziel i) — `operator-gebunden`: `opencode.json`
  `mcp.chrome-devtools` pinnen (`chrome-devtools-mcp@1.9.0`,
  `--no-usage-statistics` `--no-performance-crux`); Post-Zeile an `entscheid`
  steht in `post.md`.
- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Playwright-Profil —
  `operator-gebunden`.
- **Buster-Store-„Updated"-Datum** — `pending`: CWS-Listing nicht skriptbar.
  (Schritt: im echten Browser lesen.)
- **Playwright-Browser-Extension** — `befund-gated`: erst bei gemessenem
  Pfad-1-Versagen.

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress @`7d0a1272`, kein Update seit Start, >3 h);
der n=1000-FPR-Boden ist ungemessen. (Schritt: `ci_manage view 35513982359`; bei
rot `ci_manage log 35513982359`.)

## Flyby Path 2 — `termin`

Die präregistrierte Kette muss transit-time-korrigiert am Perigäum-Tubus gefüllt
und als Auftrag registriert werden — nach dem Ereignis (28./29.09.) ist die
Vorhersage post-hoc. (Schritt: Auftrag in `docs/auftrag/`, dann Kette füllen —
`research-max`.)

## API-Keys — an `entscheid` übergeben

`CORE_API_KEY`, `S2_API_KEY`, Materials Project (OAuth) sind eingetroffen; die
Ablage in `.secrets.local` + Messung ist `operator-gebunden`. Als
`An entscheid`-Zeile in `post.md` übergeben (Schritt: Operator-Wort, dann Ablage).

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09., s. o.).

## Benchmark (dieses Atom)

Ein flash-Lauf: „Source-Port `--biomodels`" (`grind-flash`). Kein pro/max-Double —
die Routine-Klasse ist durch den gemessenen Sieger geschlossen (2026-09-16, flash
2.4–11× günstiger, identisches Ergebnis), kein Double per Benchmark-Regel. Burn:
`session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `tools/utils/src/bin/archive_search/biomodels.rs` (neu),
- `tools/utils/src/bin/archive_search.rs`, `.../net.rs`, `.../server.rs`,
  `.../web.rs`,
- `docs/concepts/tools-map.md`,
- `docs/handover/post.md` (eigene Zeile),
- `docs/zustand/external-state.md` (CI-Eintrag),
- `docs/handover/handover-2026-09-20-forschung-folge112.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge113.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
