<!--
  title: Handover — Mycelium-Folge 259 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; adressierte Blöcke (mountain-262, river-122) am Baum gemessen gefaltet; eionet-cdr-cdn.yml gebaut; Lizenz-Census-Join auf Netloc-Ebene; dropped-gate Roster-Baseline gebunden; Stehender Pass am neuen HEAD
  class: handover
  date: 2026-10-07
  sha256: 947a084a2c7df5be0d84592e30dde0c61cdff136f2d53b9ca2c789f8a8fa7a2e
  status: live
-->
# Handover — Mycelium-Folge 259 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge258.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An river`.

## Burn: open 0.0 · close 0.0 · cap 0.5 — Grund: Meta-Pass, kein pro/max, keine Dispatch-Subagenten; gemessen `session_burn` (line, deepseek-flash)

## Operator-Wort-Register

- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | Quelle: Operator (Session, Mycelium 259) — Session-Start, Delegations-Consent.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge258.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt + Block-Grenzfall offen)
- **Status:** eigen (Block-Header) · wartend (Kraft-Verdikt)
- **Trigger:** Mountain-263 folded (2026-10-07): der 276-`field`-Block ist via `eionet_cdr_compiler --emit-field-names` reproduzierbar erzeugt; Block-Header ist mein Pen
- **Lage:** (gemessen 2026-10-07, Mycelium 259) adressierter mountain-262/263-Block gefaltet. Festquelle `EU_Report_2017_27Aug19.xml` closed; `eionet_cdr_compiler.rs` am Baum (format `eionet_cdr`, CDN-Tag `cdr.eionet.europa.eu`; `--emit-field-names` druckt 276 `field`-Zeilen). `.github/workflows/eionet-cdr-cdn.yml` gebaut (dispatch-only, Release + Compile + Verify via `register_release_set.sh`). Register-Block in `phi/sources.φ` noch `0` Treffer.
- **Blockade:** **keine Architektur-Frage** — `AGENTS.md:426` ist die stehende Grenze: Mountain schreibt `field`/`ttl` (Datenkontrakt/Verdikt), Mycelium `url`/`origin`/`compiler`/`format` (Mountain 263 weist `format` ausdrücklich meiner Feder zu). Der Block ist also der Normalfall der Zwei-Feder, kein neuer Grenzfall; kein Rat nötig (frühere Notiz berichtigt). Offen bleibt der getragene Kraft-Riss (Medium `diffusion kg` + Punkt-Kernel Slot 2; Minderheit `gravity kg`/`pending`) — er blockiert den Schreibakt nicht.
- **Braucht:** Mountains Einschreiben der 276 `field`/`ttl`-Zeilen (seine Klasse) — dann meine `url`/`origin`/`compiler`/`format`-Zeilen + Manifestation + `eionet-cdr-cdn.yml`-Dispatch. Achtung geteilter Baum: `phi/sources.φ` nur schreiben, wenn die Mountain-Session das Blatt freigegeben hat.

### OSHA-CEHD — Register-Zeile (Force-/Einheiten-`1`-Riss)
- **Status:** wartend
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07, Mycelium 259) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-Kontrakt (Compiler emittiert Masse/Massenkonzentration/amount-fraction `1`).
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann Manifestation.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255) `LICENSE`/`README` dort absent (HTTP 404 raw).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen) sind noch nicht vollständig.
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Sternkatalog nach Helligkeit ordnen (Membran progressives Laden (C))
- **Status:** wartend (auf den gaia-cdn-Lauf)
- **Trigger:** `gaia-cdn`-Lauf `37599077973` (dispatch 2026-10-07) landet mit sortiertem `dr3_stars.bin`
- **Lage:** (gemessen 2026-10-07, Mycelium 259) mountain-263 folded: der Sort ist gebaut — `tap_compiler --star-bin` sortiert 56-B-weise nach Magnitude aufsteigend (non-finite ans Ende), `tycho2_compiler --source tgas`/`tycho` ebenso (`cargo build` grün). **Dispatched:** `gh workflow run gaia-cdn.yml` → Run `37599077973` (baut `dr3_stars.bin` neu, Release-Tag `ssd.jpl.nasa.gov-gaia`).
- **Blockade:** kein — wartet auf den Lauf; danach der neue sha.
- **Braucht:** `ci_manage view 37599077973` (Ergebnis) → neuen sha ins Register + `pages-deploy.yml`-Stage, Sichtbarkeits-Reihenfolge `sun, earth, moon, katalog`.

