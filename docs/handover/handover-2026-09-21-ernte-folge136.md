<!--
  title: Handover — Ernte-Folge 136 (Ordnung: Register geschnitten, gegrindete Pools ausgelagert, Lizenz-Verdikte geschlossen) (Stand 2026-09-21)
  session: Ernte-Folge 136
  class: handover
  date: 2026-09-21
  sha256: cad5f4e54cafc5e870670bee162e5fa211cccf8d6ae464ba73798629a9cb6cb4
  status: live
-->
# Handover — Ernte-Folge 136 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt ist aufgeschlüsselt: **Lage** / **Blockade** /
**Braucht**; Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Folge 136)

- **HEAD** beim Start `af25d752` (fmt-Heilung); während der Session zogen zwei
  fremde Linien nach: `7c2d78ff` future folge85, `3b42cb72` sensory folge142 —
  HEAD jetzt `3b42cb72`.
- **Postfach** — `post.md` trägt zwei fremde Zeilen (An future `te.rs:1574` clippy;
  An mountain Riss 4); keine an Mycelium.
- **CI** — dispatcht: `babamul-cdn` `35649296996` (queued), `radnet-cdn`
  `35649300152` (queued). Watchdog aktiv: `allwise-cdn`, `ci-check`, `ps1-cdn`,
  `te-gate`, `free-model-bench`.
- **git_safety** — Snapshot `refs/safety/1790020266` (Start).
- **register_lookup --open** — nach dem Schnitt: ledger 4, index 31, witnesses 4,
  footprints 2, sources 1, nrs 1 offen.

## Ordnung — was diese Session tat (ein Rutsch)

- **Register geschnitten:** `ledger.φ` Terminal-Zustände gestrichen (kompiliert ×7,
  disponiert ×12) → offen 11→4. `index.φ` Terminal-Zustände gestrichen (descoped
  ×33, erledigt ×4, ausgelagert ×4) und die 10 portierten Queue-Zeilen entfernt →
  offen 41→31.
- **Gegrindete Pools ausgelagert** (Operator-Wort: „die pools die wir schon
  bearbeitet haben sollen nicht mehr in der pipeline verhungern"): 53 Dateien (10
  portierte `sources_*_untested_*` + 43 descopte `grind_*`) + `survivors_2026-09-20/`
  nach `archive-root/pipeline-auslese-2026-09-21/` (63 Dateien, `NOTE.md` mit
  Rückweg). Queue = 2 (nur pre-cdn).
- **Lizenz-Verdikte geschlossen:** `korpora_heim.φ` — 135 Korpora verdiktet (vorher
  16): **57 redistribute, 71 decline, 7 pending**. Zwei Durchgänge: der erste ließ 76
  pending; der zweite (Wayback-Pfad) löste 69 davon (ESO → CC BY 4.0, GBIF/iNaturalist
  via Wayback, arXiv CC0, CMR public domain, 56 TAP-Einzelhosts → decline, kein
  Weitergabe-Statement).
- **survivors gefaltet:** bereits durch `f7a92c79` (180 URLs: 18 `sources.φ`, 162
  `declined_sources.φ`, 0 neu) — Ordner ausgelagert.
- **Babamul + EPA RadNet** dispatched (run-ids in den Registern).

## Der Wald (Katalog, gemessen)

137 Inventare, 1.316.419 Zeilen. Rollen (`MANIFEST.φ` vollständig): 77 tap,
38 inventory, 10 vocabulary, 4 asteroid, 3 map, 3 disposition, 1 special. Lizenz:
57 redistribute / 71 decline / 7 pending.

## Offen (aufgeschlüsselt)

### Babamul CDN-Manifestation
- **Status:** ausstehend | **Bindung:** eigen
- **Lage:** CI `35649296996` (babamul-cdn) dispatched 2026-09-21, queued; note in `phi/sources.φ:14016` + `ledger.φ:20`.
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** einmal `ci_manage view 35649296996` → sha256/Größe in `sources.φ:14016` + `ledger.φ`.

