<!--
  title: Handover — Mycelium-Folge 196 (2026-09-28)
  session: Mycelium-Folge 196
  class: handover
  date: 2026-09-28
  sha256: 9111c334a129cae75589696a88ea3a858417ba06585d5b3b4545dbbc200484f7
  status: live
-->
# Handover — Mycelium-Folge 196 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`;
Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge195.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „naja ich möchte ja erstmal dass du die taucher mit harten bandagen auf die sonden loslässt" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „sofort prinzp warum sid die 2 kerne nicht registriert wir haben doch DE441 DE442 mich wundert was du sagst mir kommt das alles sehr dubios vor hast du wirklich korrekt in cources assetzsd und lokal geschaut?" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „ziehst du die konsquentz strukturell dann musst du sie in opencode/baum verankern" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „Diver-B/C-Befunde einzeln gegen den Baum prüfen?" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196.

## Offen (aufgeschlüsselt)

### Sonden-Routen-Vergleich → Weberin → Zeugen — Taucher gelaufen, Bau offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-09-28 (Sonden-Routen-Taucher gesetzt), nächster Bau-Schritt an der Kante (`src/weberin.rs:32-38`).
- **Lage:** (gemessen 2026-09-28 via 3× `research-max`, danach baum-verifiziert) Anderson-2008-Positive (PRL 100:091102): Galileo Ⅰ +3,92±0,08 · Galileo Ⅱ −4,60±1,00 · NEAR +13,46±0,13 · Cassini −2±1 · Rosetta Ⅰ +1,82±0,05 mm/s; MESSENGER +0,02±0,03 (Null). Nullfälle: Juno 2013 (Thompson 2014, NTRS 20160008163), Rosetta Ⅱ/Ⅲ, BepiColombo 2020. `absent` (kein publ. Residuum): Hayabusa, Stardust, OSIRIS-REx. Erdfern: Pioneer 10/11 (Anderson 2002, PRD 65:082004). Das Haus trägt die Roh-ODF für Galileo `phi/sources.φ:9764`, Cassini `:8977`, Rosetta `:8947/8957/8967`, MESSENGER `:9810/9818`, Pioneer 10 `:10236/10244`, Voyager `:9843+`; **nicht** NEAR, Juno-pre-EFB. Der Weberin-Kern kennt keinen Doppler-Arm (`BodyLine = {Spk, Dastcom, Mpc, Inpop, Epm}`, `src/weberin.rs:32-38`); der Riss steht nur als `BodyOutcome::Riss` (`:130`). DE440/DE441/DE442 registriert (`phi/sources.φ:3419-3487`, Tools `:10915`), Assets live, `flyby_ephemeris_gate` gebaut (River `d310d5888`, 2026-09-27).
- **Blockade:** keine.
- **Braucht:** (a) die 5 fehlenden Sonden-Ephemeriden (Galileo, Cassini, Rosetta, MESSENGER, NEAR) als Asset registrieren; (b) den Weberin-Doppler-`BodyLine` bauen; (c) den Vergleich der Anderson-Positiven gegen den N-body-Stack.

### Register-Lücken — Sonden-Korpus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28 via `research-max` + `sgrep`/`--verdict`, baum-verifiziert) 16 radio-trächtige Sonden ohne Ephemeriden-Asset (Galileo, Ulysses, Cassini, MAVEN, DART, MESSENGER, MGS, MRO, Odyssey, Magellan, Dawn, Mars Express, Rosetta, VEX, LRO, Pathfinder); NEAR/Stardust/Hayabusa/BepiColombo/OSIRIS-REx Radio-Routen ungemessen. Format-Faktum: `gll_rss_rsr.bin:10137` / `gll_rss_tnf.bin:10147` tragen `format cassini_rsr`/`cassini_tnf` auf Galileo-origin (`gll.rss`).
- **Blockade:** keine.
- **Braucht:** die fehlenden Ephemeriden (Muster `ephemeris_mariner10.bin:15573`) + die ungemessenen Routen über `docs/SOURCE_PORT.md` portieren/registrieren.

