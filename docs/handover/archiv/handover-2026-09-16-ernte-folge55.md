<!--
  title: Handover — Ernte-Folge 55 (Stand 2026-09-16)
  session: Ernte-Folge 55
  class: handover
  date: 2026-09-16
  sha256: 733c6a144587cb10de258f9091f811e152db8d960089e34d1ec288404c83f3dd
  status: live
-->
# Handover — Ernte-Folge 55 (2026-09-16)

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

## DEMETER — Aggregat-Cache leer, Re-Aggregation + Register offen

- `demeter-aggregate-cdn.yml` lief (35150610212, success), aber
  `aggregate: 0 files → 0 monthly assets`: der restaurierte Cache
  `demeter-harvest-Linux` trägt 0 `.DAT`-Dateien; der Harvest-Job hat
  `phi/pipeline/demeter_work` nicht befüllt (Harvest-Runs 35146819646 `in_progress`
  / 35150299541 `pending`). Der Cache-Key ist statisch — ein einmal leer
  gespeicherter Cache wird unter demselben Key nicht neu geschrieben. (Schritt:
  nach Abschluss der Harvest-Runs `gh cache list --key demeter-harvest`; bleibt der
  Cache leer → `gh cache delete demeter-harvest-Linux` + `gh workflow run
  demeter-cdn.yml`, dann `gh workflow run demeter-aggregate-cdn.yml`; erwartet
  `N files → 77 monthly assets`.)
- `sources.φ`: DEMETER fehlt. CDN-Tag gemessen `regards.cnes.fr` (nicht `earth`);
  kanonische Namen `demeter_isl_200410.bin` … `demeter_isl_201102.bin` (77). (Schritt:
  nach dem Aggregat die 77 `url`-Zeilen — `format demeter_isl`, `at earth`,
  `ttl 604800`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}` —
  Assetnamen vorher gegen die 77 kanonischen prüfen.)

## KCDC — Request 1173 offen zum Download + Register

- Der erste Job-Submit ist ausgeführt (Operator-Wort, 2026-09-16): `POST /datashop/`
  → HTTP 200 `{"redir": "/datashop/review/"}`; Request `1173/request.zip`,
  `output_format ASCII`, `status SUCCESS`. Gemessen: `quants`/`descr` brauchen
  `X-Requested-With: XMLHttpRequest` (im Compiler
  `tools/harvest/src/bin/kcdc_compiler.rs` nachgetragen). (Schritt: `1173/request.zip`
  über den Review-/Request-Download laden, `kcdc_compiler --compile <zip> --out
  <bin> --lsk <naif0012.tls> --ci-mode` → CDN-Asset, dann `sources.φ`-Block
  `kcdc_kascade`.)
- `phi/blocked_sources.φ:63` fortgeschrieben (Submit gemessen).

## rosetta_odf (planetary-odf) — CI-Lauf offen

- Run 35148936648 `in_progress`; `rosetta_odf.bin` noch nicht auf dem CDN-Tag
  `archives.esac.esa.int`. (Schritt: `gh run view 35148936648`; bei success den
  `rosetta_odf.bin`-Digest als `sha256`-Zeile in `sources.φ` ~6460 setzen und das
  `Offen:` ~6465 streichen.)

## argo-bgc — Idempotenz-Gate überspringt nicht (Workflow-Fix offen)

- sha256 verifiziert: das CDN-Asset `argo_bgc.bin` trägt Digest
  `sha256:7b6e18a062eb32ce2bd4022eb23421f056d84762e189cb9caae664786a369215`,
  deckt `sources.φ:6558` (Handover nannte 6547). Run 35148940793 `pending`
  (Concurrency-Gruppe `argo-bgc-cdn`, gehalten von Run 35115386397
  `in_progress`), keine Jobs, kein Log.
- `argo-bgc-cdn.yml:21–28` nutzt `exit 0` ohne `id`/Output und ohne `if:` am
  Compile-Step — der Satz `asset argo_bgc.bin already present — manifest
  skipped` (Run 35103101734, 2026-09-16T13:37:37Z) überspringt nichts: derselbe
  Run kompilierte weiter (`Running argo_bgc_profile_compiler` 13:38:53) und lud
  18:01:05 neu hoch. (Schritt: auf das `aia-cdn`-Muster umstellen — `id:
  idempotence` + `present`-Output + `if: steps.idempotence.outputs.present ==
  'false'`; danach die übrigen CDN-Workflows auf dasselbe Muster prüfen.)

## TNBFits — sha256 steht; `cdn_reconcile` kennt kein Sharded-Modell

- Der TNBFits-Block in `sources.φ` trägt sha256
  `374a374fae7fff03c9de7b2a1799c2e49768a575ff20a75d111240838f2bdf58` (aus
  `tnbfits.manifest`, Lauf 35148401053, 14 Shards; md5 deckt den Registerwert).
- `cdn_reconcile` erwartet den Einzelnamen `Proudfoot23_TNBFits.zip`, tatsächlich
  liegen 14 Shards + `tnbfits.manifest` → falsches `missing_assets` + 15
  `divergence`-Rows. (Schritt: `cdn_reconcile`
  (`tools/register/src/bin/cdn_reconcile.rs:284–312`) um eine Manifest-/Shard-Kanonik
  erweitern — Einzelname → `name.000…` + `<name>.manifest`, Manifest als kanonischer
  Nachweis.)

## Format-Gate — ernte-Dateien formatiert, CI-Nachweis offen

- Die vier ernte-eigenen Dateien `src/archivar/{demeter,tests,vtscat}.rs`,
  `src/mathematikerin/s2.rs` sind formatiert (rustfmt-Diff aus dem CI-Log angewandt),
  `cargo check` 0/0. (Schritt: mit dem Session-Commit pushen, dann
  `gh workflow run ci-check.yml`; `format`-Job ernte-frei.)
- `ci-check` `test` fährt jetzt `skydirection_compiler` + `kcdc_compiler`; Run
  35151105722 dispatched. (Schritt: `gh run view 35151105722`; `test`-Job grün.)

## Benchmark — flash-first, kein pro/max nötig

- Alle sechs Punkte liefen über `grind-flash`; keine Eskalation auf pro/max war
  nötig — auch die novel KCDC-Body-Konstruktion (cuts-Struktur aus `KAOSDataShop.js`
  gelesen) löste flash. Die gemessene Routine-Klasse bleibt geschlossen (flash).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
