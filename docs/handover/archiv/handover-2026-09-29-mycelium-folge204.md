<!--
  title: Handover — Mycelium-Folge 204 (2026-09-29)
  session: Mycelium-Folge 204
  class: handover
  date: 2026-09-29
  sha256: d84e4f29f6f39c642f82b6fd9332688dd70a3a7e0d36bb8adbb6fd545ad8271c
  status: live
-->
# Handover — Mycelium-Folge 204 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-29-mycelium-folge203.md` (→ `archiv/`).
Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert).

Dieser Atom hat gemessen statt getafelt: Gründe der Stauung wurden gemessen
(`register_lookup --stale --persist 3` = **0** — der Wächter liest sauber, weil jede
Runde die `Lage` neu stempele; ein Punkt kann wandern, ohne „stale" zu werden), die
Punkte dispatchen lassen und **gebaut, was der Baum wirklich offen trug**. Neue
Messung: die meisten als „pending/fehlend" getragenen Punkte tragen bereits Bin/Wf/
Arm — die Realität wanderte als falscher Stand mit.

## Operator-Wort-Register

- Wort | 2026-09-29 | „warum hast du in der letzten runde nur so wenig geschafft? ich meine du sagst doch du kannst die punkte abarbeiten warum tust du es dann nicht oder verschleppst von runde zu runde" | Quelle: Mycelium-Session 204.
- Wort | 2026-09-29 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent" | Quelle: Mycelium-Session 204 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „kannst du nicht mehr abarbeiten?" | Quelle: Mycelium-Session 203.
- Wort | 2026-09-28 | „nein ich möchte dass erst ein echtes survey gemacht wird mit flash tauchern mit harten bandagen einen pro rätseln um zu prüfen was wir haben und was fehlt eine art tabelle" | Quelle: Mycelium-Session 200.
- Wort | 2026-09-28 | „bitte mach erstmal eine archeologie wrum ich die daten überhaupt angefragt habe nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum).
- Wort | 2026-09-28 | „hast du alles bis zur kante gemessen und geplant?" | Quelle: Mycelium-Session 201.

## In diesem Atom gemessen und geheilt (kein Punkt, git trägt es)

- **`url`/`origin`-Drift in `phi/sources.φ` geheilt** (gemessen 2026-09-29, grind-flash
  via `archive_search --verdict`): `first14.json`/`nvss.json`/`curated48_spectra.bin`
  von 404-Tag (`tapvizier.cds.unistra.fr` / `exoplanetarchive.ipac.caltech.edu`) →
  `ssd.jpl.nasa.gov` (206); `LLNL_G3D_JPS.nc` / `S40RTS.nc` Platzhalter `<name>.nc` →
  echter Name. `sources.φ:2416` (`spectra.bin`) = 206, keine Drift. Nur Myceliums
  Föderalseite (`url`/`origin`), Verdiktzeilen unberührt.
- **Planetary-Arme widerlegt**: Phobos/Vega/Hayabusa (`pds3_fixed_width_compiler` /
  `pds4_fixed_width_compiler` + `*-cdn.yml`, Asset in `phi/harvest.φ`, Läufe success)
  und GOSAT (`gosat-cdn.yml`, `fae4a5081`) sind **gebaut**.
- **Kuprat-Quellmagics widerlegt**: `RIXS/RIXC/EELS/SRD6` stehen im Quellbaum
  (`0f9bfdab3`); `src/archivar/witness.rs:26` stimmt überein.
- **exzellenz-Tooling widerlegt**: `tools/science/src/bin/export_latex.rs` + Spec +
  `paper-check.yml:37` existieren; es fehlt allein die PDF-Engine (extern).
- **§5.4-Riss geheilt**: die zitierte Regel `docs/SOURCE_PORT.md §5.4` existiert nicht;
  real ist `docs/SOURCE_PORT.md:134–137` (§5 Schritt 4). Die Fehlzitate liegen nur in
  archivierten, geschlossenen Übergaben (nicht angefasst) — die lebende Zitation trägt
  dieses Register.

