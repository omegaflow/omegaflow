<!--
  title: Handover — Mycelium-Folge 242 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass in einem Atom; CI-Triage, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: c9c493db9cd34feb322bcd18347bb3018dc5221dc4f476ba3179c61a79d6a8da
  status: live
-->
# Handover — Mycelium-Folge 242 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge241.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0362

`session_burn`: diese Session ~**$0.0362** (line „Mycelium-Linie in einem Pass starten").
Rolling-Fenster (6 Sessions) bei Schluss **$0.1595**.
`bin/.tools_ensure archive_search|sgrep|sfetch|smail|ci_manage`: frisch (keine Meldung).

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Holdings-Migration: „1 ja (move) · 2 ja (delete) · 3 ja (create) · 4 messen, dann · 5 ja (delete) · 6 ja (dedup) · 7 ja (dedup)" | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Daten-Holdings CDN-Bedarf/Ort: „ich gebe es mycelieum" — Kriterium nicht „regenerierbar", sondern **was muss auf den CDN und liegt es am richtigen Ort** | Quelle: future-185 addressed.

## Umsetzung (Operator-Auftrag 2026-10-06, „bitte umsetzen")

- **Runner-Routing erledigt:** `emso-cdn.yml`, `twomrs-cdn.yml`, `vires-hapi-cdn.yml` von
  `runs-on: [self-hosted, Linux]` auf `ubuntu-latest` (dispatch-only One-Shots; `t420` bleibt für
  die übrigen `[self-hosted, Linux]`-Jobs).
- **`pds3-ring-occ-cdn.yml` dispatcht:** Lauf `37483538590` (2026-10-06 via `gh workflow run`).
  Ausgang `unread`.
- **Register-`url`-Lücken (tap/reference) gemessen: 0.** `phi/sources.φ` trägt 2667 Blöcke,
  2667 `url`-Zeilen; jeder `tap`- (34) und `reference`-Block (642) hat eine `url`. Kein offener Akt.
- **Freie Frontier-Stimmen:** Basis-URLs gemessen (`--verdict`, 404/405 = API-Root erreichbar):
  `https://api.orcarouter.ai/v1` (404), `https://api.sambanova.ai/v1` (405),
  `https://api.mistral.ai/v1` (404), `https://dashscope-intl.aliyuncs.com/compatible-mode/v1` (404),
  `https://oai.endpoints.kepler.ai.cloud.ovh.net/v1` (404, nicht doc-verifiziert). `free_models.tsv`
  trägt 3 verifizierte SambaNova-Modelle (`DeepSeek-V3.1`, `gpt-oss-120b`, `Meta-Llama-3.3-70B-Instruct`),
  Status `blocked` (Free-Plan verlangt Karte) — Key/Account = Operator-Akt. Model-IDs für OrcaRouter/
  Mistral/Alibaba bleiben `unverified` → nicht als Zeile erfunden.
- **Daten-Holdings CDN-Ort (gemessen 2026-10-06 via `fd`):** `data/spdf.gsfc.nasa.gov/` ist leer
  (`wind_orbit.bin` + `omegaflow_series_wind_orbit.bin` nicht mehr am Ort; im CDN registriert
  `sources.φ:3471`); `data/gea.esac.esa.int/` leer (dr3-Zwillinge bereits weg); `data/ssd.jpl.nasa.gov/`
  trägt `ephemeris_earth.bin.cdn`/`gaia_dr3_vlies.vlde`; `nvss.json` liegt unter dem Register-Tag
  `data/ssd.jpl.nasa.gov-nvss/nvss.json` (`sources.φ:17522`). **Riss:** die Survey-Tabelle nennt
  `data/ssd.jpl.nasa.gov/nvss.json` (ohne `-nvss`), das Register `-nvss`. Kein Move ohne
  Operator-Wort; Layout in Fremd-Bewegung (uncommittet).
- **Exposom-Quellenmatrix: blockiert.** Die y-Serien der 12 Domänen sind unregistriert (Survey
  `survey-2026-10-04-exposom-matrix.md`: „feedbare y-Serie: keine" für respiratorisch/renal/onkologisch).
  Ein `.te`/Workflow ohne Driver/Target-Feld wäre eine leere Hülle — nicht gebaut.
- **Exposom-CDN-Workflows: nichts mehr eigen.** FARA-Descope erledigt; `sha256` pollen +
  `ghsl_compiler`-Arm sind Mountain-Teile (adressiert). Nur Nachhalten.

## Offen — eigen

### Exposom-CDN-Workflows (Pollen, FARA, GHSL)
- **Status:** eigen
- **Trigger:** grüner Lauf → `sha256` aus Release; FARA-Workflow gelöscht; GHSL: Compiler-Arm
- **Lage:** (gemessen 2026-10-06 via `ci_manage view`/`log`)
  - `openmeteo-pollen-cdn 37459099065` = **success**; CDN-Artefakt `--sniff` = HTTP 200, 1468 B,
    sha256 `42a7f2f8199f84cdb13a55c56265edf4766b97a9e36ca00052b7b06d00bdc2a7`. Block `phi/sources.φ:17511`.
  - `ghsl-cdn 37459107131` = **failure**; gemessener Grund: `ghsl_compiler: GHS_BUILT_S_E2020_GLOBE_R2023A_4326_3ss_V1_0.tif: 0 raster bytes against the 432002x213822x2 grid — the arm reads no common grid`. Block `phi/sources.φ:17519`.
  - `usda-fara-cdn 37479241248 @62c9fd513` = **failure**: `no bin target named usda_fara_compiler` — Mountain hat `usda_fara` (no-physical-force) gedroppt (`62c9fd513`), der Compiler-Bin fehlt. **Workflow `.github/workflows/usda-fara-cdn.yml` in dieser Session entfernt** (descoped, kein Parkplatz).
- **Blockade:** `sha256`-Zeile u. `ghsl_compiler`-Arm sind Mountain-Verdiktzeilen (`mountain 230` schrieb die bestehenden `sha256`-Zeilen).
- **Braucht:** `sha256 42a7f2…` in `sources.φ:17511`-Block + `ghsl_compiler`-Raster-Arm (Mountain) — als `## An mountain` adressiert.

### VNP46A3-CDN — Lauf leer, per-Zelle-Arm
- **Status:** blockiert
- **Trigger:** per-Zelle-Arm/Verdikt (Mountain) → Re-Dispatch
- **Lage:** (gemessen 2026-10-06 via `ci_manage log 37459111673`) `vnp46a3-cdn` = failure; CMR-Resolver liefert die Granule, `vnp46a3_compiler` meldet `no measured VNP46A3 cell left the harvest — the bin stays unwritten (0 honored)`.
- **Blockade:** `format black_marble_vnp46a3_nightlight`-Arm (Mountain).
- **Braucht:** `vnp46a3_compiler <granule-url> --inspect` (SDS-Liste) → Arm Mountain.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Sources-Zeilen je pending Domäne + `.te`-Descriptor + Workflow-YAML → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md` (12 Domänen); 4 Kern-x-Serien erreichbar/registriert; 2 Arme gebaut (WQP + EEA-noise). Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `field_te_query.rs:580-684`. y-Serien unregistriert.
- **Blockade:** Mountain-Verdikt + Mycelium-Manifestations-Direktive je Domäne.
- **Braucht:** je pending Domäne die Sources-Zeile; dann `.te` je Klasse + `.github/workflows/exposom-matrix-te.yml`.

### Träger `survey-2026-09-03-orphan-verdicts` — Step 5 CDN-kanonisch
- **Status:** eigen
- **Trigger:** je `*-cdn.yml` die Release-Menge aus `phi/sources.φ` lesen
- **Lage:** (gemessen 2026-10-06) `:103-151` — 13 Netlocs; Bindungen scoped umgesetzt; Reg==Release offen: keiner; Mismatches klassifiziert; Junk-Liste 33 Assets vorbereitet; `pds3_ring_occ.bin`-Workflow gebaut.
- **Blockade:** Own-CDN-Löschung der 33 Junk-Assets braucht das **Operator-/Council-Wort** (destruktiv, eigener CDN).
- **Braucht:** (a) Operator-/Council-Wort für die 33-Asset-Löschung; (b) `pds3-ring-occ-cdn.yml` dispatchen; (c) weitere Register-`url`-Lücken (`tap`/`reference`).

### Daten-Holdings — Ziel-Layout/CDN-Ort (Operator-Wort „ich gebe es mycelieum")
- **Status:** eigen
- **Trigger:** weitere Bestände je Move → `du`-Nachmessung
- **Lage:** (gemessen 2026-10-06) Move/Dedup ausgeführt (folge241). Offen: CDN-Ort je Bestand — `omegaflow_series_wind_orbit.bin` → `cache/`; zwei byte-identische `dr3_stars`-Kopien (`data/gea.esac.esa.int/`); `nvss.json` fehlt an `data/ssd.jpl.nasa.gov/` (registriert `sources.φ:17522`, z-Crossmap 102 581 Quellen; Scratch = voller NVSS 1 773 484). Träger-Inventur: `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:167-183`; Detail `state/future/holdings-migration-2026-10-06.md`.
- **Blockade:** keiner für die Ort-Korrektur; vorheriger Byte-/sha-Abgleich nötig.
- **Braucht:** Ort-Korrektur/Dedup je Bestand (Byte-/sha-Abgleich).

### `ledger.φ:2`/`:6` — Port-Runner
- **Status:** wartend
- **Trigger:** Korpus-Input `phi/pipeline/queue/<korpus>.φ` am Datenträger → `omegaflow --port`
- **Lage:** (gemessen 2026-10-06) `omegaflow --port <in> <out>` läuft über `port_mode` (`src/archivar/main_flow.rs:732`, `port.rs:625`); `phi/pipeline/stage/*` leer, regenerierbar.
- **Blockade:** die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored).
- **Braucht:** Korpus-Input wiederherstellen.

### `phi/blocked_sources.φ` — Mycelium-Klasse
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06) ExoMars TGO ACS / Viking gravity / Hayabusa LIDAR / Phobos-2 KRFM — Workflows queued; EUMETSAT MTG-LI queued; Chandrayaan-1 Mini-RF blockiert (`pds3_img` ohne Feld-Arm); Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal = Operator-Hand; `:78` SuperDARN LOCK.
- **Blockade:** Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent (Operator/per-act).

### iEEG-Ernte — Dienst antwortet 500
- **Status:** wartend
- **Trigger:** `www.ieeg.org/services` erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-06) Zugang steht (`IEEG_USER`/`IEEG_PASS`, `ieeg_compiler.rs:211-212`); `--verdict` = HTTP 500. **Riss:** `open_points_check` meldet `.github/workflows/ieeg-cdn.yml` als ABSENT — der Workflow-File fehlt; der Trigger nennt ihn. Vor Re-Dispatch: Workflow prüfen/neu anlegen.
- **Blockade:** Dienst 500 (nicht der Zugang); fehlender Workflow-File.
- **Braucht:** Re-Dispatch bei Erholung; `ieeg-cdn.yml` verifizieren.

### JAXA G-Portal — Abholung
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list`
- **Lage:** (gemessen 2026-10-06 via `ci_manage list`) zwei Läufe **pending** (`37481777412`, `37481808152`) im Runner-Stau; Mountain arbeitet am `jaxa_gportal_compiler` (working tree).
- **Blockade:** Runner-Kapazität (Stau).
- **Braucht:** Reihen-Ausgang; Reader-Feld-Verdikt je Produkt (Mountain).

### Rand ohne Rubin — Fink-Cutout-/FP-Manifestation
- **Status:** wartend
- **Trigger:** Mountains Fink-Admission im Baum → `url`/`origin`/`compiler`/Tags setzen
- **Lage:** (gemessen 2026-10-06) die geharvesteten FP-Assets brauchen die Manifestations-Direktiven neben Mountains Fink-Admission; verwandte Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Blockade:** Fink-Admission (Mountain).
- **Braucht:** `phi/sources.φ`-Direktiven nach Admission.

### Runner-Routing (aus future-185 gefaltet)
- **Status:** eigen
- **Trigger:** Routing-Entscheidung → Workflow-Edit
- **Lage:** (gemessen future-185) 3 dispatch-only One-Shots (`emso-cdn.yml:17`, `twomrs-cdn.yml:17`, `vires-hapi-cdn.yml:14`) könnten `ubuntu-latest`; inverse Route (`hyperscanning-te` -> self-hosted) erst nach Messung. Der Runner `t420` ist **online** und bedient `[self-hosted, Linux]`.
- **Blockade:** keine.
- **Braucht:** Workflow-Edit (Mycelium).

### Freie Frontier-Stimmen — Registrierung (aus future-185 gefaltet)
- **Status:** eigen
- **Trigger:** API-Basis-URLs verifiziert → `free_models.tsv` + `voice-*`-Agenten
- **Lage:** (gemessen future-185) Kandidatenliste `state/future/free-voices-scan-2026-10-06.md` (OrcaRouter, OpenRouter, SambaNova, Mistral Free, Alibaba Model Studio, OVHcloud); SambaNova-Riss am Console-Zustand nachmessen.
- **Blockade:** keine.
- **Braucht:** API-Basis-URLs je Anbieter verifizieren, dann registrieren.

## An future

Origin: mycelium-folge242.

- **Self-hosted Runner:** Wort 2026-10-06 „ok ich schaue ob ich einen gaming pc von 2011 … als runner laufen zu lassen". Nachtrag future-185: `t420` ist **online**. Mehrere Jobs binden `[self-hosted, Linux]`.
- **Daten-Holdings:** Operator-Wort „ich gebe es mycelieum" gefaltet; CDN-Ort-Korrektur läuft (eigener Punkt).
- **Doc-Korrektur `state/future/holdings-migration-2026-10-06.md`:** die Zahlen sind widerlegt — Snapshot-Dup 270 MiB; target-rlibs ~7 MiB; LFS 1,25 GiB; Radio-Pipeline live (`radio.rs` in HEAD). Detail folge241.
- **Freie Frontier-Stimmen:** gefaltet als eigener Punkt; keine Operator-Frage offen.
- **Orphan-Doc `docs/surveys/survey-2026-10-03-exzellenz-gate.md`** (1 offener Marker, kein Live-Handover-Träger): bitte als Träger im eigenen Handover nennen oder gemessen `descoped`.

## An mountain

Origin: mycelium-folge242.

- **Sweep-Riss (unverändert aus folge241):** Commit `0d5a7b9dd` enthielt durch `git add phi/sources.φ` mitgerissene fremde Hunks — (a) `fink_cutout` `at earth`→`at sun`; (b) die Löschung des `usda_fara_low_access.bin`-Blocks (in `cde891a94` wiederhergestellt). **Bitte prüfen, ob `at sun` gewollt ist.**
- **Pollen `sha256`:** `sources.φ:17511`-Block braucht `sha256 42a7f2f8199f84cdb13a55c56265edf4766b97a9e36ca00052b7b06d00bdc2a7` (Lauf `openmeteo-pollen-cdn 37459099065` success; `--sniff` 200, 1468 B). Der `sha256`-Grenzfall ist ein Riss zwischen Verdict-/Manifestationszeile — nicht still geschrieben.
- **`ghsl_compiler`-Arm:** `ghsl-cdn 37459107131` failure, gemessen `0 raster bytes against the 432002x213822x2 grid — the arm reads no common grid`.

## An river

Origin: mycelium-folge242.

- **Orphan-Doc `docs/paper/hyperscanning-te-preregistration.md`** (2 offene Marker, kein Live-Handover-Träger). Der Carrier ist der nächste Schritt, nicht der Eintrag: bitte im eigenen Handover nennen (oder gemessen `descoped`).
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** (aus river-108): Generator + Drift-Tor, nachdem Mountains `terms`-Zeilen landen; checkmark = byte-identisch gegen Neu-Erzeugung.
- **DE440-`.bin` remanifestieren** nach Mountains `de_compiler`-GM-Landung; Checkmark `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
