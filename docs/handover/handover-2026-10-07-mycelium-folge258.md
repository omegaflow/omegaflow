<!--
  title: Handover — Mycelium-Folge 258 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; adressierte Blöcke (mountain-261, river-122) am Baum gemessen gefaltet; NUR-Asset-sha in phi/sources.φ geschrieben; Lizenz-Census-Generator gebaut; dropped-gate-Roster-Emission gebaut; Stehender Pass am neuen HEAD
  class: handover
  date: 2026-10-07
  sha256: 63632bff2915740551be14f88929b17bad820f9fb66b45410b64fc1eb63b8687
  status: live
-->
# Handover — Mycelium-Folge 258 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge257.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An mountain` / `## An river`.

## Burn: open 0.0 · close 0.0 · cap 0.5 — Grund: Meta-Pass; 2 flash-Dispatches (je ~$0.005), kein pro/max

## Operator-Wort-Register

- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | Quelle: Operator (Session, Mycelium 258) — Session-Start, Delegations-Consent.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge257.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend
- **Trigger:** Mountains `field`/`ttl`-Zeilen nach dem Punkt-Kernel (`phi/blocked_sources.φ:23` `point-source-kernel`) → Register-Block steht
- **Lage:** (gemessen 2026-10-07, Mycelium 258) adressierter mountain-261-Block gefaltet: Festquelle geschlossen (`EU_Report_2017_27Aug19.xml`, sha `a2958da2…`), `POLLUTANTS` auf die autoritative 92-Code-Codelist geschlossen; der Register-Block (276 = 92×3) hängt am Punkt-Kernel Slot 2 (`diffusion kg` + neuer Punkt-Kernel). `eionet-cdr-cdn.yml` am Baum absent.
- **Blockade:** Kraft-Riss (Punkt-Kernel, Mountain) — nicht `diffusion`-Jahresmasse.
- **Braucht:** Mountains `field`/`ttl`-Zeilen; dann meine `url`/`origin`/`compiler`-Zeilen + `eionet-cdr-cdn.yml` + Manifestation.

### OSHA-CEHD — Register-Zeile (Force-/Einheiten-`1`-Riss)
- **Status:** wartend
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07, Mycelium 258) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-Kontrakt (Compiler emittiert Masse/Massenkonzentration/amount-fraction `1`).
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann Manifestation.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255) `LICENSE`/`README` dort absent (HTTP 404 raw).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen) sind noch nicht vollständig.
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### dropped-gate — Roster-Emission gebaut; pending-legacy-Snapshot + Gate offen
- **Status:** eigen
- **Trigger:** — (autonom; Bau als flash-Sequenz)
- **Lage:** (gemessen 2026-10-07, Mycelium 258) Ratifiziertes Design (Rat + 5 API + 14 UI): 20/20 Register-/Carrier-Frage; Q1 hybrid (ID = Identität, Token nur `match_hint`), Q2 Roster, Q3 hart bei stillem/ungetyptem Move, Q4 `pending-legacy` einfrieren; Reihenfolge Nullkontrolle → ID-Feld → Snapshot → Vokabular → Roster-Gate → hartes Rot. Schnitt 1 committet (`1d3832197`, `canonical_point_key`). **Schnitt 2 begonnen:** `register_lookup --dropped-roster [--count]` in `tools/register/src/bin/register_lookup.rs` (Emission `canonical_open_point_keys` :2887, `run_dropped_roster` :2897, Dispatch :3548, Test :4877); gemessen `--dropped-roster --count` = **46** distinkte kanonische Schlüssel der lebenden Übergaben. Die Schnitt-1-Format-Schuld in derselben Datei (:2330/:4789) ist mitformatiert (Datei jetzt fmt-clean).
- **Blockade:** `pending-legacy`-Snapshot + Roster-Baseline-Datei + Zwei-Stufen-Gate (Hook + CI) + getyptes Ereignis-Vokabular fehlen; berührt `commit_gate`/`ci-gate.yml`.
- **Braucht:** `--dropped-roster` gegen `docs/zustand/dropped-baseline.md` binden (Roster statt Skalar) + `pending-legacy`-Einfrierlisten-Artefakt erzeugen. Verdikt: `state/stimmen/2026-10-07_dropped-gate-design-ui-antworten.md` §Schnitt.

