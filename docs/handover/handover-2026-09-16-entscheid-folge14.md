<!--
  title: Handover — Entscheid-Folge XIV (Stand 2026-09-16)
  session: Entscheid-Folge XIV
  class: handover
  date: 2026-09-16
  sha256: 53369ef27a3d2ae990234eb4c4813cba43d354a8bbaacaf7c15a0f1c88dfb8ef
  status: live
-->
# Handover — Entscheid-Folge XIV (2026-09-16)

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

- Gemessen 2026-09-16 mit der Probe `pii_exposure`
  (`cargo run -p omegaflow-register --bin pii_exposure`): alle 15 Pre-Rewrite-Commits
  sind per SHA auflösbar; **alle zehn** PII-tragenden Dateien liefert GitHub mit Inhalt
  aus Pre-Rewrite-Objekten — 45 (Datei, Ref)-Kombinationen. Spitzen: die zwei
  Konto-Dateien `docs/reference/{antares,fink}-konto-2026-09-05.md` aus neun Refs
  (`317418d, 0d76458f, 88dda694, ea696f62, 035a9191, afa96459, d9d6e800, b0bebc1c,
  d2ab19b1`), `docs/handover/archiv/fink-konto-2026-09-05.md` aus sechs, die
  `docs/auftrag/*`-Entwürfe aus je ein bis fünf Refs. Tag `v2026-09-09` → `bc113f6`.
- Zwei getrennte GitHub-Prozesse: (a) Support-Ticket #4761801 (bestätigt
  `support@githubsupport.com`; #4761482 überholt) für cached views / SHA-Abrufbarkeit —
  hat den GC noch nicht bewirkt; (b) **Data-Subject-Löschung** — das Formular
  `github.com/contact/privacy` verlangt eine angemeldete Session (leitet auf
  `help.github.com/request/landing`); die dokumentierte Route ist die E-Mail an
  `privacy@github.com` (GitHub Privacy Statement, Data-Subject-Rights). Entwurf steht
  in `state/mail/privacy-deletion-request.md` (lokal, gitignored), Scope = alle zehn
  PII-tragenden Dateien (Gremium einmütig). (Schritt: Entwurf von
  `<operator-adresse>` an `privacy@github.com` senden — Operator; dann erneut
  messen.)
- (Schritt: Postfach auf GC-/Privacy-Bestätigung prüfen; bleibt sie aus, GitHub auf
  #4761801 nachfassen — Operator.)

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha `50db1ed`; der eine
  Ask = two-/three-way-Split. Reg 4 (Amplitude) bleibt `pending` mit gemessenem
  ~5-Hz-Anker (Station 14, 1988). Operator-Entscheid 2026-09-16: noch nicht senden.
  (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) raus 2026-09-15; Eingang
  2026-09-15 21:11 bestätigt, Einzelprüfung zugesagt, Entscheidung offen
  (`state/mail/mail_ledger.φ`; `state/mail/auftrag-rubin-data-rights-antrag.md`
  fortgeschrieben). (Schritt: Postfach auf die Review-Antwort prüfen.)
- NSE/Haug — Anfrage an Keimer raus, Antwort offen. (Schritt: Postfach prüfen.)
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`). (Schritt: Postfach prüfen.)
- Postfach gemessen 2026-09-16: Pipeline lebt (`cloudflared` + `smail_recv`
  laufen), letzter Ledger-Eingang 00:40 (Rubin-Freigabe) — keine neue Antwort.
  (Schritt: nächste Session erneut messen.)

## Benchmark — PII-Expositions-Verifikation (flash gegen pro/max)

- Identischer Wortlaut, read-only: `general` (flash) $0.0138 gegen `research-max`
  (pro/max) $0.0729 — pro/max 5,3× teurer bei identischem Ergebnis (dieselbe
  15-SHA-Tabelle; max zusätzlich Blob-SHAs + authentifizierter Tag-Check).
  Sieger: flash — die Verifikation bleibt bei flash.
- `pii_exposure`-Bau (`grind-flash`) $0.0054 — Routine, flash-first, kein pro/max-Lauf;
  Gremium (Privacy-Scope) $0.0155.

## Nachricht an die Bau-Linie

Gemessen am ci-check auf `b2e5ac7` und an den Einzel-Läufen:

- `ci-check` **format** rot: flächiger `cargo fmt`-Drift in `src/archivar/*`.
  (Schritt: `cargo fmt --all` — fremde uncommittete `src/archivar`-Änderungen im Baum.)
- `ci-check` **clippy** rot. (Schritt: `cargo clippy --all-targets`.)
- `ci-check` **test** rot (3): `archivar::omni2::tests::rejects_unknown_component`
  (`src/archivar/omni2.rs:83`), `archivar::tests::test_matrix_vs_wgccre_agreement`
  (`src/archivar/tests.rs:2805`), `mathematikerin::te::tests::pcmci_recovers_known_dag`
  (`src/mathematikerin/te.rs:3723`).
- `esp32-firmware` rot: `error[E0433]: cannot find module or crate xtensa_lx`.
- `te-gate` (#13) rot: n=1000 FPR-Gate.
- `health-check` reverify (#17) rot: PurpleAir API **402** (Billing/Account — Operator).
- `number_audit`-Test rot (vor-existent): `docs/specs/bekannt-schlecht-korpus.md`.
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch absent.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
