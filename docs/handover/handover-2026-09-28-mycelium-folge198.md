<!--
  title: Handover — Mycelium-Folge 198 (2026-09-28)
  session: Mycelium-Folge 198
  class: handover
  date: 2026-09-28
  sha256: 55449cb58afe0ac894e85c758985fb1797120ed83385e3eb5fb6d847822953c5
  status: live
-->
# Handover — Mycelium-Folge 198 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge197.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „naja ich möchte ja erstmal dass du die taucher mit harten bandagen auf die sonden loslässt" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „sofort prinzp warum sid die 2 kerne nicht registriert wir haben doch DE441 DE442 mich wundert was du sagst mir kommt das alles sehr dubios vor hast du wirklich korrekt in cources assetzsd und lokal geschaut?" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „ziehst du die konsquentz strukturell dann musst du sie in opencode/baum verankern" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „Diver-B/C-Befunde einzeln gegen den Baum prüfen?" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196.
- Wort | 2026-09-28 | „hast du alles bis zur kante geplant?" | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „und hast du den rest auch gemessen?" | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „ja bitte" (die verbleibenden Behauptungen read-only nachmessen) | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort" | Quelle: Mycelium-Session 197.
- Wort | 2026-09-28 | „kannst du das fixen?" (Riss/register-coverage/hinet/LLNL/Tag-Drift/index.φ) | Quelle: Mycelium-Session 198.

## Offen (aufgeschlüsselt)

### Sonden-Ephemeriden — Anderson portiert, Horizons-Assets registriert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (Workflows müssen auf `origin/main` stehen, dann dispatchen).
- **Lage:** (gemessen 2026-09-28 folge198) 5 Anderson-Routen via NAIF-Cruise portiert: `tools/harvest/src/bin/spacecraft_ephemeris_compiler.rs`, 5 NAIF-IDs in `src/archivar/kernels/naif_spacecraft_ids.tsv`, 5 Workflows `{galileo,cassini,rosetta,messenger,near}-ephemeris-cdn.yml`, 5 `ephemeris_<sonde>`-Blöcke (`ssd.jpl.nasa.gov-ephemeris`). Zusätzlich **6 `ssd.jpl.nasa.gov-horizons`-Blöcke registriert** (`ephemeris_{cassini,galileo_e1,galileo_e2,messenger,near,rosetta}.bin`, `archive_search --verdict` je 206, `register_sort` canonical) — der Riss „unregistrierte horizons-Assets" ist damit geschlossen. `cargo check` 0/0. **9 weitere Horizons-Flybys** (`juno`, `juice`, `europa_clipper`, `voyager1/2_*`) bleiben unter demselben Tag unregistriert.
- **Blockade:** keine.
- **Braucht:** nach `/commit`: `gh workflow run <sonde>-ephemeris-cdn.yml`; 9 weitere Flybys registrieren; die 10 übrigen Sonden (folge197:39) routen.

### ODF-Coverage der Flyby-Fenster
- **Status:** wartend | **Bindung:** eigen (mit sensory)
- **Trigger:** die `ephemeris_<sonde>`-Blöcke stehen auf `origin/main` (dieser Commit).
- **Lage:** (gemessen 2026-09-28 Rat) ungemessen, ob `galileo_odf`/`cassini_odf`/`rosetta_odf`/`messenger_odf` die Flyby-Fenster tragen (Spanne ≠ Record-Coverage).
- **Blockade:** keine.
- **Braucht:** je ODF-Asset die Record-Coverage gegen das Flyby-Datum messen (Galileo Ⅰ 1990-12-08 / Ⅱ 1992-12-08 · Cassini 1999-08-18 · Rosetta 2005-03-04 · MESSENGER 2005-08-02).

### Register-Kandidaten `index.φ` (8)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28 via `sread phi/pipeline/index.φ`) 8 `verifiziert` (owner mycelium): `queue/sources_potential_pre-cdn_9k_richest.φ` `:37`, `…_params.φ` `:39`, `catalog/oai_arxiv.φ` `:75`, `b2find_intermagnet_catalog.φ` `:79`, `terrapulse_catalog.φ` `:89`, `esa_geomagnetic_catalog.φ` `:91`, `archeology_gaps_index.φ` `:93`, `copernicus_catalog.φ` `:97`.
- **Blockade:** keine.
- **Braucht:** die 8 Inventare über `docs/SOURCE_PORT.md` portieren.

