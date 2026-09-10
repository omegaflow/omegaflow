<!--
  title: Handover — Probe-Unit-Fix + Copernicus-Litmus (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: 50d09c30056872399b2cbcc4bdb5335ac0a5b30179d3c55678a1c13c46e6e25c
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Probe-Unit-Fix + Copernicus-Litmus (2026-09-10)

## Erledigt (trägt Git)

- **Probe-Einheit-Autoableitung gebaut** — `hapi_draft_fields`
  (src/archivar/port.rs) schreibt die Server-Unit verbatim in die Feld-Zeile
  und benennt den Konflikt plus die Register-Unit im Kommentar
  (`# unit kg/m3 not in force registry — register carries m/s`); die
  Guess-Unit bleibt verworfen (Heuristik ≠ Messung). Sample-Druck = erste
  finite Zeile im Fenster (Fill-Zeilen benannt). Ursache gemessen: der
  VirES-HAPI-Server trägt für `crosswind` selbst `units kg/m3` (ESA-Handbuch
  m/s) — die Probe kopierte die Server-Metadaten treu; das Register trägt m/s
  (sources.φ:43). Zwei Tests in src/archivar/tests.rs
  (`register_hapi_units_of_maps_short_parameter_to_field_unit`,
  `hapi_draft_names_register_unit_when_server_unit_is_off_registry`).
- **Copernicus-Inventar litmus-fähig** — copernicus_catalogues.φ re-harvestet
  (catalogue API v1, saubere id+title-Paare; 16 CAMS + 143 C3S). Der frühere
  173-C3S-Stand ist nicht in der aktuellen Antwort (nicht re-verifiziert). Der
  alte Stand trug misalignierte title/id-Paare + „Quality assessment"-Rauschen.
- **Bucket-Litmus + Disposition** — Vorentscheid (bucket_litmus) + Review
  (Fragen 1–4) über 159 Collections: 8 compiler-lease (insitu-observations-*,
  In-situ-Messarchive, Compiler pending) + 151 descoped (Q1 nein — Modell/
  abgeleitet/Satelliten-Retrieval/Katalog/Service). Register:
  phi/pipeline/catalog/copernicus_disposition.φ.
- **Kalibrier-Gate gemessen** — gegen die 159 Urteile: FP 1 (igra — „archive"
  matcht, die Messung ist echt), FN 27 vor dem token / 26 danach.
  „projection"/„projections" in decline_lens.φ model-forecast aufgenommen
  (räumt 1 cmip6-FN; die 4 cmip5-Titel tragen das Wort nicht und bleiben FN;
  noaa-Baseline unverändert). Gezählt, nie geglättet.
- **Ledger** — `verifiziert kandidat copernicus-litmus`, `ausstehend compiler
  copernicus-8-leases`, `verifiziert parser-gap ProbeUnitAutoableitung`.

## Offen (Register-Duty, benannt)

- **8 Copernicus-Compiler-Lease** — insitu-observations-* (ICOADS, surface-land,
  US-CRN, IGRA, WOUDC, GRUAN, CUON, GNSS-Delay); je Lease Manifestations-Duty
  CDN via --ci-mode. Kein Compiler gebaut.
- **19 noaa-nodd-Compiler-Lease** — unverändert offen (ledger.φ).

## Beobachtet (nicht diese Session, uncommitted im Worktree)

- **Konkurrierende Linie baut noaa_nodd.rs** — untracked (src/archivar/noaa_nodd.rs
  + `pub mod noaa_nodd;` in mod.rs), trägt 5 Compiler-Fehler (`parse_isd` mit `?`
  in Vec-Funktion, GeoRec-Feld `station` fehlt, Import `COMP_GSOD_MIN`).
  `cargo check` schlug zum Zeitpunkt meiner Edits 0/0 an; jetzt fällt es auf
  diese fremden Fehler — keiner in port.rs/tests.rs. Nicht angefasst.
- tools/service/src/assets/job_dashboard.html, tools/vo-tap/src/lib.rs u.a. —
  unangetastet (andere Linien).
