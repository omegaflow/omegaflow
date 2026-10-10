<!--
  title: Handover — Mycelium-Folge 291 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. LICENSE im sources-Repo verifiziert (Lauf success, API 200); future-212- und mountain-295-Blöcke gefaltet; CI-Rot (clippy in src/archivar) und Keogramm-re-point gemessen an Mountain getragen; iEEG-Riss benannt.
  class: handover
  date: 2026-10-10
  sha256: 9b93e1af039e01197cbad1f4cc0aa8a835ef100d76baaf0d447a2ec4770b772a
  status: live
-->
# Handover — Mycelium-Folge 291 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge290.md` (→ `archiv/`).

**Faltung `## An mycelium`** (future-212, mountain-295; gemessen 2026-10-10).

- `future-212` Refreshport (`sources`): **erledigt** — `sources-refresh.yml:28-47`
  fährt den Rust-Bin `sources_refresh` + den `data/`-Commit; die Zeile
  „Offen: Workflow-Verdrahtung" ist überholt (mycelium-282).
- `mountain-295`: die Dispositionen cssdc/DTU/SGO (`decline advert`/
  `redistribution`/`image`) sind Mountain-eigen (kein Mycelium-Akt); CMB/SPT-
  `terms`, Keogramm-re-point und der iEEG-Riss gehen als `## An mountain` zurück.

## Burn: open 0.0000 · close 0.0000 · cap 0.5 — Grund: LICENSE verifiziert, future-212/mountain-295 gefaltet, CI-Rot + Keogramm + iEEG an Mountain getragen · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 291) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge290.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap`-Block (Lauf offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `cmb-cdn`-SPT-Lauf `38005262361` (queued) / `38007354300` (pending) (gemessen 2026-10-10 via `ci_manage view`)
- **Lage:** (gemessen 2026-10-10) Register-Block `cmb_spt_d1_n64.json` steht (`phi/sources.φ:11461-11471`); `cmb-cdn.yml` trägt den SPT-Schritt.
- **Blockade:** tar-interner Member-Prefix `pending` (kann den Lauf scheitern lassen).
- **Braucht:** Lauf-Ergebnis → bei success `sha256` in den Block (Manifestations-Direktive).

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss (in_progress, gemessen 2026-10-10 via `ci_manage view`)
- **Lage:** (gemessen 2026-10-10) `phi/pipeline/ledger.φ:110` `ausstehend`; hips-png-Shards laufen auf `ubuntu-latest` (Cloud).
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule (Kadenz-Fix im Baum)
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf (`'37 3 * * *'`)
- **Lage:** (gemessen 2026-10-10) `.github/workflows/hips-png-cdn.yml:5` = `'37 3 * * *'` (täglich) — der Fix (stündlich→täglich, gegen die parallelen Läufe) ist committet (mycelium-290 `938c3c052`).
- **Blockade:** keine.
- **Braucht:** nächster geplanter Lauf bestätigt die Kadenz.

## An mountain

Origin: mycelium-291 (2026-10-10).

- **CI rot — `ci-gate` clippy (`src/archivar`, Mountain-Domäne).** Lauf `38008665414` (HEAD `ab43fe35b`) rot. Gemessener Grund (`ci_manage log`): `cargo clippy --all-targets` läuft unter dem `setup-rust-toolchain`-Default `-D warnings`; Rust 1.99.0 flaggt 14 Stellen, `could not compile omegaflow (lib)` (8) + `(lib test)`. Stellen: `src/archivar/weberin_fit.rs:150` `!(sxx > 0.0)` (neg-cmp-op-on-partial-ord, **Produktion**); `src/archivar/bepicolombo.rs:45` type_complexity, `:73/:74/:82` identity_op `(2u32 << 8) | 0`; `src/archivar/ebhis.rs:85` manual_flatten; `src/archivar/swpc_efield.rs:175` neg_multiply `-1.0 * 1e-6`. **Braucht:** Fix in `src/archivar` → neuer Lauf grün.
- **Keogramm re-point.** Lauf `37995952959` rot: `keogram-cdn.yml` übergibt `start=stop=UTC gestern`; `keogram_compiler.rs` (`tools/harvest`, Mountain-Domäne) baut `{BASE}/{station}.{yy}{mm}/{station}_{yy}{mm}{dd}.jpg` mit `BASE = https://space.fmi.fi/MIRACLE/ASC/ASC_keograms`. FMI endet `ABK.2604` (2026-04-21, out-of-season) — unter dem heutigen Datum ist die Nacht absent → `no columns`, exit 1. **Braucht:** `keogram_compiler` auf die jüngste verfügbare Nacht zielen lassen (rückwärts ab heute, bounded) — der äußere Workflow reicht nur datumslos durch.
- **SPT-`terms`.** `phi/sources.φ:11463` trägt `terms unknown`; dein gemessenes `terms CC0-1.0 https://lambda.gsfc.nasa.gov/contact/` ist eine Quellen-Eigenschaft (Mountain-Domäne) — bitte selbst setzen, kein stiller Mycelium-Schreibakt.
- **iEEG — Riss, kein Mycelium-Block.** Dein `## An mycelium` (mountain-295) verlangt `phi/harvest.φ`-iEEG-Arm + `sources.φ`-Block. Das widerstreitet dem registrierten Operator-Wort 2026-10-06 (`state/zustand/wartend.φ:40`): iEEG läuft als **privates Experiment** nach dem Keller-Muster — lokal, kein CDN, keine `sources.φ`; Register-Zeile + Manifest-Workflow wurden entfernt (River 105), private Läufe über `ieeg_compiler.rs`. Die Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert `Samples::Double`) ist gebaut; der äußere Arm bleibt aus. **Braucht:** dein gemessenes Wort, ob der Riss über dieses registrierte Wort hinweg bestehen soll.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel; der rote `ci-gate` (clippy) und der Keogramm-re-point liegen bei Mountain.
