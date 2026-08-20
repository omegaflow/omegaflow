# Chunk-Plan: die großen Kataloge in der CI (2026-08-20)

Selbsttragend. Interpretierbar von einer Session mit null Vorkontext. Der Plan
schärft die Daten-Pipeline für die großen Kataloge: CI-Chunking mit den
existierenden tap_compiler-Mechanismen, keine neuen Rust-Features außer einem
Zwei-Zeilen-Bugfix. Schwesterdokument: fischplan-kataloge-2026-08-20.md
(Tabellen/Spalten/Forensik).

## Die fünf Pfeiler der kausalen Wahrheitsfindung

Die kompilierten Felder sind das Blei für die Nadeln. Vier Pfeiler müssen
gespeist werden — fehlt ein Feld, bricht ein Pfeiler:

- **Mitte** (Voxelisierung): absolute 3D-Position — ra, dec, plx/dist.
- **Sensory** (Crossmatch): multiple unabhängige Messungen am selben Ort
  (Gaia-PM + RAVE-RV + …). Ein Sensor allein lügt.
- **River** (TE-Maschine): kinematische Vektoren pm/rv — Phasenraum.
- **Mycelium** (Nullkontrolle): unkorrelierte Kanäle (nicht Katalog-Sache).
- **Future** (Residuum): das rohe val (Flux, z, Teff) — Messung minus Modell.

Ein fehlendes kritischen Feld ist ein gebrochener Pfeiler: pending-Registratur,
kein Upload, „als wäre er vollständig". 0 honored.

## Pfeiler-Matrix der Kompilate

| Kompilat | Mitte | Sensory | River | Future | Farbe (Nadel V) | Pfeiler-Gate (skip-null) |
|---|---|---|---|---|---|---|
| gladep.json — **pending** (Blocker s.u.) | ra/dec/dL (Mpc) | — (Eigenwert) | z | zcmb, Bmag | — | `z` = zcmb — jede Zeile trägt z |
| rave_dr5.json | ra/dec/dist_pc | Gaia-Join | pmra/pmdec + HRV | teff, gmag | bpmag/rpmag geerntet, Manifestation pending | `rv` = HRV (RAVE-Identität) |
| pastel/wds/mktypes/denis | ra/dec/dist_pc | Gaia-Join | pmra/pmdec/rv | teff/mag/jmag | geerntet, Manifestation pending | wie chunk_master.py |
| dr3_stars.bin | ra/dec/plx | — | pmra/pmdec/rv | flux | BP−RP im farbe-Slot (44-B-Record) | star-bin-Struktur |

## Upload-Refusal-Kette (0 honored, strukturell im Compiler)

1. Spalte fehlt am Server → `column absent` → exit 1 → kein Upload.
2. Band/Slice void nach 3 Versuchen → exit 1 → kein Upload (star-bin: Teile
   bleiben als Resume; JSON: Neustart).
3. `--skip-null <pfeiler>` → Zeilen ohne das Pfeiler-Feld erreichen das Asset
   nie (row-level Gate).
4. Chunk-Schleifen (CI-Bash): Vollzähligkeits-Gate vor dem Merge — ein
   fehlender Slice → Issue + Abbruch, kein Partial-Upload.

## Mechanik

- **GLADE+** (`VII/291/gladep`, 22 M): **pending — drei gemessene Blocker.**
  1. Der `--mag-bands`-Bander stößt am `step`-Boden an: unterhalb der
     Schrittweite wird das Band ohne COUNT-Check mit `TOP limit` gepusht
     (tap_compiler.rs) — bei ~180 k Zeilen je 0.25-mag-Band (Peak >1 M)
     würde still gekappt. Dichte-Bänder brauchen RA-Slices oder async.
  2. Ein-Asset ≈ 2.4 GB JSON — über dem 2-GB-Release-Limit (Quadranten-
     Split wäre der Ausweg, löst nicht 3).
  3. `MAX_SAMPLES = 1 << 22` (archivar.rs:9038): der Feld-Rebuild hält
     die jüngsten Samples und wirft die ältesten (epoch 0.0 = Sterne +
     Kataloge) — 22 M Galaxien passen nicht ins Sample-Budget. Der
     z-Gate-Entwurf (`skip-null z`, zcmb) bleibt registriert und gilt für
     das Folge-Atom.
- **RAVE** (`III/279/rave_dr5`): 24 RA-Slices à 15° (`--where` +
  `--crossmatch I/355/gaiadr3` + `--crossmatch-pm pmRA:pmDE:Plx::Teff:
  BPmag:RPmag:Gmag`), rv-Gate `HRV` (live verifiziert — nicht `RV`).
  Der `--async`+JOIN-Weg wurde gemessen verworfen: der UWS-Job hing
  PENDING (2 CI-Läufe >600 s, lokal >3600 s) und UWS-Jobs sind
  IP-gebunden (toter Runner = verwaister Job). Die Gaia-Distanz
  (j.Dist) trägt dist_pc; die spektrophotometrische Dist/plx des
  Katalogs bleibt ungeerntet (abgeleitet, 0 honored). Ergebnis:
  472845 Zeilen.