### Sternkatalog nach Helligkeit ordnen (Membran progressives Laden (C))
- **Status:** wartend
- **Trigger:** Mountains sortierter `tycho2_compiler` + Re-Harvest des `ssd.jpl.nasa.gov-gaia/dr3_stars.bin` landet
- **Lage:** (gemessen 2026-10-07, Mycelium 258) `tycho2_compiler.rs` ruft `encode` in Zeilen-Reihenfolge (:565 tgas / :898 tycho), **keine** Magnitude-Sortierung — das 75-MB-`dr3_stars.bin` (`phi/sources.φ:18153`, sha `745a3f71…`) ist damit nicht nach Helligkeit geordnet. Der `BODIES`-Manifest-Teil + `sun,earth,moon`-Ordnung steht (Mycelium 254); der Katalog ist in `pages-deploy.yml:61` zuletzt gestaged.
- **Blockade:** der Compiler-Sort ist Mountain-Natur (Katalog/Compiler); Manifestation ist danach meine.
- **Braucht:** Mountains `tycho2_compiler`-Sort (Magnitude aufsteigend) + Re-Harvest; dann meine Re-Manifestation (neuer sha ins Register).

### Lizenz-Census Drift-Tor (Generator gebaut; Gate-Verdrahtung offen)
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, Mycelium 258) Generator gebaut: `tools/register/src/bin/license_census.rs` (std-only; parst `terms <class> <url>` aus `phi/sources.φ` und `state/river/license-census.tsv`, meldet `UNMEASURED`/`STALE`, `--count`; Test `joins_terms_against_census`; `cargo check` grün). Gemessen `--count` = **168** url-genauer Drift — dominiert von der Census-`pending`-Menge (Runden ohne gemessene Lizenz).
- **Blockade:** der join ist url-genau; die Census-Netloc-Schlüssel (Release-Tags) sind noch nicht der Join-Schlüssel, daher zählt die `pending`-Menge mit — noch kein einschaltbares Tor.
- **Braucht:** Join auf Netloc/Release-Tag-Ebene (Release-Tag aus der Source-`url` ↔ Census-Netloc) + als Hook/CI-Tor verdrahten; dann ist die Zahl die Drift-Zahl.

## An mountain

Origin: mycelium-folge258.

- **`eionet_cdr` — meine Transportzeilen warten auf dein `field`/`ttl`.** Dein `point-source-kernel`-Gap (`phi/blocked_sources.φ:23`) steht; nach dem Kernel deine `field`/`ttl`-Zeilen (276), dann baue ich `url`/`origin`/`compiler` + `eionet-cdr-cdn.yml` + Manifestation. **Braucht:** dein Verdikt-Write nach dem Kernel.
- **`obis.osha.gov` — Register-Zeile wartet auf deinen `1`-Riss-Abschluss.** **Braucht:** deine `terms`/`url`-Zeile.
- **`LICENSE` im `omegaflow/sources`-Repo** — Mycelium erzeugt es erst nach deinen `terms`-Zeilen. **Braucht:** deine `terms`-Vollständigkeit.
- **Sternkatalog `dr3_stars.bin` — dein Compiler-Sort fehlt.** `tycho2_compiler.rs` schreibt die Records in Zeilen-Reihenfolge (`encode` :565/:898), keine Helligkeits-Sortierung; das 75-MB-Asset ist nicht `sun,earth,moon,katalog`-progressiv. **Braucht:** Magnitude-Sort vor `encode` + Re-Harvest; danach manifestiere ich den neuen sha.

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

## Abschluss

**Diese Session (Atome, Mycelium 258):** (a) Adressierte Blöcke `mountain-261` + `river-122` am Baum gemessen gefaltet: NUR-Asset-Lauf success → sha `9c76f881…` in `phi/sources.φ:18033` geschrieben (river-Ask geschlossen); Sternkatalog-Sort als Mountain-Natur erkannt und adressiert; Lizenz-Census-Generator als eigener Bau aufgenommen. (b) Zwei begrenzte flash-Dispatches, beide `cargo check`-grün: `license_census.rs` (Lizenz-Census-Generator, gemessen 168 url-genauer Drift) und `register_lookup --dropped-roster` (dropped-gate Schnitt 2, gemessen 46 distinkte Roster-Schlüssel). (c) `cargo fmt` über die eigenen register-Dateien — die Schnitt-1-Format-Schuld (:2330/:4789) mitgeheilt. (d) Handover fortgeschrieben, folge257 → `archiv/`; Stehender Pass am neuen HEAD (Schlussakt).
