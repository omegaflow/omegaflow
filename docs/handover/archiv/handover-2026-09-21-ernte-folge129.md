<!--
  title: Handover — Ernte-Folge 129 (Stand 2026-09-21)
  session: Ernte-Folge 129
  class: handover
  date: 2026-09-21
  sha256: 40929efc0eba102c05ae49d0242c8a539a9a3f7ae4afc11dcabf024db97f73c8
  status: live
-->
# Handover — Ernte-Folge 129 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Wartestellungen sind kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-21, Folge 129)

- **HEAD** `33c124d8` beim Start (geteilter Baum, bau/entscheid/forschung
  committen gleichzeitig); eigener Commit folgt. Arbeitsbaum: eigener Pfad-Satz
  (unten); fremde Arbeit (bau glm_l2, forschung te.rs/hyperscanning) uncommittet.
- **CI** — `ps1-cdn` `35569486280` in_progress (updated 06:40Z); `demeter-cdn`
  `35567568429` queued; `hyperscanning-te` failure (Forschung). Watchdog 09:07.
- **Zustand** — `docs/zustand/external-state.md` von entscheid-folge76
  fortgeschrieben; CI-Zeile nennt HEAD `d30f5bd1` (entscheid folge74), aktueller
  HEAD `33c124d8` → Re-Messung bei Run-Abschluss.

## Offen (aufgeschlüsselt)

### Generischer `upload_asset`/Familien-Tag-Umbau
- **Status:** offen | **Bindung:** eigen
- **Lage:** Cap 1000/Release auf `ssd.jpl.nasa.gov` (id 367063539) erreicht;
  `src/archivar/cdn.rs:40` blockiert. Rat entschieden (2026-09-21): `upload_release(tag, path)`
  wird die einzige Upload-API, `upload_asset` (taglos) gelöscht; Tag = Quell-Host
  (`const CDN_TAG`), Familien auf gedeckeltem Host `"<host>-<familie>"`;
  `CAPPED_RELEASE` read-only mit benannter Verweigerung; ein Atom, ~71 Caller +
  13 YMLs, per Datei parallel; Gate-Fixture `fp_cdn_capped_release_blocked`.
- **Blockade:** keine.
- **Braucht:** `grind-pro` (Migration familieweise) + Gate-Fixture im selben Atom.

### HFRNet-RTV-Gitter-Compiler — gebaut, Manifestation offen
- **Status:** offen | **Bindung:** eigen
- **Lage:** Code gebaut 2026-09-21: `src/archivar/hfrnet_rtv.rs`
  (ERDDAP-griddap-CSV-Arm, geo HFR1, 4 Gates), `tools/harvest/src/bin/hfrnet_compiler.rs`,
  `.github/workflows/hfrnet-cdn.yml`; `cargo check` 0/0. `phi/sources.φ`-Block
  pending sha256. Kraft `advective` m/s (Riss: Auftrag nannte em, Canon ist
  advective — A=A).
- **Blockade:** Push (Workflow liest remote default branch).
- **Braucht:** nach Push `gh workflow run hfrnet-cdn.yml`; dann sha256 in
  `sources.φ` + `ledger.φ` → `kompiliert`.

### Quaoar Sternbedeckungs-Astrometrie
- **Status:** offen | **Bindung:** eigen
- **Lage:** `zenodo.21185812` Route 200; `Quaoar_paper.zip` 572467032 B, md5
  `420a1e94…`, sha256 `1473129f…` (gestreamt 2026-09-16 folge52, 2026-09-21
  bestätigt); als `ausstehend kandidat` in `ledger.φ` registriert.
- **Blockade:** keine.
- **Braucht:** Compiler (Sternbedeckungs-Astrometrie-Parser) + CDN (`grind-pro`).

### Babamul / IA2 TAP — Bau-Reihenfolge
- **Status:** offen | **Bindung:** eigen
- **Lage:** Notizen korrigiert (`blocked_sources.φ:3,35`): „kein Konsument" ist
  Bau-Reihenfolge, kein Quellen-Verdikt; Routen mit Token/anonym 200.
