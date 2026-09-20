<!--
  title: Handover — Forschung-Folge 117 (Stand 2026-09-20)
  session: Forschung-Folge 117
  class: handover
  date: 2026-09-20
  sha256: e03c5cae1edd15738d412a781fdedaf7a10d5d1c960e199c6c3a88cc22a70472
  status: live
-->
# Handover — Forschung-Folge 117 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 117)

- **HEAD** `66579fc3` (== `origin/main`, gemessen bei Session-Beginn); während der
  Session rückte die Linie auf `68a8240f` (fremde Commits `8172288e` ernte folge118,
  `68a8240f` bau folge109 — letzterer schließt die `.tools_ensure`-TTL-Lücke
  inhaltsadressiert, s. Benchmark). Fremde uncommittete Arbeit im Baum:
  `docs/handover/post.md`, `docs/zustand/external-state.md` (Hunks der
  entscheid-folge64-Linie: Free-Model-Katalog-Zeile + Entfernen der
  Queue-Korpora-Post-Zeile) und `docs/handover/handover-2026-09-20-entscheid-folge64.md`
  (untracked) — **nicht angefasst**.
- **Postfach** — neuer Ledger-Eingang `1789922257` (`sales@pine64.org`: Hardware-Anfrage
  → Verweis auf `info@pine64.org`); davor `1789918147` (GitHub-OAuth „ISH Chat",
  Sicherheitsereignis, bereits als `An entscheid:` geführt). `post.md` leer.
- **CI** — `ci_manage list` 2026-09-20 ~19:2xZ: **in_progress** `ci-check`
  `35523145456`, `health-check` `35519449450`; **failed (attempt 1)** `ci-check`
  `35520538766`/`35517957987`; **success** `tools-build` `35524239946`/`35524233907`,
  `harvest` `35524334872`, `harvest-dispatch` `35524311418`/`35524089704`,
  `paper-check` `35524233908`, `quake-feeds-cdn` `35524116063`, `ned-cdn`
  `35524087139`; `te-gate` `35513982359` @`7d0a1272` weiter **in_progress**
  (kein Update seit Start 13:36Z). Rat-Verdikt: **wartend**, kein Cancel.
- **Zustand-Ledger nicht fortgeschrieben** — `docs/zustand/external-state.md`
  trägt fremde uncommittete Hunks; die Zeilen CI/Postfach wurden gemessen, aber
  nicht committet (Registraturpflicht: die Zeilen beim nächsten sauberen Stand
  fortschreiben). Werte stehen hier.

## Chrome DevTools MCP — Verifikation (härtester undatiert)

Der Pin `npx -y chrome-devtools-mcp@1.9.0` ist geladen: `chrome-devtools_list_pages`
antwortet (1 Seite, `about:blank [selected]`) — die Ladung nach opencode-Neustart
ist gemessen. Offen bleibt der Membran-Lauf (`cargo run` → `127.0.0.1:1618`,
Konsole/Netz via CDP am live Chrome) — ein sichtbarer Vordergrund-Lauf.
(Schritt: Operator-Wort für den Lauf; danach CDP-Lesen. `OMEGAFLOW_HIDDEN=1`
stillt das Fenster, nicht den CDP-Bedarf.)

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` @`7d0a1272` in_progress seit 13:36Z, kein Update. Rat
(2026-09-20): **kein Cancel** — der Watchdog hat die 2×-Median-Schwelle nicht
(„no median basis — fewer than two successful runs — no action"), und der Lauf
liegt im gemessenen Mehrstunden-Profil derselben Batterie. (Schritt:
`ci_manage view 35513982359`; success → `ci_manage log 35513982359` und die vier
`fpr_rise_sigma_test`-Gate-Zeilen verifizieren; cancelled → `gh workflow run
te-gate.yml` am HEAD mit dem 3σ-Fix `5b406e16`.)

## Flyby-Path-2-Kette — `termin` (Perigäum 28./29.09.)

Bereitschaft gemessen 2026-09-20 (Addendum
`docs/paper/flyby-path-2-falsification-metric-addendum.md` §"Chain readiness");
alle Kanalrouten HTTP 200, Transit-Methode benannt, DSCOVR retired → RTSW deckt L1.
Keine Zellwerte vor dem Perigäum — jede Zelle `pending` (0 honored). (Schritt:
Zellen ab 28.09. füllen, je Messwert `source`+`active` protokollieren —
`ernte`/`research-max`.)

## Browser-Anbindung — Rest

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md`.

- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Playwright-Profil —
  `operator-gebunden`.
- **Playwright-Browser-Extension** — befund-gated (erst bei gemessenem
  Pfad-1-Versagen).

## Hardware-Sponsoring — `operator-gebunden`

Drei offene Threads im Postfach: Pine64 (`1789922257`, Verweis auf
`info@pine64.org`), Framework (Ticket `NG2HWBZM`), Tuxedo (Ticket `#991311279`).
Eine Antwort an einen Dritten ist ein consent-pflichtiger Akt → `entscheid`/Operator.
(Die Post-Zeile konnte diese Session nicht gesetzt werden: `post.md` trägt fremde
uncommittete Hunks — nicht angefasst.)

## ernte / termin

- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027).

## Benchmark (dieses Atom)

- **MP-Route** — `general` (flash) meldete „pending, HTTP 404"; die Messung traf
  ein **veraltetes** Binary (der erste `.tools_ensure`-Aufruf refreshte es erst).
  Die Session-Re-Messung lieferte HTTP 200 mit Treffern — Route-Fix live,
  **Punkt geschlossen**. Ursache war die TTL-Gate-Timing, nicht der Agent; Klasse
  „Routine-Reachability" ist im Register durch `grind-flash` $0.0008 geschlossen.
  Der Baum schloss die Ursache in `68a8240f` (bau folge109): `.tools_ensure` ist
  jetzt inhaltsadressiert (ein veraltetes Artefakt refresht innerhalb der TTL).
- **TE-Gate** — `council` (pro/max): Verdikt „wartend, kein Cancel" mit der
  Watchdog-Schwelle als Evidenz; der Entscheidungswert, nicht mechanisch.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-forschung-folge117.md` (neu),
- `docs/handover/handover-2026-09-20-forschung-folge116.md` (Move → `archiv/`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
