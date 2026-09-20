<!--
  title: Handover — Forschung-Folge 112 (Stand 2026-09-20)
  session: Forschung-Folge 112
  class: handover
  date: 2026-09-20
  sha256: 5cb81d91be10662d3e88ee638a063bac36eacdac8e2767fc85380d1b053ce4ef
  status: live
-->
# Handover — Forschung-Folge 112 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 112)

- **HEAD** `8df40994` (== `origin/main`; die ernte-Linie hat während der Session
  gepusht — Planungs-HEAD war `aa85dea6`); `git_safety` Snapshot
  `refs/safety/1789919767`. Der Baum trägt fremde Arbeit (entscheid
  `folge61`-Move + `folge62`, `post.md`, `phi/harvest.φ`) — nicht angefasst.
- **Postfach** — kein neuer forschung-eigener Eingang; `An entscheid`
  (Chrome DevTools MCP) steht in `post.md`. Die API-Keys CORE/S2/Materials Project
  sind eingetroffen (`docs/zustand/external-state.md` zitiert, nicht kopiert).
- **CI** — `te-gate` `35513982359` **in_progress** @`7d0a1272` (>3 h, alter SHA);
  `release-build` success; `ci-check`/`harvest` laufen.

## Manifestations-Audit — abgeschlossen (2026-09-20)

Read-only gegen den Baum gemessen (drei `general`-Läufe); kein Register-/Code-Edit
am Audit selbst (eine Kernel-Korrektur bleibt Architektur-Akt, Rat-Verdikt folge111).

- **1a Klassifikation** — `denis_j_mag` (`phi/sources.φ:8832`; Block
  `flux_from_mag jmag` `:8824`) und `cb_mag1` (`:8738`; `flux_from_mag mag1`
  `:8730`) sind **strahlend** — `src/archivar/extract.rs:2504-2508` transformiert
  `10^(−0.4·mag)` und leert die unit. `extinction_rv` (`:8863`, unit `1`, kein
  `flux_from_mag`), `pastel_teff_k` (`:9078`) und `polarbase_teff_k` (`:9096`)
  sind **skalar** — ihr Block-`flux_from_mag mag` (`:9070`/`:9088`) zielt auf
  `pastel_vmag` `:9080` bzw. `polarbase_vmag` `:9098`, nicht auf `teff`.
- **1b Doc↔Register-Drift** — `docs/concepts/archivar-mathematikerin.md:19`
  behauptete „5 tokens … optional `tau`"; der Parser (`src/archivar/parse.rs:878`)
  verlangt 9 Pflicht-Tokens (`field key name kernel force unit tau absorption
  advection`, τ-Gate `:902`), optional `freq`/`bin_width` (`:920`/`:928`);
  `ttl` ist Quell-Direktive (`:175`), kein Feld-Token. Doc korrigiert.
- **1c „+292 fold"** — die folge111-Notiz „nicht reproduziert" ist widerlegt:
  292 ist exakt die Zahl der `fold`-Registerzeilen (`sgrep -c "fold"
  phi/sources.φ`), eine eigene Zeilenart (`parse.rs:1151`, `units.rs:93
  fold_value`, `docs/concepts/parser-magic.md:40`). Die 669
  `inverse-square advective` zerlegen sich exakt: 336 `field` + 41
  `gaussian-inverse-square` + 292 `fold` (146 `sin_deg` + 146 `cos_deg` auf
  WDIR/MWD).

## Datenbank-Ausbau — offene Kandidaten

`--cod` gebaut (Port 2026-09-20: `tools/utils/src/bin/archive_search/cod.rs` +
Dispatch/Usage in `archive_search.rs`, `net.rs`, `server.rs`, `web.rs`,
`docs/concepts/tools-map.md`; `cargo check -p omegaflow-utils --all-targets`
0 Fehler, 0 Warnungen). Endpoint gemessen: `/cod/result?format=json` keyless
HTTP 200 `application/json` (239260 B bei `text=quartz`); Eintrags-URL
`/cod/<file>.html` 200. Bewusst nicht in `QUERY_MODES`/`--all` (Spiegel von
`--ena`). Noch offen: `--biomodels` (EBI, 200 JSON), `--nist` (NIST WebBook,
200 HTML → `parser-def`-Entscheid). (Schritt: `--biomodels` nach
`docs/SOURCE_PORT.md` portieren — `grind-flash`.)

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

`te-gate` `35513982359` (in_progress @`7d0a1272`, >3 h); der n=1000-FPR-Boden ist
ungemessen. (Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

## Flyby Path 2 — `termin`

Die präregistrierte Kette muss transit-time-korrigiert am Perigäum-Tubus gefüllt
und als Auftrag registriert werden — nach dem Ereignis (28./29.09.) ist die
Vorhersage post-hoc. (Schritt: Auftrag in `docs/auftrag/`, dann Kette füllen —
`research-max`.)

## API-Keys ablegen — `operator-gebunden`

`CORE_API_KEY` und `S2_API_KEY` sind im Postfach eingetroffen, Materials Project
via OAuth autorisiert (`docs/zustand/external-state.md`); die Ablage in
`.secrets.local` und die anschließende Messung stehen offen. Gehört zur
`entscheid`-Linie. (Schritt: entscheid / Operator-Wort.)

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09., s. o.).

## Benchmark (dieses Atom)

Vier Aufgaben, vier flash-Läufe: „Manifestations-Audit 1a/1b/1c" (`general`) und
„Source-Port COD" (`grind-flash`). Kein pro/max-Double — die Routine-Klasse ist
durch den gemessenen Sieger geschlossen (2026-09-16, flash 2.4–11× günstiger,
identisches Ergebnis), kein Double per Benchmark-Regel. Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/concepts/archivar-mathematikerin.md`,
- `docs/concepts/tools-map.md`,
- `tools/utils/src/bin/archive_search.rs`, `.../net.rs`, `.../server.rs`,
  `.../web.rs`,
- `tools/utils/src/bin/archive_search/cod.rs` (neu),
- `docs/handover/handover-2026-09-20-forschung-folge111.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge112.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
