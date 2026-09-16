<!--
  title: Handover — Ernte-Folge 54 (Stand 2026-09-16)
  session: Ernte-Folge 54
  class: handover
  date: 2026-09-16
  sha256: 4ac06fdc7f87e9649eac035d516202f8b50a2f5ff448fee19010f526294a27a4
  status: live
-->
# Handover — Ernte-Folge 54 (2026-09-16)

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

## DEMETER — Aggregat-Cache-Pfad gefixt, Re-Aggregation + Register offen

- Der aggregat-only-Job lief `0 files → 0 monthly assets`: der Cache-Restore
  verfehlte, weil `actions/cache` nach dem path-Set namespace't — der Harvest-Job
  listet drei Pfade, der Aggregat-Job nur einen. `.github/workflows/demeter-aggregate-cdn.yml`
  restauriert jetzt dieselben drei (`phi/pipeline/demeter_work`, `demeter_ledger.txt`,
  `demeter_urns.txt`); `demeter-cdn.yml` trägt `save-always: true`. (Schritt: nach Push
  `gh workflow run demeter-aggregate-cdn.yml`; `gh run view <id>` — dann die kanonischen
  `demeter_isl_{y}{m}.bin` gegen die 77 Alt-Shards mit −1970-Jahr prüfen.)
- `sources.φ`: DEMETER ist nicht registriert; sobald die kanonischen Monats-Shards
  manifestieren, 77 `url`-Zeilen (`format demeter_isl`, `at earth`, `ttl 604800`,
  Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`). (Schritt: nach dem
  Aggregat-Lauf.)

## KCDC / KASCADE-Grande — Job-Form gemessen, Submit = Operator-Wort

- Job-Formularfelder gemessen (2026-09-16, `KAOSDataShop.js` `do_post`): der POST-Body
  trägt genau zwei Felder — `out_format` (`ascii|root|hdf5`) und `data`
  (`JSON.stringify(collected_data)`); der CSRF-Token reist im `X-CSRFToken`-Header (nie im
  Body), URL `/datashop/<prefix>` (Prefix `""`). `kcdc_compiler` baut den Body über
  `--job <format> --data <file>` (druckt das exakte Kommando, kein Exec); `blocked_sources.φ:63`
  fortgeschrieben. **Submit = Dritt-Akt, Operator-Wort** (gehört als `An entscheid:`-Zeile in
  `post.md` — die Datei trägt einen uncommitteten fremden Hunk, darum hier getragen): act
  `KCDC DataShop erster Job-Submit`; command `cargo run -p omegaflow-harvest --bin kcdc_compiler
  --job ascii --data <collected_data.json> --prefix "" --jar tmp/kcdc_cookies.txt` → exaktes
  curl-Kommando, Ausführung = `--submit <body>`; das data-JSON kommt aus `--quants`/`--descr`
  (autonom). (Schritt: Operator-Wort; danach Server-Akzeptanz, CDN-Asset, `sources.φ`-Block
  `kcdc_kascade`.)

## TNBFits — Workflow läuft, sha256 offen

- `.github/workflows/tnbfits-cdn.yml` dispatched: run 35148401053 (stream+split, md5-Gate,
  per-Shard-Fold, Release-ensure alle `success` — die 13,25 GiB Shards passten auf
  `ubuntu-latest`, kein Disk-Shortfall). Das Manifest liegt **nur** als Release-Asset
  (`zenodo.org`), nicht als `upload-artifact`. (Schritt: nach Abschluss `tnbfits.manifest`
  vom Release `zenodo.org` lesen und die `sha256`-Zeile in `sources.φ` setzen.)
- `cdn_reconcile` kennt kein Sharded-Asset-Modell: der kanonische Einzelname
  `Proudfoot23_TNBFits.zip` erscheint als fehlend, bis das Modell Shards lernt. (Schritt:
  Modell erweitern — oder den Befund stehen lassen.)

## CDN-Nachzügler — planetary-odf / argo-bgc neu dispatched

- Die alten Runs (35139594201, 35139598360) liefen ohne Runner in `completed/cancelled`;
  neu dispatched: planetary-odf **35148936648**, argo-bgc **35148940793** (beide `pending`,
  `jobs: []`). (Schritt: `gh run view <id>`; bei Erfolg Asset + sha256 gegen `sources.φ`.)

## Fink + Format-Gate — CI-Lauf offen

- Fink: die 3 Tests in `skydirection_compiler.rs` hatten keine CI-Abdeckung — `ci-check`
  `test` fährt `cargo test --release` nur über das Root-Paket (`default-members = ["."]`).
  `ci-check.yml` `test` trägt jetzt `cargo test --release -p omegaflow-harvest --bin
  skydirection_compiler` + `--bin kcdc_compiler`. (Schritt: nach Push `gh workflow run
  ci-check.yml`; `test`-Job grün.)
- Format: `kcdc_compiler.rs` (in b177919f hinzugefügt, im 16-Hunk-Pass verfehlt) auf den
  `ci-check`-format-Diff formatiert. Offen bleiben die ernte-letzt-berührten
  `src/archivar/{demeter,tests,vtscat}.rs`, `src/mathematikerin/s2.rs`; der `format`-Job
  bleibt zusätzlich durch fremde Linien rot (`src/archivar/{fetch,parse}.rs`,
  `src/mathematikerin/te.rs`, `tools/measure/src/bin/*`, `tools/utils/src/bin/archive_search.rs`).
  (Schritt: rustfmt-Diff je Datei anwenden — ernte: die vier `src/`-Dateien.)

## doi.org (ETHZ) — kein Geo-Block, Zielseite rate-limitiert

- `10.3929/ethz-c-000797709` löst per **302** auf `www.research-collection.ethz.ch/handle/20.500.11850/797709`;
  die Zielseite gibt **429 „Too Many Requests"** (Apache-Rate-Limit, **kein** Geo-Block), auch über den
  `.ch`-Exit (`135.136.39.36`) — `proton-wg.sh ch` hilft nicht (Limit ist nicht exit-gebunden). Die
  Metadaten liegen via DataCite vollständig vor: „Observing spatial and temporal variations in the
  atmospheric chemistry of rocky exoplanets: Prospects for mid-infrared spectroscopy", Braam &
  Angerhausen, EDP Sciences / A&A, 2026, Alt-DOI `10.1051/0004-6361/202557807`, CC BY 4.0. (Schritt:
  keine Landingpage-Auflösung nötig — die DOI ist keine Datenquelle.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
