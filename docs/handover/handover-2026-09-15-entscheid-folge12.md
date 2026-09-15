<!--
  title: Handover — Entscheid-Folge XII (Stand 2026-09-15)
  session: Entscheid-Folge XII
  class: handover
  date: 2026-09-15
  sha256: b3c1c3a9875fefc90eacb7648f8084d7be3f75526e372b56d3452bc65775c00d
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

## GitHub-Purge der PII-Alt-Commits (härtester undatierter Punkt)

- Der PII-History-Rewrite ist ausgeführt: die vier noch erreichbaren Pfade
  (`auftrag-igets-sftp-passwort-antrag`, `auftrag-lisa-pathfinder-psd-antrag`,
  `auftrag-flyby-doppler-rohdaten`, `gavo-dc-account-anfrage`) sind samt der vier
  bereits unerreichbaren aus `main`s History entfernt (Baum-Inhalt unverändert;
  Force-Push `afae680 → b2bc1a2`, gemessen `git log main -- <8 Pfade>` leer).
  Alle acht PII-Commits sind damit unerreichbar, aber per SHA noch abrufbar.
  Der Purge-Antrag ist gesendet (Ticket **#4761482**, bestätigt
  `support@githubsupport.com` 2026-09-15). (Schritt: GitHubs Purge-Bestätigung
  abwarten; danach den alten Roh-Link auf 404 prüfen —
  `curl -sI https://raw.githubusercontent.com/omegaflow/omegaflow/<alt-SHA>/docs/auftrag/gavo-dc-account-anfrage.md`.)

## GitHub-Token — auf zwei Tokens konsolidiert (gemessen)

- Zwei Jobs → zwei Tokens, beide unter `omegaflow`: **T1 Schreiben** (classic,
  `repo`+`workflow`; lokal `GH_TOKEN` + Actions `OMEGAFLOW_TOKEN`) und **T2
  Lesen** (`GITHUB_SEARCH_TOKEN`, fine-grained read-only; nur `archive_search`).
- Messung (API): der alte omegaflow-Token (`ghp_tWpB…`) lag doppelt zu `gh-cli`;
  johannestyroller `omegaflow-cdn` war der CI-Token (public_repo);
  `.git-credentials` trug zwei tote (401) und einen johannestyroller-OAuth-Token
  — git nutzt sie nicht (Helper `gh auth git-credential`). Abgleich
  `.secrets.local`↔Actions: nur `GH_TOKEN`→`OMEGAFLOW_TOKEN`; alle übrigen
  API-Keys stehen bereits in Actions.
- Ausgeführt: T1 rotiert (neuer classic PAT, gemessen `login=omegaflow`,
  `push:true` auf `omegaflow/omegaflow`+`sources`); Actions `OMEGAFLOW_TOKEN`
  darauf gesetzt (2026-09-15T19:38Z, Verifikationslauf `health-check`
  `35014790907`); `.git-credentials` gesichert+geleert. (Schritt: Operator —
  nach grünem Lauf löschen: omegaflow `gh-cli` + altes `omegaflow`;
  johannestyroller `omegaflow-cdn`, `gho_…`, abgelaufene
  `world_magnetic_model*`/`nebra*`/`Laptop Rescue`.)

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf den neuen Sha
  `fc0e0496` (Header-sha256 `f674e8b2…`); der eine Ask = two-/three-way-Split.
  Reg 4 (Amplitude) bleibt `pending` mit gemessenem ~5-Hz-Anker (Station 14,
  1988) — kein unverankerter Wert im Text. (Schritt: Adressen bestätigen + senden
  — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

Gemessen 2026-09-15 im `state/mail/mail_ledger.φ` (58 Zeilen): keine Antwort auf
einen der offenen Posten.

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
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
