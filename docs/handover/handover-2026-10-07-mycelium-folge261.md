<!--
  title: Handover — Mycelium-Folge 261 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; Sternkatalog-Sort + vnp46a3 aufgelöst; jaxa_gpm_ku `--granule`-Emit-Pfad verdrahtet; CDN-Audit (986 Blöcke) 9 Workflow-Lücken geschlossen; dropped-gate Schritt 1+2 gebaut (`dropped_gate.rs`), Lizenz-Census Tor-Bedingung (i) (9er-Vokabel); Recherche-Schicht (SPDX + dropped-gate) + UI-Frontier-Sheets; CI-Tafel (format river/mountain, path_reference_scan river)
  class: handover
  date: 2026-10-07
  sha256: eeca848be6af1ae86ee92800fc432099757eb74e07f6852aace183c58c0347be
  status: live
-->
# Handover — Mycelium-Folge 261 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge260.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An river` / `## An future`.

## Burn: open 0.0 · close 0.0962 · cap 0.5 — Grund: Meta-Pass + Agenten-Kohorte (4 flash + 1 schwache Stimme + weitere flash-Bauschritte), kein pro/max; gemessen `session_burn` (line, deepseek-flash) bei Commit

## Operator-Wort-Register

- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | Quelle: Operator (Session, Mycelium 261) — Session-Start, Delegations-Consent.
- Wort | 2026-10-07 | „endlich mal die arbeit fertig machen" → Delegationsauftrag: eine Agenten-Kohorte (flash-first) parallel auf die arbeitsfähigen Mycelium-Atome. | Quelle: Operator (Session, Mycelium 261).
- Wort | 2026-10-07 | „ja bitte" → Recherche-Schicht für `terms`-SPDX + dropped-gate-Thresholds starten und die UI-Frontier-Fragen-Sheets bereitlegen. | Quelle: Operator (Session, Mycelium 261).
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

### dropped-gate — Schritt 1+2 gebaut; Shadow vakuos, Härtung + Pin-Wahl offen
- **Status:** eigen
- **Trigger:** — (autonom; Bau als flash-Sequenz)
- **Lage:** (gemessen 2026-10-07, Mycelium 261) `tools/register/src/bin/dropped_gate.rs` gebaut (gepusht `7c4119e70`): getyptes Vokabular `minted/carried/resolved/dropped` + `fold_events(log,pin)` + Shadow-Nullkontrolle; `--selftest`/`--shadow` grün. **Recherche-Risse (2026-10-07, getragen):** (1) der Shadow ist **vakuos** — `false_red` kann strukturell nie ≠ 0 werden, Silent-Loss per `.abs_diff(k)` neutralisiert → `--shadow` meldet immer `sharp=true`; (2) `pending-legacy` meint im Glossar die Verlust-Menge (voller Scan, letzter Sweep ~3105/993), `dropped_gate.rs:3` pinnt aber die 37-Schlüssel-Offen-Menge; (3) Revival-Lücke (`minted` nach `dropped` ohne Tombstone/Generation); (4) Roster/`archiv`-Asymmetrie.
- **Blockade:** kein einschaltbares Tor — vakuoser Shadow + ungeklärter Pin färben jede legitime Schließung rot.
- **Braucht:** UI-Frontier-Urteil (Sheet `state/stimmen/2026-10-07_dropped-gate-ui-fragen.md`, Operator-Wort) → dann Härtung (Tombstone/Generation + echtes `false_red`, begrenzter Schritt), Pin-Wahl, `pending-legacy`-Snapshot (wöchentlich), `ci-gate`-Verdrahtung. Verdikt: `state/stimmen/2026-10-07_dropped-gate-stimmen-runde.md` + `-runde-2.md`.

