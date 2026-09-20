<!--
  title: Handover — Bau-Folge 101 (Stand 2026-09-20)
  session: Bau-Folge 101
  class: handover
  date: 2026-09-20
  sha256: a110f78976cd3bea7bbb5ab9cf361e9135453422c901252d7f434acfe4e36a9b
  status: live
-->
# Handover — Bau-Folge 101 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `12856a9e` (== origin/main), Arbeitsbaum sauber; `git_safety --snapshot`
  → „working tree equals HEAD — nothing to record". HEAD wanderte während der
  Session (`e3c478af` → `12856a9e`, fremde Commits, nicht angefasst).
- **Postfach** — `post.md` leer, keine bau-Zeile; `state/mail/mail_ledger.φ`
  jüngster Eingang `1789906306` (Limadou-Weiterleitung), kein bau-relevanter
  Eingang.
- **CI** — Watchdog-Snapshot 16:03 + `ci_manage list`: `tools-build`
  `35516056756` in_progress / `35516059464` pending (bau100-Bootstrap, getriggert
  durch `e3c478af`); `te-gate` `35513982359` in_progress; `ci-check` `35513190719`
  in_progress, Kette cancelled; `release-build` `35513611936` failure.

## Offen

- **ci-check Rot-Zelle — Fix im Baum, ungeprüft (CI-Lauf)** — die Rot-Zelle ist
  gemessen (`ci_manage log 35510151014` @`d4c38b9e`; die Dateien sind seit folge96
  unverändert): die **clippy**-Job-Zelle (18 Lints: `needless_borrow`/
  `needless_lifetimes`/`question_mark`/`map_or_identity`/`get_first`/
  `too_many_arguments`/`needless_range_loop`/`manual_ok_err` in
  `src/archivar/{hdf5,snirf,brainvision}.rs` + `src/mathematikerin/te.rs`) und die
  **test**-Job-Zelle (17 Tests: 12× `snirf` scheitern am synthetischen H5-Fixture
  via `hdf5 parse: AbsentAtByte`, 3× `hdf5` lazy/budget, 2× `brainvision`).
  Die bau100-Deutung „Concurrency" ist widerlegt: `cancel-in-progress: false` ist
  ein **gesperrter Vertrag** (`tools/register/src/bin/concurrency_contract.rs`),
  und die 0-success-Ursache war die rote Zelle, nicht der verlorene Lauf. Fix im
  Baum (Parser-Wurzel: v1-Object-Header-Message-Area-Größe im snirf-Fixture;
  ReadLength vor TraversalBudget; `Hdf5WindowReader::forget` beim Rollback;
  head+tail statt Doppel-Fetch; brainvision channel-major). Lokal `cargo check`
  null Warnungen; **clippy/test laufen nie lokal** (funktionale Läufe gehen nach
  CI). (Schritt: nach Push `ci_manage list`, `ci_manage view <ci-check-id>`, bei
  Rot `ci_manage log <id>`.) · `wartend`
- **release-build Rot-Zelle — Fix im Baum, Dispatch ausstehend** — gemessen
  (`ci_manage log 35513611936`): der windows-latest-Matrixjob bricht in
  `src/archivar/bsp_reader/daf.rs` (E0433 `os::unix`, E0599 `read_exact_at`).
  Fix: portable `read_exact_at`-Hilfsfunktion (cfg unix/windows) in derselben
  Datei. (Schritt: nach Push `gh workflow run release-build.yml`, dann
  `ci_manage view <id>`.) · `wartend`
- **external-state CI-Zeile stale** — die CI-Zeile steht auf HEAD `ae0fce25`;
  HEAD wanderte. `docs/zustand/external-state.md` trägt fremde **gestagte**
  Arbeit (Ernte-Folge 108, Lasair-Zeile) — ein pfad-begrenzter Commit würde sie
  mitsenden, darum unangetastet. (Schritt: nach dem Push `ci_manage list`, die
  CI-Zeile am neuen HEAD fortschreiben, sobald die Datei frei ist.) · `wartend`

## Wartestellungen (kein Auswahlpunkt)

- **`tools-build` Bootstrap** `35516056756` in_progress (bau100). (Schritt:
  Watchdog-Snapshot `tools-latest`-Run lesen.) · `wartend`
- **`te-gate`** `35513982359` in_progress. · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benchmark

- **Bau-Folge 101**: Atom „ci-check Rot-Zelle". Kein Doppellauf — die Wurzel lag
  in bau's eigenem folge90/93/94/96-HDF5-Umbau, kein Routinefall. `grind-max` für
  die Parser-/Test-Wurzel (Urteil + Schreiben in einem Kontext), `grind-flash`
  für die mechanischen clippy-Lints (`daf.rs` pread, `te.rs` range-loop). Der
  te.rs-Erstversuch (grind-flash) verweigerte korrekt die nicht existente
  `manual_ok_err`-Zuordnung — die Lint-Karte im Log war um einen Diagnose-Block
  verschoben (rustc druckt `-->` vor dem `#lint`-Link); korrekt ist
  `needless_range_loop`.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/bsp_reader/daf.rs`,
  `src/archivar/hdf5.rs`, `src/archivar/snirf.rs`,
  `src/archivar/brainvision.rs`, `src/mathematikerin/te.rs`, dieses Handover
  (neu), Move `handover-2026-09-20-bau-folge100.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`,
  `phi/pipeline/ledger.φ`, `docs/handover/handover-2026-09-20-ernte-folge10{7,8}.md`
  (Ernte-Linie, gestaged), `docs/SOURCE_PORT.md`, `docs/concepts/tools-map.md`,
  `tools/utils/src/bin/archive_search*` (forschung). Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` (Pfade `src/**`) selbst; `release-build` ist tag/dispatch-only und
muss danach `gh workflow run release-build.yml` erhalten. Die Session pollt
nicht; das Ergebnis liest der nächste Pass aus dem Watchdog-Snapshot. `/consent`
ist der session-weite Consent, nie das Commit-Wort.
