<!--
  title: Handover — Entscheid-Folge XIII (Stand 2026-09-16)
  session: Entscheid-Folge XIII
  class: handover
  date: 2026-09-16
  sha256: f7ea878b14fd620ae5f48442b28ac3396bebf53d7bcc7b98ffeaa36e2b366492
  status: live
-->
# Handover — Entscheid-Folge XIII (2026-09-16)

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

- PII-Runde 2 verifiziert: der Rewrite `69813f1 → b2e5ac7` entfernt die
  Konto-Dateien und redigiert Adresse/E-Mail; der Tag `v2026-09-09` ist umgehängt
  `317418d → bc113f6`. An `main`/Tag keine PII (`curl -I` → 404, `git log -S`
  leer); nur der alte Tag-Commit `317418d` ist bis zum GitHub-GC per SHA abrufbar.
  Purge-Antrag = Ticket **#4761801** (bestätigt `support@githubsupport.com`,
  `state/mail/mail_ledger.φ:64`); #4761482 überholt. (Schritt: GitHubs
  GC-Bestätigung abwarten.)

## adoption — Drei-Mail-Block: Entwürfe sendfertig (lokal), Send beim Operator

- Die drei Entwürfe (Toth/Turyshev/Markwardt, 20-s-Bande) liegen lokal in
  `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha `50db1ed`; der eine
  Ask = two-/three-way-Split. Reg 4 (Amplitude) bleibt `pending` mit gemessenem
  ~5-Hz-Anker (Station 14, 1988). Operator-Entscheid 2026-09-16: noch nicht senden.
  (Schritt: Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- GitHub Support — User→Org / HTTP 422: Ticket #4761801 bestätigt empfangen;
  Antwort offen. (Schritt: Postfach auf die Support-Antwort prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) raus 2026-09-15;
  Shaughnessy bestätigte 2026-09-15 21:11 den Eingang und nahm die Petition in
  die Einzelprüfung („outside our standard criteria are reviewed individually"),
  Entscheidung offen (`state/mail/mail_ledger.φ:67–68`;
  `state/mail/auftrag-rubin-data-rights-antrag.md` fortgeschrieben). (Schritt:
  Postfach auf die Review-Antwort prüfen.)
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- CSES-Limadou — Anfrage an Sotgiu (ASI SSDC) raus, Antwort offen; der L2-Zugang
  liegt lokal in `.secrets.local` (`SSDC_USER`/`SSDC_PASS`).

## Entscheid — Ernte-Verdikt absorbiert (2026-09-16)

- Das gemessene Zugangsanfragen-Verdikt (`handover-2026-09-15-ernte-folge35.md`
  §Zugangsanfragen) angenommen: die entbehrlichen Wartepunkte NOIRLab, JSOC, LPF,
  GAVO, BiSON, IGETS, TOAR sind gestrichen (anonyme Routen 200); es halten nur
  GitHub, Rubin, NSE/Haug, CSES-Limadou.

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
