<!--
  title: Handover — Ernte-Folge 52 (Stand 2026-09-16)
  session: Ernte-Folge 52
  class: handover
  date: 2026-09-16
  sha256: c3fd897ec6ee5c1ace1f55d29cb90a4e8870175d777966d2dabd42990f8eb43b
  status: live
-->
# Handover — Ernte-Folge 52 (2026-09-16)

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

## VTSCat sky1 — Compiler gefixt, Asset pending

- Diagnose (2026-09-16, gemessen am CI-Log run 35137986016): vier Ursachen — (1)
  `yaml_val` brach bei Block-Form-YAML ab (`pos:` als eigene Zeile → `None`), (2)
  die Position wurde aus dem Beobachtungs-Sidecar `VER-<id>.yaml` statt aus
  `sources/tev-<id>.yaml` gelesen, (3) das Sidecar-Namensmuster
  (`replacen("-sed", "")`) 404te bei Mehrbeobachtungs-Quellen, (4) ein `nan` in
  der Upper-Limit-Spalte verwarf die ganze Zeile. Fix in `src/archivar/vtscat.rs`
  (`rows: Vec<Vec<Option<f64>>>`, Block-Form-Descent, `yaml_degrees`,
  `meta_source_id`) + `tools/harvest/src/bin/vtscat_compiler.rs` (VER-*-Filter,
  Registry-Position). `cargo check --workspace --tests` 0/0; Tests stehen, der
  CI-Testlauf ist offen. `phi/sources.φ:1143-1148` ist korrekt (format sky1,
  Einheit `m-2.s-1.tev-1`). (Schritt: nach Commit/Push `gh workflow run
  vtscat-cdn.yml`; dann Asset + sha256 gegen `sources.φ` messen.)

## DEMETER — Konsument verdrahtet, Re-Aggregation braucht Consent

- `demeter_isl`-Arm verdrahtet: `src/archivar/extract.rs` (`series_parse_bin` +
  6 Feld-Namen `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`),
  `src/archivar/main_flow.rs` (Serien-Formatliste), `src/archivar/demeter.rs`
  (`parse_series`, 8×f64-Record-Layout; der gelagerte `vs`-Slot wird nicht als
  Kanal exponiert). `cargo check` 0/0 + Test. **Keine** `sources.φ`-Registrierung:
  der kanonische Monats-Shard-Name fehlt (`demeter_isl_200410.bin` = 404, die 77
  Alt-Shards `…003410…004102` tragen ein −1970-Jahr), und ein Re-Run von
  `demeter-cdn.yml` koppelt an den CDPP-Harvest → neue Orders = Dritt-Akt.
- entscheid-Meldung gefaltet (2026-09-16): Key verifiziert (Login 200 / Bearer
  `REGISTERED_USER`; `rs-catalog` 57 760 `DMT_N1_1144`-Objekte; `rs-order` 42
  Orders, `demeter-0000…0026` meist `DONE`); der `403` war ein Auth-Gate, kein
  Route-Block; Ernte ~5 %. (Schritt: Consent über entscheid für den
  Re-Aggregations-Dispatch — oder ein Aggregat-only-CI-Job auf dem Cache-Workdir;
  danach 77 `url`-Zeilen registrieren.)
- Konzept-Kandidaten, in `phi/` nicht vorhanden: **ExoFOP** (`zeugnis.md:83`),
  **NANOGrav** (`kybernetische-astrophysik.md:330`), **SRCNet** (`zeugnis.md:90`).
  (Schritt: als Kandidaten in `phi/pipeline/queue/master.φ` prüfen/eintragen, erst
  nach geprobtem Feld/Oszillator nach `phi/sources.φ`.)

## GHRC GLM L1B + TRMM LIS — Compiler + Workflows gebaut, Dispatch offen

