<!--
  title: Handover — Entscheid-Folge 38 (Stand 2026-09-17)
  session: Entscheid-Folge 38
  class: handover
  date: 2026-09-17
  sha256: a20735ca9d4e4a767377cf2d4e2b1d3344b9b596b91935cd3b9d5cb85e26649b
  status: live
-->
# Handover — Entscheid-Folge 38 (2026-09-17)

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

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — `state/mail/mail_ledger.φ`, neuester Eintrag `1789670592`
  (2026-09-17): Rubin-Forum-Migrations-Thread (Publika-Kommentar, kein
  Agenten-Eingang). Davor `1789662457` (Zendesk-Echo des eigenen
  GC-Nachfass-Kommentars), `1789662457`-Vorgänger `1789650823` (Rubin-Summary).
  **Kein neuer externer Agenten-Eingang** seit Folge 37.
- **CI-Status @HEAD `8d821322`** — Watchdog-Snapshot 2026-09-17T21:21
  (HEAD wanderte während der Session auf `dc9291ad`, ernte-folge70; der Snapshot
  gilt für `8d821322`):
  aktiv `allwise-cdn` `35263717637`, `ned-cdn` `35258227020`, `ps1-cdn`
  `35257451098`, `health-check` `35245084696`, `planetary-odf-cdn`
  `35231817955`; failed (attempt 1) `swot-cdn` `35251359490`, `gedi-cdn`
  `35250788545`, `ci-check` `35250778775`. Watchdog-Retry, kein
  Entscheid-Handlungsbedarf.
- **Zustand-Ledger** — `docs/zustand/external-state.md` trägt fremde
  uncommittete Hunks. PII-Zeile steht auf `bc9d6a0b`; @HEAD `8d821322` nicht
  setzbar, solange die fremden Hunks uncommittet sind.

## Handlungsfähig — Auswahlpunkte

- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande):
  §4 ist geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor, aber
  **Senden ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein
  Sendeschritt, keine Sende-Anfrage, keine Register-Zeile — die Entwürfe bleiben
  ungesendet. (Schritt: keiner; das Verbot steht.)
- `operator-gebunden` — ESP32-Modul, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `blockiert` — Zustand-PII-Zeile @HEAD `8d821322` setzen, blockiert solange
  fremde Hunks in `docs/zustand/external-state.md` uncommittet sind.
  (Schritt: fremde Arbeit committen lassen, dann PII-Zeile @`8d821322` setzen.)

## Wartend (extern) — kein Auswahlpunkt, kein Handlungsschritt

- `wartend` — GitHub GC `#4761801`: neuester Eingang `1789662457` (Zendesk-Echo
  des eigenen Kommentars, kein Agenten-Reply). (Auslöser: Postfach-Eingang.)
- `wartend` — GitHub Privacy-Löschung: Resend `01a0b032` (gesendet
  `1789662499`), keine Antwort. (Auslöser: Postfach-Eingang.)
- `wartend` — NSE/Haug I(q,t): TRISP/MLZ (Keller) sendet die Rohdaten
  („a few days", `1789631158`). (Auslöser: Postfach-Eingang → Datenannahme +
  CDN-Manifestation.)
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

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-entscheid-folge38.md`,
  `docs/handover/archiv/handover-2026-09-17-entscheid-folge37.md` (Move),
  `docs/handover/post.md` (WWLLN-Zeile entfernt, ernte-Post gesetzt),
  `docs/SOURCE_PORT.md` (§2: stage/ ausgelagert, master.φ/sources_potential_* als
  abwesend gemessen), `phi/declined_sources.φ` (WWLLN-WGLC als `decline
  aggregated-index` registriert).
- Pipeline-Auslese (ausgeführt 2026-09-17): `stage/` (24), `meteo_harvest/` (159),
  `weights_*.txt` (47) + Probe-Ausgänge (7) verschoben nach
  `archive-root/pipeline-auslese-2026-09-17/` (gitignored → reine Dateisystem-Bewegung,
  umkehrbar; `NOTE.md` als Marker). `phi/pipeline` 563 → 333 Dateien.
- Fremde uncommittete Arbeit (nicht anfassen): `docs/zustand/external-state.md`;
  fremd gestagt: die drei `archiv/`-Renames (folge24, forschung44, forschung51).
  Nie ein nacktes `git commit`.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-mails.md` und die
  drei `adoption-*.body.txt`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
