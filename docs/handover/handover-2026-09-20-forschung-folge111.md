<!--
  title: Handover — Forschung-Folge 111 (Stand 2026-09-20)
  session: Forschung-Folge 111
  class: handover
  date: 2026-09-20
  sha256: 20846ac50a92ca0a96e8e2cb8afa35d48554d354d24bd0cada24869a6f9f3f1a
  status: live
-->
# Handover — Forschung-Folge 111 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 111)

- **HEAD** `fa44f315` (== `origin/main`); `git_safety` Snapshot
  `refs/safety/1789919142`. Der Baum trägt fremde uncommittete/staged Arbeit
  (entscheid- + ernte-Handover-Moves, `folge62`/`folge113`, `phi/sources.φ`,
  `phi/pipeline/ledger.φ`, `phi/harvest.φ`, `phi/blocked_sources.φ`, `post.md`,
  `external-state.md`) — nicht angefasst.
- **Postfach** — neuester Ledger-Eingang `1789918147` (Dritt-OAuth-App „ISH Chat",
  `read:user`/`user:email`); kein neuerer Eingang. Die angefragten API-Keys sind
  eingetroffen: CORE `1789899008`, Semantic Scholar `1789899110`, Materials Project
  (OAuth) `1789898076` — Zustand-Ledger `docs/zustand/external-state.md` zitiert,
  nicht kopiert. Kein forschung-eigener Post.
- **CI** (Watchdog 17:06 + `ci_manage` 2026-09-20): `te-gate` `35513982359`
  **in_progress** @`7d0a1272`; `ci-check` `35520538766` in_progress;
  `health-check` `35519449450` pending; `release-build` `35517958024` success.

## Manifestations-Audit — Ergebnis (read-only, 2026-09-20)

2018 `field`-Zeilen in `phi/sources.φ`, 22 Kernel×Kraft-Klassen. Die drei
Skalar-Familien sitzen ausschließlich in Non-Default-Klassen: `gaussian-inverse-square
gravity` 9 Felder = **9/9 Skalar-Katalog-Zeilen** (cb/lmxb-Massen, `planet_mass`,
`planet_radius`, 3× logg) — 0 strahlende Zeitreihen; `gaussian-inverse-square
seismic-surface` 1 Feld = `romplus_quake_mw` (Skalar). Die Skalar-Zeilen tragen den
Kraft-Token des gespeisten Feldes (gravity/seismic-surface), nicht einen
Strahlungs-Token; Register-Default (`src/mathematikerin/force.rs:53-66`) ist
gravity→inverse-square und seismic-surface→erfc. Die übrigen 20 Klassen: 0
benannte Skalar-Zeilen, ihre Zählungen sind strahlende Felder. **Rat-Verdikt
(2026-09-20) bestätigt: kein Register↔Code-Riss; Deklaration = Messung (A = A);
eine Kernel-Korrektur bleibt Architektur-Akt** (Operator + Rat, `phi/canon.φ`-Gate),
nie ein Sub-Agent-Edit. Kein Register-/Code-Edit in diesem Atom.

## Manifestations-Audit — Rest (`pending`, härtester undatiert)

- **Skalar/Strahlend-Klassifikation der übrigen katalog-abgeleiteten Felder** —
  em-Magnituden (`denis_j_mag` `phi/sources.φ:8832`, `cb_mag1` `:8738`,
  `extinction_rv` `:8863`) und thermal-Temperaturen (`pastel_teff_k` `:9078`,
  `polarbase_teff_k` `:9096`) tragen den statischen Katalog-Charakter; ob sie als
  strahlend (flux_from_mag) oder skalar zählen, ist ungemessen. (Schritt:
  `sgrep`/`archive_search --root phi` je Feld, dann eine Handover-Zeile —
  `general`.)
- **Doc↔Register-Token-Drift** — `docs/concepts/archivar-mathematikerin.md:19`
  nennt „5 tokens … optional `tau`", das Register trägt 8
  (`field key name kernel force unit ttl vx vy`). (Schritt: Konzept-Doc gegen
  Register abgleichen — `general`.)
