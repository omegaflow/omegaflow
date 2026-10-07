<!--
  title: Handover — Mycelium-Folge 263 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass. clean-tree Abnahme-Job (Schritt 2 der Bias-Tilgung) gebaut; CI-Rot gemessen (ci-gate clippy 2, register license_census 7); Handover-sha-Riss geheilt; adressierte Blöcke future-194/mountain-268 gefaltet; tools-build + gaia-cdn dispatcht; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-07
  sha256: ea58700d6352d854d26403809fb47ab12ec308525e818fff6ded9b3df0d013b5
  status: live
-->
# Handover — Mycelium-Folge 263 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge262.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.0030 · close 0.0732 · cap 0.5 — Grund: Meta-Pass, clean_tree-Bin + CI-Job gebaut, Pass geschrieben; kein pro/max; gemessen `session_burn` (line, deepseek-flash)

## Operator-Wort-Register

- Wort | 2026-10-07 | „bitte nicht verwalten deegieren und abschliessen" → Direktive: kein Verwaltungs-Theater, autonom delegieren/bauen und das Atom schließen. | Quelle: Operator (Session, Mycelium 262).
In 263 wurde kein neues Operator-Wort gegeben.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge262.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Das Operator-Gespräch ist verbatim in `state/operator-gespraeche/2026-10-07-mycelium.md` geschnitten.

## Offen — eigen

### NUR-Asset re-harvest (dB/dt–GIC-Kette) — `force`-Run läuft
- **Status:** wartend
- **Trigger:** `image-cdn`-Lauf-Ergebnis
- **Lage:** (gemessen 2026-10-07 via `ci_manage view`) `image-cdn`-Lauf `37621964105` (`start=20031029`, `days=3`, `force=true`, head `2ca80927d`) seit 12:34Z **queued** (self-hosted Runner); unverändert seit 262.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37621964105` → neuen sha von `fmi_image_mag_nur.bin` ins Register (`phi/sources.φ:18034`); dann River-Probe + Zahl Paper §4/§6.

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend (Mountain-Feld/ttl)
- **Trigger:** Mountains Einschreiben der 276 `field`/`ttl`-Zeilen in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-07, Mycelium 260/263) `.github/workflows/eionet-cdr-cdn.yml` steht (dispatch-only); `eionet_cdr_compiler --emit-field-names` druckt 276 `field`-Zeilen; Block-Header (`url`/`origin`/`compiler`/`format`) = Mycelium-Pen, wartet auf Mountains `terms`. Reader `src/archivar/osm_pbf.rs` + `src/archivar/eionet_cdr.rs` gebaut; `osm_nodes` registriert (`phi/sources.φ:1678-1684`).
- **Blockade:** Mountains `field`/`ttl`-Zeilen; geteilter Baum — `phi/sources.φ` nur schreiben, wenn Mountain freigibt.
- **Braucht:** Mountains Einschreiben; dann `url`/`origin`/`compiler`/`format`-Zeilen + Manifestation + Dispatch.

### OSHA-CEHD — Register-Zeile (Force-/Einheiten-`1`-Riss)
- **Status:** wartend (Mountain)
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07, Mycelium 260/263) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-Kontrakt (Compiler emittiert `1`). `phi/blocked_sources.φ:226` (`https://obis.osha.gov/opengov/healthsamples.zip`, `terms unbestimmt`) ist der offene Orphan-Träger dieser Linie.
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann Manifestation.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend (Mountain)
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260/263) `LICENSE`/`README` dort absent (HTTP 404 raw).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen); `license_census` zählt 2250 `no-terms` (gemessen 263).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Lizenz-Census — Gate verdrahtet (i+iii); SPDX-Migration vollzogen; Register-Job rot
- **Status:** wartend (Mountain-Dispositionen)
- **Trigger:** Mountain faltet die Dispositions-Zeilen in `docs/handover/handover-2026-10-07-mountain-folge269.md` (Bedingung ii)
- **Lage:** (gemessen 2026-10-07T~18:30Z, Mycelium 263 via `cargo run -p omegaflow-register --bin license_census`) `terms-vocab 7 violation(s)`: `unbestimmt` ×6 (Z. 1599/1608/1617/1656/1697/4024) + `OGL-Canada-2.0` ×1 (Z. 1732, `carisma_mag.bin`). Blocks 2684 · terms 153 · distinct 11 · no-terms 2250. `--fail` im `register`-Job von `ci-gate.yml` verdrahtet. SPDX-Migration im Baum = `34cbb52a5` (nicht `0fe79f7dd`; der alte sha war ein Attributions-Riss). Bedingung (ii): jeder Quellenblock ohne `terms` bildet auf eine lebende Dispositions-Zeile ab — ~2250 offen.
- **Blockade:** Mountains `terms`/Vokabel-Pen (die 6 `unbestimmt` + `OGL-Canada-2.0`).
- **Braucht:** `OGL-Canada-2.0` in die geschlossene Vokabel (Mountain) + `unbestimmt`-Auflösung + (ii)-Dispositions-Zeilen.

