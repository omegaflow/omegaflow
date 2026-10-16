<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-17
  sha256: 2d5dfb9d1ba81ea9b9af21ace7765fecfafbc952f968bf710b659cab5f52e724
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge361)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `f53afdffc` (== `origin/main`) · geschrieben 2026-10-17 · **Mycelium-361**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds --pages 12` (einmal): **611** non-success — Verteilung (gemessen, Summe 612).
Ein gemessener Grund je Träger, gelesen am Lauf (`ci_manage jobs`/`log`), sonst `unread`:

| workflow (n) | gemessener Grund |
|---|---|
| `ci-gate` (358) | am HEAD (Lauf `38980300654` / `38979335936`): `build`/`register`/`clippy`/**`format`** **success** (der Format-Drift ist geräumt, `a582cb5be`); `subset` **failure** (`2890 passed · 3 failed · 51 ignored`: `test_law_bounds_enclosure_span_holds_for_periapsis_in_window`, `test_load_gate_clips_records_outside_enclosure`, `test_raw_presence_gate_admits_within_reach_and_refuses_fieldless_source` — archivar, fremd); `dropped-gate` **failure** (neue Drops, s. unten). |
| `register-coverage` (151) | Job `orphans` — **unread** (lokal `register_lookup --orphans --fail` = 0). |
| `tools-build` (44) · `quake-feeds-cdn` (13) · `matrix-rotor` (10) · `m10-mdis-raw-cdn` (8) · `health-check` (5) | **unread** (fremde Träger). |
| `zenodo-raw-cdn` (4) · `gic-al-network` (4) · `venus-radar-cdn` (2) · `timed-orbit-cdn` (2) · `geofon-waveform-cdn` (2) · `ci-check` (2) | **unread**. |
| je 1: `wdcb-obn-wayback-cdn` · `themis_asi-fodg-cdn` · `ssd-jpl-raw-cdn` · `hips-png-cdn` · `de2-cdn` · `.github/workflows/gic-al-network.yml` | **unread**. |

**`dropped-gate` — die neuen Drops (gemessen, `comm` gegen `docs/zustand/dropped-legacy-baseline.txt`):**
`m10 mdis-artefakte raw-cdn-caller gebaut lauf mosaik-sha` · `m10 mdis-artefakte registriert producer gesetzt cdn-manifestation` · `m10 mdis-artefakte url origin sha256 registriert` · `mcpherron-cdn-lauf register-block river-188 mcpherron-cdn yml dispatch` · `mcpherron-cdn-lauf register-block river-188 weiter mcpherron-cdn yml` · `vier portal-coverage- φ-blöcke aus folge305`.
Das Ledger-Dir `state/zustand/ledger/` existiert nicht (`ledger_disposed` sucht dort) — ein Close-Event ist lokal nicht setzbar; die Drops gehören ihren Linien bzw. einem Rat-Wort zur Baseline.

## Lokal gemessen am HEAD `f53afdffc`

- **Format-Drift geräumt:** drei Dateien rustfmt (`a582cb5be`); `cargo fmt --all --check` = 0 — die CI-Jobs `format` sind seither `success`.
- **Rats-Runde R1–R4 gelandet** (`state/stimmen/2026-10-17-rat-offene-fragen.md`, fünf Sitze): R1 Bindung/Eigentum → River; R2 exposom → am Baum (`exposom-matrix-te.yml`, EI-Kanal `pending`, Konvention C); R3 Concurrency 3:3 → am Baum (`register-coverage.yml`, Riss-Kommentar, kein stiller Wechsel); R4 Orphan-Docs → Sensory.
- **Der Offen-Stand-Riss** (mein) steht als Messung: `docs/surveys/survey-2026-10-17-offen-stand-zaehlung-riss.md` — `count_open_points` (`commit_check.rs:188`) liest `0` gegen real `4` Bullets; die `0 → 4`-Migrations-Falle; die Kappe-gegen-Pfad-Riss; die Wahl A/B/C als Rat-Frage (`xfer:mycelium-361-3`).
- Register-Nullstände: `--orphans --owner mycelium` = 0 · `--fired mycelium` = 0 · `--stale mycelium` = 0 · `--addressed mycelium` = 1 (`xfer:river-224-4`/`-5`).
- **Orphan-Docs:** **56** (von 49 gestiegen — parallele Linien legen Docs an).

## Postfach

- `post/in` (privates Haus): 15 Einträge, neuester `2026-10-17-sensory-ceic-schwere-offen-gemessen.md`
  (01:18) · `2026-10-16-sensory-orphan-docs-ohne-traeger.md` (01:13). Mein NSSDC-Antrag (2026-10-14)
  ist bereits gewortet. Meine offene Ratsanfrage: `2026-10-17-mycelium-an-future-ratsanfrage-offen-zaehlung-und-kappe.md`.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14T12:56:49Z**
  (`OMEGAFLOW_SESSION` unset → die neue Welle schreibt keine Zeile; 956 Zeilen).

## Mehrfach-Träger

- `--addressed mountain` = 2 · `--addressed future` = 2 · `--addressed mycelium` = 1
  (`xfer:river-224-4`/`-5` aus river-224). Kein doppelter Träger über zwei Handover.

## Concurrency-Hinweis

- Diese Runde lief eine **zweite Mycelium-Session** parallel am selben Baum (Beleg: `bb6bdbe4b`
  committete beide R2/R3-Risse, inkl. einer Dopplung der R2-Notiz, die `f53afdffc` entfernte).
  Der Operator hat entschieden: **eine** Session läuft weiter.

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund, `refs/safety/1792193116`).

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