- **„+292 fold" (advective)** aus dem folge110-Pass nicht reproduziert — gemessen
  sind 336 `inverse-square advective`; die Fold-Semantik ist ungemessen. (Schritt:
  Herkunft der Zahl in `phi/sources.φ` messen — `general`.)

## Datenbank-Ausbau — offene Kandidaten

`--ena` gebaut (Port 2026-09-20: `tools/utils/src/bin/archive_search/ena.rs` +
Dispatch/Usage/tools-map; `cargo check -p omegaflow-utils --all-targets` 0 Fehler,
0 Warnungen; Hazard gemessen: ENA liefert ohne `fields=` den vollen ~70-MB-Feldsatz
→ `fields=` Pflicht). Noch offen: `--biomodels` (EBI, 200 JSON), `--nist`
(NIST WebBook, 200 HTML → `parser-def`-Entscheid), `--cod` (Crystallography Open
Database, 200 JSON — sauberster nächster Port). (Schritt: `--cod` nach
`docs/SOURCE_PORT.md` portieren — `grind-flash`.)

## Browser-Anbindung — Rest

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md` (Pfad-1
`@vymalo/opencode-browser` **0.17.0** gesetzt, Drop-in gemessen; Buster MV3/
`minimum_chrome_version: 123`/hCaptcha/Whisper nicht unterstützt; BrowserMCP
Apache-2.0 lokal; native-devtools-mcp Linux nicht unterstützt; Store-IDs +
`chrome-devtools-mcp` 1.9.0-Flags gemessen).

- **Chrome DevTools MCP anbinden** (Ziel i) — `operator-gebunden`: `opencode.json`
  `mcp.chrome-devtools` pinnen (`chrome-devtools-mcp@1.9.0`, `--no-usage-statistics`
  `--no-performance-crux`); Post-Zeile an `entscheid` steht in `post.md`.
- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Playwright-Profil —
  `operator-gebunden`.
- **Buster-Store-„Updated"-Datum** — `pending`: CWS-Listing nicht skriptbar.
  (Schritt: im echten Browser lesen.)
- **Playwright-Browser-Extension** — `befund-gated`: erst bei gemessenem
  Pfad-1-Versagen.

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` (in_progress @`7d0a1272`); der n=1000-FPR-Boden ist
ungemessen. (Schritt: `ci_manage view 35513982359`; bei rot `ci_manage log`.)

## Flyby Path 2 — `termin`

Das versiegelte Blatt ist kein Auftrag. Die präregistrierte Kette muss
transit-time-korrigiert am Perigäum-Tubus gefüllt und als Auftrag registriert
werden — nach dem Ereignis (28./29.09.) ist die Vorhersage post-hoc. (Schritt:
Auftrag in `docs/auftrag/`, dann Kette füllen — `research-max`.)

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

- Klasse „Manifestations-Audit" (`general`/flash) und „Source-Port ENA"
  (`grind-flash`/flash) — beide flash, beide korrekt (Audit vollständig gegen den
  Baum, `cargo check` 0/0). `grind-flash` **$0.0327** (n=1); `general` $0.0374 über
  n=2 (Aggregat, nicht atom-scharf). Keine pro/max-Gegenprobe — die
  Routine-Agent-Klasse ist durch den gemessenen Sieger geschlossen (2026-09-16,
  flash 2.4–11× günstiger, identisches Ergebnis), kein Double per Benchmark-Regel.
  (`session_burn --top 15`, 2026-09-20.)

## Geteilter Baum — eigener Pfad-Satz

- `docs/concepts/tools-map.md`,
- `tools/utils/src/bin/archive_search.rs`, `.../net.rs`, `.../server.rs`,
  `.../web.rs`,
- `tools/utils/src/bin/archive_search/ena.rs` (neu),
- `docs/handover/handover-2026-09-20-forschung-folge110.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge111.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
