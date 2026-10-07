<!--
  title: Handover — Mycelium-Folge 261 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; Sternkatalog-Sort + vnp46a3 aufgelöst; jaxa_gpm_ku `--granule` verdrahtet; CDN-Audit (986 Blöcke) 9 Workflow-Lücken geschlossen; dropped-gate Schritt 1+2 + echtes `--roster`-Gate (Tombstones/Generationen, nicht-vakuoser Shadow); Lizenz-Census (i)+(iii) (9er-Vokabel, Join am Quellenblock); Recherche-Schicht (SPDX + dropped-gate) + UI-Frontier-Runde (Qwen: b); CI-Tafel (format mountain + unnamed archive_search)
  class: handover
  date: 2026-10-07
  sha256: c2694d04ec1d707111838a17624c13e002c129e071d49c3c64e782a036b0be32
  status: live
-->
# Handover — Mycelium-Folge 261 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge260.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An river` / `## An future`.

## Burn: open 0.0 · close 0.1281 · cap 0.5 — Grund: Meta-Pass + Agenten-Kohorten (9 grind-flash, 8 general, 4 council, 2 voice-deepseek), kein pro/max; gemessen `session_burn` (line, deepseek-flash) bei Commit

## Operator-Wort-Register

- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | Quelle: Operator (Session, Mycelium 261) — Session-Start, Delegations-Consent.
- Wort | 2026-10-07 | „endlich mal die arbeit fertig machen" → Delegationsauftrag: eine Agenten-Kohorte (flash-first) parallel auf die arbeitsfähigen Mycelium-Atome. | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „ja bitte" → Recherche-Schicht für `terms`-SPDX + dropped-gate-Thresholds starten und die UI-Frontier-Fragen-Sheets bereitlegen. | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „fängst du jetzt bitte an zu arbeiten?" → Direktive: weiterarbeiten ohne Rückfrage. Gebaut: echtes dropped-gate (`--roster` + Tombstones/Generationen, nicht-vakuoser Shadow), Lizenz-Census (iii) + Join am Quellenblock. | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „das ist einfach nur quatsch … schau dir bitte an welche tabs offen sind" → Korrektur: die Frontier-Seats sind offen/eingeloggt; die Antworten werden aus den offenen Tabs gelesen (nicht aus neuen, unangemeldeten Tabs). | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „ok würdest du dann bitte migrieren?" → **Migrations-Auftrag `terms`→SPDX** (134× `CC0`→`CC0-1.0`, 1× `ODC-BY-1.0`→`ODC-By-1.0`; `PD`/`free-open`/`own-work` bleiben). Im Working Tree bereits vollzogen (uncommittet); Gate nachgezogen `9c010f725`. | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „was ist own-work?" · „bitte messen und ihr berücksichtigt schon alle phi files?" → `own-work`-Riss vermessen (eigene Kompilation, Fakten) + `terms`-Deckung nur `phi/sources.φ`. | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „hast du dich darum gekümmert?" (Force-/Einheiten-13-Baseline · obis.osha.gov · Exposom-Matrix · Sternkatalog) → tools-build dispatcht (`37631266911`), Sternkatalog verifiziert erledigt, Exposom-`eionet`-Register an Mountains ttl/field, obis.osha.gov an den `1`-Riss. | Quelle: Operator (Session, Mycelium 261).
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge260.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### NUR-Asset re-harvest (dB/dt–GIC-Kette) — `force`-Run läuft
- **Status:** eigen (Nachlauf)
- **Trigger:** `image-cdn`-Lauf-Ergebnis
- **Lage:** (gemessen 2026-10-07, Mycelium 261) `image-cdn`-Lauf `37621964105` (`start=20031029`, `days=3`, `force=true`) noch **queued** (self-hosted Runner); Idempotenz-Guard durch `force`-Input übersteuert.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37621964105` → neuen sha von `fmi_image_mag_nur.bin` ins Register (`phi/sources.φ:18034`); dann River-Probe + Zahl Paper §4/§6.

### jaxa_gpm_ku — Emit-Pfad gemessen, Workflow verdrahtet; Dispatch operator-gebunden
- **Status:** eigen → operator-gebunden (Dispatch)
- **Trigger:** Operator-Wort für den G-Portal-Download/Fetch
- **Lage:** (gemessen 2026-10-07, Mycelium 261) Der Emit-Pfad **existiert**: `jaxa_gportal_compiler --granule <hdf5> --bin <out> --ci-mode` (`tools/harvest/src/bin/jaxa_gportal_compiler.rs:1170-1245`) schreibt und (ci-mode) uploadet `jaxa_gpm_ku.bin` (format `jaxa_gpm_ku`; Register-Block `phi/sources.φ:2049-2055` vollständig, compiler-Zeile gesetzt). `.github/workflows/jaxa-gportal-cdn.yml` um Input `gpm_ku` + Compile-Schritt erweitert: nach `download+fetch` wird die gefetchte `jaxa_gportal_*.bin`-Granule via `--granule` kompiliert und `jaxa_gpm_ku.bin` in die `gportal.jaxa.jp`-Release geladen.
- **Blockade:** der `--download` schreibt in den JAXA-Account (per-Akt-Consent) → der Dispatch ist Operator-Hand.
- **Braucht:** Operator-Wort → Dispatch `jaxa-gportal-cdn` mit `dataset=12001000`, `from`/`to` (KuPR-Fenster), `download=true`, `fetch=true`, `gpm_ku=true`; danach sha von `jaxa_gpm_ku.bin` in `phi/sources.φ` und Handover.

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend (Mountain-Feld/ttl)
- **Trigger:** Mountains Einschreiben der 276 `field`/`ttl`-Zeilen in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-07, Mycelium 260) adressierter mountain-266-Block gefaltet; `.github/workflows/eionet-cdr-cdn.yml` gebaut (dispatch-only); `eionet_cdr_compiler --emit-field-names` druckt 276 `field`-Zeilen; Register-Block in `phi/sources.φ` noch **0** Treffer (`sgrep eionet` leer). Der `cdn_reconcile --fail`-Register-Job, der seit dem Workflow-Commit auf `eionet-cdr-cdn.yml:34` rot lief (`ci_manage log 37621486821`), ist geheilt: `cdr.eionet.europa.eu` als gemessene Ausnahme in `docs/specs/cdn-tag-baseline.txt` (OSHA-Präzedenz), fällt mit Mountains Block. `cdn_reconcile --fail` lokal grün (291 Hosts). Der getragene Kraft-Riss (Medium `diffusion kg` + Punkt-Kernel Slot 2; Minderheit `gravity kg`/`pending`) blockiert den Schreibakt nicht. **Exposom-Matrix §A (Mountain 248 „Register-Zeilen je Klasse"):** Reader `src/archivar/{osm_pbf,eionet_cdr}.rs` gebaut; `osm_nodes` ist registriert (`phi/sources.φ:1684-1687`, origin geofabrik; `em 1` laut Rat); der `eionet_cdr`-Register-Block fehlt weiterhin — ein Block ohne `ttl` bricht `register_sort` (`ci-gate`), und die 276 `field`/`ttl`-Zeilen sind Mountains Pen. `format osm_nodes` in `main_flow.rs:4155`. **tools-build:** dispatcht (`gh workflow run tools-build.yml` → `37631266911`), damit `tools-latest` frisch ist und Mountain die 13 Klasse-(a)-Baseline-Zeilen löschen kann. **Sternkatalog:** erledigt — `phi/sources.φ:18156-18161` (`dr3_stars.bin`, sha `49ea7595…`) + `pages-deploy.yml:61` staged; `gaia-cdn 37599077973` success.
- **Blockade:** Mountains `field`/`ttl`-Zeilen; geteilter Baum — `phi/sources.φ` nur schreiben, wenn Mountain freigibt.
- **Braucht:** Mountains Einschreiben; dann meine `url`/`origin`/`compiler`/`format`-Zeilen + Manifestation + Dispatch.

### OSHA-CEHD — Register-Zeile (Force-/Einheiten-`1`-Riss)
- **Status:** wartend (Mountain)
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07, Mycelium 260) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-Kontrakt (Compiler emittiert `1`).
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann Manifestation.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend (Mountain)
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260) `LICENSE`/`README` dort absent (HTTP 404 raw).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen) noch nicht vollständig (Mountain 266: ~160 `terms unbestimmt`-Zeilen offen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### dropped-gate — echtes `--roster`-Gate gebaut; Re-Pin/Archiv-Disziplin + CI-Verdrahtung offen
- **Status:** eigen → wartend (Pin-/Archiv-Entscheidung)
- **Trigger:** Operator-/Frontier-Wort (Sheet) zur Pin-Wahl + Archiv-Move-Disziplin
- **Lage:** (gemessen 2026-10-07, Mycelium 261) `dropped_gate.rs` ist jetzt **nicht-vakuos** (gepusht `e9a2bd8cb`): `--roster <live>` vergleicht den beobachteten Roster gegen Pin+Log (`false_green` = gepinnter Schlüssel still weg, `false_red` = beobachteter Schlüssel ohne `minted`); Tombstones/Generationen (minted nach dropped ohne `revive` = benannte Refusal); `--shadow` prüft vier Sub-Runs, `sharp=true`; Smoke `{a,b,c}` vs `{a,b,x}` → `false_red=1 false_green=1`. **Gemessen:** der public-only-Roster ist seit dem Pin (HEAD `351632051`) um **10** Schlüssel verschoben (`--baseline … --count` = 10) — die CI-Verdrahtung meldete heute 10 false_green, solange Archiv-Moves nicht als `carried`-Events geführt werden.
- **Blockade:** ungeklärte Pin-Wahl (Offen- vs. Verlust-Menge) + fehlende Archiv-Move-Disziplin (Roster/`archiv`-Asymmetrie).
- **Braucht:** UI-Frontier-/Operator-Wort (Sheet `state/stimmen/2026-10-07_dropped-gate-ui-fragen.md`) → Re-Pin mit gemessenem Grund + Event-Log-Seeding + `ci-gate`-Verdrahtung. Verdikt: `state/stimmen/2026-10-07_dropped-gate-stimmen-runde.md` + `-runde-2.md`.

### Lizenz-Census — Gate verdrahtet (i+iii); SPDX-Migration vollzogen
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 261) `license_census.rs` erzwingt die geschlossene `terms`-Vokabel auf `phi/sources.φ` (Join am Quellenblock) und druckt `blocks/terms/distinct/no-terms/pending`; `state/`-Census ist optionaler Komplement (kein `exit 2` mehr). `--fail` in den `register`-Job von `ci-gate.yml` verdrahtet. Die `terms`→SPDX-Migration ist vollzogen: Mountain-Pen-Commit `0fe79f7dd` (verifiziert 134× `CC0-1.0`, 1× `ODC-By-1.0`), Gate TERMS nachgezogen (`9c010f725`), lokal `terms-vocab 0 violation(s)` / 143 Zeilen / 9 distinct. `terms`-Direktiven existieren **nur** in `phi/sources.φ` (143); Katalog-Treffer sind englisches „in terms of". Frontier 5/5 = (b) migrieren (`state/stimmen/2026-10-07_terms-spdx-ui-antworten.md`); `own-work`-Riss vermessen + aufgelöst (eigene, quergeprüfte Kompilation; Fakten).
- **Blockade:** (ii) jeder Quellenblock ohne `terms` bildet auf eine lebende Dispositions-Zeile ab (Mountain) — ~160 offen.
- **Braucht:** die Dispositions-Zeilen (Mountain) für Bedingung (ii); (i)+(iii) stehen und sind verdrahtet.

### UI-Seat-Roster — Gretchenfrage-Rest
- **Status:** eigen (Retry)
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 259/260) aufgenommen (4/4 + Tempo): MiniMax M3 (30 s) · Google AI Studio/Gemini 3.1 Pro (≤60 s) · DeepSeek Chat (6 s) · Mistral (24 s). Offen: `chat.together.ai` Serverfehler → Retry; `aistudio.xiaomimimo.com` Ladefehler → unreachable.
- **Blockade:** Together + Xiaomi site-seitig.
- **Braucht:** Together-Retry; FMHY-Leads (`state/stimmen/2026-10-07_fmhy-ai-survey.md`) — je Gretchenfrage. Kandidatenakte: `state/stimmen/2026-10-07_ui-seat-kandidaten.md`.

### KC2G `prop.kc2g.com` — JSON-Reader-Arm (`blocked_sources.φ:234`)
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Register) `phi/blocked_sources.φ:234-236` (uncommittet, future-191/Mountain 268): `https://prop.kc2g.com/api/stations.json` JSON 200 (42482 B; mufd/fof2/tec/cs); `terms unbestimmt`; Arm: JSON-Reader.
- **Blockade:** keine.
- **Braucht:** JSON-Reader-Arm (Loader) + `url`/`format`-Zeile; bis dahin `pending`.

