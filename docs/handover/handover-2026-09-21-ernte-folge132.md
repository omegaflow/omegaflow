<!--
  title: Handover — Ernte-Folge 132 (Stand 2026-09-21)
  session: Ernte-Folge 132
  class: handover
  date: 2026-09-21
  sha256: 731b83b6eab788fe8e9870c809c83836982ab9c5386bc13050c07b1305e7485a
  status: live
-->
# Handover — Ernte-Folge 132 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Wartestellungen sind kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-21, Folge 132)

- **HEAD** `0c0b30fb` beim Start; Arbeitsbaum trägt Fremdarbeit (entscheid
  `folge78` + Move `folge77`, `external-state.md`; forschung `folge134` + Move
  `folge133`, `KERNEL_INDEX.md`, `lead-geometry-direction.md`) — nicht angetastet.
- **Postfach** — kein neues `An ernte`; `post.md` trägt `An bau` (text_review) +
  neu `An entscheid` (DEMETER ISL, diese Folge).
- **CI** — Massen-`-cdn`-Dispatch 10:14–10:16: **33 Läufe über 27 Workflows**
  gescheitert; Ursache gemessen: die Workflows tragen **keinen `actions/checkout`-Step**
  (`error: could not find Cargo.toml in /home/runner/work/omegaflow/omegaflow`).
  Erfolge: fai-kz `35591261870`, ia2 `35591241104`, de44 `35591251300`,
  neptune-de440s `35591254999`; eve `35591258884` in_progress.
  - babamul `35591238287` failure: `BABAMUL_API_TOKEN absent` — Secret existiert
    (`gh secret list`), Workflow-`env` fehlte.
  - ned `35591248345`/`35591264600` failure: Release `ned.ipac.caltech.edu-ned` fehlt.
  - psr-cdn `35587702458`/`35587801743`: TAP `curl (22) 400` → query void (quellseitig).
- **Zustand** — `external-state.md` von entscheid fortgeschrieben (fremd); der
  CI-Status-Eintrag wurde nicht angetastet (fremder Hunks in derselben Datei).

## In dieser Folge gebaut (git trägt)

- **29 CDN-Workflows gefixt:** babamul (`BABAMUL_API_TOKEN`-env), ned
  (Release-Create-Step), tess + 26 weitere (Checkout+Toolchain ergänzt:
  avo, bzcat5, cbdata, chandra-csc, corot, denis, exoplanets, first14, frb-a279,
  frbcat, gcvs, lmxb, magnetar, merlin, mktypes, nvss, pangaea, pastel, polarbase,
  rave, sb9, sncat, swiftgrb, tevcat, vsx, wds).
- **sha256 aufgenommen** (gemessen via `archive_search --sniff`, in `sources.φ`):
  fai_kz_obscore `65b9e7c8…` (244025 B), ia2_wgesdss `03a15151…` (100013 B),
  de440/de442 × earth/moon/sun (6× `…`, 6629784 B), neptune `21991403…` (1808616 B).
  `ledger.φ:10` → `kompiliert`.
- **neptune_ephemeris_compiler-Registerblock** in `sources.φ` angelegt
  (`ssd.jpl.nasa.gov-neptune`, de440s.bsp).

## Offen (aufgeschlüsselt)

### Re-Dispatch der 29 gefixten Workflows + sha256-Nachzug
- **Status:** wartend | **Bindung:** eigen
- **Lage:** 29 Workflows gefixt (git `M`), noch nicht gepusht/dispatcht.
- **Blockade:** Commit/Push-Wort.
- **Braucht:** nach Push `gh workflow run <file>` je Workflow; bei success
  sha256 → `sources.φ`-note → `kompiliert`.

### fai.kz Feld-Klassifikation
- **Status:** offen | **Bindung:** eigen
- **Lage:** obscore 5048 Zeilen (image 3003/spectrum 2045); kein skalarer Fluss.
- **Blockade:** Oszillator-Gate-Entscheid.
- **Braucht:** Feld-/force-/τ-Zeilen festlegen (`pending`).

