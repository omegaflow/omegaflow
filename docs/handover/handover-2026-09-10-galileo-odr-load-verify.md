<!--
  title: Handover — Galileo-ODR-Load-Verify: die count-Einheit ins em-Register, der Block lädt ohne Anomalie
  class: handover
  date: 2026-09-10
  sha256: 72a328eecae0e717f2b617f756549eb64a1c62881a95498a540a420156d206cf
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Galileo-ODR-Load-Verify (2026-09-10)

## Angenommen

- `handover-2026-09-10-galileo-odr-repair.md` — Offen-Punkt 1: die
  sources.φ-Load-Verifikation des `galileo_odr`-Blocks stand aus.

## Geleistet

- Ladeprüfung gemessen: der Block lädt (force em=0, kernel inverse-square=0,
  tau 604800, 4 Felder ad1..ad4), aber die Einheit `count` fehlte im
  em-Register — `report_physics_mismatch` meldete beim Laden eine
  Physics-Mismatch-Anomalie. Die Reparatur-Notiz „em count neben em cpm/em 1"
  war ungemessen: das Einheiten-Register ist fest, kein wachsendes.
- `src/archivar/units.rs`: `count` in `allowed_units_for_force(0)` (em, neben
  `cpm`) und in `convert_to_si` als dimensionslos (identity, neben `1`)
  eingetragen.
- `src/archivar/tests.rs`: Test
  `galileo_odr_register_field_matches_component_name` (4 Felder
  galileo_odr_ad1..ad4_count, force 0, kernel 0, unit count) + zwei
  Verstärkungs-Assertions in `test_allowed_units_for_force` — der Fix ist
  gepinnt, nicht nur die Parse-Struktur.
- `cargo check` 0 Warnungen; 3 Tests grün.

## Offen

- **`galileo_odr`-Format-Reader** — unverändert offen (origin-verbatim, das
  ODR-Sample-Layout AD1..AD4 ist ungedeutet). Folge-Atom.
- **880 Folgebytes von `70580900.ODR`** — erhalten, ungedeutet.
- **`at earth`-Anker im galileo_odr-Block** — Council-Befund: gegen den
  Record-Inhalt ungeprüft (trägt c118073); pending, kein Blocker dieses Atoms.
- Die fünf übrigen Galileo-Floor-Atome stehen im Autonom-Handover.

## Archiv

- `handover-2026-09-10-galileo-odr-repair.md` → `docs/handover/archiv/`
  (Offen-Punkt 1 hier gearbeitet; 2–4 weitergetragen).