### Lizenz-Census — Tor-Bedingung (i) gebaut; SPDX-Migration + Mountain-`terms` offen
- **Status:** eigen → operator-gebunden (SPDX-Frage)
- **Trigger:** Operator-Wort für die UI-Frontier-Runde (Sheet) + Mountains `terms`-Befüllung
- **Lage:** (gemessen 2026-10-07, Mycelium 261) Tor-Bedingung (i) gebaut (gepusht `7c4119e70`): `license_census.rs` erzwingt die geschlossene `terms`-Vokabel byte-exakt; 0 Verstöße / 143 Zeilen / 9 distinct. Rat-Verdikt: die 9 exakten Werte. **Recherche-Riss (2026-10-07):** 3 der 9 weichen von SPDX ab (`CC0`→`CC0-1.0`, `ODC-BY-1.0`→`ODC-By-1.0`; `PD` hat keinen SPDX-Identifier); DataCite=Freitext+optional SPDX, EUMETSAT nicht CC, SPDX kennt `LicenseRef-…`. Empfehlung: hybrid (b′) — Lizenz-Tokens auf SPDX, Rest als Status-Achse, nie still mappen.
- **Blockade:** (ii) Mountains ~160 `terms unbestimmt`-Zeilen fehlen (`phi/blocked_sources.φ` leer); (iii) Tree-Count − `terms`-Count.
- **Braucht:** UI-Frontier-Urteil zur SPDX-Migration (Sheet `state/stimmen/2026-10-07_terms-spdx-ui-fragen.md`, Operator-Wort) + Mountains `terms`; dann (ii)/(iii) + `ci-gate`-Schritt. Der `state/`-Pregate bleibt Komplement, nie das Tor.

### UI-Seat-Roster — Gretchenfrage-Rest
- **Status:** eigen (Retry)
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 259/260) aufgenommen (4/4 + Tempo): MiniMax M3 (30 s) · Google AI Studio/Gemini 3.1 Pro (≤60 s) · DeepSeek Chat (6 s) · Mistral (24 s). Offen: `chat.together.ai` Serverfehler → Retry; `aistudio.xiaomimimo.com` Ladefehler → unreachable.
- **Blockade:** Together + Xiaomi site-seitig.
- **Braucht:** Together-Retry; FMHY-Leads (`state/stimmen/2026-10-07_fmhy-ai-survey.md`) — je Gretchenfrage. Kandidatenakte: `state/stimmen/2026-10-07_ui-seat-kandidaten.md`.

## An river

Origin: mycelium-folge258/260.

- **NUR-Asset `fmi_image_mag_nur.bin` — re-harvest läuft (force).** `image-cdn`-Lauf `37621964105` (`start=20031029`, `days=3`, `force=true`) noch queued; der neue sha folgt in `phi/sources.φ:18034`. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.
- **Lizenz-Census-Heimat — entschieden (Option (c)).** Dein Rat-Verdikt gefaltet: `license_census.rs` joint zur Gate-Zeit direkt gegen `phi/sources.φ` (tracked), nicht gegen `state/river/license-census.tsv`. Der `state/`-Pregate bleibt Komplement. **Braucht:** die geschlossene `terms`-Vokabel als Definition; danach verdrahte ich den `ci-gate`-Schritt.
- **Stale see-also in deiner Survey (CI `path_reference_scan`).** `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md:7` zeigte auf den archivierten `handover-2026-10-07-river-folge122.md` (gemessen `ci_manage log 37622417707`). Die Survey liegt uncommittet im Baum geändert — deine Hand; nach deinem Commit fällt der rote `register`-Job.
- **`1-ui` (Alt-UI-Gruppe, river-Besitz) — schließen, wenn Mountain sie freigibt.** `browser_*` meldet `group "1-ui" is owned by another client`; nicht linien-exklusiv. **Braucht:** nach Mountains Round `1-ui` schließen; **kein Nachfolger** (`shared-ui` gestrichen); uniform nur `<line>-ui` (JIT) + `open-weight-ui` (JIT + Lock `state/zustand/ui-open-weight.lock`).

## An future

Origin: mycelium-folge255/260.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch) — jetzt inkl. `jaxa-gportal-cdn`-Dispatch (siehe `## Offen — eigen`).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).
- **UI-Frontier-Fragen liegen bereit (Operator-Queue):** `state/stimmen/2026-10-07_terms-spdx-ui-fragen.md` (SPDX-Migration) + `state/stimmen/2026-10-07_dropped-gate-ui-fragen.md` (Schwellen/Ownership) — Recherche-Schicht gefahren, Quellen URL-gemessen. **Braucht:** Operator-Wort für die UI-Chat-Runde (Send = Operator-Hand).

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
