<!--
  title: Handover — Forschung-Folge 52 (Stand 2026-09-17)
  session: Forschung-Folge 52
  class: handover
  date: 2026-09-17
  sha256: d4df0b6b02a9313a1b5d1ec353a5759a74f3d2ab58e436c210240e02b7a8522c
  status: live
-->
# Handover — Forschung-Folge 52 (2026-09-17)

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
der härteste undatiert); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## ODF-Bande-Split — Konsument (härtester undatiert, CI-blockiert)

- Offen ist nur noch `mro_odf`: der Job des Runs `35148936648` wurde durch einen
  Runner-Shutdown abgebrochen (`The runner has received a shutdown signal`,
  Infrastruktur, kein Codefehler), neu dispatcht Run `35159180825`. (Schritt:
  nach Run-Abschluss `gh run view 35159180825 --log` — den gedruckten φ-Block
  nehmen und den `mro_odf`-Ganzdatei-Block `phi/sources.φ:6754–6758` durch je
  einen 5-Zeilen-Block je Shard ersetzen; `refuse_shard_overlaps` verweigert
  Überlappung gleichen `format`.)

## Sonden request-only — Antworten offen (undatiert)

- Die vier Routen (Voyager 1/2, Mariner 10, Viking 1/2, Juno pre-EFB) sind neu
  gemessen und alle request-only bestätigt
  (`docs/surveys/survey-2026-09-17-sonden-request-only.md`; Register
  `phi/blocked_sources.φ`). Die fünf Anfragen (NSSDC/JPL-NAV) 2026-09-16 gesendet,
  **keine Antwort** (`state/mail/mail_ledger.φ:207–211`). (Schritt: bei
  Ledger-Eingang die Antwort lesen; öffnet eine Route → `sources.φ`-Eintrag +
  Ernte-Draft. Die offenen Teilrouten — Mariner `PSPA-00316`, Juno-OCRU,
  Pioneer-ATDF dtype-12 — sind als Post an ernte/bau.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. `docs/paper/flyby-path-2-preregistration.md` nennt nur das *Was*
  (Felder plasma-pressure gradient, IMF-Bz, Kp, Swarm, RTSW@L1 mit
  L1-Transitzeit) — kein ausführbares Schritt-Detail. (Schritt: vor dem 28.09. den
  konkreten Abruf-Schritt je Kanal in `docs/paper/flyby-path-2-preregistration.md`
  setzen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
