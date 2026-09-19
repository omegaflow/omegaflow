<!--
  title: Handover — Ernte-Folge 99 (Stand 2026-09-19)
  session: Ernte-Folge 99
  class: handover
  date: 2026-09-19
  sha256: b6a978a5d710e903f11786ec87076e9f7bec13e23a7949c00aed055c26a8c33d
  status: live
-->
# Handover — Ernte-Folge 99 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-19, Folge 99)

- **HEAD** `7aa5c23e` (== `origin/main`, „bau folge94: fold the ernte98 post into
  the handover"); Safety-Net `refs/safety/1789850905` (Start).
- **Postfach** bei Start leer (`post.md` nur Header); der `--sniff`-Riss der Folge 98
  ist bereits **an bau gefaltet** (`bau-folge94.md:43`, commit `7aa5c23e`) und in
  Arbeit (bau-Linie: `net.rs` uncommittet, `bau-folge95.md` neu) — kein Ernte-Punkt
  mehr. Eine bau-Zeile (gedi_l2a + icesat2 + OpenNeuro-Arme) in `post.md` gesetzt.
- **CI** — `external-state.md:22` zitiert (HEAD-Wechsel gemessen, `ci_manage list`
  ~20:50Z): `openneuro-cdn 35468606830` in_progress @`7aa5c23e`, `allwise-cdn
  35464786486` success (Final fehlt, 127/304 Spans), `ci-check 35468441157` pending.
  Kein neuer Ledger-Eingang seit `1789795811`.

## Offen

Kein abarbeitbarer undatierter Ernte-Punkt — die offenen Punkte sind `wartend`
(Trigger Run-Abschluss / bau), `blockiert` (extern), `operator-gebunden` oder
`termin`. Der einzige Ernte-Zugriff dieser Session war der Re-Dispatch der
ds007822-Probe.

- **OpenNeuro ds007822** `phi/pipeline/ledger.φ:122-124` — die Probe `35468307481`
  @`cbd8573a` wurde **cancelled** (Konkurrenz-Dispatch `35468312819`, ds005034-
  Idempotenz, `cancel-in-progress`); **Re-Dispatch `35468606830` @`7aa5c23e`**
  in_progress. **Schritt:** `ci_manage view 35468606830`; grün → `sources.φ`-
  Registrierung (CDN-Pflicht). `wartend`.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20` —
  `blockiert` (extern): sync-QUERY beider VOTable-`ERROR` localhost:5432 connection
  refused, Root je 200 (Re-Messung 2026-09-19). **Schritt:** Re-Check
  (`archive_search --verdict` + sync-QUERY), Trigger Backend-Erholung. `blockiert`.
- **an bau übergeben** (post.md, aus Folge 98): gedi_l2a (`phi/harvest.φ:57-65`,
  hdf5 `gather_messages`-Hang) + icesat2_atl03 (`phi/harvest.φ:75-83`, Budget
  `--limit 1` + `--skip`) + OpenNeuro ds007471 BrainVision-Arm + ds008192
  SNIRF-Arm (`phi/pipeline/ledger.φ:126-132`). `wartend` auf bau.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — Run `35464786486` success, aber Final-Asset fehlt: 127/304 Spans
  (`allwise_part_*.fp01`), `allwise_coverage.fp01` absent; stündlicher Schedule
  läuft weiter. Bei Abschluss CDN-Manifestationspflicht (`sources.φ`-Registrierung
  fehlt). **Schritt:** `ci_manage view <allwise-run>`. `wartend`.
- **Lasair-LSST** `external-state.md:23` — direct+Proton 502, Wayback 200 ohne
  Snapshot. `wartend`.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `phi/blocked_sources.φ` —
  Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:30-33` — Freigabe ~April. `wartend`.
- **Limadou PI-Freigabe** `phi/pipeline/ledger.φ:26-28` — per-act consent.
  `operator-gebunden`.
- **Queue-Korpora** `phi/pipeline/ledger.φ:82-120` — `--port` braucht das
  Operator-Wort. `operator-gebunden`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — maxTime unverändert
  `2026-07-30T23:30:00Z`. Nächste Re-Messung **2026-10-19**. `termin`.

## Benchmark

- **Ernte-Folge 99** (build/flash, Hauptsession): Register-gegen-Baum-Messung zeigte,
  dass der bestätigte Plan-Halbsatz `--sniff` bereits an bau gefaltet war
  (`bau-folge94.md:43`) — kein pro/max-Delegat nötig; die CI-/Run-Messung und
  Re-Dispatch liefen in der Hauptsession (flash). Kein neuer Klassen-Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (nur die ds007822-Note),
  `docs/handover/post.md` (eine bau-Zeile + Header-sha),
  `docs/zustand/external-state.md` (nur die CI-Status-Zeile),
  `docs/handover/handover-2026-09-19-ernte-folge99.md` (neu), Move
  `docs/handover/handover-2026-09-19-ernte-folge98.md` → `archiv/`.
- **Fremd (nicht anfassen):** `tools/utils/src/bin/archive_search/net.rs` (bau,
  `--sniff`-Fix in Arbeit), `docs/handover/handover-2026-09-19-bau-folge95.md` (neu),
  der bau94-Move → `archiv/`, `.github/workflows/measure-gates.yml` (modifiziert),
  `.github/workflows/silence-map-probe.yml` (untracked), die drei
  `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
