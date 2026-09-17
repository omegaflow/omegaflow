<!--
  title: Handover — Ernte-Folge 60 (Stand 2026-09-17)
  session: Ernte-Folge 60
  class: handover
  date: 2026-09-17
  sha256: dd0ee6ef33dc97514afc5376057281604784a95185ade5a8671881a634ced713
  status: live
-->
# Handover — Ernte-Folge 60 (2026-09-17)

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

## KCDC — Join-Fix steht im Baum, Re-Dispatch + Registrierung offen (härtester undatiert)

- Der Registry-Stamm-Join-Fix (`tools/harvest/src/bin/kcdc_compiler.rs`: `component_maps` trägt die Stämme, Dateiname `{stem}.txt`, `general` über `own_gt_times`) + Regressionstest `compile_joins_component_rows_through_the_registry_stems` sind im Baum, `cargo check` clean (0 Fehler / 0 Warnungen, beide Crates, gemessen 2026-09-17). Lauf `35157749807` failure ist ein Fetch-`curl`-Timeout (exit 28) — `--compile` wurde nicht erreicht, der Fix ist damit noch ungetestet. (Schritt: nach `/commit`+Push `gh workflow run kcdc-cdn.yml`; bei success `sha256`-Zeile + Block `kcdc_kascade` in `phi/sources.φ`. Design-Grenze: der Request läuft 2026-09-30 ab — Re-Submit = Dritt-Akt über entscheid.)

## MAVEN TNF — 2.34 GB über dem GitHub-Asset-Cap

- MAVEN-Lauf `35156057281` failure: Compile ok (`32529787 TNF DT0 samples`, roundtrip parses), Upload HTTP 422 — `maven_tnf.bin` = 2342144672 B > 2147483648 B (2 GiB Release-Asset-Cap); kein Asset unter `pds-ppi.igpp.ucla.edu`. (Schritt: Chunking wie `voyager_odr_s0..s13` oder alternater Manifest-Pfad; dann `maven_tnf`-Block in `phi/sources.φ`.) Cassini (`cassini_tnf.bin`, 191163176 B, sha256 b8d0dc52…9ab6) und DART (`dart_tnf.bin`) sind registriert.

## DEMETER — Harvest 0 Dateien (Quellen-WAF)

- Harvest `35146819646` success = Budget-Stopp: Log `0 files on disk, 0 orders done`; jede Order `WAF blocked … waiting 1800s` / `order parse void`; demeter-cdn `35150299541` + aggregate `35150610212` success mit 0 Assets. (Schritt: neue Orders = Dritt-Akt → Consent über entscheid; ohne WAF-Durchlass keine `.DAT`.)

## rosetta_odf / mro_odf — Job-Caps

- `35148936648` failure: `rosetta_odf` cancelled am 4h-Job-Cap, `mro_odf` Runner-Shutdown; keine neuen Assets. (Schritt: `sha256`-Zeile zwischen `sources.φ:6472/6473` und `Offen:`-Klausel Z. 6476 bleiben; Re-Dispatch mit längerem/gesplittetem Budget, `mro_odf`-Fehler messen.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt. Erster Nachweis: argo-bgc `35154746956` zeigt `present=true` + übersprungenen Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein Massen-Dispatch.)

## Mariner 10 PSPA-00316 — Route lebt, Compiler + Layout offen

- Gemessen 2026-09-17: `spdf.gsfc.nasa.gov/pub/data/mariner/mariner10/celestial_mechanics_and_radio_science/red_tele_signal_data_venus_occlt/` HTTP 200, 9 `.tar` (~19,6 MB je) ustar; das kanonische `DD029604_F1.DAT` trägt bereits 2-Byte-Header-BINARY (`STREAM_TYPE BINARY`, `NSSD1346`), kein 7-Bit-Unpacker nötig. Fehlt: ein `mariner_occlt`-Compiler (Geschwister `voyager_saturn_compiler.rs`) + die NSSD1346-Record-Layout-Messung (max 4108 B ≠ voyager 8066 B). (Schritt: Grind-Draft + Layout-Messung am `.DAT`.)

## Juno post-EFB OCRU — disjunkt zum Asset, Decoder deckt

- Gemessen 2026-09-17: `atmos.nmsu.edu/PDS/data/jnogrv_0001/DATA/ODF/` HTTP 200, 26 `.ODF` 2013-284→2016-137 — disjunkt zu `juno_odf.bin` (`jnogrv_1001`, 2016-185→2017-244). `parse_odf` liest 36-Byte-DSN-ODF-Records → deckt das Layout. Offen: ob die OCRU-`data_type`-Codes im Keep-Filter `(11..=14)` (`juno_odf_compiler.rs:55`) liegen. (Schritt: Filter am `.LBL`/Record prüfen, dann Compiler-Arm für `jnogrv_0001`.)

## Geteilter Baum — eigener Pfad-Satz

- Eigene uncommittete Arbeit: `tools/harvest/src/bin/kcdc_compiler.rs` (KCDC-Fix), `phi/sources.φ` (Cassini-Block, einziger Hunk). `post.md` (ernte-Zeilen gelöscht), `external-state.md` (CI-Zeile aktualisiert) und `blocked_sources.φ` tragen fremde uncommittete Hunks → nicht im eigenen Commit-Pfad. (Schritt: `/commit` pfad-begrenzt auf die zwei Dateien + dieses Handover.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
