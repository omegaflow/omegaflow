<!--
  title: Handover — Ernte-Folge 41 (Stand 2026-09-16)
  session: Ernte-Folge 41
  class: handover
  date: 2026-09-16
  sha256: a9f0f1cf8644f0cb1c1f7de82c3d9bbaee0a2c3504087c1eb15778caa22e3f4d
  status: live
-->
# Handover — Ernte-Folge 41 (2026-09-16)

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

## Tor-1-Register — gebaute Harvests registriert, drei Reste

Folge 41 hat das Entscheid-Verdikt ausgeführt (Tor 1: der Konsument ist kein
Kriterium; ein gebauter Harvest wird in `sources.φ` + CDN registriert). Die
ODF-Familie ist registriert (`juno_odf`, `magellan_odf`, `mgs_odf`, `mro_odf`,
`odyssey_odf`, `messenger_odf`, `mars_express_odf`, `rosetta_odf` — Muster
`vex_odf`: `at earth`, `field observable_hz <name>_observable_hz inverse-square
em Hz 604800 0.0 0.0`), die stale pendings (Dawn, Voyager-Saturn) sind aus
`blocked_sources.φ` gestrichen, `juno_odf_compiler` steht in der
`planetary-odf-cdn.yml`-Matrix. Die Verdikte CSES-SPA (`leos.ac.cn`),
Neutrino-JUNO, TA, HAWC, LHAASO sind im Register gemessen — **kein Eintrag war
zu streichen**: CSES-SPA und Neutrino-JUNO existieren nicht; Telescope-Array
(Kosmik) ist in `declined_sources.φ` bereits als Einzelereignis abgelehnt;
HAWC-Zugriff via CA-Bundle ist in `witnesses.φ` notiert; LHAASO trägt nur CASDC.
Offen:

- **TNF**: `tnf_compiler` gebaut (SFDU+DT0 → CSV, netloc
  `pds-smallbodies.astro.umd.edu`); offen ist der exakte `.tnf`-Datei-URL
  (voller Pfad von `lunocc2012.tnf` unter dem holdings-Root — gemessen ist nur
  der Dateiname) + die CSV-Register-Form (`epoch` aus year+doy+sec) + der
  CDN-Workflow. (Schritt: holdings-Verzeichnis lesen → Pfad; dann `sources.φ`-Block
  + `nh-rex-tnf-cdn.yml`.)
- **CORS**: RINEX-2.11/Hatanaka-Parser + `cors-cdn.yml` stehen; das Asset ist je
  Stations-Tag, kein Einzel-URL. (Schritt: Serien-Aggregation über Stations-Tage
  oder Wahl eines Stations-Assets.)
- **Voyager ODR/RSS**: Reader steht (`src/archivar/voyager_odr.rs`), kein
  Compiler → kein Asset. (Schritt: `voyager_odr_compiler` bauen, `.bin`
  manifestieren.)

## Bau-Reihenfolge — Membran-Leser (Post an bau)

- Die registrierten Serien-Formate sind ohne Membran-Leser: `extract.rs`
  `series_parse_bin`/`series_component_name` + die `main_flow.rs`-`matches!`-Liste
  haben keinen Arm für `juno_odf`, `magellan_odf`, `mgs_odf`, `mro_odf`,
  `odyssey_odf`, `messenger_odf`, `mars_express_odf`, `rosetta_odf`,
  `voyager_odr`, `cors_rinex`, `tnf`; die schon registrierten `vex_odf`,
  `galileo_odf`, `dawn_odf`, `galileo_odr` sind ebenso inert. (Schritt: Leser je
  Format + Field-Key-Mapping, Muster `geo.rs`/`voyager_saturn`.)
- Die sechs Compiler-Formate ohne Leser: VLST/VLCT, NCDO, GED1, AT31, SWS1.
  (Schritt: Reader je Format, Muster `geo.rs`.)

## CDN-Manifestation (gated — nach Push + Consent)

- `juno_odf` via `planetary-odf-cdn.yml` (in diesem Push geändert →
  `auto-dispatch.yml` dispatcht selbst).
- folge40: `vlass-tap-cdn.yml`, `noaa-cdo-cdn.yml`, `gedi-cdn.yml`,
  `icesat2-cdn.yml`, `swot-cdn.yml` (auto-dispatched im folge40-Push). (Schritt:
  `gh run view` der Runs.)
- Unveränderte Ziele manuell: celestrak-eop, goes, himawari, gk2a, uscrn, cosmic,
  maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass,
  eri + `source-census.yml`. (Schritt: `gh workflow run <wf>` nach Push + Consent.)
- celestrak-eop: fremd-staged; nicht angefasst.
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`. (Schritt:
  `gh workflow run` nach Push + Consent.)

## Sensor-Welle — offene Re-Checks (gemessen 2026-09-16)

- `erddap.emso.eu`-Index: lebendes EMSO-Dataset wählen (OBSEA eingefroren
  2026-05-12).
- `mercator.env.nm.gov` AQI Table 1: `max(DATE_TIME)` erneut messen.
- `erddap.emodnet-physics.eu HFRADAR_NADR_Totals`: `maxTime` erneut messen.
- SondeHub-`serial` / IOOS-Glider: Kadenz-Re-Check-Duty.
- IGRA-2 bleibt `blocked parser-def zip/fixed-width-text`.

## Ledger — offene Routen (gemessen 2026-09-16)

- `dachs.fai.kz/tap`, `vo.lmd.jussieu.fr/tap`, `pithia.cbk.waw.pl/tap`:
  `sync`-QUERY erneut.
- `limadou.ssdc.asi.it`: PI-Freigabe (Sotgiu); Nachfassen = per-Akt-Consent
  (Operator).

## Tor 1 — Konsumenten (Operator)

- ERI/VLASS/CORS-Konsument; LASzip-Decoder steht, Konsument fehlt; JVO
  skynode-TAP; Babamul (Zugang steht, Harvest fehlt); GHRC-DAAC Produkt +
  Konsument; WFAU VSA/WSA `superseded-by-integrated`. (Schritt: Operator.)

## Parser-Magic — Gaps

- Gap 1 + Gap 8 (`Frame::Data` in `types.rs` + `flush!()`-Gate); Gap 12
  (Category/Group-Vererbung). (Schritt: Konsument — Operator.)

## Ernte-Nachlauf (gemessen 2026-09-15)

- GES-DISC OAuth `client_id`; MPC-Shard UnnObs; GHRC-DAAC; NOIRLab Gaia DR4
  ≥ Dez 2026 (Wiedervorlage schweigt); Survey §1 (26 Pendings). (Schritt:
  Operator bzw. `client_id`.)

## Register-Digest-Überführung (Rest)

- LAIC-Bausteine gemessen (CSES-SPA gestrichen); JUNO/Neutrino-JUNO (kein
  Eintrag); TA (USArray-TA-FDSN = Messung, Telescope-Array-Kosmik declined);
  Fink/ALeRCE (Proxy-Pfad); Lasair (Wiedervorlage 2026-09-18); Hinson 1997
  (Registereintrag oder descope); S3-OAuth (`client_id`), TDAT/FITS-Konsument;
  INTERMAGNET/IONEX (`client_id`)/GIC; Akteure (Erdmoden/Radon — keine Quelle
  benannt); Daten-Holdings (erste Messung); Orphan-Verdicts (repo_tag-CDN-
  Bereinigung). (Schritt: Operator bzw. erste Messung.)

## Baum

- Fremd-uncommittet: `docs/paper/twenty-second-band-ground-chain.md` — nicht
  angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
