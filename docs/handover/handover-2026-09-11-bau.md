<!--
  title: Handover — Bau & Code (Stand 2026-09-11)
  class: handover
  date: 2026-09-11
  sha256: fc6833aaa5166077c390eb1817f674ded787d7b56c5e104328c1b4200f82a0bd
  status: live
-->
# Handover — Bau & Code (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand.

## Compiler & Format

- ANR-Wiederanlage in `geo.rs` + `antares_loci_compiler`-Commit (Compiler
  gebaut, der geo-Support fehlt).
- `pack_iaga` für 163 Ziffer-Codes (Wire-Format).
- `galileo_odr`-Format-Reader (AD1..AD4 ungedeutet); 880 Folgebytes von
  `70580900.ODR` erhalten, ungedeutet; `at earth`-Anker gegen den Record-Inhalt
  prüfen (trägt c118073).
- Broker-Compiler für die anonymen Rubin-Broker (ALeRCE/ANTARES/Fink/Babamul);
  DECaPS + VTSS/Mellinger + die 10 Katalog-Kandidaten (eROSITA/Fermi/XMM/GALEX/
  DSS2/Finkbeiner/SDSS9/PanSTARRS/GLIMPSE/SPITZER) disponieren; RSP-Bilder
  brauchen Datenrechte, die Alerts sind offen.

## Reader / Struktur

- Struktur-Reader — Parquet, GRIB-2, OPeNDAP (FITS + netCDF-4 + CDF-1/2).
- OPeNDAP-Integration als Fetch-Format.
- Gaia-XP-Brücke — Ring↔Nest source_id↔FP01-ipix; `xp_pilot_p6144.bin` zum CDN-Asset.
- Membran M02–M07.

## TE-Bau

- TE-Baupunkte — `cycle_phase_shift_surrogate`-Nutzung; bedingte Multi-Force-TE
  (Phasenraum).
