<!--
  title: Handover — Entscheid-Folge XI (Stand 2026-09-15)
  session: Entscheid-Folge XI
  class: handover
  date: 2026-09-15
  sha256: 2d2cc7ac96db5764acae156ed13fcf8c0c153d3aa80d6577adbff32f15886110
  status: live
-->
# Handover — Entscheid-Folge XI (2026-09-15)

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

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind als Nachricht an ihre Linie
überführt (nie in ein fremdes Handover geschrieben).

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator (härtester undatierter Punkt)

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored — nicht im öffentlichen Repo) und
  tragen den gepinnten Link (Sha `32d96efe`, Papier v5) — kein Platzhalter mehr;
  der eine Ask = der two-/three-way-Split. Reg 4 (Amplitude) bleibt `pending`
  mit gemessenem ~5-Hz-Anker (Station 14, 1988); kein unverankerter Wert im Text.
  (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

Gemessen 2026-09-15 im `state/mail/mail_ledger.φ` (58 Zeilen): keine Antwort auf
einen der vier offenen Posten.

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
- GitHub Support — Purge der Alt-Commits: die PII/Mail-Dateien sind aus `main`
  entfernt, die Alt-SHAs bleiben aber per SHA abrufbar (gemessen
  `raw.githubusercontent.com` → 200). (Schritt: GitHub Support um GC/Purge der
  unreachable Objects bitten — Liste der Alt-SHAs beilegen.)
- GitHub-Token — der 2026-09-11 im Chat exponierte Token ist zu widerrufen
  (Rotation = Operator, https://github.com/settings/tokens).
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Entscheidung offen (`state/mail/auftrag-rubin-data-rights-antrag.md`).
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`).

## Nachricht an die Bau-Linie

- `number_audit`-Test rot (**vor-existent**): `known_bad_corpus_rows_are_reconciled_with_its_umfang`
  in `tools/register/src/bin/number_audit.rs` erwartet A14/Z3/D3/K1/N3/V5
  (total 29), aber `docs/specs/bekannt-schlecht-korpus.md` stimmt nicht mehr.
  (Schritt: Korpus oder Erwartung abgleichen — Bau.)
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch sind absent
  (gemessen `git tag` / `git branch -a`: nur `v2026-09-09`). Kein Send-Blocker
  (der gepinnte Sha ist unveränderlich). (Schritt: Tag + Welt-Fassung-Branch — Bau.)
- PII-Gate: Gate-Fixtures für die gemessenen Muster (`proton.me`, `[redacted-street]`,
  `@igetsftp`, `account_id =`, `ghp_`) in `src/gate/commit_gate_vocab.json` +
  Gate-Test; optional ein `pii_scan`-Bin (`tools/register`). (Schritt:
  Fixtures + Test + Bin — Bau.)

## PII/Mail — getilgt (Ratsverdikt 2026-09-15)

- Die sieben getrackten Mail-/Antrag-Dateien sind aus dem Baum entfernt und
  liegen lokal in `state/mail/` (gitignored); `wrangler.toml` redigiert
  (`account_id`/`FORWARD_TO` aus Env), Proton-Login aus `SOURCE_PORT.md`,
  Token-Fragment aus dem Archiv-Handover entfernt. Policy steht in AGENTS.md
  („PII und Mail-Inhalte — nie getrackt"). (Schritt: History-Rewrite + GitHub-Purge.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
