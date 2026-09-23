<!--
  title: Handover — River-Folge 11 (Stand 2026-09-23)
  session: River-Folge 11
  class: handover
  date: 2026-09-23
  sha256: 73f59ef587728d4ca2e37581161d58f3283781e9f870cd8cd4ba759940b86808
  status: live
-->
# Handover — River-Folge 11 (2026-09-23)

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
`blockiert` und `wartend` werden benannt, nie dispatcht.

## Stehender Pass (gemessen 2026-09-23, River-Folge 11)

- **HEAD** beim Session-Ende `bbf519150` (`mycelium 145`) == `origin/main`. Beim
  Start stand `35fc5655d` (river folge10); dazwischen fremde Atome (`3d7f82036`
  mountain141, `bbf519150` mycelium145). Der Arbeitsbaum war zu Session-Beginn
  **dirty** mit fremder Arbeit (`src/archivar/extract.rs`, `tests.rs`,
  `demeter_harvest.rs`, `ps1-cdn.yml`, `post.md`, mountain-Handover-Move) und wurde
  **während** der Session von den fremden Linien committet — keine River-Berührung.
- **`git_safety --snapshot`** — `refs/safety/1790159521`.
- **`register_lookup --open`** — **kein `owner=river`**; kein POST an River; keine
  `river`-Zeile im Mail-Ledger. Postfach (extern): letzter Eingang `1790116573`
  (LSST-Forum-Summary, Maschinendigest, keine Aktion) — kein River-Akt.
- **`open_points_check` folge10** — 18 Pfad-Refs, **0 absent**.
- **CI** (Watchdog-Snapshot + `ci_manage view/log`) — `ci-check 35838658864`
  @`2339533c2` **failure**: `dropped-gate` **delta 92** (baseline 2680 | current
  2772), `test` rot — Klasse mycelium (geroutet); `tools-build 35844365704`
  failure (release-asset-404, `tools-build.yml:41–64`, `An mycelium:` in
  `post.md`); `te-gate 35834155918`, `health-check 35813434762`,
  `ci-check 35844564961` in_progress. Kein River-Akt. Zustand in
  `docs/zustand/external-state.md` (gitignored; nur die eigene CI-Zeile ergänzt).

## Diese Session geschlossen (git trägt es)

- **Blockade-Trigger gefeuert.** Die fremde uncommittete `src/archivar/extract.rs`
  wurde von Mycelium 145 committet (`bbf519150`). Damit fiel die Atom-D-Blockade;
  der Bau wurde in derselben Session ausgeführt.
- **Rat — Atom D nicht aufspalten.** Verdikt (pro/max): **ein Atom**; Konsens
  Berg/Fluss/Myzel/Sensorik/Zukunft — die Spec trägt „one atom, no split"
  (`spectral-oscillator.md:261/294`); ein konsumentenloser Producer ist der
  0-Kanon-Zustand `pending`, kein gebauter Node — und die Gates sähen den Split
  nicht (`src/lib.rs`, `pub fn` → `cargo check` 0/0 schweigt).
- **Atom D — Route B gebaut (`grind-max`, ein Atom)**: Producer
  `odf.rs::carrier_phase_rad`/`TnfPhaseRow`/`tnf_phase_series` (`phase =
  Some(fract(cycles)·2π)`; `freq = ramp_freq` aus der PODF-Zeile, nie hartkodiert;
  `bin_width = 0.0` null-echt; absent-Phase → `None`); Phase-Arm
  `extract.rs::sample_phase`/`phase_series_parse_bin` + `channels.rs:1038`
  (`phase: sample_phase(...)`, Diskriminator `ul_phase_cycles`); Serien-Bau
  `main_flow.rs` schreibt jetzt `freq`/`bin_width` aus der Zeile statt `0.0`
  (kein `types.rs`-Eingriff — `Channel.freq/bin_width` existierten bereits);
  WGSL `shaders.rs::beat_pair` (Paar-Gates: beide presence/ν>0, gleiche Force,
  `Δν·dt < 0.5`; Term `A_a·A_b·cos(2π·Δν·t + Δφ)`). Drei-Schichten:
  5+2 Rust-Tests, `static/constants.test.mjs` (3 JS-Fixtures, in
  `.github/workflows/ci-check.yml` registriert), 5 CPU-Mirror-WGSL-Tests; naga-
  Validierung auf CI. `cargo check` + `cargo check --tests` **0 Fehler / 0
  Warnungen**. Spec-Status `spectral-oscillator.md:221ff` auf „built" gezogen
  (sha256 `f6020c77…`).
