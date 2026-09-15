<!--
  title: Handover — Entscheid-Folge XII (Stand 2026-09-15)
  session: Entscheid-Folge XII
  class: handover
  date: 2026-09-15
  sha256: d9caa9168c4a613f5b979c47cd251ec49121e354f39c4fd0f83c47c4ac757d4c
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

- PII-Runde 2 ausgeführt und unabhängig verifiziert: der zweite Rewrite
  `69813f1 → b2e5ac7` entfernt die Konto-Dateien
  (`docs/reference/{antares,fink}-konto-2026-09-05.md`,
  `docs/handover/archiv/fink-konto-2026-09-05.md`) und redigiert Adresse/E-Mail
  in `cloudflare/wrangler.toml`, `handover-2026-09-13-entscheid-folge4`,
  `handover-2026-09-15-entscheid-folge11` (sha256-Seals neu); der Tag
  `v2026-09-09` ist umgehängt `317418d → bc113f6` (Release + Binär-Assets
  bleiben). Messung: keine PII mehr an `main`/Tag (`curl -I` → 404,
  `git log -S` leer). Nur der alte Tag-Commit `317418d` ist bis zum GitHub-GC
  per SHA abrufbar. (Schritt: GitHubs GC-Bestätigung abwarten; der aktualisierte
  Purge-Antrag liegt in `state/mail/github-purge-request.md` — senden an
  https://support.github.com/contact.)

## GitHub-Token — auf zwei Tokens konsolidiert (gemessen)

- Zwei Jobs → zwei Tokens, beide unter `omegaflow`: **T1 Schreiben**
  (`omegaflow-write`, classic `repo`+`workflow`; lokal `GH_TOKEN` + Actions
  `OMEGAFLOW_TOKEN`) und **T2 Lesen** (`omegaflow-read`, fine-grained read-only;
  nur `archive_search`). `gh auth` liegt im Keyring, git pusht ohne `GH_TOKEN`.
- Erledigt: alte omegaflow-Tokens gelöscht, CI-Secret rotiert (19:38Z),
  `.git-credentials` geleert. Kein Konto-übergreifender Token mehr. (Schritt:
  keiner offen — git trägt es.)

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf den neuen Sha
  `50db1ed` (Header-sha256 `889ea9bc…`); der eine Ask = two-/three-way-Split.
  Reg 4 (Amplitude) bleibt `pending` mit gemessenem ~5-Hz-Anker (Station 14,
  1988). (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- GitHub Support — User→Org / HTTP 422: Ticket ist raus, Antwort offen.
  (Schritt: Postfach auf die Support-Antwort prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Entscheidung offen (`state/mail/auftrag-rubin-data-rights-antrag.md`).
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`).

## Nachricht an die Bau-Linie

Gemessen am ci-check auf `b2e5ac7` und an den Einzel-Läufen:

- `ci-check` **format** rot: flächiger `cargo fmt`-Drift in `src/archivar/*`
  (`cargo fmt --all --check` zeigt Dutzende Dateien). (Schritt: `cargo fmt --all`,
  aber fremde uncommittete `src/archivar`-Änderungen liegen im Baum.)
- `ci-check` **clippy** rot. (Schritt: `cargo clippy --all-targets`.)
- `ci-check` **test** rot (3): `archivar::omni2::tests::rejects_unknown_component`
  (`src/archivar/omni2.rs:83`), `archivar::tests::test_matrix_vs_wgccre_agreement`
  (`src/archivar/tests.rs:2805`), `mathematikerin::te::tests::pcmci_recovers_known_dag`
  (`src/mathematikerin/te.rs:3723`). (Schritt: je Test messen.)
- `esp32-firmware` rot: `error[E0433]: cannot find module or crate xtensa_lx`
  in `esp-sync`. (Schritt: esp-hal-Version/Feature prüfen.)
- `te-gate` (#13) rot: n=1000 FPR-Gate. (Schritt: `src/mathematikerin/te.rs`.)
- `health-check` reverify (#17) rot: PurpleAir API **402 Payment Required** —
  kein Code-Fehler, Billing/Account (Operator).
- `number_audit`-Test rot (**vor-existent**): `docs/specs/bekannt-schlecht-korpus.md`
  stimmt nicht mehr. (Schritt: Korpus oder Erwartung abgleichen.)
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch absent.
  (Schritt: Tag + Welt-Fassung-Branch — Bau.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
