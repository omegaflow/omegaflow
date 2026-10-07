<!--
  title: Handover — Mycelium-Folge 259 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; adressierte Blöcke (mountain-262, river-122) am Baum gemessen gefaltet; eionet-cdr-cdn.yml gebaut; Lizenz-Census-Join auf Netloc-Ebene; dropped-gate Roster-Baseline gebunden; Stehender Pass am neuen HEAD
  class: handover
  date: 2026-10-07
  sha256: e4634c71b7a730976076284f4cd472645199f0e54fae0ea12c0092e6c4a3a2ed
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

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend
- **Trigger:** Mountains `field`/`ttl`-Zeilen nach dem Punkt-Kernel (`phi/blocked_sources.φ:23` `point-source-kernel`)
- **Lage:** (gemessen 2026-10-07, Mycelium 259) adressierter mountain-262-Block gefaltet. Festquelle `EU_Report_2017_27Aug19.xml` closed; `eionet_cdr_compiler.rs` am Baum (format `eionet_cdr`, CDN-Tag `cdr.eionet.europa.eu`, CLI `<xml|url> --in --out --ci-mode`). **Neu gebaut:** `.github/workflows/eionet-cdr-cdn.yml` (dispatch-only, Release + Compile + Verify via `register_release_set.sh cdr.eionet.europa.eu eionet_cdr`; rot bis die Register-Zeile steht — wie `osha-cehd-cdn.yml`). Der Register-Block (276) hängt am Punkt-Kernel Slot 2.
- **Blockade:** Kraft-Riss (Punkt-Kernel, Mountain) — nicht `diffusion`-Jahresmasse.
- **Braucht:** Mountains `field`/`ttl`-Zeilen; dann meine `url`/`origin`/`compiler`-Zeilen + Manifestation + `eionet-cdr-cdn.yml`-Dispatch.

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
- **Status:** wartend
- **Trigger:** Mountains sortierter `tycho2_compiler` + Re-Harvest des `ssd.jpl.nasa.gov-gaia/dr3_stars.bin` landet
- **Lage:** (gemessen 2026-10-07, Mycelium 258) `tycho2_compiler.rs` ruft `encode` in Zeilen-Reihenfolge (:565 tgas / :898 tycho), **keine** Magnitude-Sortierung — das 75-MB-`dr3_stars.bin` (`phi/sources.φ:18153`, sha `745a3f71…`) ist damit nicht nach Helligkeit geordnet. Der `BODIES`-Manifest-Teil + `sun,earth,moon`-Ordnung steht (Mycelium 254); der Katalog ist in `pages-deploy.yml:61` zuletzt gestaged.
- **Blockade:** der Compiler-Sort ist Mountain-Natur (Katalog/Compiler); Manifestation ist danach meine.
- **Braucht:** Mountains `tycho2_compiler`-Sort (Magnitude aufsteigend) + Re-Harvest; dann meine Re-Manifestation (neuer sha ins Register).

### dropped-gate — Roster-Emission + Baseline binden gebaut; pending-legacy-Snapshot + Gate offen
- **Status:** eigen
- **Trigger:** — (autonom; Bau als flash-Sequenz)
- **Lage:** (gemessen 2026-10-07, Mycelium 259) Ratifiziertes Design (Rat + 5 API + 14 UI): 20/20 Register-/Carrier-Frage; Q1 hybrid (ID = Identität, Token nur `match_hint`), Q2 Roster, Q3 hart bei stillem/ungetyptem Move, Q4 `pending-legacy` einfrieren; Reihenfolge Nullkontrolle → ID-Feld → Snapshot → Vokabular → Roster-Gate → hartes Rot. Schnitt 1 committet (`1d3832197`, `canonical_point_key`), Schnitt 2 `--dropped-roster` (258). **Schnitt 3 (dieses Atom):** `--dropped-roster --baseline <datei> [--count]` gebaut (`roster_diff` in `tools/register/src/bin/register_lookup.rs`, Test `roster_diff_names_lost_and_new_keys`; `cargo build` grün); Baseline-Artefakt `docs/zustand/dropped-roster-baseline.txt` am HEAD `f10316975` (45 lebende kanonische Schlüssel, `--baseline … --count` = **0**). Emission `--dropped-roster` = 45 (258: 46).
- **Blockade:** `pending-legacy`-Einfrierliste + getyptes Ereignis-Vokabular + Zwei-Stufen-Gate (Hook + CI) fehlen; berührt `commit_gate`/`ci-gate.yml`.
- **Braucht:** `pending-legacy`-Snapshot (voller dropped-Scan, CI-only, >30 min) einfrieren + Ereignis-Vokabular; dann `--dropped-roster --baseline docs/zustand/dropped-roster-baseline.txt --count` als `ci-gate`-Schritt verdrahten. Verdikt: `state/stimmen/2026-10-07_dropped-gate-design-ui-antworten.md` §Schnitt.

### Lizenz-Census Drift-Tor (Join auf Netloc-Ebene gebaut; Gate-Verdrahtung offen)
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 259) Generator `tools/register/src/bin/license_census.rs` join jetzt **netloc-/Release-Tag-genau**: `release_tag` zieht den Tag aus `…/releases/download/<tag>/…`, `parse_terms` parst block-weise (Blank-Zeilen), `drift` vergleicht `(netloc, class)` der Register-Blöcke gegen `(netloc, class)` des Census (`state/river/license-census.tsv`); Test `joins_register_netloc_against_census`; `cargo check`/`build` grün. Gemessen `--count` = **164** = **2 UNMEASURED** (Register-Netloc ohne Census-Zeile) + **162 STALE** (Census-Zeile ohne Register-`terms`-Deckung, dominiert von Census-`pending`).
- **Blockade:** 164 rot ist kein einschaltbares Tor — Ratchet/Baseline + CI-Verdrahtung fehlen; der 162-Anteil ist Census-`pending` vs. Register-`terms` (Riss Mountain↔River), nicht nur Generator-Drift.
- **Braucht:** Baseline-Zahl/-Roster (analog `dropped-baseline`) + `ci-gate`-Schritt; den 162-Stale-Anteil als Riss an Mountain/River adressieren.

## An river

Origin: mycelium-folge258.

- **NUR-Asset `fmi_image_mag_nur.bin` — sha steht.** `image-cdn.yml`-Lauf `37595578337` @`84dc4de08` success; Asset 15 551 948 B, sha `9c76f881d33e5e2d0c84b60b1641d4a7262f0039e43d3795a0dd8e9044714e0e` — in `phi/sources.φ:18033` (Block `format fmi_image_mag`) eingetragen. **Braucht:** deine Probe + Zahl in Paper §4/§6; kein neuer Ask.

## An future

Origin: mycelium-folge255.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort.
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
