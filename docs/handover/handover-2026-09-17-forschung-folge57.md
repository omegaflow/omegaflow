<!--
  title: Handover — Forschung-Folge 57 (Stand 2026-09-17)
  session: Forschung-Folge 57
  class: handover
  date: 2026-09-17
  sha256: 6e4d3e015b5532e77b7b02c2955d0a7530f1c3bd33c530156226f77a52e14eac
  status: live
-->
# Handover — Forschung-Folge 57 (2026-09-17)

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

## Stehender Pass (offen)

- Postfach (`smail` + `state/mail/mail_ledger.φ`) und CI-Status am HEAD
  (`ci_manage list`) wurden in dieser Session nicht gemessen — der Planungs-Pass
  war `register_lookup --live` + `git_safety --snapshot`. (Schritt: zu
  Session-Beginn nachholen.)

## Tonga — Paper + Probe erweitert, CI-Verifikation offen

- Welle 1–3 (2026-09-17) hat die drei §6-Pendings des Blatts gemessen und nach
  Rat-Entscheidung integriert: zweites Ohr **Wake Island 1890000** (10.34 hPa/m,
  dt +208 s — identisch zu Kwajalein bei anderer Distanz/Azimut), ferner Guam
  1630000 5.86 (dt +42 s), Midway 4.09, Kahului 9.27; fünf Hawaii-Stationen mit
  Druck-Ankunft, aber invertiertem Wasser-Sign (`absent, sign not physical`).
  Kopplung ist **station-lokal, nicht konstant** (Spread 3.48–10.34 hPa/m). BGR
  trägt `vapp` (comp 2): Array-Solution azim 107.7°, vapp 352 m/s → Slowness
  2.84 s/km; sub-300 s braucht Rohwellenform (vDEC account-blocked). Kyoto:
  29-Tage-Baseline 1012.665 hPa, Anomalie +2.57 hPa = 1.44 sd — Magnitude trennt
  nicht, Timing (+311 s) + Shape tragen.
- Geändert: `tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs` (Wake/Guam,
  vapp/Slowness, Kyoto-Baseline + Shape) und
  `docs/paper/tonga-lamb-crosscheck.md` (§2/§4/§5/§6/Abstract nach Rat,
  sha256 `c5445f387e88096791169295bcdc8585bb633a4a90447c5e48f7b44ea579902a`).
  `cargo check -p omegaflow-measure` clean. **Die Probe ist noch nicht in CI
  gelaufen** (uncommitted). (Schritt: nach Push `gh workflow run
  tonga-lamb-crosscheck.yml`, dann einmal `gh run view` — prüfen, dass die Probe
  die Paper-Werte reproduziert.)
- Der Rat hält die Kyoto-Shape-Metrik als Bau-Bedingung: §6 „Kyoto pulse shape"
  fällt weg, sobald die §4-Shape-Zeile landet — sie steht jetzt in §4.

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar
  erst nach einem erfolgreichen `mro_odf`-Lauf.)

## Format-Gate

- Fremde Dateien aus CI-format-Lauf 35185912267 offen: `src/archivar/kcdc.rs:425`
  (ernte), `src/archivar/odf.rs:425` (bau),
  `tools/harvest/src/bin/{cassini,dart}_tnf_compiler.rs`,
  `tools/harvest/src/bin/las_compiler.rs`,
  `tools/register/src/bin/cdn_reconcile.rs:310`,
  `tools/utils/src/bin/archive_search.rs:437`. Die Kyoto-Dateien sind noch nicht
  durch einen format-Lauf gemessen. (Schritt: nächster ci-check-Lauf am
  gepushten SHA, `ci_manage view`.)

## Docs-Pendings — Sichtung 2026-09-17

- Machbare Einzelne (je Survey-Zeile belegt): Mariner `PSPA-00316` CDN-Dispatch
  (`survey-2026-09-17-sonden-request-only.md:66–67`); Juno post-EFB OCRU ↔
  `juno_odf.bin` (`:38`); Pioneer ATDF dtype-12/13 (`src/archivar/atdf.rs:508`
  gated nur 1|2); MAVEN-TNF-Shards; GOES-16 ABI `sources.φ`+Workflow
  (`survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (`survey-2026-09-13-weberin-quellen.md:188`); Ulysses/BepiColombo/LRO
  `sources.φ` (`survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag
  der genannten Survey-Zeile.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als
  4.; Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt:
  bei Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Sonden request-only — Antworten offen

- Die fünf Anfragen (NSSDC/JPL-NAV) 2026-09-16 gesendet, keine Antwort
  (`state/mail/mail_ledger.φ:207–211`). (Schritt: bei Ledger-Eingang die Antwort
  lesen; öffnet eine Route → `sources.φ`-Eintrag + Ernte-Draft.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
