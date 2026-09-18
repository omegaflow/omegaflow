<!--
  title: Bindings — dust-maske + bathymetrie-gebco (Prosa-Preregistrierung)
  class: concept
  date: 2026-09-18
  sha256: c1d50f25f34a9b42a354ebf1f7f08250ddb655bb9309e511a72d40bef3822627
  status: live
  see-also: phi/bindings/dust-maske.φ, phi/bindings/bathymetrie-gebco.φ, phi/canon.φ
-->
# Bindings — dust-maske + bathymetrie-gebco

Dies ist das Prosa-Heim der beiden Bindings. Die `phi/bindings/*.φ` tragen nur
noch die Direktiven: die Compiler lesen sie als Existenz-/Preregistrierungs-Gate
(`dust_map_compiler --mask-run --binding <file>` prüft `Path::exists()`, nie den
Text), die Begründung lebt hier. Der `#`-freie Register-Kanon gilt für die `.φ`,
nicht für dieses Konzept-Dokument.

## dust-maske — Planck-DL07-Staub-Vordergrund

Registered 2026-09-05, Council verdict (do not re-litigate). The committed
preregistration the compiler's `--mask-run` gate reads before a science run.

### Binding 1 — column
- AV_RQ (column 3) of COM_CompMap_Dust-DL07-AvMaps_2048_R2.00.fits carries the
  measured reddening value. AV_DL stays unread.
- `--column` selects the FITS BINTABLE column; default 3. The JSON value key is
  the selected TTYPE lowercased (`av_rq`). R_V 3.1, unit mag.

### Binding 2 — geometry
- The map is a single 2D foreground shell carried as `dist`, a screen distance
  D_screen in pc — never the redshift `z` (the cmap parser gates `z` at
  z > 0.0, so `z` 0.0 would place nothing).
- The shell length is physical, not invented: the compiler refuses without
  `--screen-pc <pc>` (the screen distance is never silent).
- D_screen = 205 pc (median 194, 1 sigma 71 pc), MEASURED 2026-09-05 from the
  Bayestar19 (Green+2019, arXiv:1905.02734) dust-column centroid over galactic
  |b| >= 30 deg: D_col = sum(r_bar * dE) / sum(dE) per ray, where dE is the
  positive increase of the cumulative E(B-V) profile between distance samples.
  The previously adopted 300 pc is NOT supported by the measurement (the
  high-latitude dust column centroid is ~200 pc, nearer the lower edge of the
  100-200 pc cavity - 300-400 pc sheet bracket).
  Caveat: a single shell is an approximation, not an exact constant — the
  high-latitude column is compact (~200 pc) but carries ~1 sigma 71 pc scatter
  and real longitude structure (l-sector means 157-237 pc, factor ~1.5). The
  low-latitude sky |b| < 20 is not representable by one screen (band centroids
  ~1.4-1.7 kpc). Source FITS sha256 03f43318f90faed414d9db99dcf34c2217089636
  d4697fa70cc081eb1b5528ac; DM grid mu = 4.0 + 0.125 j, r = 63 pc..59.6 kpc.

### Binding 3 — resolution
- nside 512 by default. Pixel 0.11 deg <= sigma/2 of the GD-1 12 arcmin
  stream width.

### Binding 4 — media-token pin
- FITS source:
  https://irsa.ipac.caltech.edu/data/Planck/release_2/all-sky-maps/maps/component-maps/foregrounds/COM_CompMap_Dust-DL07-AvMaps_2048_R2.00.fits
- sha256 of the downloaded FITS (measured 2026-09-05, dust-cdn run 33965354922):
  90f0ff42e2eced3105281ee70a5a8faef9758851a4faaac01d687d889a750cef

## bathymetrie-gebco — Gestalt-Witness

Registered 2026-09-07 (die-weberin witness kinds, Council 2026-09-07). The
committed preregistration the Gestalt-witness compiler reads before a science
run. A non-radiating measurement declined as oscillator is not declined as
witness: the terrain/bathymetry entries re-labelled in dead_sources.φ are
re-read here as witnesses for shape.

### Binding 1 — source
- opentopodata `gebco2020` (recheck 2026-09-07, HTTP 200): the query
  `?locations=<lat>,<lng>` returns one elevation in meters; negative means
  below sea level (measured −833 m at NRS01 72.49, −156.6).
- Land sibling `eudem25m` (recheck 2026-09-07, HTTP 200) carries the land form
  (measured 286.25 m at 51.2, 10.4). The two together are one Gestalt: the
  solid surface, above and below the water line.

### Binding 2 — field and unit
- Value = surface elevation/depth, unit meter (SI). It is a measured scalar.
  A bare coordinate without a value stays dropped — a witness carries its
  measurement, never an empty pointing.

### Binding 3 — geometry and holding
- A body-surface scalar field over lat/lon (WGS84/ICRS surface). Held as a
  binding (the dust-maske pattern), never as a force field and never as an
  oscillator; it does not radiate.
- τ = geological stability (long ttl), unlike the τ=0 event witnesses. A DEM
  height changed last in geological time, not yesterday.

### Binding 4 — consumption
- Held through the Station thread (`Motion::Surface`, Archivar): the thread
  first, the projection second. The Gestalt-witness compiler fetches point
  threads (lat/lon/depth records) and manifests them via `--ci-mode`; the
  Station-thread integration into the Archivar (`motion.rs`) is the named
  follow-on, not silently assumed. The Mathematikerin does not hold a
  non-radiating shape — it evaluates force fields; the S² direction witnesses
  live there, the body-surface Gestalt witness lives in the Archivar.

### Binding 5 — media-token pin
- https://api.opentopodata.org/v1/gebco2020?locations=<lat>,<lng> — live,
  recheck 2026-09-07. Land sibling: https://api.opentopodata.org/v1/eudem25m?locations=<lat>,<lng>.
- EMODnet-Bathymetrie-WCS stays dead 404 (recheck 2026-09-07) — not a source
  for this binding.

### Register cross-reference
- The Gestalt-witness identity now also lives in phi/witnesses.φ (witness gestalt, record gbco):
  this binding stays the primary home (Haupt-Heim); the register names the kind.
