<!--
  title: Handover — Mycelium-Folge 146 (CI-Verifikation entlarvt Port-Rot, Kernel-Rat, Sharding/Dual-Comb geschlossen) (Stand 2026-09-23)
  session: Mycelium-Folge 146
  class: handover
  date: 2026-09-23
  sha256: b4ad062e7bcb79e5b8f726ddbba337b4abe9f52138ae3c0e1cd6cceaff52e19a
  status: live
-->
# Handover — Mycelium-Folge 146 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-23-mycelium-folge145.md` konsumiert und die
sechs parallel abarbeitbaren Punkte aus dem Planungs-Pass dispatcht (4×
`grind-flash`, 2× `grind-pro`) + den Rat für ein Kernel-Urteil.

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** beim Pass `145cbe612` (river folge11); `git_safety --snapshot`:
  Arbeitsbaum == HEAD, nichts zu sichern.
- **Postfach:** `state/mail/` absent, `mail_digest absent`; `post.md` trug die
  `An mycelium`-Dual-Comb-Zeile (river folge11) — **gefaltet + gelöscht**; drei
  `An future`-Zeilen bleiben stehen (13. Korpus, DEMETER, SuperDARN).
- **CI:** `ci-check`-Serie rot. Ursache gemessen: der deterministische Port-Test
  `test_port_block_hapi_live_measure_classifies_em` (Fabrication, s.u.) + 4
  `clippy len_zero` in `tests.rs` + `dropped-gate` (Baseline 2680, live 2826
  bzw. 2893). `solar-system-open-data-cdn.yml` nie gelaufen → dispatcht
  (`35861691075`).
- **`register_lookup --open`:** gelaufen; `open_points_check` folge145: 1 absent
  (`:227` nennt `river-folge10.md`, existiert nicht mehr — Fremd-Liste, mit dem
  Archiv-Move erledigt).

## Diese Session geschlossen (git trägt es)

- **pre-cdn CI-Verifikation entlarvt echtes Rot.** `ci-check 35844564961`
  (head `1d23d352`) ist **failure**, kein Transient: `build`/`format` grün, rot
  aber `test` (Assert `tests.rs:6876`), `clippy` (4× `fields.len() == 0`) und
  `dropped-gate` (`delta 146`). **Rat-Verdikt einstimmig:** die Test-Erwartung
  `gaussian-inverse-square em nT` ist die Fabrication (Kopie-Residue aus dem
  Nachbartest `…seismic-body`, ohne die Kraftgrenze zu beachten);
  `inverse-square` ist die Wahrheit (Code `default_kernel_for("em")` +
  Register-Familie `intermagnet_*_nt`). Konfund benannt: die legitimen
  `gaussian-inverse-square em`-Registerzeilen sind PSF-Magnituden, nie
  magnetische nT-Felder. **Fix:** Test-Erwartung auf `inverse-square`
  (`tests.rs:6884`); die vier Clippy-Lints auf `is_empty()` (`tests.rs:2619`,
  `2630`, `2641`, `2653`). `cargo check --tests` 0/0.
