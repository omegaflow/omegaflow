<!--
  title: Handover — Ernte-Folge 56 (Stand 2026-09-16)
  session: Ernte-Folge 56
  class: handover
  date: 2026-09-16
  sha256: e1e114d668b235ab16f99e3b322a1740e2af6cde69768de22cec9ac18aa08692
  status: live
-->
# Handover — Ernte-Folge 56 (2026-09-16)

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

## DEMETER — Harvest-Runs offen, kanonische Assets fehlen (härtester undatierter Punkt)

- Harvest-Runs 35146819646 `in_progress` (~60 min; Budget 14000 s), 35150299541
  `pending` (Concurrency) — kein `success`; Gate „Harvest success UND Cache 0 `.DAT`"
  unerfüllt, kein Dispatch. Cache `demeter-harvest-Linux` (id 7639768970) 1 185 232 B
  (created 2026-09-13); Restore 35150610212 `0 files → 0 monthly assets` = 0 `.DAT`.
- CDN-Tag `regards.cnes.fr` liegt in `omegaflow/sources`; 77 Assets vorhanden, aber
  unter Alt-Namen `demeter_isl_003410…004102.bin` (−1970-Präfix, vor Fix 975d4d33);
  kanonisch `demeter_isl_200410.bin` = HTTP 404. (Schritt: nach Run-Ende
  `gh run view 35146819646`; bei success + Cache 0 `.DAT` `gh cache delete
  demeter-harvest-Linux` + `gh workflow run demeter-cdn.yml` + `gh workflow run
  demeter-aggregate-cdn.yml`; dann die 77 `url`-Zeilen — `format demeter_isl`,
  `at earth`, `ttl 604800`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`
  — ans Ende von `phi/sources.φ`, Namen gegen den Tag geprüft.)

## KCDC — CI-Route gebaut, erster Lauf offen

- Download gemessen: `https://kcdc.iap.kit.edu/datashop/download/1173/request.zip`
  → HTTP 200, 225 852 B, sha256 `da6e598a73dd6d1d5940fca3dac2759dcf6a2d2038fa9b0b10fc1b3d2efcfbad`,
  `PK\x03\x04`; ohne Session Login-Seite (18 372 B), `/review/…` 404.
- `.github/workflows/kcdc-cdn.yml` neu: Login aus env (`KCDC_USER`/`KCDC_PASS`,
  Repo-Credentials vorhanden), Cookie-Jar-Download, `kcdc_compiler --compile
  --ci-mode`, Upload Tag `kcdc.iap.kit.edu`, Idempotenz-Gate (aia-cdn-Muster).
  (Schritt: nach Push `gh workflow run kcdc-cdn.yml`; bei success den
  `kcdc_kascade.bin`-Digest als `sha256`-Zeile + Block `kcdc_kascade` in `sources.φ`.)

## rosetta_odf — CI-Lauf offen, Digest pending

- Run 35148936648 `in_progress` (Job 104974275001; mro_odf abgebrochen); CDN-Tag
  `archives.esac.esa.int` noch nicht angelegt → sha256 pending. Der Block steht
  `sources.φ:6471–6476` (Handover nannte ~6460). (Schritt: `gh run view
  35148936648`; bei success `sha256`-Zeile zwischen Z. 6472 (`format rosetta_odf`)
  und 6473 (`at earth`), `Offen:`-Klausel am Ende von Z. 6476 streichen.)

## argo-bgc — Workflow-Fix steht, Verifikation offen

- `argo-bgc-cdn.yml` Idempotenz-Gate wirksam gemacht: `exit 0` → `id: idempotence` +
  `present`-Output + `if: steps.idempotence.outputs.present == 'false'` am
  Compile-Step (Muster aia-cdn.yml). (Schritt: nach Push `gh workflow run
  argo-bgc-cdn.yml`; der Lauf muss bei vorhandenem Asset `present=true` zeigen und den
  Compile-Step überspringen — Log-Nachweis.)
- Danach die übrigen CDN-Workflows auf dasselbe Muster prüfen (`exit 0`-Gates ohne
  `id`/`if:`).

## Benchmark — flash-first, ein pro-Einsatz

- Fünf Punkte über `grind-flash` (DEMETER, KCDC, rosetta, argo, Format) — kein
  pro/max nötig. TNBFits über `grind-pro` (Register-Code-Kanonik, ~36 Zeilen, ein
  Pass); kein flash-Vergleichslauf (Klasse nicht gedoppelt). Die gemessene
  Routine-Klasse bleibt geschlossen (flash).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
