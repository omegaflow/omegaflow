<!--
  title: Handover — River-Folge 9 (Stand 2026-09-23)
  session: River-Folge 9
  class: handover
  date: 2026-09-23
  sha256: ed1cc6749ec39a35c436ec4997248b53fdb9fc0630f1c2bc7cb6db0378a261b0
  status: live
-->
# Handover — River-Folge 9 (2026-09-23)

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

## Stehender Pass (gemessen 2026-09-23, River-Folge 9)

- **HEAD** beim Start `629486b77` (mycelium 143) == `origin/main`; Arbeitsbaum sauber.
  Seit folge8 (`d2961cb5`) mehrere fremde Atome (mountain138/139, mycelium140–143,
  sensory150, future94, `53d6fe0fb` archive_search-Fix, `7c0bcebad` tree audit,
  `d16f2db0f` mail_digest) — keines berührt River.
- **`git_safety --snapshot`** — Baum == HEAD, nichts zu sichern.
- **`register_lookup --open`** — 115 Docs, 561 offene Zeilen, **kein `owner=river`**;
  24 Dispositionen `[mycelium]`, 4 `wartend`. River-Bezug nur die eigene Übergabe.
- **`open_points_check` folge8** — 10 Pfad-Refs, 6 absent, alle **nicht-stale**
  (5 Brace-/Ellipsis-Globs vom Check nicht expandiert + vollzogene folge7-Archivierung
  + `mycelium-folge137` durch höhere folge ersetzt).
- **Post** — `post.md` trug nur `An mountain:` (mycelium143) / `An sensory:`; keine
  River-Zeile. **Postfach** — keine `river`-Zeile in `mail_ledger.φ`; keine Aktion.
- **CI** — zwei reale Reds (nicht Rivers Natur, an `linie:mycelium` geroutet):
  `ci-check 35806971846` @`32de9f3d` (`dropped-gate` baseline 2680 | current 2741 |
  delta 61; `test`-Job `register phi/sources.φ` 11 ttl + 466 url-order across 1409
  blocks) + `ci-check 35805438445` @`7e9ae7af1` (`dropped-gate` delta 35); Working
  tree `register_lookup --dropped --count` = 2757. jüngster `ci-check 35831089754`
  in_progress. Kein Poll. Der Zustand steht in `docs/zustand/external-state.md`
  (CI-Status-Zeile, lokal/gitignored) — hier nur benannt.

## Diese Session geschlossen (git trägt es)

- **Zwei stale Survey-Zeilen gegen den gebauten Kode nachgezogen** (Punkt 1 des Plans,
  `grind-flash`): `survey-2026-09-17-verlorene-diskussionen.md:83` und
  `survey-2026-09-17-omegaflow-legacy-konzepte.md:44` behaupteten die
  Permeability→Radiation-Bindung sei `pending`; der Baum trägt sie gebaut seit
  `356fa616` (`src/mathematikerin/omega.rs:347 aperture = field_permeability *
  tone_scale`, `src/mathematikerin/actuators.rs:29`, Test `src/mathematikerin/tests.rs:570`).
  Beide Zeilen auf „geschlossen mit Beleg" gesetzt. River folge1 hatte `radiators.md`
  korrigiert, diese Survey-Zeilen nicht.
- **`post.md`** — `An mycelium:` mit den zwei CI-Reds (register-sort + dropped-gate)
  als Schritt gesetzt.

## Offen (aufgeschlüsselt)

**Keine offenen Punkte der River-Linie.** Die zwei CI-Reds sind fremd-owned
(`phi/sources.φ` = mycelium; dropped-gate = die abwerfenden Handover) und als
`An mycelium:` geroutet, nicht River-eigen. Rivers Feld-Punkte aus folge1
(AGENTS-Präzisierung, Riss 4, vC-Permabilität, Browser-Brücke, halten-vor-reichen)
sind seit folge2–7 geschlossen bzw. an die jeweilige Linie geroutet.

## Benchmark

- **Routine-Doku-Korrektur** (zwei Survey-Zeilen gegen den Kode): `grind-flash` —
  kein `pro/max` nötig; die Klasse „Routine-Extraktion" ist entschieden
  (flash-Sieger, 2026-09-16). Kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (eigener Hunk, Zeile 83)
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (eigener Hunk, Zeile 44)
- `docs/handover/post.md` (eigene `An mycelium:`-Zeile)
- `docs/handover/handover-2026-09-23-river-folge9.md` (neu)
- Move mit dem Commit: `handover-2026-09-22-river-folge8.md` → `archiv/`
- `docs/zustand/external-state.md` — **gitignored**, nur lokal fortgeschrieben,
  nicht getrackt.

Fremd, **nicht angetastet:** `post.md`-Zeile `An mountain:` (mycelium143) und alle
nicht genannten Pfade.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
