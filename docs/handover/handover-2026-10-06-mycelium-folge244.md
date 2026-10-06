<!--
  title: Handover — Mycelium-Folge 244 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass in einem Atom; API-Modell-Test, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: 72ad1d0e7e0c24d453e74342b4b575ab2731a3ad32cb636ef09eb49af55e8d36
  status: live
-->
# Handover — Mycelium-Folge 244 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge243.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0406

`session_burn`; `.tools_ensure archive_search|sgrep|sfetch|smail|ci_manage`: frisch.

## Freie Stimmen — API-Modell-Test (Operator-Frage 2026-10-06 „hast du die api modelle getestet?")

- **Der CI-Lauf testete die 5 neuen Anbieter nicht:** `free-model-bench 37487436436`
  (`sambanova,orcarouter,alibaba,ovhcloud,mistral`) = success, aber das Artefakt
  `free-model-bench.tsv` trägt für **alle 11 Modelle** `pending_no_key` — der
  `FREE_MODEL_KEYS`-Secret führt keinen der neuen Provider-Keys
  (gemessen 2026-10-06 via `gh api …/artifacts/11425169303/zip`).
- **Lokal real getestet** (`free_model_bench --task T5`, 12 Worker, gemessen
  2026-10-06; Ergebnis `state/mycelium/free-model-bench-2026-10-06.tsv`): die
  Provider mit lokalem Key (auth.json/`.secrets.local`) antworten. **Pass 3/3:**
  `kilo dots-3-note-preview`, `kilo kilo-auto/free`, `kilo nvidia/nemotron-3-super-120b`,
  `nvidia meta/llama-3.2-11b/90b-vision`, `nvidia nvidia/nemotron-3-super-120b`,
  `nvidia openai/gpt-oss-20b`, `google gemini-3.1-flash-lite`, `google gemini-3.5-flash-lite`.
  **2/3:** `kilo openrouter/free`, `kilo ling-3.0-flash-sante`, `nvidia nemotron-3-ultra-550b`,
  `google gemini-3.8-flash`. **Tote Arme (registriert):** `nvidia mistralai/mistral-nemotron`
  = http_410 „end of life 2026-09-28" → `struck`; `google gemini-2.5-flash` = http_404
  „no longer available to new users" → `struck`; `mistral mistral-large-latest` = http_403
  `tier_not_allowed` auf dem Free-Key → `blocked`. **Transient:** `cloudflare-workers-ai`
  alle 11 = `pending_rate_limited` („daily free allocation of 10,000 neurons" verbraucht,
  Tagesquote); `mistral-medium-latest` = 429.
- **Offen (Operator/per-act):** Keys für SambaNova, OrcaRouter, Alibaba Model Studio,
  OVHcloud, Mistral-Free in CI (`FREE_MODEL_KEYS`) — erst dann sind ihre `voice-*`-Agenten
  baubar; die `free_models.tsv`-Dispositionen sind auf Messung korrigiert.

## Offen — eigen

### Daten-Holdings — `opencode-tmp`-Dump CDN-Bedarf/Uniqueness
- **Status:** eigen
- **Trigger:** je Posten CDN-Bedarf/Uniqueness gemessen → Move/Delete je Bestand
- **Lage:** (gemessen 2026-10-06) vier Bestände ausgeführt: `wind_orbit`/`dr3_stars` verortet,
  Snapshot-Dedup 270 MiB, archive-state-Hardlink 1,21 GiB; `nvss.json` liegt korrekt unter
  `data/ssd.jpl.nasa.gov-nvss/nvss.json` (`sources.φ:17784`); `radio.bin` live + registriert.
  Offen: `~/archive/knowledge/data/opencode-tmp-2026-09-01/` = 13 GB/11 603-Dateien-Dump.
- **Blockade:** Umfang (11 603 Dateien) — je Posten Einzelmessung.
- **Braucht:** je Top-Dir `du`-Größe + Uniqueness vs Register/`archive_search`; Detail
  `state/future/holdings-migration-2026-10-06.md:26`. **Träger** für
  `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** blockiert
- **Trigger:** Sources-Zeilen je pending Domäne im Baum → `.te`-Descriptor + Workflow-YAML → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md`
  (12 Domänen); 4 Kern-x-Serien erreichbar/registriert; 2 Arme gebaut (WQP + EEA-noise).
  Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `field_te_query.rs:580-684`.
  y-Serien unregistriert, erreichbar (`archive_search`, alle 200).
- **Blockade:** Mountain-Verdikt (Admission) je y-Quelle.
- **Braucht:** je pending Domäne die Sources-Zeile (Mountain); dann `.te` je Klasse
  + `.github/workflows/exposom-matrix-te.yml`.

### `phi/blocked_sources.φ` — Mycelium-Klasse
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06) ExoMars TGO ACS / Viking gravity / Hayabusa LIDAR / Phobos-2 KRFM —
  Workflows queued; EUMETSAT MTG-LI queued; Chandrayaan-1 Mini-RF blockiert (`pds3_img` ohne Feld-Arm);
  Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal = Operator-Hand; `:78` SuperDARN LOCK.
- **Blockade:** Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent (Operator/per-act).

