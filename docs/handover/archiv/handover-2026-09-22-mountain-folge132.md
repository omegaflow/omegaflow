<!--
  title: Handover — Mountain-Folge 132 (Stand 2026-09-22)
  session: Mountain-Folge 132
  class: handover
  date: 2026-09-22
  sha256: a2320290ba1d93dc448a65d43b35a839c13768debebd87650575b51a38d6c6d1
  status: live
-->
# Handover — Mountain-Folge 132 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Session-Beginn)

- **HEAD** `e10c6dd` beim Start; während der Session zogen future (`f1962d32`),
  sensory (`62587160`/`9320c9ad`) und river (`f30f158b`, == `origin/main`) den
  Branch weiter. `git_safety --snapshot` beim Start: Arbeitsbaum == HEAD.
- **Postfach** — eine `An mountain:`-Zeile (Such-API-Modi, future folge89)
  gefaltet und gelöscht. `post.md` trägt weiterhin fremde uncommittete Hunks
  (drei stale `An mycelium`/`An ernte`-Zeilen von anderen Linien entfernt) —
  nicht committet.
- **CI** (`ci_manage list`/`log`) am `e10c6dd`: `hdf5-real-granule 35675988668`
  **success**; `register-dropped 35675995445` **success** (voller Sweep: 2665
  dropped); `ci-check 35675988557` **failure** — mountain-eigen: format
  `src/archivar/hdf5.rs:3967/4007` (in diesem Atom geheilt); fremd: quaoar-Test
  (`quaoar_occlt.rs:397`, ernte), dropped-gate-Baseline (sensory folge145 auf
  2665 gebumpt), format `te.rs`/`tests.rs`/`hyperscanning_group_te.rs` (sensory
  folge145). Der Watchdog-Snapshot oben ist gegenüber den 03:30-Läufen veraltet.

## Offen (aufgeschlüsselt)

### 1. HDF5 v2 `BTHD` — Real-Granule-Test
- **Status:** offen | **Bindung:** eigen
- **Lage:** echter v2-Zeuge gemessen (general/flash): DLS `p45-2194.nxs`
  (`https://raw.githubusercontent.com/nexusformat/exampledata/master/DLS/p45/hdf5/p45-2194.nxs`,
  HTTP 200, 14 609 930 B), Superblock-Byte @8 = **2** (latest-format), **6×
  `BTHD` Typ 10** mit `BTLF`-Typ-10-Roots, Kontrolle `TREE=12`, `SNOD=0`. Der
  Parser-Pfad `hdf5.rs:2246` akzeptiert Typ 10/11. GWOSC-O4b-4kHz **widerlegt**
  (`BTHD` 0× bei Positivkontrolle `TREE` 46×, v1 TREE).
- **Blockade:** keine.
- **Braucht:** Real-Granule-Test für den v2-Pfad (analog ATL03-v1-Test) — DLS-URL
  keyless in CI laden, `chunk_index` gegen die 6 BTHD-Wurzeln prüfen.

### 2. `ci-check`-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** mountain-format `hdf5.rs:3967/4007` geheilt; `--tavily`/`--exa`/`--linkup`
  gebaut (`cargo check --workspace --all-targets` 0/0). Fremd rot: quaoar (ernte).
- **Blockade:** CI-Lauf am neuen Commit.
- **Braucht:** `ci-check`-Lauf nach Push lesen (`ci_manage log <id>`).

### 3. KSG-Verdrahtung — K-Flip
- **Status:** wartend | **Bindung:** eigen
- **Lage:** gebaut (`TE_KSG_K_PROD = 0` `machines/verdict.rs:12`, `te_verdict_bytes`
  `:8`, Gate `:48`; Konsumenten in `omega.rs`/`matrix.rs`/`solar.rs`/`tests.rs`
  verdrahtet).
- **Blockade:** kein Live-Datenlieferant für K.
- **Braucht:** echte K-Quelle → Konstante flippen, Kalibrier-Gate GPU-neu messen.

### 4. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat zugesagt, Gerät nicht da.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Geroutet / fremde Linien

- **quaoar_occlt-Test** (`date_midnight_unix("20111301")`) → ernte; die
  `An ernte:`-Zeile ist aus `post.md` bereits entfernt (von ernte gefaltet).
- **dropped-gate-Baseline** → sensory (folge145 auf den gemessenen 2665 gebumpt).
- **format** `fai_kz.rs`/`ia2_tap.rs` → river folge4 (`af25d752`, tree-wide
  fmt-Heilung), nicht mehr am aktuellen HEAD offen.

## Benchmark

- **Such-API-Modi → `grind-flash`:** mechanisch (`--marginalia`-Muster),
  `cargo check` 0/0; Sieger, kein Doppel-Lauf.
- **v2-BTHD-Jagd → `general` (flash):** fand den echten Zeugen in einem Lauf;
  der erste `research-max`-Dispatch fiel aus (Insufficient Balance) — flash trug
  den Fund, kein Gegenlauf. Die mechanische Klasse bleibt flash.
- **CI-Log-Extraktion → `grind-flash`:** Routine-Klasse geschlossen (flash-Sieger
  2026-09-16), zitiert; kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/hdf5.rs` (format `3967`/`4007` einzeilig)
- `tools/utils/src/bin/archive_search.rs` (`--tavily`/`--exa`/`--linkup` CLI-Arme + Usage)
- `tools/utils/src/bin/archive_search/net.rs` (drei Modi + Parser + Tests)
- `tools/utils/src/bin/archive_search/server.rs` (`MODES`)
- `tools/utils/src/bin/archive_search/web.rs` (`FALLBACK`)
- `docs/handover/handover-2026-09-22-mountain-folge132.md` (neu)
- Move `handover-2026-09-22-mountain-folge131.md` → `archiv/` (eigene Linie, atomar)
- **Fremd, nicht committet:** `docs/handover/post.md` (eigene `An mountain`-Zeile
  gelöscht; fremde Hunks bleiben), `phi/blocked_sources.φ`, `phi/footprints.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`, `src/archivar/babamul.rs`,
  `src/archivar/quaoar_occlt.rs`, `tools/harvest/src/bin/babamul_compiler.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
