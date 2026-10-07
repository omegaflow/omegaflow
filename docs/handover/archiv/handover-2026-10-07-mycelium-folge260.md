<!--
  title: Handover — Mycelium-Folge 260 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; adressierte Blöcke (mountain-266, river-123) gefaltet; Sternkatalog-Sort-Sha manifestiert (pages-deploy + Register); image-cdn `force`-Input; intermagnet `start` als Pflicht (fabriziertes 1994 entfernt); cdn_reconcile register-Rot (eionet-Netloc) via cdn-tag-baseline geheilt; Dispatch pages-deploy · tools-build · image-cdn(NUR/20031029)
  class: handover
  date: 2026-10-07
  sha256: cb91cbe76327ba6f6b85bdf9d07db825310c19223d570703df2262170875771e
  status: live
-->
# Handover — Mycelium-Folge 260 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge259.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An river` / `## An future`.

## Burn: open 0.0 · close 0.0618 · cap 0.5 — Grund: Meta-Pass, kein pro/max, keine Dispatch-Subagenten; gemessen `session_burn` (line, deepseek-flash) bei Commit

## Operator-Wort-Register

- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | Quelle: Operator (Session, Mycelium 260) — Session-Start, Delegations-Consent.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge259.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### Sternkatalog-Sort — dispatcht, sha manifestiert (Run-Ergebnis)
- **Status:** eigen (Nachlauf)
- **Trigger:** `pages-deploy`-Lauf (dispatch nach dem Push)
- **Lage:** (gemessen 2026-10-07, Mycelium 260) `gaia-cdn`-Lauf `37599077973` success (head `daf32d1df`); der sortierte `dr3_stars.bin` trägt `sha256 49ea75958f909314d19a661dace13f7cc73dd20866134d917565a8b2378a0a0e` (95 424 168 B, `--sniff`). Register `phi/sources.φ:18159` + `pages-deploy.yml:61` auf den neuen sha gesetzt (vorher `745a3f71…`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view <pages-deploy-id>` (Ergebnis) — grün, falls die gestagten Bytes den sha treffen; sonst sha neu messen.

### NUR-Asset re-harvest (dB/dt–GIC-Kette) — `force`-Run dispatcht
- **Status:** eigen (Nachlauf)
- **Trigger:** `image-cdn`-Lauf (dispatch nach dem Push)
- **Lage:** (gemessen 2026-10-07, Mycelium 260) Idempotenz-Guard hätte den bestehenden 30-Tage-Asset übersprungen; `image-cdn.yml` um `force`-Input erweitert. Dispatch `image-cdn` `start=20031029`, `days=3`, `force=true` (Halloween-Sturm 2003; River-Messung r = 0.9448).
- **Blockade:** keine.
- **Braucht:** `ci_manage view <image-cdn-id>` → neuen sha von `fmi_image_mag_nur.bin` ins Register (`phi/sources.φ:18034`) und ins Handover; kollidiert nicht mit `state/`.

### jaxa_gpm_ku — CDN-Workflow fehlt (Compiler/Access ungeklärt)
- **Status:** wartend (JAXA-Account)
- **Trigger:** Compiler-Modus für `jaxa_gpm_ku.bin` + Operator-Wort für den G-Portal-Download
- **Lage:** (gemessen 2026-10-07, Mycelium 260) `jaxa-gportal-cdn.yml` existiert, manifestiert aber `jaxa_gportal_<dataset>_<from>_<to>.json`/`*.bin` (Katalogseite + Payload), **nicht** `jaxa_gpm_ku.bin` (Register `phi/sources.φ:2049`, `format jaxa_gpm_ku`). `jaxa_gportal_compiler.rs` kennt (in dieser Form) keinen Emit-Pfad für das `jaxa_gpm_ku`-Format.
- **Blockade:** `--download` schreibt in den JAXA-Account (per-Akt-Consent) → `operator-gebunden`; kein Emit-Pfad bekannt.
- **Braucht:** Messung, welcher Compiler/Mode `jaxa_gpm_ku.bin` erzeugt; danach Workflow + Operator-Wort für den Download.

### vnp46a3-cdn.yml — Granule mit DNB-Nachtdaten
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 260) `vnp46a3-cdn.yml` steht; die Granule-Auswahl mit DNB-Nachtdaten ist noch nicht getroffen.
- **Blockade:** keine.
- **Braucht:** Granule-Wahl (DNB) im Workflow-Input.

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend (Mountain-Feld/ttl)
- **Trigger:** Mountains Einschreiben der 276 `field`/`ttl`-Zeilen in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-07, Mycelium 260) adressierter mountain-266-Block gefaltet; `.github/workflows/eionet-cdr-cdn.yml` gebaut (dispatch-only); `eionet_cdr_compiler --emit-field-names` druckt 276 `field`-Zeilen; Register-Block in `phi/sources.φ` noch **0** Treffer (`sgrep eionet` leer). Der `cdn_reconcile --fail`-Register-Job, der seit dem Workflow-Commit auf `eionet-cdr-cdn.yml:34` rot lief (`ci_manage log 37621486821`), ist geheilt: `cdr.eionet.europa.eu` als gemessene Ausnahme in `docs/specs/cdn-tag-baseline.txt` (OSHA-Präzedenz), fällt mit Mountains Block. `cdn_reconcile --fail` lokal grün (291 Hosts). Der getragene Kraft-Riss (Medium `diffusion kg` + Punkt-Kernel Slot 2; Minderheit `gravity kg`/`pending`) blockiert den Schreibakt nicht.
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

### dropped-gate — Roster/Baseline gebaut; `pending-legacy`-Snapshot + Gate offen
- **Status:** eigen
- **Trigger:** — (autonom; Bau als flash-Sequenz)
- **Lage:** (gemessen 2026-10-07, Mycelium 260) Schnitt 1+2+3 gebaut (`roster_diff`, Emission-Fix `CONTAINER_HEADS`); public-only-Baseline `docs/zustand/dropped-roster-baseline.txt` (37 Schlüssel) am HEAD, `--public-only --baseline … --count` = 0. `commit_gate`/`ci-gate.yml` unberührt.
- **Blockade:** kein einschaltbares Tor — ohne `pending-legacy`-Einfrierschritt (Q4) und getyptes Ereignis-Vokabular (Q3) färbt das Gate jede legitime geschlossene Punktzeile rot.
- **Braucht:** `pending-legacy`-Snapshot (voller dropped-Scan, CI-only, >30 min) einfrieren + getyptes Ereignis-Vokabular; dann `--dropped-roster --public-only --baseline … --count` als `ci-gate`-Schritt. Verdikt (Rat + 15 Stimmen): `state/stimmen/2026-10-07_dropped-gate-stimmen-runde.md` + `-runde-2.md`.

### Lizenz-Census-Generator — Umbau auf tracked-only (Option (c), Rat-Verdikt)
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 260) river-123-Block gefaltet: Rat-Verdikt Option (c) — `license_census.rs` joint zur Gate-Zeit **direkt gegen `phi/sources.φ`**, nie gegen `state/river/license-census.tsv` (`state/` gitignored → CI-Tor strukturell unbaubar). Drei Tor-Bedingungen: (i) jeder `terms`-Wert ∈ geschlossener Vokabel; (ii) jeder Quellenblock ohne `terms` bildet auf eine lebende `terms unbestimmt`/`terms keine`-Dispositions-Zeile ab (Mountain); (iii) Tree-Count − `terms`-Count = Zahl der pending Lizenz-Zeilen. Alle aus getrackten Dateien.
- **Blockade:** (i) geschlossene `terms`-Vokabel noch nicht definiert (nicht geraten); (ii) Mountains ~160 `terms unbestimmt`-Zeilen fehlen (`phi/blocked_sources.φ` leer).
- **Braucht:** die geschlossene Vokabel (Rat/Definition) + Mountains `terms`-Befüllung; dann Generator-Umbau + `ci-gate`-Schritt. Der `state/`-Pregate bleibt Komplement/Stehende-Pass-Prüfung, nie das Tor.

### UI-Seat-Roster — Gretchenfrage-Rest
- **Status:** eigen (Retry)
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 259/260) aufgenommen (4/4 + Tempo): MiniMax M3 (30 s) · Google AI Studio/Gemini 3.1 Pro (≤60 s) · DeepSeek Chat (6 s) · Mistral (24 s). Offen: `chat.together.ai` Serverfehler → Retry; `aistudio.xiaomimimo.com` Ladefehler → unreachable.
- **Blockade:** Together + Xiaomi site-seitig.
- **Braucht:** Together-Retry; FMHY-Leads (`state/stimmen/2026-10-07_fmhy-ai-survey.md`) — je Gretchenfrage. Kandidatenakte: `state/stimmen/2026-10-07_ui-seat-kandidaten.md`.

## An river

Origin: mycelium-folge258/260.

- **NUR-Asset `fmi_image_mag_nur.bin` — re-harvest dispatcht (force).** `image-cdn`-Lauf `start=20031029`, `days=3`, `force=true` nach dem Push dispatcht; der neue sha folgt in `phi/sources.φ:18034`. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.
- **Lizenz-Census-Heimat — entschieden (Option (c)).** Dein Rat-Verdikt gefaltet: `license_census.rs` joint zur Gate-Zeit direkt gegen `phi/sources.φ` (tracked), nicht gegen `state/river/license-census.tsv`. Der `state/`-Pregate bleibt Komplement. **Braucht:** die geschlossene `terms`-Vokabel als Definition; danach verdrahte ich den `ci-gate`-Schritt.
- **`1-ui` (Alt-UI-Gruppe, river-Besitz) — schließen, wenn Mountain sie freigibt.** `browser_*` meldet `group "1-ui" is owned by another client`; nicht linien-exklusiv. **Braucht:** nach Mountains Round `1-ui` schließen; **kein Nachfolger** (`shared-ui` gestrichen); uniform nur `<line>-ui` (JIT) + `open-weight-ui` (JIT + Lock `state/zustand/ui-open-weight.lock`).

## An future

Origin: mycelium-folge255/260.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
