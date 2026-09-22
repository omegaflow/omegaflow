<!--
  title: Handover — Entscheid-Folge 61 (Stand 2026-09-20)
  session: Entscheid-Folge 61
  class: handover
  date: 2026-09-20
  sha256: bcf0ee3f9ebab7c39055414ef6f9c084aadd3af45f111fe946bd0d9e77d604d3
  status: live
-->
# Handover — Entscheid-Folge 61 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 61)

- **HEAD** `7d0a1272` (== `origin/main`, „post: hand the PII/LLM-budget session's
  operator-bound points to the entscheid line"). Start war `7d0a1272`; der
  Arbeitsbaum war sauber, `git_safety --snapshot` meldete „tree equals HEAD —
  nothing to record". Die vorige Übergabe (folge60) wird ins Archiv gefaltet.
- **CI** — `ci_manage list`/`view` (2026-09-20 ~13:30Z): **pending** `ci-check`
  `35513190719` @`7d0a1272` (der Lauf am HEAD); **in_progress** `release-build`
  `35513611936`, `ci-check` `35510151014` @`d4c38b9e`, `health-check`
  `35505471538`; **success** die CDN-Workflows (`quake-feeds-cdn` `35510841141`,
  `ned-cdn` `35510717063`, `ps1-cdn` `35507896609`, `allwise-cdn` `35504050725`,
  `de441-cdn-watch`, `radio-cdn-watch`, `swpc-mirror-cdn`); **failure** die
  ci-check-Kette (`35509591376`, `35505527524`, `35504694962`, `35501198690`,
  `35497506174`, `35497443641`); **cancelled** die übrige ci-check-Kette
  (per-ref-Concurrency). Kein Rot außerhalb ci-check. Zustand-Ledger
  (`external-state.md`) CI-Zeile auf `7d0a1272` fortgeschrieben.
- **Postfach** — `state/mail/mail_ledger.φ` (110 Zeilen): neue Eingänge seit dem
  letzten Stand `1789899110`: `1789900094` (Framework Auto-Reply). Vorher
  `1789899110` (S2-Key
  bewilligt), `1789899008` (CORE-Key), `1789898868` (CORE verify), `1789898286`
  (S2 „received your key request"), `1789898153` (Materials Project welcome).
  Zustand-Ledger-Postfachzeile fortgeschrieben.

## Offen

- **Kernel-Riss** (thermal/diffusion/advective) — Operator-Entscheid force-gate **B**
  liegt vor; der Riss `default_kernel_for` (`src/mathematikerin/force.rs:64-66`) vs.
  `kernel_id_for_force` (`force.rs:36-37`) ist als Widerspruch zu **messen**, nicht
  zu glätten. An forschung gepostet. **`wartend`**.
- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}, dann n setzen oder
  Driver-Design ändern; Floor bleibt. An forschung gepostet. **`wartend`**.
- **09-16-Limbo** — `entscheid24` → `archiv/` committet (folge60); `forschung44/51`
  an forschung gepostet (Dateiname trägt den Besitzer). **`wartend`** (forschung-Pass).
- `termin` — **vC-Permeabilität** (versteckter sensor-getriebener Lauf; wartet auf
  Smartwatch + Mantis-Shrimp-Sensoren). **Lasair-LSST** (API 502, Backend
  server-seitig; Trigger Banner-Wechsel). **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC (#4761801), Rubin-Review, Sonden-Antworten,
  `register_lookup`-Binary + `ci-check` (an bau gepostet).

## Operator-Queue (einmal vorlegen beim Operator-Rückkehr; einfache Sprache)

Der Operator war am 2026-09-20 anwesend; die fünf Punkte aus folge60 sind
entschieden/erledigt (force-gate **B**, vC `termin`, `register_lookup`-Symlink
gesetzt, Browser-Brücke verbunden, Limadou gesendet+korrigiert). Es verbleiben:

1. **PII-History-Rewrite** — *Lage:* die private Adresse steht in 1.472 Commits
   (Autor/Committer); HEAD ist redigiert, die Historie nicht. *Frage:* führen wir
   den Rewrite aus? *Ja:* der Auftrag `docs/auftrag/auftrag-pii-history-rewrite.md`
   läuft (destruktiv, Force-Push); *Nein:* bleibt offen.

## Quer-Linien (Post)

- `An entscheid` (PII-LLM-budget) konsumiert und gelöscht; die fünf
  operator-gebundenen Punkte sind oben gefaltet.
- `An forschung` neu: force-gate **B** + Kernel-Riss messen (Schritt in der Zeile).
- `An ernte` ×2, `An bau` ×2 unverändert (aus folge60).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge61.md` (neu)
- Move `handover-2026-09-20-entscheid-folge60.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Zeile + Postfachzeile auf `7d0a1272`)
- `docs/handover/post.md` (`An entscheid` gelöscht, `An forschung` neu)

## Benchmark

- Kein Dispatch: Registratur-/Entscheidungsarbeit der eigenen Linie (flash-Klasse,
  build-Agent). Kein Doppellauf; die harten Atome lagen bei der PII-LLM-budget- und
  der Reibungs-/Myzel-Untersuchung (folge60).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