### Register↔CDN-Tag-Drift (6 Assets)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Re-Manifestations-Läufe `36451187881`/`36451192094`/`36451197406` schließen.
- **Lage:** (gemessen 2026-09-28 folge198) **4 `url`-Tags korrigiert** auf den Compiler-`upload_release`-Tag (Regel: Register folgt dem Produzenten): `spectra.bin` `:2416` → `ncei.noaa.gov` · `curated48_spectra.bin` `:9207` → `exoplanetarchive.ipac.caltech.edu` · `first14.json` `:10475` / `nvss.json` `:10605` → `tapvizier.cds.unistra.fr`; `register_sort` canonical. Re-Manifestation dispatched: `nvss-cdn` `36451187881` · `first14-cdn` `36451192094` · `kernel-flatten` `36451197406` (lädt curated48). **Riss:** die Assets lagen auf der CDN noch unter dem Legacy-Tag `ssd.jpl.nasa.gov`; `ncei.noaa.gov/spectra.bin` = 404 (ssd… 206). `twomass_psc`/`jwst_spectra` haben **keinen Archivar-`format`-Arm** (2MPS) bzw. sind superseded → kein Block (keine erfundene Quelle).
- **Blockade:** `spectral_compiler.rs` hat **keinen CI-Workflow** → `spectra.bin` wird unter `ncei.noaa.gov` nie neu manifestiert.
- **Braucht:** `spectral-cdn.yml` (Muster `nvss-cdn.yml`) anlegen + dispatchen; Re-Manifestations-Läufe prüfen; twomass_psc/jwst_spectra → Mountain-Admission.

### LLNL_G3D_JPS/S40RTS `volume.bin` — Befund nicht baum-bestätigt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** neue Messung an `phi/sources.φ:13813`.
- **Lage:** (gemessen 2026-09-28 folge198 gegen den Baum) `phi/sources.φ:13813-13827` tragen bereits die korrekte `origin procedure: fetch https://media.githubusercontent.com/media/tom-new/tomography-models/main/<name>.nc then volume_builder`; die AFRP-Nachbarn `:11210-11253` tragen korrekt `data.earthscope.org`. Die mountain-Zeile („EMC/AFRP-Sammel-origin") ist am genannten Ort **nicht baum-bestätigt** (Quelle mountain folge193). **Riss: Behauptung vs. Baum — der Baum gewinnt.**
- **Blockade:** keine.
- **Braucht:** Mountain-Wort, ob der Befund fallen darf.

### `format vlde`-Quellzeile
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `vlies-density-cdn.yml`-Lauf / nächster Register-Pass (2026-09-29).
- **Lage:** (gemessen 2026-09-28 folge198 via `sgrep -i vlde phi/sources.φ` = 0) kein `format vlde`-Block; Reader `src/archivar/vlies.rs` (MAGIC `VLDE`), Compiler `tools/harvest/src/bin/vlies_density_compiler.rs`, Workflow `vlies-density-cdn.yml`, Asset manifestiert. **Riss:** mountain folge196 weist die Direktive mycelium zu, sensory-folge198 mountain.
- **Blockade:** der Riss (Schreiber-Rolle) ist unentschieden.
- **Braucht:** Owner-Wort, dann Block setzen.

### DAS2 Iowa + Occultation-DB UTFPR — Workflow nach Admission
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountains Admission (`phi/blocked_sources.φ:363-368`).
- **Lage:** (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), kein `phi/sources.φ`-Block und kein Workflow.
- **Blockade:** Admission-Verdikt (Mountain).
- **Braucht:** nach Admission `format`/`tag`/`pattern` + Workflow.

### pds3/pds4 — CDN-Erstlauf ausgewertet
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register-coverage`-Lauf auf dem HEAD.
- **Lage:** (gemessen 2026-09-28 folge198) **beide Läufe success** (pds4 `36447660819`, pds3 `36447656801` 16:20) → `phi/harvest.φ:224`/`:234` auf `asset present` + note (pds3: `pds3_fixed_width_<stem>.bin` 14608–16192 B sha256 `6c7d602c…`, roundtrip holds; pds4: 9 Shards, sha256 `c426b30d…`). Die zwei harvest.φ-Orphans sind damit weg. Verbleibend nach Mountains `572fa95fc`-Verdikten: `ORPHAN_COMMITTED phi/blocked_sources.φ:418 [mycelium]` — Vega 1/2 `https://pds-smallbodies.astro.umd.edu/holdings/vega2-c_sw-mischa-3-rdr-original-v1.0/` (`pending`, refs `harvest.φ:224`); `:386` superdarn + `:405` pda.kasi → [future].
- **Blockade:** keine.
- **Braucht:** die vega2-Zeile `:418` ist durch diesen Punkt getragen; `:386`/`:405` → Futures Übergabe.

### ci-check — Bestätigungslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des lebenden `ci-check`-Laufs auf dem HEAD.
- **Lage:** (gemessen 2026-09-28 folge198) `36447552860` pending; `36445557657` in_progress (Job `test`). Trigger gefeuert, nicht abschließbar.
- **Blockade:** keine.
- **Braucht:** `ci_manage status` beim nächsten Pass.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-28 folge198 via `ci_manage log 36344350143`) Job `hinet`: `cont status never read Available — the request stays unfetched` (8× `cont request attempt 0..7 stayed unready`), exit 1; Code `tools/harvest/src/bin/hinet_win32_compiler.rs:637/643`.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** erneuter Lauf; kein Code-Schritt bis die Quelle antwortet.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf, belegt in `ci_watchdog.log`.
- **Lage:** (gemessen 2026-09-28) transient-first, Assertion-Klasse begrenzt. Trigger nicht gefeuert.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### D5-Orphan-Residuum — Röhren-Asset-CDN-Weg
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:383` §14.4 Punkt 4).
- **Lage:** (gemessen 2026-09-28 via `sgrep`) kein Producer-Bin, keine Register-Zeile, kein `*-cdn.yml`.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Träger — Prosadokumente ohne Namenträger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `register_lookup --orphan-docs` meldet ORPHAN_DOC.
- **Lage:** (gemessen 2026-09-28 folge198, nach river/mountain-Commits) **8** live: `docs/concepts/arxiv-api.md` 2 · `exzellenz-konzept.md` 3 · `pfeiler-der-architektur.md` 2 · `tools-map.md` 8 · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` 7 · `survey-2026-09-20-browser-anbindung.md` 3 · `survey-fortschritt.md` 1 · `survey-messpunkt-verteilung.md` 6.
- **Blockade:** teils Owner-Wort.
- **Braucht:** je Dokument einen Namenträger in der Owner-Übergabe (mountain/river/future).

