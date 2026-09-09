<!--
  title: Handover — de441-mars-Rekompiilat: Re-Verifikation pending (Run 34393748385 läuft)
  class: handover
  date: 2026-09-09
  sha256: 7fea3422fe7e13b7cfba2e9a06eb935df283369de67949514554895794018f04
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-de441-pin-redispatch.md docs/befund/befund-2026-09-09-de721-planeten-selektion.md
-->
# Handover — de441-mars-Rekompiilat: Re-Verifikation pending

Übergabe der Re-Verifikations-Linie. Gemessen statt beteuert: der Re-Dispatch
(Run 34393748385, head b59bbfb) läuft; der bodies-Job ist in_progress, das CDN
trägt noch die Kurzform. Die Re-Verifikation bleibt pending — sie trägt die
Sitzung, die nach grünem bodies-Job läuft.

## 1. Run-Stand (gemessen, GitHub-Actions-API)

Run 34393748385 (workflow_dispatch, gestartet 2026-09-09T19:13:13Z):
- eve ✓ (19:13:26Z), aia ✓ (19:13:43Z), index ✓ (19:29:53Z), bodies in_progress
  (ab 19:29:56Z), jwst-spectra queued.
- Der bodies-Job erscheint erst nach grünem index (needs: index; gemessen am
  alten Run 34387873054: bodies ab 18:36:27Z nach index 18:36:23Z). Sein Fehlen
  im ersten Job-Schnappschuss war kein Ausfall.
- Der index-Job committete KERNEL_INDEX.md → origin/main 2046db7 (CI-Auto-Commit,
  19:29Z); der jwst-spectra-Job reiht sich hinter den noch laufenden
  jwst-spectra-Job des alten Runs ein (cancel-in-progress: false).

## 2. CDN-Zustand (gemessen, Release-API)

Noch die Kurzform: sun 19 018 832 B, earth 19 018 776 B, mars 13 822 248 B
(updated 18:41Z — Uploads des alten Runs). Die 183-MB-Form kehrt mit dem grünen
Flatten des bodies-Jobs zurück.

## 3. Fix gemessen im lokalen Baum

ephemeris_compiler.rs:485 pinnt `planets` auf die Basis de441 (bases.get("de441"));
der horizons-Write-Pfad legt das Eltern-Verzeichnis an (b59bbfb).

## 4. Re-Verifikation (pending — die Sitzung nach grünem bodies-Job)

1. Local auffrischen: `data/ssd.jpl.nasa.gov/` löschen + Membran-Cache
   `cache/omegaflow_eph_*.bin` (cache_root = `cache/`, fetch.rs:855).
2. `OMEGAFLOW_HIDDEN=1 cargo run -p omegaflow-measure --bin orientation_probe`
   — de441-mars-Anker-Δ in die ~0-km-Klasse (vorher 6 045,3 km); die Bin-Größen
   im Output sind die erste 183-MB-Bestätigung.
3. `cargo test -p omegaflow-measure --bin orientation_probe` (Lehrbuch-Gate, silent).
4. `cargo run -p omegaflow-measure --bin ephemeris_structure_probe
   data/ssd.jpl.nasa.gov/ephemeris_earth.bin data/ssd.jpl.nasa.gov/ephemeris_mars.bin`
   — 183-MB-Gestalt (346 876 Granulen, 30 390 yr, Kadenz 32 d).
5. de441-cdn-watch grün (sun+earth ≥ 183 000 000 B = das Kriterium); der grüne
   Watch entblockt S14.

## 5. Register-Reste (übernommen, unverändert offen)

witness presence bleibt reserviert (Consent-Wurzel Art (c)). feature-gate `gpu` =
eigenes Atom, pending. Membran-Reste M02–M07, Kamera-Pixel-Quellen als
WS-Traffic-Hotspot, OPeNDAP-Integration, advective per-Quelle.

## 6. Zustand des Arbeitsbaums

Gemeinsamer Baum; fremde parallele Sitzung aktiv (te-/galileo-Linie). Uncommittet
der fremden Sitzung (gemessen, unangetastet): phi/sources.φ, xml_harvester.rs,
depth_phase_*.rs, depthphase.rs, laic_probe.rs, mseed_measure.rs,
nobel_probe_bz.rs, quake_location_probe.rs, number_audit.rs,
docs/specs/lauf-log.md (Lauf-4-Nummern-Audit-Eintrag); untracked:
docs/befund/befund-2026-09-09-bestand-korpus-herkunft.md. Mein Commit berührt nur
diese Übergabe + die Archiv-Verschiebung der Vorgängerin. origin/main trug zum
Schreibzeitpunkt 2046db7 (CI-Auto-Commit des index-Jobs).
