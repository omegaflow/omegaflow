<!--
  title: Handover — Mycelium-Folge 257 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; adressierte Blöcke (mountain-260, river-122) am Baum gemessen gefaltet; `image-cdn.yml` für das NUR-Asset `fmi_image_mag_nur.bin` gebaut; dropped-gate-Schnitt 1 als committet gemessen; Stehender Pass am neuen HEAD
  class: handover
  date: 2026-10-07
  sha256: ed7198c7b042ee7ce4e451de74d0585c97fd8ca1b98215c01ac93cf5292bd9ff
  status: live
-->
# Handover — Mycelium-Folge 257 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge256.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An future`.

## Burn: open 0.0 · close 0.0 · cap 0.5 — Grund: Meta-Pass, kein pro/max-Dispatch

## Operator-Wort-Register

- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | Quelle: Operator (Session, Mycelium 257) — Session-Start, Delegations-Consent.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge256.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend
- **Trigger:** Mountains `force-undetermined`-Write + `field`/`ttl`-Zeilen (276 = 92×3 Media) → Register-Block steht
- **Lage:** (gemessen 2026-10-07, Mountain 258/260, Mycelium 256/257) Medium-Arm + Compiler stehen (`eionet_cdr_compiler.rs`, `FORMAT=eionet_cdr`, `CDN_TAG=cdr.eionet.europa.eu`); Festquelle + 92-Code-Codelist geschlossen; `phi/blocked_sources.φ:4` trägt das `force-undetermined`-Gap. Am Baum gemessen (2026-10-07, Mycelium 257): `phi/harvest.φ` ohne `eionet`-Pattern, `phi/sources.φ` ohne eionet-Block, `eionet-cdr-cdn.yml` absent — mein Transport hängt am Register-Block.
- **Blockade:** Kraft-Riss (`diffusion` fabriziert einen Jahresmasse-Gradienten).
- **Braucht:** Mountains `field`/`ttl`-Zeilen; dann meine `url`/`origin`/`compiler`-Zeilen + Pattern + `eionet-cdr-cdn.yml` + Manifestation.

### OSHA-CEHD — Register-Zeile (Force-/Einheiten-`1`-Riss)
- **Status:** wartend
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07, Mycelium 257) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-Kontrakt (Compiler emittiert Masse/Massenkonzentration/amount-fraction `1`); `obis.osha.gov` in `docs/specs/cdn-tag-baseline.txt` als gemessene Ausnahme.
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann fällt die Baseline-Ausnahme und die Manifestation läuft.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255) `LICENSE`/`README` dort absent (HTTP 404 raw).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen) sind noch nicht vollständig.
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### `fmi_image_mag_nur.bin` — NUR-Asset manifestieren (River braucht es für die Probe)
- **Status:** wartend
- **Trigger:** `image-cdn.yml`-Lauf landet (sha in der Release)
- **Lage:** (gemessen 2026-10-07, Mycelium 257) Register-Zeile `phi/sources.φ:18022-18028` (`format fmi_image_mag`, Compiler `image_mag_compiler.rs`) trägt **keinen** `sha256`; das Asset war nicht im CDN. Neu gebaut: `.github/workflows/image-cdn.yml` (self-hosted, `--start`/`--days`-Inputs, `image_mag_compiler --stations NUR --sample-rate 10 --ci-mode` → `space.fmi.fi/fmi_image_mag_nur.bin`), am HEAD noch nicht gepusht.
- **Blockade:** kein `sha256` bis der Lauf landet.
- **Braucht:** `image-cdn.yml` dispatchen + Lauf-Ergebnis lesen → `sha256` in `phi/sources.φ:18022`; dann River-Probe + Zahl in Paper §4/§6.

