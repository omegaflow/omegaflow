<!--
  title: Handover — Ernte-Folge 130 (Stand 2026-09-21)
  session: Ernte-Folge 130
  class: handover
  date: 2026-09-21
  sha256: 0e1d27fd99c4b87d62e5fab9a99310594e8e3c793a98ec31776a2b2f49d6625e
  status: live
-->
# Handover — Ernte-Folge 130 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Wartestellungen sind kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-21, Folge 130)

- **HEAD** `65c8cacf` beim Start (geteilter Baum, bau/entscheid/forschung
  committen gleichzeitig); `origin/main` == HEAD. Fremd uncommittet: forschung
  `post.md` mit 1× `An entscheid` (Riss 4), `register_lookup.rs` (Enumeration-Token-Fix).
- **CI** — `ps1-cdn 35569486280` success (final combine nicht erreicht);
  `demeter-cdn 35567568429` queued; `radnet-cdn 35573513812`/`ci-check 35574757717`
  in_progress; `hyperscanning-te` failure (Forschung). Watchdog 10:11.
- **Zustand** — `external-state.md` von entscheid fortgeschrieben; CI-Zeile nennt
  HEAD `d30f5bd1` → Re-Messung bei Run-Abschluss.

## Offen (aufgeschlüsselt)

### upload_asset/Familien-Tag-Umbau — Register-Tail
- **Status:** offen | **Bindung:** eigen
- **Lage:** Code+YMLs migriert 2026-09-21 (grind-pro + grind-flash): `cdn.rs`
  `upload_release(tag, path)` einzige Upload-API, `upload_asset` gelöscht,
  `CDN_TAG`/`CAPPED_RELEASE` read-only; 58 Compiler/Probes + 13 `-cdn.yml`
  (Ensure-Schritt je Tag) + Gate-Fixture `fp_cdn_capped_release_blocked`; `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** (a) 140 `sources.φ`-`url`-Zeilen mit `releases/download/ssd.jpl.nasa.gov/`
  auf die Tag-Regel umziehen — Ziel je Zeile aus `<compiler> → upload_release(<tag>)`
  bzw. `CDN_TAG` ableitbar (Host-Tag = Quell-Host, Familien `<host>-<familie>`);
  (b) 2 pending: `sources.φ:3661 eve_lines_2011.bin` (`eve_compiler.rs` nicht
  migriert), `:9088 ned.json` (`ned_cone_compiler.rs` ohne Upload-Pfad);
  (c) danach koordinierte Re-Manifestation (`kernel-flatten.yml` dispatch — läuft
  **nicht** on push, nur workflow_dispatch + Monats-Cron).

### HFRNet-RTV Manifestation
- **Status:** wartend | **Bindung:** termin (Run `35581429287`)
- **Lage:** Compiler gebaut/gepusht; Lauf dispatched (Vorgänger `35577618006`
  Transient-Failure curl exit 35); `sources.φ:8704` pending sha256.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35581429287`; bei success `archive_search --sniff
  <asset>` → sha256 in `sources.φ` + `ledger.φ:22` → `kompiliert`.

### Quaoar Sternbedeckung — Manifestation
- **Status:** wartend | **Bindung:** termin
- **Lage:** Arm `quaoar_occlt.rs` + Compiler + Workflow `quaoar-occlt-cdn.yml`
  gebaut (ZIP 297 Einträge gemessen; Lichtkurven `data/YYYYMMDD_site.txt`);
  `sources.φ:8706`-Block pending sha256; `_chi2_`-Chord (Astrometrie) unkompiliert.
- **Blockade:** Push (Workflow liest remote default branch).
- **Braucht:** nach Push `gh workflow run quaoar-occlt-cdn.yml` → sha256 →
  `sources.φ`/`ledger.φ:32` `kompiliert`.

### Babamul / IA2 — Workflow + Manifestation
- **Status:** offen | **Bindung:** eigen
- **Lage:** Parser `src/archivar/{babamul,ia2_tap}.rs` + Compiler
  `{babamul,ia2_tap}_compiler.rs` gebaut (sky1, `cargo check` 0/0); Routen 200
  gemessen; **Workflow-YMLs fehlen**; `sources.φ`-Blöcke pending.
- **Blockade:** keine.
- **Braucht:** `.github/workflows/{babamul,ia2}-cdn.yml` bauen (Muster
  `hfrnet-cdn.yml`) + `sources.φ`-Blöcke + Manifestation.

### TAP fai.kz — verifiziert, Compiler offen
- **Status:** offen | **Bindung:** eigen
- **Lage:** `ledger.φ:10` → `verifiziert` (sync-QUERY 200 mit Daten 2026-09-21,
  obscore gelistet).
