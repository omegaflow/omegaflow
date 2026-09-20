<!--
  title: Handover — Bau-Folge 99 (Stand 2026-09-20)
  session: Bau-Folge 99
  class: handover
  date: 2026-09-20
  sha256: 158de7f0b051dc0306ba07e2995afd0383387d147b3ced44e5ba27c3c988cfc5
  status: live
-->
# Handover — Bau-Folge 99 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `64b30f58` (== `origin/main`, „ernte folge104"; während der Session
  von `f7364835` fortgeschrieben — ernte folge104 committete den
  `openneuro_brainvision`-Block + `openneuro_snirf present` in `phi/harvest.φ`,
  `ledger.φ` und den `ernte-folge103`-Move; nicht angefasst). Safety-Net
  `refs/safety/1789860115` (Start).
- **Postfach** — `post.md` leer; `state/mail/mail_ledger.φ` jüngster Eingang
  `1789853943` (Rubin/LSST-Summary, informativ), kein bau-relevanter Eingang.
- **CI** — der Watchdog-Snapshot (2026-09-20T00:37Z) ist überholt: er nennt
  `openneuro-eeg-probe 35473247219` failure. Frisch via `ci_manage`:
  **`openneuro-eeg-probe 35474638076` success** @`bfa204ae` (beide Arme grün,
  bau98-Fix `resolve_companion` enthalten); `ci-check 35475257980` +
  `te-gate 35475226890` pending.

## Offen

- **Kein bau-eigener abarbeitbarer undatierter Punkt.** `ds007471` BrainVision
  geschlossen: Fix `resolve_companion` in `8c7b885e`; Re-Probe `35474638076`
  @`bfa204ae` grün, BrainVision-Arm 64 ch × 2318620 pnts, `.bin` 593625434 B
  roundtrip parses — der CDN-Block + Lauf `35476235446` liegt bei der ernte-Linie.
  `icesat2_atl03` (offener Marker `phi/harvest.φ` → ernte) per Post an ernte
  übergeben (args `--skip 1` → `--skip 2`, `harvest.yml force=true`); der
  bau-eigene Hunk wurde zurückgezogen, da die Datei fremd-besetzt war. Es bleiben
  nur Wartestellungen (unten).

## Wartestellungen (kein Auswahlpunkt)

- **ci-check Harvest-Bin-Tests** `35475257980` @`f7364835` — pending. · `wartend`
- **`--sniff` Partial-Hash-Fix + lazy_chunk-/folge93-Gates + TE-Gates** —
  derselbe `ci-check`-Lauf. · `wartend`
- **`te-gate`** `35475226890` @`5b406e16` — pending. · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benchmark

- **Bau-Folge 99**: Probe-Log-Extraktion → `grind-flash` (Routine-Klasse, bereits
  gemessen geschlossen — flash $0.0008–0.0017 gegen pro/max $0.0041–0.0090; kein
  Doppellauf).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `docs/handover/post.md` (An-ernte-Zeile),
  `docs/handover/handover-2026-09-20-bau-folge99.md` (neu), Move
  `handover-2026-09-20-bau-folge98.md` → `archiv/`.
- **Fremd (nicht anfassen):** die drei `handover-2026-09-16-*`-Moves (`D`+`??`),
  `phi/harvest.φ`/`phi/pipeline/ledger.φ` (ernte-Linie aktiv). Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Kein Workflow dieser
Session geändert — kein Dispatch. `/consent` ist der session-weite Consent, nie
das Commit-Wort.