### `ledger.φ:2`/`:6` — Port-Runner
- **Status:** wartend
- **Trigger:** Korpus-Input `phi/pipeline/queue/<korpus>.φ` am Datenträger → `omegaflow --port`
- **Lage:** (gemessen 2026-10-06) `omegaflow --port <in> <out>` läuft über `port_mode`
  (`src/archivar/main_flow.rs:732`, `port.rs:625`); `phi/pipeline/stage/*` leer, regenerierbar.
- **Blockade:** die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored).
- **Braucht:** Korpus-Input wiederherstellen.

### CDN-Workflows der neuen Arme — Ausgang
- **Status:** wartend
- **Trigger:** Reihen-Ausgang der 4 CDN-Workflows → `ci_manage list`
- **Lage:** (gemessen 2026-10-06) `nasa-power-t2m-cdn 37491655041` = success;
  `epa-aqs-pm25-cdn 37491649258` = in_progress; `epa-aqs-voc-cdn 37491643441` und
  `zcta-gazetteer-cdn 37491637848` = queued; `jaxa-gportal-cdn 37492107559` = pending im Runner-Stau.
- **Blockade:** Runner-Stau; `jaxa_gpm_ku` ist nicht Workflow-manifestierbar
  (Arm braucht `--granule <pfad>`, Granule = per-act).
- **Braucht:** Reihen-Ausgang; Reader-Feld-Verdikt je Produkt (Mountain).

### Rand ohne Rubin — Fink-Cutout-/FP-Manifestation
- **Status:** wartend
- **Trigger:** Mountains Fink-Admission im Baum → `url`/`origin`/`compiler`/Tags setzen
- **Lage:** (gemessen 2026-10-06) die geharvesteten FP-Assets brauchen die Manifestations-Direktiven
  neben Mountains Fink-Admission; verwandte Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Blockade:** Fink-Admission (Mountain).
- **Braucht:** `phi/sources.φ`-Direktiven nach Admission.

### DE440-Remanifest vs. Pin — `pages-deploy` rot (Riss)
- **Status:** blockiert
- **Trigger:** Mountains DE440-Autoritäts-Verdikt → Pins + `sha256`-Direktive oder Remanifest
- **Lage:** (gemessen 2026-10-06, river-110) `pages-deploy 37489805784 @3eac2e119` = failure:
  `sha256 mismatch for ephemeris_de440_earth.bin: got 5554915d… want adc990bc…`; Release
  `ssd.jpl.nasa.gov-de` trägt neu `earth 5554915d…`/`sun 093b3ab5…`/`moon d9b40917…` (2026-10-06T13:25Z);
  `pages-deploy.yml:60-62` pinnt die alten shas; `phi/sources.φ` trägt keine `sha256`-Zeile für die DE440-Linie.
- **Blockade:** Autoritäts-Entscheid (Mountain) — neuer `de_compiler`-Output oder Remanifest.
- **Braucht:** Pins auf die gemessenen shas **und** `sha256`-Direktive je DE440-Zeile,
  dann `pages-deploy` neu auslösen; Checkmark `nearCount(<1e13 m) > 0` (River).

### `omegaflow/sources` — LICENSE-Generator + Drift-Tor
- **Status:** wartend
- **Trigger:** Mountains `terms`-Zeilen im Register
- **Lage:** (gemessen 2026-10-06, river-110) ein Compiler soll die `terms`-Zeilen aus
  `phi/sources.φ` lesen und ein nach Lizenzklassen gruppiertes `LICENSE` emittieren (`pending` namentlich);
  ein CI-Tor prüft byte-identisch gegen die Neu-Erzeugung.
- **Blockade:** Mountains `terms`-Zeilen fehlen noch.
- **Braucht:** Generator + Drift-Tor, nachdem die `terms`-Zeilen landen.

### `static/membrane.html` BODIES-Handkopie
- **Status:** eigen
- **Trigger:** Hüllen-Pipeline liefert das BODIES-Manifest → Build-Time-Einbindung
- **Lage:** (gemessen 2026-10-06, river-110) `static/membrane.html:49` trägt eine Handkopie der BODIES.
- **Blockade:** keine.
- **Braucht:** Build-Time-Manifest aus der Hüllen-Pipeline ersetzen.

## An future

Origin: mycelium-folge244.

- **Freie Frontier-Stimmen — Keys (Operator/per-act):** `free-model-bench 37487436436`
  ergab `pending_no_key` für SambaNova/OrcaRouter/Alibaba/OVHcloud/Mistral-Free
  (Artefakt `gh api …/artifacts/11425169303/zip`); die `voice-*`-Agenten dieser 5
  Provider warten auf `FREE_MODEL_KEYS`-Einträge. Vorbereitung (Test, IDs, Disposition)
  liegt; nur der Key-Akt ist Operator-Hand.
- **Orphan-Doc `docs/surveys/survey-2026-10-03-exzellenz-gate.md`** (1 offener Marker,
  kein Live-Handover-Träger): bitte Träger nennen oder gemessen `descoped`.

## An river

Origin: mycelium-folge244.

- **Orphan-Doc `docs/paper/hyperscanning-te-preregistration.md`** (2 offene Marker,
  kein Live-Handover-Träger): Carrier nennen oder gemessen `descoped`.
- **DE440-Remanifest vs. Pin** und **LICENSE-Generator** sind in dieser Übergabe als
  eigene Punkte geführt; der nächste Schritt (Autoritäts-Verdikt / `terms`-Zeilen) liegt bei Mountain.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein
  Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
