<!--
  title: Handover — Mountain-Folge 295 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: f7ba43eee2ee4c4edc04043c712f64be4ae0223d78cdba7eacde23023325832d
  status: live
-->
# Handover — Mountain-Folge 295 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge294.md` (→ `archiv/`).
flash only, kein pro/max. Der Baum trug während der Session fremde uncommittete
Arbeit (river: `src/archivar/parse.rs`, `src/mathematikerin/channel.rs`,
`tools/measure/src/bin/ssb_field_bake.rs`) — unangetastet.

## Burn: open 0.0000 · close 0.1131 (gemessen via session_burn: Taucher general×4 = $0.1131; der Linien-Burn liegt bei Commit noch nicht in der DB)

## Offen (aufgeschlüsselt)

### P10 Register-Physik — `gravity`-Gruppe migriert; `last`-Zeilen + Regime-Achse offen
- **Status:** eigen | **Bindung:** eigen · river
- **Trigger:** die nächste Gruppe (`seismic`)
- **Lage:** (gemessen 2026-10-10) `tools/register/src/bin/p10_gravity_migrate.rs` (neu)
  hat 100 `field … gravity` → `quantity … geometry` und 5 → `quantity … source-parameter`
  umgetaggt (`phi/sources.φ`); 10 gravity-Keep + 2 pending unverändert; `register_sort`
  kanonisch (2708 Blöcke), `cargo check` 0/0. **Riss:** der Klassifikator zählt 104
  geometry, gemessen sind es 100 `field` + 4 `last`; die 4 `last`-Zeilen
  (`hydrosphere_river_stage_m`, `hydrosphere_tide_water_level_m`,
  `gracefo_kbr_range_rate_m_s`, `gracefo_kbr_range_accl_m_s2`) tragen nur eine force —
  kein `quantity`-kind-Arm, Um-Tagung braucht eine Parser-Erweiterung.
- **Blockade:** `last … <kind> …`-Arm fehlt; `parse.rs` trug fremde uncommittete Arbeit.
- **Braucht:** river: `last`-Quantity-Arm in `parse.rs`; dann die 4 Zeilen migrieren.
  Danach Gruppe `seismic` (dann `acoustic`/`diffusion`, `em` zuletzt).

### P10 Regime-Achse — `<regime>`-Token benannt verweigert; kein Descriptor-Slot
- **Status:** eigen | **Bindung:** river
- **Trigger:** eine `Regime`-Achse im `ChannelDescriptor`
- **Lage:** (gemessen 2026-10-10, unverändert river-161) der optionale `<regime>`-Token
  wird bei `field` benannt verweigert (`parse.rs:1249`, „a regime token has no
  descriptor axis yet"), nicht still verworfen. Der vollständige Deskriptor braucht
  Conserved + Boundary + Regime (quasi-statisch/elliptisch vs strahlend).
- **Blockade:** die Regime-Achse existiert nicht.
- **Braucht:** river: `Regime`-Achse + Token-Auflösung (quasistatisch vs strahlend).

### CMB/SPT — `terms` gemessen (LAMBDA no-restrictions); Block = Mycelium
- **Status:** eigen (Register) | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den SPT-`cmap`-Block
- **Lage:** (gemessen 2026-10-10) LAMBDA/AWS-Registry „There are no restrictions on the
  use of this data"; NASA-weit CC0 (`science.data.nasa.gov/about/license`); LAMBDA
  `contact/` erbittet Acknowledgement (keine Bedingung). `product/data_policy.html` 404.
  Tar-interner Member-Prefix `pending`.
- **Blockade:** keine (Mountain-Seite geschlossen).
- **Braucht:** Mycelium: SPT-`cmap`-Block (`terms CC0-1.0 https://lambda.gsfc.nasa.gov/contact/`).

### IRIS — Rohpixel, kein Feld; BUNIT-Riss benannt
- **Status:** eigen (Register) | **Bindung:** mycelium (Block)
- **Trigger:** Rat-Verdikt zum `count`-Token / sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `iris_compiler.rs` liest kein BUNIT, druckt keinen
  Feld-Token; `src/archivar/iris.rs:6` `UNIT="count"` ist eine Baum-Konstante, nicht die
  Quelle (BUNIT `Corrected DN`, mountain-292). Riss: mycelium-288/289 lesen „BUNIT count
  gemessen" — das ist die Konstante. Verdikt: rohe DN ist kein physikalisches Feld →
  keine `field`/`quantity`-Zeile; der `count`-Token ist zu beheben.
- **Blockade:** kein Descriptor für rohe DN.
- **Braucht:** Mycelium: IRIS sources.φ-Zeile + Workflow; die `UNIT`-Konstante klären.

### Redistributions — Lizenzen gemessen; GIRO admissibel (Weberin-Kandidat)
- **Status:** eigen | **Bindung:** eigen · mycelium (Block)
- **Trigger:** GIRO-Arm
- **Lage:** (gemessen 2026-10-10) GIRO DIDBase **CC BY-NC-SA 4.0** (Redistribution
  ausdrücklich erlaubt, `giro.uml.edu/didbase/RulesOfTheRoad.html` 200) → admissibel;
  INTERMAGNET CC BY-NC 4.0 + Bulk-Klausel (bedingt, `intermagnet.org/data_conditions.html`);
  GRDC „No redistribution" (`grdc.bafg.de/about/data_policy/`, 206) → declined; RIPE RIS
  keine explizite Lizenz → nicht admissibel.
- **Blockade:** GIRO-Compiler fehlt.
- **Braucht:** grind-flash: GIRO-DIDBase-Compiler + harvest.φ-Arm + sources.φ-Block; dann
  `weberin kette:station` + `field fof2_mhz` (schließt zugleich den Weberin-Punkt).

### Keogramm ABK — Quelle saisonal; Workflow-Ziel falsch (Mycelium)
- **Status:** eigen | **Bindung:** mycelium
- **Trigger:** Mycelium-Workflow-Fix
- **Lage:** (gemessen 2026-10-10) FMI `ABK.2604` letzte echte Nacht 2026-04-21 (2605/2606
  leer, 2607–2610 404); SGO `emCCD_ABK` Index lebt (Dirs bis 202610, jüngste echte Nacht
  2026-04-21); `keogram-cdn 37995952959` rot auf `ABK.2610` (404). SGO-Keogramm-JPG ist
  Bildprodukt (declined), der ABK-Index läuft über FMI.
- **Blockade:** keine.
- **Braucht:** Mycelium: `keogram-cdn.yml` auf die jüngste verfügbare ABK-Nacht zielen.

### blocked_sources.φ — letzter Parser-Gap: particle-cern ROOT-TTree
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Branch-Decode
- **Lage:** (gemessen 2026-10-09, unverändert) `root.rs` + `cern_root_compiler` lesen
  TFile-Header + 34 TKey; `root::parse_tree` benannte Verweigerung + Gate-Test;
  `dead_sources.φ:496` korrekt.
- **Blockade:** TTree/Branch-Decode.
- **Braucht:** `parse_tree` real dekodieren + `extract.rs`-Arm (oder Descope mit Befund).

### GIC-Estimator — Ground-Truth NOT PASS (Riss, unverändert)
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** eigen
- **Trigger:** Estimator-Reparatur oder Paper-Descope
- **Lage:** (gemessen 2026-10-09, unverändert) TE(X→Y)=2.457e-1, Rückkanal 3.64e-2 >
  fam bei c=0.20 → NOT PASS; Riss im Paper (`:17`, `:842`).
- **Blockade:** keine.
- **Braucht:** Rückkanal unter die Familie nullen, oder die Riss-Zeile bleibt.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent;
  Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### PDS-PPI — `quantity`-Projektion gebaut; Block = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `units.rs` + `pds_ppi_compiler.rs` emittieren je Spalte
  eine `quantity`-Zeile aus dem gemessenen PDS4-`<unit>`; Spalten ohne Label-Einheit
  bleiben `pending`.
- **Blockade:** keine.
- **Braucht:** Mycelium: PDS-PPI-Block (manifest + `quantity`-Zeilen).

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
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–289)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)

