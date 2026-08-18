# Handover — Sternphysik-Ernte (2026-08-18)

Selbsttragend. Interpretierbar von einer Session mit null Vorkontext.
Einstiegspunkte: `AGENTS.md` (Constraint-Matrix, bindend), `TODO.md`
(das vollständige Register der offenen Arbeit), `docs/SOURCE_PORT.md`
(Source-Arbeit). Repo: `/home/johannes/projects/omegaflow`, Branch `main`,
synchron mit `origin/main`.

## Was diese Session getan hat (der Arc)

1. **Crossmatch-Welle** — 12 distanzlose Sternkataloge (gcvs/cbdata/denis/
   merlin/pastel/polarbase/psr/sb9/vsx/wds + neu corot/mktypes) bekamen echte
   Gaia-Distanzen via `tap_compiler --crossmatch`. wd nutzt eigene Parallaxe.
   evs_cat (B1950) bleibt Gap.
2. **Sternkinematik** — Crossmatch-Ziel auf `I/355/gaiadr3` gewechselt (hat
   pmRA/pmDE/Plx/RV/Teff/BPmag/RPmag/Gmag + Dist in EINEM Join). `--crossmatch-pm`
   zieht pmra/pmdec/plx/rv/gaia_teff/bpmag/rpmag/gmag. 13 cmap-Blöcke tragen
   `pmra`/`pmdec`/`radvel` (3D-Raumgeschwindigkeit).
3. **Sternfarbe** — Teff/BPmag/RPmag/Gmag geerntet (BP−RP-Farbe bereit), noch
   NICHT gerendert.
4. **Harvests** — b2find (Host-Umzug `b2find.dkrz.de` → `b2find.eudat.eu`,
   Science-Crawl 166.980 Datasets), PANGAEA-Vollharvest (446.802, LFS),
   NEOWISE/AKARI Asteroiden-Durchmesser+Albedo (129.771 + 5.120).
5. **Sprache (A=A)** — „worldline" getilgt (die Presence IST eine freie Linie
   `p + v·(t − t0)`); Bewegungsverben (moving/navigates/travels) getilgt;
   „songline" entfernt (nie verwendet, nur besprochen); „anthromachinistic"
   → „anthropomachinocentric" (Präfix + -centric). `flow` bleibt Feld-Identität,
   `presence` bleibt Code-Identifier (Drei-Schichten-Kontrakt unberührt).
6. **Presence** — Deep-Link-Init `#x,<x>,<y>,<z>,<t>` implementiert; Audit:
   kein Earth-/Operator-/Station-Rückfall (Init = SSB-Ursprung, `#body=` nur
   Sensor-Träger, `jump()` ist Feature).
7. **Rat** — zwei Sitzungen („Songline"/„Flow"): Einsicht adoptiert (der Pfad
   existiert ohne den Wanderer), Wort nicht übernommen (A=A + Reziprozität).

## Zustand der Daten (live auf CDN `ssd.jpl.nasa.gov`)

Jeder der 12 Sternkataloge trägt jetzt: ra/dec (ICRS) + dist_pc (Bailer-Jones)
+ pmra/pmdec (Eigenbewegung) + rv (Radialgeschwindigkeit) + gaia_teff/bpmag/
rpmag/gmag (Temperatur+Photometrie). Größte: mktypes 686.886, denis 501.055,
wds 136.548, gcvs 60.644 (voll, chunked), vsx 48.912, pastel 37.064.

Asteroiden: DASTCOM trägt GM für Ceres/Pallas/Vesta/Hygiea/Psyche/Kalliope/
Ida/Mathilde/Eros; **4 Lücken** (45 Eugenia, 87 Sylvia, 90 Antiope, 216
Kleopatra — GM aus Perturbation/Doppel/Radar publiziert, DASTCOM=0).
Durchmesser+Albedo liegen in `phi/katalog/asteroid_diameters_*.φ`.

## Offene Arbeit (das TODO ist das Register)

- **`TODO.md` → `## Stern-/Asteroiden-Physik — Handover C`** (der Hauptfaden):
  1. Sternfarbe-**Rendering** (BP−RP/Teff → RGB in der WGSL statt reiner
     Luminanz) — kleinster Schritt, schließt den „ich mag die Farben"-Bogen.
  2. Die 7 Geometrie-Rechnungen (reine Geometrie, nur möglich weil alles einen
     ICRS-4D-Rahmen teilt): stellare **Okkultationen** (Flaggschiff),
     Hill-Sphären, hydrostatische Abplattung, Co-moving-Gruppen,
     Sternbegegnungen, paarweise 3D-Abstände, g/v_esc.
  3. Massen-Lücken harvesten, NEOWISE-Join (Durchmesser als Feld), LCDB-Pole,
     DAMIT-Formen.
- **`TODO.md` → `## Archivar — Membran-scoped Cache`**: HEALPix-gebinnte
  Assets, Archivar holt nur die Bins der Hülle (kein Blockuniversum im Speicher).
- **`TODO.md` → `## CI — Chunk-Kompilation`**: `tap_compiler --chunk-bands`
  (Rust) statt `chunk_master.py` (Python) im monatlichen Workflow.

## Hart erworben (spart der nächsten Session Schmerz)

- **VizieR-Sync-Crossmatch trunkiert/voidet bei großen Katalogen** (Server-
  Zeitlimit, ~20–40 k). Für >50 k chunken: `--where 't."RAJ2000" >= X AND
  t."RAJ2000" < Y'` — **mit `t.`-Qualifier** (gaiadr3 hat eigenes RAJ2000,
  unqualifiziert → TAP-400 „ambiguous column").
- `phi/port/chunk_master.py` ist fortsetzbar (fertige Chunks werden resümiert,
  void-Bänder 3× retried, Merge erst wenn alle Bänder Daten haben).
- **Chunk-Datei-Namen**: `{name}_c{lo}.json` — Vorsicht mit Glob
  `gcvs_c*.json` (kollidiert mit dem Asset `gcvs_cat.json`!).
- gaiadr3 (`I/355/gaiadr3`) hat pmRA/pmDE/Plx/RV/Teff/BPmag/RPmag/Gmag + Dist
  (Bailer-Jones) — EIN Join statt paramp (nur Dist) + gaiadr3 (pm).
- Monitoring langer Hintergrund-Jobs: ein `general`-Agent mit selbsttragendem
  Prompt (Upload + Ledger + Commit bei jedem `TOTAL`, Neustart bei Tod).
- Commit nur `phi/port/ledger.φ` + `TODO.md`; die parallele Arbeit anderer
  Sessions bleibt unberührt (git ist aktuell sauber).

## Verifikation

`cargo check` 0 Fehler / 0 Warnungen. `cargo test --bin omegaflow
test_extract_cmap` (5 Tests) + `test_live_sources_extract` grün. Die
Daten-Seite (Ernte) ist abgeschlossen; die Nutzung (Rendering + Geometrie)
ist die offene Arbeit.
