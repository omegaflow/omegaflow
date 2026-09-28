<!--
  title: Handover — Mycelium-Folge 201 (2026-09-28)
  session: Mycelium-Folge 201
  class: handover
  date: 2026-09-28
  sha256: caaa0ca145fbc90ce1ebe5679a23684ef8057e670446a2cf245019c7bde5e879
  status: live
-->
# Handover — Mycelium-Folge 201 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge200.md` (→ `archiv/`).

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert).

## Operator-Wort-Register

- Wort | 2026-09-28 | „nein ich möchte dass erst ein echtes survey gemacht wird mit flash tauchern mit harten bandagen einen pro rätseln um zu prüfen was wir haben und was fehlt eine art tabelle" | Quelle: Mycelium-Session 200.
- Wort | 2026-09-28 | „bitte mach erstmal eine archeologie wrum ich die daten überhaupt angefragt habe nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum).
- Wort | 2026-09-28 | „hast du alles bis zur kante gemessen und geplant?" | Quelle: Mycelium-Session 201 (Anlass: `register_lookup --addressed mycelium` nachgeholt, Tafel korrigiert).

## Offen (aufgeschlüsselt)

### Sonden-Ephemeriden — BIG-IEEE-Fix gebaut, Läufe nach Push
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener Push des Fix (`src/archivar/bsp_reader/daf.rs`, dieser Atom).
- **Lage:** (gemessen 2026-09-28 folge201) `src/archivar/bsp_reader/daf.rs` akzeptiert jetzt BIG-IEEE (LOCFMT wählt Endianness, LTL-Pfad byte-identisch, Paritätstest; `cargo check` 0/0) — **uncommittet**; die 5 `*-ephemeris-cdn` scheiterten an `unsupported binary format … only LTL-IEEE` (BIG-Kernel).
- **Blockade:** keine (Fix liegt im Arbeitsbaum).
- **Braucht:** nach Push `gh workflow run galileo-ephemeris-cdn.yml` (+cassini/rosetta/messenger/near).

### spectral-cdn — Lauf dispatched
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36488149432`.
- **Lage:** (gemessen 2026-09-28 folge201) `gh workflow run spectral-cdn.yml` → `36488149432`; `spectra.bin` unter `ncei.noaa.gov` = 404 (folge200).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36488149432`; danach `archive_search --verdict <asset-url>`.

