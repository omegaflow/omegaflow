<!--
  title: Befund — Richtungs-Atom: die Himmelsrichtung gehalten, nicht erfunden
  class: befund
  date: 2026-09-06
  sha256: 1f3bb62a6801caa0b1478a3a9fa59a5b7acbccfadc07e1bd2f5ceb3897a82b95
  status: done
  antwortet-auf: docs/auftrag/auftrag-richtungs-transient-atom.md
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/docs-naming.md docs/TODO.md phi/blocked_sources.phi phi/dead_sources.phi
-->

# Befund — SkyDirection: die Himmelsrichtung gehalten, nicht erfunden

Der Auftrag (docs/auftrag/auftrag-richtungs-transient-atom.md, Rat 2026-09-05)
beauftragte: eine ra/dec-Richtung ohne Distanz ist ein Punkt auf der
Einheits-Himmelskugel S², kein Ort im ℝ³-Voxel — sie wird als eigene,
eigenbenannte Archivar-Entität gehalten, nicht erfinden, nicht in die toten
Wire-Slots umbiegen, nicht still verwerfen. Der Rat hat den Ort entschieden:
eine neue Archivar-Entität + ein neuer Harvest-Compiler. Dieser Befund schließt
den Auftrag.

## Was gebaut ist

- `src/archivar/skydirection.rs` (neu): die Entität `SkyDirection`. Echte
  Feldnamen `name`, `ra_deg`, `dec_deg` (Grad, ICRS), eine band-strukturierte
  Magnituden-Serie (`bands: Vec<SkyBandSeries>`, jede Serie mit `band:
  Option<String>` — None, wo die Quelle keinen Passband-Namen liefert, und
  `samples: Vec<SkySample>` mit `tdb` (Sekunden seit J2000, TDB-Skala) und
  `mag`), sowie `distance: Option<f64>` und `redshift: Option<f64>` — beide
  None, absent, nie ein 0.0-Sentinel. `unit_direction()` = `[cos(dec)·cos(ra),
  cos(dec)·sin(ra), sin(dec)]` (ra/dec in Radiant), dieselbe Formel, die in
  extract.rs ~2133 und spatial.rs ~287 lebt; p̂ ist eine Query-Eigenschaft, keine
  gespeicherte Redundanz. Serialisierung als eigenes SKD1-Asset
  (`write_bin`/`parse_bin`, Muster ztf.rs/ir.rs/radio.rs); Option-Distanz/-Redshift
  als Präsenz-Flag + Wert (absent = Flag 0, nie 0.0). Die Entität schreibt nichts
  in den 26×f64-Positions-Wire — kein Flag, keine Umwidmung, kein Hack.
- `tools/harvest/src/bin/skydirection_compiler.rs` (neu): der Richtungs-Harvest.
  Quellen-Flags `--lasair-window <jd_start>`, `--antares [--antares-limit N]`,
  `--fink-cone <ra> <dec> <radius-arcsec>` (wiederholbar), `--alerce` (Probe),
  `--ci-mode` gated den CDN-Upload (Muster bayestar_compiler/twomass_compiler);
  ohne `--ci-mode` nur lokaler Build. Name = Implementation: Entität
  `SkyDirection` ↔ Compiler `skydirection_compiler`.