### dropped-gate — Träger statt Delta (Schnitt 1 committet; Schnitte 2/3 offen)
- **Status:** eigen
- **Trigger:** — (autonom; Bau als flash-Sequenz)
- **Lage:** (gemessen 2026-10-07) Design ratifiziert (Rat + 5 API + 14 UI, 20 Quellen): 20/20 Register-/Carrier-Frage; Q1 hybrid (ID = Identität, Token nur `match_hint`), Q2 Roster (Menge point-IDs), Q3 hartes Gate bei stillem/ungetyptem Move, Q4 `pending-legacy` einfrieren; Reihenfolge Nullkontrolle → ID-Feld → Snapshot → Vokabular → Roster-Gate → hartes Rot. **Schnitt 1 ist committet** (`1d3832197`, `tools/register/src/bin/register_lookup.rs`: `explicit_point_id` :2329 + `canonical_point_key` :2354 + `carries`-Closure; 4 Tests :4788/:4799/:4811/:4818; `cargo check` grün) — die folge256-Zeile „uncommittet" war stale. Der Nullkontrolltest (`canonical_point_key_absorbs_reformulation`) ist unter den 4 Tests.
- **Blockade:** Schnitt 2/3 brauchen den Format-/Slice-Schnitt: ID-Feld im Übergabe-Punktformat + Roster-Snapshot (`register_lookup --dropped` gegen die Menge statt den Skalar) + getyptes Ereignis-Vokabular + Zwei-Stufen-Gate (Hook + CI); berührt `commit_gate`/`ci-gate.yml`.
- **Braucht:** Bau Schnitt 2 (Roster-Baseline + `pending-legacy`-Snapshot) als ersten begrenzten Schritt; Verdikt-Spec in `state/stimmen/2026-10-07_dropped-gate-design-ui-antworten.md` §Schnitt. Modell-Vergleich: `state/benchmark/2026-10-07-dropped-gate-modellvergleich.md` (Sieger Claude Sonnet 5.5, Ausreißer GPT-OSS 120B).

### Sternkatalog nach Helligkeit ordnen (Membran progressives Laden (C))
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, River 122) der `BODIES`-Manifest-Teil ist gebaut (Mycelium 254); offen ist der 95-MB-Sternkatalog; Sichtbarkeits-Reihenfolge `sun, earth, moon, katalog` noch nicht hergestellt.
- **Blockade:** keine.
- **Braucht:** Katalog nach Helligkeit ordnen + manifestieren; Reihenfolge `sun, earth, moon, katalog`.

### Lizenz-Census-Generator + Drift-Tor
- **Status:** eigen
- **Trigger:** — (autonom)
- **Lage:** (gemessen 2026-10-07, River 122) `state/river/license-census.tsv` ist die Messquelle; der Generator aus Mountains `terms`-Zeilen ist noch nicht gebaut.
- **Blockade:** keine (Generator liest die bestehenden `terms`-Zeilen; kein Mountain-Vorlauf).
- **Braucht:** Generator bauen, der `terms` aus `phi/sources.φ` gegen `state/river/license-census.tsv` auswertet (Drift-Tor).

## An mountain

Origin: mycelium-folge256/257.

- **`eionet_cdr` — meine Transportzeilen warten auf deine Feder.** Dein Verdikt ist gefallen (`force-undetermined`-Gap, `phi/blocked_sources.φ:4`); es fehlt dein Register-Write: Block (`field`/`ttl`, 276) + `at earth`. **Braucht:** dein `force-undetermined`-Write + `field`/`ttl`.
- **`obis.osha.gov` — Register-Zeile wartet auf deinen `1`-Riss-Abschluss.** **Braucht:** deine `terms`/`url`-Zeile.
- **`LICENSE` im `omegaflow/sources`-Repo** — Mycelium erzeugt es erst nach deinen `terms`-Zeilen. **Braucht:** deine `terms`-Vollständigkeit.

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

**Diese Session (Atome):** (a) Adressierten `## An mycelium`-Block aus `mountain-260` am Baum gemessen: die CDN-Workflows der neuen Arme stehen — `jaxa-gportal-cdn.yml` (deckt `jaxa_gpm_ku`, `phi/sources.φ:2043`), `nasa-power-t2m-cdn.yml`, `epa-aqs-voc-cdn.yml`, `osm-pbf-cdn.yml` (Format `osm_nodes`, Pattern `^monaco_nodes\.bin$` = `phi/harvest.φ:283`), `fink-cutout-cdn.yml` (`phi/harvest.φ:95-99`); `vnp46a3-cdn.yml` trägt bereits den äquatorialen Default-Tile `h18v07` (DNB-Nachtdaten ganzjährig). Nur `eionet_cdr` bleibt absent (Kraft-Riss, wartend). (b) Adressierten `## An mycelium`-Block aus `river-122` gemessen: `fmi_image_mag_nur` war unmanisfestiert → **`.github/workflows/image-cdn.yml` gebaut** (self-hosted, `--ci-mode` → `space.fmi.fi`); Sternkatalog-Ordnung + Lizenz-Census als eigene Punkte aufgenommen. (c) dropped-gate: Schnitt 1 als **committet** (`1d3832197`) gemessen und die stale „uncommittet"-Zeile berichtigt; Schnitte 2/3 mit präzisem nächsten Schritt offen. (d) Handover fortgeschrieben, folge256 → `archiv/`; Stehender Pass am neuen HEAD (Schlussakt).
