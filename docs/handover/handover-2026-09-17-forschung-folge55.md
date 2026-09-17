<!--
  title: Handover — Forschung-Folge 55 (Stand 2026-09-17)
  session: Forschung-Folge 55
  class: handover
  date: 2026-09-17
  sha256: 22201fe0fac34e6d4ddf2bbd3d288e4191d23441275bf62dfc229ddb19617efd
  status: live
-->
# Handover — Forschung-Folge 55 (2026-09-17)

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

## ODF-Bande-Split `mro_odf` (härtester undatiert, CI-blockiert)

- Run `35159180825` trägt nach 48 min **keine Jobs** (gemessen 2026-09-17, HEAD
  `bc9d6a0b`); der Ganzdatei-Block `phi/sources.φ:6754–6758` steht unverändert.
  (Schritt: nach Job-Anlauf `gh run view 35159180825 --log` — den gedruckten
  φ-Block nehmen und den `mro_odf`-Ganzdatei-Block durch je einen 5-Zeilen-Block
  je Shard ersetzen; `refuse_shard_overlaps` verweigert Überlappung gleichen
  `format`.)

## Erde als Sender A — korrigierte Probe, CI-Lauf offen

- Rat (2026-09-17): **kein Blatt, kein Siegel**. Die globale-Extrema-Methode maß
  zwei ungekoppelte, gezeitengetriebene Extrema (Druck-Peak +14.4 h, Wasser-Trog
  +36.6 h nach der Vorhersage) — `3.67 hPa/m` war kein Lamb-Signal; die
  Handover-54-Zeile (07:42/09:58) war nicht reproduzierbar.
- Die Probe `tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs` trägt jetzt
  `ARRIVAL_WINDOW_S=7200` / `COUPLING_WINDOW_S=21600` (Fenster um `predicted`),
  die globalen Extrema getrennt als benannte Schranke, und `zip_member_names`
  druckt bei fehlendem Kyoto-Member die Zenodo-Member-Namen. `cargo check`
  clean. (Schritt: nach Push `gh workflow run tonga-lamb-crosscheck.yml`, dann
  `gh run view <id>` — das Fenster-Extremum + die Member-Liste sind das
  Blatt-Material; erst dann Rat für Blatt + Siegel.)

## BGR-2022-Ernte

- `.github/workflows/bgr-infrasound-cdn.yml` kompiliert jetzt 2022
  (`--station IS52 --year 2022 --out-bin bgr_infrasound_IS52_2022.bin`,
  eigene Idempotence); `phi/sources.φ` registriert den Asset (`format
  bgr_infrasound`, Felder identisch — `refuse_shard_overlaps` greift nicht,
  keine Format-Dedup). (Schritt: nach Push `gh workflow run
  bgr-infrasound-cdn.yml`; der Lauf misst zugleich, ob der BGR-ZIP
  IS52-2022-Einträge trägt — Absenz bricht mit `matched no entry … pending` ab,
  kein erfundener Asset.)

## Kyoto-Zenodo-Layout

- Der CI-Lauf meldete `220115.txt absent from the Zenodo archive`; die Probe
  druckt jetzt die Member-Namen. (Schritt: die Member-Liste aus dem korrigierten
  CI-Lauf lesen; den echten Kyoto-Dateinamen + Layout messen, dann
  `sources.φ:6256–6258` von `reference` auf `format` heben und den Compiler in
  `tools/harvest/src/bin/` auf das gemessene Layout setzen.)

## Sonden request-only — Antworten offen (undatiert)

- Die fünf Anfragen (NSSDC/JPL-NAV) 2026-09-16 gesendet, **keine Antwort**
  (`state/mail/mail_ledger.φ:207–211`). (Schritt: bei Ledger-Eingang die Antwort
  lesen; öffnet eine Route → `sources.φ`-Eintrag + Ernte-Draft. Die offenen
  Teilrouten — Mariner `PSPA-00316`, Juno-OCRU, Pioneer-ATDF dtype-12 — sind als
  Post an ernte/bau.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## post.md — fremde uncommittete Arbeit (Format-Gate)

- Der forschung-Teil von `post.md:18` (format-Gate) ist erledigt; die Zeile
  bleibt stehen, weil `post.md` fremde uncommittete Arbeit trägt. (Schritt: nach
  dem fremden `post.md`-Commit die forschung-Zeile streichen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
