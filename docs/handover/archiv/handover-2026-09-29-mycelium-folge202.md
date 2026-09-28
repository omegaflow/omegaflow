<!--
  title: Handover — Mycelium-Folge 202 (2026-09-29)
  session: Mycelium-Folge 202
  class: handover
  date: 2026-09-29
  sha256: 076300cb6947b95977b18d6f377baae49e3f366adfd7ef9e31df008f82ef160a
  status: live
-->
# Handover — Mycelium-Folge 202 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge201.md` (→ `archiv/`).

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert).

## Operator-Wort-Register

- Wort | 2026-09-28 | „nein ich möchte dass erst ein echtes survey gemacht wird mit flash tauchern mit harten bandagen einen pro rätseln um zu prüfen was wir haben und was fehlt eine art tabelle" | Quelle: Mycelium-Session 200.
- Wort | 2026-09-28 | „bitte mach erstmal eine archeologie wrum ich die daten überhaupt angefragt habe nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum).
- Wort | 2026-09-28 | „hast du alles bis zur kante gemessen und geplant?" | Quelle: Mycelium-Session 201.
- Wort | 2026-09-29 | „hast du alle eigenen punkte bis zur kante geplant?" | Quelle: Mycelium-Session 202 (Anlass: erste Tafel war handover-only; Register/Post/`## An mycelium` nachgeholt).

## Offen (aufgeschlüsselt)

### Sonden-Ephemeriden — URL geheilt, Rosetta-Granule offen
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Re-Dispatch `messenger-ephemeris-cdn.yml`/`near-ephemeris-cdn.yml` nach Push.
- **Lage:** (gemessen 2026-09-29 folge202 via `ci_manage`) der BIG-IEEE-Fix wirkt: `galileo 36491836382` + `cassini 36491839712` = **success**. `messenger 36491847094` + `near 36491850837` scheiterten an `curl (22) 404` — die SPK-URLs fehlten den PDS-Präfix. Die echten Dateien liegen unter `…/pub/naif/pds/data/mess-e_v_h-spice-6-v1.0/messsp_1000/data/spk/msgr_040803_120516_140823_od268sc_0.bsp` bzw. `…/pds/data/near-a-spice-6-v1.0/nearsp_1000/data/spk/near_cruise_nav_v1.bsp` (`--verdict`: 206 direct + Wayback). Beide Workflow-URLs in diesem Atom geheilt (committet mit `/commit`). `rosetta 36491843356` = **failure**: `naif -226 carries no granule across the given kernels — the anchor stays unwritten`.
- **Blockade:** Rosetta = Code/Granule-Pfad (Mountain).
- **Braucht:** nach Push `gh workflow run messenger-ephemeris-cdn.yml` + `near-ephemeris-cdn.yml`; Rosetta-Kernel-/Typ-18-Arm → `## An mountain`.

