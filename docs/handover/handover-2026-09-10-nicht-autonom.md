<!--
  title: Handover — nicht-autonom: was die Kybernautin nicht selbstständig ausführt (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: ec29d64c7bf33691167859f261b4376d6a8e63b7d5a531626464fc20799fb3ed
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — nicht-autonom (2026-09-10)

Was die Kybernautin nicht selbstständig ausführt — jede Zeile trägt ihren
Entsperrer. Die autonomen Pflichten stehen im Gegen-Handover.

## Operator-Wort

- **vo-tap / uvor** — Crate pushen (`ivoa/uvor` HTTP 200, Seed steht); hängt an
  der Markus-Übergabe.
- **SPICE-`.bc`-Kernels** — `gll-ck-cdn.yml` steht, naif-Release leer (404);
  Dispatch.
- **Desktop-Fork (GTX 970)** — 30-Jahres-Lauf.
- **sicherung-risiko-heime** — einzige-Kopie-Risiko-Heime sichern; der
  Backup-Akt selbst ist Operator-Sache.
- **adoption** — Repo public + Drei-Mail-Block (Toth/Turyshev/Markwardt).
- **lisa-pathfinder** — Δg-Zeitreihe anfragen; Antwort = Messung.
- **Hi-net/NIED** — NIED verlangt Registrierung.
- **TNO-Kette (Weberin)** — keine MPC-unabhängige Linie der 8.082-Menge
  (not-published).

## CI / Merge

- **papier-kleinpass** — nach dem Merge, Zahlen je Blatt.
- **de441 Re-Verifikation** — die 5-Schritt-Sequenz nach grünem bodies-Job
  (Run 34393748385).
- **matrixmachine-Urkunde** — die Urkunden-Zeile gegen den letzten grünen
  CI-Lauf (`ci-check.yml`); kein lokaler Lauf.

## Kalender

- **Nadel Ⅰ** — Jeans-Residuum bis Gaia DR4 (2.12.2026); Deduktion 42
  (VLBI+Doppler-Sonde).
- **Nadel Ⅱ** — Flyby-Prüftermine JUICE 28./29.9.2026 + Europa Clipper
  3.12.2026; AGU-2013-Abstract (Anderson) menschlich zu prüfen.
- **gaia-dr4-iapetus** — Gaia DR4 (2.12.2026) als 4D-Feld.
- **flyby2-addendum** — Metrik vor dem 28.09.
- **LISA Pathfinder** — die Archiv-Selbstregistrierung ist bis **15.09.2026**
  gesperrt („Self-registration temporarily unavailable"); danach registrieren →
  AIO-HTTP-Zugang (`aio/metadata-action`, `aio/data-action`) → Δg-Zeitreihe
  programmatisch. Die AIO existiert — der „interactive-only"-Auftrag lag falsch,
  der echte Blocker war Auth + Termin.

## Laufende Ernten (Check-back, kein Todo)

- **AllWISE** — läuft (~13 Tage); `allwise_coverage.fp01` dann verifizieren.
- **PS1-fraktional** — läuft; Partial-Landung reißt am 180-min-Timeout
  (Chunking/Timeout — eigenes Atom).
- **ned-Crawl** — 1/40 Slices; `ned.json` bei 40/40; IPAC-Antwort ausstehend.
- **ned-objdir** — IPAC-Anfrage versandt, Antwort ausstehend.

## Architektur-Frage

- **Korpora-Heim** — die ~120-MB-Katalog-Korpora in ein privates Repo
  (Discovery-Front CI-fähig)? Unentschieden: Drittanbieter-Redistribution +
  Quota gegen CI-Fähigkeit. Braucht eine Entscheidung (Rat oder Operator).
