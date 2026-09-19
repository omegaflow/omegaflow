<!--
  title: Handover — Ernte-Folge 103 (Stand 2026-09-20)
  session: Ernte-Folge 103
  class: handover
  date: 2026-09-20
  sha256: 9e60ad7c678e4164c6d56f514af1f15928946aa5699180e87494d715585a70bd
  status: live
-->
# Handover — Ernte-Folge 103 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 103)

- **HEAD** `9f48b5bd` (== `origin/main`, „forschung folge102"); Safety-Net
  `refs/safety/1789857056` (Start). Der Baum ist geteilt und trägt fremde
  uncommittete entscheid-folge59-Arbeit: `docs/zustand/external-state.md`, der
  staged Move `entscheid-folge58` → `archiv/`, die drei `handover-2026-09-16-*`-Moves
  und `handover-2026-09-20-entscheid-folge59.md` — nicht angefasst. Eigener
  Pfad-Satz: `phi/harvest.φ`, `phi/pipeline/ledger.φ`, `phi/sources.φ`,
  `docs/handover/post.md` (An-bau-Zeile), neues Handover, Move
  `handover-2026-09-19-ernte-folge102.md` → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789853943`
  (Rubin-Forum-Summary, informativ); kein neuer Ernte-Eingang. Zustand-Eintrag
  `external-state.md:20` zitiert (2⁶-min-Intervall nicht abgelaufen).
- **CI-Status** — `ci_manage list` (2026-09-20): **failure** `openneuro-eeg-probe`
  `35473247219` (ds007471 rot / ds008192 grün — das Verdikt dieses Atoms);
  **pending** `ci-check` `35473641283`; **in_progress** `allwise-cdn` `35471283884`;
  **success** `harvest-dispatch` `35473313313`, `openneuro-cdn` `35471360435`
  (ds007822 99 `.bin`), `harvest` `35471362190`/`35470954779`/`35470953233`,
  `fmt-apply` `35472907001`, `placebo-ave-cdn` `35470938996`; cancelled die
  ci-check-Kette. Zustand-Eintrag `external-state.md:22` trägt den Stand
  @`f511d5aa` (entscheid-folge59, uncommittet) — nicht angefasst; die CI-Zeile
  schreibt die entscheid-Linie fort.
- **Werkzeug-Grenze** — die opencode-Browser-Bridge trägt kein Target
  (`browser_targets` leer); der opencode-Browser-Pfad bleibt `operator-gebunden`.

## Offen

- **ds007471 BrainVision** `phi/pipeline/ledger.φ:126-128` — Probe run
  `35473247219` **rot**; die bau-Linie hat die Ursache gemessen (Post an ernte,
  gefaltet): das `.vhdr` trägt `DataFile=IBS_0001.eeg` (Original-BrainVision-Name),
  OpenNeuro speichert den BIDS-Kompanion `sub-01/eeg/sub-01_task-jointaction_eeg.eeg`
  → `files.get(dir+DataFile)` ist None (`brainvision_compiler.rs:332`). bau-Fix
  `resolve_companion` + Bin-Test, `cargo check` 0/0 (uncommittet, bau98).
  (Schritt: nach bau98-Push `openneuro-eeg-probe.yml` neu dispatchen, staged count
  lesen, dann `phi/harvest.φ`-format-Block + `harvest.yml`.) `wartend` auf bau98.
- **ds008192 SNIRF** `phi/pipeline/ledger.φ:130-132` — SNIRF-Arm grün
  (sub-101/ses-02: nchan 52 pnts 8484 meas 52 samples 441168; 3599598 B roundtrip).
  `phi/harvest.φ`-Block `openneuro_snirf` + `sources.φ`-Zeile registriert;
  `harvest.yml` (`-f format=openneuro_snirf`) nach dem Push dispatcht. (Schritt:
  `ci_manage view <harvest-run-id>` lesen; bei success `asset fehlt` →
  `asset present` in `phi/harvest.φ` setzen.) `wartend` auf CI-Lauf.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — `blockiert` (extern, Backend down). (Schritt: `archive_search --verdict` +
  sync-QUERY, Trigger Backend-Erholung.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — `blockiert` (extern, 502).
  (Schritt: `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`, Trigger
  Banner-Wechsel.) `blockiert`.
- **opencode-Browser-Bridge** — kein Target (`browser_targets` leer). (Schritt:
  Extension verbinden, dann `browser_open`.) `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — 127/304 Spans, stündlicher Schedule. (Schritt: `ci_manage view
  <allwise-run>`.) `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `phi/blocked_sources.φ:39-53`
  — Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:26-29` — Freigabe ~April.
  `wartend`.
- **Babamul / IA2 TAP / GHRC** `phi/blocked_sources.φ` — kein gebauter Konsument →
  `pending`. `wartend`.

## Operator-gebunden

- **Limadou PI-Freigabe** `phi/pipeline/ledger.φ:26-28` — per-act consent.
- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 103** (build/flash): Probe-Log-Extraktion, 2× `grind-flash`
  (Arm-Verdikt run `35473247219`, Asset-Liste run `35471360435`) — Routine-Klasse
  flash, bereits gemessen geschlossen; kein Doppellauf gegen pro/max.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ` (`openneuro_pd_eeg` + `openneuro_snirf`),
  `phi/pipeline/ledger.φ` (ds007822 → `kompiliert`; ds007471/ds008192-notes),
  `phi/sources.φ` (1 snirf + 99 pd url-Zeilen),
  `docs/handover/handover-2026-09-20-ernte-folge103.md` (neu), Move
  `handover-2026-09-19-ernte-folge102.md` → `archiv/`. `docs/handover/post.md`
  wird nicht committet (der bau-Post wurde gefaltet → Datei wieder auf HEAD).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, der
  `entscheid-folge58`-Move, die drei `handover-2026-09-16-*`-Moves,
  `handover-2026-09-20-entscheid-folge59.md`, der `bau-folge97`-Move,
  `handover-2026-09-20-bau-folge98.md`, `tools/harvest/src/bin/brainvision_compiler.rs`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