### dropped-gate — `--carrier` (Archiv-Move-Disziplin) gebaut; Re-Pin/CI-Verdrahtung offen
- **Status:** wartend (Pin-/Frontier-Entscheidung)
- **Trigger:** Operator-/Frontier-Wort (Sheet) zur Pin-Wahl + Archiv-Move-Disziplin
- **Lage:** (gemessen 2026-10-07, Mycelium 262) `dropped_gate.rs` trägt `--carrier`/`--count`: `derive_carriers()` leitet die Carrier je Lauf aus live-Handovers + `git log` + `phi/*.φ` ab (nie gespeichert); 3 Unit-Tests; `cargo build` grün, `--selftest passes`; `--carrier --count` = **13**. Der public-only-Roster ist seit Pin (HEAD `351632051`) um 10 Schlüssel verschoben. Offene Risse: Pin-Artefakt, Shadow-Härte, Ownership.
- **Blockade:** ungeklärte Pin-Wahl + Shadow-Vakuum.
- **Braucht:** Frontier-/Operator-Wort (Sheet `state/stimmen/2026-10-07_dropped-gate-ui-fragen.md`) → Re-Pin + Tombstone/Generation-Härtung + `ci-gate`-Verdrahtung.

### KC2G `prop.kc2g.com` — JSON-Reader-Arm (`blocked_sources.φ:236`)
- **Status:** wartend (Mountain-`terms`)
- **Trigger:** Mountains `terms`-Verdikt für `prop.kc2g.com`
- **Lage:** (gemessen 2026-10-07, Mycelium 262) JSON-Reader-Arm gebaut und committet (`tools/harvest/src/bin/kc2g_stations.rs`; 2 Tests). `pattern` in `phi/harvest.φ` + CDN-Workflow + `url`/`origin`/`compiler`/`format`-Block stehen nach Mountains `terms`.
- **Blockade:** `terms unbestimmt` (Mountain-Pen).
- **Braucht:** Mountains `terms`-Zeile; dann `pattern ^kc2g_stations\.csv$` + `kc2g-cdn.yml` + Register-Block + Dispatch.

### clean-tree Abnahme-Messung — Job gebaut, rot (Bias-Tilgung Schritt 2)
- **Status:** wartend (River/Mountain — Bias-Arbeit)
- **Trigger:** Rivers WP13-Lauf (`docs/concepts/remove-bias.md`) schließt die Treffer
- **Lage:** (gemessen 2026-10-07T~18:35Z, Mycelium 263 via `cargo run -p omegaflow-register --bin clean_tree`) neuer Job `clean-tree` im `register`-Job von `ci-gate.yml`; 266 Nicht-Test-`src/`-Dateien, **10 Treffer**: `nexrad.rs:188/382/398: EARTH_RADIUS`, `odp.rs:9: "earth"`, `rinex.rs:5: 6378137.0`, `rinex.rs:58: "earth"`, `media.rs:33: "earth"`, `weberin.rs:254/258: "earth"`, `MEDIA-TABLE media.rs`. Neuer Bin `tools/register/src/bin/clean_tree.rs` (3 Tests, `cargo check` grün).
- **Blockade:** die src-Bias-Arbeit (River `remove-bias.md` WP13) + `media.rs`-Tabelle (WP8/9/11).
- **Braucht:** Rivers WP13-Run (remove-bias) + `media.rs`→`BodyProperties`; danach ist der Job grün und bindet die Regression.

### UI-Seat-Roster — Gretchenfrage-Rest (Together-Retry gemessen)
- **Status:** wartend
- **Trigger:** ein Together-Submit liefert wieder eine Konversation (`state/stimmen/2026-10-07_ui-seat-kandidaten.md`)
- **Lage:** (gemessen 2026-10-07T~18:40Z, Mycelium 263 via `mycelium-ui`-Browser) `chat.together.ai` **lädt** (kein Serverfehler-Banner; Sidebar zeigt frühere Chats), aber ein Submit erzeugt **keine Konversation** (kein Chat-POST, URL bleibt `/`, Composer leert sich) → keine Antwort messbar; die Site trägt die Fähigkeit (frühere Chats existieren), dieser Submit-Pfad liefert nichts. Aufgenommen (4/4 + Tempo): MiniMax M3 (30 s) · Google AI Studio/Gemini 3.1 Pro (≤60 s) · DeepSeek Chat (6 s) · Mistral (24 s).
- **Blockade:** Together-Submit-Pfad site-seitig (kein Chat-POST).
- **Braucht:** erneuter Together-Submit in einem frischen Chat (evtl. anderer Browser-Zustand); Kandidatenakte `state/stimmen/2026-10-07_ui-seat-kandidaten.md`.

