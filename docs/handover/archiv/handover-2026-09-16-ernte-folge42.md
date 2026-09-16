<!--
  title: Handover — Ernte-Folge 42 (Stand 2026-09-16)
  session: Ernte-Folge 42
  class: handover
  date: 2026-09-16
  sha256: 180361a2cbd60e3fc44902ef21d09ce2726e51282b035feb86dcbece250820bf
  status: live
-->
# Handover — Ernte-Folge 42 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur
Commits, der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

Folge 42 hat die drei Tor-1-Reste vollständig abgearbeitet: TNF registriert
(Phasen-Zusammensetzung spec-gemessen), CORS registriert (Membran-Leser
gebaut), Voyager ODR als Roh-Pack registriert (Jahr-Quelle gemessen); dazu das
CI-Format-Gate der ernte-eigenen Compiler gefixt.

## Voyager ODR/RSS — registriert; Reihen-Leser offen

- Registriert (`format voyager_odr`, force-0/`count`, Roh-Pack `VODR` via
  `voyager_odr_compiler`, `pds-ppi.igpp.ucla.edu/voyager_odr.bin`); das Jahr
  kommt je Datei aus dem PDS3-Label `START_TIME` (gemessen C0XR13AA.LBL =
  1981-08-26T04:05:00.000) bzw. `INDEX/INDEX.TAB`. Offen: der Reihen-Leser-Arm
  (`extract.rs` `series_parse_bin`, Epoche aus Jahr + BCD-Tag) — Post an bau;
  die `sample_count`-Semantik (Spec ≤ 299999 vs. gemessene Record-1-Werte
  ~4,29e9; Offset 52 bestätigt); die Serie über die 968 RSS-Dateien. (Schritt:
  Post an bau.)

## TNF — registriert; CI-Fetch offen

- Vollständig registriert (`phi/sources.φ`: `format csv` + `rows .` +
  `epoch epoch` + `field ramp_freq … Hz` + `field ul_phase_cycles
  nh_rex_tnf_uplink_phase_cycle … cycle`). Die Zusammensetzung ist spec-gemessen
  (DSN TRK-2-34 Rev O, Note 8: `PHASE = hi·2^32 + lo + frac·2^-32`, cycles); der
  Compiler emittiert die kombinierte `ul_phase_cycles`-Spalte. `nh-rex-tnf-cdn.yml`
  steht. Offen: der Workflow-Lauf nach Push + Consent ist die
  pdssbn-Erreichbarkeits-Messung (lokales DNS void, gemessen). (Schritt:
  `gh workflow run nh-rex-tnf-cdn.yml`.)

## CORS — registriert; SNR + CDN offen

- Registriert (`phi/sources.φ`: `format cors_rinex`, `cors_1lsu_2024001.bin`,
  8 Felder C1/P1/C2/P2/C5 [m] + L1/L2/L5 [cycle]); der Membran-Leser
  `cors::parse_series` + `extract.rs`/`main_flow.rs`-Arme stehen;
  `cors-cdn.yml` ruft `cors_rinex_compiler --lsk`. Offen: die SNR-Felder
  S1/S2/S5 brauchen die `dbhz`-Einheit in der Registry (`convert_to_si` +
  `allowed_units_for_force(0)` + Test); die CDN-Manifestation nach Push +
  Consent. (Schritt: `dbhz` registrieren, `gh workflow run cors-cdn.yml`.)

## CI-Gate (gemessen @625452e5 — Eintrag `docs/zustand/external-state.md`)

- Die 5 ernte-eigenen harvest-Compiler sind rustfmt-formatiert. Offen: die
  fremden fmt-Diffs (`src/gate/commit_gate.rs:540`,
  `aia_ladder_probe`/`trishuli_gauge_probe`, `archive_search.rs`) — Post an die
  Linien; der `test`-Job `cancelled/unvermessen`. Lokale fmt-Läufe strukturell
  verweigert. (Schritt: rustfmt-Diff aus ci-check run 35091175017.)
- Fremder Compile-Bruch im geteilten Baum: `tools/harvest/src/bin/drs_fits_compiler.rs:93`
  (`{epoch:.3f}` = ungültiger Format-Trait) blockt `cargo check -p
  omegaflow-harvest` — DRS-FITS-Linie, nicht angefasst. (Schritt: die
  DRS-FITS-Linie fixt ihre Datei.)

## CDN-Manifestation (gated — nach Push + Consent)

