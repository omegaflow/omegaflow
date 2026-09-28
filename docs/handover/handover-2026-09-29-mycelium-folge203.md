<!--
  title: Handover — Mycelium-Folge 203 (2026-09-29)
  session: Mycelium-Folge 203
  class: handover
  date: 2026-09-29
  sha256: 7588be8d9336438d7100247f1188110cb5b99f456a73ab4f97a0c47d959ec548
  status: live
-->
# Handover — Mycelium-Folge 203 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-29-mycelium-folge202.md` (→ `archiv/`).

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). In diesem Atom re-dispatcht: `messenger-ephemeris-cdn`
`36498410453`, `near-ephemeris-cdn` `36498422789`, `hinet-cdn` `36498426236`.

## Operator-Wort-Register

- Wort | 2026-09-29 | „kannst du nicht mehr abarbeiten?" | Quelle: Mycelium-Session 203 (Anlass: Plan hatte nur die Tafel vorgelegt; Operator verlangt Abarbeitung über den Plan hinaus).
- Wort | 2026-09-28 | „nein ich möchte dass erst ein echtes survey gemacht wird mit flash tauchern mit harten bandagen einen pro rätseln um zu prüfen was wir haben und was fehlt eine art tabelle" | Quelle: Mycelium-Session 200.
- Wort | 2026-09-28 | „bitte mach erstmal eine archeologie wrum ich die daten überhaupt angefragt habe nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" | Quelle: Mycelium-Session 199.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum).
- Wort | 2026-09-28 | „hast du alles bis zur kante gemessen und geplant?" | Quelle: Mycelium-Session 201.

## Offen (aufgeschlüsselt)

### Sonden-Ephemeriden — messenger/near re-dispatcht, Rosetta-Granule offen
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Ergebnis der Re-Dispatch-Läufe (`36498410453`/`36498422789`); Mountain-Kernel/Arm für Rosetta (`## An mountain`).
- **Lage:** (gemessen 2026-09-29 folge203) URL-Fix committet (`aa78cf331`); `messenger-ephemeris-cdn 36498410453` + `near-ephemeris-cdn 36498422789` dispatcht. `rosetta 36491843356` rot: `naif -226 carries no granule across the given kernels — the anchor stays unwritten`.
- **Blockade:** Rosetta = Code/Granule-Pfad (Mountain).
- **Braucht:** `ci_manage view 36498410453`/`36498422789`; Rosetta-Kernel-/Typ-18-Arm → `## An mountain`.

### spectral-/pds3-/pds4-CDN — Läufe grün, Zulassung melden
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`spectral-cdn.yml`/`pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 folge202 via `ci_manage`) `spectral-cdn 36488149432`, `pds3-fixed-width 36488153465`, `pds4-fixed-width 36488157946` = **success**.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `## An mountain` — Zulassung + `sources.φ`-Zeilen.

