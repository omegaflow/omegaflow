<!--
  title: Handover — Mycelium-Folge 243 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass in einem Atom; CI-Triage, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: c437062e6c079647c3af5b75ed176e0b36e5abf2a2392809f66f8b69bd7d1e4e
  status: live
-->
# Handover — Mycelium-Folge 243 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge242.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0608

`session_burn` (Line „Mycelium-Linie in einem Pass abarbeiten"): diese Session ~**$0.0608**.
`.tools_ensure`: frisch.

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Holdings-Migration: „1 ja (move) · 2 ja (delete) · 3 ja (create) · 4 messen, dann · 5 ja (delete) · 6 ja (dedup) · 7 ja (dedup)" | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Daten-Holdings CDN-Bedarf/Ort: „ich gebe es mycelieum" — Kriterium nicht „regenerierbar", sondern **was muss auf den CDN und liegt es am richtigen Ort** | Quelle: future-185 addressed.
- Wort | 2026-10-06 | „Ein Dispatch = ein begrenzter Schritt" — ein Agent plant im Output-Budget; ein über-großer Auftrag wird als Sequenz begrenzter Schritte gebaut | Quelle: Operator-Session 2026-10-06, als Regel in `AGENTS.md`.
- Wort | 2026-10-06 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | Quelle: Operator (Session, Mycelium 243).

## Offen — eigen

### Daten-Holdings — `opencode-tmp`-Dump CDN-Bedarf/Uniqueness
- **Status:** eigen
- **Trigger:** je Posten CDN-Bedarf/Uniqueness gemessen → Move/Delete je Bestand
- **Lage:** (gemessen 2026-10-06) vier Bestände ausgeführt: `wind_orbit`/`dr3_stars` verortet,
  Snapshot-Dedup 270 MiB, archive-state-Hardlink 1,21 GiB; `nvss.json` liegt korrekt unter
  `data/ssd.jpl.nasa.gov-nvss/nvss.json` (Register-Tag `ssd.jpl.nasa.gov-nvss`, `sources.φ:17784`);
  `radio.bin` live + registriert. Offen: `~/archive/knowledge/data/opencode-tmp-2026-09-01/` ist
  ein 13 GB/11 603-Dateien-Dump (`provenienz` 2,3 G, `demeter_full` 1,8 G, `meteo-pr` 1,7 G,
  `kollab_mseed` 366 M).
- **Blockade:** Umfang (11 603 Dateien) — je Posten Einzelmessung.
- **Braucht:** je Top-Dir `du`-Größe + Uniqueness vs Register/`archive_search` messen; Detail
  `state/future/holdings-migration-2026-10-06.md:26`. **Träger** für
  `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (dessen offener Marker keine
  andere Live-Übergabe hält).

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** blockiert
- **Trigger:** Sources-Zeilen je pending Domäne im Baum → `.te`-Descriptor + Workflow-YAML → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md`
  (12 Domänen); 4 Kern-x-Serien erreichbar/registriert; 2 Arme gebaut (WQP + EEA-noise).
  Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `field_te_query.rs:580-684`.
  y-Serien unregistriert, Erreichbarkeit gemessen (`archive_search`, alle 200).
- **Blockade:** Mountain-Verdikt (Admission) je y-Quelle.
- **Braucht:** je pending Domäne die Sources-Zeile (Mountain, `## An mountain`); dann `.te` je Klasse
  + `.github/workflows/exposom-matrix-te.yml`.

### Registry↔CDN-Reconciliation Step 5 — Löschung der Junk-Assets
- **Status:** operator-gebunden
- **Trigger:** Operator-/Council-Wort zur Löschung
- **Lage:** (gemessen 2026-10-06) Bindungen scoped umgesetzt; Reg==Release offen: keiner;
  Mismatches klassifiziert; Junk-Liste ~~33~~ (vorbereitet, `docs/surveys/survey-2026-09-03-orphan-verdicts.md:103-151`).
- **Blockade:** Own-CDN-Löschung ist destruktiv — Operator-/Council-Wort.
- **Braucht:** Operator-Wort (über Future-Queue vorgelegt, `## An future`).

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

### JAXA G-Portal — Abholung
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list`
- **Lage:** (gemessen 2026-10-06) Läufe pending im Runner-Stau; Mountain arbeitet am `jaxa_gportal_compiler`
  (working tree). Der `jaxa_gpm_ku`-Arm braucht `--granule <pfad>` (Granule-Download) — nicht
  Workflow-seitig manifestierbar ohne Granule.
- **Blockade:** Runner-Kapazität (Stau) + Granule-Beschaffung (per-act).
- **Braucht:** Reihen-Ausgang; Reader-Feld-Verdikt je Produkt (Mountain).

### Rand ohne Rubin — Fink-Cutout-/FP-Manifestation
- **Status:** wartend
- **Trigger:** Mountains Fink-Admission im Baum → `url`/`origin`/`compiler`/Tags setzen
- **Lage:** (gemessen 2026-10-06) die geharvesteten FP-Assets brauchen die Manifestations-Direktiven
  neben Mountains Fink-Admission; verwandte Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Blockade:** Fink-Admission (Mountain).
- **Braucht:** `phi/sources.φ`-Direktiven nach Admission.

### Freie Frontier-Stimmen — Registrierung
- **Status:** eigen
- **Trigger:** API-Basis-URLs verifiziert → `free_models.tsv` + `voice-*`-Agenten
- **Lage:** (gemessen 2026-10-06) `free_models.tsv` korrigiert/erweitert (10 neue Zeilen:
  orcarouter/mistral/alibaba/ovhcloud; SambaNova `V3.1`→`V3.2`); Model-IDs belegt aus
  `~/.cache/opencode/models.json`. Endpunkt-Probe keyless: orcarouter/mistral/alibaba/sambanova
  = HTTP 401, ovhcloud = 429 → alle lebendig. `free_model_bench` erweitert (`--provider`-Filter +
  Export aller Bundle-Keys). Offen: die `voice-*`-Agenten für die neuen Provider.
- **Blockade:** Keys liegen im CI-Secret `FREE_MODEL_KEYS`, lokal nur `MISTRAL_API_KEY`.
- **Braucht:** `voice-*`-Agent je Provider in `opencode.jsonc`, sobald der Key lokal/CI steht.

## An mountain

Origin: mycelium-folge243.

- **Neue CDN-Workflows gebaut (Mycelium):** `zcta-gazetteer-cdn.yml`, `epa-aqs-pm25-cdn.yml`,
  `epa-aqs-voc-cdn.yml`, `nasa-power-t2m-cdn.yml` (je `ubuntu-latest`, `--ci-mode`, Release-Ensure +
  `register_release_set.sh`-Verify). `jaxa_gpm_ku` ist **nicht** Workflow-manifestierbar: der Arm
  braucht `--granule <pfad>` (Granule-Download = per-act).
- **Pollen `sha256`:** `sources.φ:17511`-Block braucht `sha256 42a7f2f8199f84cdb13a55c56265edf4766b97a9e36ca00052b7b06d00bdc2a7` (Lauf `openmeteo-pollen-cdn 37459099065` success; `--sniff` 200, 1468 B). Der `sha256`-Grenzfall ist ein Riss zwischen Verdict-/Manifestationszeile — nicht still geschrieben.
- **`ghsl_compiler`-Arm:** `ghsl-cdn 37459107131` failure, gemessen `0 raster bytes against the 432002x213822x2 grid — the arm reads no common grid`.
- **VNP46A3-CDN:** `vnp46a3-cdn 37459111673` failure — `vnp46a3_compiler` meldet `no measured VNP46A3 cell left the harvest — the bin stays unwritten (0 honored)`. **Braucht:** `format black_marble_vnp46a3_nightlight`-Arm.
- **Sweep-Riss (unverändert):** Commit `0d5a7b9dd` enthielt durch `git add phi/sources.φ` mitgerissene fremde Hunks — (a) `fink_cutout` `at earth`→`at sun`; (b) die Löschung des `usda_fara_low_access.bin`-Blocks (in `cde891a94` wiederhergestellt). Bitte prüfen, ob `at sun` gewollt ist.
- **Exposom y-Serien (Admission/Verdikt):** TOLIFE `10.5281/zenodo.16642439`, AAMOS-00 `10.7488/ds/3775`, Wearable+PRO `10.5281/zenodo.8018238`, ADARP `10.5281/zenodo.6640290`, CrossCheck (Kaggle), Labbaf `10.7280/D1WH6T` — alle erreichbar. Bitte je Quelle Admission + Compiler-Arm (CSV/zip-Reader) → dann Mycelium-Manifestation + `.te` + Workflow.

## An future

Origin: mycelium-folge243.

- **Operator-Frage (Own-CDN-Löschung, operator-gebunden):** die vorbereitete Junk-Liste der CDN-Reconciliation (Step 5, `docs/surveys/survey-2026-09-03-orphan-verdicts.md:103-151`) wartet auf ein Wort. Lage: 13 Netlocs, Bindungen scoped umgesetzt, Reg==Release offen: keiner. Frage: dürfen die klassifizierten Junk-Assets aus dem eigenen CDN-Release gelöscht werden? Bei Ja: je Asset `gh release delete-asset` (nie die letzte Kopie); bei Nein: Liste bleibt als `verwahrt` liegen. (Löschung = destruktiv → Operator-Wort.)
- **Self-hosted Runner:** `t420` ist **online** und bedient `[self-hosted, Linux]` (gemessen `gh api …/actions/runners`). Runner-Routing der 3 dispatch-only One-Shots (`emso`/`twomrs`/`vires-hapi`) auf `ubuntu-latest` **in dieser Session erledigt**.
- **Doc-Korrektur `state/future/holdings-migration-2026-10-06.md`:** Zeile `nvss.json` ist widerlegt — das Asset liegt korrekt unter `data/ssd.jpl.nasa.gov-nvss/nvss.json` (Register-Tag `ssd.jpl.nasa.gov-nvss`, nicht `ssd.jpl.nasa.gov`). Die Zahlen-Snapshot-Risse (270 MiB, 1,21 GiB) sind ausgeführt; die Riss-Zeile zum `opencode-tmp`-Dump (13 GB/11 603 Dateien) bleibt gültig.
- **Orphan-Doc `docs/surveys/survey-2026-10-03-exzellenz-gate.md`** (1 offener Marker, kein Live-Handover-Träger): bitte als Träger im eigenen Handover nennen oder gemessen `descoped`.

## An river

Origin: mycelium-folge243.

- **iEEG descoped:** River 105 hat iEEG.org + `ieeg-cdn.yml` entfernt (Lizenz ungeklärt, `c6ecce4e8`); das Vorgänger-Handover trug noch einen iEEG-Trigger — aufgelöst.
- **Orphan-Doc `docs/paper/hyperscanning-te-preregistration.md`** (2 offene Marker, kein Live-Handover-Träger). Der Carrier ist der nächste Schritt: bitte im eigenen Handover nennen (oder gemessen `descoped`).
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** (aus river-108): Generator + Drift-Tor, nachdem Mountains `terms`-Zeilen landen; checkmark = byte-identisch gegen Neu-Erzeugung.
- **DE440-`.bin` remanifestieren** nach Mountains `de_compiler`-GM-Landung; Checkmark `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
