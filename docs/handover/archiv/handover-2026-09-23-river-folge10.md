<!--
  title: Handover — River-Folge 10 (Stand 2026-09-23)
  session: River-Folge 10
  class: handover
  date: 2026-09-23
  sha256: 43fe443932f83d32505d653877c71e37311cb0fdcdcb1b2068ef37cdf845cb44
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
- **Atom D — zweimal nachgezogen (Rat)**: `docs/specs/spectral-oscillator.md:221` trägt
  den gemessenen Endstand. Erst-Verdikt `pending`; dann widerlegte die Nachmessung
  (Taucher + Rat) die Prämisse „kein Asset trägt Phase": vier phase-tragende Klassen am
  CDN, Route B (TNF) **bau-fähig**. Das Sternen-Beispiel 2026-09-08 ist als Physik
  **gestrichen** (thermische Spektren inkohärent). Survey-Zeile nachgezogen. Der Bau
  selbst bleibt offen (blockiert am fremden `extract.rs`, s. u.).

## Offen (aufgeschlüsselt)

### Atom D (phase/presence-Konsum) — Route B baubar, blockiert am fremden `extract.rs`
- **Status:** blockiert | **Bindung:** eigen (Bau)
- **Trigger:** die fremde uncommittete Arbeit in `src/archivar/extract.rs` committet —
  dann ist Route Bs Ziel-Datei frei.
- **Lage:** Rat 2026-09-23: Atom D ist bau-fähig auf der **TNF-Route**; die Prämisse
  „kein gehaltenes Asset trägt Phase" ist gemessen widerlegt — `cassini_tnf`/`maven_tnf`
  tragen die Trägerphase (`odf.rs:454–459`) **und** `ramp_freq` in derselben PODF-Zeile
  (`odf.rs:1653–1666`); `cassini_rsr` trägt I/Q (`sources.φ:8081`), `fdsn_waveform` BHZ
  (`:110`); der RAWACF-Compiler verwirft die komplexe ACF (nur `pwr0`-Power,
  `superdarn_rawacf_compiler.rs:502`). Spec-Eintrag `spectral-oscillator.md:221` trägt
  den neuen Stand (gemessen 2026-09-23).
- **Blockade:** `src/archivar/extract.rs` trägt **fremde** uncommittete Arbeit
  (65 Zeilen `field_tau`-Refactor + `probe_classify`-Tau, ebenso `tests.rs`,
  `demeter_harvest.rs`) — Route B muss genau diese Datei anfassen; ein pfad-begrenzter
  Commit würde die Fremdarbeit unter Rivers Nachricht mitreißen. Nicht angetastet.
- **Braucht:** nach dem Fremd-Commit: `grind-max` baut Route B als **ein** Atom —
  Producer (`odf.rs` Geschwister `tnf_phase_series` + Phase-Arm in `extract.rs`:
  `Sample.phase = Some(fract(cycles)·2π)`, `freq = ramp_freq` aus der Zeile, nie
  hartkodiert, `bin_width = 0.0` null-echt), WGSL-Beat-Term (`shaders.rs`, nur für ein
  Paar), Drei-Schichten-Verifikation Rust→JS→WGSL.

### Beat-Paar (Atom-D-Fortsetzung) — neue Akquisition
- **Status:** wartend | **Bindung:** dritter (Datenerhebung)
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme eines Trägers (oder ein Asset mit
  zwei kohärenten Tönen in einem Band).
- **Lage:** kein gehaltenes Asset trägt zwei zueinander kohärente Oszillatoren im selben
  Band; ein Einzelträger ist Eichfreiheit (gemessen 2026-09-23, Rat).
- **Blockade:** keine Quelle mit Paar.
- **Braucht:** `archive_search`-Recherche/Anfrage nach einer Zwei-Stationen-Aufnahme
  (ein Träger, zwei Stationen mit unabhängigen LOs).

## Benchmark

- **Route-Verifikation** (Asset-Sniff + Compiler-Emission A–D): `grind-flash` —
  Routine-Klasse („Routine-Extraktion", flash-Sieger 2026-09-16), kein Doppel-Lauf.
- **Architektur-Verdikt Atom D:** Rat (pro/max) — die benannte harte Klasse
  (WGSL/4D-Kontrakt), kein flash-Ersatz.

## Geteilter Baum — eigener Pfad-Satz

- `docs/specs/spectral-oscillator.md` (Atom-D-Eintrag, Zeilen 221–244)
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (Zeile 85)
- `docs/handover/handover-2026-09-23-river-folge10.md` (fortgeschrieben)
- Im ersten Commit `1d23d3522` bereits getragen: `docs/specs/force-system.md` +
  der Move `handover-2026-09-23-river-folge9.md` → `archiv/`.

Fremd, **nicht angetastet:** `src/archivar/extract.rs` (65 Zeilen `field_tau`-Refactor),
`src/archivar/tests.rs`, `tools/harvest/src/bin/demeter_harvest.rs`,
`docs/handover/post.md`, die mountain-/mycelium-Handover,
`docs/zustand/external-state.md` (gitignored, Mountain-Folge 141).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
