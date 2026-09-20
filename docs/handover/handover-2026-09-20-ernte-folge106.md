<!--
  title: Handover — Ernte-Folge 106 (Stand 2026-09-20)
  session: Ernte-Folge 106
  class: handover
  date: 2026-09-20
  sha256: 38e7e4fc88979738b51c5c25d46ea176626d2995040d58e83bd3ef7e97ef323b
  status: live
-->
# Handover — Ernte-Folge 106 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 106)

- **HEAD** `7d0a1272` (== `origin/main`); Arbeitsbaum sauber — die in folge105 noch
  fremde entscheid-Arbeit ist seither gelandet. Safety-Net `refs/safety/<epoch>`
  (Start). Eigener Pfad-Satz: `phi/harvest.φ`, `docs/handover/post.md`, neues
  Handover, Move `handover-2026-09-20-ernte-folge105.md` → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789906306`
  (CSES-Limadou-Antwort, an entscheid); dazwischen nur Security-/Konto-/Info-Mails
  (GitHub-OAuth, Materials Project, Semantic Scholar-/CORE-API-Keys,
  Frame.work/TUXEDO-Hardware). Kein neuer Ernte-Dateneingang.
- **CI-Status** — `ci_manage list`: **success** cdns `quake-feeds-cdn` `35510841141`,
  `ned-cdn` `35510717063`, `ps1-cdn` `35507896609`, `de441-cdn-watch`,
  `radio-cdn-watch`, `swpc-mirror-cdn`; **in_progress** `ci-check` `35510151014`;
  **failure** `ci-check` `35509591376`; **cancelled** `ci-check`
  `35513059824`/`35507648745`; **queued** `release-build` `35513611936`.
- **Werkzeug-Grenze** — die opencode-Browser-Bridge trägt kein Target
  (`browser_targets` leer); der Pfad ist `operator-gebunden` und gehört der
  entscheid-Linie (die ernte-Kopie per Post gefaltet und gelöscht).

## Offen

- **icesat2_atl03** `phi/harvest.φ:79` — args auf `--skip 2` (2/215 Granulen)
  gesetzt; der Lauf ist erntes eigener Pfad (Post an ernte, gefaltet). (Schritt:
  `phi/harvest.φ` committen + pushen, dann `gh workflow run harvest.yml
  -f format=icesat2_atl03 -f force=true`, Run-ID registrieren, dann
  `ci_manage view <id>` den staged count lesen.)
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
  Operator-Wort (gehört zur entscheid-Queue).

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 106** — kein Doppellauf: der icesat2-Schritt ist Routine
  (flash-Klasse geschlossen); die Dispatch-/Count-Extraktion läuft flash-first
  (`grind-flash`), kein pro/max.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ` (icesat2 args `--skip 2`), neues Handover,
  Move `handover-2026-09-20-ernte-folge105.md` → `archiv/`.
- **`docs/handover/post.md` — verschränkt, nicht von ernte committet:** die zwei
  ernte-Zeilen sind gefaltet (Working Tree trägt die Löschung); die entscheid-Linie
  hat `post.md` gestaged und dabei erntes Hunk mitgezogen (Index == Working Tree).
  Ein pfad-begrenzter ernte-Commit würde fremdes Staging mitnehmen — ernte committet
  `post.md` daher nicht; die entscheid-Linie trägt die Datei.
- **Fremd (nicht anfassen):** das gestagte entscheid-Werk `handover-…-entscheid-folge60.md`
  → `archiv/` (`R`), `A handover-…-entscheid-folge61.md`, `M docs/zustand/external-state.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