## Offen (aufgeschlüsselt)

### `phi/sources.φ` — Rest-Migration + `twomass_psc`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Verdikt für `twomass_psc` + nächster Migrations-Atom.
- **Lage:** (gemessen 2026-09-29 folge204) 5 `url`/`origin`-Zeilen geheilt. Offen:
  `twomass_psc` fehlt in `phi/sources.φ` (Kandidat
  `phi/pipeline/stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`); systemische
  Legacy-Tag-Migration (`wartend.φ:29` legacy-cdn-ssd).
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `twomass_psc`-Zeile (Mountain); Migration je Asset.

### ENSO-Manifestation — Zeile steht, Compiler offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster CDN-Atom (`ersstv5-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 river-folge63 + mountain-folge202) die Zeile
  `ersstv5_nino34` ist in `phi/sources.φ` (river-folge63: committet; mountain-folge202
  nennt `format ersstv5_nino34` / `origin …/nceiErsstv5` /
  `compiler ersstv5_compiler.rs`). `--ci-mode`-Lauf steht aus.
- **Blockade:** keine.
- **Braucht:** `ersstv5_compiler --ci-mode` fahren, Release-Asset
  `coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin` manifestieren.

### Sources-Zeilen-Endpunkte (future151/152) + Tianwen1-MoRIC-Riss
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** nächster `docs/SOURCE_PORT.md`-Register-Pass (§5 Schritt 4).
- **Lage:** (gemessen 2026-09-29 folge202 via `--verdict`/`--sniff`) alle erreichbar bis
  auf den Riss `alasky.cds.unistra.fr/Tianwen1-MoRIC/` = **404**.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** je Endpunkt Format (Mountain); MoRIC-Pfad neu messen oder verwerfen.
- **Träger:** https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/ (carriert phi/blocked_sources.φ:430).

