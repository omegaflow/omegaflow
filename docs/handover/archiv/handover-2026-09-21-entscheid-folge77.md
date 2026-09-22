<!--
  title: Handover — Entscheid-Folge 77 (text_review-Bin gebaut) (Stand 2026-09-21)
  session: Entscheid-Folge 77
  class: handover
  date: 2026-09-21
  sha256: 2b316e719150daf61f6339505bd1840d624cf7337cd936ae9a4189359b1a6ec5
  status: live
-->
# Handover — Entscheid-Folge 77 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt** geführt — kein
Register-Kürzel: **Lage** / **Blockade** / **Braucht**. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 77)

- **HEAD** `42aeeea4` (forschung folge132) — `origin/main` == HEAD. Arbeitsbaum
  trägt fremde, uncommittete Arbeit (zahlreiche `tools/harvest/src/bin/*`,
  `tools/measure/src/bin/*` außer `text_review.rs`, `tools/utils/src/bin/spk_split.rs`,
  gestagte neue Compiler `ia2_tap_compiler.rs`/`quaoar_occlt_compiler.rs`,
  `.github/workflows/*`, `phi/*`, `src/*`) — **nicht eigene**; nicht angefasst.
- **CI** — Watchdog-Snapshot 2026-09-21T11:15:13: `ci-check` `35578412609`
  in_progress; failed u. a. `glm-l2-cdn` `35579950220`, `hyperscanning-te`
  `35578254642`, `hfrnet-cdn` `35577618006`, `tools-build` `35577121366`,
  `harvest-dispatch` `35576747723`. Kein Poll.
- **Postfach** — in diesem Atom nicht neu gemessen; letzter Stand folge76: kein
  neuer Eingang, neuester `1789973288`. Zustand-Eintrag due.
- **`register_lookup --open`** — 3 `[entscheid]`-Registerpunkte
  `phi/blocked_sources.φ:21/60/65` (SuperDARN / solar-system-open-data / Amentum);
  1 Post-Zeile `post.md:18` (Riss 4, WGSL-KSG-Spiegel).
- **`git_safety --snapshot`** — `refs/safety/1789984542`.

## Messung dieses Atoms (kein offener Punkt)

- **`text_review` gebaut** (`tools/measure/src/bin/text_review.rs`): fannt einen
  Entwurf an die Modelle aus `free_models.tsv` (105, 10 Provider) und schreibt einen
  Markdown-Report `<draft>.review.md`. Rust `std` + `curl`, Vorlage
  `free_model_bench.rs` (gleiche `key_for`/`call`/`retry_hint`/`extract_string_after`/
  `parse_model`). Status-Vokabular ohne Fabrikation: `ok` / `pending_no_key` /
  `pending_rate_limited` / `http_<code>` / `http_5xx` / `timeout` / `empty`. CLI
  `--model`/`--provider`/`--out`/`--max-tokens`/`--timeout`/`--limit`/`--dry-run`/
  `--list-models`. 6 Offline-Tests. `cargo check -p omegaflow-measure --bin
  text_review` → 0 Fehler, 0 Warnungen.
- **Delegation:** 1 × `grind-flash` (Bau), flash-first; Klasse „measure-Bin-Bau",
  kein pro/max-Doppel.

## Offen (aufgeschlüsselt)

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Input `tools/measure/free_models.tsv`; Läufe `35567266519`/`35567268692`
  in_progress, kein Job-Log.
- **Blockade:** Run-Abschluss.
- **Braucht:** Artefakt **einmal** lesen (`ci_manage view`), Ranking oder `pending`
  als Zeile tragen. Kein Poll.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** linie:bau
- **Lage:** Ox64 zugesagt; Presence-Hardware ungebaut.
- **Blockade:** Hardware fehlt; Bau gehört zur bau-Linie.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### SSDC Limadou
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Konto existiert, CAS-Login lädt; „Permission Denied" = fehlende
  PI-Freigabe; PI bat um Wartezeit.
- **Blockade:** PI-Freigabe ausstehend; wir warten.
- **Braucht:** Wiedervorlage (Trigger: Prozedur-Update).

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** App hat Kontozugriff (`read:user`/`user:email`).
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen?

### SuperDARN
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:21` (`blocked account`); Route über Globus +
  PI-Vereinbarung.
- **Blockade:** kein Konto.
- **Braucht:** Operator-Wort Konto; Session liest die PI-Vereinbarung vorab.

### solar-system-open-data REST
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:60` (`blocked key`); HTTP 401.
- **Blockade:** kein Token.
- **Braucht:** Operator-Wort Konto/Token.

### Amentum Developer
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:65` (`blocked account`); Reg. 200.
- **Blockade:** keine Registrierung.
- **Braucht:** Operator-Wort `developer.amentum.io/register`.

### Split-Routing-Verifikation
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 8 `000`-Hosts ungemessen.
- **Blockade:** sudo + Netz.
- **Braucht:** Wort/Route `./bin/proton-exit.sh ca`.

### Cookie-Transfer
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

### Riss 4 — WGSL-KSG-Spiegel off-path (`post.md:18`)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** CPU-TE läuft über KSG (`src/mathematikerin/te.rs`), GPU-Shader über KDE —
  verschiedene Zahlen für dieselbe Reihe.
- **Blockade:** Operator-Entscheid.
- **Braucht:** Rat-Empfehlung (bauen vs. KDE lassen + Riss registrieren) →
  Operator-Wort.

## Operator-Queue (Stand folge77; einfache Sprache, Lage/Blockade/Braucht, mit Alter)

3. **Riss 4** — **Braucht:** bauen oder KDE lassen + Riss registrieren. (neu,
   `post.md:18`)
4. **Mantis-Shrimp** — bauen oder descopen. (seit 2026-09-16)
5. **SSDC** — warten, kein Follow-up. (seit 2026-09-16)
6. **ISH Chat** — App widerrufen? (seit 2026-09-20)
7. **SuperDARN** — Konto + PI-Vereinbarung? (seit 2026-09-16)
8. **solar-system-open-data** — Konto/Token? (seit 2026-09-20)
9. **Amentum** — registrieren? (seit 2026-09-20)
10. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
12. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Benchmark

- **Delegation (Folge 77):** 1 × `grind-flash` (text_review-Bau). Klasse
  „measure-Bin-Bau" — flash-first, kein pro/max-Doppel; `cargo check` 0/0.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/text_review.rs` (neu)
- `docs/handover/handover-2026-09-21-entscheid-folge77.md` (neu)
- Move `handover-2026-09-21-entscheid-folge76.md` → `archiv/` (eigene Linie, atomar)
- **nicht** angefasst: fremde `tools/harvest/*`, `tools/measure/*` (außer
  `text_review.rs`), `tools/utils/*`, `.github/workflows/*`, `phi/*`, `src/*`,
  `post.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
