<!--
  title: Handover — Entscheid-Folge XV (Stand 2026-09-16)
  session: Entscheid-Folge XV
  class: handover
  date: 2026-09-16
  sha256: 020a655f9f9afa83aba104ac7bd8b6bf21c2c6244ff28ffc3a0c92a55c3e910e
  status: live
-->
# Handover — Entscheid-Folge XV (2026-09-16)

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

- Erneut gemessen 2026-09-16 mit `pii_exposure`
  (`cargo run -p omegaflow-register --bin pii_exposure`): unverändert 45
  (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien über 15 Pre-Rewrite-Refs;
  der GitHub-GC hat noch nicht gegriffen. Spitzen: die zwei Konto-Dateien aus je neun
  Refs, die `docs/auftrag/*`-Entwürfe aus je ein bis fünf. Tag `v2026-09-09` → `bc113f6`.
- Support-Ticket #4761801 (`support@githubsupport.com`) bestätigt empfangen
  2026-09-15 22:12; die GC-Bestätigung steht aus. (Schritt: Postfach prüfen; bleibt
  sie aus, GitHub auf #4761801 nachfassen — Operator.)
- Data-Subject-Löschung: gesendet 2026-09-16 an `privacy@github.com` über Resend
  von `code@omegaflow.space` (Id `16c1b8c6-d24a-458a-a49d-7c1362b6aa8e`), Body =
  alle zehn PII-tragenden Dateien, Entwurf `state/mail/privacy-deletion-request.md`
  (lokal, gitignored). Die hinterlegte Konto-Adresse ist
  `johannes.tyroller@proton.me`; gemessen in der GitHub-Doku
  (`docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement`):
  keine Absenderpflicht, ein „authorized agent" ist zugelassen, eine
  Identitätsprüfung bleibt vorbehalten — der Versand ist damit nicht belegt
  unwirksam. Der Proton-Versand bleibt als Operator-Schritt bereit, falls GitHub
  Verifikation verlangt. Der Resend-Schlüssel ist auf Senden beschränkt
  (`restricted_api_key`), der Zustellstatus ist nicht abfragbar — das Postfach ist
  der Kanal. (Schritt: Postfach auf die Privacy-Antwort prüfen; bei
  Verifikationsbitte aus Proton senden — Entwurf liegt bereit.)

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha `50db1ed`; der eine
  Ask = two-/three-way-Split. Reg 4 (Amplitude) bleibt `pending` mit gemessenem
  ~5-Hz-Anker (Station 14, 1988). Operator-Entscheid 2026-09-16: noch nicht senden.
  (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) raus 2026-09-15; Eingang
  bestätigt, Einzelprüfung zugesagt, Entscheidung offen
  (`state/mail/mail_ledger.φ`; `state/mail/auftrag-rubin-data-rights-antrag.md`
  fortgeschrieben). (Schritt: Postfach auf die Review-Antwort prüfen.)
- NSE/Haug — Anfrage an Keimer raus, Antwort offen. (Schritt: Postfach prüfen.)
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`). (Schritt: Postfach prüfen.)
- Postfach erneut gemessen 2026-09-16 (`smail_recv` lebt, letzter Ledger-Eingang
  00:40): keine neue Antwort. (Schritt: nächste Session erneut messen.)

## Benchmark — PII-Expositions-Verifikation (flash gegen pro/max)

- Identischer Wortlaut, read-only: `general` (flash) $0.0138 gegen `research-max`
  (pro/max) $0.0729 — pro/max 5,3× teurer bei identischem Ergebnis. Sieger: flash.
- `pii_exposure`-Bau (`grind-flash`) $0.0054 — Routine, flash-first; Gremium
  (Privacy-Scope) $0.0155.

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