- `nh-rex-tnf-cdn.yml`, `voyager-odr-cdn.yml` (neu), `cors-cdn.yml` (geändert)
  → `auto-dispatch.yml`; `juno_odf` via `planetary-odf-cdn.yml`; folge40:
  `vlass-tap`/`noaa-cdo`/`gedi`/`icesat2`/`swot`-cdn.
- Unveränderte Ziele manuell: celestrak-eop, goes, himawari, gk2a, uscrn,
  cosmic, maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod,
  vlass, eri + `source-census.yml`; Gaia DR3 XP Voll-Survey
  `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`.

## Sensor-Welle — offene Re-Checks (gemessen 2026-09-16)

- `erddap.emso.eu`-Index: lebendes EMSO-Dataset (OBSEA eingefroren 2026-05-12).
- `mercator.env.nm.gov` AQI Table 1: `max(DATE_TIME)` erneut messen.
- `erddap.emodnet-physics.eu HFRADAR_NADR_Totals`: `maxTime` erneut messen.
- SondeHub-`serial` / IOOS-Glider: Kadenz-Re-Check.
- IGRA-2 bleibt `blocked parser-def zip/fixed-width-text`.

## Ledger — offene Routen (gemessen 2026-09-16)

- `dachs.fai.kz/tap`, `vo.lmd.jussieu.fr/tap`, `pithia.cbk.waw.pl/tap`:
  `sync`-QUERY erneut.
- `limadou.ssdc.asi.it`: PI-Freigabe (Sotgiu) = per-Akt-Consent (Operator).

## Tor 1 — Konsumenten (Operator)

- ERI/VLASS/CORS-Konsument; LASzip-Decoder steht, Konsument fehlt; JVO
  skynode-TAP; Babamul; GHRC-DAAC; WFAU VSA/WSA. (Schritt: Operator.)

## Parser-Magic — Gaps

- Gap 1 + Gap 8 (`Frame::Data` in `types.rs` + `flush!()`-Gate); Gap 12
  (Category/Group-Vererbung). (Schritt: Konsument — Operator.)

## Ernte-Nachlauf (gemessen 2026-09-15)

- GES-DISC OAuth `client_id`; MPC-Shard UnnObs; GHRC-DAAC; NOIRLab Gaia DR4
  ≥ Dez 2026; Survey §1 (26 Pendings). (Schritt: Operator bzw. `client_id`.)

## Register-Digest-Überführung (Rest)

- Fink/ALeRCE (Proxy-Pfad); Lasair (Wiedervorlage 2026-09-18); Hinson 1997;
  S3-OAuth (`client_id`), TDAT/FITS-Konsument; INTERMAGNET/IONEX
  (`client_id`)/GIC; Akteure; Daten-Holdings (erste Messung); Orphan-Verdicts.
  (Schritt: Operator bzw. erste Messung.)

## Post von forschung — Quellen-Inventur (Forschung-Folge 31, gefaltet)

- Offene Quellen zum Registrieren (`sources.φ` + CDN): Juno ODF (PDS
  `JUNO-J-RSS-1-OCRU`/`JUGR`), NH REX Pluto (`NH-P-REX-2-PLUTO-V1.0`), Cassini
  PDS RSS raw, Wohlmuth 1997 Wayback-PDF (`97-0783.pdf`), Super-K
  (`zenodo.org/records/8401262`), Telescope Array (`10.5281/zenodo.8427755`),
  LHAASO (`english.ihep.cas.cn/lhaaso/pdl/`), KASCADE-Grande
  (`kcdc.ikp.kit.edu`), H.E.S.S. (`hess_dl3_dr1.tar`), MAGIC
  (`opendata.magic.pic.es`), VERITAS
  (`github.com/VERITAS-Observatory/VERITAS-VTSCat`), HF-Radar
  (`hfradar.ioos.us`), BGC-Argo Sprof NetCDF, Lasair/SuperDARN. Reader-Bauten:
  LASzip/COPC (NOAA NOS, USGS 3DEP), NetCDF (COSMIC, HF-Radar, BGC-Argo), FITS
  NAXIS=0/TUNIT, FLAC (NRS), tar+FITS (H.E.S.S.). (Schritt: je Quelle
  `sources.φ`-Block + Compiler, Muster `geo.rs`.)

## Baum

- Fremd-uncommittet (nicht angefasst): DRS-FITS-Arbeit
  (`src/archivar/{fits,mod,zeuge,drs_fits}.rs`,
  `tools/harvest/src/bin/drs_fits_compiler.rs`),
  `docs/paper/twenty-second-band-ground-chain.md`,
  `docs/handover/handover-2026-09-16-entscheid-folge21.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
