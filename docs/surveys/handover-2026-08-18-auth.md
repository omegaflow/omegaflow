# Handover: AUTH-Bereinigung, Frost/Lasair/Feuerkugeln live, ci_mode-Drei-Klassen

Selbsttragende Karte für eine frische Session. Stand: Branch `main`,
HEAD = `0de82c0` (Recheck b4 in Registern + GitHub-Secrets übertragen).

## Erster Schritt der neuen Session

```bash
cd /home/johannes/projects/omegaflow
git status                      # muss leer sein
cargo check                     # muss 0/0 sein (0 Fehler, 0 Warnungen)
cargo test --bin omegaflow      # 71 Tests grün
cargo test --lib                # 42 Tests grün
```

**Lesepflicht zu Beginn:** `AGENTS.md` (die primäre Constraint-Matrix),
`TODO.md` (kanonisch — offene Arbeit), `docs/SOURCE_PORT.md` (der eine
Source-Pfad), `docs/plans/AUTH_APIS.md` §E (Auth-Register),
`docs/surveys/handover-atome.md` (die Atom-Karte).

---

## Was diese Session getan hat (2026-08-18, AUTH/Source-Port/ci_mode-Linie)

### 1. Auth-Bereinigung — komplett

Das gesamte Credential-Feld ist durchgearbeitet. Zustand in `docs/plans/
AUTH_APIS.md` §E (Tabelle + §E.1 „integriert") und `.secrets.local` (gitignored):

- **Integriert live in `phi/sources.φ`:** frost.met.no (FROST_BASIC_AUTH),
  lasair-ztf.lsst.ac.uk (LASAIR_TOKEN), ssd-api.jpl.nasa.gov fireball.api
  (keyless), services.arcgis.com GOES_GLM_Bolides (keyless).
- **Besorgt, Route ausstehend:** MAST_TOKEN (TIC-Katalog — eigener Compiler-
  Atom), ICOS_USER/PASS + CEDA_USER/PASS (Bulk-Archive, Harvester fehlt),
  IRSA_USER/PASS (decline redundant für ZTF, S3 offen).
- **Refused:** gracedb.ligo.org (LVC/MOU-Gruppenmitgliedschaft, kein
  Self-Service; public superevents offen in sources.φ).
- **Pending:** toar-data.fz-juelich.de (Registrierung offen — Operator-Aktion).
- **Decline no-physical-force:** networkrail, marinecadastre, nass,
  zooniverse/whale.fm, antares. **Decline redundant:** irsa-ZTF.
- **SuperMAG:** Username `omegaflow` registriert (logon-only, kein Passwort) —
  API wirft Server-Fault (`db-get`-Binary nicht ausführbar), ihr Server-Problem.

GitHub-Secrets (`omegaflow/omegaflow`) sind mit `.secrets.local` synchron —
gesetzt: FROST_BASIC_AUTH, TNS_UA, LASAIR_TOKEN, MAST_TOKEN, ICOS_USER/PASS,
CEDA_USER/PASS, IRSA_USER/PASS. Der Token in `.secrets.local` hat `repo,
workflow` Scopes (kein `read:org` → `gh auth login` schlägt fehl, aber
`gh secret set` über `GH_TOKEN`-Env funktioniert).

### 2. Neue Feldquellen in `phi/sources.φ` (alle live verifiziert)

- **frost.met.no** (Z.617–640): air_temperature (thermal) + wind_speed
  (advective), Stations-Fanout 40, Basic-Auth-Header `{FROST_BASIC_AUTH}`.
- **lasair-ztf.lsst.ac.uk** (Z.641–648): ZTF-Transienten (em), `/api/query/`
  SQL-SELECT, token in URL, `flux_from_mag gmag`, 24h-Fenster `{jd_start}`.
  Kein z in `objects` → Himmelssphäre (0 honored). Lokaler Crossmatch gegen
  TNS-z ist als eigener Atom in TODO.md registriert (pending).
- **ssd-api.jpl.nasa.gov fireball.api** (Z.650–661): Feuerkugeln (em),
  `lat_sign`/`lon_sign` (N/S,E/W→Vorzeichen), `epoch 0` (Space-ISO-Format),
  energy = e10j (J·10¹⁰), impact-e = kt_tnt.
- **services.arcgis.com GOES_GLM_Bolides** (Z.663–673): GLM-L2-LCFA-
  Radiant-Energie in J (SI, Faktor 1 — NOAA PUG Vol.5 §5.26), `epoch_scale
  0.001` (ms), `tau_key detected_duration` (Blitzdauer).

**Wichtige physikalische Unterscheidung:** CNEOS-Gesamtenergie (J·10¹⁰) und
GLM-Radiant-Energie (fJ) sind **verschiedene Messgrößen** — kein Umrechnungs-
faktor ohne Bolide-Modell, keine Konvertierung fabriziert. Beide stehen
getrennt als ehrliche Felder.

### 3. ci_mode — Drei-Klassen-CDN-Spiegelung

`src/main.rs` `ci_mode` (`cargo run -- --verify phi`, healthcheck.yml alle 3h):

- **plain** (kein `{`): spiegeln mit Auth-Headern (`render_headers` statt
  leerer Header).
- **secret-in-URL** (`{FIRMS_MAP_KEY}`): live-only, Key fließt nie in den
  Asset-Namen.
- **template** (`{lat}`/`{today}`): Probe am deklarierten Anker in Tag
  `{netloc}-template` (separater Release-Tag — `fetch_one` erreicht ihn
  strukturell nie, Probe ≠ Feld ist durch Bau gesichert).
- **fanout** (`{station}`): Stationsliste spiegeln, Daten-URL an der ersten
  Station proben in `-template`-Tag.
- **skip-statt-fail:** fehlendes Secret = `pending`, nicht `dead`.
- `healthcheck.yml` injiziert alle Auth-Secrets als env.

`ci_probe_render` (Probe-Renderer) löst `{today}/{yesterday}/{week_ago}/
{now}/{hour_ago}/{year}/{jd_now}/{jd_start}/{jd_end}/{lat}/{lon}/bbox` und
`resolve_secret`.

### 4. Neue Extract-Direktiven

- `lat_sign` / `lon_sign` (map-Extract): Vorzeichen aus N/S bzw. E/W-String.
- `epoch_scale` (map-Extract): Zeitstempel-Skalierung (ms→s für ArcGIS).
- `parse_iso_tdb` liest jetzt auch das Space-Format `"2026-08-01 17:43:48"`.
- Einheiten: `e10j` (J·10¹⁰), `kt_tnt` (kt TNT = 4.184e12 J) in
  `convert_to_si` + `allowed_units_for_force` (force em).

### 5. Recheck b4 (grind-flash delegiert)

Mechanischer Source-Recheck, Befunde in `phi/port/stage/recheck_b4.φ` und in
`blocked_sources.φ` / `interesting_domains.φ` gespiegelt. dead: argovis-
Route, sensor.community, geomag.bgs.ac.uk-SAA, ncei-normals, OGLE-Pfad.
parser-gap: gong (FITS), ASAS, aavso-vsx. environment.data.gov.uk bestätigt
(Flood-Monitoring-Fanout existiert bereits in sources.φ).

---

## Offene Arbeit

Siehe `TODO.md` (kanonisch). Die wichtigsten offenen Atome, die diese Session
bewusst NICHT angefasst hat:

- **Membran-scoped Cache** (Archivar, HEALPix-Binning) — TODO Z.9.
- **Lokaler Crossmatch** (Lasair-Sphäre × TNS-z) — TODO Z.17, pending.
- **CI-Chunk-Kompilation** (pastel/wds/mktypes/denis, Rust statt Python) —
  TODO Z.29.
- **Stern-/Asteroiden-Physik** (Okkultationen, Hill, Abplattung, Massen-
  Lücken, NEOWISE-Join, Sternfarbe-Rendering) — TODO Z.38, Handover C.
  Die Daten sind geerntet (gaiadr3-Crossmatch pmra/pmdec/rv + Teff/BPmag/
  RPmag; NEOWISE/AKARI-Durchmesser in `phi/katalog/asteroid_diameters_*.φ`).
- **MAST-TIC-Katalog** — eigener Compiler-Atom, Token unkonsumiert.
- **ArcGIS-Bolides detected_energy** — gelöst (J, GLM-L2-LCFA-Radiant-Energie).

## Doku-Drift

Keine offenen Drift-Stellen aus dieser Session. Alle Register (blocked_sources,
interesting_domains, AUTH_APIS §E, ledger.φ) sind mit dem Ist-Zustand in Deckung.
