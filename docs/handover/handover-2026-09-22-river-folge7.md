<!--
  title: Handover — River-Folge 7 (Stand 2026-09-22)
  session: River-Folge 7
  class: handover
  date: 2026-09-22
  sha256: c261ea9fd45ef6adc7e97f81eeb4d50035b87a4094583cc54dd482bc790b0d74
  status: live
-->
# Handover — River-Folge 7 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt: **Lage** / **Blockade** / **Braucht**.
`operator-gebunden`, `blockiert` und `wartend` werden benannt, nie dispatcht.

## Stehender Pass (gemessen 2026-09-22, River-Folge 7)

- **HEAD** beim Start `e10c6dd3` == `origin/main`; während der Session zogen fremde
  Linien nach (HEAD jetzt `f1962d32`). `git_safety --snapshot` beim Start:
  „working tree equals HEAD — nothing to record".
- **Postfach** — `state/mail/mail_ledger.φ` trägt **keine `river`-Zeile**; neue
  Eingänge (Globus/SuperDARN, GitHub-OAuth/2FA) sind Maschine/Adresse →
  mycelium/future. `post.md` trägt fremde Zeilen (mycelium/mountain) — keine an River.
- **`register_lookup --open`** — 616 offene Zeilen über 119 Docs, **kein
  `owner=river`**; River-Bezug nur die eigene Übergabe. (26 released, 16 zustand
  due fremd; pipeline: ledger 4 / index 31 / sources 1 / witnesses 4 / footprints 2
  / nrs 1.)
- **`open_points_check` folge6** — 11 Pfad-Refs, 4 „absent": 3 Brace-Globs
  (`phi/*.φ`, `{shaders,te}.rs`, `{index,ledger}.φ`, vom Check nicht expandiert)
  + 1 echter (Zeile 105, die vollzogene folge5-Archivierung) — kein stale Punkt.
- **CI** — Watchdog-Snapshot `2026-09-22T03:15` + `ci_manage list`: `te-gate`/
  `ci-check`/`hyperscanning-te` pending/in_progress, `allwise-cdn`/`ps1-cdn`/
  `health-check` in_progress, `quake-feeds-cdn` failure. Kein Poll.

## Offen (aufgeschlüsselt)

**Keine offenen Punkte der River-Linie.** Die zwei `operator-gebunden`-Punkte aus
folge6 sind mit dem Operator-Wort „lass uns die beiden punkte fixen" (2026-09-22)
geschlossen und gebaut (Rat 2026-09-22, einstimmig):

- **Linien-Umbenennung — Rest der Routing-Vokabel.** Fassung (b): die lebende
  Mycelium-Übergabe (`handover-2026-09-21-ernte-folge136.md`) ist Myceliums eigener
  Move — als `An mycelium:`-Post übergeben (post.md). `docs/concepts/docs-naming.md`
  auf die fünf Stimmen-Namen gesetzt; die Alt-Namen-Klasse in `external-state.md`,
  `dropped-baseline.md`, `tools-map.md` auf die Stimmen-Namen gezogen (Prosa-Form,
  kein nicht auflösender Slug). Gate: `check_line_routing` scannt jetzt auch den
  **Pfad** (Alt-Slug im lebenden Pfad blockiert, Archiv befreit) + zwei Tests.
- **format-Drift.** Fassung (b): lokales `cargo fmt -- <eigene Pfade>` als
  erlaubter Syntax-Schritt (AGENTS.md + zwei `opencode.json`-Muster); das
  CI-`cargo fmt --check`-Gate (seit `72f02d66`) bleibt das Netz, nacktes
  `cargo fmt` (fremde Dateien) bleibt strukturell verweigert.

Der `An mycelium:`-Post (Übergabe-Umbenennung) ist Myceliums Schritt, nicht Rivers;
er steht in `post.md`.

## Benchmark

- **`council`** (Architektur, pro/max) — Verdikt einstimmig, kein Dissens. Zwei
  Spannungen benannt statt geglättet: das Architektur-Wort lizenziert nicht den
  Grenzübertritt (Write-boundary steht über der Lizenz); der Format-Weg vertraut
  dem Schreiber, während das CI-Gate strukturell bleibt. Prämisse korrigiert
  (A = A): „kein Gate" war halb wahr — `cargo fmt --check` ist seit `72f02d66`
  gebaut, es fehlte die Heilung. Die Alt-Namen-Klasse wurde vollständig gemessen
  (nicht nur die eine sichtbare Zeile).
- Kein Doppel-Lauf: die Klasse „Architektur-Verdikt" trägt den `council` als
  Vertreter; die Umsetzung ist Register-/Gate-Arbeit und trägt die Session selbst.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-river-folge7.md` (neu)
- Move `docs/handover/handover-2026-09-22-river-folge6.md` → `docs/handover/archiv/`
- `docs/handover/post.md` (eine Zeile `An mycelium:`)
- `docs/concepts/docs-naming.md`, `docs/concepts/tools-map.md`
- `docs/zustand/external-state.md`, `docs/zustand/dropped-baseline.md`
- `AGENTS.md`, `opencode.json`, `src/gate/commit_gate.rs`

Fremd, **nicht angetastet:** `src/mathematikerin/{shaders,te,tests}.rs`,
`src/archivar/{hdf5,babamul,quaoar_occlt}.rs`, `phi/pipeline/{index,ledger}.φ`,
`phi/{sources,blocked_sources,footprints}.φ`, `.github/workflows/*`,
`tools/utils/src/bin/archive_search/*`, `tools/harvest/src/bin/babamul_compiler.rs`,
`tools/measure/**` und alle nicht genannten Pfade.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