- Route selbst gemessen (2026-09-16, nicht Agenten-Hörensagen): keine token-freie
  NOAA-AWS-Datenroute (die NOAA-Buckets tragen nur GLM-L2-LCFA, kein L1B; TRMM LIS
  hat keine NOAA-Route); die Wissenschaftsdaten sind EDL-gated via
  `s3://ghrcw-protected/<entry_id>/`. Direkt gegen die Granule geprüft: ohne Token
  HTTP **401**, mit `EARTHDATA_EDL_TOKEN` (682 Zeichen) HTTP **206**,
  `application/x-netcdf` — für TRMM LIS (`TRMM_LIS_SC.04.1_1998.001.00539.nc`) und
  GLM L1B (`OR_I_GLM-L1b-Event_G16_…nc`). Der netCDF-4/HDF5-Parser existiert
  (`src/archivar/hdf5.rs`). Compiler `tools/harvest/src/bin/glm_l1b_compiler.rs` +
  `trmm_lis_compiler.rs`, `src/archivar/geo.rs`, `extract.rs`, `main_flow.rs`, die
  `sources.φ`-Zeilen (format `trmm_lis`/`glm_l1b`) und die Workflows
  `.github/workflows/trmm-lis-cdn.yml` + `glm-l1b-cdn.yml` (Muster `iss-lis-cdn.yml`,
  `--ci-mode`, Secret `EARTHDATA_EDL_TOKEN`) stehen, `cargo check` 0/0. (Schritt:
  nach Push `gh workflow run trmm-lis-cdn.yml glm-l1b-cdn.yml`; dann Asset +
  sha256 gegen `sources.φ` messen.)

## Zenodo — Quaoar gemessen, TNBFits pending

- Quaoar (`zenodo.21185812`): HTTP 200, `content-length` 572467032 == erwartet;
  md5 `420a1e9435d569a600c195d39d87dce8` ✓; sha256
  `1473129fee9e488dec3587dc5ef83f921cfd9f9b0a945f662fe88b0a75577574`. Zenodo ist
  wieder erreichbar (das frühere 504 ist nicht mehr gemessen).
- TNBFits (`zenodo.10620251`): HTTP 200, `content-length` 14232134885 == erwartet;
  md5 `a1aada719292a579bf0695e02e7d2afd` (Zenodo-veröffentlicht, nicht unabhängig
  gehasht); sha256 `pending` — 13,25 GiB > 2-GiB-CDN-Asset-Limit, das
  CI-Stream-+-Granulat-Verfahren existiert noch nicht. (Schritt: CI-Workflow bauen,
  der den Stream zieht, `sha256sum` faltet und in <2-GiB-Granulate schneidet.)

## CDN-Nachzügler — dispatched, Ergebnis offen

- `planetary-odf-cdn` run 35139594201 + `argo-bgc-cdn` run 35139598360 dispatched
  2026-09-16 (der vorige PODF-Lauf 35138720790 war cancelled). (Schritt: `gh run
  view <id>` — Ergebnis in die nächste Handover-Zeile.)

## Fink — Flux-Serie gebaut, TAI-Fold ungetestet

- SKD1 um eine additive Flux-Serie erweitert (`src/archivar/skydirection.rs`,
  `tools/harvest/src/bin/skydirection_compiler.rs`): `r:psfFlux`/`r:psfFluxErr`/
  `r:midpointMjdTai` werden gehalten (band r, nJy, TAI→TDB via
  `fink_mjd_tai_to_tdb`), Flux 0 als Messwert, fehlende Err als `None`;
  `phi/witnesses.φ:21` angepasst. (Schritt: `fink_mjd_tai_to_tdb` gegen den
  Live-Konus messen oder ein Testmodul ergänzen.)

## KASCADE-Grande — SSO, Operator-gebunden

- DataShop ist Keycloak-SSO/JS, kein URL/POST-Endpoint; `blocked_sources.φ:63`
  korrekt. (Schritt: Post an entscheid — Minimal-Job mit dem `omegaflow`-Konto,
  ASCII-Download, Spaltenlayout messen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
