<!--
  title: Handover — Ernte-Folge 104 (Stand 2026-09-20)
  session: Ernte-Folge 104
  class: handover
  date: 2026-09-20
  sha256: e68935b308296c4d7616ce6faaa7ee5d55b2a3295df46ba380847434c24ea1e9
  status: live
-->
# Handover — Ernte-Folge 104 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 104)

- **HEAD** `f7364835` (== `origin/main`); Safety-Net `refs/safety/1789859783`
  (Start). Der Baum trägt fremde uncommittete entscheid-folge59-Arbeit: die drei
  `handover-2026-09-16-*`-Moves (`D` + `?? archiv/`) — nicht angefasst. Eigener
  Pfad-Satz: `phi/harvest.φ`, `phi/pipeline/ledger.φ`, neues Handover, Move
  `handover-2026-09-20-ernte-folge103.md` → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789853943`
  (Rubin-Forum-Summary, informativ); kein neuer Ernte-Eingang. Zustand-Eintrag
  `external-state.md` zitiert (2⁶-min-Intervall nicht abgelaufen).
- **CI-Status** — `ci_manage list` (2026-09-20): **success**
  `openneuro-eeg-probe` `35474638076` (brainvision + snirf, bau-Fix greift),
  `harvest` `35474615876` (snirf), `harvest` `35474641409`, `swpc-mirror-cdn`,
  `ned-cdn`; **pending** `ci-check` `35475257980`, `te-gate` `35475226890`;
  **in_progress** `quake-feeds-cdn` `35475632601`, `ps1-cdn` `35475033451`.
  Zustand-Eintrag `external-state.md` trägt den Stand @`f7364835` — die CI-Zeile
  schreibt die entscheid-Linie fort.
- **Werkzeug-Grenze** — die opencode-Browser-Bridge trägt kein Target
  (`browser_targets` leer); der opencode-Browser-Pfad bleibt `operator-gebunden`.

## Offen

- **ds007471 BrainVision** `phi/pipeline/ledger.φ:126-128` — Probe run
  `35474638076` **grün**: sub-01 nbchan 64 pnts 2318620 samples 148391680,
  593625434 B roundtrip parses; 32 `.vhdr` gemessen, alle `task-jointaction`.
  `harvest.φ`-Block `openneuro_brainvision` (`asset fehlt`, `shard 32`,
  pattern `^sub-[0-9]+_task-jointaction_eeg\.bin$`) + Ledger registriert.
  (Schritt: nach Push `harvest.yml -f format=openneuro_brainvision` dispatchten,
  Run einmal lesen, dann `sources.φ` url-Zeilen + `asset present` setzen,
  Ledger → `kompiliert`.) `wartend` auf CI-Lauf.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — `blockiert` (extern, Backend down). (Schritt: `archive_search --verdict` +
  sync-QUERY, Trigger Backend-Erholung.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — `blockiert` (extern, 502).
  (Schritt: `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`, Trigger
  Banner-Wechsel.) `blockiert`.
- **opencode-Browser-Bridge** — kein Target (`browser_targets` leer). (Schritt:
  Extension verbinden, dann `browser_open`.) `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — stündlicher Schedule. (Schritt: `ci_manage view
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

- **Ernte-Folge 104** (build/flash): 2× `grind-flash` (Asset/sha256 aus dem
  snirf-harvest-Run `35474615876`; Probe-Verdikt aus `35474638076`) —
  Routine-Klasse, flash bereits gemessen geschlossen; kein Doppellauf gegen
  pro/max.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ` (`openneuro_brainvision` neu,
  `openneuro_snirf` → `present`), `phi/pipeline/ledger.φ` (ds008192 →
  `kompiliert`; ds007471-note), neues Handover, Move
  `handover-2026-09-20-ernte-folge103.md` → `archiv/`.
- **Fremd (nicht anfassen):** die drei `handover-2026-09-16-*`-Moves
  (`D` + `?? archiv/`), `docs/zustand/external-state.md`,
  `handover-2026-09-20-entscheid-folge59.md`, `handover-2026-09-20-bau-folge98.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
