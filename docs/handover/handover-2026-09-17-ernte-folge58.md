<!--
  title: Handover — Ernte-Folge 58 (Stand 2026-09-17)
  session: Ernte-Folge 58
  class: handover
  date: 2026-09-17
  sha256: 237953e48296a740cfcd14975014eaf614a493952c9f56055ba5cb07fb5cb674
  status: live
-->
# Handover — Ernte-Folge 58 (2026-09-17)

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

## DEMETER — Harvest-Lauf läuft, kanonische Assets fehlen (härtester undatiert)

- Harvest-Run 35146819646 `in_progress` (5h-Budget, gestartet 20:29Z); `demeter-cdn` 35150299541 `pending` (Concurrency); `demeter-aggregate-cdn` 35150610212 `success`, Restore `0 files → 0 .DAT`. CDN-Tag `regards.cnes.fr` trägt 77 Assets unter Alt-Namen (gemessen 2026-09-16). (Schritt: `gh run view 35146819646`; bei success + Cache 0 `.DAT` → `gh cache delete demeter-harvest-Linux` + `gh workflow run demeter-cdn.yml` + `gh workflow run demeter-aggregate-cdn.yml`; dann die 77 `url`-Zeilen — `format demeter_isl`, `at earth`, `ttl 604800`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}` — ans Ende von `phi/sources.φ`.)
- Post von entscheid (2026-09-16): neue DEMETER-Orders sind ein Dritt-Akt über die `entscheid`-Linie; die Harvest-Registrierung (die 77 `url`-Zeilen) liegt bei ernte.

## TRK-2-34 (Cassini/MAVEN/DART) — DART registriert, MAVEN/Cassini laufen

- `dart_tnf.bin` manifestiert (run 35156059824 `success`; 620394632 B, sha256 `b38e476a7a938efd574163b4d8671566a0b3f8ce6c1ea11e3ed59d52ad0749fb`) und in `phi/sources.φ` registriert (Block `dart_tnf`, Feld `dart_tnf_ul_phase_cycles`, `at earth`, `ttl 604800`). MAVEN 35156057281 + Cassini 35156062244 `in_progress`, Assets noch nicht am Tag. (Schritt: `gh run view 35156057281` / `35156062244`; bei success je `url https://github.com/omegaflow/sources/releases/download/<pds-ppi.igpp.ucla.edu|atmos.nmsu.edu>/<maven_tnf.bin|cassini_tnf.bin>` + `format <maven_tnf|cassini_tnf>` + `sha256 <digest>` + `at earth` + `ttl 604800` + `field ul_phase_cycles <fmt>_ul_phase_cycles inverse-square em cycle 604800 0.0 0.0` ans Ende von `phi/sources.φ`.)

## KCDC — Parser-Join gefixt, Request 1175 (98 832 Events), CI-Lauf offen

- Submit ausgeführt (Operator-Wort, 2026-09-17): Request 1174 (Instant-Cut → 1 Event) und 1175 (`Datetime`-Bereich `2005-06-01..2005-06-02` → 98 832 Events; general 98 831, array 79 906, grande 19 588, calorimeter/lopes leer). Gemessen: `row_mapping.txt` = Tabellen-Namen-Zeile + eine Zeile je General-Event, Wert = Tabellen-Zeilenindex; `Datetime`-Cut `[d,d]` trifft exakt den Instant `dT00:00:00`. Join-Fix gebaut (`src/archivar/kcdc.rs` `parse_row_mapping`/`own_gt_times`/`mapped_times`; `kcdc_compiler.rs` `compile_table(times)`), `cargo check` clean; `kcdc-cdn.yml` auf Request 1175 + Trailing-Slash-URL (`…/request.zip/`) + Bounded-Retry. (Schritt: nach `/commit`+Push `gh workflow run kcdc-cdn.yml`; bei success `kcdc_kascade.bin`-Digest als `sha256`-Zeile + Block `kcdc_kascade` in `sources.φ`. Design-Grenze: der Request läuft 2026-09-30 ab — Re-Submit = Dritt-Akt über entscheid.)

## rosetta_odf — Lauf läuft, Digest pending

- Run 35148936648 `in_progress` (Job 104974275001); `mro_odf` `failure`, Log erst nach Run-Ende. Block `phi/sources.φ:6471–6476`. (Schritt: nach Lauf-Ende `gh run view 35148936648`; bei success die `sha256`-Zeile zwischen Z. 6472 (`format rosetta_odf`) und 6473 (`at earth`) setzen, `Offen:`-Klausel am Ende von Z. 6476 streichen; den `mro_odf`-Fehler messen.)

## argo-bgc — Dispatch läuft, Nachweis offen

- Run 35154746956 `pending`. (Schritt: `gh run view 35154746956`; der Lauf muss bei vorhandenem Asset `present=true` zeigen und den Compile-Step überspringen — Log-Nachweis.)

## CDN-Idempotenz-Gates — 47 Workflows gefixt, Verifikation über die nächsten Läufe

- 47 `*-cdn.yml` vom void-`exit 0` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt; unberührt `planetary-odf`, `kernel-flatten`/`source-census`, `laic-verdict`/`signal-cone-audit`/`sky-crossmatch`. (Schritt: der nächste reguläre Lauf je Workflow zeigt `present=true` + übersprungenen Compile-Step; kein Massen-Dispatch.)

## Benchmark — KCDC-Diagnose gedoppelt, flash gewinnt

- Klasse: Parser-def-Diagnose (hartes Atom). `grind-pro` und `grind-flash` lieferten identische Ursache (fehlender Slash + `Referer`); Sieger `grind-flash` — die pro-Eskalation war unnötig. Die gemessene Routine-Such-Klasse bleibt geschlossen (flash).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
