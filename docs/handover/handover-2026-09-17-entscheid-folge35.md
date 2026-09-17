<!--
  title: Handover — Entscheid-Folge 35 (Stand 2026-09-17)
  session: Entscheid-Folge 35
  class: handover
  date: 2026-09-17
  sha256: 3827bd1cda4fc8b23966f70b2dc0dfdb95e08aa2a3542775b6c3cdaf4f845a2c
  status: live
-->
# Handover — Entscheid-Folge 35 (2026-09-17)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — `state/mail/mail_ledger.φ` gemessen (Session-Read, 224 Zeilen):
  kein neuer externer Eingang seit dem Rubin-Forum-Summary (`1789650823`). Davor
  Keller/TRISP (`1789631158` sendet die Daten, „a few days"; Danke-Antwort raus
  `1789650050`), GitHub-PAT `omegaflow-ci-write` (`1789641380`), Cloudflare-
  Login-Codes (`1789628784`–`1789629095`, IP `185.51.184.2`) — gemessen, nicht
  bewertet. Die fünf Sonden, Rubin-Review, GitHub Privacy bleiben ohne Antwort.
- **CI-Status @HEAD** — HEAD bei Session-Beginn `0548776e`, während der Session
  durch fremde Linien auf `f54d35e2` bewegt (forschung/research/ernte). Watchdog-
  Snapshot `/tmp/opencode/ci_status.md` (Poll 15:05Z): `pii-exposure`
  `35230489744` **failure = `exit 2` (PII retrievable — by design)** @`5dcdda0d`
  (45 Kombinationen, unverändert); `paper-check` `35228278279`/`35224760511`
  failure (stale — sha-Fix in `5dcdda0d`); CDN-Runs aktiv (demeter, ps1,
  physionet ×2, gaia-xp-full queued ×3, planetary-odf). Die aktiven Läufe sind
  statische Archive; ihr `updated_at` friert (langer Step), die `queued`-Folger
  sind Concurrency-Wartende. (Schritt: `ci_manage list`/`view`.)
- **Zustand-Ledger** — `docs/zustand/external-state.md` trägt weiter **fremde
  uncommittete Hunks** (Postfach/PII/CI-Zeilen). Die PII-Zeile steht auf
  `bc9d6a0b`; die Messung @`5dcdda0d` (45, unverändert) wird **zitiert, nicht
  überschrieben**. Eigener Hunk erst, wenn die fremden Hunks committet sind.
  (Schritt: fremde Arbeit committen lassen, dann PII-Zeile @`5dcdda0d` setzen.)

## GitHub GC/Privacy — beide Anträge gesendet, Antwort offen

Der E-Mail-Kanal (GC) ist tot (`1789624439`, „Support Ticket Declined", `[RRKJ24-2JN2V]`);
die angemeldete Support-Seite ist die Route. **Vollzogen 2026-09-17:**

- **GC-Ticket `#4761801`** (Kategorie Repositories,
  `support.github.com/ticket/personal/0/4761801`) — Nachfass-Reply aus
  `state/mail/github-gc-webform.md`, **vom Operator abgeschickt**. Antwort offen.
- **Privacy** — `support.github.com/contact/privacy` ist **kein Freitext-Formular**,
  sondern ein Router: `access`/`change` → Kontoeinstellungen, `delete` →
  Konto-Selbstlöschung, `other` → `mailto:privacy@github.com`. Content-Löschung
  daher per E-Mail: **erneut gesendet 2026-09-17, Resend
  `01a0b032-923a-7596-89fe-d071fabd050c`** (Erstversuch `01a0aded-819e…` ohne
  Antwort).
- Beide Consent-Akte sind damit vollzogen; der Punkt ist ab jetzt ein normaler
  Warte-Punkt auf Antwort, kein operator-gebundener Consent mehr. (Schritt:
  Postfach-Eingang — die 15. Wiederholung ist beendet.)
- **Benchmark** — Route-Messung an `general` (flash) delegiert: die Klasse
  „Routine-Recherche" ist geschlossen (flash-Sieger, `docs/concepts/tools-map.md`,
  2026-09-16) — flash-first, keine Doppelmessung. (Burn-Zahl beim Abschluss aus
  `session_burn`/opencode.db, nicht fabriziert.)

## NSE/Haug — Datenannahme

Keller (TRISP/MLZ) sendet die I(q,t)-Rohdaten („a few days", `1789631158`; Danke
raus `1789650050`). Bei Eingang: Datenannahme registrieren (`pending` bis dahin).
(Schritt: Postfach-Eingang → `state/mail/` + CDN-Manifestation.)

## Warten auf Rückmeldung (extern)

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen. (Schritt: Postfach-Eingang.)
- **Rubin-Review** (Shaughnessy, high volume) — offen; Forum zieht am
  **2026-09-24** auf `rubin.community`. (Schritt: Postfach/Forum bei Eingang.)
- **GitHub GC `#4761801`** + **Privacy** — siehe oben (operator-gebunden).
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Operator-gebundene Punkte

- **adoption-Block** — §4 bleibt offen, forschung-eigen
  (`docs/paper/twenty-second-band-ground-chain.md:188–190`). Post `An forschung`
  wurde von forschung gefaltet (nicht mehr in `post.md`). (Schritt: forschung
  benennt §4-Abschluss; danach holt entscheid das Sende-Wort ein.)
- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

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

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-entscheid-folge35.md`,
  `docs/handover/archiv/handover-2026-09-17-entscheid-folge34.md` (Move),
  `docs/handover/post.md` (eigene Zeile an bau/ernte). Entwürfe in `state/mail/`
  sind gitignored (nie getrackt). Fremde uncommittete Arbeit (nicht anfassen):
  die vielen `.github/workflows/*.yml` (bau), `docs/concepts/tools-map.md`,
  `docs/zustand/external-state.md`, `phi/sources.φ`, `tools/...`; fremd gestagt:
  die `archiv/`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
