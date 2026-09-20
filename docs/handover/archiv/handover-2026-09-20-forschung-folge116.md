<!--
  title: Handover — Forschung-Folge 116 (Stand 2026-09-20)
  session: Forschung-Folge 116
  class: handover
  date: 2026-09-20
  sha256: 5cb626fbc42a0c256d9b09ea61b8b026a02f0df617827251d0ad77c4584cb4de
  status: live
-->
# Handover — Forschung-Folge 116 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 116)

- **HEAD** `c29234d1` (== `origin/main`); die Übergabe folge115 nannte `1963be30` —
  überholt (fremde Commits `e52b59d0` port, `5c61e16e` entscheid folge63,
  `c29234d1` sources rv_scale). Fremde uncommittete Arbeit im Baum:
  `phi/harvest.φ`, `phi/sources.φ`, `handover-2026-09-20-ernte-folge117.md` —
  **nicht angefasst**.
- **Postfach** — letzter Ledger-Eingang `1789918147`; S2-Key `1789899110` +
  CORE-Key `1789899008` gelesen; `post.md` ist leer (entscheid folge63 hat die
  Zeilen gefoldet). `external-state.md` Postfach-Eintrag fortgeschrieben.
- **CI** — `ci_manage list` 16:5xZ: `te-gate` `35513982359` @`7d0a1272` weiter
  **in_progress** (kein Update seit 13:36Z — stale/ghost, kein Verdikt);
  `tools-build`/`ps1-cdn`/`harvest` success. `external-state.md` CI-Eintrag auf
  `c29234d1` fortgeschrieben.

## Flyby-Path-2-Kette — `termin` (Perigäum 28./29.09.)

Bereitschaft gemessen 2026-09-20 (Addendum
`docs/paper/flyby-path-2-falsification-metric-addendum.md` §"Chain readiness"):
alle Kanalrouten HTTP 200 (stage 1), Selektoren gegen die File-Ordnung geprüft,
Transit-Methode benannt (Lichtzeit 5,0 s; Advektion `d/v_sw`; Kp 3-h; Swarm am
Ort). DSCOVR: eigene keyless Route retired (404), L1 lebt im RTSW-Feed
(multi-source, `source` je Messwert) → keine neue `sources.φ`-Zeile. Keine
Zellwerte vor dem Perigäum — jede Zelle `pending` (0 honored). (Schritt: Zellen ab
28.09. füllen, je Messwert `source`+`active` protokollieren — `ernte`/`research-max`.)

## Chrome DevTools MCP — gepinnt, Verifikation offen

`opencode.json` trägt `mcp.chrome-devtools` = `npx -y chrome-devtools-mcp@1.9.0
--no-usage-statistics --no-performance-crux` (JSON validiert). Der MCP lädt erst
nach **opencode-Neustart**; die Verifikation (ein Membran-Lauf `cargo run` →
`127.0.0.1:1618`, Konsole/Netz via CDP gelesen) ist ungemessen. (Schritt: opencode
neu starten, dann Membran-Lauf.)

## MP-Route-Fix — CI-Release offen

`tools/utils/src/bin/archive_search/materialsproject.rs`: ENDPOINT
`…/summary/search` → `…/summary/` + `_fields` (gemessen: `/summary/` HTTP 200,
`/summary/search` 404; `?formula=Si` 200). `cargo check -p omegaflow-utils --bin
archive_search` sauber. Der PATH-Tool (`tools-latest`) trägt die Route erst nach
dem CI-Release. (Schritt: nach Push `tools-build` dispatcht — die Session tut das.)

## Browser-Anbindung — Rest

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md`. Buster-Store-Datum
gemessen (3.4.0, 20.06.2026) → Store-`pending` geschlossen; Survey-sha erneuert.

- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Playwright-Profil —
  `operator-gebunden`.
- **Playwright-Browser-Extension** — befund-gated (erst bei gemessenem
  Pfad-1-Versagen).

## API-Keys — erledigt (kein offener Punkt)

`CORE_API_KEY`, `S2_API_KEY` (Mail `1789899110`), `MP_API_KEY` (Dashboard via
Browser) in `.secrets.local`; CORE/S2 gemessen (Suche liefert Treffer, kein
Rate-Limit-`pending`). MP gemessen nach Route-Fix.

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` in_progress @`7d0a1272`, kein Update seit 13:36Z (>5 h).
(Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log 35513982359`.)

## ernte / termin

- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027).

## Benchmark (dieses Atom)

`research-max` (Flyby-Ketten-Bereitschaft): 13 Werkzeug-Aufrufe; das Urteil
(DSCOVR retired / RTSW deckt L1 / Latenz-Deutung) war der Wert, die 16
Reachability-Verdicts mechanisch. Der Taucher nennt eine **flash-Vorstufe** für
die mechanischen Verdicts als ausreichend (Klasse „Routine-Reachability" im
Register durch grind-flash $0.0008 geschlossen). Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `opencode.json` (MCP-Pin),
- `tools/utils/src/bin/archive_search/materialsproject.rs` (Route-Fix),
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (Chain readiness),
- `docs/auftrag/auftrag-flyby2-kette.md`,
- `docs/surveys/survey-2026-09-20-browser-anbindung.md`,
- `docs/zustand/external-state.md` (CI + Postfach),
- `docs/handover/handover-2026-09-20-forschung-folge115.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge116.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