### Register-Kandidaten index.φ (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28) `phi/pipeline/index.φ` trägt 8 `verifiziert` (owner mycelium): `queue/sources_potential_pre-cdn_9k_richest.φ`, `…_params.φ`, `catalog/oai_arxiv.φ`, `b2find_intermagnet_catalog.φ`, `terrapulse_catalog.φ`, `esa_geomagnetic_catalog.φ`, `archeology_gaps_index.φ`, `copernicus_catalog.φ`.
- **Blockade:** keine.
- **Braucht:** die 8 Inventare über `docs/SOURCE_PORT.md` portieren.

### Register-Riss — parser-def-Pen
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Ratifikation der neuen `parser-def`-Blöcke.
- **Lage:** (gemessen 2026-09-28 via `sread` `phi/blocked_sources.φ`) 8 `parser-def`-Blöcke + 5 gap-Klassen; die 5 Reader-Arme fehlen.
- **Blockade:** Pen-Grenze Mountain/Mycelium.
- **Braucht:** Mountains Ratifikation; die 5 Reader-Arme (PDS3/PDS4/HTML).

### ci-check — clippy geheilt, Bestätigungslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des lebenden `ci-check`-Laufs.
- **Lage:** (gemessen 2026-09-28) `36413456788` in_progress; clippy-Rot `src/archivar/odf.rs:209` in `9f8debcc3` geheilt.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36413456788`.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`/`jobs`) `36344350143` rot, Job-Log `unread`.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143`.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf, belegt in `ci_watchdog.log`.
- **Lage:** (gemessen 2026-09-28 via `bash -n` + `sread` `bin/ci_watchdog.sh`) transient-first, Assertion-Klasse begrenzt.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### D5-Orphan-Residuum — CDN-Manifestations-Weg
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288`).
- **Lage:** (gemessen 2026-09-27 via `sgrep` + Register) kein Producer-Bin, keine Register-Zeile, kein `*-cdn.yml`.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Träger — Prosadokumente mit offenen Markern
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `register_lookup --orphan-docs`-Lauf meldet einen ORPHAN_DOC.
- **Lage:** (gemessen 2026-09-28 via `register_lookup --orphan-docs`) 0 orphan docs — jeder offene Marker trägt einen Namenträger.
- **Blockade:** teils Operator-Wort/Trigger.
- **Braucht:** `register_lookup --orphan-docs` beim nächsten Pass.

## An river

- **Doc-Riss flyby-path-2** | (gemessen 2026-09-28) `docs/paper/flyby-path-2-preregistration-revised.md:85-88` nennt δ `pending` („needs the two NAIF kernels registered"), während `phi/sources.φ:3419-3487` DE441/DE442 trägt und `flyby_ephemeris_gate` gebaut ist (`d310d5888`). Der Baum widerlegt die Doc-Zeile. Braucht: die Estimator-Validation-Zeile auf den Baum ziehen.
Origin: mycelium-folge196

## An mountain

- **Format-Faktum Cassini/Galileo** | (gemessen 2026-09-28) `phi/sources.φ:10137` `gll_rss_rsr.bin` trägt `format cassini_rsr` + `cassini_rsr_*`-Felder, `:10147` `gll_rss_tnf.bin` `format cassini_tnf` — auf Galileo-origin (`gll.rss`, Compiler `gll_rss_*_compiler.rs`). Riss (falscher Formatname) oder geteilte Format-Familie ist ungemessen. Braucht: Mountain-Pen-Verdikt.
Origin: mycelium-folge196

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). **Operator-Wort 2026-09-28: Commit erst, wenn alle anderen Sessions committet haben** — der geteilte Baum trägt fremde uncommittete Arbeit (`src/mathematikerin/ksg_k.rs`), die nicht berührt wird.
