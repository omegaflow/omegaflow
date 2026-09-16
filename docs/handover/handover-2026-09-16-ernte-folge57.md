<!--
  title: Handover — Ernte-Folge 57 (Stand 2026-09-16)
  session: Ernte-Folge 57
  class: handover
  date: 2026-09-16
  sha256: 2d4510e8a87b6be8613e2571910a896af91e9dd9c4106d6d368235d146ffe48e
  status: live
-->
# Handover — Ernte-Folge 57 (2026-09-16)

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

## DEMETER — Harvest-Run läuft, kanonische Assets fehlen (härtester undatiert)

- Harvest-Run 35146819646 `in_progress` (headSha 42647f32, gestartet 20:29Z); `demeter-cdn` 35150299541 `pending` (Concurrency, 53 min); `demeter-aggregate-cdn` 35150610212 `success`, Restore `0 files → 0 monthly assets` = 0 `.DAT`. Gate „Harvest success UND Cache 0 `.DAT`" unerfüllt. CDN-Tag `regards.cnes.fr` trägt 77 Assets unter Alt-Namen (gemessen 2026-09-16: `gh release view regards.cnes.fr` → 77, erste `demeter_isl_003410.bin`; `curl -sI …/demeter_isl_200410.bin` → HTTP 404). (Schritt: `gh run view 35146819646`; bei success + Cache 0 `.DAT` → `gh cache delete demeter-harvest-Linux` + `gh workflow run demeter-cdn.yml` + `gh workflow run demeter-aggregate-cdn.yml`; dann die 77 `url`-Zeilen — `format demeter_isl`, `at earth`, `ttl 604800`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}` — ans Ende von `phi/sources.φ`, Namen gegen den Tag geprüft.)

## rosetta_odf — Lauf läuft, Digest pending

- Run 35148936648 `in_progress` (headSha 83fa92ec; `rosetta_odf`-Job läuft, `mro_odf` cancelled); CDN-Tag `archives.esac.esa.int`, `rosetta_odf.bin` = HTTP 404. Der Block steht `phi/sources.φ:6471–6476`, Workflow `planetary-odf-cdn.yml`. (Schritt: nach Lauf-Ende `gh run view 35148936648`; bei success `sha256`-Zeile zwischen Z. 6472 (`format rosetta_odf`) und 6473 (`at earth`), `Offen:`-Klausel am Ende von Z. 6476 streichen.)

## KCDC — Login-void ist parser-def, Fix steht, Re-Dispatch offen

- Beide Läufe 35154479872 / 35154458948 `failure` im Step „Login to the KCDC DataShop": `kcdc_compiler: login void` (exit 1). Ursache gemessen (nicht geraten): `LOGIN_PATH` ohne Slash → `301` wandelt POST→GET und verwirft Credentials/CSRF; zusätzlich fehlt der `Referer` → Django-403 (strict-referer). Secrets `KCDC_USER`/`KCDC_PASS` sind vorhanden. Fix steht in `tools/harvest/src/bin/kcdc_compiler.rs:13` (Slash) + `Referer`-Header am Login-POST (dieser Commit); `cargo check` clean, Live-Login 200 → `/accounts/profile/`, `sessionid` gesetzt. (Schritt: nach Push `gh workflow run kcdc-cdn.yml`; Log-Nachweis Login 200 + `sessionid`; bei success `kcdc_kascade.bin`-Digest als `sha256`-Zeile + Block `kcdc_kascade` in `sources.φ`.)

## argo-bgc — Dispatch läuft, Nachweis offen

- Run 35154746956 `pending` (dispatcht 21:52Z); der Gate-Fix (`id: idempotence` + `present`-Output + `if:`) steht in `151eaaad`. (Schritt: `gh run view 35154746956`; der Lauf muss bei vorhandenem Asset `present=true` zeigen und den Compile-Step überspringen — Log-Nachweis.)

## CDN-Idempotenz-Gates — 47 Workflows gefixt, Verifikation über die nächsten Läufe

- 47 `*-cdn.yml` vom void-`exit 0` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if: steps.idempotence.outputs.present == 'false'`) umgestellt; Referenz `aia-cdn` / `argo-bgc` / `kcdc`; unberührt bleiben `planetary-odf` (Same-Step-Early-Exit), `kernel-flatten`/`source-census` (Retry-Loops), `laic-verdict`/`signal-cone-audit`/`sky-crossmatch` (Soft-Fail). (Schritt: der nächste reguläre Lauf je Workflow zeigt `present=true` + übersprungenen Compile-Step; kein Massen-Dispatch — der Trigger ist der nächste reguläre Lauf.)

## Benchmark — KCDC-Diagnose gedoppelt, flash gewinnt

- Klasse: Parser-def-Diagnose (hartes Atom, kein vorheriger Sieger) — die KCDC-Login-void-Diagnose lief über `grind-pro` und `grind-flash`: identische Ursache (fehlender Slash + `Referer`), `file:line` und Fix deckungsgleich. `grind-flash` liefert dasselbe billiger → Sieger `grind-flash`; die pro-Eskalation war unnötig. Die gemessene Routine-Such-Klasse bleibt davon unberührt geschlossen (flash).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