- Register: `src/archivar/mod.rs` (`pub mod skydirection;`, alphabetisch),
  `src/lib.rs` (`pub use archivar::skydirection;`), `phi/blocked_sources.φ`
  (Lasair/ANTARES/Fink-LSST als „gehalten via skydirection_compiler,
  positions-pending" benannt), `phi/dead_sources.φ` (ALeRCE-404-Stub um den
  Harvest-Status ergänzt), `docs/TODO.md` (Zustand „Richtung
  geerntet-und-gehalten, positions-pending").

## Endpoint-Befunde der vier Quellen (gemessen 2026-09-06, curl)

- Lasair-ZTF `api/query/` (objects-Window): erreichbar. Mit `LASAIR_TOKEN`
  (Header `Authorization: Token`) HTTP 200; anonym HTTP 401 („Authentication
  credentials were not provided."). Antwort: JSON-Array je Objekt mit
  `objectId, ramean, decmean, gmag, jdmin, jdmax`. Gemessener Lauf
  (jdmax>2460500): 1000 Objekt-Zeilen; 942 tragen die gelieferte gmag mit
  Einzel-Detektions-Epoche (jdmin==jdmax), 58 tragen eine gmag ohne ankerbare
  Epoche (Multi-Detektions-Objekt, die g-Band-Epoche wird nicht geliefert) —
  diese 58 Magnituden bleiben ungehalten (0 honored), die Richtung wird
  gehalten.
- ANTARES `api.antares.noirlab.edu/v1/loci`: erreichbar, anonym HTTP 200.
  Antwort: JSON:API-Objekt; Loci mit `id`, `attributes.ra`, `attributes.dec`,
  `attributes.properties.{newest,brightest,oldest}_alert_magnitude` +
  `..._observation_time` (MJD). Gemessener Lauf (limit 20): 10 Loci, 11
  Summary-Magnituden-Proben, unbanded gehalten (die Loci-Auflistung liefert
  keinen Passband-Namen; die ZTF-Band-Zuordnung wäre geraten, also absent).
- Fink-LSST `api.lsst.fink-portal.org/api/v1/conesearch`: erreichbar, anonym
  HTTP 200 (POST ra/dec/radius). Antwort: JSON-Array mit `r:diaObjectId`,
  `r:ra`, `r:dec`, `r:nDiaSources`, `r:midpointMjdTai`. Die Konus-Zeilen tragen
  KEINE em-Photometrie (gemessen: auch eine angefragte `r:magpsf`-Spalte wird
  nicht geliefert; die Lichtkurven-Endpunkte /sources und /fp liefern Flux,
  keine Magnitude) — der Konus wird als Richtung gehalten, Magnituden absent,
  die Per-Objekt-Lichtkurven ein benanntes Pending. Gemessener Lauf (Konus bei
  ra 148.8746 dec 2.5208, 60 arcsec): 61 diaObject-Richtungen.
- ALeRCE `api.alerce.online/objects`: NICHT erreichbar, HTTP 404, 0 Byte —
  der Retired-Direktdatenbank-Stub (dead_sources.φ: „Direct database access is
  being retired"). Der Compiler misst und benennt das (--alerce), erzeugt keine
  Records, nie ein Negativ, keine Fabrication.

Lokaler Asset-Lauf (ohne --ci-mode, nach /tmp/opencode/skydir/sky_directions.bin):
1071 Richtungs-Records (1000 Lasair + 10 ANTARES + 61 Fink), 952 Band-Serien,
64 878 Byte, Roundtrip liest zurück. ALeRCE steuert 0 bei, ehrlich benannt.

## 0 honored — wo nichts erfunden wird

- `distance`/`redshift` bleiben None (absent) in jeder gehaltenen Richtung —
  die Distanz kommt NUR durch den Crossmatch (direction_distance_join.rs,
  Gaia-Parallaxe, Redshift), nie erraten, kein Referenzradius.
- Magnituden ohne ankerbare Epoche (Lasair Multi-Detektions-gmag) bleiben
  ungehalten. Magnituden ohne gelieferten Passband (ANTARES-Summary) werden
  unbanded (Option None) gehalten, nie einer Bande zugeschlagen.
- Quellen ohne Photometrie (Fink-Konus) liefern Richtung ohne Magnitude; die
  Per-Objekt-Lichtkurven sind ein benanntes Pending, kein stiller Null.
- Unerreichbare Quellen (ALeRCE 404) werden als unreachable benannt, nie als
  „nicht vorhanden" gewertet.
- Der 26×f64-Positions-Wire, `extract.rs` `no_distance_skipped`, `ztf.rs` und
  die CelestialMap-Distanz-Gate bleiben unverändert.
- Die sieben verbotenen Marker (`unwrap_or(0.0)`, `unwrap_or(0)`,
  `unwrap_or_else(`, `unwrap_or_default(`, `#[derive(Default)]`, `_ => 0`,
  `.max(1)`) sind in beiden neuen .rs-Dateien 0 Treffer (gemessen).

## Verifikation

- `cargo check -p omegaflow`: 0 Fehler, 0 Warnungen.
- `cargo check -p omegaflow-harvest --bin skydirection_compiler`: 0 Fehler,
  0 Warnungen.
- `cargo test -p omegaflow skydirection`: 7 Tests, alle grün (unit_direction,
  SKD1-Roundtrip inklusive absent-distanz = None und gelieferter Distanz,
  Multi-Band-Serien, Ablehnung nicht-endlicher Werte und fehlgeformter Assets,
  leeres Asset als gültiger gehaltener Zustand).
- Der Compiler lief gegen die vier echten Endpunkte (gemessen, oben).

## Benannte Pendings (Register, keine Schuld)

- Fink-Per-Objekt-Lichtkurven (Flux-Serien aus /sources, /fp) als Magnituden-
  bzw. Flux-Rohmaterial — Konus-Zeilen tragen keine Photometrie.
- CDN-Manifestations-Route für das Richtungs-Asset (cdn-Workflow nach dem
  Muster bayestar-cdn.yml, der den Compiler mit `--ci-mode` und der echten
  Window-/Konus-Wahl fährt) — ohne sie steht der Bestand nur lokal
  (docs/TODO.md).
- S²-Winkel-Kernel (Winkel-Kernel, Winkel-Residuum) bleibt nach Rats-Verdikt
  deferriert — ein benanntes Zukunfts-Atom, kein aktives Pending.
