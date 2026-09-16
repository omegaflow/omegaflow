<!--
  title: Handover — Forschung-Folge 53 (Stand 2026-09-17)
  session: Forschung-Folge 53
  class: handover
  date: 2026-09-17
  sha256: 826596209a29f48b9e5d2d16205bc86fab40b3e5d508963e5b412769b050c68b
  status: live
-->
# Handover — Forschung-Folge 53 (2026-09-17)

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

## Erde als Sender A — Kalibrierung (nächstes Atom, beschlossen)

- Die Kreuz-Medium-Kalibrierung (Tonga 2022) als Kalibrier-Atom beschlossen
  (`docs/concepts/die-akteure-im-boden-und-wasser.md:59,92–99`): zwei Uhren
  (Luft-Lamb, Wasser-Tsunami) an einem Ohr, die Kopplungszahl (hPa↔Wasser) wird
  gewogen, nicht zitiert. Reihenfolge: (1) **das eine Ohr finden** — eine Station
  mit Barometer **und** Wasserpegel (research-max; erste Messung, gated alles);
  (2) BGR-2022-Ernte `bgr_infrasound_compiler --year 2022` → CDN + `sources.φ`;
  (3) Zenodo-Druck-Compiler/Registratur (`data.zip` → parsebare Serie, `format`
  statt `reference`, `sources.φ:6256–6258`); (4) CI-Workflow für
  `tonga_lamb_crosscheck_probe`; (5) die Wiege (Uhr- + Kopplungskoeffizient);
  (6) Blatt + Siegel (Rat). (Schritt: research-max — die Ohr-Suche.)

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
  sind formatiert (`cargo fmt -p omegaflow-measure -- --check` clean). Die Zeile
  bleibt stehen, weil `post.md` fremde uncommittete Arbeit trägt (ernte/bau-
  Nachrichten + Header-sha) — nicht überschrieben. (Schritt: nach dem fremden
  `post.md`-Commit die forschung-Zeile streichen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
