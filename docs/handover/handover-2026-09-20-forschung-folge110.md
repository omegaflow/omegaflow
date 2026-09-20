<!--
  title: Handover — Forschung-Folge 110 (Stand 2026-09-20)
  session: Forschung-Folge 110
  class: handover
  date: 2026-09-20
  sha256: 35818ad81431999d4af0994fe5688b085e0a0f13bf49b7a8733486f211d12663
  status: live
-->
# Handover — Forschung-Folge 110 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 110)

- **HEAD** `5b59c2ab` (== `origin/main`); `git_safety` Snapshot
  `refs/safety/1789917637`. Der Baum trägt bei Beginn fremde staged Arbeit
  (entscheid-Handover-Move + `folge62`) — nicht angefasst.
- **Postfach** — neuester Ledger-Eingang `1789918147` (GitHub-Dritt-OAuth-App
  „ISH Chat"); kein neuerer Eingang (gemessen, `state/mail/mail_ledger.φ`, 111
  Zeilen). Kein forschung-eigener Post; zwei `An entscheid`-Zeilen (Forschung:
  Chrome DevTools MCP; Ernte: Lasair) stehen. Zustand-Eintrag zitiert, nicht kopiert.
- **CI** (Watchdog 17:06 + `ci_manage`): `te-gate` `35513982359` **in_progress**,
  `allwise-cdn` `35516186859` success, `health-check` `35505471538` in_progress;
  `ci-check`-Kette cancelled/failed; `release-build` `35517958024` @`aa120ac6`
  success (windows-Matrix hält).

## Kernel-Force-Review — read-only Manifestations-Audit (härtester undatiert)

Die per-Klasse-Physik wurde gemessen (`grind-pro`, Klasse-Tabelle: inverse-square
acoustic 371 · inverse-square advective 336 (+292 fold) · inverse-square thermal 220 ·
gaussian-inverse-square em 59 · … · gaussian-inverse-square gravity 9). Ein
916-Zeilen-Massed it des Sub-Agenten auf `phi/sources.φ` (Kernel-Token je Klasse
überschrieben) wurde **revertiert** — es ist die verbotene per-Zeile-Glättung auf
einem geteilten Kanon-Register. **Rat-Verdikt (2026-09-20): die Deklarationen bleiben
die Messung (A = A); kein Register↔Code-Riss; eine Korrektur ist ein Architektur-Akt
(Operator + Rat, `phi/canon.φ`-Gate), nie ein Sub-Agent-Wort.** Kleinster nächster
Atom: read-only Manifestations-Audit — je Klasse die strahlenden Felder gegen die
Skalar-Katalog-Zeilen (`planet_mass`, `logg`, `romplus_quake_mw`) zählen, dann **eine**
Handover-Zeile mit der per-Klasse-Begründung; kein Register-/Code-Edit.
(Schritt: `sgrep`/`archive_search --root phi` — `research-max`.)

## Datenbank-Ausbau — offene Kandidaten

`--entrez` gebaut. Noch offen: `--ena`, `--biomodels`, `--nist`, `--cod` (HTTP 200
direct außer KEGG/Lizenz). (Schritt: nächsten Kandidaten nach `docs/SOURCE_PORT.md`
portieren — `grind-flash`.)

## Browser-Anbindung — Rest

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md` aktualisiert: Pfad-1
`@vymalo/opencode-browser` **0.17.0 gesetzt** (Drop-in gemessen — 0.17.0-BREAKING
betrifft nur `lightbridge`); Buster MV3/`minimum_chrome_version: 123`/hCaptcha
nicht unterstützt/lokales Whisper; BrowserMCP Apache-2.0 lokal; native-devtools-mcp
Linux nicht unterstützt; Store-IDs + `chrome-devtools-mcp` 1.9.0-Flags gemessen.

- **Chrome DevTools MCP anbinden** (Ziel i) — `operator-gebunden`: `opencode.json`
  `mcp.chrome-devtools` pinnen (`chrome-devtools-mcp@1.9.0`, `--no-usage-statistics`
  `--no-performance-crux`); Post-Zeile an `entscheid` steht.
- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Profil — `operator-gebunden`.
- **Buster-Store-„Updated"-Datum** — `pending`: CWS-Listing nicht skriptbar.
  (Schritt: im echten Browser lesen.)
- **Playwright-Browser-Extension** — `befund-gated`: erst bei gemessenem Pfad-1-Versagen.

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress); der n=1000-FPR-Boden ist ungemessen.
(Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

## Flyby Path 2 — `termin`

Das versiegelte Blatt ist kein Auftrag. Die präregistrierte Kette muss
transit-time-korrigiert am Perigäum-Tubus gefüllt und als Auftrag registriert werden —
nach dem Ereignis (28./29.09.) ist die Vorhersage post-hoc. (Schritt: Auftrag in
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

- Klasse „Chrome-Extension-Survey": `general`/flash **$0.0187** (deepseek-v4-flash)
  lieferte die vollständige Messung (alle Survey-Pendings) gegen den früheren
  `research-max`-Lauf (folge109) → **flash gewinnt**. Gegenprobe: `grind-pro`/pro
  **$0.1005** lieferte beim Kernel-Force-Review einen 916-Zeilen-Overreach
  (revertiert) — teurer und falsch; `council`/pro $0.0069 trug das Urteil.
  (`session_burn --top 15 --dir …`, 2026-09-20.)

## Geteilter Baum — eigener Pfad-Satz

- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (Pendings → gemessen),
- `docs/handover/handover-2026-09-20-forschung-folge109.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge110.md` (neu),
- `~/.config/opencode/opencode.jsonc` (global, untracked: Plugin 0.16.1 → 0.17.0).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
