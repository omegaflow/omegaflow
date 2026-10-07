<!--
  title: Handover — Mycelium-Folge 262 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; KC2G-JSON-Reader-Arm (Vorgänger-Atom) committet; dropped-gate --carrier (Archiv-Move-Disziplin) committet; übrige Punkte wartend auf Mountain/Operator/Runner; Stehender Pass am HEAD neu geschrieben.
  class: handover
  date: 2026-10-07
  sha256: eaa147a7c9152d138d96a2bf598071c65c7628910253747f18ea0883a7afcfe9
  status: live
-->
# Handover — Mycelium-Folge 262 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge261.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An river` / `## An future`.

## Burn: open 0.0 · close 0.0442 · cap 0.5 — Grund: Meta-Pass, zwei eigene Bins gebaut/geprüft (`dropped_gate --carrier`, `kc2g_stations`), Pass geschrieben; kein pro/max; gemessen `session_burn` (line, deepseek-flash)

## Operator-Wort-Register

- Wort | 2026-10-07 | „bitte nicht verwalten deegieren und abschliessen" → Direktive: kein Verwaltungs-Theater, autonom delegieren/bauen und das Atom schließen. | Quelle: Operator (Session, Mycelium 262).
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge261.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Das Operator-Gespräch ist verbatim in `state/operator-gespraeche/2026-10-07-mycelium.md` geschnitten.

## Offen — eigen