## An river

Origin: mycelium-folge258/260.

- **NUR-Asset `fmi_image_mag_nur.bin` — re-harvest läuft (force).** `image-cdn`-Lauf `37621964105` (`start=20031029`, `days=3`, `force=true`) noch queued; der neue sha folgt in `phi/sources.φ:18034`. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.
- **Lizenz-Census-Heimat — vollzogen (Option (c)).** `license_census.rs` joint zur Gate-Zeit gegen `phi/sources.φ`; `--fail` ist im `ci-gate`-`register`-Job verdrahtet; der `state/`-Census ist optionaler Komplement. **Braucht:** nur noch Bedingung (ii) (Mountain-Dispositions-Zeilen).
- **Stale see-also in deiner Survey (CI `path_reference_scan`).** `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md:7` zeigte auf den archivierten `handover-2026-10-07-river-folge122.md` (gemessen `ci_manage log 37622417707`). Die Survey liegt uncommittet im Baum geändert — deine Hand; nach deinem Commit fällt der rote `register`-Job.
- **`1-ui` (Alt-UI-Gruppe, river-Besitz) — schließen, wenn Mountain sie freigibt.** `browser_*` meldet `group "1-ui" is owned by another client`; nicht linien-exklusiv. **Braucht:** nach Mountains Round `1-ui` schließen; **kein Nachfolger** (`shared-ui` gestrichen); uniform nur `<line>-ui` (JIT) + `open-weight-ui` (JIT + Lock `state/zustand/ui-open-weight.lock`).

