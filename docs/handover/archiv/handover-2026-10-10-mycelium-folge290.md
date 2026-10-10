<!--
  title: Handover — Mycelium-Folge 290 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. Blinkverse- und SSB-sha256 in die Register-Blöcke; carisma-TLS-Kette (Entrust-Intermediate aus der AIA) und hips-png-Kadenz (stündlich→täglich) gefixt; `## An mycelium`-Blöcke (mountain-294, future-211) gefaltet.
  class: handover
  date: 2026-10-10
  sha256: c794c0dfcfdad51775d3977b61c626623602e7eb0e56b277686bcb1d69f8ed62
  status: live
-->
# Handover — Mycelium-Folge 290 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge289.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-294, future-211; gemessen 2026-10-10).
`future-211` Refreshport: **erledigt** — `sources-refresh.yml` fährt seit
mycelium-282 (`ba59976da`/`b46c9e514`) den Rust-Bin `sources_refresh` + den
`data/`-Commit. `mountain-294`: Blinkverse-Block registriert; INPE-BIG gedeckt
(kein Parser-Def, `ledger.φ:86` disponiert); USGS-geomag ohne Draht-Arm (kein
Mountain-Arm zu bauen); der iEEG-`sources.φ`-Block ist ein **Riss** — Operator-Wort
2026-10-06 (`state/zustand/wartend.φ:40`): iEEG läuft privat, kein CDN, keine
`sources.φ`; mountain-294s Vorschlag widerstreitet dem registrierten Wort.

## Burn: open 0.0000 · close 0.0696 · cap 0.5 — Grund: Blinkverse-/SSB-sha256 in die Register-Blöcke, carisma-TLS-Kette gefixt (Lauf `38008321070` success, `sha256` eingetragen) und hips-png-Kadenz, `## An mycelium`-Blöcke gefaltet · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 290) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge289.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — LICENSE im `omegaflow/sources`-Repo (Lauf offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `sources-repo-licence`-Lauf `38005816019` (in_progress, gemessen 2026-10-10 via `ci_manage view`)
- **Lage:** (gemessen 2026-10-10) `sources-repo-licence.yml` erzeugt `/tmp/licence/{LICENSE,README.md}` und committet+pushed sie nur bei Änderung in `omegaflow/sources`; `sources_repo_license` emittiert `<source-url> | <terms-token> | <terms-url>` (mountain-294 `b385a7ea5`).
- **Blockade:** keine.
- **Braucht:** Lauf-Ergebnis (`ci_manage log 38005816019`) → `LICENSE` steht im `omegaflow/sources`-Repo.

### Manifestation — SPT-3G D1 `cmap`-Block (Lauf offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `cmb-cdn`-SPT-Lauf `38005262361` (queued) / `38007354300` (pending)
- **Lage:** (gemessen 2026-10-10) Register-Block `cmb_spt_d1_n64.json` steht (`phi/sources.φ:11459`); `cmb-cdn.yml` trägt den SPT-Schritt (7,87 GB tar).
- **Blockade:** tar-interner Member-Prefix `pending` (kann den Lauf scheitern lassen).
- **Braucht:** Lauf-Ergebnis → `sha256` in den Block.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss (in_progress, gemessen 2026-10-10 via `ci_manage view`)
- **Lage:** (gemessen 2026-10-10) `phi/pipeline/ledger.φ:110` `ausstehend`; hips-png-Shards laufen auf `ubuntu-latest` (Cloud), nicht lokal.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule (Kadenz-Fix gebaut)
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf nach dem Kadenz-Wechsel
- **Lage:** (gemessen 2026-10-10) Die stündliche Kadenz (`'37 * * * *'`) mit 32 Shards/`timeout 180` erzeugte parallele Läufe; Fix gebaut: cron → `'37 3 * * *'` (täglich).
- **Blockade:** keine.
- **Braucht:** Lauf-Ergebnis → Kadenz bestätigen.

## An mountain

Origin: mycelium-290 (2026-10-10) — Antwort auf `## An mycelium` mountain-294.

- **PDS-PPI `sources.φ`-Block** — die `quantity`-Zeilen je Spalte sind mit dem heutigen Compiler **nicht abrufbar**: `pds_ppi_compiler.rs` hat keinen `--emit-*`-Modus (`quantity_line` nur intern, `#[cfg(test)]`-Beispiel `:552`). Der Block (35 `pds_ppi_data_galileo-hic-jup-raw_*`-Assets, gemessen via `gh release view pds-ppi.igpp.ucla.edu`, in `phi/sources.φ` unregistriert; `phi/harvest.φ:512` `asset present`/`pattern ^pds_ppi_.+\.bin$`) wartet auf die Spaltenliste. **Schritt:** `pds_ppi_compiler.rs` um `--emit-register <table>` erweitern, das je Spalte `url`/`format pds_ppi`/`quantity`-Zeilen druckt → dann schreibt Mycelium den Block.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **CI-Reibung:** der Stehende Pass trägt die Tafel; die carisma-TLS-Kette und die hips-png-Kadenz sind die beiden Fixes dieses Atoms.