### pds3-/pds4-fixed-width CDN — Läufe dispatched
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Läufe `36488153465` / `36488157946`.
- **Lage:** (gemessen 2026-09-28 folge201) beide Workflows dispatched (Asset-Producer steht, `138b785a5`, `harvest.φ:224/234`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36488153465` + `…57946`; danach Mountain den Zulassungspunkt melden.

### Sonden-Konten CNSA/NSSDC/ISRO/EMM — pending → sources
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Konten per Operator-Hand registriert (gefeuert) — `phi/blocked_sources.φ:373/377/381/385`; Download end-to-end ungemessen.
- **Lage:** (gemessen 2026-09-28 folge201 via `archive_search --verdict`) die vier Portale `moon.bao.ac.cn`, `nssdc.ac.cn`, `pradan.issdc.gov.in/ch2`, `sdc.emiratesmarsmission.ae` sind direct erreichbar (206/200); `phi/blocked_sources.φ:373/377/381/385` = `pending`; das Login-Gate ist das Operator-Browser-Profil.
- **Blockade:** Download end-to-end braucht die angemeldete Operator-Session.
- **Braucht:** `archive_search --verdict <asset-url>` im angemeldeten Profil; danach `pending` → `sources.φ` (Admission = Mountain).

### Sources-Zeilen PDS Chang'e / Zenodo / KASI / ShadowCam / KARI / DARTS / PSA
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `docs/SOURCE_PORT.md`-Pass.
- **Lage:** (gemessen 2026-09-28 folge201) alle Endpunkte direct erreichbar (206/200); KMTNet-MOC = **FITS** (`--sniff` 1 848 960 B, sha256 `f34ff61f…`); ESA PSA = TAP/ADQL; übrige Format unread.
- **Blockade:** `format`/Verdikt = Mountain; `url`/`origin`/`compiler` = Mycelium.
- **Braucht:** je Endpunkt `archive_search --verdict` + Format messen; dann Zeile setzen.

### Register-Kandidaten `index.φ` (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `docs/SOURCE_PORT.md`-§5.4-Merge.
- **Lage:** (gemessen 2026-09-28 folge201, nachgezählt durch flash-Taucher) 7/8 Konversionen stehen in `phi/pipeline/stage/` (33 Dateien); nur der §5.4-Merge fehlt; `oai_arxiv` braucht eine Disposition. **Note-Drift (korrigiert):** die Note steht auf `index.φ:38` (nicht :37) und sagt „776 Blöcke, 6947 Z." gegen gemessen **777 `^url `-Blöcke / 7010 Z.** (die 782 der ersten Zählung waren eine Substring-Zählung inkl. 5 Kommentarzeilen: `:1193/1866/1963/4645/6382`) → Drift **+1 Block/+63 Z.**. **Weiterer Note-Riss:** die Quellen-Zahl „825" und „49 ohne url" stimmen nicht (Quelle: 824 Blöcke, alle mit url).
- **Blockade:** keine.
- **Braucht:** die 8 Stage-Ergebnisse in die Register mergen (Register-Pen) + `oai_arxiv` disposzieren.

### `format vlde` — Source-Zeile fehlt (Reader/Compiler/Asset stehen)
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** nächster `phi/sources.φ`-Register-Pass / Mountain-Admission.
- **Lage:** (gemessen 2026-09-28 folge201; folge201-„Träger fehlt" durch flash-Taucher **falsifiziert**) folge200s Prämisse **stimmt**: Reader `src/archivar/vlies.rs:3` (`MAGIC b"VLDE"`), Fetch `src/archivar/main_flow.rs:4614` (`format == "vlde"`), Compiler `tools/harvest/src/bin/vlies_density_compiler.rs`, Probe `tools/measure/src/bin/vlies_density_probe.rs`, Workflow `.github/workflows/vlies-density-cdn.yml:27`; Asset `…/releases/download/ssd.jpl.nasa.gov/vlies_density.vlde` → `--verdict` **206**. Die `format vlde`-Zeile in `phi/sources.φ` fehlt vollständig (`sgrep -i vlies phi/` = 0 — der Suchbereich war `phi/`, nicht `tools/`/`src/`). **Nebenriss:** der Compiler lädt auf den Family-Tag `ssd.jpl.nasa.gov-vlies` (`:348`) → 404; das Asset liegt auf der capped Release `ssd.jpl.nasa.gov` (#6).
- **Blockade:** `format`/`field`/`ttl`/Admission = Mountain; `url`/`origin`/`compiler`/Tag = Mycelium.
- **Braucht:** Mountain admittiert (`format vlde`, `field`, `ttl`), dann Mycelium `url`/`origin`/`compiler` — siehe `## An mountain`.

### DEMETER — CDPP server-seitig defekt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** quellenseitige CDPP-Reparatur / neuer Order-Versuch — `phi/blocked_sources.φ:89-91`.
- **Lage:** (gemessen 2026-09-28 folge201) Register-Note `phi/blocked_sources.φ:89-91` geschärft: server-seitig (`availableFilesCount 0`, `filesInErrorCount 1000`), kein ip-Block, UA-Riss mountain 403/403 vs sensory 403/202; Wait `state/zustand/wartend.φ:4/5` (Aufnehmer sensory).
- **Blockade:** quellenseitige Reparatur.
- **Braucht:** kein Code-Schritt; CDPP-Reparatur abwarten.

### ODF-Coverage der Flyby-Fenster — Fenster fehlt
- **Status:** wartend | **Bindung:** eigen (mit sensory)
- **Trigger:** neue Pre-Flyby-ODF-/Kernel-Quelle — `phi/sources.φ` galileo_odf:9764 / messenger_odf:9810 / cassini_odf:8977 / rosetta_odf:8947.
- **Lage:** (gemessen 2026-09-28 folge201, durch flash-Taucher bestätigt) **keines** der vier ODF-Assets trägt das Flyby-Fenster auf Record-Ebene: galileo 1996-07→1997-12 (Flybys 1990/1992), messenger 2007-06→2015-04 (Flyby 2005-08), cassini 2002-06→2016-12 (1999-08), rosetta Shard1 2004-03→2015-05 mit Record-Loch (CVP2-0010 START 2004-09-11T23:30 → CR2-0012 START 2005-04-06T13:21:24; Flyby 2005-03-04). **Zusatz-Riss (nicht geglättet):** `state/zustand/external-state.md:34` (Rosetta ODF) nennt „6 Shards in `phi/sources.φ`" und Shard1 „82 063 760 B"; `phi/sources.φ` trägt **3** Shard-Blöcke, `harvest.φ:251`/`frame_registry.φ:71-76` nennen **6**, der Taucher misst Shard1 `1 073 741 816 B` — Shard-Zahl und Shard1-Größe divergieren zwischen den Zeugen.
- **Blockade:** die registrierten ODFs sind Post-Flyby-Missiondaten.
- **Braucht:** Pre-Flyby-ODF-Quelle finden oder den Befund als Verdikt tragen (Mountain).

### Kuprat — witness + tag
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission / Operator-Wort zur Zeugenart.
- **Lage:** (gemessen 2026-09-28) vier Kanäle (RIXS spin/charge, EELS, SRD62) auf der CDN, `sgrep -i kuprat phi/witnesses.φ` = 0; Tag-Drift `ssd.jpl.nasa.gov` in Probes/Workflows.
- **Blockade:** `witness kuprat`/`format`/`field` = Mountain; `url`/`origin`/Tag = Mycelium.
- **Braucht:** Mycelium: Tag-Drift heilen (nach Re-Materialisation); Mountain: `witness kuprat` + `format`/`field`.

### Release-Namespace-Migration — `ssd.jpl.nasa.gov`-Legacy-Tag
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort / nächster CDN-Migrations-Atom.
- **Lage:** (gemessen 2026-09-28 folge201 via `sgrep -i ssd.jpl.nasa.gov tools/ .github/workflows/`) Inventar der hartkodierten Legacy-Tag-Sites steht; Compiler laden schon unter Produzenten-Tag, physisch liegt das Asset unter `ssd.jpl`.
- **Blockade:** systemische Migration (Re-Materialisation + Probe-/Workflow-Flip), kein Spot-Fix.
- **Braucht:** pro Asset Re-Materialisation unter Produzenten-Tag, danach Probe-Fetch/Workflow-Download flippen.

### Register-`url`/`origin`-Drift (twomass_psc, jwst_spectra, LLNL/S40RTS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `phi/sources.φ`-Register-Pass.
- **Lage:** (gemessen 2026-09-28 folge201) `sgrep -i twomass_psc phi/sources.φ` = 0 (nicht in `sources.φ`); `jwst_spectra` steht `phi/sources.φ:9208-9213`; `LLNL_G3D_JPS/S40RTS volume.bin` trägt eine falsche `origin`-Direktive.
- **Blockade:** keine.
- **Braucht:** die Drift-Zeilen im Register korrigieren (`url`/`origin` = Mycelium).

### Rätsel-Bestand (Survey) — Träger + Verdikt-Trägerschaft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste Rätsel-Messung (`docs/surveys/survey-raetsel-bestand.md`).
- **Lage:** (gemessen 2026-09-28 folge200) das Survey steht (12 Nadeln + 2 Blätter + Kuprat); das Verdikt von 10 der 15 Rätsel lebt nur als Paper-/Archiv-Prosa (`sgrep` je 0).
- **Blockade:** keine.
- **Braucht:** je Rätsel den `Nächster Schritt` dispatchen; Ⅹ descope-Befund belegen oder `pending`; Verdikt-Zeilen = Mountain.

### Rätsel-Fehlkanäle — CDN-Materialisation
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission der Fehlkanäle (`## An mountain`).
- **Lage:** (gemessen 2026-09-28 folge200) Kanäle registriert, neuer Producer/Verdikt fehlt → Compiler/CDN-Weg fehlt.
- **Blockade:** Mountain-Admission.
- **Braucht:** je neuem Kanal Compiler + CDN-Workflow (Muster `nvss-cdn.yml`), dann `gh workflow run <kanal>-cdn.yml`.

### DAS2 Iowa + Occultation-DB UTFPR — nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission (`phi/blocked_sources.φ:363-368`).
- **Lage:** (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), kein `sources.φ`-Block/Workflow.
- **Blockade:** Admission-Verdikt.
- **Braucht:** nach Admission `format`/`tag`/`pattern` + Workflow.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-28 folge198) Job `hinet`: `cont status never read` (8×), exit 1.
- **Blockade:** quellenseitige Readiness.
- **Braucht:** erneuter Lauf; kein Code-Schritt.

