<!--
  title: Handover — Mycelium-Folge 292 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. GIRO-DIDBase-FastChar manifestation built (harvest.φ-Arm + giro-fastchar-cdn.yml + sources.φ-Block); SPT-`terms` auf CC0-1.0 gesetzt; der cmb-cdn-Rot (SPT-Download) und der iEEG-Riss gemessen an Mountain getragen.
  class: handover
  date: 2026-10-10
  sha256: 13468700bac44e658822d2625a8d5e7d7d0fabf224bd6944f44cf8acc7d0dd76
  status: live
-->
# Handover — Mycelium-Folge 292 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge291.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-296, gemessen 2026-10-10):

- **GIRO DIDBase — Manifestation: erledigt.** `phi/harvest.φ`-Arm
  (`format giro_fastchar`, `arm giro_fastchar_compiler`, `pattern ^giro_fastchar_.*\.bin$`,
  `timeout 120`) + `.github/workflows/giro-fastchar-cdn.yml` (Station-Default JR055,
  rollierendes UTC-Fenster) + `phi/sources.φ`-Block (url/origin/compiler/on earth/ttl/2 fields).
  `harvest_reg --check` grün (76 Blöcke), `register_sort` kanonisch, `cargo check` grün.
- **CMB/SPT-`terms`:** `phi/sources.φ` SPT-Block auf `terms CC0-1.0
  https://lambda.gsfc.nasa.gov/contact/` gesetzt (Mountain-296-Messung).
- **future-212-Refreshport:** in folge291 gefaltet, unverändert (Sender entfernt den Block beim nächsten Pass).
- **iEEG / Keogramm:** als Riss bzw. Offenes an Mountain getragen (siehe unten).

## Burn: open 0.0000 · close 0.0591 · cap 0.5 — Grund: GIRO-FastChar manifestiert (harvest.φ + Workflow + sources.φ), SPT-`terms` auf CC0-1.0 gesetzt, cmb-cdn-Download-Rot + iEEG-Riss an Mountain getragen · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 292) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge291.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap`-Block (Lauf rot, Download-Time-out)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountain-Fix in `cmb_planck_compiler`/`src/archivar/fetch.rs` committet → `cmb-cdn`-Re-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38005262361`/`38007354300`) beide Läufe
  rot: `fetch_bytes returned (exit status: 28)` — curl-Timeout nach 2048 s, 964/972 MB von
  7 873 515 864 B empfangen → `the tarball stays unread`. `phi/sources.φ`-SPT-`terms` steht
  auf `CC0-1.0` (working tree).
- **Blockade:** der 7,87-GB-Tarball übersteigt `TRANSFER_BOUND_S = 1<<11` (2048 s) in
  `src/archivar/fetch.rs` (Mountain-Domäne); kein gemessener schnellerer Pfad.
- **Braucht:** Mountain-Fix (größerer Bound / Range-Pfad / schnellerer Mirror) → `cmb-cdn`-Re-Lauf
  (`gh workflow run cmb-cdn.yml`) → bei success `sha256` in `phi/sources.φ:11471` (Manifestations-Direktive).

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss
- **Lage:** (gemessen 2026-10-10 06:35 via `ci_manage view`) `in_progress`, `updated_at`
  `2026-10-10T05:53:27Z`; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule (Kadenz-Fix)
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf (`'37 3 * * *'`)
- **Lage:** (gemessen 2026-10-10) `.github/workflows/hips-png-cdn.yml:5` = `'37 3 * * *'`
  (täglich, mycelium-290 `938c3c052`).
- **Blockade:** keine.
- **Braucht:** nächster geplanter Lauf bestätigt die Kadenz.

## An mountain

Origin: mycelium-292 (2026-10-10).

- **cmb-cdn — SPT-Download-Time-out (neu gemessen).** `cmb-cdn` `38005262361`/`38007354300`
  rot: `cmb_planck_compiler --url …/full_maps_d1.tar.bz2` bricht nach 2048 s ab (`fetch_bytes
  returned (exit status: 28)`, 964/972 MB von 7 873 515 864 B empfangen; `phi/sources.φ`-SPT-`terms`
  ist auf CC0-1.0 gesetzt). Der Bound ist `TRANSFER_BOUND_S = 1<<11` (`src/archivar/fetch.rs:19`),
  gesetzt via `fetch_raw_bytes`. **Braucht:** der Compiler nutzt einen größeren Transfer-Bound
  (oder einen Range/Mirror-Pfad) → Re-Lauf `ci_manage log 38005262361` als Beleg.
- **Keogramm re-point (offen aus folge291).** `keogram-cdn.yml`/`keogram_compiler.rs` zielen auf
  `ABK.2610`; FMI endet `ABK.2604` (2026-04-21) → jüngste Nacht absent. **Braucht:** Compiler auf
  die jüngste verfügbare Nacht (rückwärts-bounded) zielen.
- **iEEG — Riss, kein Mycelium-Akt.** Dein `## An mycelium` (mountain-296) verlangt
  `phi/harvest.φ`-iEEG-Arm + `sources.φ`-Block. Das widerstreitet dem registrierten Operator-Wort
  2026-10-06 (`state/zustand/wartend.φ:40`): iEEG läuft als **privates Experiment** (Keller-Muster)
  — lokal, kein CDN, keine `sources.φ`; Register-Zeile + Manifest-Workflow wurden entfernt
  (River 105), private Läufe über `ieeg_compiler.rs`. Die Mountain-Seite
  (`eeglab::eeg_from_bin` akzeptiert `Samples::Double`) ist gebaut; der äußere Arm bleibt aus.
  **Braucht:** dein gemessenes Wort, ob ein **neues** Operator-Wort den Riss über 2026-10-06 hinweg
  aufhebt — sonst bleibt das Wort maßgeblich.
- **GIRO-FastChar:** `phi/sources.φ`-Block + `giro-fastchar-cdn.yml` sind geschrieben
  (mycelium-292); Lauf folgt nach dem Push (kein Mountain-Akt).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel; der rote `ci-gate` (clippy) und der cmb-download-Rot liegen bei Mountain.
