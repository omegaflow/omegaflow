<!--
  title: Handover — Ernte-Folge 61 (Stand 2026-09-17)
  session: Ernte-Folge 61
  class: handover
  date: 2026-09-17
  sha256: 611eb673afbfd740e2a2f2d757038609d49789d7a1726dbbd732b25aca8dbcb0
  status: live
-->
# Handover — Ernte-Folge 61 (2026-09-17)

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

## MAVEN TNF — 2.34 GB über dem GitHub-Asset-Cap (härtester undatiert)

- MAVEN-Lauf `35156057281` failure: Compile ok (`32529787 TNF DT0 samples`, roundtrip parses), Upload HTTP 422 — `maven_tnf.bin` = 2342144672 B > 2147483648 B (2 GiB Release-Asset-Cap); kein Asset unter `pds-ppi.igpp.ucla.edu` (`gh release view`). (Schritt: Chunking wie `voyager_odr_s0..s13` oder alternater Manifest-Pfad; dann `maven_tnf`-Block in `phi/sources.φ`.)

## Mariner 10 PSPA-00316 — Route lebt, Compiler + Layout offen

- Gemessen 2026-09-17: `spdf.gsfc.nasa.gov/pub/data/mariner/mariner10/celestial_mechanics_and_radio_science/red_tele_signal_data_venus_occlt/` HTTP 200, 9 `.tar` (~19,6 MB je) ustar; das kanonische `DD029604_F1.DAT` trägt bereits 2-Byte-Header-BINARY (`STREAM_TYPE BINARY`, `NSSD1346`), kein 7-Bit-Unpacker nötig. Fehlt: ein `mariner_occlt`-Compiler (Geschwister `voyager_saturn_compiler.rs`) + die NSSD1346-Record-Layout-Messung (max 4108 B ≠ voyager 8066 B). (Schritt: Grind-Draft + Layout-Messung am `.DAT`.)

## rosetta_odf / mro_odf — Job-Caps, keine Assets

- Gemessen 2026-09-17 (`gh run view 35148936648`): Job `rosetta_odf` `cancelled` (4h-Job-Cap), Job `mro_odf` `failure`; kein `rosetta_odf.bin` unter `archives.esac.esa.int`, kein `mro_odf.bin` unter `pds-geosciences.wustl.edu` (`gh release view`). Block `phi/sources.φ:6471–6476` bleibt ohne `sha256`-Zeile, `Offen:`-Klausel Z. 6476 steht. (Schritt: `gh run view 35148936648 --log` für den `mro_odf`-Fehler; Re-Dispatch mit längerem/gesplittetem Budget.)

## Juno post-EFB OCRU — kompatibel, Compiler-Arm offen

- Gemessen 2026-09-17: OCRU-Doppler-Records tragen `data_type` 11/12/13 (one-way/two-way/three-way) — alle im Keep-Filter `(11..=14)` (`tools/harvest/src/bin/juno_odf_compiler.rs:55`); `data_type 37` (SRA-Range) fällt bewusst heraus. `parse_odf` deckt das Format-2-36-Byte-Layout (`src/archivar/odf.rs:61–86`). Verzeichnis `atmos.nmsu.edu/PDS/data/jnogrv_0001/DATA/ODF/` trägt **21** `.ODF`+`.LBL` (nicht 26), 2013-284→2016-137, disjunkt zu `juno_odf.bin` (`jnogrv_1001`). (Schritt: `juno_odf_compiler.rs:5` von `jnogrv_1001` auf eine `jnogrv_0001`-Variante erweitern + CDN-Workflow-Arm; Registrierung `phi/sources.φ` nach success.)

## DEMETER — Harvest 0 Dateien (Quellen-WAF)

- Harvest `35146819646` success = Budget-Stopp: Log `0 files on disk, 0 orders done`; jede Order `WAF blocked … waiting 1800s` / `order parse void`. (Schritt: neue Orders = Dritt-Akt → Consent über `entscheid`; ohne WAF-Durchlass keine `.DAT`. `post.md` trägt fremde uncommittete Hunks → Post-Zeile erst nach dessen Bereinigung.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt; erster Nachweis argo-bgc `35154746956` `present=true` + übersprungener Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein Massen-Dispatch.)

## Geteilter Baum — eigener Pfad-Satz

- Fremde uncommittete Arbeit (nicht im eigenen Commit-Pfad): `src/archivar/{atdf,odf}.rs`, `src/gate/{commit_gate.rs,commit_gate_vocab.json}`, `tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs`, `.github/workflows/tonga-lamb-crosscheck.yml`, `docs/handover/post.md`, `docs/zustand/external-state.md`, `phi/blocked_sources.φ`. (Schritt: eigener Pfad = `phi/sources.φ` (KCDC-Block, Z. 9207–9232) + dieses Handover + `folge60` → `archiv/`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
