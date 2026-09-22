<!--
  title: Handover — Bau-Folge 113 (Stand 2026-09-20)
  session: Bau-Folge 113
  class: handover
  date: 2026-09-20
  sha256: c8f8260119d1fadda395fc6bacee2d8b1aef6a5574a051106ffdbf7a47feb775
  status: live
-->
# Handover — Bau-Folge 113 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als Tafel (Punkt | Status | Bindung |
Schritt); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `8218f46a` == `origin/main` → gepusht, Fast-Forward. Arbeitsbaum trägt
  fremde uncommittete Arbeit (`handover-…-entscheid-folge66.md`, der
  `folge65`-Move,
  `.github/workflows/free-model-bench.yml` + `tools/measure/src/bin/free_model_bench.rs`)
  — **nicht angefasst**.
- **Postfach** — kein bau-relevanter Eingang; `post.md` trägt Zeilen an *ernte* und
  *entscheid*. Letzter Ledger-Eingang `1789930255` (Pine64 `info@`: Ox64-Zusage,
  von Forschung-Folge 123 beantwortet, `sent_ledger 1789931195`).
- **CI** — `ci_manage list` 2026-09-20: `ci-check` `35531572974` @`8218f46a`
  **in_progress** (kein Verdikt); `te-gate` `35531196101` in_progress;
  `tools-build` `35531153732` **success** (Binär `02ee415e`).
  CI-Zeile in `docs/zustand/external-state.md` auf HEAD `8218f46a` fortgeschrieben.
- **Binär** — `02ee415e`, frisch (Träger `tools-build` `35531153732`).

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| `ci-check`-Verdikt am HEAD `8218f46a` | `wartend` | `termin` (Run `35531572974`) | Run einmalig aus `/tmp/opencode/ci_status.md` / `ci_manage view 35531572974` lesen; bei Rot `ci_manage log <id>`. |
| 3 `ausstehend` Queue-Korpora (30-astro, earth-stac-sentinel, exotic-neutrino-ligo; `ledger.φ:58–66`) | `blockiert` | `linie:ernte` | force-gate-B-Fix steht (`port.rs` `field_or_review`); Re-Lauf auf dem nächsten frischen Binär, `# pending`-Review-Zeilen zählen, dann Disposition SOURCE_PORT §5.4 (`post.md`). |
| 7 `verifiziert` Korpora / 368 Survivor (`ledger.φ:70–96`) | `blockiert` | `linie:ernte` | Survivor-Review/Disposition nach SOURCE_PORT §5.4; `post.md`-Zeile steht. |

Kein session-abarbeitbarer undatierter Bau-Punkt — der CI-Punkt ist Wartestellung;
beide Queue-Punkte sind an die Ernte-Linie gebunden.

## Messung dieses Atoms (kein Punkt)

- **Kernel-Riss getragen** (`src/mathematikerin/force.rs`
  `kernel_riss_thermal_diffusion_advective`): `default_kernel_for` nennt genau
  einen Kernel je Force (thermal→`exponential-decay`, diffusion→`gaussian-inverse-square`,
  advective→`patch-levy`); die live `phi/sources.φ` trägt je Force mehrere
  (thermal `exponential-decay`+`erfc`; diffusion `erfc`; advective
  `patch-levy`/`gaussian-inverse-square`/`inverse-square`/`erfc`). Der Widerspruch
  wird gemessen und **nicht geglättet** — der Test bricht bei stiller Angleichung ab.
- **force-gate B gebaut** (`src/archivar/port.rs` `field_or_review`): ein Block
  ohne `force`-Direktiv fällt nicht mehr still (`default_kernel_for("") = None` →
  Drop); er bleibt `# pending … review`. τ≤0 bleibt die korrekte Stille (0 honored).
  Gate-Test `test_port_block_without_force_directive_stays_review`.
- `cargo check --all-targets`: 0 Fehler, 0 Warnungen.

## Benchmark

- **Bau-Folge 113**: das Atom (Riss-Messung + force-gate-B-Fix + zwei Gate-Tests)
  lief auf der Session (`build`/flash). Kein pro/max-Dispatch — flash trug das
  Urteil; kein Doppellauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/mathematikerin/force.rs` (Riss-Test),
  `src/archivar/port.rs` (`field_or_review` + vier Aufrufe),
  `src/archivar/tests.rs` (Gate-Test), `docs/zustand/external-state.md` (CI-Zeile),
  `docs/handover/post.md` (Post an ernte), neues
  `docs/handover/handover-2026-09-20-bau-folge113.md`, Move
  `handover-2026-09-20-bau-folge112.md` → `archiv/`.
- **Fremd (nicht anfassen):** `handover-…-entscheid-folge66.md`, der
  `folge65`-Move,
  `free-model-bench.yml` + `free_model_bench.rs`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
