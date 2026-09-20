<!--
  title: Handover — Forschung-Folge 115 (Stand 2026-09-20)
  session: Forschung-Folge 115
  class: handover
  date: 2026-09-20
  sha256: 8d688d52ffa39ce739cdcff423918e498154d550ba431628c2c17d51e187da13
  status: live
-->
# Handover — Forschung-Folge 115 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 115)

- **HEAD** `1963be30` (== `origin/main`; sauberer Baum — `git_safety` „the working
  tree equals HEAD, nothing to record"). Der Vorgänger-Handover folge114 nannte
  `813a82c4`; das ist überholt.
- **Postfach** — letzte Ledger-Zeile `1789918147` (unverändert seit Ernte-Folge
  113, kein Neueingang); `external-state.md` Postfach-Eintrag fortgeschrieben.
- **CI** — Watchdog 18:10:36: **in_progress** `tools-build` `35521838130`,
  `ps1-cdn` `35521569959`, `ci-check` `35520538766`, `health-check` `35519449450`;
  **failed** `ci-check` `35517957987`/`35517136467`/`35516232478`; `te-gate`
  `35513982359` @`7d0a1272` weiter **in_progress** (kein Update seit Start 13:36Z,
  kein Verdikt). `external-state.md` CI-Eintrag auf `1963be30` fortgeschrieben.
- **Register gegen Baum gehalten:** **MAG-Asset** ist erledigt —
  `bc_mpo_mag_compiler.rs`, `sources.φ:8624-8633`, `harvest.φ:2-8`, Workflow
  `bc-mpo-mag-cdn.yml` stehen, CDN-Asset gemessen present (1733432 B, sha256
  `8f83ec79…`). Der Register-Punkt war stale und ist geschlossen, nicht getragen.

## Browser-Anbindung — Rest (härtester undatierter, `operator-gebunden`)

Survey `docs/surveys/survey-2026-09-20-browser-anbindung.md`. Die survey-eigenen
Bau-Schritte sind am Baum gemessen: die Drei-Pfade-Tabelle steht in
`tools-map.md:247-270`, die globale Config pinnt `@vymalo/opencode-browser@0.17.0`
— die Versionslücke ist geschlossen (`tools-map.md:270` korrigiert).

- **Chrome DevTools MCP anbinden** (Ziel i) — `operator-gebunden`: Post an
  `entscheid` steht (post.md:18); Telemetrie-Flags sind Bedingung.
- **Cookie-Editor-Transfer** Operator-Profil ↔ persistentes Playwright-Profil —
  `operator-gebunden`.
- **Buster-Store-„Updated"-Datum** — `pending`: CWS-Listing nicht skriptbar
  (Schritt: im echten Browser lesen).
- **Playwright-Browser-Extension** — `befund-gated`: erst bei gemessenem
  Pfad-1-Versagen.

## TE-Gate-Verdikt — `wartend`

`te-gate` `35513982359` in_progress @`7d0a1272`, kein Update seit Start (>5 h);
der n=1000-FPR-Boden ist ungemessen. (Schritt: `ci_manage view 35513982359`; bei
rot `ci_manage log 35513982359`.)

## Flyby Path 2 — `termin` (28./29.09.)

Auftrag registriert: `docs/auftrag/auftrag-flyby2-kette.md` (sha256 `3df49a1c…`)
— die präregistrierte Kette transit-time-korrigiert am Perigäum-Tubus **vor** dem
28.09. füllen. Kanal-Ernte-Zustand gemessen: RTSW/Kp/Swarm/OMNI2/ACE registriert,
**DSCOVR fehlt**, `ledger.φ` trägt keinen Kette-Ernte-Zustand. (Schritt: Kanalzellen
messen + Addendum committen — `research-max`, vor dem 28.09.)

## API-Keys — an `entscheid` übergeben

`CORE_API_KEY`, `S2_API_KEY`, Materials Project (OAuth) sind eingetroffen; Ablage
in `.secrets.local` + Messung `operator-gebunden`. Post an `entscheid` steht
(post.md:22).

## ernte / termin

- **NSE/Haug** — `wartend`: Trigger Dateieingang.
- **BepiColombo MORE** — `termin` (~April 2027).

## Benchmark (dieses Atom)

Ein flash-Lauf: „Flyby Path 2 chain state" (`general`, `$0.0091`). Ergebnis: der
exakte Ketten-Zustand (Siegel + σ-Metrik stehen, Kette offen) und die Auftrags-Lücke
(kein Registrierungs-Auftrag in `docs/auftrag/`). Kein pro/max-Double — die
Routine-Klasse ist durch den gemessenen flash-Sieger geschlossen (2026-09-16).
Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/zustand/external-state.md` (Postfach- + CI-Eintrag),
- `docs/concepts/tools-map.md` (Pfad-1-Version),
- `docs/auftrag/auftrag-flyby2-kette.md` (neu),
- `docs/handover/handover-2026-09-20-forschung-folge114.md` (Move → `archiv/`),
- `docs/handover/handover-2026-09-20-forschung-folge115.md` (neu).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