- **Queue-Lag (port_mode realigned).** Der `source`-Header-Lag ist
  Queue-Lag, kein Riss — belegt: `source2` ist ausnahmslos die alphabetisch
  nächste Station. Kein committeter Queue-Generator; der eigentliche Fehler saß
  im **Konsumenten** `port.rs::port_mode`: er behandelte die mitten im Block
  stehende `source`-Direktive als Blockgrenze und ordnete `ttl`/`force` dem
  Folgblock zu. Fix: nur `url` begrenzt einen Block, `source` fällt wieder als
  Direktive (deckt sich mit `SOURCE_PORT.md` §9). Die 22 INTERMAGNET-Erstblöcke
  ohne ttl waren diese Fehlzuordnung. Kanonwidrigen `witness source-riss`-Block
  (`witnesses.φ:109–111`, vierter Zeugentyp + verbotene „source waehlen") als
  `absent` entfernt. `cargo check --tests` 0/0.
- **dropped-gate / solar-system-open-data vermessen.** Secret
  `SOLAR_SYSTEM_OPEN_DATA_KEY` **vorhanden** (Name in `.secrets.local`; Wert
  nicht gelesen). Workflow `solar-system-open-data-cdn.yml` triggert nur auf
  `workflow_dispatch` und war nie gelaufen → **dispatcht** (`35861691075`).
  Reader-Gap exakt: kein `solar_system_bodies`-Arm in `extract.rs:77` und in
  der Series-Format-Liste `main_flow.rs:2185` (nur Registerzeile
  `sources.φ:8001`). **Baseline nicht gehoben:** 2680 vs live 2893; die Hebung
  gehört in den annehmenden Commit bei aufgelösten Drops.
- **Free-Model-Bench Sharding gebaut.** `--models-shard i:N` in
  `free_model_agent_bench.rs` (Streuung `idx % N == i` über die volle
  `free_models.tsv`-Liste, `0 <= i < N`, sonst `exit(2)`; ohne Flag unverändert).
  Workflow: Matrix `shard:[0..15]`, `fail-fast:false` + neuer `merge`-Job
  (lädt `*-shard-*` mit `merge-multiple`, header-dedupliziert zu einer TSV) —
  behebt die 16-fach-Überschreib-Konfliktstelle. `cargo check -p
  omegaflow-measure --bin free_model_agent_bench` 0/0.
- **DataONE per-Record-Lizenz gemessen.** Kein Lizenzfeld in
  SystemMetadata/Solr (`intellectualRights:*`/`license:*` → HTTP 400 „undefined
  field"); Lizenz nur im Metadatenobjekt: EML `<intellectualRights>` Freitext
  (teils CC-URL), schema.org JSON-LD strukturiertes `license` (CC-URL + Name),
  **kein SPDX**. `korpora_heim.φ:28` auf ≤256-Zeichen-Stand gekürzt,
  `index.φ:108`-Note nachgezogen. Katalog-/Terms-Lizenz weiter 401.
- **Dual-Comb (post.md-Zeile).** Alle fünf Zenodo-Kandidaten
  (`2542265`, `6413816`, `6451876`, `10709448`, `15095848`) gemessen:
  **declined `no-physical-force`** — Detektorstrom zweier aktiv phasenstarrer
  Kämme = Aktuator-Signal, kein propagierendes Feld der 9 Medien. 5 Einträge in
  `declined_sources.φ`; post.md-Zeile gefaltet + gelöscht.

## Offen (aufgeschlüsselt)

### pre-cdn CI-Grün bestätigen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf auf dem folge146-Commit.
- **Lage:** Port-Test + Clippy-Lints lokal geheilt, `cargo check --tests` 0/0
  (gemessen 2026-09-23); CI-Bestätigung steht aus. `dropped-gate` bleibt bis zur
  Baseline-Hebung rot (eigener Punkt).
- **Blockade:** keine.
- **Braucht:** nach Push `ci_manage view <id>`; Ergebnis ins nächste Handover.

### dropped-gate solar-system-open-data Reader
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ergebnis Workflow-Lauf `35861691075`.
- **Lage:** Asset `solar_system_bodies.bin` 404; Workflow dispatcht
  (gemessen 2026-09-23); Reader fehlt (kein Arm in `extract.rs:77` /
  `main_flow.rs:2185`); Baseline 2680 vs live 2893.
- **Blockade:** Reader-Bau.
- **Braucht:** `ci_manage view 35861691075`; Reader-Arm für
  `solar_system_bodies` (Body-Catalog, nicht `series_parse_bin`); dann Baseline
  in `docs/zustand/dropped-baseline.md:16` im annehmenden Commit heben
  (Präzedenz `576dddf98`).

### pre-cdn Stage-Regeneration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** Queue-Dateien gitignored → CI sieht sie nicht; Regeneration leicht
  (887 Blöcke, 141 keyless HAPI-Fetches).
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` +
  `…_params.φ`, gebunden im Merge-Atom (nicht als eigener Schritt).

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Sharding-Lauf bis alle Band-Parts stehen.
- **Lage:** `35824584575` success; `ps1_dr2_coverage.fp01` absent 404;
  Band-Parts 637–671 bei `band_max 2643`; Sharding N=8 gebaut.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` `footprints.φ:19` setzen.

### DataONE Katalog-/Terms-Lizenz
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Auth-Zugang oder Operator-Wort.
- **Lage:** `/terms`+`/data-policy` → HTTP 401 (Basic realm, Apache), kein
  Wayback-Snapshot (gemessen 2026-09-23); per-Record-Lizenz gemessen, Katalog-
  /Terms-Klausel unbelegt.
- **Blockade:** Auth/Operator.
- **Braucht:** DataONE Terms via Auth/Operator (`www.dataone.org/terms`).

### 13. Korpus
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-/Ratswort.
- **Lage:** Träger vorhanden, Duplikat-Quote 1,2 % exakt — echte Neuzugänge
  (gemessen 2026-09-23).
- **Blockade:** Scope-Wort.
- **Braucht:** `post.md` an future — **steht** (`post.md:11`).

### DEMETER Order 18387
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort für die Neuordnung.
- **Lage:** 44,5 % Fehler, 16 % fest, Ablauf 2026-09-28 (gemessen 2026-09-23).
- **Blockade:** CNES-Schreibakt.
- **Braucht:** `post.md` an future — **steht** (`post.md:13`).

### SuperDARN MAP
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Globus-Konto/Token oder Web-UI-Bestätigung.
- **Lage:** kein anonymer Mirror; lokal 2932 `.map` (1993–2002), ~1562 fehlen
  (gemessen 2026-09-23).
- **Blockade:** kein Globus-Zugang.
- **Braucht:** `post.md` an future — **steht** (`post.md:15`).

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

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert.
  Dispatches dieser Session: 4× `grind-flash` (CI-Log, dropped-gate/Secret,
  Free-Model-Sharding, DataONE) + 2× `grind-pro` (Queue-Lag/Register,
  Dual-Comb-Force-Gate) + 1× `council` (Kernel-Urteil). flash-first; `grind-pro`
  nur für die zwei Urteils-Atome; kein `grind-max`.
- Das Rat-Atom (Kernel) lieferte den planverändernden Härtetreffer: die
  Test-Erwartung war die Fabrication, nicht der Code.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `src/archivar/tests.rs`,
  `src/archivar/port.rs`, `phi/witnesses.φ`, `phi/declined_sources.φ`,
  `phi/pipeline/catalog/korpora_heim.φ`, `phi/pipeline/index.φ`,
  `tools/measure/src/bin/free_model_agent_bench.rs`,
  `.github/workflows/free-model-agent-bench.yml`, `docs/handover/post.md`, neues
  Handover `docs/handover/handover-2026-09-23-mycelium-folge146.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge145.md` →
  `archiv/`.
- **Fremd (unangetastet):** `docs/handover/handover-2026-09-23-river-folge11.md`,
  `handover-2026-09-23-mountain-folge141.md`, `handover-2026-09-23-sensory-folge152.md`.
- **Dispatch nach Push:** `.github/workflows/free-model-agent-bench.yml` ist
  geändert → `gh workflow run free-model-agent-bench.yml` nach dem Commit.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