- **Beat-Paar-Recherche** (`general`/flash, `archive_search`): für die
  Zweistationen-Open-Loop-Variante **gemessene Absenz** (`--mwmbl`/`--ads`/
  `--zenodo`/`--github`/`--heasarc`, kein Roh-Asset); für „zwei kohärente Töne in
  einem Band" konkrete **Dual-Comb**-Kandidaten (Zenodo, open Dataset:
  `6413816` HDF5 verifiziert, `2542265` RF-Beatnote, `6451876`, `10709448`,
  `15095848`) — optisches Band, nicht DSN-RF. Als `An mycelium:`-Zeile geroutet
  (`post.md`), Force-Gate-Klassifikation nötig.

## Offen (aufgeschlüsselt)

### Beat-Paar (Atom-D-Fortsetzung)
- **Status:** wartend | **Bindung:** dritter (Datenerhebung) / linie:mycelium
- **Trigger:** eine Zweistationen-Open-Loop-Aufnahme eines Trägers, oder ein
  gehaltenes Asset mit zwei kohärenten Tönen in einem Band.
- **Lage:** kein gehaltenes Asset trägt zwei zueinander kohärente Oszillatoren im
  selben Band (gemessen 2026-09-23, Rat + `archive_search`-Recherche). Der WGSL-
  Beat-Term ist gebaut und feuerfähig (zwei Presence+ν>0-Slots), aber kein
  Datensatz liefert das Paar; die Dual-Comb-Kandidaten liegen bei mycelium zur
  Force-Gate-Prüfung.
- **Blockade:** keine Quelle mit Paar; Dual-Comb-Klassifikation offen.
- **Braucht:** mycelium prüft die Dual-Comb-Kandidaten gegen Force-Gate/Registry
  (`post.md`-Zeile) und registriert/verwirft; sonst `archive_search --all`/
  `--mwmbl` nach einer Zweistationen-Aufnahme.

### Atom D — CI-Verifikation der neuen Tests
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das Commit-Wort des Operators (`/commit`) → der `ci-check`-Lauf
  über die neuen Rust-Tests, `constants.test.mjs` und die naga-WGSL-Validierung.
- **Lage:** lokal `cargo check`/`--tests` 0/0; der GPU-Fixture-Lauf des Beat-Terms
  bleibt offen (nur naga + CPU-Mirror lokal, gemessen 2026-09-23).
- **Blockade:** keine.
- **Braucht:** nach dem Commit `ci_manage list` → den neuen `ci-check`-Lauf mit
  `ci_manage log <id>` lesen; bei rot die vier neuen Test-Schichten prüfen. Ein
  GPU-Kalibrier-Fixture-Lauf (`mathematikerin/tests.rs`/`omega.rs`) wäre der
  nächste Schritt auf CI.

## Benchmark

- **Architektur-Verdikt Atom D:** Rat (pro/max) — die benannte harte Klasse
  (WGSL/4D-Kontrakt), kein flash-Ersatz.
- **Atom-D-Bau:** `grind-max` — die benannte härteste Bau-Klasse (novel Producer +
  WGSL 4D), kein Doppel-Lauf.
- **Beat-Paar-Recherche:** `general`/flash — Routine-Recherche, flash-first,
  kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/odf.rs`, `src/archivar/extract.rs`, `src/archivar/channels.rs`,
  `src/archivar/main_flow.rs`, `src/archivar/tests.rs`
- `src/mathematikerin/shaders.rs`
- `.github/workflows/ci-check.yml`, `static/constants.test.mjs` (neu)
- `docs/specs/spectral-oscillator.md` (Atom-D-Status)
- `docs/handover/post.md` (nur die `An mycelium:`-Zeile)
- `docs/handover/handover-2026-09-23-river-folge11.md` (neu; folge10 → `archiv/`)
- `docs/zustand/external-state.md` (nur die eigene CI-Zeile; gitignored)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
