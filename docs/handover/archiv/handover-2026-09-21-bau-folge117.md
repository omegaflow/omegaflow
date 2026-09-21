<!--
  title: Handover — Bau-Folge 117 (Stand 2026-09-21)
  session: Bau-Folge 117
  class: handover
  date: 2026-09-21
  sha256: 89a406d753556429e32e3d14852028691868daa0005d233106d203727ded3422
  status: live
-->
# Handover — Bau-Folge 117 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keinen „härtesten Punkt" mehr — die offenen Punkte werden **parallel**
von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** Session-Beginn `f95e6d9a`; eigenes Atom `4a0c7803` (folge116) +
  `a70b1ee2` (fmt). `origin/main` bei Push-Beginn `4a0c7803`.
- **Postfach** — kein `An bau` außer der smail-Zeile (unten); die
  entscheid-folge72-Session hat `external-state.md` (CI-Zeile auf `4a0c7803`)
  und `post.md` fortgeschrieben (fremd, nicht angefasst).
- **CI** — `ci-check` `35568374735` pending; `hyperscanning-te` `35567708611`
  failure; `te-gate` in_progress; `tools-build` success. Kein Poll.

## Messung dieses Atoms (kein Punkt)

- **fmt-Rot repo-weit geheilt** (`a70b1ee2`): 5 Dateien
  (`tools/measure/{free_model_agent_bench,free_model_bench,hyperscanning_group_te}.rs`,
  `tools/register/register_lookup.rs`, `tools/harvest/bia_efield_compiler.rs`) —
  Hand-Formatierung nach dem CI-Diff (`35537130867` @`5894b345`), **2 parallele
  `grind-flash`-Agenten**. `ps1_coverage_compiler.rs` bleibt ernte (fremd
  uncommitted).
- **Kanon-Akt „strukturierte Feld-Grammatik" entschieden** (Operator-Wort
  2026-09-21): die Grammatik **IST** `docs/specs/sources-v2-spec.md` §1 (645 Z.,
  „Verified against the living parser"). Audit Spec ↔ Parser am HEAD `4a0c7803`:
  - 3-Token-`field <key> <identifier>`: Spec §1 Z.73 + §3.7 erlaubten sie,
    `parse.rs:866` verweigert sie (τ-Gate) → Spec auf den Parser angeglichen und
    als §10-Gap registriert.
  - Header-Referenz `load_sources` in `src/main.rs` → `src/archivar/parse.rs:34`.
  - 8 undokumentierte Parser-Arme (`profile`, `quakeml`, `alerce`, `xmlcount`,
    `lastobj`, `lastline`, `lat_sign`, `lon_sign`, `epoch_scale`) als
    §10-Pending benannt.
  - Spec-sha256 `11f6d1bc…`; `An entscheid`-Zeile: Nr. 11 schließen.

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| smail-Wahrheits-Riegel (`An bau`) | `wartend` | `eigen` | `tools/service/src/bin/smail.rs`: `--send` parst den QUELLEN-Block, verweigert fehlenden/unaufgelösten Block (exit 2), `--dry-run` druckt die Tabelle (claim \| source \| resolves); `src/gate/*` trägt bereits einen **fremden uncommitteten** `state_claim`-Ansatz — vorher messen, nicht doppelt bauen. |
| fmt-Rest `ps1_coverage_compiler.rs` | `wartend` | `linie:ernte` | ernte committet die Datei (fremd uncommitted); danach `ci-check` format grün. |

## Benchmark

- **Bau-Folge 117**: 2 parallele `grind-flash` (fmt, mechanisch) + 1 `council`
  (Kanon-Akt, pro/max). Der Spec-Audit lief direkt im `build`-Kontext. Flash
  trug die Mechanik — kein Doppel-Lauf nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `docs/specs/sources-v2-spec.md`, `docs/handover/post.md`
  (**nicht committet** — trägt fremde uncommittete Zeilen `An bau`/`An forschung`;
  die eigene `An entscheid`-Zeile wartet auf den nächsten Committer der Datei),
  neues `docs/handover/handover-2026-09-21-bau-folge117.md`, Move
  `handover-2026-09-21-bau-folge116.md` → `archiv/`, fmt-Commit `a70b1ee2`
  (5 Fremd-Linien-Dateien, eigene Agenten-Arbeit).
- **Fremd (nicht angefasst):** `AGENTS.md`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json`, `phi/sources.φ`, `phi/blocked_sources.φ`,
  `tools/harvest/src/bin/ps1_coverage_compiler.rs`,
  `docs/handover/handover-2026-09-21-entscheid-folge72.md`,
  `docs/zustand/external-state.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
