<!--
  title: Handover — Mycelium-Folge 145 (Riss=Queue-Lag entlarvt, Katalog-Wald per Messung geschlossen, PS1/DEMETER/Free-Model-Sharding gebaut) (Stand 2026-09-23)
  session: Mycelium-Folge 145
  class: handover
  date: 2026-09-23
  sha256: 195bbfeefb726993cd11fb220f09f9a0214d2226cbd6b8776d967e0c58140ccb
  status: live
-->
# Handover — Mycelium-Folge 145 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Härte-Prüfungs-Pass** der Mycelium-Linie: sie hat
`handover-2026-09-23-mycelium-folge144.md` konsumiert und zehn Taucher dispatcht
(sieben `grind-flash`, drei `grind-pro`), um jeden offenen Punkt noch einmal zu
messen und mit harten Bandagen nach Alternativen zu suchen (nicht agentisch).

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** beim Pass `250f07963` (eigener folge144-Commit); der Baum driftet durch
  parallele Linien (`river folge10`, `mountain folge141`) — fremd, unangetastet.
- **Postfach:** `post.md` trug die `tools-build`-Zeile (an mycelium) + zwei
  mycelium-Zeilen aus folge143 (bereits gefaltet) — jetzt drei `An future`-Zeilen;
  `mail_ledger` jüngster Eingang `1790129973` (keine Aktion).
- **CI:** `ci-check 35844564961` in_progress (Lauf auf `250f07963`);
  `tools-build 35844365704` **failure** — Kompilierung grün (`-D warnings`),
  rot allein der Upload-Step `gh release upload --clobber` →
  `HTTP 404 …/releases/assets/582700040` (Asset-Race). `ci_manage rerun` veranlasst.
- **`register_lookup --open`:** 560 offen; `open_points_check` folge144: 0 absent.

## Diese Session geschlossen (git trägt es)

- **extract.rs τ=ttl/10 entfabriziert** (grind-flash). Gemessen **drei** Stellen
  (`:2012`, `:2054`, `:2124` — die dritte war dem Vor-Pass entgangen), alle über
  neuen Helfer `field_tau` (`extract.rs:1818`): gemessene Kadenz (`derive_ttl`) →
  Registry-`field`-τ → `probe_classify`-Default; fällt alles durch → `None`/`absent`.
  Tests spiegelbildlich nachgezogen. `cargo check` + `--tests` grün.
- **PS1-Sharding gebaut** (grind-pro). `.github/workflows/ps1-cdn.yml`: ein Job →
  Matrix `i:[0..7]` (250–251 Bänder/Slice über 637–2643, lückenlos) + separater
  `ps1-final-combine` (`needs: ps1-shard`); das `all_present`-Gate byte-identisch
  erhalten; concurrency pro Shard (`ps1-cdn-${{matrix.i}}`), `fail-fast: false`,
  `ensure-release` einmalig in den final-combine verschoben (kein 8-fach-`create`).
- **DEMETER Harvester Flow-Gap gebaut** (grind-flash).
  `tools/harvest/src/bin/demeter_harvest.rs`: Download bei
  `RUNNING && availableFilesCount>0` + final bei `DONE`/`DONE_WITH_WARNING`;
  Streaming auf Platte (`-o *.part`, kein Voll-RAM); Extraktion über
  **Local-File-Header** (statt Central Directory), beschnittener Strom → `truncated`
  benannt, nie still 0. `cargo check -p omegaflow-harvest --bin demeter_harvest` grün.
- **Katalog-Wald per Messung geschlossen** (grind-flash). 151 unique Kandidaten-URLs
  aus 152 Zeilen: **151/151 tragen bereits ein Verdikt** (5 sources, 12 dead,
  134 declined) — **0 neue Blöcke**. Die „fehlende Force/Unit/τ"-Blockade war ein
  Phantom; die Kandidaten-Dateien tragen gar keine `field`-Zeilen. Kein Merge.