### de440_neptune `-de`-Block (404)
- **Status:** offen | **Bindung:** eigen
- **Lage:** `sources.φ` Block `ssd.jpl.nasa.gov-de/ephemeris_de440_neptune.bin`
  → sniff 404; `de_compiler`-Scope ist sun/moon/earth. Der gültige Block ist
  `ssd.jpl.nasa.gov-neptune` (`21991403…`).
- **Blockade:** keine.
- **Braucht:** stale Block prüfen und disponieren/entfernen.

### psr-cdn TAP HTTP 400
- **Status:** offen | **Bindung:** eigen
- **Lage:** psr-cdn `35587702458`/`35587801743` — `curl (22) 400` → query void;
  Workflow trägt checkout.
- **Blockade:** quellseitig (VizieR/TAP).
- **Braucht:** TAP-Query-Adresse/Format messen.

### Quaoar Sternbedeckung — Manifestation
- **Status:** wartend | **Bindung:** termin (Run)
- **Lage:** Arm/Compiler/Workflow stehen; `sources.φ:8706` Block pending sha256;
  Workflow-Tag `zenodo.org` konsistent.
- **Blockade:** Lauf.
- **Braucht:** `gh workflow run quaoar-occlt-cdn.yml` (nach Push); bei success
  sha256 → `sources.φ:8706` / `ledger.φ:32`.

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin
- **Lage:** `footprints.φ:19` — Asset `ps1_dr2_coverage.fp01` absent;
  Workflow-Tag `ssd.jpl.nasa.gov-ps1`; kein `sources.φ`-Eintrag (Footprint).
- **Blockade:** Lauf.
- **Braucht:** `gh workflow run ps1-cdn.yml` (nach Push); bei final → `footprints.φ:19`.

### DEMETER (CDN + ISL)
- **Status:** wartend / operator-gebunden | **Bindung:** termin + operator
- **Lage:** CDN-Lauf `35567568429` queued; ISL account-gated (`blocked_sources.φ:70`).
- **Blockade:** Lauf / Konto.
- **Braucht:** `ci_manage view 35567568429`; ISL-Konto → entscheid (`post.md:20`).

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (CSES-02-Umbau)
- **Lage:** `ledger.φ:26` — PI „wait a few weeks"; kein Follow-up fällig.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### EPA RadNet Koordinaten
- **Status:** wartend | **Bindung:** termin (FRS)
- **Lage:** `blocked_sources.φ:74` — FRS `ofmpub get_facilities` 503.
- **Blockade:** FRS-Maintenance.
- **Braucht:** FRS-Retry `pgm_sys_acrnm RadNet`.

### EMODnet maxTime
- **Status:** wartend | **Bindung:** termin (Host)
- **Lage:** `ledger.φ:18` — `erddap.emodnet-physics.eu` HTTP 0.
- **Blockade:** Host.
- **Braucht:** Re-Messung bei Erreichbarkeit.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Route)
- **Lage:** `ledger.φ:14` — direct+proton HTTP 0, wayback 503.
- **Blockade:** Route.
- **Braucht:** `bin/proton-wg.sh suggest pithia.cbk.waw.pl` (Operator-Wort).

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten, BepiColombo bc_mpo_more,
  NRS02-10/12/13 SHAPE, EMODnet HFRADAR NADR (Re-Messung fällig 2026-10-19).

## Benchmark

- Kein Doppel-Lauf diese Folge: A/C liefen `grind-flash` (sha256/sniff bzw.
  post/Bereitschaft), B lief `grind-pro` (27-Workflow-Klassifikation) — keine
  flash/pro-Paarung auf derselben Aufgabe, kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** 29 `.github/workflows/*-cdn.yml`, `phi/sources.φ`,
  `phi/pipeline/ledger.φ`, `docs/handover/post.md`, neues Handover
  `handover-2026-09-21-ernte-folge132.md`, Move
  `handover-2026-09-21-ernte-folge131.md` → `archiv/`.
- **Fremd (nicht angetastet):** entscheid (`folge78`, Move `folge77`,
  `external-state.md`), forschung (`folge134`, Move `folge133`, `KERNEL_INDEX.md`,
  `lead-geometry-direction.md`).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
