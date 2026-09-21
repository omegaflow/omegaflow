<!--
  title: Handover — Bau-Folge 118 (Stand 2026-09-21)
  session: Bau-Folge 118
  class: handover
  date: 2026-09-21
  sha256: 448804dab409d0984ee28daa8ef64e837f0b5377f4d31792ee4846599a148b53
  status: live
-->
# Handover — Bau-Folge 118 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keinen „härtesten Punkt" mehr — die offenen Punkte werden **parallel**
von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** Session-Beginn `d6bf1df1` (folge117) == `origin/main`.
- **Postfach** — die `An bau`-Zeile (smail-Riegel) in diesem Atom gefaltet und
  aus `post.md` gelöscht; `An forschung` (Ksg) steht (fremd). `external-state.md`
  (CI-Zeile) und `post.md` wurden von der entscheid-Session fortgeschrieben
  (fremd, nicht angefasst).
- **CI** — `ci-check` `35569256029` queued (der format-Job sollte bis auf
  `ps1_coverage_compiler.rs` grün sein). Kein Poll.

## Messung dieses Atoms (kein Punkt)

- **smail-Wahrheits-Riegel gebaut** (`An bau` gefaltet; Operator-Regel „Truth
  gate for outgoing mail", 2026-09-21): `tools/service/src/bin/smail.rs` —
  `gate(body)` parst den QUELLEN-Block, verweigert einen fehlenden oder
  unaufgelösten Block mit `exit(2)`, `--dry-run` druckt die Tabelle
  (`claim | source | resolves`); drei Quellen-Formen (`file:line`,
  `register#key`, `command@ts → artifact`), Rust std, kein Netz; 6 Gate-Tests;
  `cargo check -p omegaflow-service` 0/0. Kein Bypass-Flag.
- **fmt-Rest `ps1_coverage_compiler.rs` gemessen:** der fmt-Fix ist **bereits auf
  der Platte** (uncommittet) — die als „fremd" gemeldete Änderung *ist* die zwei
  fmt-Reformatierungen (605/644, +16/−2). Kein eigener Eingriff nötig; der
  Commit ist erntes.

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| fmt-Commit `ps1_coverage_compiler.rs` | `wartend` | `linie:ernte` | ernte committet die Datei (der fmt-Fix liegt uncommittet vor); danach `ci-check` format grün. |
| `An forschung` Ksg off-path | `wartend` | `linie:forschung` | Zeile steht in `post.md`; forschung faltet sie. |

## Benchmark

- **Bau-Folge 118**: 2 parallele Agenten — `grind-pro` (smail-Riegel,
  Urteil/Parser) + `grind-flash` (ps1-fmt, mechanisch; Befund: schon erledigt).
  `cargo check` direkt im `build`-Kontext verifiziert.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/service/src/bin/smail.rs`,
  `docs/handover/post.md` (An-bau-Zeile gefaltet), neues
  `docs/handover/handover-2026-09-21-bau-folge118.md`, Move
  `handover-2026-09-21-bau-folge117.md` → `archiv/`.
- **Fremd (nicht angefasst):** `AGENTS.md`, `src/gate/*` (state_claim/
  serial_priority-Gate, in Arbeit), `phi/*`,
  `tools/harvest/src/bin/ps1_coverage_compiler.rs` (fmt-Fix, ernte),
  `docs/zustand/external-state.md`,
  `handover-2026-09-21-entscheid-folge72.md`,
  `handover-2026-09-21-ernte-folge126.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
