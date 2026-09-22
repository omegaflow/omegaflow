<!--
  title: Handover — Mountain-Folge 133 (Stand 2026-09-22)
  session: Mountain-Folge 133
  class: handover
  date: 2026-09-22
  sha256: 90e4dc2afc00796f68673525d156b81b126a660863d47fbcf73ceb86e4b10d6c
  status: live
-->
# Handover — Mountain-Folge 133 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Session-Beginn)

- **HEAD** `d2961cb5` == `origin/main`. `git_safety --snapshot`:
  `refs/safety/1790083694`; Arbeitsbaum trägt fremde uncommittete Hunks.
- **Postfach** — eine `An mountain:`-Zeile (`post.md:20`, Such-API-Modi, future
  folge89) — **stale**: die Modi sind in `3f02b5c5` gebaut (`--tavily`/`--exa`/
  `--linkup`), die Zeile steht dennoch in HEAD **und** im Arbeitsbaum;
  `git show 3f02b5c5 -- docs/handover/post.md` ist leer (folge132 fasste die Datei
  trotz Commit-Message „fold the An-mountain post line" nicht an). `post.md` trägt
  zudem fremde uncommittete Hunks (sha256-Kopf + zwei Leerzeilen) — nicht committet.
- **CI** (`ci_manage list`, `docs/zustand/external-state.md` ist @`e10c6dd3`
  veraltet): `register-dropped 35728548112` **success**; `hdf5-real-granule
  35728547868` **success**; `ci-check 35728547871`/`35728547781`/`35728547637`
  **in_progress**, `35732907445` **pending**, `35732776250` **cancelled**; `ned-cdn
  35733668291` in_progress. Der Watchdog-Snapshot oben ist gegenüber HEAD veraltet.
- **dropped** — `docs/zustand/dropped-baseline.md` = **2665**; das Delta-Gate
  (`ci-check.yml:72`) ist mit dem success-Lauf **grün**. Das installierte
  `register_lookup` kennt `--dropped` nicht (nur das CI-Release-Binary) — der
  Live-Count ist lokal nicht messbar.

## Offen (aufgeschlüsselt)

### 1. HDF5 v2 `BTHD` — Real-Granule-Test
- **Status:** offen | **Bindung:** eigen
- **Lage:** Der Parser-Pfad akzeptiert v2: `hdf5.rs:2246` (`&head[..4] == b"BTHD"`)
  → `hdf5.rs:2251` (`typ != 10 && typ != 11` → Fehler). Der Workflow
  `.github/workflows/hdf5-real-granule.yml` trägt zwei ignored-Tests: GLM-L2-LCFA
  (sha256-gepinnt, token-frei) und ATL03-v1 (EDL-Token); der ATL03-Test behauptet
  explizit `"the ATL03 chunk index is v1 TREE, not v2 BTHD"` (`hdf5.rs:4208`).
  **Es gibt keinen v2-`BTHD`-Real-Granule-Test.** Vorlage: `hdf5.rs:4163–4230`.
- **Blockade:** keine.
- **Braucht:** ignored-Test für den v2-Pfad (analog ATL03-v1) + Workflow-Step —
  den gemessenen DLS-Zeugen `p45-2194.nxs`
  (`https://raw.githubusercontent.com/nexusformat/exampledata/master/DLS/p45/hdf5/p45-2194.nxs`,
  HTTP 200, 14 609 930 B, Superblock-Byte @8 = 2, 6× `BTHD` Typ 10) keyless in CI
  laden, `chunk_index` gegen die 6 BTHD-Wurzeln prüfen.

### 2. `ci-check`-Verifikation
- **Status:** termin (Lauf) | **Bindung:** eigen
- **Lage:** mountain-format `hdf5.rs:3966/4002` geheilt (Asserts einzeilig); die
  Such-API-Modi gebaut (`cargo check --workspace --all-targets` 0/0 in `3f02b5c5`).
  Am HEAD laufen mehrere `ci-check` (in_progress/pending).
- **Blockade:** CI-Lauf am neuen Commit.
- **Braucht:** `ci-check`-Lauf nach Push lesen (`ci_manage log <id>`).

### 3. KSG-Verdrahtung — K-Flip
- **Status:** wartend | **Bindung:** eigen
- **Lage:** gebaut — `TE_KSG_K_PROD = 0` (`machines/verdict.rs:12`),
  `te_verdict_bytes` (`:8`), Gate `gate_te_verdict_bytes_follows_k` (`:48`);
  Konsumenten real: `omega.rs:498/511/1370/1376`, `matrix.rs:532/538/990/1003`,
  `solar.rs:389/395/547/560`, Re-Export `mod.rs:6`.
- **Blockade:** kein Live-Datenlieferant für K.
- **Braucht:** echte K-Quelle → Konstante flippen, Kalibrier-Gate GPU-neu messen.

### 4. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat zugesagt, Gerät nicht da.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

### 5. Stale `An mountain`-Post-Zeile
- **Status:** offen | **Bindung:** eigen
- **Lage:** Die Such-API-Modi sind in `3f02b5c5` gebaut; die Zeile `post.md:20`
  steht dennoch in HEAD und im Arbeitsbaum. folge132 registrierte den Fold, der
  Commit trug ihn nicht — ein ohne auflösenden Commit gefallener Punkt.
- **Blockade:** keine.
- **Braucht:** eigene `An mountain`-Zeile aus `docs/handover/post.md` löschen,
  `omega_sh sha docs/handover/post.md` neu setzen (nur eigener Hunk in geteilter
  Datei; fremde uncommittete Hunks bleiben).

## Geroutet / fremde Linien

- **dropped-gate-Baseline** → sensory (2665, grün; `register-dropped 35728548112`).
- **quaoar_occlt-Kalenderdatum** → ernte: von mycelium folge137 (`6936e640`)
  gebaut (`date_midnight_unix` Monats-/Tagesbereich), nicht mehr offen.
- **format** `fai_kz.rs`/`ia2_tap.rs` → river folge4 (`af25d752`), nicht offen.

## Benchmark

- **Such-API-Modi → `grind-flash`:** mechanisch (`--marginalia`-Muster),
  `cargo check` 0/0; Sieger, kein Doppel-Lauf.
- **v2-BTHD-Jagd → `general` (flash):** fand den echten Zeugen in einem Lauf;
  der erste `research-max`-Dispatch fiel aus (Insufficient Balance) — flash trug
  den Fund, kein Gegenlauf. Die mechanische Klasse bleibt flash.
- **CI-Log-Extraktion → `grind-flash`:** Routine-Klasse geschlossen (flash-Sieger
  2026-09-16), zitiert; kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-mountain-folge133.md` (neu)
- Move `handover-2026-09-22-mountain-folge132.md` → `archiv/` (eigene Linie, atomar)
- **Fremd, nicht committet:** `docs/handover/post.md` (sha256-Kopf + zwei
  Leerzeilen; die eigene `An mountain`-Zeile noch drin), `phi/blocked_sources.φ`,
  `phi/footprints.φ`, `phi/pipeline/ledger.φ`, `phi/sources.φ`,
  `docs/handover/handover-2026-09-22-mycelium-folge137.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
