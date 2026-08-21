<!--
  title: Handover: Wind-Frame-Orbit — CDF 2.5 → GCI_POS + erster kernel_flatten-Lauf
  class: handover
  date: 2026-08-21
  sha256: a023c00734894c844d3b98415df42af7c237946b1fc1e81cca69c03ce27d315e
  status: live
  see-also: phi/pipeline/research/agent_output/wind_frame_2026-08-21.φ
-->
# Handover: Wind-Frame-Orbit — CDF 2.5 → GCI_POS + erster kernel_flatten-Lauf

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit nennen (parallele Sessions in flight)
cargo check --bin wind_waves_compiler   # eigene Dateien 0/0
cargo test --lib wind            # 13 Tests inkl. Real-Dateien, sobald der fremde Baum grün ist
# Orbit-Proben liegen lokal:
#   /tmp/opencode/wi_or_pre_20210101.cdf  (CDF 2.5 — der Parser-Gap)
#   /tmp/opencode/wi_k0_spha_20210101.cdf  (Spin-Phase, keine Position)
# Der wav_h1-Ernte-Kontext:
#   /tmp/opencode/wav_h1_20210101.cdf, /tmp/opencode/wav_h1_19941110.cdf,
#   /tmp/opencode/wind_waves_1994.bin (Stichproben-Kompilat)
```

Referenzen (stehend): `phi/pipeline/research/agent_output/wind_frame_2026-08-21.φ`
(alle Befunde dieser Recherche, selbsttragend), `src/cdf.rs` (der CDF-3.x-Parser),
`src/bin/wind_waves_compiler.rs` (die Ernte-Maschine, seit 2026-08-21
Fenster+Parallel), `src/wind.rs` (Bin magic WAV1), `src/lsk.rs`
(days_from_civil/civil-Muster in bia_efield_compiler.rs), das Register
TODO.md („Frame at wind (pending)") + ledger.φ („kraft-kanal electric
wind-waves").

## Verifizierter Kontext (2026-08-21)

- **Ernte-Maschine fertig (Commit 2b73165):** `wind_waves_compiler` trägt
  `--window-start/--window-end` + `--jobs` (AtomicI64-Queue + Mutex-Sammlung,
  Muster `bia_efield_compiler`), Final-Sort (t, freq, receiver) macht das
  Bin trotz Parallelität byte-reproduzierbar. Stichproben: 1994-11-10 →
  1994-12-31 (52 Tage, 0 void, 31616 Records = 608 Bins/Tag, Roundtrip ✓)
  und 2021-01-Prototyp (18848 Records). CI-Schritt im kernel_flatten
  sun-Job (Timeout 240→360, Asset-Guard auf Release spdf.gsfc.nasa.gov,
  `--ci-mode 1994-11-10 → 2021-12-31 --jobs 8` via `upload_release` —
  `upload_asset` ginge hart auf ssd.jpl.nasa.gov). Register-Zeilen dieser
  Einheit liefen im Fremd-Commit 319d710 mit (parallele GOES-Session).
- **Frame at wind — KEIN SPK (verifiziert):** NAIF-PDS-Liste
  (`naif.jpl.nasa.gov/pub/naif/pds/data/`) komplett ohne Wind-Bundle;
  `/pub/naif/archive/` trägt 404; NAIF-ID −485 ohne Kern. `at wind` bleibt
  ohne Orbit-Ernte pending (0 honored — keine fabrizierte L1-Position).
- **Der Positions-Träger existiert:** CDAWeb `wind/orbit/pre_or`
  (`cdaweb.gsfc.nasa.gov/pub/data/wind/orbit/pre_or/`), täglich 1994–2026,
  `wi_or_pre_YYYYMMDD_vNN.cdf` (~76 KB/Tag). Stichprobe 2021-01-01 (geladen):
  CDF 2.5 (magic `00 00 FF FF`, big-endian — `file` bestätigt
  „Version 2.5 or earlier"; `src/cdf.rs` parst CDF 3.x — der Parser-Gap).
  Variablen (strings-Befund): `GCI_POS`/`GCI_VEL` (geocentric celestial
  inertial, kartesisch), `GSE_POS`/`GSE_VEL`, `GSM_POS`/`GSM_VEL`,
  „GCI Sun Position Vector", `Epoch` (CDF_EPOCH), `EPOCH_DATE` +
  Delta-Qualitätsfelder (ALNG_TRK_MAX_POS_DIFF u. a. — Trk-Vergleich gegen
  die definitive FDF-Bahn). GCI ≈ ICRS: J2000-Mitteläquator/-äquinoktium,
  Frame-Bias ~0,02° liegt weit unter der FDF-Orbit-Genauigkeit (km-Maßstab)
  — Identität benannt, Ausrichtungskorrektur optional beim Kompilat.
  `def_or` = definitive FDF-Orbit, nur Frühjahre (1994–1997 sichtbar,
  letzte Änderung 2012-09-30 — Produkt eingestellt); `pre_at`/`def_at` =
  Attitude; `spha_k0` = SPIN-PHASE (SPIN_PHASE/AVG_SPIN_RATE/STNDEV_-
  SPIN_RATE) — KEINE Position, verworfen, nicht verwechseln.
- **Offen aus der Ernte-Einheit:** der erste kernel_flatten-Lauf — bis
  dahin fehlt das CDN-Asset `wind_waves.bin` (fehlt, null nicht); der
  wav_h1-Baum endet 2021 (2022+ ungeprüft, eigenes Atom).

## Auftrag

A. **CDF-2.5-Parser** (eigenes Atom, `src/cdf25.rs` oder Erweiterung —
   Entscheidung der Session): der alte Container (GDR/Header + VDR/ADR-
   Offset-Records, unkomprimiert — die ISTP-Orbit-Dateien tragen keine
   Kompression) parst nur die Variablen dieser Dateien: Epoch (CDF_EPOCH,
   ms seit 0 AD), GCI_POS/GCI_VEL (REAL4/R8 [3]) — kein generischer
   CDF-2.6-Komplett-Parser nötig. Proben liegen in /tmp/opencode.
B. **`wind_orbit_compiler`:** pre_or täglich 1994–2026 → `wind_orbit.bin`
   (magic eigen, Records t_tdb + GCI x/y/z + vx/vy/vz; def_or 1994–1997
   als definitive Früh-Epoche falls Layout-gleich — sonst benannt
   auslassen). Tages-Records: Kadenz aus dem File (1-min laut spha-Parallele;
   im Probe verifizieren), Decimation gegen das Sample-Budget wie rpw.
   Parallel-Muster + CI (`--ci-mode`, Asset-Guard, Release
   spdf.gsfc.nasa.gov) stehen in wind_waves_compiler.
C. **BODY_REGISTRY + sources.φ-Block** — erst nach B: Body `wind` (NAIF
   −485) + Motion aus dem Orbit-Bin, dann Block `format wind_waves` (force
   em, V, gaussian-inverse-square, tau aus der Datei-Messung) — der
   Loader joint wind_waves.bin-Records mit dem Orbit. Ohne B bleibt der
   Block im Register pending, die Ernte trägt t_tdb/freq/val ohne
   Raumposition.
D. **kernel_flatten-Lauf verifizieren:** wind_waves.bin auf dem CDN
   (Release spdf.gsfc.nasa.gov) — Asset vorhanden, Download-Roundtrip
   byte-identisch, kein void-Jahr im Log.

## Gates

- cargo check 0/0 (eigene Dateien; fremde Warnungen/Fehler benennen, nicht
  flicken), cargo test — die wind/cdf-Tests laufen kopf-los gegen die
  /tmp/opencode-Real-Dateien; der fremde Baum muss dafür grün sein.
- Orbit-Ernte kopf-los, Roundtrip ✓, Stichprobe 1994 + 2021.
- Register: TODO.md („Frame at wind" → erledigt, „2022+" bleibt),
  ledger.φ kraft-kanal electric wind-waves, Befund in
  phi/pipeline/research/agent_output/ fortgeschrieben.
- Ein Commit je Einheit; diese Datei nach Abschluss archivieren (AGENTS.md:
  nur nach eigenem Commit, status: consumed, cp + git rm).

## Nicht anfassen

Im Baum läuft fremde, teils uncommitete Arbeit paralleler Sessions:
`src/mathematikerin.rs` (presence_tx-Tupel-Edit in flight — bricht den
lib-check), `src/archivar.rs`, `src/relay.rs`, `static/constants.js`,
`static/index.html`, `.github/workflows/ci.yml` + `healthcheck.yml` +
`scripts/`, `phi/pipeline/refusal_ledger.φ`, AGENTS.md, TODO.md und die
acht uncommitteten Handover (ausgabe-flaechen-sensoren, spektral-atom-c,
sphaeren-ringe-warp, stern-asteroid-physik, source-port-pipeline,
katalog-luecken, validation-ci, archivar-arbeitsliste). Erst `git status`
lesen; nicht deren Dateien anfassen; kernel_flatten.yml ist committet und
frei.
