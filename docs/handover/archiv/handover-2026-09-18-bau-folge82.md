<!--
  title: Handover — Bau-Folge 82 (Stand 2026-09-18)
  session: Bau-Folge 82
  class: handover
  date: 2026-09-18
  sha256: 2dc09e8388fa43091f751452412e4042e907f277dbdf963f8abf685d1041a473
  status: live
-->
# Handover — Bau-Folge 82 (2026-09-18)

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
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18, HEAD 427f029a)

- **Postfach** — `post.md` leer (keine `An bau`-Zeile); `state/mail/mail_ledger.φ`
  letzter Eingang `1789718159` (SuperDARN-Canada, kein Agenten-Eingang).
- **CI am HEAD** — `fugin-cdn` `35324143526` **success** @`3c3de485`; `ci-check`
  `35324223604` @`427f029a` cancelled (Folgelauf verdrängt); letzter abgeschlossener
  `ci-check` `35317922675` @`d8a8ed3e` **rot** (41 Tests + clippy + rustfmt — in
  diesem Atom gefixt, Verifikation ausstehend); `te-gate` `35324015019` pending
  @`fb6b62b4`; `planetary-odf-cdn` `35321177747` pending @`1a09d8e9`.
- **HEAD** `07c8d6a7` (== `origin/main`, nach den ernte-Commits
  `edecb965`/`4554c40d`/`07c8d6a7` während dieser Session); Sicherheitsnetz
  `refs/safety/1789719854` (Session-Start @`427f029a`).
- **Zustand-Ledger** `docs/zustand/external-state.md` ist fremd-uncommittet
  geändert — nicht angefasst, hier nur benannt.

## Red main — 41 Tests + clippy/rustfmt (gefixt, CI-Verifikation ausstehend)

- Ursachen gemessen (`explore`, Log `/tmp/opencode/cicheck.log`): 37 `archivar`-Tests
  **real at HEAD**, kein gemeinsamer Helfer. Cluster + Fix (alle am Baum):
  - `ee4db3aa`-Regression `mat5`/`matfile`/`zarr` (15): absolutes `Vec::resize`
    statt relativem Padding (`mat5.rs:225`, `matfile.rs:407,488,547`) + LZ4-Overlap
    (`zarr.rs:202` byte-weiser Kopier) — production.
  - `las/mod.rs:835` `Vec::with_capacity` → `vec![0u8; …]` (4).
  - `galileo_odr`/`voyager_odr` (7): `1e-12`/`1e-9` unerreichbar bei |TDB|~1e8 →
    ULP-Toleranz `8·f64::EPSILON·|t|`.
  - `atdf.rs:1133,1140` `set_field` LSB→MSB (`extract` liest MSB-first) (4).
  - `vtscat.rs:135` `parse_sexagesimal` skaliert h-Wert ganz (2) — production.
  - `mariner_occlt.rs:133` `DAY_BASE_S` 1497→1496 (`ymd_to_days` 0-based; Epoche
    1974-02-05) (2) — production.
  - `tests.rs` stale Register-/Port-Erwartungen (3) + octal-escapes `\0`→`\x00`.
  - `igra.rs:152` `too_many_arguments` → `DataLineFields`-Struct.
  - rustfmt `omega_sh.rs`/`harvest_reg.rs`.
- `cargo check --workspace` 0 Fehler / 0 Warnungen. (Schritt: nach dem Push
  `ci-check` **einmal** lesen; grün → geschlossen, rot → Restzelle ins Handover.) · `wartend`

## FUGIN — 269 Cubes offen (Pilot geschlossen)

- Pilot `fgn00000000.sky1` via `fugin-cdn` `35324143526` **success** manifestiert;
  `phi/blocked_sources.φ`-Eintrag fortgeschrieben. (Schritt: die restlichen **269
  Cubes** je Messung registrieren + dispatchen — `fugin-cdn.yml`-Inputs `url`/`asset`;
  TAP-Liste 270: 89×12CO, 89×13CO, 92×C18O.) · `pending`

## TE-Gate — Gate-Verdikt + Rename (c)

- `35324015019` @`fb6b62b4` pending. (Schritt: `ci_manage view 35324015019` einmal
  lesen; grün → Rename `arx_restricted_surrogate_conditional`; rot → rote Zelle +
  FN-Arm `found/meas ≥ 0.5` beider Richtungen, Design verbreitern.) · `wartend`

## planetary-odf-cdn — Verdikt

- `35321177747` @`1a09d8e9` pending. (Schritt: `ci_manage view 35321177747` einmal
  lesen; grün → geschlossen.) · `wartend`

## Offen (kein Handlungsschritt)

- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Benchmark

- Diagnose der 41 roten Tests → `explore` (flash-Klasse); Fixes → `grind-pro`
  (production: mat5/matfile/zarr/vtscat/mariner/igra) + `grind-flash` (Test/Lint).
  Flash-first; kein Doppellauf — die Parser-Klasse (NUL-CSV, folge77) und die
  Routine-Klasse (8-Profil-Messung 2026-09-16) sind gemessen entschieden.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `src/archivar/atdf.rs`, `galileo_odr.rs`, `igra.rs`,
  `las/mod.rs`, `mariner_occlt.rs`, `mat5.rs`, `matfile.rs`, `tests.rs`,
  `voyager_odr.rs`, `vtscat.rs`, `zarr.rs`,
  `tools/utils/src/bin/{harvest_reg,omega_sh}.rs`, `phi/blocked_sources.φ`
  (FUGIN-Note), `docs/handover/handover-2026-09-18-bau-folge82.md` (neu),
  Move `handover-2026-09-18-bau-folge81.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, die
  entscheid-Handover-Moves (`handover-2026-09-16-{entscheid-folge24,forschung-folge44,
  forschung-folge51}.md`), `handover-2026-09-18-entscheid-folge{48,49}.md`,
  `src/archivar/hdf5.rs`, `tools/utils/src/bin/hdf5_reader.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