### spectral-/pds3-/pds4-CDN — Läufe grün, Zulassung melden
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission der drei Assets (`spectral-cdn.yml`/`pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 folge202 via `ci_manage`) `spectral-cdn 36488149432`, `pds3-fixed-width 36488153465`, `pds4-fixed-width 36488157946` = **success** — Assets auf der CDN.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `## An mountain` — Zulassung + `sources.φ`-Zeilen.

### Sonden-Konten CNSA/NSSDC/ISRO/EMM — pending, ISRO-url-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Konten per Operator-Hand registriert (gefeuert) — `phi/blocked_sources.φ:373/377/381/385`.
- **Lage:** (gemessen 2026-09-29 folge202) vier Portale direct erreichbar (206/200); Klassen `pending`. **Riss:** `phi/blocked_sources.φ:382` führt `url https://pradan.issdc.gov.in/ch2`, die note misst `chmapbrowse 200 (mrbrowse 404)` — `url` ist Mycelium-Feder, die note Morgen-Feder (Mountain).
- **Blockade:** Download end-to-end braucht das angemeldete Operator-Browser-Profil; ISRO-`url` ungemessen welcher Pfad 200 liefert.
- **Braucht:** `archive_search --verdict <asset-url>` im angemeldeten Profil; ISRO-`url`-Riss per Messung heilen.

### Sources-Zeilen — Endpunkte gemessen (future-folge151-Liste)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `docs/SOURCE_PORT.md`-Register-Pass.
- **Lage:** (gemessen 2026-09-29 folge202 via `archive_search --verdict`/`--sniff`) alle erreichbar: Chang'e-MRM-Bundle (200, Verzeichnis), `zenodo.org/records/15812343` (200 HTML), `pds.wh.sdu.edu.cn` (206), `data.kasi.re.kr` (200), KMTNet-MOC **FITS** 1 848 960 B sha256 `f34ff61f…`, `pds.shadowcam.im-ldi.com/derived/` (200), `www.kari.re.kr/kpds/` (200), DARTS PDS4-Root (206), ESA PSA TAP (200). **Riss:** `alasky.cds.unistra.fr/Tianwen1-MoRIC/` = **404** unter diesem Namen.
- **Blockade:** `format`/Verdikt = Mountain; `url`/`origin`/`compiler` = Mycelium.
- **Braucht:** je Endpunkt Format festlegen; Tianwen1-MoRIC-Pfad neu messen oder verwerfen.
- **Träger:** `phi/blocked_sources.φ:410` (`https://pds-geosciences.wustl.edu/Lunar/urn-nasa-pds-chang_e_microwave_processed/`), `:414` (`https://zenodo.org/api/records/15812343`), `:435` (`http://pds.wh.sdu.edu.cn`).

### Register-Kandidaten `index.φ` (8) — 7/8 Stage, `oai_arxiv`-Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `docs/SOURCE_PORT.md`-§5.4-Merge.
- **Lage:** (gemessen 2026-09-29 folge202, flash-Taucher) 7/8 Stage-Ergebnisse stehen in `phi/pipeline/stage/` (nur `oai_arxiv` fehlt); alle 8 Zeilen noch `verifiziert`. `oai_arxiv.φ` = 1300-Zeilen-Inventar, 0 `url`-Blöcke — kein Block-Draft.
- **Blockade:** keine.
- **Braucht:** die 7 Stage-Ergebnisse in die Register mergen; `oai_arxiv` auf `index`/`descoped`-Disposition (kein `stage/`-Ergebnis erzeugbar).

### `format vlde` — Source-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`format vlde`, `field`, `ttl`).
- **Lage:** (gemessen 2026-09-28 folge201) Reader `src/archivar/vlies.rs:3`, Fetch `main_flow.rs:4614`, Compiler `vlies_density_compiler.rs`, Workflow `vlies-density-cdn.yml:27` stehen; Asset `vlies_density.vlde` 206. Die `format vlde`-Zeile in `phi/sources.φ` fehlt. Tag-Riss: Compiler lädt auf Family-Tag `ssd.jpl.nasa.gov-vlies` (`:348`) → 404, Asset liegt auf `ssd.jpl.nasa.gov` (#6).
- **Blockade:** `format`/`field`/`ttl`/Admission = Mountain.
- **Braucht:** Mountain admittiert → Mycelium `url`/`origin`/`compiler` + Tag heilt.

### DEMETER — CDPP server-seitig defekt (Klasse ungeschärft)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** quellenseitige CDPP-Reparatur / neuer Order-Versuch — `phi/blocked_sources.φ:89-91`.
- **Lage:** (gemessen 2026-09-29) Register-Note `:89` trägt noch `pending` mit Alt-note; Wait `state/zustand/wartend.φ:4/5` (Aufnehmer sensory); Order 18400 `availableFilesCount 0`, `filesInErrorCount 1000`, Ablauf 2026-10-05.
- **Blockade:** quellenseitige Reparatur; Klasse „CDPP-Generierung defekt — wartend" ungeschärft (Mountain-Note).
- **Braucht:** `## An mountain` — Klasse schärfen; kein Code-Schritt.

### ODF-Coverage der Flyby-Fenster — Galileo-Treffer, 3 x nicht gefunden
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt / neue Pre-Flyby-Quelle (`phi/sources.φ` galileo_odf:9764 / messenger_odf:9810 / cassini_odf:8977 / rosetta_odf:8947).
- **Lage:** (gemessen 2026-09-29 folge202, `research-max` via `archive_search`) **Galileo:** PPI-Annex `https://pds-ppi.igpp.ucla.edu/annex/GO-SUN-RSS-1-TDF-V1.0/TDF/` (200, TDF/ATDF) trägt `0333342A`–`0343344A` = 1990-11-29→1990-12-10, deckt **Earth-1** (1990-12-08). Earth-2 (1992-12): nichts. PDS3 `GO-V-RSS-1-TDF-V1.0` nominell Venus+Earth-1, aber kein Online-Zugriff gemessen. **MESSENGER/Cassini/Rosetta:** nicht gefunden — alle gemessenen Archive beginnen nach dem jeweiligen Flyby (MESSENGER 2006/07+, Cassini Roh 2001-11+, Rosetta RSI-Lücke CVP1-0004→CR2-0012 deckt 2005-03-04 nicht). **Zusatz-Riss:** `state/zustand/external-state.md:34` nennt „6 Shards" / Shard1 „82 063 760 B"; `phi/sources.φ` trägt 3 Shard-Blöcke, `harvest.φ:251`/`frame_registry.φ:71-76` nennen 6, der Taucher misst Shard1 `1 073 741 816 B`.
- **Blockade:** die registrierten ODFs sind Post-Flyby.
- **Braucht:** Galileo-TDF-Quelle registrieren (Mountain-Verdikt + Mycelium `url`/`origin`); Shard-Zahl-/Größen-Riss tragen; MESSENGER/Cassini/Rosetta als `nicht gefunden`-Verdikt (Mountain).

### Kuprat — witness + tag
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission / Operator-Wort zur Zeugenart.
- **Lage:** (gemessen 2026-09-28) vier Kanäle auf der CDN, `sgrep -i kuprat phi/witnesses.φ` = 0; Tag-Drift `ssd.jpl.nasa.gov` in Probes/Workflows.
- **Blockade:** `witness kuprat`/`format`/`field` = Mountain.
- **Braucht:** Mycelium Tag-Drift heilen (nach Re-Materialisation); Mountain `witness kuprat` + `format`/`field`.

### Release-Namespace-Migration + `url`/`origin`-Drift
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort / nächster CDN-Migrations-Atom.
- **Lage:** (gemessen 2026-09-29 folge202) `wartend.φ:29` (legacy-cdn-ssd): die 4 Register-`url`-Zeilen `sources.φ:2416/:10605/:10475/:9207` sind 404, Assets liegen 200 unter Legacy-Tag `ssd.jpl.nasa.gov`. Taucher: `twomass_psc` fehlt in `sources.φ` (0 Treffer; Kandidat in `stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`); `jwst_spectra:s9207-9210` url-Basename `curated48_spectra.bin` ≠ format/compiler-Name; `LLNL_G3D_JPS.volume:s13813` + `S40RTS.volume:s13821` tragen `origin procedure: …/<name>.nc` mit ungebundenem Platzhalter.
- **Blockade:** systemische Migration (Re-Materialisation + Probe-/Workflow-Flip).
- **Braucht:** pro Asset Re-Materialisation unter Produzenten-Tag; `url`/`origin`-Drift-Zeilen korrigieren (Mycelium).

### Rätsel-Bestand (Survey) — Träger + Verdikt-Trägerschaft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste Rätsel-Messung (`docs/surveys/survey-raetsel-bestand.md`).
- **Lage:** (gemessen 2026-09-28 folge200 + River-61-Befund) das Survey steht (12 Nadeln + 2 Blätter + Kuprat); 10/15 Verdikt nur als Paper-/Archiv-Prosa; **Riss (River 61, widerlegt):** `survey-raetsel-bestand.md:34` führt `spatial.rs:549-554` als „Kanal vorhanden", während `:34` denselben `max(0,|Δt|−d/c)`-Fold als fehlend listet — self-widersprüchlich; der Fold lebt (`spatial.rs:549-550`).
- **Blockade:** keine.
- **Braucht:** je Rätsel den `Nächster Schritt` dispatchen; Ⅹ descope-Befund belegen oder `pending`; Verdikt-Zeilen = Mountain.

### Planetary-Arme — `blocked_sources.φ` pending (Compiler/Asset fehlt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster CDN-Materialisations-Atom (`phi/blocked_sources.φ`).
- **Lage:** (gemessen 2026-09-29 via `register_lookup --open`) `[mycelium] pending` in `phi/blocked_sources.φ` mit stehendem Arm, aber fehlendem Compiler-Bin/Asset: `:337` GOSAT-GW (Run `36283215548` rot, Leerfenster-Overflow), `:393` ExoMars TGO, `:397` Akatsuki (dir 503), `:401` Kaguya/SELENE, `:405` Chandrayaan-1 Mini-RF, `:419` Phobos 2 KRFM, `:423` Vega 1/2, `:427` Hayabusa. `:389` MAP-Grid Globus-Auth + `:58` BepiColombo psahelp = Operator/Dritter.
- **Blockade:** Compiler-Bin + `*-cdn.yml` je Arm fehlt.
- **Braucht:** je Arm Compiler-Bin + Workflow (Muster `nvss-cdn.yml`), dann `gh workflow run`; `:337` `ci_manage log 36283215548`.

### DAS2 Iowa + Occultation-DB UTFPR — nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/blocked_sources.φ:365-371`).
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
- **Trigger:** 2026-10-02 (Routen `superdarn-af68c4f1`, CSES `laic-cses`) / 2026-12-02 (NOIRLab/Gaia-DR4) / 2027-04-01 (BepiColombo `bepicolombo-more`).
- **Lage:** (gemessen 2026-09-28) `pithia` backend-tot; `lasair` direct absent/proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Gefaltete `## An mycelium`-Blöcke (2026-09-28, committete Sender)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Block der benannte nächste Schritt (`register_lookup --addressed mycelium`).
- **Lage:** (gemessen 2026-09-29, `register_lookup --addressed mycelium`) vier committete Blöcke:
  - **future-folge151** — Journal-Emitter-Rest-Haken: (a) `register_lookup --open` lernt `ereignisse.φ` — **in folge201 committet (`625fb2a2b`)**; (b)(c) Planungs-Pass-Regel + Stehender-Pass liest `ereignisse.φ` — im Stehenden Pass aufgenommen; (d) `tool.execute.before`-Probe (`/tmp/opencode/act-probe.log`) **offen**; Sources-Zeilen = Offen-Punkt oben.
  - **mountain-folge200** — Fixe-Tabellen-Feder-Split (`url`/`origin`/`compiler` = Mycelium) + ISRO-`url`-Riss (oben) + DEMETER/`format vlde` (oben) + `[redacted]` (siehe Riss).
  - **river-folge61** — Rätsel Ⅶ Doc-Riss (oben benannt); Name-Riss `signalkegel_audit_probe` stale (`survey-2026-09-03-daten-holdings-inventur.md:51/:77`, Bin = `signal_cone_audit_probe`); FRB Ⅸ gemessen (kein Pfeil, 0 honored); `matrix-rotor 36436173707` Runner-Preemption; dropped-gate-Umbau; `flyby-odf-cdn 36414284741` rot (`odf-persist` ohne `--file` → 0-Byte census); `devcontainer`-CLI; Juice-Kernel `…261003` unregistriert (`sources.φ:7151` endet vor Perigäum).
  - **sensory-folge202** — DEMETER-Klasse (oben); 4 trägerlose Docs: `docs/concepts/tools-map.md` (Chrome-DevTools-MCP `:310-313`, Tool-Pflege `:329-347`), `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (`:100-151`), `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` (`:109-144`), `docs/concepts/pfeiler-der-architektur.md` (`:37`/`:135`).
- **Blockade:** gemischt (`d)`/`[redacted]`/Docs ungemessen).
- **Braucht:** (d)-Probe messen; Docs-Träger setzen oder `descoped`; die Workflow-/Doc-Namen heilen.

### Riss — `orphan-docs`-Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Stehender Pass mit frischem Bin (`state/zustand/standing-pass.md`).
- **Lage:** (gemessen 2026-09-29) `register_lookup --orphan-docs` = **0**; sensory-folge202 misst via flash-Taucher **4** trägerlose Docs (oben). Divergenz benannt, nicht geglättet.
- **Blockade:** Release-Bin-Freshness unbestimmt.
- **Braucht:** `register_lookup --orphan-docs` mit frischem Bin re-messen; die 4 Docs gegen den Baum prüfen.

### Riss — `[redacted]`-Wait
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Einverständnis zur Redistribution (SAMPLE_CONTACT) — `state/mail/[redacted].md`.
- **Lage:** (gemessen 2026-09-29) mountain-folge200 behauptet „Trigger gefeuert — Wait kann geschlossen werden". Widerlegt am Register: `wartend.φ:8` Träger = „Einverständnis zur Redistribution"; die Mail `mail_ledger.φ:181` ist Eingang, nicht Einverständnis; die note sagt „Send = Operator-Hand; bis zum Einverständnis kein CDN". Der Wait **bleibt offen**.
- **Blockade:** Reply-Entwurf `state/mail/[redacted].md`; Send = Operator-Hand.
- **Braucht:** Operator sendet die Lizenz-/Redistributionsfrage; Antwort = Trigger.

## An mountain

- **Rosetta-Granule** | (gemessen 2026-09-29) `rosetta 36491843356` rot: `naif -226 carries no granule across the given kernels — the anchor stays unwritten`; Kernel-Set/Typ-18-Arm prüfen.
  Origin: mycelium-folge202
- **spectral/pds3/pds4-CDN-Zulassung** | (gemessen 2026-09-29) drei Läufe success — Assets auf der CDN, `format`/Verdikt + `sources.φ`-Zeilen offen.
  Origin: mycelium-folge202
- **ODF-Flyby-Verdikt** | (gemessen 2026-09-29) Galileo PPI-Annex-TDF deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden. Feder: Verdikt + Shard-Riss-Tragung (`external-state.md:34` vs `sources.φ`/`harvest.φ`).
  Origin: mycelium-folge202
- **DEMETER-Klasse schärfen** | (gemessen 2026-09-28 sensory) `phi/blocked_sources.φ:89-91` noch `pending` mit Alt-note; Klasse „CDPP-Generierung defekt — wartend".
  Origin: sensory-folge202
- **`format vlde`-Admission** | (gemessen 2026-09-29) Reader/Compiler/Wf stehen, Asset 206; `format`/`field`/`ttl` + Tag-Riss (`ssd.jpl.nasa.gov-vlies` 404).
  Origin: mycelium-folge202

## An future

- **NSE-Redistribution + Dank** | (gemessen 2026-09-28 folge199) Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) um die Lizenz-/Redistributionsfrage erweitern; Akt = Operator-Hand. Bis zum Einverständnis kein CDN.
  Origin: mycelium-folge199
- **ENSO-Zuschnitt** | (gemessen 2026-09-28) `state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`. Braucht: Operator-Wort.
  Origin: mycelium-folge200
- **Kuprat-Zeugenart** | (gemessen 2026-09-28) `phi/witnesses.φ` trägt nur `s2-direction`, `gestalt`; Kuprat-Zeugenart fehlt. Braucht: Operator-Wort.
  Origin: mycelium-folge200
- **`gic-causal-driver.md` DOI-Minting** | (gemessen 2026-09-28) `docs/paper/gic-causal-driver.md:531/538` DOIs `pending`. Mint = Dritter-Akt → Operator-Hand.
  Origin: mycelium-folge201

## An sensory

- **`reference_verify.rs` Test-Pfad** | (gemessen 2026-09-28 folge201) `tools/register/src/bin/reference_verify.rs:332` importiert `extract_arxiv_ids` ohne `extract_dois` (`:371` nutzt es) — `cargo check -p omegaflow-register --tests` bricht; committet `5cdf92677`. Feder: sensory (Urheber).
  Origin: mycelium-folge201

## An river

- **Rätsel Ⅶ Doc-Riss + Name-Riss** | (gemessen River 61) `survey-raetsel-bestand.md:34` self-widersprüchlich (`spatial.rs:549-554` als fehlend gelistet, obwohl der Fold lebt); `signalkegel_audit_probe` stale ↔ Bin `signal_cone_audit_probe`. Feder: river (Surveys).
  Origin: river-folge61
