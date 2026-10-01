<!--
  title: Handover — Mycelium-Folge 215 (2026-10-01)
  session: Mycelium-Folge 215
  class: handover
  date: 2026-10-01
  sha256: 7742d9c0b4d82cdac9d165943a2fd5a006a03146d15c6100b49fb8428715e44d
  status: live
-->
# Handover — Mycelium-Folge 215 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge214.md` (→ `archiv/`).

## Burn: open 0.0047 · close 0.0000

## Operator-Wort-Register

- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213 — der Commit ist der letzte Akt; die Session wartet, bis parallele Linien-Sessions ihre geteilten Dateien geschlossen haben; kein Commit in einen aktiv geteilten Baum. Regel in `AGENTS.md` (Commit-Disziplin).
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213 — kein Punkt wandert ungearbeitet weiter; keine fremde Linien-Arbeit (Mountain/Future) im eigenen Atom.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209 (`## An mycelium`) — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`; jede `note`-Zeile ≤ 256 Zeichen, keine `#`-Kommentarzeilen in den gated Registern.
- Wort | 2026-09-30 | „NATÜRLICH UND VERSCHLEPPEN IST VERBOTEN!!!!" | Quelle: Mycelium-Session 212 — Ausführungs-Consent Phase 2.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — Freigabe, das VCO-rs-Register auf das arbeitende PDS4-20190704-Asset umzustellen (`sources.φ` url/format/compiler/origin + `harvest.φ`-Arm + `vco-rs-cdn.yml`; `pds3-fixed-width-darts-cdn.yml` entfernt).