## An mycelium

Origin: mountain-295 (2026-10-10).

- **Dispositionen im finalen Register** (der Operator hat das Parken in `blocked_sources.φ`
  gerügt): `cssdc.ac.cn/en` → `decline advert`; `ftp.space.dtu.dk/…/CSES/` →
  `decline redistribution` (DTU-Policy untersagt Weitergabe/DB ohne Zustimmung);
  `www.sgo.fi/pub_asc/emCCD_ABK/` → `decline image` (JPG-Bildprodukt wie FMI). Der
  frühere `pending`-Block cssdc/LEOS ist aus `blocked_sources.φ` entfernt; LEOS steht
  als `decline not-redistributable` (Note aufgefrischt: Login+captcha gemessen, Riss
  Auth-vs-Lizenz benannt).
- **CMB/SPT** — terms gemessen (CC0/no-restrictions, LAMBDA/AWS): der SPT-`cmap`-Block
  kann geschrieben werden (`terms CC0-1.0 https://lambda.gsfc.nasa.gov/contact/`).
- **Medizin-Kante (iEEG)** — Mountain-Seite geschlossen: `eeglab::eeg_from_bin`
  akzeptiert `Samples::Double`. Offen bei euch: `phi/harvest.φ`-iEEG-Arm
  (`--dataset ds004100`, `^sub-HUP…_ieeg\.bin$`) + `sources.φ`-Block.
- **Keogramm** — `keogram-cdn.yml` zielt auf `ABK.2610` (404); FMI endet `ABK.2604`
  (2026-04-21, out-of-season), SGO-Index lebt. Auf die jüngste verfügbare Nacht zielen.

## An river

Origin: mountain-295 (2026-10-10).

- **P10 `gravity`-Migration** ist gelaufen (`field` → `quantity geometry`/`source-parameter`,
  100+5 Zeilen). **Riss:** 4 `last`-Zeilen (river_stage/tide_water/range_rate/range_accl)
  trägt der Klassifikator als geometry, aber der `last`-Arm kennt nur eine force — es
  fehlt ein `last … <kind> …`-Arm (`parse.rs`). Bis dahin bleiben sie `gravity`.
- Der optionale `<regime>`-Token + die `Regime`-Achse bleiben bei euch (P10 braucht den
  vollständigen Deskriptor: Conserved + Boundary + Regime).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade dieses Atoms:
`tools/register/src/bin/p10_gravity_migrate.rs` · `tools/measure/src/eeglab.rs` ·
`phi/sources.φ` · `phi/declined_sources.φ` · `phi/blocked_sources.φ` ·
`docs/handover/handover-2026-10-10-mountain-folge295.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge294.md` (Move).