### index.φ — 7 Stage-Merges (Riss geheilt, Merge offen)
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Merge-Pass nach `docs/SOURCE_PORT.md:134–137` §5 Schritt 4.
- **Lage:** (gemessen 2026-09-29 folge204) die Zitation ist geheilt (s. „gemessen und
  geheilt"); `phi/pipeline/index.φ:75` `oai_arxiv` → `index`. Die 7 Stage-Ergebnisse
  stehen in `phi/pipeline/stage/`; der Merge berührt Mountain-Verdiktregister.
- **Blockade:** Mountain-Feder.
- **Braucht:** Merge mit Mountain-Freigabe im selben Atom.

### Planetary/Kleinkörper — Körper-Registrierung `at halley`/`at itokawa`
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Körper-Registrierung (`frame_registry.φ`, Komet `1P` / `25143`).
- **Lage:** (gemessen 2026-09-29 folge204, grind-pro) `at halley` (1P) und
  `at itokawa` (25143) = **0 Treffer in `phi/`**; die `at`-Direktive für Vega/Hayabusa
  ist bis dahin nicht schreibbar. ExoMars/Kaguya/Chandrayaan ohne Live-Sample;
  Akatsuki dir-GET **503**.
- **Blockade:** Körper-/Frame-Registrierung fehlt.
- **Braucht:** `## An mountain`.

### Kuprat — Re-Manifestation + fehlender `--eels`-Step
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster CDN-Atom (`cuprate-cdn.yml` / `srd62-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 folge204, grind-pro) Quellmagics migriert; offen: die
  CDN trägt alte Bins (`cuprate-cdn` / `srd62-cdn` dispatcht nicht neu);
  `cuprate-cdn.yml` hat keinen `--eels`-Schritt (EELS → `eels_acoustic.bin` bleibt
  unre-manifestiert); Tag-Riss (Probes `ssd.jpl.nasa.gov` vs Compiler-Upload
  `crystallography.net`/`srdata.nist.gov`).
- **Blockade:** EELS-`.mat`-Quelle ungemessen.
- **Braucht:** EELS-Quelle messen, dann Step; `cuprate-cdn`/`srd62-cdn` nach Push
  dispatch.

### HiPS-PNG MoRIC — Tree-Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Arm-Bau (`hips_png_compiler.rs` / `hips-png-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 folge204, grind-pro) `src/archivar/hips.rs` +
  `hips_png_compiler.rs` können **nur eine Kachel**; der Tree 12·4⁷ = 196 608 Kacheln
  hat keinen Arm (kein Norder-Walk, kein Tree-Index, `CAPPED_RELEASE` 1000 ≪ Tree).
  Keine Workflow-Direktive gebaut — sie würde den Baum still unter-manifestieren.
- **Blockade:** Tree-Enumerator/Index fehlt.
- **Braucht:** Tree-Arm + Dir-Sharding + Manifest, dann `hips-png-cdn.yml`.

### `format vlde` — Source-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`format vlde`, `field`, `ttl`).
- **Lage:** (gemessen 2026-09-29 mountain-folge201) exakte Direktiven geliefert;
  Reader/Compiler/Wf stehen; Asset `vlies_density.vlde` 206. Tag-Riss: Workflow-Tag
  `ssd.jpl.nasa.gov` vs Compiler-Tag `-vlies`.
- **Blockade:** Admission = Mountain.
- **Braucht:** nach Admission `url`/`origin`/`compiler` + Tag heilen.

### spectral/pds3/pds4-CDN — Zulassung
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/sources.φ`).
- **Lage:** (gemessen 2026-09-29) drei Läufe success (Idempotenz-Skips, „already
  present"). Assets liegen unter `ncei.noaa.gov`, `pds-smallbodies.astro.umd.edu`,
  `sbnarchive.psi.edu`.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `## An mountain` — Zulassung + `sources.φ`-Zeilen.

### EELS/`witness` — siehe Kuprat.

### ODF-Coverage der Flyby-Fenster — Shard-Riss
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt (`sources.φ:8947-8975`).
- **Lage:** (gemessen 2026-09-29 folge202) Galileo PPI-Annex TDF deckt Earth-1;
  MESSENGER/Cassini/Rosetta nicht gefunden. Shard-Riss: `sources.φ:8947-8975` = 3
  Rosetta-ODF-Shards; `harvest.φ:251` + `frame_registry.φ:71-76` = 6;
  `external-state.md:34` Shard1 `82 063 760 B` vs Taucher `1 073 741 816 B`.
- **Blockade:** Verdikt/Registrierung = Mountain.
- **Braucht:** `## An mountain`.

### Quellenseitige Waits (in `state/zustand/wartend.φ` geführt)
- **DEMETER** CDPP defekt (`blocked_sources.φ:89-91` noch `pending`; Wait `wartend.φ:4`,
  Aufnehmer sensory).
- **hinet-cdn** `36498426236` rot: `cont status never read Available` (8 Re-Requests,
  exit 1) — quellenseitige Readiness.
- **GOSAT** `36283215548` rot: Server-Überlauf/Leer („exceeds 3000", „search result is 0");
  Empty/Overflow-Fix 2026-09-27 — Re-Dispatch nach Push.
- **[redacted]** Wait (`wartend.φ:8`): Einverständnis zur Redistribution; Send =
  Operator-Hand.

### Orphan-Docs-Zensus — 3 Docs, Zuordnung gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Namenträger setzen oder gemessen `descoped`.
- **Lage:** (gemessen 2026-09-29 folge204, general-Taucher) `register_lookup
  --orphan-docs` = **3**: `docs/concepts/kybernetische-astrophysik.md` (9 Marker — 7
  substanziell, 2 Scanner-Fehltreffer `wartet`; `:282` bei `:299` bereits `descoped`),
  `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (2 — beide
  Scanner-Fehltreffer `erwartete`, der Silence-Map-Punkt ist gebaut), 
  `docs/surveys/survey-fortschritt.md` (1, working-tree-modifiziert — fremd).
- **Blockade:** keine.
- **Braucht:** Träger Mountain für die Konzept-Essays; die 3 Survey-/Konzept-Marker
  redaktionell entschärfen oder Trage-Zeilen setzen.

### exzellenz-konzept — PDF-Rendering-Stufe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** LaTeX-Engine im Runner (`paper-check.yml`).
- **Lage:** (gemessen 2026-09-29 folge204) `export_latex.rs` + Spec + `paper-check.yml:37`
  existieren; keine TeX-Engine auf der Maschine (`tlmgr|tectonic|pdflatex|…` leer).
- **Blockade:** externe LaTeX-Distribution.
- **Braucht:** Engine provisionieren (CI), dann `tectonic`; PDF-Nummern-Gate.

### `ci_watchdog` — Matcher
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf.
- **Lage:** (gemessen 2026-09-29 folge204) Trigger nicht gefeuert.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4).
- **Lage:** (gemessen 2026-09-29 folge204) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer-Bin/Register/Workflow fehlt (`docs/concepts/zeugnis.md:383`).
- **Braucht:** kein Schritt zur Kante.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02 / 2027-04-01
- **Trigger:** `superdarn-af68c4f1`/CSES `laic-cses` (10-02) / NOIRLAB/Gaia-DR4
  (12-02) / BepiColombo `bepicolombo-more` (2027-04-01).
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## An mountain

- **Körper-Registrierung `at halley`/`at itokawa`** | (gemessen 2026-09-29 folge204) 1P + 25143 = 0 Treffer in `phi/`; die Vega-/Hayabusa-`at`-Direktive steht bis dahin.
  Origin: mycelium-folge204
- **Rosetta-Granule** | (gemessen 2026-09-29) `rosetta 36491843356` rot: `naif -226 carries no granule`; Kernel-Set/Typ-18.
  Origin: mycelium-folge203
- **ODF-Flyby-Verdikt + Shard-Riss** | (gemessen 2026-09-29) Galileo deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden.
  Origin: mycelium-folge203
- **spectral/pds3/pds4 + `format vlde` + fixe Tabellen** | (gemessen 2026-09-29) Läufe/Arme stehen, `format`/`field`/`ttl` + `sources.φ`-Zeilen offen.
  Origin: mycelium-folge203
- **DEMETER-Klasse schärfen** | (gemessen 2026-09-28 sensory) `blocked_sources.φ:89-91` noch `pending` mit Alt-note.
  Origin: sensory-folge204
- **`twomass_psc`-Verdikt** | (gemessen 2026-09-29 folge204) fehlt in `sources.φ`; Kandidat `stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`.
  Origin: mycelium-folge204

## An future

- **NSE-Redistribution + Dank** | (gemessen 2026-09-28 folge199) Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) um die Lizenzfrage erweitern; Akt = Operator-Hand.
  Origin: mycelium-folge199
- **ENSO-Zuschnitt** | (gemessen 2026-09-28) `state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`.
  Origin: mycelium-folge200
- **Kuprat-Zeugenart** | (gemessen 2026-09-28) `phi/witnesses.φ` trägt Kuprat-Zeugenart nicht.
  Origin: mycelium-folge200
- **`gic-causal-driver.md` DOI-Minting** | (gemessen 2026-09-28) `docs/paper/gic-causal-driver.md:531/538` DOIs `pending`. Mint = Dritter-Akt.
  Origin: mycelium-folge201
- **Stauungs-Wort des Operators** | (gemessen 2026-09-29 folge204) „warum … nur so wenig geschafft" — die Antwort ist gebaut (dieser Atom dispatcht + heilt), kein neuer Punkt.
  Origin: mycelium-folge204

## An sensory

- **`reference_verify.rs` Test-Pfad** | (gemessen 2026-09-28 folge201) `tools/register/src/bin/reference_verify.rs:332` importiert `extract_arxiv_ids` ohne `extract_dois`; `cargo check --tests` bricht. Feder: sensory.
  Origin: mycelium-folge201

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
