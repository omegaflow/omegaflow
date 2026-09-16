<!--
  title: Handover — Ernte-Folge 53 (Stand 2026-09-16)
  session: Ernte-Folge 53
  class: handover
  date: 2026-09-16
  sha256: da013164d809d846f9f73d361a2ac14893a43eb452172b80b22b992f40d09bcd
  status: live
-->
# Handover — Ernte-Folge 53 (2026-09-16)

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

## KCDC / KASCADE-Grande — Konsument gebaut, erster Job offen

- Rezeptur gemessen (entscheid, 2026-09-16): **kein API-Key**, kein `blocked account`.
  Der Zugang ist die Django-**Session** (`KCDC_USER`/`KCDC_PASS` in `.secrets.local`,
  Login `POST /accounts/login`) + CSRF; unauthentifiziert liefern die Endpunkte leer.
  DataShop-Endpunkte: `GET /datashop/quants/?det_prefix=&det_name=<array|grande|calorimeter|lopes>`
  → JSON, `GET /datashop/descr/?det_prefix=&det_name=<name>[&quant_name=…]` → JSON,
  `POST /datashop/<prefix>` (Prefix `""`, Header `X-CSRFToken`); Formate `root|hdf5|ascii`;
  `array`=KASCADE, `grande`=GRANDE.
- **Konsument gebaut** (2026-09-16): `tools/harvest/src/bin/kcdc_compiler.rs` +
  `src/archivar/kcdc.rs` (Format `kcdc_kascade`; Session+CSRF, quants/descr, Job-POST
  vorbereitet, header-getriebener ASCII-Reader mit plausibility-Gates, unix→TDB,
  CDN-Upload via `--ci-mode`); `extract.rs`/`main_flow.rs` verdrahtet;
  `cargo check --workspace --tests` 0/0. `blocked_sources.φ:63` auf `pending`
  (Konsument gebaut, Harvest offen) fortgeschrieben. (Schritt: den ersten Job fahren —
  die KAOSDataShop-Job-Formularfelder messen; `--submit` ist Operator-Wort (Dritt-Akt);
  danach CDN-Asset + `sources.φ`-Block.)

## VTSCat sky1 / GHRC TRMM-LIS + GLM-L1b — CI dispatched, Asset+sha256 offen

- Drei Workflows dispatched 2026-09-16: `vtscat-cdn` run 35144789674, `trmm-lis-cdn`
  run 35144792734, `glm-l1b-cdn` run 35144795693 (alle `in_progress`). (Schritt:
  `gh run view <id>`; dann Asset + sha256 gegen `phi/sources.φ` messen — VTSCat
  `sources.φ:1143-1148`, `glm_l1b`/`trmm_lis`.)

## TNBFits — Workflow gebaut, Dispatch + sha256 offen

- `.github/workflows/tnbfits-cdn.yml` (neu): streamt `zenodo.10620251`
  (Proudfoot23_TNBFits.zip, 14232134885 B) via curl, faltet `sha256`+`md5`
  (md5-Gate gegen `a1aada719292a579bf0695e02e7d2afd`), schneidet 14 Shards à 1 GiB
  (`Proudfoot23_TNBFits.zip.000…013`) + `tnbfits.manifest` (per-Shard sha256 +
  Offset/Länge + whole sha256/md5), Upload `--clobber` auf Release `zenodo.org`.
  `phi/sources.φ`-Zeile gesetzt (`format reference`, sha256 pending). (Schritt: nach
  Commit/Push `gh workflow run tnbfits-cdn.yml`; dann sha256 aus `tnbfits.manifest`
  lesen und die `sha256`-Zeile in `sources.φ` setzen.)
- Offen: Runner-Disk (~14 GB frei vs. 13,25 GiB Shards — der erste Lauf misst, ob es
  passt); `cdn_reconcile` kennt kein Sharded-Asset-Modell (der kanonische Einzelname
  `Proudfoot23_TNBFits.zip` erscheint als fehlend, bis das Modell Shards lernt).
  (Schritt: ersten Lauf abwarten; ggf. größerer Runner als Operator-Entscheid.)

## CDN-Nachzügler — planetary-odf/argo-bgc queued, kein Runner

- `planetary-odf-cdn` run 35139594201 + `argo-bgc-cdn` run 35139598360: status
  `pending`, `jobs: []` (seit ~52 min kein Runner zugewiesen). (Schritt:
  `gh run view <id>` erneut; bei anhaltendem Queued-State Re-Dispatch.)

## Fink — TAI-Fold getestet, CI-Lauf offen

- `tools/harvest/src/bin/skydirection_compiler.rs` trägt jetzt ein
  `#[cfg(test)] mod tests` (3 Tests): TDB−TAI = 32,184 s (naif0012.tls), MJD 60000 TAI
  → 730 555 232,184 s J2000, `None` vor der Leap-Tabelle. Die Formel
  (`unix_to_tdb(unix − leap)`) ist korrekt; der ~1,66-ms-periodische Term ist bewusst
  nicht im LSK. `cargo check --workspace --tests` 0/0. (Schritt: CI-Testlauf grün.)

## Format-Gate — 10 harvest-Compiler formatiert, CI-Lauf offen

- rustfmt-Diff aus `ci-check` run 35144463339 angewandt auf 10 ernte-eigene
  `tools/harvest/src/bin/*.rs` (16 Hunks); fremde Dateien (andere Linien) unberührt.
  `cargo check -p omegaflow-harvest` 0/0. (Schritt: CI-`format` grün.)

## DEMETER — Aggregat-only-CI-Job gebaut, Harvest offen (~5 %)

- Key verifiziert (2026-09-16): Login 200 / Bearer `REGISTERED_USER`; `rs-catalog`
  57 760 `DMT_N1_1144`-Objekte; `rs-order` 42 Orders, `demeter-0000…0026` meist
  `DONE`; der frühere `403` war ein Auth-Gate, kein Route-Block. `demeter-cdn.yml` +
  `demeter_compiler` + `src/archivar/demeter.rs` stehen.
- **Aggregat-only-Job gebaut**: `.github/workflows/demeter-aggregate-cdn.yml` —
  restauriert den Harvest-Cache (`demeter-harvest-${{ runner.os }}`), baut nur
  `demeter_compiler`, läuft `--aggregate … --ci-mode`, lädt auf die CDN. **Keine**
  CDPP-Auth, **keine** neuen Orders. (Schritt: nach Push `gh workflow run
  demeter-aggregate-cdn.yml`; dann die 77 −1970-Jahr-Shards gegen die Monats-Namen
  prüfen.)
- Limit: der Cache ist ein eingefrorener Snapshot (kein `save-always` in
  `demeter-cdn.yml`) + 7-Tage-Eviction → nur die Monate im Snapshot werden
  re-aggregiert. (Schritt: `save-always: true` in `demeter-cdn.yml` — Harvest-Job,
  eigener Hunk.) Die Harvest-Fortsetzung selbst bleibt order-gebunden (Dritt-Akt).
- Konzept-Kandidaten in `phi/pipeline/ledger.φ` eingetragen (2026-09-16): **ExoFOP**,
  **NANOGrav**, **SRCNet** (alle `ausstehend/kandidat`, Roots HTTP 200 gemessen;
  Herkunft `zeugnis.md:83`/`:90`, `kybernetische-astrophysik.md:330`). (Schritt: je
  Datenendpoint + Feld/Oszillator probe, erst dann `sources.φ`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
