<!--
  title: Handover — Ernte-Folge 59 (Stand 2026-09-17)
  session: Ernte-Folge 59
  class: handover
  date: 2026-09-17
  sha256: 1d0de55e95c4ae060ac90bb4c27884789f168a4c429ee9e01b3ec5bf22ea9a81
  status: live
-->
# Handover — Ernte-Folge 59 (2026-09-17)

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

## KCDC — Join-Bug gefixt (Stem-Mismatch), CI-Lauf + Registrierung offen (härtester undatiert)

- Gemessen (CI-Lauf `35157739414`): `kcdc_compiler --compile` lieferte 0 Records — `mapped_times` bekam `"array.txt"`, die Registry-Stämme sind aber `calorimeter|grande|general|array|lopes` → leeres Vec → jede Zeile „without time". Fix (diese Session, `tools/harvest/src/bin/kcdc_compiler.rs`): `component_maps` trägt die Stämme, Dateiname = `{stem}.txt`, `mapped_times` bekommt den Stamm; die `general`-Tabelle nutzt `own_gt_times` (ihr eigenes Gt, gemessen 98832 timed). `cargo check` clean; Regressionstest `compile_joins_component_rows_through_the_registry_stems`. Der Re-Dispatch `35157749807` lief noch mit dem alten Code (`failure`) — erwartet, nicht aussagekräftig. (Schritt: nach `/commit`+Push `gh workflow run kcdc-cdn.yml`; bei success `kcdc_kascade.bin`-Digest als `sha256`-Zeile + Block `kcdc_kascade` in `sources.φ`. Design-Grenze: der Request läuft 2026-09-30 ab — Re-Submit = Dritt-Akt über entscheid.)

## DEMETER — Harvest-Lauf läuft, kanonische Assets fehlen

- Run `35146819646` `in_progress` (gestartet 20:29Z, 5h-Budget); `demeter-cdn` `35150299541` pending; `demeter-aggregate-cdn` `35150610212` success, Restore 0 `.DAT`. CDN-Tag `regards.cnes.fr` trägt 77 Assets unter Alt-Namen. (Schritt: `gh run view 35146819646`; bei success + Cache 0 `.DAT` → `gh cache delete demeter-harvest-Linux` + `gh workflow run demeter-cdn.yml` + `gh workflow run demeter-aggregate-cdn.yml`; dann die 77 `url`-Zeilen — `format demeter_isl`, `at earth`, `ttl 604800`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}` — ans Ende von `phi/sources.φ`.)

## TRK-2-34 (Cassini/MAVEN) — Cassini success, Registrierung durch fremden `sources.φ`-Hunk blockiert

- `dart_tnf.bin` manifestiert und in `phi/sources.φ` registriert. **Cassini `35156062244` success**: Asset `cassini_tnf.bin`, 191163176 B, sha256 `b8d0dc527febf16bd1c3d2172d5e83acd921e39f142efa0a2204983faf129ab6` (gemessen via `gh release view -R omegaflow/sources atmos.nmsu.edu`). Die Registrierung ist blockiert: `phi/sources.φ` trägt einen fremden uncommitteten Hunk (`odyssey_odf`-Shards, Linie forschung) — ein `sources.φ`-Commit würde fremde Arbeit sweepen, darum nicht angefasst. MAVEN `35156057281` `in_progress` (Duplikat pending: `35156078571`). (Schritt: sobald `phi/sources.φ` sauber ist, `url https://github.com/omegaflow/sources/releases/download/atmos.nmsu.edu/cassini_tnf.bin` + `format cassini_tnf` + `sha256 b8d0dc52…9ab6` + `at earth` + `ttl 604800` + `field ul_phase_cycles cassini_tnf_ul_phase_cycles inverse-square em cycle 604800 0.0 0.0` ans Ende setzen; MAVEN bei success analog mit `pds-ppi.igpp.ucla.edu/maven_tnf.bin`.)

## rosetta_odf — Lauf läuft, Digest pending

- Run `35148936648` `in_progress`; `mro_odf` failure, Log erst nach Run-Ende. Block `phi/sources.φ:6471–6476`. (Schritt: nach Lauf-Ende `gh run view 35148936648`; bei success die `sha256`-Zeile zwischen Z. 6472 (`format rosetta_odf`) und 6473 (`at earth`) setzen, `Offen:`-Klausel am Ende von Z. 6476 streichen; den `mro_odf`-Fehler messen.)

## argo-bgc — Dispatch läuft, Nachweis offen

- Run `35154746956` pending. (Schritt: `gh run view 35154746956`; der Lauf muss bei vorhandenem Asset `present=true` zeigen und den Compile-Step überspringen — Log-Nachweis.)

## CDN-Idempotenz-Gates — Verifikation über die nächsten Läufe

- 47 `*-cdn.yml` vom void-`exit 0` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt. (Schritt: der nächste reguläre Lauf je Workflow zeigt `present=true` + übersprungenen Compile-Step; kein Massen-Dispatch.)

## post.md — „An ernte"-Zeilen löschen

- Die Datei trägt fremde uncommittete Hunks (`An bau: …`), darum in dieser Session nicht angefasst. Die TRK-2-34-Zeile ist in Handover 58 gefaltet; neu adressiert an ernte: `Mariner PSPA-00316` (SPDF Venus-Okkultation, 9 `.tar` ~19,5 MB, UNIVAC-1108, 7-Bit-Stream) als `voyager_saturn`-Geschwister, und `Juno post-EFB OCRU` (`atmos.nmsu.edu/PDS/data/jnogrv_0001/`) ↔ `juno_odf.bin`-Abgleich. (Schritt: bei sauberer Datei die beiden `An ernte:`-Zeilen löschen; Mariner/Juno als Ernte-Duty aufnehmen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
