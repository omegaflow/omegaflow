<!--
  title: Handover — Forschung-Folge 54 (Stand 2026-09-17)
  session: Forschung-Folge 54
  class: handover
  date: 2026-09-17
  sha256: 76f44c746f036d8de3b9fc8fc5146dc468f13c3191d3de85b4d5866e45d80a6d
  status: live
-->
# Handover — Forschung-Folge 54 (2026-09-17)

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

- Der Job des Runs `35159180825` (`planetary-odf-cdn`, headSha `d340a8c6`) ist
  **pending** (keine Jobs, gemessen 2026-09-17) — der Split ist blockiert; der
  Ganzdatei-Block `phi/sources.φ:6754–6758` steht unverändert (gegen den Baum
  gehalten). (Schritt: nach Run-Abschluss `gh run view 35159180825 --log` — den
  gedruckten φ-Block nehmen und den `mro_odf`-Ganzdatei-Block durch je einen
  5-Zeilen-Block je Shard ersetzen; `refuse_shard_overlaps` verweigert
  Überlappung gleichen `format`.)

## Erde als Sender A — Kalibrierung (Ohr steht, Rest offen)

- Das eine Ohr ist gemessen und gebaut (2026-09-17): NOAA CO-OPS Kwajalein
  `1820000` (8.731667 N, 167.73611 E; `air_pressure` + `water_level`, 6-min,
  keyless; Lamb +2.4 hPa 07:42 UTC, Pegeltrog −0.578 m 09:58 UTC im selben
  Raster). `tonga_lamb_crosscheck_probe.rs` trägt die Ohr-Sektion (Druck-Peak,
  Wasser-Trog, Kopplungszahl hPa/m), `.github/workflows/tonga-lamb-crosscheck.yml`
  fährt sie in CI. (Schritt: nach Push `gh workflow run tonga-lamb-crosscheck.yml`,
  dann `gh run view <id>` — die gemessene Kopplungszahl ist das Blatt-Material.)
- **BGR-2022-Ernte offen:** der Probe liest `bgr_infrasound_IS52_2022.bin`
  lokal; auf dem CDN liegt nur `bgr_infrasound.bin` (2024). (Schritt: eigener
  Workflow `bgr_infrasound_compiler --out-bin bgr_infrasound_IS52_2022.bin
  --station IS52 --year 2022 --lsk kernels/naif0012.tls --ci-mode` + `sources.φ`-
  Eintrag; die Feld-Definitionen gegen `bgr_infrasound` (Zeile 6599–6606)
  abgleichen, Feld-Namens-Dopplung zuerst messen.)
- **Zenodo-Druck-Compiler/Registratur offen:** `data.zip` → parsebare Serie,
  `format` statt `reference`, `sources.φ:6256–6258`. (Schritt: Compiler in
  `tools/harvest/src/bin/`; das Kyoto-`220115.txt`-Layout aus
  `tonga_lamb_crosscheck_probe.rs` (`parse_pressure`) übernehmen.)
- **Blatt + Siegel offen** (Rat), erst nach dem CI-Lauf der Kopplungszahl.
  (Schritt: `gh run view` des tonga-Laufs lesen, dann Rat.)

## Sonden request-only — Antworten offen (undatiert)

- Die fünf Anfragen (NSSDC/JPL-NAV) 2026-09-16 gesendet, **keine Antwort**
  (`state/mail/mail_ledger.φ:207–211`). (Schritt: bei Ledger-Eingang die Antwort
  lesen; öffnet eine Route → `sources.φ`-Eintrag + Ernte-Draft. Die offenen
  Teilrouten — Mariner `PSPA-00316`, Juno-OCRU, Pioneer-ATDF dtype-12 — sind als
  Post an ernte/bau.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. `docs/paper/flyby-path-2-preregistration.md` nennt nur das *Was* — kein
  ausführbares Schritt-Detail. (Schritt: vor dem 28.09. den konkreten
  Abruf-Schritt je Kanal in `docs/paper/flyby-path-2-preregistration.md` setzen.)

## post.md — fremde uncommittete Arbeit (Format-Gate)

- Der Forschungsteil von `post.md:18` (format-Gate) ist erledigt: die drei
  `tools/measure/src/bin/{band_amplitude_probe,corona_event_probe,s2_weberin_probe}.rs`
  sind formatiert. Die Zeile bleibt stehen, weil `post.md` fremde uncommittete
  Arbeit trägt (ernte/bau-Nachrichten + Header-sha) — nicht überschrieben.
  (Schritt: nach dem fremden `post.md`-Commit die forschung-Zeile streichen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
