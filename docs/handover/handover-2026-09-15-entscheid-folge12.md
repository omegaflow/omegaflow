<!--
  title: Handover — Entscheid-Folge XII (Stand 2026-09-15)
  session: Entscheid-Folge XII
  class: handover
  date: 2026-09-15
  sha256: 0af5a68b5c0c7011a49e76f6b40462cc6aedd15d8b46814744161c91ad6f4441
  status: live
-->
# Handover — Entscheid-Folge XII (2026-09-15)

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

## PII-History-Rewrite + GitHub-Purge (härtester undatierter Punkt)

- Vier PII-Pfade liegen noch in `origin/main`s erreichbarer Historie (gemessen
  `git merge-base --is-ancestor`): `docs/auftrag/auftrag-igets-sftp-passwort-antrag.md`,
  `docs/auftrag/auftrag-lisa-pathfinder-psd-antrag.md`,
  `docs/auftrag/auftrag-flyby-doppler-rohdaten.md`,
  `docs/auftrag/gavo-dc-account-anfrage.md`; vier weitere
  (`auftrag-adoption-mails`, `auftrag-rubin-data-rights-antrag`,
  `auftrag-sonden-rohdaten-anfragen`, `archiv/antares-konto-2026-09-05`) sind
  bereits unerreichbar. Der Purge-Antrag mit Commit-/Blob-Liste liegt lokal in
  `state/mail/github-purge-request.md` (gitignored). (Schritt: Operator-Consent →
  Rewrite im Clone `git clone --no-local` +
  `git filter-branch --force --index-filter 'git rm --cached --ignore-unmatch
  <die vier Pfade>' --prune-empty -- --all`, Force-Push; danach Antrag senden —
  `https://support.github.com/contact`.)
- Der gepinnte Paper-Sha `bc7e7ac` (in den drei adoption-Entwürfen) ist ein
  Nachfahre aller PII-Commits — jeder Rewrite ändert ihn. Nach dem Push: die
  drei Entwürfe in `state/mail/adoption-mails.md` auf den neuen Sha umhängen.
  (Schritt: `git log --format=%H -1 -- docs/paper/twenty-second-band-ground-chain.md`.)

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha `bc7e7ac`; der
  eine Ask = two-/three-way-Split. Reg 4 (Amplitude) bleibt `pending` mit
  gemessenem ~5-Hz-Anker (Station 14, 1988) — kein unverankerter Wert im Text.
  (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`. Vor dem
  Send den Rewrite/Re-Pin abwarten.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

Gemessen 2026-09-15 im `state/mail/mail_ledger.φ` (58 Zeilen): keine Antwort auf
einen der offenen Posten.

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
- GitHub-Token — der 2026-09-11 im Chat exponierte Token ist zu widerrufen.
  (Schritt: Operator, https://github.com/settings/tokens.)
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
- PII-Gate: Gate-Fixtures für die gemessenen PII-Muster (Mail-Domain,
  Straßenname, SFTP-Login, Cloudflare-account_id, Token-Präfix) in
  `src/gate/commit_gate_vocab.json` + Gate-Test; optional ein `pii_scan`-Bin
  (`tools/register`). (Schritt: Fixtures + Test + Bin — Bau.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