### ci_watchdog — Matcher
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf.
- **Lage:** (gemessen 2026-09-28) Trigger nicht gefeuert.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### D5-Orphan-Residuum — Röhren-Asset-CDN-Weg
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:383` §14.4 Punkt 4).
- **Lage:** (gemessen 2026-09-28) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02 / 2027-04-01
- **Trigger:** 2026-10-02 (Routen, CSES) / 2026-12-02 (NOIRLab/Gaia-DR4) / 2027-04-01 (BepiColombo).
- **Lage:** (gemessen 2026-09-28) `pithia` backend-tot; `lasair` direct absent/proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## An mountain

- **Rätsel-Fehlkanäle — Quellen + Admission + Parser** | (gemessen 2026-09-28 folge200) je fehlender Ader Producer + Verdikt + Parser: Ⅳ Swarm-TEC (`vires.services` HAPI), Ⅷ tiefste z≳10, Ⅻ B-Moden, Ⅺ Placebo HRV, ENSO Becken-Windfeld, GIC Mäntsälä-dB/dt. Karte: `docs/surveys/survey-raetsel-bestand.md`.
  Origin: mycelium-folge200
- **Rätsel-Verdikt-Zeilen (10)** | (gemessen 2026-09-28 folge200) Verdikt von 10/15 nur als Paper-/Archiv-Prosa (`sgrep` je 0). Feder: Verdikt-Zeilen in `phi/`.
  Origin: mycelium-folge200
- **Ⅹ Kugelblitz — descope registerlos** | (gemessen 2026-09-28 folge200) `descoped` nur als Prosa `docs/concepts/kybernetische-astrophysik.md:294-300` ohne Befund-Token. Braucht: gemessener Befund oder `pending`.
  Origin: mycelium-folge200
- **`format vlde` — Admission + `field`/`ttl`** | (gemessen 2026-09-28 folge201) Reader `src/archivar/vlies.rs:3` + Compiler `vlies_density_compiler.rs` + Workflow `vlies-density-cdn.yml` stehen, Asset `vlies_density.vlde` 206; die `phi/sources.φ`-Zeile fehlt vollständig. Feder: `format vlde`/`field`/`ttl` + Admission; Mycelium setzt danach `url`/`origin`/`compiler`.
  Origin: mycelium-folge201
- **ODF-Flyby-Fenster fehlt** | (gemessen 2026-09-28 folge201) keines der vier ODF-Assets trägt das Flyby-Fenster (galileo/messenger/cassini Post-Flyby, rosetta mit Record-Loch). Feder: Befund/Quelle.
  Origin: mycelium-folge201

## An river

- **Rätsel-Rechen-/Probe-Lücken (Code)** | (gemessen 2026-09-28 folge200) je Rätsel fehlt die Rechnung: Ⅰ per-Voxel-Jeans-Engine, Ⅶ `max(0,|Δt|−d/c)`-Fold, Ⅸ FRB-Paar (`frb_blatt_probe.rs:295`), ENSO TE-Probe. Karte: `docs/surveys/survey-raetsel-bestand.md`.
  Origin: mycelium-folge200

## An future

- **NSE-Redistribution + Dank** | (gemessen 2026-09-28 folge199) Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) um die Lizenz-/Redistributionsfrage erweitern; Akt = Operator-Hand. Bis zum Einverständnis kein CDN.
  Origin: mycelium-folge199
- **ENSO-Zuschnitt** | (gemessen 2026-09-28 folge200) `state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`. Braucht: Operator-Wort.
  Origin: mycelium-folge200
- **Kuprat-Zeugenart** | (gemessen 2026-09-28 folge200) `phi/witnesses.φ` trägt nur `s2-direction`, `gestalt`; Kuprat-Zeugenart fehlt. Braucht: Operator-Wort.
  Origin: mycelium-folge200
- **`gic-causal-driver.md` DOI-Minting** | (gemessen 2026-09-28 folge201) `docs/paper/gic-causal-driver.md:531/538` DOIs `pending`. Der Mint ist ein Dritter-Akt → Operator-Hand.
  Origin: mycelium-folge201

## An sensory

- **`reference_verify.rs` Test-Pfad defekt** | (gemessen 2026-09-28 folge201) `tools/register/src/bin/reference_verify.rs:332` importiert `extract_arxiv_ids` ohne `extract_dois` (von `:371` genutzt) — `cargo check -p omegaflow-register --tests` bricht; committet `5cdf92677`. Feder: sensory (Urheber).
  Origin: mycelium-folge201
