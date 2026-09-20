<!--
  title: Handover — Bau-Folge 100 (Stand 2026-09-20)
  session: Bau-Folge 100
  class: handover
  date: 2026-09-20
  sha256: 6e36253ffeba1d25a162e7679c381d2aa2db5d241ff7ea8644acd485e91f4829
  status: live
-->
# Handover — Bau-Folge 100 (2026-09-20)

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

- **HEAD** `ae0fce25` („research: add reactome and interpro search modes;
  re-dispatch the cancelled te-gate"), Arbeitsbaum sauber; `git_safety --snapshot`
  → „working tree equals HEAD — nothing to record". HEAD wanderte während der
  Session (`7d0a1272` → `ae0fce25`), fremde Commits, nicht angefasst.
- **Postfach** — `post.md` trug **2× `An bau`** (`register_lookup`-Instrument,
  `ci-check`-Concurrency) — beide gefaltet und gelöscht; `state/mail/mail_ledger.φ`
  jüngster Eingang `1789906306` (Limadou-Weiterleitung durch den Operator),
  kein bau-relevanter Mail-Eingang.
- **CI** — Watchdog-Snapshot 16:03 + `ci_manage list` (~14:07Z): `release-build`
  `35513611936` **failure** (attempt 1, @`7d0a1272`); `ci-check`-Kette
  cancelled/failure (per-ref-Concurrency); `te-gate` `35513982359` in_progress;
  `harvest` `35515194248` success. Zustand-Zeile in `docs/zustand/external-state.md`
  auf `ae0fce25` fortgeschrieben.

## Offen

- **`ci-check` verliert Läufe (Concurrency)** — 29/36 = 80,6 % cancelled, 0
  success in den letzten 100 Läufen; vier pushende Linien reseten die Gate-Uhr
  (`concurrency: ci-check-${ref}`, `cancel-in-progress: false` ⇒ nur ein wartender
  Follower, den jeder neue Push cancelt). Queue + Glue-Period statt Loss: ein Push
  **joined** den laufenden Lauf, eine Gate-Periode pro Ref bedient die Union der
  Pushes im Fenster. (Schritt: `ci-check.yml` concurrency + Mindest-Periode;
  bau98 hat die Pfad-Filter-Tuning begonnen.)
- **`release-build` rot** — `35513611936` **failure** (attempt 1, @`7d0a1272`),
  ebenso `35302966400` failure (2026-09-18); Ursache unbenannt. Der Lauf baut nur
  das Kern-Binär `omegaflow` (`Cargo.toml:22` `default-members = ["."]`), nicht die
  Tools. (Schritt: `ci_manage log 35513611936`, rote Zelle benennen, code-seitig
  beheben, `gh workflow run release-build.yml`.)

## Wartestellungen (kein Auswahlpunkt)

- **`tools-build` Bootstrap + Verifikation** — der Push dieses Commits
  (`bin/**`-Pfad) triggert `.github/workflows/tools-build.yml`, publiziert das
  rollierende Release `tools-latest`; erst danach zieht `bin/.tools_ensure` die
  frischen Binaries und `register_lookup --open` antwortet. Bis dahin liest der
  Wrapper `pending`/installierte SHA, nie eine stille Null. (Schritt: nach Push
  `ci_manage list`/`view <tools-build-id>`; dann `register_lookup --open`.) · `wartend`
- **`te-gate`** `35513982359` in_progress. · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benchmark

- **Bau-Folge 100**: Atom „CI-Tools-Release" (Delivery-Form für die PATH-Binaries).
  Design via **`council`** (Empfehlung Option B: ein Helfer `bin/.tools_ensure`
  gegen das rollierende Release `tools-latest`, content-addressed per sha256),
  Umsetzung via **`grind-pro`** (Urteil: Staleness/SHA/Netz-Fallback). Kein
  flash-Doppellauf — der Council-Entscheid + die SHA-Logik sind die benannte
  Urteilsklasse; Sieger `grind-pro`.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `.github/workflows/tools-build.yml` (neu),
  `bin/.tools_ensure` (neu), `bin/sread` + `bin/session_burn` (neu),
  `bin/{archive_search,ci_manage,git_safety,omega_sh,register_lookup,sfetch,sgrep}`
  (auf `.tools_ensure` umgestellt, lokaler `cargo build`-Block entfernt),
  `AGENTS.md` (zwei Regelzeilen), `docs/concepts/tools-map.md` (Wrapper-Zeilen),
  `docs/handover/post.md` (2 bau-Zeilen gelöscht),
  `docs/zustand/external-state.md` (CI-Zeile), dieses Handover (neu), Move
  `handover-2026-09-20-bau-folge99.md` → `archiv/`.
- **Fremd (nicht anfassen):** `phi/harvest.φ`/`phi/pipeline/ledger.φ` (ernte-Linie
  aktiv). Nie ein nacktes `git commit`.
- **Operator-Domain (ausgeführt, nicht getrackt):** `~/.local/bin/{archive_search,
  ci_manage,register_lookup,sfetch,sread,session_burn}` → `bin/<tool>` re-pointed
  (`sgrep`/`git_safety`/`omega_sh` zeigten bereits auf `bin/`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`tools-build` (Pfad `bin/**`) — kein separater Dispatch nötig; die Session pollt
nicht, das Ergebnis liest der nächste Pass aus dem Watchdog-Snapshot. `/consent`
ist der session-weite Consent, nie das Commit-Wort.
