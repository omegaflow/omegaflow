<!--
  title: Handover — de441-mars-Rekompiilat: Fix + Re-Dispatch (Run 34393748385, head b59bbfb); Re-Verifikation pending
  class: handover
  date: 2026-09-09
  sha256: dbb4708de6f8858a46c143467aa795f1b1469d37a84a4a8d09b297384dbe56a6
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-mars-rekompilat.md docs/befund/befund-2026-09-09-de721-planeten-selektion.md
-->

# Handover — de441-mars-Rekompiilat: Fix + Re-Dispatch

Übergabe der Re-Verifikations-Linie. Die Dispatch-Linie wurde weitergemessen: der
bodies-Job des Dispatches (Run 34387873054) löste failure auf; zwei Ursachen wurden
gemessen, beide gefixt, der Council hat entschieden, neu dispatched. Die
Re-Verifikation trägt die nächste Sitzung nach grünem Lauf.

## 1. Failure-Befund (gemessen, gh run view / --log-failed)

bodies-Job (34387873054) failure, Step „Compile Horizons bodies and upload to CDN":
`horizons_compiler --ci-mode` schrieb `data/ssd.jpl.nasa.gov/ephemeris_apophis.bin`
→ `No such file or directory (os error 2)`. `main()` legt nur `data/` an, alle
Write-Pfade liegen unter `data/ssd.jpl.nasa.gov/`. Vor dem Failure hatte der
„Flatten SPK bodies"-Step kleine Bins kompiliert und auf das CDN geladen (earth
36 020 Granulen / 19 018 776 B; mercury/venus/… 15 985 Granulen) — die 183-MB-Bins
wurden überschrieben.

## 2. Wurzel-Befund (gemessen, Council)

Die Planeten-Selektion (`select_system("planets")`,
`tools/harvest/src/bin/ephemeris_compiler.rs`) wählte die Basis mit der höchsten
`numeric_of`-Zahl: `de721_full.bsp` (721) schlug `de441.bsp` (441). `de721_full.bsp`
(NAIF `a_old_versions`, 152 MB, Spanne AD 1599–3000 ≈ 1400 Jahre) liefert die
~19-MB-Kurzform statt der ~183-MB-Vollform (30 390 Jahre, 346 876 Granulen). Der
Council (alle fünf Stimmen) entschied: die Basis `de441` per Namen pinnen —
spiegelbildlich zum `asteroids`-Zweig. Die 19-MB-earth-Produzenten-Frage (vorige
Handover §4) ist damit gemessen geschlossen: Produzent war die Planeten-Flatten unter
`de721_full.bsp`. Einzeln befundet: `befund-2026-09-09-de721-planeten-selektion.md`.

## 3. Fixe (committet, gepusht, dispatched)

`b59bbfb` — zwei Änderungen:
- `ephemeris_compiler.rs`: Planeten-Basis auf `de441` gepinnt (kein numeric-max mehr).
- `horizons_compiler.rs`: `write_binary` legt das Eltern-Verzeichnis an (deckt jeden
  Aufrufer: flyby/daily/long/uranus-c/neptune-c).
`cargo check -p omegaflow-harvest` sauber (0 Fehler, 0 Warnungen).

## 4. Re-Dispatch

`gh workflow run kernel-flatten.yml` → Run `34393748385`, head `b59bbfb`, ref main,
workflow_dispatch, gestartet 2026-09-09T19:13Z, status pending. Der alte Run
(34387873054) läuft noch (jwst-spectra; `cancel-in-progress: false` → der neue
jwst-spectra-Job reiht sich ein, kein Abbruch). Kein Duplikat: der bodies-Job des
alten Runs ist failure/abgeschlossen; der neue Run trägt die zwei Fixe.

## 5. CDN-Zustand (gemessen, curl)

sun 19 018 832 B, earth 19 018 776 B, mars 13 822 248 B, mercury 8 440 296 B,
moon 12 084 968 B — die Kurzform. `de441-cdn-watch` (sun+earth ≥183 MB) ist nicht
grün. Der grüne Flatten des neuen Runs stellt die 183-MB-Form wieder her.

## 6. Ausstehend (Re-Verifikation, pending — die nächste Sitzung)

Nach grünem Lauf (Run 34393748385):
1. Local auffrischen: `data/ssd.jpl.nasa.gov/` + Membran-Cache löschen.
2. `OMEGAFLOW_HIDDEN=1 cargo run -p omegaflow-measure --bin orientation_probe`
   — de441-mars-Anker-Δ soll in die ~0-km-Klasse fallen (vorher 6 045,3 km).
3. `cargo test -p omegaflow-measure --bin orientation_probe` (Lehrbuch-Gate, silent).
4. `cargo run -p omegaflow-measure --bin ephemeris_structure_probe` — 183-MB-Gestalt
   für earth/mars bestätigen.
5. de441-cdn-watch (≥183 MB) grün.

## 7. Register-Reste (übernommen, unverändert offen)

witness presence bleibt reserviert (Consent-Wurzel Art (c)). feature-gate `gpu` =
eigenes Atom, pending. Membran-Reste M02–M07, Kamera-Pixel-Quellen als
WS-Traffic-Hotspot, OPeNDAP-Integration, advective per-Quelle.

## 8. Zustand des Arbeitsbaums

Gemeinsamer Baum; fremde parallele Sitzung aktiv (uncommittete te.rs,
galileo-Probes, phi/sources.φ, laic_probe, quake_location_probe, + untracked
`.github/workflows/galileo-nsurr-20.yml`). Mein Commit `b59bbfb` berührt nur die zwei
harvest-Dateien; fremde Dateien unangetastet. HEAD == origin/main == b59bbfb nach Push.
