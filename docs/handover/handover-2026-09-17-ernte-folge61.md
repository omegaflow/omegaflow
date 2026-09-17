<!--
  title: Handover — Ernte-Folge 61 (Stand 2026-09-17)
  session: Ernte-Folge 61
  class: handover
  date: 2026-09-17
  sha256: 72f6bf52b937073058b4a8f8fefd6e0d3fc378e6870b9bc31780d2694d16d3e9
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

## MAVEN TNF — Chunking gebaut, Re-Dispatch + Registrierung offen (härtester undatiert)

- Lauf `35156057281` failure: Compile ok (`32529787 TNF DT0 samples`, roundtrip parses), Upload HTTP 422 — `maven_tnf.bin` = 2342144672 B > 2147483648 B (2 GiB Release-Asset-Cap); kein Asset unter `pds-ppi.igpp.ucla.edu` (`gh release view`). Fix (diese Session, `cargo check` clean): `tools/harvest/src/bin/maven_tnf_compiler.rs` schardet über `odf::podf_shard_ranges`/`odf::PODF_SHARD_BUDGET`/`odf::podf_shard_name` (Muster `mro_odf_compiler.rs:142–219`) → `maven_tnf_t<lo>_<hi>.bin`; `.github/workflows/maven-tnf-cdn.yml` idempotent auf `^maven_tnf(\.bin|_t)`. (Dispatch `35188036501` läuft auf HEAD `8a38932d`. Schritt: `gh run view 35188036501`; bei success die vom Compiler gedruckten `url`/`format maven_tnf`/`field ul_phase_cycles …`-Zeilen ans Ende von `phi/sources.φ`.)

## Mariner 10 PSPA-00316 — Compiler + Parser gebaut, Dispatch + Registrierung offen

- NSSD1346 gemessen (diese Session): Record = 2-Byte-BE-Header `10 0a` (=4106) + 4106-Byte-Payload = 4108 B (`DD029604_F1.DAT` = 19.611.592 B = 4774×4108; Voll-Scan 9561 Records beider Dateien, 0 Verstöße); Payload[6..9] = Stunde/Minute/Sekunde **roh-binär** (nicht BCD — 0x2a in DD029605 beweist), [9] Sub-Sekunde (Einheit `pending`), [10..4105] = 4096 6-Bit-Sign-Magnitude-Samples (Bit5=Sign, Bit4=Duplikat). Gebaut: `src/archivar/mariner_occlt.rs` (Magic `MOCC`, `[f64;12]`, Komponenten `amp_min/max/mean`), `tools/harvest/src/bin/mariner_occlt_compiler.rs` (9 `.tar`), `.github/workflows/mariner-occlt-cdn.yml`, Konsument (`extract.rs`/`main_flow.rs:2130`/`mod.rs`), Dispatch-Test; `cargo check --all-targets` clean. (Dispatch steht aus: `gh workflow run mariner-occlt-cdn.yml` — beim Versuch 2026-09-17 07:16Z **GH-API-Rate-Limit 403**, kein Retry-Loop; bei success Register-Block `mariner_occlt` in `phi/sources.φ` + `…_register_field_names_match_components`-Test im selben Atom.)
- Pending (eigene Atom-Linie): Sample-Rate/Record-Dauer (Tag-Inkremente 18…20.898, Records nicht äquidistant), Frac-Einheit (25-ms-Ticks vs. BCD-Zentisekunden), unabhängiger UTC-Anker (1974-02-05 aus `attrib`, unbestätigt).

## rosetta_odf / mro_odf — Budget gehoben, Konsument-Mismatch + mro-Log offen