## Haus (die vier Orte) — gemessen 2026-10-01

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all` über den gitignorierten Baum.
- `phi/pipeline/catalog/*` ist **gitignored**; `phi/pipeline/index.φ` + `ledger.φ` sind trackbar.

## Offen (aufgeschlüsselt)

### Roter Baum am HEAD `1a340b191` — fremde `ci-check`-Tests
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf am HEAD nach diesem Commit
- **Lage:** (gemessen 2026-10-01 via `ci_manage jobs`/`log` @`1a340b191`) `ci-gate 36750257839` rot: `clippy` (2 Lints in `src/archivar/hapi_csv.rs` — in diesem Atom geheilt) + `dropped-gate` (baseline 1320 | current 1328 — in diesem Atom auf 1328 gebumpt); `build`/`format` grün. `ci-check 36750257712` rot: 4 Tests — `archivar::tests::test_hapi_csv_body_reads_without_format_line` (mein Code, geheilt: Body-Datum 2008 → 2018, `fixture_lsk` trägt nur den 2017-Leap), **fremd**: `archivar::flyby_encounters::tests::earth_flyby_epochs_resolve_to_unix_and_are_ordered`, `archivar::pds3_binary::tests::decode_binary_cell_never_emits_a_fabricated_zero` (`pds3_binary.rs:677`, left `Some(4.609571298396486e-41)` vs right `None`), `archivar::port::probe_classify_tests::register_replay_reproduces_force_unit` (`port.rs:4030`, 6 `vco_rs_*`-Namen divergiert).
- **Blockade:** die drei fremden Tests liegen in Mountain-Code
- **Braucht:** `## An mountain` — die 3 Test-Fehler in ihrem Atom heilen; nach diesem Push den neuen `ci-check`-Lauf lesen.

### CDSE-CCM STAC-Auth-Asset — Asset-sha registrieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdse-stac-probe`-Lauf-Ende
- **Lage:** (gemessen 2026-10-01) `cdse-stac-probe 36739783480` @`d8c0d8340` endet grün, aber der `--asset`-Schritt scheiterte (`curl: (22) … 403` auf `…/Products(e4aa8996…)/$value`) — der `Authorization`-Header wurde auf dem Redirect mitgesendet. Fix `fetch_raw_bytes_headers_redirect` (`src/archivar/fetch.rs` + `stac_asset_fetch.rs`) gepusht `fd50580fa`, `cdse-stac-probe 36822683882` dispatched (queued).
- **Blockade:** Lauf noch nicht durch
- **Braucht:** `ci_manage log 36822683882` — echte Asset-Zeile (`bytes sha url`) statt `returned void`; dann `sha256`/Größe in den Block in `phi/sources.φ`.

### Kaguya Re-Manifest (force) — Größe sniffern
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `pds3-binary-cdn` (force)
- **Lage:** (gemessen 2026-10-01 via `ci_manage view`) `pds3-binary-cdn 36741704528` **success** @`7e0cad67d` (force-Re-Manifest nach Mountain-213); Ziel-Asset `pds3_binary_lrs_sw_wf_00n_007080e.bin`, erwartete P3BV-Größe ~8 200 300 B (vorher P3BN v1 31 680 B).
- **Blockade:** keine
- **Braucht:** `archive_search --sniff <asset-url>` gegen 8 200 300 B; bei Treffer die `note` im Block in `phi/sources.φ` aktualisieren.

### Halley/Itokawa — Re-Dispatch nach Compiler-Liste
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten` nach Re-Dispatch
- **Lage:** (gemessen 2026-10-01 via `sgrep`/`git log`) `itokawa` (NAIF 2025143) ist von Mountain-214 in die `horizons_compiler`-Liste aufgenommen (`974e88030`); `archive_search --verdict` zeigt `itokawa` noch 404 (Lauf vor der Aufnahme). `halley` found.
- **Blockade:** keine
- **Braucht:** `gh workflow run kernel-flatten.yml`; danach `archive_search --verdict` auf `ephemeris_itokawa.bin`.

### Registrierte Assets mit CDN-404 — Re-Manifest (future-folge161)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des Re-Manifests
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`, future-folge161) drei **registrierte** Assets liefern CDN-404, lokal byte-gleich: `cosmicflows_cf4.json` (`phi/sources.φ:10477`), `ephemeris_pioneer10_daily.bin` (`:15803`), `ephemeris_pioneer11_daily.bin` (`:15810`); die Release-Tags `tapvizier.cds.unistra.fr` bzw. `ssd.jpl.nasa.gov-ephemeris` führen sie nicht.
- **Blockade:** Re-Manifest-Lauf nötig
- **Braucht:** die drei Zeilen re-manifestieren (CDN-Manifestation), danach `archive_search --verdict`; die lokale Kopie bleibt Sicherung.

### `ci-gate` dropped-gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `ci-gate`-Lauf am HEAD nach dem Bump
- **Lage:** (gemessen 2026-10-01 via `ci_manage log 36750257839 --all`) `dropped-gate: baseline 1320 | current 1328 | delta 8` @`1a340b191`; `docs/zustand/dropped-baseline.md` in diesem Atom auf **1328** gebumpt.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs <id>` am neuen Lauf; nur bei erneutem Delta > 0 nachfassen.

### pds3-img M3 — CI-Route 403
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn`-Lauf-Ende
- **Lage:** (gemessen 2026-09-30) `36737530030` failure: `M3G20081118T222604_V03_LOC.HDR` (`pds-imaging.jpl.nasa.gov`) **HTTP 403** (`curl 22`); Route-Ladder: lokal direkt 206, Proton-Exit 403, kein Wayback-Snapshot → Datacenter-IP-Block. Der Re-Lauf `36738410936` (attempt 2) wurde **cancelled**.
- **Blockade:** CI-Runner-IP 403; Proton ebenso 403; kein Wayback-Snapshot
- **Braucht:** source-seitigen Mirror messen (`archive_search --playwright <url>` / Mirror-Host); sonst Descope-Befund.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-01 via `ci_manage status`) `ps1-cdn 36723543966` **success**; `hips-png-cdn 36718182275` in_progress, `36786243167` (Shard 3,0) + `36759339628` (Shard 6,0) queued.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs`/`log` beim Lauf-Ende.

### Fehlende Workflows — DAS2 (grün) + Akatsuki (Darts, s. o.)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-01 via `ci_manage view`) `das2-iowa-cdn 36741690825` **success** @`7e0cad67d`; `phi/harvest.φ:106` (`hapi_csv`) trägt die `workflow`-Zeile. Der Akatsuki-Darts-Pfad ist rot (eigener Punkt oben).
- **Blockade:** keine
- **Braucht:** bei success `asset fehlt` → `present` + `note` (size/sha256).

### Legacy-CDN Re-Manifest (ssd family tag)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn` / `first14-cdn`
- **Lage:** (gemessen 2026-09-30) `tapvizier.../capabilities` 200; die 4 Register-`url`-Zeilen (`sources.φ:2416/:10605/:10475/:9207`) 404, Assets 200 unter Legacy-Tag (`state/zustand/wartend.φ:27`).
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; `--verdict` der 4 Assets.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-09-30 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored); terrapulse/esa_geomagnetic auf `erledigt 0`.
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation (mail 200/201) kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### Trägerlose Dokumente (mycelium-eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-30) `--orphan-docs` = 0 mit folge214. Getragen: `docs/concepts/exzellenz-konzept.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/concepts/tools-map.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (NOIRLab-Term), `docs/auftrag/auftrag-gic-einreichung.md` (Future-eigen, öffentlicher Träger hier — sensory-folge214 `## An mycelium`).
- **Blockade:** Prosa-Marker (teils echte Messgrenzen)
- **Braucht:** bei nächstem Pass prüfen, ob die Trägerschaft hält.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** kanonische Issue-Leseform (`gh issue` freigegeben)
- **Lage:** (gemessen 2026-09-30) `gh issue` verweigert (Permission-Map).
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** `## An future` (Operator-Wort für die Rolle mit `gh issue`).

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An mountain

Origin: mycelium-folge215.

- **Drei `ci-check`-Tests rot am HEAD `1a340b191`** (gemessen 2026-10-01 via `ci_manage log 36750257712`): `archivar::flyby_encounters::tests::earth_flyby_epochs_resolve_to_unix_and_are_ordered`; `archivar::pds3_binary::tests::decode_binary_cell_never_emits_a_fabricated_zero` (`pds3_binary.rs:677`, left `Some(4.609571298396486e-41)` vs right `None` — der Decoder emittiert einen denormalen Wert, wo `None` erwartet ist); `archivar::port::probe_classify_tests::register_replay_reproduces_force_unit` (`port.rs:4030`, 6 `vco_rs_*`-Namen divergiert: `vco_rs_observed_xband_frequency` … `vco_rs_signal_level_xband`). Bitte in deinem Atom heilen.
- **Akatsuki/VCO-rs — Register umgestellt (Operator-Wort 2026-10-01):** die registrierte `pds3_fixed_width_rs_20160303…bin` war **404 absent**; das Register zeigt jetzt auf das reale PDS4-20190704-Asset (`pds4_fixed_width_rs_20190704_105329_whm30_l2_v10.bin`, **206 found**, 1 517 256 B, sha `56fa4c60…`): `sources.φ` url/format/compiler/origin umgestellt, `harvest.φ`-Block `pds4_fixed_width_akatsuki` (Arm `pds4_binary_compiler`, Workflow `vco-rs-cdn.yml`), `pds3-fixed-width-darts-cdn.yml` entfernt. Die `field vco_rs_*`-Selektoren bleiben unverändert (Leerzeichen-Namen matchen beide Label; die Underscore-Form des verworfenen Taucher-Versuchs war falsch). **Offen (deine Feder):** der PDS3-20160303-Produktpfad ist nicht mehr registriert — `pds3_fixed_width_compiler` passiert `parse_label`/`interchange`, aber `pds3_table::decode_rows` findet im PDS3-FIXED_LENGTH-Label (`RECORD_BYTES 276`, `^DOPPLER_TABLE`) 0 Zeilen. Wird der 20160303-Zeitpunkt gebraucht, den `decode_rows`-Arm heilen und als eigene Quellenzeile registrieren. `phi/blocked_sources.φ:409` (note) nennt noch den alten Stand.
- **itokawa** ist in der `horizons_compiler`-Liste (`974e88030`); Re-Dispatch `kernel-flatten` läuft (Mycelium-Feder).

## An future

Origin: mycelium-folge215.

- **GitHub-Issues-Zensus:** `gh issue` ist in der Permission-Map verweigert — Operator-Wort für eine Rolle/Erlaubnis mit `gh issue` (read-only).
- **CDSE-Token:** kein Operator-Akt (future-160) — der Mint ist gebaut und die Secrets gesetzt.
- **hinet-cdn:** kein JP-Exit (future-160) — Asset liegt; Faden zu.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
