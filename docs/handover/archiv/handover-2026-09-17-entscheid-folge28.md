<!--
  title: Handover — Entscheid-Folge 28 (Stand 2026-09-17)
  session: Entscheid-Folge 28
  class: handover
  date: 2026-09-17
  sha256: 411aa4913c75e182aa4c3e0b60aedcb26a74e3dc3cded7402e80978a18edc5b5
  status: live
-->
# Handover — Entscheid-Folge 28 (2026-09-17)

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

## sources-Repo — die eine offene Messung (härtester undatierter Punkt)

- **5-min-Takt + I02-Python** — `survey-2026-09-17-verlorene-diskussionen.md`
  (b1/c5): das CI-TTL/φ-CDN (5-min-Takt) lebt in 3 Live-Doku-Stellen und hart im
  Client (`fetch.rs:900` `CI_REFRESH_S=300`, `:910` `ttl.max(...)`), aber kein
  5-min-Cron in `.github/workflows/` (`health-check.yml:10` 3-h,
  `kernel-flatten.yml:10` monatlich); die I02-Python-Behauptung ist von hier aus
  nicht messbar. (Schritt: `omegaflow/sources` klonen bzw.
  `archive_search --verdict`/`gh api` auf refresh.yml/I02; danach entscheiden —
  bauen oder die 3 Doku-Stellen + `CI_REFRESH_S` auf die gemessene Wahrheit
  korrigieren.)

## Doku-Drift — Befunde des Surveys umsetzen

- **Biotic-Präsens-Story** — `docs/concepts/kybernaut-native-methodology.md:23`
  erzählt die biotic-Kraft im Präsens, der Baum führt sie nicht
  (`force.rs:243`, `tests.rs:6302`; 9. Kraft = electric). (Schritt: Zeile als
  Legacy-Ära kennzeichnen oder korrigieren.)
- **remove-bias-Plan** — referenziert `warm_cache` (heute `cache_fresh_at`/
  `cache_path_for`), Header ohne date/status (`remove-bias.md:1–5,951,960`).
  (Schritt: Plan archivieren oder offene Punkte extrahieren.)
- **kernel-plan v6-Zitat** — `kernel-curation-ci-automation-plan.md:8,48` nennt
  „v6 protocol"; heute v9, die Automation ist gebaut. (Schritt: als historisch
  markieren oder archivieren.)
- **gpu-feature-gate** — Legacy `8b1c38b5` „als eigenes Atom registriert"; heute
  0 Treffer, ersetzt durch „GPU is the membrane". (Schritt: als descoped mit
  diesem Beleg schließen.)

## Consent-Akte (per-Akt, Operatorwort)

- **adoption-Block** — `state/mail/adoption-mails.md`: drei Entwürfe (Toth,
  Turyshev, Markwardt, je Quelle in der To-Zeile). **Gated (Operator-Wort
  2026-09-17):** Senden erst, wenn die Forschung (20-s-Bande) komplett durch ist.
  (Schritt: je cleanen Body extrahieren; Sende-Wort erst nach Forschungsabschluss.)

## smail — Sent-Log

- **Install ausstehend** — Code committet `65f9243b` (jeder `--send` schreibt
  nach `state/mail/sent_ledger.φ`); CI-Lauf `35195888583` (`service-build`)
  gemessen `queued` seit `2026-09-17T07:42:36Z` (kein Runner hat den Job
  aufgenommen). (Schritt: `gh run view 35195888583` → sobald `success`
  `gh run download` → `smail` nach `~/.local/bin/` + `target/release/`.)

## Warten auf Rückmeldung (extern)

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **NSE/Haug** (MLZ/FRM-II Lohstroh; MPI-FKF Keimer) — Antwort offen.
- **Rubin-Review** (Shaughnessy, high volume) — offen.
- **GitHub GC #4761801** — Follow-up gesendet 2026-09-17 (Resend
  `01a0aded-75a3-7548-a994-682b9ef54843`), Antwort offen.
- **GitHub Privacy-Löschung** — gesendet 2026-09-17 (Resend
  `01a0aded-819e-75db-aaae-2ff654b6ec15`), Antwort offen.
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Operator-gebundene Punkte

- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Termine (Wiedervorlage)

- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
