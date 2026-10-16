<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-17
  sha256: f0b8f16e2ccc6b92c932153015438d55708c20640df40775caed0962bff5850e
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge361)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `4e2ff67d8` (== `origin/main`) · geschrieben 2026-10-17 · **Mycelium-361**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds --pages 12` (einmal): **612** non-success — Verteilung (gemessen, Summe 612).
Ein gemessener Grund je Träger, gelesen am HEAD-Lauf (`ci_manage jobs`/`log`), sonst `unread`:

| workflow (n) | gemessener Grund |
|---|---|
| `ci-gate` (353) | am HEAD `4e2ff67d8` (Lauf `38975908313`): `build`/`register`/`clippy` **success**; `format` **failure** (rustfmt-Drift `tools/harvest/src/bin/venus_radar_compiler.rs` + `tools/gate/src/bin/commit_check.rs:171` + `tools/register/src/bin/open_points_check.rs:495` — **in diesem Pass local geräumt**); `subset` **failure** (`2890 passed · 3 failed · 51 ignored`: `test_law_bounds_enclosure_span_holds_for_periapsis_in_window`, `test_load_gate_clips_records_outside_enclosure`, `test_raw_presence_gate_admits_within_reach_and_refuses_fieldless_source` — archivar, fremd); `dropped-gate` **failure** (`current 11 · pinned 89 · new 11`, u.a. `f4 schumann-antrieb wglc global täglich` · `higp-modvolc-archiv`). |
| `register-coverage` (155) | Job `orphans` — **unread** (lokal `register_lookup --orphans --fail` = 0). |
| `tools-build` (44) · `quake-feeds-cdn` (14) · `matrix-rotor` (11) · `m10-mdis-raw-cdn` (8) · `health-check` (6) | **unread** (fremde Träger). |
| `zenodo-raw-cdn` (4) · `gic-al-network` (4) · `timed-orbit-cdn` (2) · `geofon-waveform-cdn` (2) · `ci-check` (2) | **unread**. |
| je 1: `wdcb-obn-wayback-cdn` · `venus-radar-cdn` · `themis_asi-fodg-cdn` · `ssd-jpl-raw-cdn` · `hips-png-cdn` · `de2-cdn` · `.github/workflows/gic-al-network.yml` | **unread**. |

## Lokal gemessen am HEAD `4e2ff67d8`

- **Venus-Block geschlossen** (der Dispatch): das `.parts`-Manifest liegt im CDN
  (`zenodo.org/Venus_RadarBasemap_2025_Global_75m_v1.tif.parts`, 4755 B, 38 Teile); der
  Ganz-`sha256` daraus = `1ddfcd6b9a9feff5ac3a573c8c26dc7a9a03685a99aed84bf5ce95e28b03d98b`,
  deckungsgleich mit `phi/sources.φ:30588` (kein `md5`).
- **format-Drift geräumt** (dieses Rund): die drei Dateien rustfmt; `cargo fmt --all --check` = 0;
  `cargo check -p omegaflow-gate -p omegaflow-register -p omegaflow-harvest` = 0/0.
- **Orphan-Docs:** von **54** auf **49** gefaltet — die fünf eigenen Quellen-Surveys
  (`weberin-thread-matrix` · `weberin-quellen-rerun` · `weberin-zweite-linie` · `exposom-matrix` ·
  `worldwide-lidar-quellen`) sind jetzt in folge361 benannt.
- Register-Nullstände: `--orphans --fail` = 0 · `--fired mycelium` = 0 · `--stale mycelium` = 0 ·
  `--addressed mycelium` = 1 (`xfer:river-224-4`/`-5`).

## Postfach

- `post/in` (privates Haus): 13 Einträge, neuester `2026-10-16-sensory-orphan-docs-ohne-traeger.md`
  (00:24) · `2026-10-17-mountain-ratsanfrage-acht-faeden-physik.md` (00:12). Mein NSSDC-Antrag
  (2026-10-14) ist bereits gewortet. `post/paid-open` (27 B) fremd — nur die Größe gelesen.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14T12:56:49Z**
  (`OMEGAFLOW_SESSION` unset → die neue Welle schreibt keine Zeile).

## Mehrfach-Träger

- `--addressed mountain` = 2 · `--addressed future` = 2 · `--addressed mycelium` = 1
  (`xfer:river-224-4`/`-5` aus river-224). Kein doppelter Träger über zwei Handover.

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund, `refs/safety/1792189897`).

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
