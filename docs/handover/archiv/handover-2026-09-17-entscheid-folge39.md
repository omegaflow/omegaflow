<!--
  title: Handover — Entscheid-Folge 39 (Stand 2026-09-17)
  session: Entscheid-Folge 39
  class: handover
  date: 2026-09-17
  sha256: 1765eb173471861fac117a392f89b06387d630d61afc19ddcc30ef8dc6c41aa7
  status: live
-->
# Handover — Entscheid-Folge 39 (2026-09-17)

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

## Stehender Pass (gemessen 2026-09-17)

- **HEAD** `ab45b3f3`; `origin/main == HEAD` (Fast-Forward). Fremde Linien
  committeten während der Session (`dbe647d1` forschung, `dda7e458`/`ab45b3f3`
  ernte) — der Arbeitsbaum trägt fremde gestagte `archiv/`-Renames +
  `phi/pipeline/ledger.φ` (nicht angefasst).
- **Postfach** — kein neuer externer Agenten-Eingang: neuester Ledger-Eintrag
  `1789670592` (2026-09-17, Rubin-Forum-Migrations-Thread — Publika-Kommentar);
  davor `1789662457` (GitHub-Support #4761801 GC-Thread-Update, kein
  Agenten-Reply), `1789650050` (Keller/TRISP-MLZ: sendet die NSE I(q,t)-Rohdaten
  in einigen Tagen), `1789650823` (Rubin-Summary).
- **CI-Status @`ab45b3f3`** — Watchdog-Snapshot 22:26 + `ci_manage list`:
  Ulysses-Nachfolge aktiv (`auto-dispatch` `35273075509`, `harvest-dispatch`
  `35273075476`, `harvest` `35272163623`/`35270658104` queued); letzte CDN-Runde
  success (trmm-lis `35270822493`, lis-otd `35270818836`, kcdc `35270814640`,
  iss-lis `35270812515`, ionex `35270810486`, igets `35270808236`); failure:
  swot-cdn `35270820657`, harvest `35270996845`; `gaia-sso-cdn` `35272160298`
  queued; `pioneer-cell-census` `35266366575` + `health-check` `35245084696`
  in_progress. Fremde Linien — kein Entscheid-Handlungsbedarf.
- **Zustand-Ledger** — die drei orphaned Zeilen (Postfach/PII/CI @`bc9d6a0b`)
  sind **eigenes, nie committetes Werk der Entscheid-Linie**: `entscheid-folge26`
  (`ff994faa`) schrieb sie und ließ sie uncommittet (Präzedenz `2d5f3386`);
  `bau-folge66:62` nannte sie „die besitzende Linie faltet den Postfach-Eintrag".
  Dieses Atom hat sie gefaltet: Postfach (Keller-Eingang), PII (Messung @HEAD
  dispatcht), CI (@`ab45b3f3`) — `docs/zustand/external-state.md` committet.

## Handlungsfähig — Auswahlpunkte

- `wartend` — **PII-Messung @`ab45b3f3`** (run `35273689697`, queued; letzte
  Messung 45 @`bc9d6a0b`). (Auslöser: Lauf-Abschluss → `ci_manage view
  35273689697` einmal, Wert in die PII-Zeile von `external-state.md` setzen.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande): §4 ist
  geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor, aber
  **Senden ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein
  Sendeschritt, keine Sende-Anfrage, keine Register-Zeile. (Schritt: keiner; das
  Verbot steht.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Wartend (extern) — kein Auswahlpunkt, kein Handlungsschritt

- `wartend` — GitHub GC `#4761801`: neuester Eingang `1789662457` (Zendesk-Echo
  des eigenen Kommentars, kein Agenten-Reply). (Auslöser: Postfach-Eingang.)
- `wartend` — GitHub Privacy-Löschung: Resend `01a0b032` (gesendet
  `1789662499`), keine Antwort. (Auslöser: Postfach-Eingang.)
- `wartend` — NSE/Haug I(q,t): TRISP/MLZ (Keller) sendet die Rohdaten
  („in einigen Tagen", `1789650050`). (Auslöser: Postfach-Eingang → Datenannahme
  + CDN-Manifestation.)
- `wartend` — fünf Sonden-Anfragen (NSSDC Voyager/Mariner 10/Viking, Cassini,
  Juno): gesendet 2026-09-16. (Auslöser: Postfach-Eingang.)
- `wartend` — Rubin-Review (Shaughnessy): Forum zieht 2026-09-24 auf
  `rubin.community`. (Auslöser: Postfach/Forum bei Eingang.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/zustand/external-state.md` (Postfach/PII/CI
  @`ab45b3f3`; folge26-Orphan gefaltet), `docs/handover/post.md`
  (An-forschung-Zeile: WWLLN entschieden), `docs/handover/handover-2026-09-17-
  entscheid-folge39.md`, `docs/handover/archiv/handover-2026-09-17-entscheid-
  folge38.md` (Move).
- Fremde uncommittete Arbeit (nicht anfassen): die gestagten `archiv/`-Renames
  (`handover-2026-09-16-entscheid-folge24`, `-forschung-folge44`, `-forschung-
  folge51`) und `phi/pipeline/ledger.φ` (fremd-modifiziert). Nie ein nacktes
  `git commit`.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-mails.md` und die
  drei `adoption-*.body.txt`.

## Benchmark

- Kein Doppellauf: die Ledger-Faltung ist Routine (flash-Klasse), keine gemessene
  Benchmark-Klasse; kein pro/max-Dispatch nötig. Die Besitz-Messung (git log +
  Handover-Vergleich) lief im Hauptkontext.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