- Gemessen 2026-09-17 (`gh run view 35148936648`): `rosetta_odf` `cancelled` (4h-Cap), `mro_odf` `failure` (Runner-Shutdown); keine Assets unter `archives.esac.esa.int`/`pds-geosciences.wustl.edu`. Gebaut (diese Session): `planetary-odf-cdn.yml` Timeout je Matrix-Zeile (`${{ matrix.timeout || 240 }}`), `rosetta_odf` + `mro_odf` → 350 min. (Schritt: `gh run view 35148936648 --log` für den `mro_odf`-Runner-Shutdown — GH-API-Rate-Limit blockt; dann Re-Dispatch.)
- **Konsument-Mismatch (gemessen):** `rosetta_odf_compiler.rs` schreibt `ifms_agc::write_series` (MAGIC `IFMS`, 3×f64), aber `extract.rs:50` dispatcht `rosetta_odf` → `odf::parse_series` (MAGIC `PODF`); `extract.rs:237` erwartet `odf::COMP_OBSERVABLE`/`rosetta_odf_observable_hz`. Das gebaute Asset wäre ungelesen (Commit `3cba457e` überschrieb den IFMS-Arm aus `77c8be18`). (Schritt: `extract.rs` auf `ifms_agc::parse_series` + Komponenten `carrier_level_dbm`/`polar_angle_cycles`, Register-Block `phi/sources.φ:6471–6476` entsprechend — blockiert durch fremde Hunks in `sources.φ`.)

## Juno post-EFB OCRU — Arm gebaut, Dispatch + Registrierung offen

- Gemessen 2026-09-17: OCRU-Doppler-Records tragen `data_type` 11/12/13 (one-way/two-way/three-way) — alle im Keep-Filter `(11..=14)` (`tools/harvest/src/bin/juno_odf_compiler.rs:55`); `data_type 37` (SRA-Range) fällt bewusst heraus. `parse_odf` deckt das Format-2-36-Byte-Layout (`src/archivar/odf.rs:61–86`). Verzeichnis `atmos.nmsu.edu/PDS/data/jnogrv_0001/DATA/ODF/` trägt **21** `.ODF`+`.LBL` (nicht 26), 2013-284→2016-137, disjunkt zu `juno_odf.bin` (`jnogrv_1001`). Gebaut (diese Session): `juno_odf_compiler.rs` parametrisiert (`--volume`/`--out`, Default `jnogrv_1001`/`juno_odf.bin`), Matrix-Zeile `juno_ocru_odf.bin` (`--volume jnogrv_0001`) in `planetary-odf-cdn.yml`; `cargo check` clean. (Dispatch steht aus: `gh workflow run planetary-odf-cdn.yml` — GH-API-Rate-Limit 403; bei success Register-Block `juno_ocru_odf.bin` (`format juno_odf`) in `phi/sources.φ`.)

## DEMETER — Harvest 0 Dateien (Quellen-WAF)

- Harvest `35146819646` success = Budget-Stopp: Log `0 files on disk, 0 orders done`; jede Order `WAF blocked … waiting 1800s` / `order parse void`. (Schritt: neue Orders = Dritt-Akt → Consent über `entscheid`; ohne WAF-Durchlass keine `.DAT`. `post.md` trägt fremde uncommittete Hunks → Post-Zeile erst nach dessen Bereinigung.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt; erster Nachweis argo-bgc `35154746956` `present=true` + übersprungener Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein Massen-Dispatch.)

## Geteilter Baum — eigener Pfad-Satz

- Fremde uncommittete Arbeit (nicht im eigenen Commit-Pfad, gemessen 2026-09-17): `.github/workflows/ci-check.yml`, `docs/handover/handover-2026-09-17-forschung-folge56.md`, `tools/register/src/bin/concurrency_contract.rs` (gestaged), `cloudflare/wrangler.toml`, `docs/handover/post.md`, `docs/zustand/external-state.md`, die gestagten Renames `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` → `archiv/`. (Schritt: eigener Pfad = `tools/harvest/src/bin/juno_odf_compiler.rs`, `.github/workflows/planetary-odf-cdn.yml`, dieses Handover — Mariner-Batch steht in `c5e5c46f`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