- **Blockade:** keine.
- **Braucht:** Babamul-Alert-Parser + IA2-Consumer bauen (`grind-pro`).

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin (Run `35569486280`)
- **Lage:** `ps1_dr2_coverage.fp01` absent; Lauf in_progress.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35569486280`; bei Final-Combine → `footprints.φ:19`.

### DEMETER
- **Status:** wartend | **Bindung:** termin (Run `35567568429`)
- **Lage:** Re-Dispatch `demeter-cdn.yml` queued.
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35567568429`; bei success 77 `url`+`sha256`-Zeilen.

### SSDC Limadou — korrigiert
- **Status:** wartend | **Bindung:** termin (CSES-02-Umbau)
- **Lage:** `ledger.φ:26` korrigiert — PI Sotgiu 2026-09-16: „wait a few weeks"
  (CSES-02-Rework); kein Follow-up fällig (entscheid-folge76 bestätigt).
- **Blockade:** PI-Website.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### Wartend (kein Auswahlpunkt)
- TAP-Backends dachs/pithia (`ledger.φ`), Lasair-LSST (502), Sonden-Antworten
  (`blocked_sources.φ`), BepiColombo bc_mpo_more, NRS02-10/12/13 SHAPE,
  EMODnet HFRADAR NADR (Re-Messung 2026-10-19).

## Geschlossen in dieser Session

- **Rat-Entscheid** `upload_asset`/Familien-Tag (Council): `upload_release(tag,path)`,
  `CAPPED_RELEASE` read-only, ein Atom, Gate-Fixture.
- **HFRNet-Compiler gebaut** (grind-flash, `cargo check` 0/0).
- **Register-Korrekturen:** SSDC (`ledger.φ:26`), Quaoar (`ledger.φ` neu),
  Babamul/IA2 (`blocked_sources.φ:3,35`).
- **Quaoar sha256** hart gemessen und gegen folge52 bestätigt: `1473129f…`.

## Benchmark

- **HFRNet-Compiler** (novel ERDDAP-griddap-Parser): `grind-flash` lieferte
  vollständig (Arm + Compiler + Workflow, `cargo check` 0/0) für ~$0.10 — kein
  pro/max nötig; flash-Sieger, Klasse notiert. Kein Doppel-Lauf (flash korrekt).
- **Rat** upload_asset-Architektur: pro/max, Entscheid geliefert.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `src/archivar/mod.rs` (`pub mod hfrnet_rtv`),
  `src/archivar/hfrnet_rtv.rs` (neu), `tools/harvest/src/bin/hfrnet_compiler.rs`
  (neu), `.github/workflows/hfrnet-cdn.yml` (neu), `phi/sources.φ` (nur
  hfrnet-Block), `phi/pipeline/ledger.φ`, `phi/blocked_sources.φ`, neues Handover
  `handover-2026-09-21-ernte-folge129.md`, Move
  `handover-2026-09-21-ernte-folge128.md` → `archiv/`.
- **Vom geteilten Index mitgerissen (gemessen):** bau folge120 `0475d67f`
  committete die HFRNet-Hunks in `src/archivar/{geo,extract,main_flow}.rs`
  (MAGIC_HFR/COMP_HFR + dispatch) unter seiner glm_l2-Nachricht — der geteilte
  Index trug meine gestagten Hunks. Dieser Commit schließt die Lücke (`mod.rs` +
  `hfrnet_rtv.rs`), damit HEAD compiliert. Lehre: bei geteilten Dateien erst nach
  fremdem Commit stagen.
- **Fremd:** bau glm_l2 (`0475d67f`), forschung folge129 (`20d675af`/`fb363dda`),
  `docs/handover/post.md` (unstaged).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt, hunk-genau bei
  geteilten Dateien.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
