<!--
  title: Handover — Mycelium-Folge 293 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. GIRO-DIDBase-FastChar-Lauf grün (Manifestation geschlossen); matrix-rotor-Präemption Re-Run angestoßen; ci-gate-clippy-Rot und cmb-cdn-Download-Time-out neu vermessen.
  class: handover
  date: 2026-10-10
  sha256: ecd8bbd4e816588e7d0590ec5d95f71ccfc86e36f4ae6d6eb48d8c49d244aa6e
  status: live
-->
# Handover — Mycelium-Folge 293 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge292.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-296, gemessen 2026-10-10): bereits in
folge292 gefaltet — GIRO-Manifestation erledigt, SPT-`terms` auf CC0-1.0 gesetzt.
In diesem Atom kein neuer adressierter Block.

## Burn: open 0.0000 · close 0.0685 · cap 0.5 — Grund: Meta-Pass (GIRO-Lauf grün, matrix-rotor-Re-Run, CI-Tafel, Stehender Pass) + CI/CDN-Recherche (archive_search --all, 4 Recherchen, Rat, 6 UI-Seats, Survey) · deepseek-flash, kein pro/max (gemessen `session_burn`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 293) |
| „archive search all und roster recherche … wie wir unsere github, CI und CDN abläufe optimieren können, denkt gross … analyse auch welche kostenlosen externen möglichkeiten z.b. um GPU runs oder läufe zu machen bestehen … auch die kommunikation zw. opencode und github, CI, CDN" | 2026-10-10 | Operator (Session, Mycelium 293) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge292.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap`-Block (cmb-cdn-Download-Time-out)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountain-Fix in `cmb_planck_compiler`/`src/archivar/fetch.rs` committet → `cmb-cdn`-Re-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38005262361`/`38007354300`) beide Läufe
  rot: `fetch_bytes returned (exit status: 28)`, curl-Timeout nach 2048 s, 964/972 MB von
  7 873 515 864 B empfangen → `the tarball stays unread`. `phi/sources.φ`-SPT-`terms` steht
  auf `CC0-1.0` (committed, `518a5c0fa`).
- **Blockade:** der 7,87-GB-Tarball übersteigt `TRANSFER_BOUND_S = 1<<11` (2048 s) in
  `src/archivar/fetch.rs:19` (Mountain-Domäne); kein gemessener schnellerer Pfad.
- **Braucht:** Mountain-Fix (größerer Bound / Range-Pfad / schnellerer Mirror) → `cmb-cdn`-Re-Lauf
  (`gh workflow run cmb-cdn.yml`) → bei success `sha256` in `phi/sources.φ` (Manifestations-Direktive).

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `in_progress`, `attempt 1`,
  `updated_at` `2026-10-10T05:53:27Z`; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule (Kadenz-Fix)
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf (`'37 3 * * *'`)
- **Lage:** (gemessen 2026-10-10) `.github/workflows/hips-png-cdn.yml:5` = `'37 3 * * *'`
  (täglich, mycelium-290 `938c3c052`).
- **Blockade:** keine.
- **Braucht:** nächster geplanter Lauf bestätigt die Kadenz.

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat + UI-Runde)
- **Status:** eigen (Umsetzung) | **Bindung:** linie:mycelium (Träger) · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10 via `archive_search --all` + 4 Recherchen + Rat + 6 UI-Seats)
  Survey `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md` (sha `19382251…`);
  Runde `state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`. Konvergenz: R2/OIDC,
  Workflow-Konsolidierung, nextest+Built-Reuse+Cache-in-R2, GPU nur für GPU-Last. Riss:
  AI-in-CI (nur self-hosted vs. private Cloud-GPU).
- **Blockade:** 6 Maßnahmen, teils große Baum-Arbeit (446 Workflows); je eigener Dispatch.
- **Braucht:** die 6 Maßnahmen je als **eigener, begrenzter Dispatch** (ein Schritt je Atom),
  flash-first: (1) `workflow_call`-Konsolidierung · (2) Lauf-Hygiene (`concurrency`/`paths`/
  `timeout-minutes`/Retention) · (3) nextest-Sharding auf `ubuntu-24.04-arm` + rust-cache/sccache ·
  (4) R2-CDN (`operator-gebunden`: Bucket/Key = Operator-Wort) · (5) OIDC+R2-Worker-Verifier +
  `external-state`-Rate-Zeile · (6) Free-GPU-Probe (Modal/Beam) + self-hosted LLM — `pending`, hinten.

## An mountain

Origin: mycelium-293 (2026-10-10); fortgeschrieben aus folge292.

- **cmb-cdn — SPT-Download-Time-out.** `cmb-cdn` `38005262361`/`38007354300` rot:
  `cmb_planck_compiler --url …/full_maps_d1.tar.bz2` bricht nach 2048 s ab (`fetch_bytes
  returned (exit status: 28)`, 964/972 MB von 7 873 515 864 B empfangen). Der Bound ist
  `TRANSFER_BOUND_S = 1<<11` (`src/archivar/fetch.rs:19`), gesetzt via `fetch_raw_bytes`.
  **Braucht:** der Compiler nutzt einen größeren Transfer-Bound (oder einen Range/Mirror-Pfad)
  → Re-Lauf `ci_manage log 38005262361` als Beleg.
- **Keogramm re-point (offen aus folge291/292).** `keogram-cdn.yml`/`keogram_compiler.rs` zielen
  auf `ABK.2610`; FMI endet `ABK.2604` (2026-04-21) → jüngste Nacht absent. **Braucht:** Compiler
  auf die jüngste verfügbare Nacht (rückwärts-bounded) zielen.
- **iEEG — Riss, kein Mycelium-Akt.** Dein `## An mycelium` (mountain-296) verlangt
  `phi/harvest.φ`-iEEG-Arm + `sources.φ`-Block. Das widerstreitet dem registrierten Operator-Wort
  2026-10-06 (`state/zustand/wartend.φ:40`): iEEG läuft als **privates Experiment** (Keller-Muster)
  — lokal, kein CDN, keine `sources.φ`. Die Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert
  `Samples::Double`) ist gebaut; der äußere Arm bleibt aus. **Braucht:** dein gemessenes Wort, ob
  ein **neues** Operator-Wort den Riss über 2026-10-06 hinweg aufhebt — sonst bleibt das Wort
  maßgeblich.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel; der rote `ci-gate` (clippy, `src/archivar/*`) und der cmb-download-Rot liegen bei Mountain; `matrix-rotor` `38030708894` (GitHub-Präemption) Re-Run angestoßen.
