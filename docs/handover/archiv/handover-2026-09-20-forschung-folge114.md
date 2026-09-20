<!--
  title: Handover — Forschung-Folge 114 (Stand 2026-09-20)
  session: Forschung-Folge 114
  class: handover
  date: 2026-09-20
  sha256: 545cdfcfc71352cb5ff7b04b6b6a25dec37a4a905dc88432e1b46d124cfcbef9
  status: live
-->
# Handover — Forschung-Folge 114 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 114)

- **HEAD** `813a82c4` (== `origin/main`; lokale Bau-/Parallel-Commits darüber, der
  geteilte Baum bewegt sich während der Session); `git_safety` Snapshot
  `refs/safety/1789920635`. Der Baum trägt fremde, gestagte Arbeit (entscheid
  `folge61`-Move + `folge62`) — nicht angefasst, nicht mitcommittet.
- **Postfach** — nicht fällig: letzter Ledger-Eingang `1789918147`, jetzt
  `1789920678` (42 min < 2⁶ min); Eintrag in `external-state.md` zitiert, keine
  Neumessung. Keine neue forschung-eigene Post-Zeile.
- **CI** — Watchdog-Snapshot 17:06:38: `te-gate` `35513982359` weiter
  **in_progress** (`ci_manage view 35513982359`, kein Verdikt, kein Update seit
  Start); `ci-check`-Kette rot. Zustand-Eintrag CI in `external-state.md` auf
  HEAD `813a82c4` fortgeschrieben.

## Browser-Anbindung — Rest (härtester undatierter, `operator-gebunden`)

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

Ein flash-Lauf: „Source-Port `--nist`" (`grind-flash`, `$0.0138`). Ergebnis: kein
Bau — der NIST-WebBook-Query-Modus ist HTML-only ohne maschinenlesbaren Endpunkt,
also `blocked parser-def` (Eintrag in `phi/blocked_sources.φ`); kein pro/max-Double
(Routine-Klasse durch den gemessenen flash-Sieger geschlossen, 2026-09-16). Burn:
`session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `phi/blocked_sources.φ` (Eintrag `--nist`),
- `docs/zustand/external-state.md` (CI-Eintrag),
- `docs/handover/handover-2026-09-20-forschung-folge113.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge114.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