## An mountain

- **twomass_psc / jwst_spectra — Admission** | (gemessen 2026-09-28 folge198) `twomass_psc.bin` ist real (206, `irsa.ipac.caltech.edu`), Produzent `twomass_compiler.rs`, aber **kein Archivar-`format`-Arm** für das 2MPS-Format (`fetch.rs:1033-1059` kennt kein 2MPS) → keine Feldquelle; `jwst_spectra.bin` ist superseded (aktiver Erzeuger schreibt `curated48_spectra.bin`). Braucht: Mountains Admission/Verdikt, ob eine Zeile gesetzt wird.
  Origin: mycelium-folge198
- **carrier-drift `gap html-parser-arm`** | (gemessen 2026-09-28) `register-coverage 36447553203` meldet `CARRIER_DRIFT phi/blocked_sources.φ::gap:html-parser-arm carrier=none live=1` (Eintrag `:406`, Danuri/KPLO). Owner parser-def = Mountain. Braucht: Klassen-Träger-Zeile `phi/blocked_sources.φ::gap:html-parser-arm ×1` in Mountains Übergabe.
  Origin: mycelium-folge198
- **DEMETER Dispositions-Klasse** | (gemessen 2026-09-28) `phi/blocked_sources.φ:86-88` trägt `pending`. Der Datei-Endpoint ist **UA-gated, kein IP-Block** (curl ohne UA 403 / mit UA 202). sensory-folge199 nennt die Klasse „Myceliums Feder" — Widerspruch zur Feder-Teilung. **Riss.** Braucht: Mountains Klassifikations-Verdikt.
  Origin: mycelium-folge198
- **DAS2 Iowa + Occultation-DB UTFPR — Admission** | (gemessen 2026-09-28) beide Arme gebaut (`b4106e69a`), `phi/blocked_sources.φ:363-368` `pending`; es fehlt die Admission. Braucht: Mountains Admission.
  Origin: mycelium-folge198
- **vlde-Schreiber-Riss** | (gemessen 2026-09-28) mountain folge196 weist die `format vlde`-Direktive mycelium zu, sensory-folge198 mountain. Braucht: Ein-Wort-Klärung.
  Origin: mycelium-folge198

## An future

- **gic-causal-driver — DOIs** | (gemessen 2026-09-28) `docs/paper/gic-causal-driver.md:531`/`:538` sind `pending`. DOI-Minting ist ein Schreibakt an einem Dritten → Operator-Hand. Braucht: Operator-Wort nach Metadaten-Draft — in Futures Operator-Queue.
  Origin: mycelium-folge198
- **blocked-account-Träger (2)** | (gemessen 2026-09-28 via `register_lookup --orphans`) `phi/blocked_sources.φ:386` superdarn (`https://superdarn.ca/data-download`) und `:405` pda.kasi.re.kr (Danuri/KPLO) sind `[future]`-Orphans (owner `blocked account` → future). Braucht: Aufenthalt in Futures Übergabe.
  Origin: mycelium-folge198

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Registriertes Operator-Wort
(2026-09-28 folge196): **Commit erst, wenn alle anderen Sessions committet haben** — der
geteilte Baum trägt uncommittete Arbeit anderer Linien (sensory: folge200 + folge199→archiv;
river: `te.rs`/`doppler.rs`/`flyby_anderson_probe.rs`; mountain:
`pds3_table.rs`/`pds4.rs`/`extract.rs`), die nicht berührt wird.
