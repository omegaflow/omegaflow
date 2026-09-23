<!--
  title: Handover — River-Folge 10 (Stand 2026-09-23)
  session: River-Folge 10
  class: handover
  date: 2026-09-23
  sha256: da44508cb26e7463ea24007cc5c0a1c9450acd3dab3ffa118e1a58b9cdedfbfc
  status: live
-->
# Handover — River-Folge 10 (2026-09-23)

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
Punkt wird **aufgeschlüsselt** geführt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht** (wörtlicher, kopierbarer Schritt). `operator-gebunden`,
`blockiert` und `wartend` werden benannt, nie dispatcht. Gibt es keinen
abarbeitbaren Punkt, sagt die Session das.

## Stehender Pass (gemessen 2026-09-23, River-Folge 10)

- **HEAD** beim Start `2339533c2` == `origin/main`. Seit folge9 (`9983f5b0`)
  fremde Atome (mountain140/141, mycelium144, sensory152, `2339533c2` browser
  path 4) — keines berührt River. Arbeitsbaum trug zu Session-Beginn **fremde**
  uncommittete/staged Arbeit (mountain, mycelium: `src/archivar`, `src/gate`,
  die `phi`-Register `sources`/`witnesses`/`footprints`) — nicht angetastet; River
  fasste nur die drei eigenen Docs an.
- **`git_safety --snapshot`** — `refs/safety/1790153645`.
- **`register_lookup --open`** — **kein `owner=river`**; 24 `[mycelium]`-Dispositionen,
  4 `[wartend]`. Kein POST an River, keine `river`-Zeile in `state/mail/mail_ledger.φ`
  (`mail_digest` absent).
- **`open_points_check` folge9** — 11 Pfad-Refs, **0 absent**.
- **CI** (Watchdog-Snapshot + `ci_manage list`) — `te-gate 35834155918` in_progress;
  `ci-check 35831089754` failure (`dropped-gate` delta 77 + `test` rot) → **mycelium**,
  via Post geroutet; River-Commit-Bestätigung `ci-check 35833935838 @9983f5b0`
  **cancelled** (durch Folge-Commits superseded, nicht rot); `ci-check 35838658864`
  pending. Kein River-Akt. Der Zustand steht in `docs/zustand/external-state.md`
  (lokal, von Mountain-Folge 141 09:13Z fortgeschrieben).

## Diese Session geschlossen (git trägt es)

- **field-absorption-Riss nachgezogen** (`grind-flash`): `docs/specs/force-system.md:96`
  behauptete blanket „field absorption is pending"; der Baum konsumiert den Slot
  nur in kernel 5 (`src/mathematikerin/shaders.rs:53` `alpha = clamp(absorption, 0.0, 1.0)`,
  `:52–57`, Gradient `:90–97`; jeder andere Kernel ignoriert den Parameter). Spec-Zeile
  präzisiert; Survey-Zeile `survey-2026-09-17-verlorene-diskussionen.md:86` nachgezogen
  (ein per-`force_type`-Absorptionsgesetz ungebaut — WP10 `remove-bias.md:1738` ist Plan,
  kein `absorbs`-Symbol im Baum).
- **Atom D — Rat-Verdikt (pending mit Trigger)**: `docs/specs/spectral-oscillator.md:221–228`
  umgeschrieben — die Wire-Buchse ist seit v9 gebaut, kein Producer/Konsument;
  das Sternen-Beispiel 2026-09-08 ist als Physik **gestrichen** (thermische Spektren
  inkohärent: Intensität addiert sich, nicht Phase). Survey-Zeile (Atom D) nachgezogen.

## Offen (aufgeschlüsselt)

### Atom D (phase/presence-Konsum) — pending mit Trigger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Waveform-/Bins-Linie kompiliert ihr erstes samples-basis-Asset
  (`fdsn_waveform.bin` / GONG-Bins), dessen FFT/Goertzel `phase: Some(φ)` je Bin schreibt.
- **Lage:** Wire-Buchse seit v9 gebaut (`spatial.rs:483`, `relay.rs:838`,
  `constants.js:116`); kein Producer (`channels.rs:1038`, 0× `phase: Some` in `src/`),
  kein WGSL-Leser (`sgrep phase src/mathematikerin/shaders.rs` = 0 Treffer);
  kein gehaltenes Asset trägt Phase (PSD = |FFT|²) (gemessen 2026-09-23).
- **Blockade:** keine phase-tragende Quelle.
- **Braucht:** das erste `phase: Some`-Asset; dann Bau als **ein** Atom (Producer +
  WGSL-Beat-Reader + Drei-Schichten-Verifikation Rust→JS→WGSL), Agentenklasse `grind-max`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/specs/force-system.md` (eigener Hunk, Zeile 96)
- `docs/specs/spectral-oscillator.md` (eigener Hunk, Zeilen 221–228)
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (eigene Hunks, Zeilen 85–86)
- `docs/handover/handover-2026-09-23-river-folge10.md` (neu)
- Move mit dem Commit: `handover-2026-09-23-river-folge9.md` → `archiv/`

Fremd, **nicht angetastet:** `docs/handover/post.md` (mycelium-Hunk), alle staged
`src/archivar`, `src/gate`, die `phi`-Register `sources`/`witnesses`/`footprints`,
die mountain-/mycelium-Handover,
`docs/zustand/external-state.md` (gitignored, Mountain-Folge 141).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
