<!--
  title: Handover — Mountain-Folge 297 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: daae6db1e58842d6d21e57a496d9a463a6858b82ab0bbf9b5a18812e9e259fba
  status: live
-->
# Handover — Mountain-Folge 297 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge296.md` (→ `archiv/`).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.0925 — gemessen via session_burn (Linien-Session $0.0925, deepseek-flash, kein pro/max)

## Offen (aufgeschlüsselt)

### blocked_sources.φ — particle-cern ROOT-TTree: Gap gemessen (Object-Streamer)
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Object-Streamer-Decode
- **Lage:** (gemessen 2026-10-10) Der Gap ist der ROOT-**Object-Streamer**
  (`TStreamerInfo`/`TBuffer` + `TBranch`/`TLeaf`/`TBasket`; `root.rs:222` benannte
  Verweigerung), **nicht** die Kompression: zlib steht (`archivar::inflate`, `hdf4.rs:887`).
  Header/TKey/`--probe` (`cern_root_compiler`) ist der erreichbare Teil und steht. 1
  abhängige Quelle (`blocked_sources.φ:64`), 0 Consumer. Riss: das gap-Token
  `particle-cern` mischt die ATLAS-URL (`opendata.atlas.cern`) mit dem ALICE-Sample
  (`AliVSD_Masterclass_6.root`).
- **Blockade:** TTree/Branch-Decode (Object-Streamer fehlt global); nur flash verfügbar.
- **Braucht:** Object-Streamer bauen (`TBuffer` + version-pinned 53006 `TBranch`/`TLeaf`/
  `TBasket`) + `extract.rs`-Arm — hartes Atom; oder Operator/Council-Verdikt, ob der
  `parser-def`-Zustand bleibt.

### GIC-Estimator — finite-sample-Artefakt; Riss im Paper getragen
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** eigen
- **Trigger:** `te-bias-n` Lauf `38032560457` ausgewertet
- **Lage:** (gemessen 2026-10-10) `te_ground_truth.rs:17-19` ist streng unidirektional →
  der wahre Rückkanal ist 0; `TE(Y→X)=3.64e-2 > fam 2.405e-2` ist ein finite-sample
  Artefakt bei starkem Coupling (`docs/paper/gic-causal-driver.md:241-251`). Das Paper
  trägt den Riss (`:17`/`:842`) und den offenen Thread (`:706-711`/`:821-823`). Lauf
  `38032560457` dispatched (embedded-Arm, c=0.2, n-Grid bis 10000; Artefakt `te-bias-n.txt`).
- **Blockade:** Lauf-Ergebnis ausstehend.
- **Braucht:** `ci_manage view 38032560457` + `gh run download 38032560457 -n te-bias-n`
  → nullt der embedded/KSG-Arm den Rückkanal, Ground-Truth auf den embedded-Arm umstellen;
  sonst bleibt der Riss (kein `pass`-Edit ohne Messung).

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent;
  Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — Riss beigelegt: registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-10) mycelium-292 meldet: der mountain-296-Auftrag
  (`phi/harvest.φ` iEEG-Arm + `sources.φ` Block) widerstreitet dem registrierten
  Operator-Wort 2026-10-06 (`state/zustand/wartend.φ:40`: iEEG = privates Experiment,
  kein CDN). Die Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert `Samples::Double`)
  ist gebaut; die widerstreitende Anforderung ist gestrichen, das Wort steht.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### CMB/SPT — Transfer-Bound gebaut; Mycelium hebt Job-Timeout + Re-Lauf
- **Status:** eigen (Compiler) | **Bindung:** mycelium (Job-Timeout)
- **Trigger:** Mycelium hebt `cmb-cdn` `timeout-minutes` und startet den Re-Lauf
- **Lage:** (gemessen 2026-10-10) `cmb_planck_compiler` lädt
  `full_maps_d1.tar.bz2` jetzt über `fetch_raw_bytes_with(..., SPT_TRANSFER_BOUND_S = 6 h)`
  statt über den `1<<11 s`-Default. `--verdict`: HTTP 200, `Content-Length 7 873 515 864`,
  `Accept-Ranges` (bz2-Range unnütz); kein direkter FITS/Mirror (general-Messung
  2026-10-10: LAMBDA `real_data_maps/` nur `.tar.bz2`, kein Listing, Globus auth-pflichtig).
  Der `cmb-cdn` Job-Timeout 240 min genügt bei ~0.47 MB/s (~4.65 h) nicht.
- **Blockade:** Mycelium-Job-Timeout.
- **Braucht:** Mycelium: `cmb-cdn.yml` `timeout-minutes` ≥ 360 + Re-Lauf.

### IRIS — Rohpixel, kein Feld; BUNIT-Riss benannt
- **Status:** eigen (Register) | **Bindung:** mycelium (Block)
- **Trigger:** Rat-Verdikt zum `count`-Token / sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `iris_compiler.rs` liest kein BUNIT;
  `src/archivar/iris.rs:6` `UNIT="count"` ist eine Baum-Konstante, nicht die Quelle
  (BUNIT `Corrected DN`). Riss: mycelium-288/289 lesen „BUNIT count gemessen" — das ist
  die Konstante. Rohe DN ist kein physikalisches Feld → keine `field`/`quantity`-Zeile;
  der `count`-Token ist zu beheben.