### Sonden-Konten CNSA/NSSDC/ISRO/EMM — pending, ISRO-Riss gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Konten per Operator-Hand registriert (gefeuert) — `phi/blocked_sources.φ:373/377/381/385`.
- **Lage:** (gemessen 2026-09-29 folge203, general-Taucher via `archive_search --verdict`) `https://pradan.issdc.gov.in/ch2` = **200** (31050 B); `chmapbrowse`, `mrbrowse`, `ch2/chmapbrowse`, `ch2/mrbrowse` = **404**. Der `url`-Eintrag `/ch2` ist der korrekte Pfad; die note `phi/blocked_sources.φ:384` („chmapbrowse 200 (mrbrowse 404)") ist stale → **Mountain-Feder**.
- **Blockade:** Download end-to-end braucht das angemeldete Operator-Browser-Profil.
- **Braucht:** `## An mountain` — note `:384` heilen; Operator-Browser-Profil für e2e.

### Sources-Zeilen — Endpunkte gemessen (future-folge151/152-Liste)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `docs/SOURCE_PORT.md`-Register-Pass (§5 Schritt 4).
- **Lage:** (gemessen 2026-09-29 folge202 via `--verdict`/`--sniff`) alle erreichbar: Chang'e-MRM-Bundle (200), `zenodo.org/records/15812343` (200), `pds.wh.sdu.edu.cn` (206), `data.kasi.re.kr` (200), KMTNet-MOC FITS (1 848 960 B, sha256 `f34ff61f…`), `pds.shadowcam.im-ldi.com/derived/` (200), `www.kari.re.kr/kpds/` (200), DARTS PDS4-Root (206), ESA PSA TAP (200). **Riss:** `alasky.cds.unistra.fr/Tianwen1-MoRIC/` = **404**.
- **Blockade:** `format`/Verdikt = Mountain; `url`/`origin`/`compiler` = Mycelium.
- **Braucht:** je Endpunkt Format festlegen; Tianwen1-MoRIC-Pfad neu messen oder verwerfen.
- **Träger:** `phi/blocked_sources.φ:410/:414/:435`.

### Register-Kandidaten `index.φ` — oai_arxiv geflippt, 7 Merges blockiert (Riss §5.4)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** korrigierter `docs/SOURCE_PORT.md`-Merge-Pass.
- **Lage:** (gemessen 2026-09-29 folge203, grind-flash) `phi/pipeline/index.φ:75` `oai_arxiv` → `index` (0 `url`-Direktiven, reines Inventar; Begründung in der `:76`-note). **Riss:** die zitierte Regel `docs/SOURCE_PORT.md §5.4` **existiert nicht** (`sgrep "5.4"` über `docs/`+`phi/` = 0; Historie nie) — über folge198–202 ungeprüft getragen. Die reale Regel ist `docs/SOURCE_PORT.md:134–137` (§5 Schritt 4: echte Neue → `phi/sources.φ`, Varianten/Modelle/Tote → `phi/dead_sources.φ`, `ledger.φ` updaten).
- **Blockade:** Merge der 7 Stage-Ergebnisse berührt Mountain-Verdiktregister — unter mycelium-Pfad-Grenze nicht ausführbar; falsche Zitation.
- **Braucht:** §5.4-Verweis auf §5 Schritt 4 korrigieren; Merge an Mountain übergeben bzw. mit Mountain-Freigabe im selben Atom.

### `format vlde` — Source-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`format vlde`, `field`, `ttl`).
- **Lage:** (gemessen 2026-09-29 mountain-folge201) exakte Direktiven geliefert: `format vlde` / `ttl 604800` / `field count vlies_density_count inverse-square em count 604800 0.0 0.0`. Reader `src/archivar/vlies.rs:3`, Fetch `main_flow.rs:4614`, Compiler `vlies_density_compiler.rs`, Wf `vlies-density-cdn.yml:27` stehen; Asset `vlies_density.vlde` 206. Meine Federseite: `url`/`origin`/`compiler`. Tag-Riss: Compiler `ssd.jpl.nasa.gov-vlies` (`:348`) → 404.
- **Blockade:** `format`/`field`/`ttl`/Admission = Mountain.
- **Braucht:** nach Admission `url`/`origin`/`compiler` + Tag heilen.

### DEMETER — CDPP server-seitig defekt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** quellenseitige CDPP-Reparatur — `phi/blocked_sources.φ:89-91`.
- **Lage:** (gemessen 2026-09-28 sensory-folge203) Datei-Endpoint UA-gated, kein IP-Block; `availableFilesCount 0`, `filesInErrorCount 1000`; Klasse liest noch `pending` mit Alt-note. Wait `state/zustand/wartend.φ:4` (Aufnehmer sensory).
- **Blockade:** quellenseitig; Klasse ungeschärft (Mountain-Note).
- **Braucht:** `## An mountain` — Klasse „CDPP-Generierung defekt — wartend" schärfen; kein Code-Schritt.

### ODF-Coverage der Flyby-Fenster — Galileo-Treffer, 3 x nicht gefunden
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt / neue Pre-Flyby-Quelle (`phi/sources.φ` galileo_odf:9764 / messenger_odf:9810 / cassini_odf:8977 / rosetta_odf:8947).
- **Lage:** (gemessen 2026-09-29 folge202, research-max) **Galileo:** PPI-Annex TDF trägt `0333342A`–`0343344A` = 1990-11-29→1990-12-10, deckt Earth-1. Earth-2 nichts. **MESSENGER/Cassini/Rosetta:** nicht gefunden. **Shard-Riss** (gemessen 2026-09-29 folge203): `phi/sources.φ:8947-8975` trägt **3** Rosetta-ODF-Shards; `harvest.φ:251` + `frame_registry.φ:71-76` nennen **6**; `external-state.md:34` nennt 6 und Shard1 `82 063 760 B` (nur dort belegt), harvest/Taucher messen Shard1 `1 073 741 816 B`. Zitat `sources.φ:7600` in `external-state.md` ist verdriftet (heute DPD-Block).
- **Blockade:** registrierte ODFs Post-Flyby; Shard-Zahl/-Größe widersprüchlich.
- **Braucht:** Galileo-TDF registrieren; Shard-Riss tragen; MESSENGER/Cassini/Rosetta als `nicht gefunden`-Verdikt (Mountain).

### Kuprat — witness + tag
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission / Operator-Wort zur Zeugenart.
- **Lage:** (gemessen 2026-09-28) vier Kanäle auf der CDN, `sgrep -i kuprat phi/witnesses.φ` = 0; Tag-Drift `ssd.jpl.nasa.gov` in Probes/Workflows.
- **Blockade:** `witness kuprat`/`format`/`field` = Mountain.
- **Braucht:** nach Re-Materialisation Tag-Drift heilen; Mountain `witness kuprat`.

### Release-Namespace-Migration + `url`/`origin`-Drift
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort / nächster CDN-Migrations-Atom.
- **Lage:** (gemessen 2026-09-29 folge202) `wartend.φ:29` (legacy-cdn-ssd): die 4 Register-`url`-Zeilen `sources.φ:2416/:10605/:10475/:9207` sind 404, Assets liegen 200 unter Legacy-Tag `ssd.jpl.nasa.gov`. `twomass_psc` fehlt in `sources.φ` (Kandidat `stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`); `jwst_spectra:s9207-9210` url-Basename `curated48_spectra.bin` ≠ format/compiler-Name; `LLNL_G3D_JPS.volume:s13813` + `S40RTS.volume:s13821` tragen `origin procedure: …/<name>.nc` mit ungebundenem Platzhalter.
- **Blockade:** systemische Migration (Re-Materialisation + Probe-/Workflow-Flip).
- **Braucht:** pro Asset Re-Materialisation unter Produzenten-Tag; `url`/`origin`-Drift-Zeilen korrigieren.

### Rätsel-Bestand (Survey) — Träger + Verdikt-Trägerschaft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste Rätsel-Messung (`docs/surveys/survey-raetsel-bestand.md`).
- **Lage:** (gemessen river-folge62) Ⅶ (`max(0,|Δt|−d/c)`-Fold, `spatial.rs:549-550`) und Ⅸ (`frb_blatt_probe.rs:295`, 0 honored) sind **stale** — keine fehlenden Rechnungen, die Survey-Zeilen sind zu korrigieren. Ⅰ (Jeans) und ENSO bleiben offen (river-folge62 getragen).
- **Blockade:** keine.
- **Braucht:** Survey-Zeilen Ⅶ/Ⅸ korrigieren; je Rätsel `Nächster Schritt`; Verdikt-Zeilen = Mountain.

### Planetary-Arme — `blocked_sources.φ` pending (Compiler/Asset fehlt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster CDN-Materialisations-Atom (`phi/blocked_sources.φ`).
- **Lage:** (gemessen 2026-09-29) `[mycelium] pending` mit stehendem Arm, fehlendem Compiler-Bin/Asset: `:338` GOSAT-GW (Run `36283215548` rot), `:394` ExoMars TGO, `:398` Akatsuki (503), `:402` Kaguya, `:406` Chandrayaan-1 Mini-RF, `:418` Phobos 2, `:422` Vega, `:426` Hayabusa. mountain-folge201 liefert exakte Direktiven für Phobos (`at mars`), Vega (`at halley`), Hayabusa (`at itokawa`). `:390` MAP-Grid Globus-Auth + `:59` BepiColombo psahelp = Operator/Dritter.
- **Blockade:** Compiler-Bin + `*-cdn.yml` je Arm; `at halley`/`at itokawa` Körper-Registrierung fehlt.
- **Braucht:** je Arm Compiler-Bin + Workflow (Muster `nvss-cdn.yml`), dann `gh workflow run`; `:338` `ci_manage log 36283215548`.

### DAS2 Iowa + Occultation-DB UTFPR — nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/blocked_sources.φ:365-371`).
- **Lage:** (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), kein `sources.φ`-Block/Workflow.
- **Blockade:** Admission-Verdikt.
- **Braucht:** nach Admission `format`/`tag`/`pattern` + Workflow.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36498426236` (re-dispatcht 2026-09-29 folge203).
- **Lage:** (gemessen 2026-09-28 folge198) Job `hinet`: `cont status never read` (8×), exit 1.
- **Blockade:** quellenseitige Readiness.
- **Braucht:** `ci_manage view 36498426236`.

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
- **Trigger:** 2026-10-02 (Routen `superdarn-af68c4f1`, CSES `laic-cses`) / 2026-12-02 (NOIRLAB/Gaia-DR4) / 2027-04-01 (BepiColombo `bepicolombo-more`).
- **Lage:** (gemessen 2026-09-28) `pithia` backend-tot; `lasair` direct absent/proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Orphan-Docs-Zensus — 2 gemessen, Riss zur sensory-Zahl
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs` mit frischem Bin.
- **Lage:** (gemessen 2026-09-29 folge203) Tool = **2**: `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` (3 Marker) + `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (3 Marker). Die 4 sensory-Docs sind gegen den jetzigen Baum getragen (mycelium-folge202:157 / sensory-folge203:157). Die general-Simulation ergäbe nur `membran-ladearchitektur.md` — die Tool-Zahl ist die Messung, die Simulation die Behauptung (A = A). Divergenz benannt, nicht geglättet.
- **Blockade:** Bin-Freshness.
- **Braucht:** die 2 Docs lesen, Owner bestimmen, Namenträger setzen oder gemessen `descoped`.

### future-folge152 — exzellenz-konzept-Paper-Tooling
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Tooling-Atom (`docs/concepts/exzellenz-konzept.md`).
- **Lage:** (gemessen future-folge152) `docs/concepts/exzellenz-konzept.md` (3 offene Marker, `:32-253`: Paper-Maßstab/10 Stufen, Export-Stufe); LaTeX/PDF-Tooling = Mycelium. Journal-Emitter-Rest-Haken: (a) committet `625fb2a2b`; (b)(c) im Stehenden Pass aufgenommen; (d) `tool.execute.before`-Probe **feuert** (gemessen 2026-09-29 folge203: `/tmp/opencode/act-probe.log`, 5146 Z., `:79-84` appendFileSync).
- **Blockade:** keine.
- **Braucht:** LaTeX/PDF-Tooling für `exzellenz-konzept.md` bauen; Marker schließen.

### ENSO-Manifestation — nach Mountains Zeile
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains `sources.φ`-Zeile für ENSO steht (river-folge62).
- **Lage:** (gemessen river-folge62) Asset `coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin` + Compiler `--ci-mode` zu fahren, sobald die Zeile steht.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** nach Admission Release-Asset + Compiler `--ci-mode` fahren.

### Riss — `[redacted]`-Wait
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Einverständnis zur Redistribution (SAMPLE_CONTACT) — `state/mail/[redacted].md`.
- **Lage:** (gemessen 2026-09-29) `wartend.φ:8` Träger = „Einverständnis zur Redistribution"; die Mail `mail_ledger.φ:181` ist Eingang, nicht Einverständnis; „bis zum Einverständnis kein CDN". Der Wait bleibt offen.
- **Blockade:** Reply-Entwurf `state/mail/[redacted].md`; Send = Operator-Hand.
- **Braucht:** Operator sendet die Lizenz-/Redistributionsfrage; Antwort = Trigger.

## An mountain

- **Rosetta-Granule** | (gemessen 2026-09-29) `rosetta 36491843356` rot: `naif -226`; Kernel-Set/Typ-18-Arm prüfen.
  Origin: mycelium-folge203
- **ISRO-note stale** | (gemessen 2026-09-29 folge203) `phi/blocked_sources.φ:384` note „chmapbrowse 200 (mrbrowse 404)" widerlegt: `ch2`=200, `chmapbrowse`/`mrbrowse`/`ch2/chmapbrowse`/`ch2/mrbrowse`=404. Feder: note heilen.
  Origin: mycelium-folge203
- **spectral/pds3/pds4-CDN-Zulassung** | (gemessen 2026-09-29) drei Läufe success — Assets auf der CDN, `format`/Verdikt + `sources.φ`-Zeilen offen.
  Origin: mycelium-folge202
- **ODF-Flyby-Verdikt** | (gemessen 2026-09-29) Galileo PPI-Annex-TDF deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden. Feder: Verdikt + Shard-Riss-Tragung (`external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76`).
  Origin: mycelium-folge203
- **DEMETER-Klasse schärfen** | (gemessen 2026-09-28 sensory) `phi/blocked_sources.φ:89-91` noch `pending` mit Alt-note; Klasse „CDPP-Generierung defekt — wartend".
  Origin: sensory-folge203
- **`format vlde`-Admission** | (gemessen 2026-09-29) Reader/Compiler/Wf stehen, Asset 206; `format`/`field`/`ttl` + Tag-Riss (`ssd.jpl.nasa.gov-vlies` 404).
  Origin: mycelium-folge203

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

- **Rätsel Ⅶ Doc-Riss + Name-Riss** | (gemessen River 62) `survey-raetsel-bestand.md:34` self-widersprüchlich (`spatial.rs:549-554` als fehlend gelistet, obwohl der Fold lebt); `signalkegel_audit_probe` stale ↔ Bin `signal_cone_audit_probe`. Feder: river (Surveys).
  Origin: river-folge62

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
