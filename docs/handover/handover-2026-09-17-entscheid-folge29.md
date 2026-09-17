<!--
  title: Handover — Entscheid-Folge 29 (Stand 2026-09-17)
  session: Entscheid-Folge 29
  class: handover
  date: 2026-09-17
  sha256: 3b60fa0c8edbeb911f9e51af72203e2af479a2d9873eb8245967ff069ac5b44d
  status: live
-->
# Handover — Entscheid-Folge 29 (2026-09-17)

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

## TTL/φ-CDN-CI-Instanz — Entscheid ausstehend (härtester undatierter Punkt)

- **5-min-Takt + I02** — an die Bau-Linie gereist (`post.md`, `An bau`): kein
  5-min-Cron in `.github/workflows/` (`health-check.yml:10` 3 h,
  `kernel-flatten.yml:10` monatlich), im Client hart unterstellt
  (`src/archivar/fetch.rs:900` `CI_REFRESH_S=300`), in 3 Live-Doku-Stellen
  behauptet (`pfeiler-der-architektur.md:155–158,178–181`,
  `archivar-mathematikerin.md:22`, `sources-v2-spec.md:57`); die CI-Instanz müsste
  in `omegaflow/sources` leben. (Schritt: Bau-Messung abwarten → entscheiden:
  Cron bauen oder die 3 Doku-Stellen + `CI_REFRESH_S` auf die gemessene Wahrheit
  korrigieren.)

## Consent-Akte (per-Akt, Operatorwort)

- **DEMETER** (aus `post.md` gefaltet) — neue Orders sind ein Dritt-Akt; Harvest
  `35146819646` = Budget-Stopp, 0 `.DAT` (WAF blocked / order parse void).
  (Schritt: Consent-Wort für neue Orders oder die Route als `blocked` führen.)
- **adoption-Block** — `state/mail/adoption-mails.md`: drei Entwürfe (Toth,
  Turyshev, Markwardt, je Quelle in der To-Zeile). **Gated (Operator-Wort
  2026-09-17):** Senden erst, wenn die Forschung (20-s-Bande) komplett durch ist.
  (Schritt: je cleanen Body extrahieren; Sende-Wort erst nach Forschungsabschluss.)

## Operator-gebundene Punkte

- **CI-Schreib-Token-Budget** (aus `post.md` gefaltet) — planetary-odf
  `35190614514` (headSha `1e4faf79`) scheiterte in 7/9 Legs an `HTTP 403: API
  rate limit exceeded` beim `ensure release` (`OMEGAFLOW_TOKEN`), `rosetta_odf`
  cancelled; `docs/specs/ref-auth-apis.md:68–79` nennt seit 2026-09-17 einen
  eigenen CI-PAT `omegaflow-ci-write` mit getrenntem Bucket — die Läufe zeigen den
  403 dennoch. (Schritt: messen, ob die Secret-Rotation griff und ob
  GitHub-PAT-Buckets pro Token getrennt sind — operator-gebunden.)
- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

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
