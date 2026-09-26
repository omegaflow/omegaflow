<!--
  title: Handover — Mountain-Folge 168 (2026-09-26)
  session: Mountain-Folge 168
  class: handover
  date: 2026-09-26
  sha256: 324c5bd2727711297c422d82981d52340ea544cc78ceffc77a92e6ace43960c1
  status: live
-->
# Handover — Mountain-Folge 168 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### gll.rss + Klasse-5 — Commit/Push + Dispatch + Register-Nachtrag
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf-Ende der dispatchten Runs.
- **Lage:** (gemessen 2026-09-26) Committet `cc991b751` + gepusht (HEAD ==
  `origin/main`); dispatcht `gll-rss-atdf-cdn 36233487657`, `tools-build
  36233489263` — Ergebnis ungemessen. Eigene Arbeit:
  `phi/sources.φ` (gll_rss_rsr/atdf/atdf_x/messenger_tnf/ams02_spec + 46
  ams02-Felder, `+86`), `phi/blocked_sources.φ` (unit-auto-detect-Batch `-169`
  + Query-Fixes), `src/archivar/units.rs` und `src/archivar/tests.rs` (`gev`/`gv` in
  `allowed_units_for_force(0)`, `cargo check` 0/0), `src/archivar/main_flow.rs`
  (`cassini_rsr`-Fetch-Arm), neu `.github/workflows/gll-rss-atdf-cdn.yml`;
  `origin/main` Vorfahr von HEAD (`merge-base --is-ancestor` bestätigt). CDN-Läufe
  (via `ci_manage view`): `gll-rss-tnf 36227808151` **success** (gll_rss_tnf.bin,
  6151544 B, 85438 Samples), `messenger-tnf 36227810541` **success**
  (messenger_tnf.bin, 193905440 B, 2693131 Samples, 45 Dateien), `ams02-tdat
  36227812411` **success** (ams02_spec.bin, 1320488 B, 18340 Zeilen);
  `gll-rss-odr 36227804843` **in_progress**; kein Lauf-Log nennt sha256/Shards.
  Via `--sniff` nachgemessen (2026-09-26): `gll_rss_tnf.bin` 200/6 151 544 B/sha
  `0edf5ab2…` und `ams02_spec.bin` 200/1 320 488 B/sha `1a64e5b3…` in
  `phi/sources.φ` eingetragen; `messenger_tnf.bin` 200/≥93 634 469 B (sha
  pending — Teildownload).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36233487657` / `ci_manage view 36233489263`
  (einmalig); dann `messenger_tnf.bin`-sha, die Shards von `gll-rss-odr`/
  `gll-rss-atdf` und `gll-rss-rsr-cdn.yml` nachtragen.

#### gll_rss_rsr — Workflow fehlt
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via `sgrep`) Kein getrackter Workflow nennt
  `gll_rss_rsr_compiler`/`gll_rss_rsr`; Asset `gll_rss_rsr.bin` ist registriert
  (`phi/sources.φ:8474`), `cassini_rsr`-Parse-Arm (`extract.rs:78`) und
  Fetch-Arm (`main_flow.rs`) stehen.
- **Blockade:** keine.
- **Braucht:** `gll-rss-rsr-cdn.yml` bauen (Muster `.github/workflows/
  galileo-atdf-cdn.yml`, Release `pds-rings.seti.org`, Compiler
  `gll_rss_rsr_compiler -- --ci-mode`, Idempotenz auf `gll_rss_rsr.bin`).

#### atdf.rs F8-Item-20 Doppler-Bias (`/1000`)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via grind-flash) Code `atdf.rs:383`
  `doppler_bias: extract(rec, &TKFORM8[12]) / 1000` (Item 20, `atdf.rs:243`);
  der Format-4-Pfad `atdf.rs:349` teilt **nicht**; einzige Spec im Baum ist
  Format 4 (`docs/reference/trk-2-25-atdf.txt:239`, „MHz"); SFOC-NAV-2-25-SIS
  fehlt. Wirkung nur im Gate `bias.abs() > 1` (`atdf.rs:724`).
- **Blockade:** keine.
- **Braucht:** SFOC-NAV-2-25-SIS beschaffen **oder** am echten F8-Granule
  `gll_rss_2002308t0717_dssmm_tdf.dat` die Item-20-Rohwerte gegen die
  physikalische Bias-Skala lesen — dann `/1000` belegen oder entfernen.

#### cassini_rsr SFDU-Magic — GLL ungemessen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via grind-flash) Cassini-Kopf
  `NJPL2I00C997` (`cassini_rsr.rs:188`, `phi/harvest.φ:39`); DSN-SIS 1992 nennt
  `NJPL2I00C371` bei identischem CHDO-Layout (Offsets 256/258); GLL-SIS 1997
  nennt keine ID; kein echtes `gll.rss_rsr`-Label in `data/`/`cache/`.
- **Blockade:** keine.
- **Braucht:** erste 12 Bytes eines echten `gll.rss_rsr/*.dat` (PDS-rings-Bundle)
  lesen — falls ≠ `C997`, weist `read_record` (`cassini_rsr.rs:63`) die
  GLL-Route ab.

#### unit-auto-detect — Präzedenz-Zuordnung sz/kp
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via grind-pro) `sz_signal_to_noise` →
  `port.rs:2082` und `kp_value` → `port.rs:2118` sind **rein namensbasiert** auf
  `("em","1")` gesetzt; kein Feld im Register trägt die Einheit. Wolf-Zahl/σ sind
  gemessen descoped (`blocked_sources.φ:276/280/252` etc.).
- **Blockade:** keine.
- **Braucht:** echten sz-/kp-Datensatz registrieren und die Einheit am Feld
  messen (dann Präzedenz ersetzen oder descoped mit Befund).

#### Parquet-/GRIB-2-Codec-Bau
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Datensatz-Trigger (GRIB-2 simple/complex/CCITT-G4: sofort).
- **Lage:** (gemessen 2026-09-26 via grind-pro) Structure-Reader stehen.
  **Bauen:** GRIB-2 simple 5.0 + complex/spatial 5.2/5.3 (NOAA-GFS) +
  CCITT-G4 42/40020 (ECMWF-IFS, nah); Parquet gzip/delta/LZ4 bei Trigger
  (nah-mittel). **Descoped (Befund):** Parquet zstd/BROTLI/LZO, GRIB-2 JPEG2000
  — „kein registrierter Datensatz nutzt X, Voll-Codec außerhalb dieses Gaps".
- **Blockade:** keine.
- **Braucht:** je „bauen"-Eintrag den std-Arm + Gate-Test
  (`parquet.rs:1003`/`1016`/`1088`, `grib2.rs:193`).

#### NAIF mariner10 — dauerhafte Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** frame_registry-Regeneration.
- **Lage:** (gemessen 2026-09-26 via grind-flash) `at mariner10` steht jetzt an
  der NAIF-`url` in `phi/blocked_sources.φ`; `phi/sources.φ` trägt `at mariner10`
  bei `:3240` (die Handover-Zahl `:3204` war gedriftet — dort steht `at
  jupiter`). `frame_registry.φ:523` trägt den NAIF-Key, ist aber aus dem
  aktuellen Bestand nicht reproduzierbar.
- **Blockade:** generierte Datei (`gitignored`).
- **Braucht:** `OMEGAFLOW_HIDDEN=1 cargo run -- --draft-context` (Regeneration),
  dann `phi/pipeline/frame_registry.φ` prüfen; oder CI-Lauf, der sie erzeugt.

#### Atom D — Beat-Paar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** two-station open-loop Aufnahme eines Trägers (extern) — sonst kein Flip.
- **Lage:** (gemessen 2026-09-26) Atom D gebaut (`odf.rs::tnf_phase_series`,
  WGSL `beat_pair` hinter presence-/ν-Gates); das Paar selbst absent.
- **Blockade:** keine Messdaten für ein kohärentes Paar.
- **Braucht:** dual-comb-/two-station-Kandidat an Mycelium; bis dahin `wartend`.

### Operator handelt

#### S3-Scheme (Token) + `epochrange` Wire-Slot + Membran-Debug (Chrome MCP)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26) S3-Scheme `blocked key` (SigV4+Handshake
  gebaut; alter 401 = Register-Umbuchung); `epochrange` (MJD-Breite) hat keinen
  Wire-Slot (Architektur-Akt); Chrome-DevTools-MCP antwortet `-32001 timeout`,
  braucht Start mit `--no-usage-statistics --no-performance-crux`.
- **Blockade:** Operator-Wort (bzw. Token).
- **Braucht:** je ein Operator-Wort; MCP-Start-Config danach in
  `docs/concepts/tools-map.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