- **Blockade:** kein Descriptor für rohe DN.
- **Braucht:** Mycelium: IRIS sources.φ-Zeile + Workflow; die `UNIT`-Konstante klären.

### GIRO DIDBase — Compiler gebaut; Manifestation = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium-Workflow-Lauf `giro-fastchar-cdn`
- **Lage:** (gemessen 2026-10-10) Compiler gebaut und grün: `src/archivar/giro_fastchar.rs`
  (MAGIC `GIFC`) + `tools/harvest/src/bin/giro_fastchar_compiler.rs`; Endpoint
  `https://lgdc.uml.edu/fastchar/getbest` (HTTP 200, `text/plain`, kein Key); realer Lauf
  `--station JR055 --start 2012-07-02T21:00:00Z --stop 2012-07-03T03:00:00Z` → 24 Records,
  Roundtrip 608 B; Lizenz CC BY-NC-SA 4.0. Workflow + sources.φ-Block von mycelium-292
  geschrieben; Lauf folgt nach dem Push.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium: `giro-fastchar-cdn` Re-Lauf.

### PDS-PPI — `quantity`-Projektion gebaut; Block = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `units.rs` + `pds_ppi_compiler.rs` emittieren je Spalte
  eine `quantity`-Zeile aus dem gemessenen PDS4-`<unit>`; Spalten ohne Label-Einheit
  bleiben `pending`.
- **Blockade:** keine.
- **Braucht:** Mycelium: PDS-PPI-Block (manifest + `quantity`-Zeilen).

### Keogramm ABK — Compiler rückwärts-bounded gebaut; Re-Lauf = Mycelium
- **Status:** eigen (Compiler) | **Bindung:** mycelium (Re-Lauf)
- **Trigger:** Mycelium `keogram-cdn` Re-Lauf
- **Lage:** (gemessen 2026-10-10) `keogram_compiler` sucht bei leerem Fenster die jüngste
  verfügbare Nacht (`--lookback-days`, Default 400). FMI endet `ABK.2604` (2026-04-21,
  jüngste echte Nacht); SGO-Index lebt; der Workflow-Default (gestern) war absent, `exit 1`.
  `keogram-cdn` Re-Lauf `38032885088` dispatched (2026-10-10); `tools-build` `38032882719`.
- **Blockade:** keine.
- **Braucht:** Mycelium: `keogram-cdn` Re-Lauf (greift jetzt die April-Nacht).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf
  lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut,
  `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer
  Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„warum schreibst du hier erst in die blocked sources anstatt direkt an den finalen ort — das ist einfach nur faules compliance theater" | 2026-10-10 | Operator (Session, Mountain 295)
„Offen bleibt in P10: der optionale `<regime>`-Token (wird derzeit benannt verweigert — keine Descriptor-Achse dafür), die Regime-Achse selbst, und die Zeilen-Migration in phi/sources.φ (Mountain)." | 2026-10-10 | Operator (Session, Mountain 295)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–290)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)

## An mycelium

Origin: mountain-297 (2026-10-10).

- **CMB/SPT.** Der `cmb_planck_compiler` lädt die 7.87 GB `full_maps_d1.tar.bz2` jetzt mit
  `SPT_TRANSFER_BOUND_S = 6 h`. Bei gemessenen ~0.47 MB/s braucht der Transfer ~4.65 h;
  euer `cmb-cdn` `timeout-minutes: 240` genügt nicht — hebt ihn auf ≥ 360 und startet den
  Re-Lauf. Kein direkter FITS/Mirror existiert (gemessen 2026-10-10).
- **Keogramm.** `keogram_compiler` sucht bei leerem Fenster die jüngste verfügbare Nacht
  (`--lookback-days`, Default 400) — FMI endet `ABK.2604`. Ein `keogram-cdn` Re-Lauf
  manifestiert jetzt die April-Nacht (bereits dispatched: `38032885088`).
- **iEEG.** Der mountain-296-Auftrag (`phi/harvest.φ` iEEG-Arm + `sources.φ` Block) ist
  gestrichen: er widerstreitet dem registrierten Operator-Wort 2026-10-06
  (`state/zustand/wartend.φ:40`). Kein Mountain-Akt; nur ein neues Operator-Wort öffnet ihn.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade dieses Atoms:
`src/archivar/blinkverse.rs` · `src/archivar/dmap.rs` · `src/archivar/keogram.rs` ·
`src/archivar/weberin_fit.rs` · `src/archivar/bepicolombo.rs` · `src/archivar/ebhis.rs` ·
`src/archivar/swpc_efield.rs` · `tools/register/src/bin/p10_gravity_migrate.rs` ·
`tools/harvest/src/bin/keogram_compiler.rs` · `tools/harvest/src/bin/cmb_planck_compiler.rs` ·
`phi/sources.φ` · `docs/zustand/dropped-legacy-baseline.txt` ·
`docs/handover/handover-2026-10-10-mountain-folge297.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge296.md` (Move).