- **pastel/wds/mktypes/denis**: CI-Replikat von chunk_master.py in Bash —
  RA-Slices mit `--where "t.\"<racol>\" >= lo AND < hi"`, 3 Versuche je
  Slice, `--limit 90000`, `jq -s 'add'`-Merge, `gh release upload
  ssd.jpl.nasa.gov … --clobber --repo omegaflow/sources` (= upload_asset
  aus src/cdn.rs). Slice-Raster: pastel 8×45° (RAdeg), wds 8×45°,
  mktypes 80×4.5°, denis 16×22.5°.

## Verifizierte Fakten (live, 2026-08-20)

- gladep: RAJ2000/DEJ2000 (deg), Bmag (mag), zhelio, zcmb, dL (Mpc) — alle
  vorhanden, Probe mit Zeilen belegt. dist_scale = 3.085677581e22.
- rave_dr5: RAJ2000/DEJ2000, HRV (km/s), TeffK (K) — Probe belegt.
- NED: `https://ned.ipac.caltech.edu/tap/sync` antwortet auf den Standard-
  Sync-Stil; Tabelle `NEDTAP.objdir` (ra, dec, z, prefname, type_key,
  n_spectra). Sync-COUNT → 60-s-Timeout (Server: „use async mode").
- 2MASS `II/246/out`: sync-COUNT > 60 s → der adaptive COUNT-Bander ist
  nicht CI-tragfähig; Bulk-Route (cdsarc-ftp) braucht einen Kompilator.
- MAX_SAMPLES = 1 << 22 (archivar.rs:9038); der Rebuild sortiert nach
  epoch absteigend und wirft die ältesten (15543–15551) — Katalog-Zeilen
  (epoch 0.0) sind die ersten Verlierer der Kappung. Das Budget des
  Gesamtfeldes (Sterne + Asteroiden + NVSS + FIRST + Chandra + vier
  Chunks + …) ist ungemessen.
- Bugfix (Commit 1): der JSON-`--mag-bands`-Pfad schrieb `[` zwischen
  die Bänder (`if !first_row` → `if first_row`, tap_compiler.rs) —
  korruptes JSON für jedes mehrbandige JSON-Kompilat. Der Dry-run
  verifiziert den Fix live gegen gladep.

## Registraturen (pending, keine Fabrikation)

- **Nadel I — GLADE+**: Mechanik + z-Gate entworfen und live verifiziert,
  drei gemessene Blocker (s.o.): Schrittboden-Kappung, 2-GB-Release-Limit,
  MAX_SAMPLES. Eigenes Atom: RA-Slice/async-Mechanik + Quadranten-Assets
  + Sample-Budget-Entscheidung. Kein Upload, als wäre er vollständig.
- **Sample-Budget des Feldes (kritisch, eigenes Atom):** die Summe der
  Katalog-Blöcke liegt über MAX_SAMPLES; welcher Anteil der
  epoch-0.0-Samples den Rebuild überlebt, ist ungemessen. Commit 2
  (sources.φ-Blöcke) steht unter diesem Vorbehalt.
- Nadel V — color_index aus JSON-cmap: Compiler ernten bpmag/rpmag, aber
  nur der star-bin-Pfad manifestiert BP−RP; cmap-Farb-Schlüssel oder
  Compiler-bp_rp-Alias ist ein eigenes Atom.
- Nadel V — Frequenzachse: NVSS/FIRST (1.4 GHz) erreichen freq/bin_width
  nicht (kein Compiler-Flag).
- Nadel I — NED: Root/Spalten verifiziert; async-Slice-Counts messen, dann
  Chunk-Schritt (eigenes Atom).
- 2MASS: Bulk-Kompilator-Atom (COUNT-Timeout-Evidenz registriert).
- Fermi 4FGL: erg/cm²/s-Unit-Arm pending. Drift benannt: der
  chandra_csc-Block trägt erg/cm2, CSC-Fluxb ist physikalisch erg/cm²/s.
- APOGEE: SDSS-CAS-Root pending.

## Commit-Rhythmus

- **Commit 1** (dieser): Bugfix + chunk_catalogs-Job (RAVE + die vier
  Chunk-Kataloge) + dieses Dokument + fischplan-Update + TODO
  (Pfeiler-Registraturen, GLADE+-Blocker, Budget-Registratur). Vorher:
  lokaler Dry-run (2+ Bänder mag-bands auf gladep, jq-Validierung —
  verifiziert den Bugfix; das Kompilat selbst wird nicht hochgeladen) +
  cargo check 0/0.
- **Commit 2** (erledigt 2026-08-20): sources.φ-Block (rave_dr5.json) +
  ledger-Einträge (denis 955434 — 50k-Kappen-Fund; rave 472845 mit
  Feldzählungen) + YAML (RAVE auf RA-Slices, health-Label-Anlage im
  Prepare-Schritt). Verbleibend: ein voller grüner Lauf + die
  MAX_SAMPLES-Budget-Messung.