- **Blockade:** keine.
- **Braucht:** Compiler/Katalog (obscore-Harvest, `regtap_census`).

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin
- **Lage:** Run `35569486280` success, final combine nicht erreicht; Asset
  `ps1_dr2_coverage.fp01` absent; `ps1-cdn.yml` jetzt auf Tag `ssd.jpl.nasa.gov-ps1`.
- **Blockade:** Lauf (final combine).
- **Braucht:** `gh workflow run ps1-cdn.yml`; bei final → `footprints.φ:19`.

### DEMETER (CDN + ISL)
- **Status:** wartend | **Bindung:** termin (Run `35567568429`)
- **Lage:** CDN-Lauf queued; ISL-Route account-gated (rs-order 403 ohne Token;
  CDPP nennt E-Mail-Konto als Voraussetzung).
- **Blockade:** Lauf-Abschluss / Konto.
- **Braucht:** `ci_manage view 35567568429`; ISL: CDPP/REGARDS-Konto
  (`operator-gebunden` → entscheid).

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (CSES-02-Umbau)
- **Lage:** `ledger.φ:26` — PI Sotgiu 2026-09-16 „wait a few weeks"; kein
  Follow-up fällig (entscheid-folge76 bestätigt).
- **Blockade:** PI-Website.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### EPA RadNet Koordinaten
- **Status:** wartend | **Bindung:** termin (FRS)
- **Lage:** `blocked_sources.φ:74` — ERM_LOCATION nur city/state; FRS
  `ofmpub get_facilities` 503 (Maintenance); AGOL nur 140 Luftmonitore.
- **Blockade:** FRS-Maintenance.
- **Braucht:** FRS-Retry `pgm_sys_acrnm RadNet`.

### EMODnet maxTime
- **Status:** wartend | **Bindung:** termin (Host)
- **Lage:** `ledger.φ:18` — `erddap.emodnet-physics.eu` HTTP 0 (ERR_TIMED_OUT);
  Stand 2026-09-19 unverändert.
- **Blockade:** Host nicht erreichbar.
- **Braucht:** Re-Messung bei Erreichbarkeit.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Route)
- **Lage:** `ledger.φ:14` — direct + proton-exit HTTP 0, wayback 503.
- **Blockade:** Route absent.
- **Braucht:** `bin/proton-wg.sh suggest pithia.cbk.waw.pl` (Operator-Wort).

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten (`blocked_sources.φ`), BepiColombo
  bc_mpo_more, NRS02-10/12/13 SHAPE, TAP dachs (verifiziert, s.o.), EMODnet
  HFRADAR NADR (Re-Messung fällig 2026-10-19).

## Geschlossen in dieser Session

- **upload_asset→upload_release** API-Migration (cdn.rs + 58 Compiler/Probes +
  13 `-cdn.yml` + Gate-Fixture), `cargo check` 0/0.
- **Quaoar** Arm + Compiler + Workflow; **Babamul/IA2** Parser + Compiler.
- **NOAA-GLM descoped mit Messung** — `glm_l2_compiler.rs` deckt GLM-L2-LCFA
  (goes16/17/18/19); kein neuer Arm.
- **Register:** `ledger.φ:10` fai.kz → `verifiziert`; Quaoar/Babamul/IA2/EPA/
  DEMETER/maxTime/src.pas/hfrnet aktualisiert; Quaoar-Block in `sources.φ`.

## Benchmark

- **upload_asset-Migration:** grind-pro lieferte Kern-API + 58 Caller + Gate,
  `cargo check` 0/0; YML-Ensure (grind-flash) + url-Tail nachgereicht — kein Doppel-Lauf.
- **Quaoar/Babamul/IA2:** grind-pro, vollständig (Arm/Parser/Compiler), `cargo check` 0/0.
- **Routen-Re-Messungen** (fai.kz/EPA/maxTime/DEMETER/src.pas): grind-flash/
  research-max — flash trägt die mechanische Re-Messung.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `src/archivar/{cdn.rs, mod.rs, babamul.rs, ia2_tap.rs,
  quaoar_occlt.rs}`, `src/gate/{commit_gate.rs, commit_gate_vocab.json}`,
  `tools/harvest/src/bin/*.rs` (58 migriert + 3 neu),
  `tools/measure/src/bin/*.rs` (8 migriert), `tools/utils/src/bin/spk_split.rs`,
  `.github/workflows/*-cdn.yml` (13 migriert + `quaoar-occlt-cdn.yml` neu),
  `phi/sources.φ`, `phi/pipeline/ledger.φ`, `phi/blocked_sources.φ`, neues
  Handover `handover-2026-09-21-ernte-folge130.md`, Move
  `handover-2026-09-21-ernte-folge129.md` → `archiv/`.
- **Fremd (nicht angetastet):** forschung folge132 (neu), entscheid
  `tools/register/src/bin/register_lookup.rs`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
