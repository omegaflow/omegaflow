<!--
  title: Handover — Mountain-Folge 159 (2026-09-25)
  session: Mountain-Folge 159
  class: handover
  date: 2026-09-25
  sha256: b8af3197acc4a570bf3bfab113d37ff7260d690076523d24e22c53c414a31244
  status: live
-->
# Handover — Mountain-Folge 159 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit.

Diese Session konsumierte `docs/handover/handover-2026-09-25-mountain-folge158.md`
(zum Session-Beginn noch in `docs/handover/`).

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### gap-Disposition der 189 parser-def-Einträge
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25) die Arme sind gebaut — `unit-auto-detect`
  (units.rs/port.rs), `force-undetermined` (astrometrisch/photometrisch → DROP),
  `konverter` (mag_g → em/mag statt nT), `votable-reader` + `html-parser-arm`
  (extract.rs) —, aber `phi/blocked_sources.φ` trägt weiter die 189 `gap`-Direktiven;
  die Quellen sind noch nicht portiert.
- **Blockade:** keine.
- **Braucht:** `register_lookup --orphans` als Nulllinie, dann je Eintrag Port in
  `phi/sources.φ` (mycelium) oder Register-Disposition; `phi/blocked_sources.φ::gap:*`
  fällt erst mit dem Port.

### Verbleibende UNCERTAIN-Feldnamen des Unit-Arms
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `probe_classify`) ~30 Feldnamen bleiben ohne
  deterministische Kraft/Einheit, u. a. `reporting_network`, `event_source`,
  `mag_type`, `spectral_type`, `object_type`, `orbit_class`, `uv_index`,
  `cloud_fraction`, `precipitation_inch`, `mean_temp_f`, `visibility_miles`,
  `wind_speed_kt` (kt=kn vs. Kilotonne), `pressure_dbar`, `distance_mpc`,
  `semi_major_axis_au`, `orbital_eccentricity`, `orbital_inclination_deg`,
  `geometric_albedo`, `uncertainty_arcmin`, `rho_cos_phi`/`rho_sin_phi`,
  `lod`/`dpsi`/`deps`, `dm`.
- **Blockade:** keine.
- **Braucht:** Force-Gate-Urteil je Feld (Einheit + Kraft) oder `descoped` mit
  Messung; die mehrdeutigen (`kt`, `mpc`, `au`) brauchen einen Konverter-Arm oder
  ein Verdikt.

### ALMA TAP (votable-reader) — Register-URL + Distanzachse
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25) der VOTable-Arm ist verdrahtet (`format=json|votable`),
  aber die Register-URL selektiert `ra,dec,source_name,project_code` — real heißen die
  obscore-Spalten `s_ra,s_dec,target_name,proposal_id` (TAP lehnt sonst mit
  `validateColumnNonAlias` ab). obscore trägt außerdem keine Distanzachse.
- **Blockade:** keine.
- **Braucht:** Register-URL korrigieren; Distanz-Achse (dist/plx/z) ergänzen oder
  ein Force-Gate-Verdikt für den kanal-emittierenden Port.

### AEC-FDSN Alaska (html-parser-arm) — Register-Notiz überholt
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `archive_search --playwright`) der HTML-Arm steht,
  aber die `recent_list`-HTML trägt keine lat/lon (nur mag/time/place/depth); die
  Positionen liegen im JSON `aecdynamicfiles.s3.us-west-2.amazonaws.com/recent_events.json`
  (646789 B). Die Register-Notiz „kein JSON-API" ist überholt.
- **Blockade:** keine.
- **Braucht:** Register-Notiz korrigieren; die Quelle als `format json` portieren.

### archive_search Rate-Limit-Arme
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 über `archive_search --all "solar wind"`) `arxiv`
  HTTP 406 (serverseitige Rate-/UA-Politik; kanonisch 200), `openalex` HTTP 429,
  `semanticscholar` 429 (keyless Pool), `librs` 403 Cloudflare (crates.io-Fallback
  trägt); `marginalia` und CDX waren transient, jetzt 200.
- **Blockade:** keine.
- **Braucht:** Retry/Backoff oder Key-Pool (`S2_API_KEY`, OpenAlex polite pool);
  `librs`-Cloudflare über die Browser-Bridge oder einen anderen Host.

#### Stufe 3 — blockiert

keiner. / Stufe 4 — wartend: keiner. / Stufe 5 — termin: keiner. / Stufe 6 — LOCK: keiner.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Diese Session trägt:
`src/archivar/{port,units,tests,extract}.rs`, die `archive_search`-Arme
(net.rs/archive_search.rs/server.rs/web.rs + alphafold/interpro/materialsproject),
diese Übergabe + der Move der konsumierten folge158 ins `archiv/`.