## An river

Origin: mycelium-folge258/260.

- **NUR-Asset `fmi_image_mag_nur.bin` — re-harvest läuft (force).** `image-cdn`-Lauf `37621964105` noch queued; der neue sha folgt in `phi/sources.φ:18034`. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.
- **`1-ui` (Alt-UI-Gruppe) — schließen.** `browser_*` meldet `group "1-ui" is owned by another client`; nicht linien-exklusiv. Konvention ist `<line>-ui` (JIT); Mycelium fährt `mycelium-ui`, kein Fremd-Composer. **Braucht:** nach Mountains Round `1-ui` schließen; **kein Nachfolger**.
- **Deine uncommitteten CI-Heilungen (gemessen, nicht committet):** `main_flow.rs:207` match→`?` und `goes16_mag.rs:37` `bytes[0..4] != MAGIC` (staged) heilen die zwei `clippy`-Fehler von `ci-gate 37661810257`. **Braucht:** committen, damit `ci-gate` grün wird.
- **clean-tree Treffer — deine src-Bias-Arbeit (Bias-Tilgung).** Der neue `ci-gate`-Schritt `clean-tree` misst 10 Treffer in `src/`: `nexrad.rs` EARTH_RADIUS ×3 · `odp.rs`/`rinex.rs`/`media.rs`/`weberin.rs` `"earth"` · `rinex.rs` `6378137.0` · `MEDIA-TABLE media.rs`. **Braucht:** `remove-bias.md` WP13 (src-Bias) → Job grün.

## An future

Origin: mycelium-folge255/260.

- **JAXA G-Portal `jaxa_gpm_ku` — Dispatch operator-gebunden.** Emit-Pfad + Workflow stehen; der `--download` schreibt in den JAXA-Account. **Braucht:** Operator-Wort → Dispatch `jaxa-gportal-cdn` (`dataset=12001000`, KuPR-Fenster, `download=true`, `fetch=true`, `gpm_ku=true`).
- **Frontier-(b)-Riss (SPDX):** 5/5 = (b) migrieren; Operator-Entscheid `keine`/NONE vs. `ohne-lizenz` offen — berührt die 6 `unbestimmt`-Register-Job-Verletzungen.
- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung.
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen.
- **FMHY-Stimmen-/Werkzeug-Kandidaten (future-191):** Lumo erreichbar; ISH/ChatWave/LongCat/Tencent/Meta/Sarvam/Apertus login-gated; Poolside kein Web-Chat; Inception 403. Werkzeuge: Typst (Rust) · SimpleTex · LaTeX-OCR. **Braucht:** Operator/Rat-Entscheid. | `state/future/handover/handover-2026-10-07-future-folge195.md`.
- **dropped-gate Re-Pin/Archiv-Move-Disziplin — Operator-/Frontier-Wort.** Sheet `state/stimmen/2026-10-07_dropped-gate-ui-fragen.md` (offen: Pin-Artefakt Offen- vs. Verlust-Menge, Shadow-Härte, Ownership). **Braucht:** Operator-/Frontier-Wort → Re-Pin mit gemessenem Grund + `ci-gate`-Verdrahtung.

## An mountain

Origin: mountain-folge267/268.

- **`tools-build` — dispatcht.** `gh workflow run tools-build.yml` → run `37670040626` (2026-10-07, Mycelium 263); der frische Ratchet schließt die 13 Baseline-Zeilen nicht mehr über ein altes `tools-latest`.
- **Sternkatalog `dr3_stars.bin` — re-harvest dispatcht** (`gh workflow run gaia-cdn.yml` → run `37670044924`; Träger `tap_compiler --star-bin`).
- **`license_census` 7 Verletzungen (dein `terms`-Pen):** `unbestimmt` ×6 (Z. 1599/1608/1617/1656/1697/4024) + `OGL-Canada-2.0` (Z. 1732, `carisma_mag.bin`); `ci-gate register` ist deswegen rot (gemessen `37661810257` + lokal 263). `OGL-Canada-2.0` in die geschlossene Vokabel oder Disposition.
- **CDN-Workflows gemessen vorhanden:** `osm-pbf-cdn.yml` · `fink-cutout-cdn.yml` · `nasa-power-t2m-cdn.yml` · `jaxa-gportal-cdn.yml` · `epa-aqs-voc-cdn.yml` · `vnp46a3-cdn.yml`; offen bleiben die Harvest-`pattern`-Zeilen (`^monaco_nodes\.bin$`, `^fink_cutout\.bin$`) + `vnp46a3`-Granule (DNB-Nachtdaten).
- **clean-tree-Abnahme (Bias-Tilgung Schritt 2):** Schritt im `register`-Job (`cargo run -q -p omegaflow-register --bin clean_tree -- --fail`); die 10 Treffer in src sind Rivers src-Bias-Arbeit (`## An river`), nicht deine.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
