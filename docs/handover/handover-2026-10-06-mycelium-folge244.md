<!--
  title: Handover — Mycelium-Folge 244 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass in einem Atom; API-Modell-Test, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: 4d9a0813dbdb6214c852daae3ab46892dd5f6fd0613961ec8ca94ece1bbc070b
  status: live
-->
# Handover — Mycelium-Folge 244 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge243.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.3124 · cap 0.45 — Grund: langer Meta-Pass (Bench-Läufe, Config-Edit, Reconcile, Stimmen-Adressierung, DE440-Pin)

`session_burn`; `.tools_ensure archive_search|sgrep|sfetch|smail|ci_manage`: frisch.

## Operator-Wort-Register

- Wort | 2026-10-06 | „ist auch bekannt welche UI Chats funktionieren und wie sie addressiert werden sollen (bei architektur/ethik fragen mit den 5 stimmen und den 4 axiomen) gilt auch für die API modelle" — Architektur-/Ethikfragen werden mit den fünf Stimmen + vier Axiomen addressiert, für UI-Chats wie API-Modelle | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „ich will nicht nur dass du registriert sondern auch dass du tatsächlich die confgs bearbeitest und bitte kümmer dich auch um cdn_reconciliation.json" | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „kannst du bitte die nutzlosen modelle und anbieter deaktivieren? also auch die die immer rate limited sind" — nicht aufrufbare Arme aus `free_models.tsv` austragen (`struck`) | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „nein wir haben glm max über ui chat" — OrcaRouter nicht verfolgen (GLM-Route läuft über den UI-Chat) | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Holdings-Migration: „1 ja (move) · 2 ja (delete) · 3 ja (create) · 4 messen, dann · 5 ja (delete) · 6 ja (dedup) · 7 ja (dedup)" | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Daten-Holdings CDN-Bedarf/Ort: „ich gebe es mycelieum" — Kriterium nicht „regenerierbar", sondern was auf den CDN muss und am richtigen Ort liegt | Quelle: future-185 addressed.
- Wort | 2026-10-06 | „Ein Dispatch = ein begrenzter Schritt" — ein Agent plant im Output-Budget; ein über-großer Auftrag wird als Sequenz begrenzter Schritte gebaut | Quelle: Operator-Session 2026-10-06, als Regel in `AGENTS.md`.
- Wort | 2026-10-06 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | Quelle: Operator (Session, Mycelium 243).
- Wort | 2026-10-06 | Own-CDN-Junk-Bereinigung: „ich folge deiner Empfehlung" — die Messung korrigierte: nur 1 stale Blatt gelöscht | Quelle: Operator (Session, Mycelium 243).

## Offen — eigen