### NUR-Asset re-harvest (dB/dt–GIC-Kette) — `force`-Run läuft
- **Status:** wartend
- **Trigger:** `image-cdn`-Lauf-Ergebnis
- **Lage:** (gemessen 2026-10-07T13:56Z, Mycelium 262 via `ci_manage view`) `image-cdn`-Lauf `37621964105` (`start=20031029`, `days=3`, `force=true`, head `2ca80927d`) seit 12:34Z **queued** (self-hosted Runner); Idempotenz-Guard durch `force` übersteuert.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37621964105` → neuen sha von `fmi_image_mag_nur.bin` ins Register (`phi/sources.φ:18034`); dann River-Probe + Zahl Paper §4/§6.

### jaxa_gpm_ku — Emit-Pfad gemessen, Workflow verdrahtet; Dispatch operator-gebunden
- **Status:** wartend (Operator-Wort)
- **Trigger:** Operator-Wort für den G-Portal-Download/Fetch
- **Lage:** (gemessen 2026-10-07, Mycelium 261) Der Emit-Pfad **existiert**: `jaxa_gportal_compiler --granule <hdf5> --bin <out> --ci-mode` (`tools/harvest/src/bin/jaxa_gportal_compiler.rs:1170-1245`) schreibt und (ci-mode) uploadet `jaxa_gpm_ku.bin` (format `jaxa_gpm_ku`; Register-Block `phi/sources.φ:2049-2055` vollständig). `.github/workflows/jaxa-gportal-cdn.yml` um Input `gpm_ku` + Compile-Schritt erweitert.
- **Blockade:** der `--download` schreibt in den JAXA-Account (per-Akt-Consent) → der Dispatch ist Operator-Hand.
- **Braucht:** Operator-Wort → Dispatch `jaxa-gportal-cdn` mit `dataset=12001000`, `from`/`to` (KuPR-Fenster), `download=true`, `fetch=true`, `gpm_ku=true`; danach sha von `jaxa_gpm_ku.bin` in `phi/sources.φ` und Handover.

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend (Mountain-Feld/ttl)
- **Trigger:** Mountains Einschreiben der 276 `field`/`ttl`-Zeilen in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-07, Mycelium 260) `.github/workflows/eionet-cdr-cdn.yml` gebaut (dispatch-only); `eionet_cdr_compiler --emit-field-names` druckt 276 `field`-Zeilen; Register-Block in `phi/sources.φ` noch **0** Treffer. `cdn_reconcile --fail` geheilt (Ausnahme `cdr.eionet.europa.eu` in `docs/specs/cdn-tag-baseline.txt`, OSHA-Präzedenz), lokal grün (291 Hosts). Der getragene Kraft-Riss (Medium `diffusion kg` + Punkt-Kernel Slot 2; Minderheit `gravity kg`/`pending`) blockiert den Schreibakt nicht. Reader `src/archivar/{osm_pbf,eionet_cdr}.rs` gebaut; `osm_nodes` registriert (`phi/sources.φ:1678-1684`). **tools-build** `37631266911` success.
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
- **Blockade:** die `terms`-Zeilen (Mountain-Pen) noch nicht vollständig (~160 `terms unbestimmt`-Zeilen offen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### dropped-gate — `--carrier` (Archiv-Move-Disziplin) gebaut; Re-Pin/CI-Verdrahtung offen
- **Status:** wartend (Pin-/Frontier-Entscheidung)
- **Trigger:** Operator-/Frontier-Wort (Sheet) zur Pin-Wahl + Archiv-Move-Disziplin
- **Lage:** (gemessen 2026-10-07, Mycelium 262) `dropped_gate.rs` trägt jetzt zusätzlich `--carrier`/`--count`: `derive_carriers()` leitet die Carrier je Lauf aus live-Handovers + `git log` + `phi/*.φ` ab (nie gespeichert), `dropped_keys` zählt einen Pin-Schlüssel nur als dropped, wenn er auf keiner Carrier-Zeile als ganzes Token steht (3 Unit-Tests: live-carried, archiv-ohne-Träger, exakter Token). `cargo build` grün, `--selftest passes`. **Gemessen:** `--carrier --count` = **13** (2026-10-07T16:07Z; die public-only-Pin-Differenz bleibt auch mit Carrier-Ableitung, der Archiv-Move `folge261` entfernt einen Live-Träger). Der public-only-Roster ist seit Pin (HEAD `351632051`) um 10 Schlüssel verschoben. Offene Risse im Design-Sheet: Pin-Artefakt (Offen- vs. Verlust-Menge), Shadow-Härte, Ownership.
- **Blockade:** ungeklärte Pin-Wahl + Shadow-Vakuum.
- **Braucht:** Frontier-/Operator-Wort (Sheet `state/stimmen/2026-10-07_dropped-gate-ui-fragen.md`) → Re-Pin mit gemessenem Grund + Tombstone/Generation-Härtung + `ci-gate`-Verdrahtung.

### Lizenz-Census — Gate verdrahtet (i+iii); SPDX-Migration vollzogen
- **Status:** wartend (Mountain-Dispositionen)
- **Trigger:** Mountain faltet die Dispositions-Zeilen in `docs/handover/handover-2026-10-07-mountain-folge268.md` (Bedingung ii)
- **Lage:** (gemessen 2026-10-07, Mycelium 261) `license_census.rs` erzwingt die geschlossene `terms`-Vokabel auf `phi/sources.φ` (Join am Quellenblock), `--fail` im `register`-Job von `ci-gate.yml` verdrahtet. `terms`→SPDX vollzogen (Mountain `0fe79f7dd`, Gate `9c010f725`), lokal `terms-vocab 0 violation(s)` / 143 Zeilen / 9 distinct. Frontier 5/5 = (b) migrieren.
- **Blockade:** (ii) jeder Quellenblock ohne `terms` bildet auf eine lebende Dispositions-Zeile ab (Mountain) — ~160 offen.
- **Braucht:** die Dispositions-Zeilen (Mountain) für Bedingung (ii); (i)+(iii) stehen.

### KC2G `prop.kc2g.com` — JSON-Reader-Arm (`blocked_sources.φ:236`)
- **Status:** wartend (Mountain-`terms`)
- **Trigger:** Mountains `terms`-Verdikt für `prop.kc2g.com`
- **Lage:** (gemessen 2026-10-07, Mycelium 262) JSON-Reader-Arm **gebaut und committet** (`tools/harvest/src/bin/kc2g_stations.rs`: `--url`/`--input`/`--out`/`--ci-mode`, JSON-Reader `parse_stations` → CSV `code,lat_deg,lon_deg,mufd_mhz,fof2_mhz,tec_tecu,cs,time_unix`, Release-Upload, 2 Tests; `cargo build` grün). Der Register-Vermerk nennt den Arm. `pattern` in `phi/harvest.φ` + CDN-Workflow + `url`/`origin`/`compiler`/`format`-Block stehen nach Mountains `terms` (blocked_sources trägt `terms unbestimmt`).
- **Blockade:** `terms unbestimmt` (Mountain-Pen) — Manifestation erst nach Admission.
- **Braucht:** Mountains `terms`-Zeile; dann `pattern ^kc2g_stations\.csv$` + `kc2g-cdn.yml` + Register-Block + Dispatch.

### UI-Seat-Roster — Gretchenfrage-Rest
- **Status:** wartend
- **Trigger:** Together-Retry (`state/stimmen/2026-10-07_ui-seat-kandidaten.md`)
- **Lage:** (gemessen 2026-10-07, Mycelium 259/260) aufgenommen (4/4 + Tempo): MiniMax M3 (30 s) · Google AI Studio/Gemini 3.1 Pro (≤60 s) · DeepSeek Chat (6 s) · Mistral (24 s). Offen: `chat.together.ai` Serverfehler → Retry; `aistudio.xiaomimimo.com` Ladefehler → unreachable. Der lokale Stimmen-Roster ist strikt DeepSeek-only (Operator-Wort 2026-10-07); die UI-Chats bleiben der zweite Kanal.
- **Blockade:** Together + Xiaomi site-seitig.
- **Braucht:** Together-Retry; FMHY-Leads (`state/stimmen/2026-10-07_fmhy-ai-survey.md`) — je Gretchenfrage. Kandidatenakte: `state/stimmen/2026-10-07_ui-seat-kandidaten.md`.

## An river

Origin: mycelium-folge258/260.

- **NUR-Asset `fmi_image_mag_nur.bin` — re-harvest läuft (force).** `image-cdn`-Lauf `37621964105` noch queued; der neue sha folgt in `phi/sources.φ:18034`. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.
- **Stale see-also in deiner Survey (CI `path_reference_scan`).** `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md:7` zeigte auf den archivierten `handover-2026-10-07-river-folge122.md`. Die Survey liegt uncommittet im Baum geändert — deine Hand; nach deinem Commit fällt der rote `register`-Job.
- **`1-ui` (Alt-UI-Gruppe) — schließen, wenn Mountain sie freigibt.** `browser_*` meldet `group "1-ui" is owned by another client`; nicht linien-exklusiv. **Braucht:** nach Mountains Round `1-ui` schließen; **kein Nachfolger** (uniform nur `<line>-ui` (JIT) + `open-weight-ui`).

## An future

Origin: mycelium-folge255/260.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch) — inkl. `jaxa-gportal-cdn`-Dispatch (siehe `## Offen — eigen`).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).
- **Frontier-Antworten liegen vor:** `terms`-Vokabel-Quorum (6 Seats) — `unbestimmt`/`ohne-lizenz`, Quellen-Eigentümer; getragener Riss bei (b). SPDX-Migration: 5/5 = (b) migrieren. **Braucht:** Operator-Entscheid zum (b)-Riss (`keine`/NONE vs. `ohne-lizenz`).
- **FMHY-Stimmen-/Werkzeug-Kandidaten (future-191 gefaltet):** Lumo (Proton, Gast-Login optional) als einziger erreichbar; ISH/ChatWave/LongCat/Tencent/Meta/Sarvam/Apertus login-gated; Poolside kein Web-Chat; Inception 403. Werkzeug-Kandidaten: Typst (Rust) · SimpleTex · LaTeX-OCR. **Braucht:** Operator/Rat-Entscheid. | `state/future/handover/handover-2026-10-07-future-folge191.md`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
