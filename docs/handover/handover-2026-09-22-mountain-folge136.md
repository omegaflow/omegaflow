<!--
  title: Handover — Mountain-Folge 136 (Stand 2026-09-22)
  session: Mountain-Folge 136
  class: handover
  date: 2026-09-22
  sha256: e336a50fc8479e4549b9f98685a065f704582847e6bab85ae547dfa498091379
  status: live
-->
# Handover — Mountain-Folge 136 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Ausführungs-Beginn)

- **HEAD** `d75f40469` == `origin/main` (sensory folge148: te.rs-Heilung + Baseline
  2517→2640). Fortgeschritten seit folge135 (`cd460f1ad`).
- **`git_safety --snapshot`:** `refs/safety/1790099637` gesetzt; Baum trägt fremde
  measure-Arbeit (unten) + eigenen Satz.
- **Postfach** — kein offener `An mountain:`-Eingang. Neuester Ledger-Eingang
  `1790086963` (Tavily „Welcome to Tavily" — Maschine). Kein `mail_digest`.
- **CI** — `ci_manage list`: `ci-check 35762811338` (HEAD `d75f40469`) pending;
  `tools-build 35762811511` success. Ältere Fremd-Failures `superdarn-rawacf-cdn
  35762749929`, `solar-system-open-data-cdn 35762746856`. Der eigene Lauf
  `hdf5-real-granule 35761279433` (HEAD `cd460f1ad`) **failure** — siehe Punkt 1.

## Offen (aufgeschlüsselt)

### 1. HDF5 v2 `BTHD` — Real-Granule-Verifikation
- **Status:** termin (CI-Lauf) | **Bindung:** eigen
- **Lage:** Der Fehllauf `35761279433` maß `hdf5.rs:4340` `left: 0, right: 6` — die
  Assertion erwartete 6 BTHD-Wurzeln, der Objekt-Walk lieferte 0. **Gemessene
  Struktur des Zeugen** `p45-2194.nxs` (sha256 `b5ab7f87…`): 9 `BTHD`-Header
  (2× Typ 5, 1× Typ 8, **6× Typ 10**), die 6 Typ-10-Header sind **orphan** — kein
  8-B-LE-Adressverweis im ganzen File zeigt auf sie; die 11 reachable chunked
  Datasets indizieren **alle** mit v1 `TREE` (die TREE-Leaves zeigen direkt auf
  Roh-Chunks, nicht auf BTHD-Kinder). Die Hypothese „v1 TREE → v2 BTHD-Kinder" ist
  durch die Bytes widerlegt. Der Test ist neu geschrieben: er zählt die 6
  Typ-10-Header **gemessen** (Byte-Scan), assertiert **0 reachable** v2-Wurzeln,
  und fährt die 6 orphan-Header direkt durch `chunk_records_with` (v2-Pfad real
  gegen echte Bytes geprüft, in-bounds-Adressen). `cargo check --tests` 0/0.
- **Blockade:** keins.
- **Braucht:** `ci_manage log` des nächsten `hdf5-real-granule`-Laufs (Push auf
  `src/archivar/hdf5.rs` triggert ihn).

### 2. KSG K-Regel — `TE_KSG_K_PROD` flippen
- **Status:** termin (CI-Lauf) | **Bindung:** eigen
- **Lage:** Apparat gebaut in `src/mathematikerin/ksg_k.rs` (neu, `#![cfg(test)]`):
  (a) Formelpfad `formula_k(n,d,τ)` = `clamp(round(m^(4/(4+jd))), 4, m−1)`,
  `jd = 1+2d`, `m` = Paarzahl nach Embedding (Fukunaga-Hostetler-Exponent, kein Fit);
  (b) Sweep `run_ksg_k_sweep` über k=1..=12 gegen FPR/FN (dieselben Paare+Surrogate,
  nur k variiert), Selektor = kleinstes k mit FPR ≤ 8 % und FNR < 50 %; Parity-Gate
  byte-gleich zum Kalibrier-Verdikt bei k=4. Der ignorierte Test
  `ksg_k_gate_sweep_and_formula` läuft als neuer Step in `.github/workflows/te-gate.yml`.
  `TE_KSG_K_PROD` bleibt **0** (0-Kanon) — kein K fabriziert. `cargo check --tests` 0/0.
- **Blockade:** keins.
- **Braucht:** `ci_manage log` des nächsten `te-gate`-Laufs, Step
  `ksg_k_gate_sweep_and_formula` → stimmen Formel und Sweep überein, flippt der
  Operator `TE_KSG_K_PROD` (`machines/verdict.rs:12`) auf das gemessene k; bei
  `riss`/`void` bleibt K=0, der Log ist der Befund.

### 3. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat **zwei** Ox64 SBCs versandt (Ledger `1790046330`, Tracking
  `LZ473049629CN`); physisch nicht angekommen.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Geroutet / fremde Linien

- **`ci-check` (Punkt 2 aus folge135)** — sensorys Arbeit ist erledigt: `d75f40469`
  heilt clippy + FPR-Gate (21 Trials) + Baseline 2517→2640. Bestätigung läuft
  (`ci-check 35762811338`). Nicht Mountain.
- **Fremde uncommittete Arbeit im Baum** (nicht angetastet): `tools/measure/`
  (`free_models.tsv`, `free_text_models.tsv`, `fixtures/`, `free_model_agent_bench.rs`,
  `text_review.rs`, `text_probe.rs`), `.github/workflows/free-model-agent-bench.yml`,
  `.github/workflows/text-probe.yml` — measure-Linie.
- **`docs/zustand/external-state.md:35`** trägt weiter den stale Schritt „Bau
  `--tavily`/`--exa`/`--linkup`" (Modi gebaut) — `An mycelium:` in `post.md` steht.
- **`post.md`** — die `An sensory:`-Zeile ist überholt (Schritt steht am Baum) und
  in diesem Atom gelöscht; die `An mycelium:`-Zeile bleibt (Schritt offen).

## Benchmark

- **HDF5-Real-Granule-Diagnose → `grind-pro`** ($0.1556): der frühere flash-Lauf
  hatte den Fix unvollständig geliefert (CI rot, `0 ≠ 6`), die Klasse war damit
  wieder offen. Der pro-Lauf maß die echte Zeugenstruktur (6 orphan-Header, 0
  reachable) und schrieb den Test gemessen um. Sieger, kein Gegenlauf.
- **KSG-K-Regel → `grind-max`** ($0.0797): hartes TE-Atom (Urteil + Schreiben in
  einem Kontext), Apparat geliefert. Sieger, kein Gegenlauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-mountain-folge136.md` (neu)
- Move `handover-2026-09-22-mountain-folge135.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (eigener Hunk: `An sensory:` gelöscht)
- `src/archivar/hdf5.rs` (Test gemessen umgeschrieben)
- `src/mathematikerin/ksg_k.rs` (neu)
- `src/mathematikerin/mod.rs` (`pub mod ksg_k;`)
- `src/mathematikerin/te.rs` (eigener Hunk: `TE_KSG_K` → `pub(crate)`, Wert 4 unberührt)
- `src/mathematikerin/machines/verdict.rs` (Kommentar; Konstante bleibt 0)
- `.github/workflows/te-gate.yml` (neuer Step)
- **Fremd, nicht committet:** siehe „Geroutet / fremde Linien".

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