### EPA RadNet AGOL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** CI `35649300152` (radnet-cdn) dispatched, queued; `blocked_sources.φ:63`, `sources.φ:1164`.
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** einmal `ci_manage view 35649300152` → `unjoined`-Zähler in die note.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10-12`; `/tap/tables` HTTP 500 (PostgreSQL localhost:5432 refused).
- **Blockade:** Pithia-DB.
- **Braucht:** Re-Messung bei 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14-16`; Portal 200, keine neue Anleitung.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### DEMETER ISL Parser-Gap
- **Status:** blockiert | **Bindung:** linie:mountain
- **Lage:** `ledger.φ:22-24`; `demeter.rs:60` verlangt `ISL SURVEY`, `DMT_N1_1143` trägt `ISL BURST`.
- **Blockade:** Parser (mountain).
- **Braucht:** mountain: Marker auf `ISL SURVEY|ISL BURST` erweitern.

### pre-cdn Pools
- **Status:** offen | **Bindung:** eigen
- **Lage:** `phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ` und `phi/pipeline/queue/sources_potential_pre-cdn_params.φ` (9045 + 561 Zeilen).
- **Blockade:** keine.
- **Braucht:** grind/port (Join-Quelle der Lost-Blocks).

### Katalog-Lizenz pending
- **Status:** offen | **Bindung:** eigen
- **Lage:** 7 `pending` in `korpora_heim.φ`: dataone (terms 401), bcodmo + erddap_bcodmo (Policy 404), bodc (Policy 404), esa_eogateway (JS ohne Lizenzklausel), gfz_igets (Terms nicht auffindbar), gaia_swpc (gemischt © ESA + PD).
- **Blockade:** Seite 404/401 bzw. gemischter Provider.
- **Braucht:** Einzelmessung (dataone Auth, BCO-DMO/BODC Policy-Weg, GFZ-DOIDB, Gaia/SWPC-Anteile trennen).

### Katalog-Inventare (der Wald)
- **Status:** offen | **Bindung:** eigen
- **Lage:** `index.φ` 28 Inventar-Zeilen, 1,32 Mio Zeilen.
- **Blockade:** keine.
- **Braucht:** grind (Blöcke extrahieren).

### SuperDARN MAP — Transfer läuft
- **Status:** offen | **Bindung:** eigen
- **Lage:** Gruppen angenommen (`MAP files`, `fitacf_25`, `fitacf_30`; `rawacf` weder beantragt noch gewährt). GCP 3.3.1 registriert (`omegaflow`, id `35cd9948-b600-11f1-bb02-02ffe792127d`) + läuft (`connected`). Transfer-Task `af68c4f1-b601-11f1-b9a2-0affd5e180af`: SuperDARN Mirror `/local_data/map/` → `data/superdarn/map/` — 6.561 Dateien, 105 Verzeichnisse, 21,93 GB, ACTIVE.
- **Blockade:** keine (Transfer läuft im Hintergrund).
- **Braucht:** nach Abschluss MAP-Compiler bauen + in `phi/sources.φ` registrieren (CDN-Manifestation; 2-GB-Grenze → ein kompiliertes Record). FITACF bereits gebaut (`sources.φ:9465`). RAWACF bleibt benannter späterer Punkt.

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), BepiColombo, NRS SHAPE, EMODNET HFRADAR NADR (Re-Messung
  fällig 2026-10-19).

## Benchmark

- Kein Doppel-Lauf: die Routine-Klasse ist geschlossen (flash-Sieger). `korpora_heim`
  lief `grind-pro` (Lizenz-Verdikt = Urteilsklasse), `survivors` + `babamul/radnet`
  liefen `grind-flash` (mechanisch) — keine flash/pro-Paarung auf derselben Aufgabe,
  kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `phi/pipeline/ledger.φ`, `phi/pipeline/index.φ`,
  `phi/pipeline/catalog/korpora_heim.φ`, `phi/blocked_sources.φ`, `phi/sources.φ`,
  neues Handover `handover-2026-09-21-ernte-folge136.md`, Move
  `handover-2026-09-21-ernte-folge135.md` → `archiv/`.
- **Fremd (nicht angetastet):** `src/archivar/hdf5.rs` +
  `.github/workflows/hdf5-real-granule.yml` (andere Linie, uncommittet); die
  Fremd-Commits future folge85 / sensory folge142.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