## An future

Origin: mycelium-folge255/260.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch) — jetzt inkl. `jaxa-gportal-cdn`-Dispatch (siehe `## Offen — eigen`).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).
- **Frontier-Antworten liegen vor:** `terms`-Vokabel-Quorum (6 Seats, `state/stimmen/2026-10-07_terms-vokabel-frontier-antworten.md`) — `unbestimmt`/`ohne-lizenz`, Quellen-Eigentümer; getragener Riss bei (b). SPDX-Migration gestellt: 5/5 = (b) migrieren (`..._terms-spdx-ui-antworten.md`). **Braucht:** Operator-Entscheid zum (b)-Riss (`keine`/NONE vs. `ohne-lizenz`) + SPDX-Migrationsauftrag (Mountain-Pen).
- **FMHY-Stimmen-/Werkzeug-Kandidaten (future-191 gefaltet):** Lumo (Proton, Gast-Login optional) als einziger erreichbar; ISH/ChatWave/LongCat/Tencent/Meta/Sarvam/Apertus login-gated; Poolside kein Web-Chat; Inception 403. Werkzeug-Kandidaten: Typst (Rust) · SimpleTex · LaTeX-OCR. **Braucht:** Operator/Rat-Entscheid, ob ein Seat aufgenommen wird; meine Roster-Registrierung dann. | `state/future/handover/handover-2026-10-07-future-folge191.md`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
