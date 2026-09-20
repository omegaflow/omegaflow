<!--
  title: Handover — Ernte-Folge 105 (Stand 2026-09-20)
  session: Ernte-Folge 105
  class: handover
  date: 2026-09-20
  sha256: c01a2496fc31326c215e806c8d267154bc08a0815e71290a60860a8117a7db67
  status: live
-->
# Handover — Ernte-Folge 105 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 105)

- **HEAD** `c5f9fc68` (== `origin/main`); Safety-Net `refs/safety/1789861516`
  (Start). Die Ernte-folge104-Commits (`95769e75`, `64b30f58`), `bau folge99`
  (`8d1a995d`) und `forschung folge104` (`c5f9fc68`) sind seither gelandet. Der
  Baum trägt fremde uncommittete entscheid-Arbeit: der gestagte
  Rename `handover-2026-09-20-entscheid-folge59.md` → `archiv/`, die drei
  `handover-2026-09-16-*`-Moves (`D` + `?? archiv/`), `?? handover-…-entscheid-folge60.md`,
  `M docs/zustand/external-state.md` — nicht angefasst. Eigener Pfad-Satz:
  `phi/sources.φ`, `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `docs/handover/post.md`, neues Handover, Move
  `handover-2026-09-20-ernte-folge104.md` → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789853943`
  (Rubin-Forum-Summary, informativ); kein neuer Ernte-Eingang. `post.md` trägt
  fremde, uncommittete Zeilen — darunter zwei an ernte (icesat2; Dedup
  Browser-Bridge/Limadou) — hierher gefaltet; `post.md` selbst bleibt fremd
  (nicht angefasst).
- **CI-Status** — `ci_manage list` (2026-09-20): **success** `harvest`
  `35476235446` (openneuro_brainvision, 32 Assets), `openneuro-eeg-probe`
  `35474638076`, `harvest` `35474615876`, `quake-feeds-cdn` `35475632601`;
  **in_progress** `ci-check` `35475257980`, `te-gate` `35475226890`. Zustand-Eintrag
  `external-state.md` trägt den Stand @`f7364835` — die CI-Zeile schreibt die
  entscheid-Linie fort.
- **Werkzeug-Grenze** — die opencode-Browser-Bridge trägt kein Target
  (`browser_targets` leer); der opencode-Browser-Pfad bleibt `operator-gebunden`.

## Offen

- **icesat2_atl03** `phi/harvest.φ:79` (Post an ernte, gefaltet) — args
  `--limit 1 --skip 1` → `--skip 2` (2/215 Granulen) fortschreiben und
  `harvest.yml -f format=icesat2_atl03 -f force=true` dispatchten, dann staged
  count lesen. (Schritt: `phi/harvest.φ`-args-Zeile setzen, committen, dann
  `gh workflow run harvest.yml`.) `wartend`.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — `blockiert` (extern, Backend down). (Schritt: `archive_search --verdict` +
  sync-QUERY, Trigger Backend-Erholung.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — `blockiert` (extern, 502).
  (Schritt: `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`, Trigger
  Banner-Wechsel.) `blockiert`.

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

- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 105** (build/flash): 1× `grind-flash` (32 Asset-Namen/Größen/sha256
  aus Run `35476235446`) — Routine-Extraktion, flash-Klasse bereits gemessen
  geschlossen; kein Doppellauf gegen pro/max.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/sources.φ` (32 `openneuro_brainvision` url-Zeilen),
  `phi/harvest.φ` (`openneuro_brainvision` → `asset present`),
  `phi/pipeline/ledger.φ` (ds007471 → `kompiliert`), neues Handover, Move
  `handover-2026-09-20-ernte-folge104.md` → `archiv/`.
- **Fremd (nicht anfassen):** der gestagte Rename `…entscheid-folge59.md`,
  die drei `handover-2026-09-16-*`-Moves (`D` + `?? archiv/`),
  `?? handover-…-entscheid-folge60.md`, `docs/zustand/external-state.md`,
  `docs/handover/post.md` (fremde uncommittete Zeilen). Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