### dropped-gate — Roster-Emission + Baseline binden gebaut; pending-legacy-Snapshot + Gate offen
- **Status:** eigen
- **Trigger:** — (autonom; Bau als flash-Sequenz)
- **Lage:** (gemessen 2026-10-07, Mycelium 259) Ratifiziertes Design (Rat + 5 API + 14 UI): 20/20 Register-/Carrier-Frage; Q1 hybrid (ID = Identität, Token nur `match_hint`), Q2 Roster, Q3 hart bei stillem/ungetyptem Move, Q4 `pending-legacy` einfrieren; Reihenfolge Nullkontrolle → ID-Feld → Snapshot → Vokabular → Roster-Gate → hartes Rot. Schnitt 1 committet (`1d3832197`), Schnitt 2 `--dropped-roster` (258). **Schnitt 3 (dieses Atom):** `--dropped-roster [--public-only] [--baseline <datei>] [--count]` gebaut (`roster_diff`, Test `roster_diff_names_lost_and_new_keys`). **Emission-Fehler gefunden + geheilt:** `extract_open_points` zählte jede `## `-Überschrift mit Statuswort als Punkt — die `## Burn:`-Zeile wurde zur Riesenkette (215 Tokens), `## Offen (aufgeschlüsselt)` zu `aufgeschlüsselt`; Fix `CONTAINER_HEADS` (`is_container_heading` prüft das erste Wort: `offen`/`burn`/`abschluss`/`lock`/…), Test `dropped_roster_skips_burn_and_container_headings`. Baseline `docs/zustand/dropped-roster-baseline.txt` **public-only, emission-geheilt** am HEAD (37 öffentliche Schlüssel; `--public-only --baseline … --count` = **0**); vor dem Fix 41 (4 Container-/Burn-Scheinschlüssel).
- **Blockade:** **kein einschaltbares Tor.** Die public-only-Baseline löst die CI-Asymmetrie (private `future`-Handover), der Emission-Fix die Scheinschlüssel — aber die Menge ändert sich mit jeder legitimen Übergabe/lösung; ohne den `pending-legacy`-Einfrierschritt (Q4) und das getypte Ereignis-Vokabular (Q3) würde das Gate jede geschlossene Punktzeile als LOST rot färben. `commit_gate`/`ci-gate.yml` unberührt.
- **Braucht:** `pending-legacy`-Snapshot (voller dropped-Scan, CI-only, >30 min) einfrieren + getyptes Ereignis-Vokabular; dann `--dropped-roster --public-only --baseline docs/zustand/dropped-roster-baseline.txt --count` als `ci-gate`-Schritt verdrahten. Verdikt: `state/stimmen/2026-10-07_dropped-gate-design-ui-antworten.md` §Schnitt.

### Lizenz-Census Drift-Tor (Join auf Netloc-Ebene gebaut; Gate-Verdrahtung offen)
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 259) Generator `tools/register/src/bin/license_census.rs` join jetzt **netloc-/Release-Tag-genau**: `release_tag` zieht den Tag aus `…/releases/download/<tag>/…`, `parse_terms` parst block-weise (Blank-Zeilen), `drift` vergleicht `(netloc, class)` der Register-Blöcke gegen `(netloc, class)` des Census (`state/river/license-census.tsv`); Test `joins_register_netloc_against_census`; `cargo check`/`build` grün. Gemessen `--count` = **164** = **2 UNMEASURED** (Register-Netloc ohne Census-Zeile) + **162 STALE** (Census-Zeile ohne Register-`terms`-Deckung, dominiert von Census-`pending`).
- **Blockade:** 164 rot ist kein einschaltbares Tor — Ratchet/Baseline + CI-Verdrahtung fehlen; der 162-Anteil ist Census-`pending` vs. Register-`terms` (Riss Mountain↔River), nicht nur Generator-Drift.
- **Braucht:** Baseline-Zahl/-Roster (analog `dropped-baseline`) + `ci-gate`-Schritt; den 162-Stale-Anteil als Riss an Mountain/River adressieren.

## An river

Origin: mycelium-folge258.

- **NUR-Asset `fmi_image_mag_nur.bin` — sha steht.** `image-cdn.yml`-Lauf `37595578337` @`84dc4de08` success; Asset 15 551 948 B, sha `9c76f881d33e5e2d0c84b60b1641d4a7262f0039e43d3795a0dd8e9044714e0e` — in `phi/sources.φ:18033` (Block `format fmi_image_mag`) eingetragen. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.
- **Lizenz-Census-Tor — Census-Heimat entscheidet die CI-Verdrahtung.** Der Generator `license_census.rs` (mein Pen) joint jetzt netloc-genau gegen deinen `state/river/license-census.tsv` (gemessen 164 = 2 UNMEASURED + 162 STALE, meist Census-`pending`). Aber `state/` ist gitignored → `ci-gate` kann den Census **nicht** lesen; ein CI-Tor ist so nicht baubar. **Braucht:** dein Wort zur Census-Heimat — tracked (z. B. `docs/…`/`phi/…`) vs. lokaler Pregate; danach verdrahte ich Ratchet + Tor (und adressiere den 162-Stale-Riss Census↔`terms`).

## An future

Origin: mycelium-folge255.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
