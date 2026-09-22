<!--
  title: Handover — Mountain-Folge 137 (Stand 2026-09-22)
  session: Mountain-Folge 137
  class: handover
  date: 2026-09-22
  sha256: 3e2ac9db08cd3fd2b03ee50f08df7b1e1b539b0feb3ec58b2c4c7026966f2274
  status: live
-->
# Handover — Mountain-Folge 137 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Ausführungs-Beginn)

- **HEAD** `f676260da` == `origin/main` (measure-Linie: Katalog-Zeilen ohne
  Registry-Träger entfernt, Agent-Modell-IDs auf `deepseek-flash` gezogen).
- **`git_safety --snapshot`:** Baum == HEAD, nichts zu sichern (Planungs-Pass).
- **Postfach** — kein Postfach-Ledger vorhanden, kein `An mountain:`-Eingang,
  kein `mail_digest`. Kein offener Eingang.
- **CI** — `ci_manage list`: `te-gate 35767848399` (head_sha `7ddd75edb`)
  **in_progress**. `hdf5-real-granule 35767837109` (head_sha `7ddd75edb`)
  **success** — Punkt 1 CI-verifiziert (drei ignorierte Real-Granule-Tests grün:
  GLM L2, ATL03 v1-TREE, DLS v2-BTHD). Fremd: `ci-check 35767837058` failure
  (sensorys te.rs-Heilung, Punkt in sensory-folge148), `free-model-agent-bench` /
  `text-probe` (measure-Linie).

## Offen (aufgeschlüsselt)

### 1. KSG K-Regel — `TE_KSG_K_PROD` flippen
- **Status:** termin (CI-Lauf) | **Bindung:** eigen
- **Lage:** Apparat gebaut in `src/mathematikerin/ksg_k.rs` (`#![cfg(test)]`):
  Formelpfad `formula_k(n,d,τ)`, Sweep `run_ksg_k_sweep` k=1..=12 gegen FPR/FN,
  Parity-Gate bei k=4. Der Test `ksg_k_gate_sweep_and_formula` läuft als Step in
  `.github/workflows/te-gate.yml`. Der CI-Lauf `te-gate 35767848399` auf
  `head_sha 7ddd75edb` ist **in_progress** (seit 18:32Z). `TE_KSG_K_PROD` bleibt
  **0** (`machines/verdict.rs:12`) — kein K fabriziert, solange die Messung fehlt.
- **Blockade:** keins.
- **Braucht:** `ci_manage log 35767848399` **einmal**, sobald der Lauf beendet ist
  → stimmen Formel und Sweep überein, flippt der Operator `TE_KSG_K_PROD` auf das
  gemessene k; bei `riss`/`void` bleibt K=0, der Log ist der Befund.

### 2. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Lage:** PINE64 hat **zwei** Ox64 SBCs versandt (Ledger `1790046330`, Tracking
  `LZ473049629CN`); physisch nicht angekommen.
- **Blockade:** Geräteankunft.
- **Braucht:** Ankunft abwarten → Bring-up + Kopplung messen.

## Geroutet / fremde Linien

- **Punkt 1 aus folge136 (HDF5 v2 `BTHD` Real-Granule)** — **erledigt und
  CI-verifiziert**, aus dem Register gelöscht: `hdf5-real-granule 35767837109`
  (head_sha `7ddd75edb`) grün, Log belegt
  `real_granule_dls_v2_bthd_chunk_index_materializes` gegen den Zeugen
  `p45-2194.nxs` (sha256 `b5ab7f87…`) bestanden.
- **`ci-check` 35767837058** — sensorys te.rs-Heilung; nicht Mountain.
- **`docs/zustand/external-state.md:35`** trägt weiter den stale Schritt „Bau
  `--tavily`/`--exa`/`--linkup`" (Modi gebaut) — `An mycelium:` in `post.md` steht,
  nicht Mountain.
- **measure-Linie** — `tools/measure/` (`free_models.tsv`, `free_text_models.tsv`,
  `free_model_agent_bench.rs`, …) und die Workflows `free-model-agent-bench.yml`,
  `text-probe.yml` sind am HEAD `f676260da` **committet**; nicht Mountain. Der
  Baum war zu Session-Beginn sauber (keine fremde uncommittete Arbeit).

## Benchmark

- Kein Doppel-Lauf, kein Modell-Burn: Punkt 1 wurde per CI-Log-Beleg geschlossen
  (kein Agent), die Klasse war in folge136 mit `grind-pro` entschieden. Punkt 2
  bleibt CI-gebunden, nicht dispatchbar.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-mountain-folge137.md` (neu)
- Move `handover-2026-09-22-mountain-folge136.md` → `archiv/` (eigene Linie, atomar)
- **Nur diese zwei Dateien** — fremde Arbeit (measure) liegt committet am HEAD,
  nicht in diesem Commit.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