### Freie Frontier-Stimmen — Registrierung (deaktiviert per Operator-Wort 2026-10-06)
- **Status:** eigen
- **Trigger:** `auth login`/Konten-Freischaltung eines gestrichenen Anbieters → `free_model_bench --provider <p>`
- **Lage:** (gemessen 2026-10-06) `free_models.tsv` bereinigt — **`struck` gesetzt** für:
  `cloudflare-workers-ai` (11, Tagesquote 10 000 Neuronen immer verbraucht), `groq` (4, kein
  Credential), `sambanova` (3, `http_402 PAYMENT_METHOD_REQUIRED`), `mistral` large (403 tier) +
  medium (429), `alibaba` (2, kein Credential), `ovhcloud` (2, kein Credential), `zai` (2,
  `1113 Insufficient balance`), `orcarouter` (2, descoped), plus `gemini-2.5-flash` (404) und
  `nvidia/mistral-nemotron` (410 EOL). **Aktiv bleiben** `google`, `kilo`, `nvidia` (HTTP) und
  `opencode` (client). Test: `state/mycelium/free-model-bench-2026-10-06.tsv`.
  **Configs bearbeitet (Operator-Wort):** `~/.config/opencode/opencode.jsonc` — `disabled_providers`
  um `cloudflare-workers-ai`, `groq`, `sambanova`, `mistral`, `alibaba`, `ovhcloud`, `orcarouter`
  erweitert; tote Provider-Blöcke (cloudflare, sambanova) entfernt; `cohere/north-mini-code:free`
  aus der kilo-Whitelist. `opencode.json` — Agent `voice-qwen` (cloudflare) entfernt + aus der
  `line`-Dispatch-Allowlist. Beide JSON validiert (`jaq empty`).
  **zai-Whitelist:** `glm-4.7-flash` + `glm-4.6v-flash` beide lebendig (antworten; T5 leer nur
  wegen Reasoning-Budget) — kein toter Arm, Whitelist bleibt. **Arch-Adressierung:** fünf Stimmen
  + vier Axiome jetzt im Basis-`voice`-Rollenprompt (`opencode.json`), im `--mode arch`-PROLOG von
  `state/mycelium/voice-swarm.sh` und in `state/stimmen/prompt-arch-ethik.txt` (UI-Chats);
  arbeitende UIs in `state/stimmen/README.md`; Konzept `docs/concepts/free-voices.md`.
  **UI-Tab-Zensus** (gemessen 2026-10-06 via Chrome :9222, read-only, kein Send): z.ai,
  claude (2 Tabs), chatgpt, qwen, duck.ai, arena, kimi, grok, mistral, tryingopen offen —
  Composer je erreichbar (`state/stimmen/README.md`). Alle UI-Chats wie API-Modelle mit
  `prompt-arch-ethik.txt` addressierbar.
- **Blockade:** keine — die Deaktivierung ist der Vollzug des Worts.
- **Braucht:** nichts; ein Anbieter kehrt per `auth login`/Zahlungsmethode und Dispositions-Flip zurück.

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

### DE440 `pages-deploy` — Pin gesetzt, Lauf offen
- **Status:** wartend
- **Trigger:** Ausgang `pages-deploy 37508187212` → `ci_manage view 37508187212`
- **Lage:** (gemessen 2026-10-06, Mycelium-244) die drei Pins in `pages-deploy.yml:60-62` auf die
  gemessenen Release-Bytes gezogen (`archive_search --sniff`: earth `5554915dc7c2…`, moon
  `d9b4091731f1…`, sun `093b3ab5ee36…`); `pages-deploy 37508187212` dispatcht. `phi/sources.φ`
  trägt weiter keine `sha256`-Zeile für die DE440-Linie.
- **Blockade:** keine (Pins gesetzt); die `sha256`-Register-Direktive ist Mountains Verdikt-Zeile.
- **Braucht:** Lauf-Ausgang lesen; `sha256`-Direktive je DE440-Zeile (Mountain); Checkmark
  `nearCount(<1e13 m) > 0` (River).

### vnp46a3-cdn — Granule mit Nachtdaten wählen
- **Status:** eigen
- **Trigger:** Granule mit DNB-Nachtdaten gewählt → `vnp46a3-cdn` dispatchen
- **Lage:** (gemessen 2026-10-06, mountain-247) `vnp46a3-cdn.yml:24-32` nimmt den CMR-neuesten
  Granule; der war `…A2026213.h17v01` (arktisch, Polartag) → `no measured VNP46A3 cell` (0 honored).
  Der Compiler liest/selectet die SDS korrekt (Mountain `--inspect`); nur das Granule trägt keine DNB-Nachtdaten.
- **Blockade:** keine.
- **Braucht:** ein Granule mit Nachtdaten (Kachel/Datum) in `vnp46a3-cdn.yml` wählen, dann dispatchen.

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

- **Freie Frontier-Stimmen:** die nicht aufrufbaren Arme sind per Operator-Wort 2026-10-06 in
  `free_models.tsv` `struck` (s. eigener Punkt oben) — kein Operator-Akt mehr offen; reaktivierbar
  per `auth login`/Zahlungsmethode.
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
