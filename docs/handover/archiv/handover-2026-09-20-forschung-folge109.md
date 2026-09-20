<!--
  title: Handover — Forschung-Folge 109 (Stand 2026-09-20)
  session: Forschung-Folge 109
  class: handover
  date: 2026-09-20
  sha256: a116f19201f10829f05a2147bfb61c8b81711e5df3ace3c37721f5c2d23a5141
  status: live
-->
# Handover — Forschung-Folge 109 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 109)

- **HEAD** `aa120ac6` (== `origin/main`); `git_safety` Snapshot
  `refs/safety/1789915943`. Der Baum trägt bei Beginn fremde uncommittete Arbeit
  (bau/entscheid/ernte-Handover-Moves + neue folge102/62/109, `docs/zustand/
  external-state.md` staged M, `phi/harvest.φ` M) — nicht angefasst.
- **Postfach** — `post.md` trägt eine `entscheid`-Zeile (Lasair-Token, von ernte),
  keine forschung-eigene Post.
- **CI** (Watchdog-Snapshot 2026-09-20T16:03+02:00): in_progress `harvest`
  `35515194248`, `te-gate` `35513982359`, `ci-check` `35513190719`, `health-check`
  `35505471538`; failure `release-build` `35513611936` + die ci-check-Kette.

## Browser-Anbindung — Anbindung schließen (härtester undatiert)

Messung + Verdikte: `docs/surveys/survey-2026-09-20-browser-anbindung.md`; Rollen in
`docs/concepts/tools-map.md`. Rat: **nicht konsolidieren** — drei Pfade, drei Rollen.

- **Chrome DevTools MCP anbinden** (Ziel i, Membran-Debug) — `operator-gebunden`:
  braucht das Operator-Wort (Debugger-Rechte am live Chrome). Schritt: `opencode.json`
  `mcp.chrome-devtools` pinnen (npm `chrome-devtools-mcp@1.9.0`) mit
  `--no-usage-statistics` + `--no-performance-crux`; Verifikation: ein Membran-Lauf,
  Konsole/Netz gelesen. Post-Zeile an `entscheid` steht.
- **Pfad-1-Version** `@vymalo/opencode-browser` 0.16.1 → 0.17.0 (npm gemessen; Repo
  jetzt `ADORSYS-GIS/lightbridge-opencode-toolbeit`) — Schritt: Breaking-Change-Prüfung
  0.16.1→0.17.0, dann Plugin-Version in der globalen `opencode.jsonc` setzen.
- **Cookie-Editor** als manuelles Operator-Werkzeug — `operator-gebunden`: Transfer
  Operator-Profil ↔ persistentes Profil nur per Akt mit Operator-Wort (Cookies =
  Zugangsdaten).
- **Buster (Captcha-Option b)** — `descoped mit Befund`: solange das Operator-Wort
  ausbleibt; Option (a) (Operatorin löst selbst) ist gesetzt, die Captcha reitet den
  Registrierungs-Akt. Bezahlte Solver (c) ausgeschlossen.
- **Playwright-Browser-Extension** — **nicht** eingebaut; befund-gated: erst bei
  gemessenem Pfad-1-Versagen an einer konkreten Seite.
- **Survey-pending** (Schritt je vermerkt): Buster manifest/MV3 + Store-„Updated" +
  STT-Backend + hCaptcha (Store-Seite braucht Consent-Cookies im Playwright-Profil
  oder `--headed`); BrowserMCP Lizenz/Datensendung; native-devtools-mcp Linux-Support;
  Store-Zuordnung „OpenCode Browser" `cabnfapnafjlijmbpmgjkgobhdkbmpci`;
  mcp-chrome-README (`master`); pp-browser-extension Store-IDs; generische MV3-CRX-
  Prüfung; K5 Screenshot + SessionBuddy; Chrome-146-Web-MCP-Status.

## Kernel-Force-Review — Warteschlange

Rat-Verdikt (2026-09-20): kein Register↔Code-Riss, keine Glättung. Offen bleibt die
per-Klasse-Physikbewertung der deklarierten Default-Abweichungen (`inverse-square
acoustic` 371, `inverse-square advective` 336, `inverse-square thermal` 220,
`gaussian-inverse-square em` 60 u. a.). (Schritt: je Klasse die Physik bewerten —
`grind-pro`; nie per-Zeile-Glättung.)

## Datenbank-Ausbau — offene Kandidaten

`--entrez` gebaut. Noch offen: `--ena`, `--biomodels`, `--nist`, `--cod` (HTTP 200
direct außer KEGG/Lizenz). (Schritt: nächsten Kandidaten nach `docs/SOURCE_PORT.md`
portieren — `grind-flash`.)

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress); der n=1000-FPR-Boden ist ungemessen.
(Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

## Flyby Path 2 — `termin`

Das versiegelte Blatt ist kein Auftrag. Die präregistrierte Kette muss transit-time-
korrigiert am Perigäum-Tubus gefüllt und als Auftrag registriert werden — nach dem
Ereignis (28./29.09.) ist die Vorhersage post-hoc. (Schritt: Auftrag in
`docs/auftrag/`, dann Kette füllen — `research-max`.)

## API-Keys ablegen — `operator-gebunden`

`CORE_API_KEY`, `S2_API_KEY`, `MP_API_KEY` fehlen lokal. Gehört zur
`entscheid`-Linie. (Schritt: Keys nach `.secrets.local`, dann messen.)

## ernte / termin

- **MAG-Asset** — `blockiert` (ernte): Compiler nach `voyager_odr_compiler.rs`,
  dann `sources.φ` + CI-Manifestation.
- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027). **Flyby-Path-2-Abruf** —
  `termin` (28.09., s. o.).

## Benchmark (dieses Atom)

- Kein Doppellauf: das Atom war ein Recherche-Atom (Chrome-Extension-Survey) —
  `research-max` (pro/max) für die Messung, `vision` für die Buster-Figuren,
  `council` (pro/max) für das Architektur-Urteil. Klasse „Chrome-Extension-Survey"
  ist neu; ein flash-Gegenlauf ist offen (Kandidat für das nächste Atom).

## Geteilter Baum — eigener Pfad-Satz

- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (neu),
  `docs/concepts/tools-map.md`,
  `docs/handover/handover-2026-09-20-forschung-folge108.md` (Move → `archiv/`),
  `docs/handover/handover-2026-09-20-forschung-folge109.md` (neu),
  `docs/handover/post.md` (eigene `An entscheid`-Zeile).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