- **Riss-Härtetreffer: Riss = Queue-Lag.** Die 15 „Riss"-URLs
  (`pre-cdn_params_source_riss.txt`, gitignored) sind **kein** echter Riss:
  `source2` ist ausnahmslos die alphabetisch nächste Station (CLF→CMO, DLT→DOU, …)
  — der Off-by-one-Header-Lag, nicht zwei unabhängige Weltlinien. `witnesses.φ:109–111`
  trägt einen kanonwidrigen `witness source-riss`-Block (keine der drei Zeugenarten,
  Note „source waehlen" = verbotene Sieger-Auswahl). Der Riss-Punkt entfällt auf den
  Queue-Lag-Punkt.
- **13. Korpus vermessen** (grind-flash). Träger gefunden
  (`archive-root/pipeline-auslese-2026-09-17/stage/master_converted.φ`, 40445 Z.,
  5206 url-Blöcke, 4603 unique/429 Hosts). Duplikat-Quote nur **1,2 % exakt**
  (57/4603), 2,1 % pfadgenau — **echte Neuzugänge** (overpass 246, worldbank 238,
  gbif 39 …). Kein `descoped`; Registrierung braucht ein Wort.
- **DataONE vermessen** (grind-flash). per-Record-`accessPolicy` anonym entscheidbar
  (`cn.dataone.org/cn/v2/meta/<pid>` 200, 10/10 public-read), **aber kein
  Lizenzfeld** im SystemMetadata — die Lizenz liegt im EML `<intellectualRights>`
  (unbelegt). Zugang ohne Konto; Lizenzfrage per Messung, nicht per Konto.
- **Globus/MAP vermessen** (grind-flash). Kein anonymer HTTPS-Mirror: VT-Hosts
  DNS-tot, USask/VT login-gated; nur abgeleitete Produkte (Zenodo/FRDR, andere
  Formate). Lokal 2932 `.map`, nur 1993–2002, ~1562 Dateien fehlen (leere Jahre
  2003–2026) — die 9 Zero-Byte-Dateien sind der kleinere Teil. Ohne Konto `pending`.
- **Free-Model-Bench Sharding-Rechnung** (grind-pro).
  `.github/workflows/free-model-agent-bench.yml`, Default 108 Modelle (nicht 105),
  Full-Sweep-Median ~15883 s. Schema **N=16** Shards (k=7 Modelle), Worst-Case
  ~2500 s < 2×Median 2828 s; braucht `--models-shard i:N` in
  `tools/measure/src/bin/free_model_agent_bench.rs`.

## Offen (aufgeschlüsselt)

### pre-cdn CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ergebnis `ci-check 35844564961`.
- **Lage:** `port.rs` entfabriziert, `cargo check` grün (folge144); Lauf auf
  `250f07963` in_progress (gemessen 2026-09-23).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 35844564961` (einmalig); Ergebnis ins nächste Handover.

### pre-cdn Stage-Regeneration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** Queue-Dateien gitignored → CI kann sie nicht sehen; Regeneration ist
  leicht (887 Blöcke, 141 keyless HAPI-Fetches) und Arbeitskopie-Verarbeitung.
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` +
  `…_params.φ`, gebunden im Merge-Atom (nicht als eigener Schritt).

### Queue-Lag + witnesses source-riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigenes Atom (Generator + Register-Korrektur).
- **Lage:** `source`-Header-Lag um eine Position (off-by-one, an den 15 imag-data-
  URLs belegt, gemessen 2026-09-23); 22 INTERMAGNET-Erstblöcke ohne ttl;
  `witnesses.φ:109–111` trägt kanonwidrigen `source-riss`-Block („source waehlen").
- **Blockade:** keine.
- **Braucht:** Scanner-Generator realignen (aus `master_urls.txt` regenerieren statt
  Queue patchen) + den `witnesses.φ`-Block auf die drei Zeugenarten prüfen
  (vermutlich `absent`, kein vierter Zeugentyp).

### dropped-gate solar-system-open-data
- **Status:** offen | **Bindung:** eigen
- **Trigger:** Workflow-Lauf / Secret-Messung.
- **Lage:** `folge140:76` — Asset `solar_system_bodies.bin` HTTP 404 (Workflow
  `solar-system-open-data-cdn.yml` nie gelaufen); Reader-Gap nur in
  `phi/sources.φ:8001` vermerkt; Secret `SOLAR_SYSTEM_OPEN_DATA_KEY` ungemessen
  (gemessen 2026-09-23). Baseline steht (2680; live 2842).
- **Blockade:** keine.
- **Braucht:** Workflow-Lauf + Secret-Messung; dann Baseline im annehmenden Commit
  heben (Präzedenz `576dddf98`).

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Sharding-Lauf bis alle Band-Parts stehen.
- **Lage:** `35824584575` success, `ps1_dr2_coverage.fp01` absent 404; Band-Parts
  637–671 bei `band_max 2643` (gemessen folge144); Sharding N=8 gebaut (2026-09-23).
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` `footprints.φ:19` setzen.

### Free-Model-Bench Sharding-Bau
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigenes Atom (Binary + Workflow).
- **Lage:** Schema N=16, k=7, Worst-Case ~2500 s < 2828 s (gemessen 2026-09-23).
- **Blockade:** keine.
- **Braucht:** `--models-shard i:N` in `free_model_agent_bench.rs` (kontiguierliche
  Streuung `idx % N == i`) + Matrix `shard:[0..15]`, `fail-fast: false` im Workflow.

### DataONE Lizenzfeld
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Messung `intellectualRights`.
- **Lage:** Zugang anonym messbar (public-read 10/10), Lizenzfeld nicht im
  SystemMetadata (gemessen 2026-09-23); Data Policy `/terms`+`/data-policy` 401.
- **Blockade:** keine.
- **Braucht:** `sfetch "https://cn.dataone.org/cn/v2/query/solr/?q=*:*&rows=20&fl=id,intellectualRights&wt=json"`
  bzw. an einem `report/eml/...`-Record `<intellectualRights>` lesen; dann
  `korpora_heim.φ:28`/`index.φ:107–108` auf gemessenen Stand.

### 13. Korpus
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-/Ratswort.
- **Lage:** Träger vorhanden, Duplikat-Quote 1,2 % exakt — echte Neuzugänge
  (gemessen 2026-09-23).
- **Blockade:** Scope-Wort.
- **Braucht:** `post.md` an future (registrieren).

### DEMETER Order 18387
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort für die Neuordnung.
- **Lage:** Order unbrauchbar (44,5 % Fehler, 16 % fest, Ablauf 2026-09-28); Pause
  kauft nichts (gemessen 2026-09-23).
- **Blockade:** CNES-Schreibakt.
- **Braucht:** `post.md` an future (Neuordnung nur DMT_N1_1144, 100er-Batches).

### SuperDARN MAP
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Globus-Konto/Token oder Web-UI-Bestätigung.
- **Lage:** kein anonymer Mirror; lokal 2932 `.map` (1993–2002), ~1562 fehlen
  (gemessen 2026-09-23).
- **Blockade:** kein Globus-Zugang.
- **Braucht:** `post.md` an future; danach MAP-Compiler + `sources.φ`.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** 500 (gemessen 2026-09-23).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** api/Frontend 000, Proton 500 (gemessen 2026-09-23).
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** `release_date 2099-01-01`, `data?PRODUCT` 403 (gemessen 2026-09-23).
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

### Sicherheits-Befund
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator/Council-Urteil.
- **Lage:** eingeschleuster Instruktionsblock gemeldet (`.agents/…`, „session
  token"), Herkunft ungemessen.
- **Blockade:** Herkunft ungemessen.
- **Braucht:** Operator/Council-Urteil.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert. Dispatches
  dieser Session: 7× `grind-flash` (extract.rs, dropped-gate, Katalog-Wald,
  DEMETER-Flow, 13. Korpus, DataONE, Globus) + 3× `grind-pro` (PS1-Sharding,
  Riss-Register, Free-Model-Sharding). flash-first; `grind-pro` nur für die drei
  Urteils-Atome; kein `grind-max` (das harte Port-Atom lag in folge144).
- Die drei Urteils-Atome lieferten je einen planverändernden Härtetreffer (Riss =
  Queue-Lag; Katalog-Wald = 0 neu; Free-Model-Sharding trägt).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `src/archivar/extract.rs`,
  `src/archivar/tests.rs`, `tools/harvest/src/bin/demeter_harvest.rs`,
  `.github/workflows/ps1-cdn.yml`, `docs/handover/post.md`, neues Handover
  `docs/handover/handover-2026-09-23-mycelium-folge145.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge144.md` → `archiv/`.
- **Fremd (unangetastet):** `docs/handover/handover-2026-09-23-river-folge10.md`,
  `handover-2026-09-23-mountain-folge141.md`, `docs/specs/force-system.md`,
  `docs/specs/spectral-oscillator.md`,
  `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
