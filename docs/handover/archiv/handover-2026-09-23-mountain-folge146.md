<!--
  title: Handover — Mountain-Folge 146 (2026-09-23)
  session: Mountain-Folge 146
  class: handover
  date: 2026-09-23
  sha256: 6f7c07f34869505bd92c5c5ef99d07e0f061f05549565866f91e378b22f497c8
  status: live
-->
# Handover — Mountain-Folge 146 (2026-09-23)

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
Punkt wird **aufgeschlüsselt** geführt — **Trigger** / **Lage** (gemessen, mit
Messstempel) / **Blockade** / **Braucht** (der wörtliche, kopierbare Schritt).
`operator-gebunden`, `blockiert` und `wartend` werden benannt, nie dispatcht. Jeder
Punkt trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23 ~22:4xZ)

- **HEAD** `486b8e6c1` (beim Start gemessen; sensorische Arbeit danach committet);
  Arbeitsbaum trägt jetzt **fremde** Arbeit der **mycelium**-Linie:
  `docs/handover/archiv/handover-2026-09-23-mycelium-folge148.md` (Move),
  `docs/handover/handover-2026-09-23-mycelium-folge149.md` neu, `phi/blocked_sources.φ`,
  `phi/footprints.φ`, `phi/witnesses.φ`, `phi/pipeline/index.φ`,
  `phi/pipeline/catalog/korpora_heim.φ`, und ein `neracoos`-`url`-Block in
  `phi/sources.φ`. **Kollision:** mycelium und mountain editieren `phi/sources.φ`
  gleichzeitig — der Commit staged nur die eigenen Hunks (`git apply --cached`),
  nie die fremden.
- **CI** (`ci_manage list`/`view`, ~22:4xZ): `gmrt-cdn 35906178815` **success**
  (19:00Z), `opensensemap-cdn 35906130561` **success** (19:02Z); `te-gate
  35893882101` @`6a68c1901` **in_progress** (der blockierende `35875025486` ist
  inzwischen **cancelled**); `free-model-bench 35893010538` @`d754ecdf8`
  **in_progress**, `free-model-agent-bench 35893014541` @`d754ecdf8` **success**
  (18:10Z).
- **Postfach:** `state/mail/mail_ledger.φ` **absent** → keine Mountain-Zeile.
- **`register_lookup --open`:** keine offene mountain-eigene Register-Zeile (die
  beiden `parser-def`-Zeilen GMRT/openSenseMap wurden in folge145 gebaut).
- **`open_points_check` folge146:** nach Erstellung gemessen (Pfad-Abgleich).

## In diesem Atom gebaut (git trägt)

- **Punkt 1 GMRT-CDN-Manifestation:** Run `35906178815` success; `archive_search
  --sniff` maß `gmrt_bathymetry.bin` 6569840 B sha256
  `0acd52f33ab9ff7d998e85b37a230d8fbf527ab0948c7ce5c69d3b139c83b52e`. `sha256`-Direktiv
  in `phi/sources.φ` (GMRT-Block), `asset present` + gemessene note in `phi/harvest.φ`
  (gmrt-Arm).
- **Punkt 2 openSenseMap-CDN-Manifestation:** Run `35906130561` success;
  `opensensemap_temperatur.bin` 2888 B sha256
  `f5a043b1a9c53461b00d55aea7b357416ad647c1931621981cbca6717767cfbe`. `sha256`-Direktiv
  in `phi/sources.φ` (openSenseMap-Block), `asset present` + note in `phi/harvest.φ`.
  `sgrep "asset fehlt" phi/harvest.φ` → 0 (alle Arme `asset present`).
- **Punkt 4 (Teil) agent-bench T7:** `ci_manage log 35893014541 --all` — T7 pass **5** /
  108 Statuszeilen (38 `wrong_answer`, 39 `no_output`, 26 `timeout`). **T4** und
  ein CF-`http_401`-Anteil sind in diesem Log **absent**: beide gehören zum Sibling
  `free-model-bench 35893010538` (noch in_progress).

## Offen (aufgeschlüsselt)

### 3. flare-Re-Insert green-confirm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss von `te-gate 35893882101` @`6a68c1901`
- **Lage:** in_progress (gemessen 2026-09-23 ~22:4xZ via `ci_manage view`); der
  blockierende `35875025486` ist **cancelled**
- **Blockade:** der Lauf dauert (Job-timeout 330 min) — eine Messung, kein cancel
- **Braucht:** nach Abschluss `ci_manage view 35893882101` → `flare`-Job grün
  (assert + `flare power probe:`-Zeilen)

### 4. Free-Model-Bench T4 + CF-`http_401`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss von `free-model-bench 35893010538` @`d754ecdf8`
- **Lage:** in_progress (gemessen 2026-09-23 ~22:4xZ via `ci_manage view`); der
  agent-bench-Teil ist eingelöst (T7 pass 5/108)
- **Blockade:** Runner-Queue
- **Braucht:** `ci_manage log 35893010538 --all` → T4-pass counts; bei vorhandenem
  CF-`http_401`-Anteil eine `An future:`-Zeile (operator-gebunden, Zugang/Key)

### 5. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** nicht angekommen (gemessen via `state/mail/mail_ledger.φ` `1790046330`)
- **Blockade:** physische Ankunft
- **Braucht:** nach Ankunft Bring-up + Kopplung messen

### 6. `register_lookup --dropped` Zählwurzel (Semantik-Wechsel + Eichung)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `register-dropped 35922188109` (dispatcht 2026-09-23; Nicht-`--count`-Lauf liest `Z dropped` / `R commit-resolved` @`486b8e6c1`-Nachbartree)
- **Lage:** `run_dropped` meldet `--count` **vor** der Git-Auflösung (überzeichnet; ~95–96 % `commit-resolved`). Der Fix (Auflösung vor den `--count`-Shortcut, Druck `dropped - resolved`) wurde gebaut, dann **zurückgenommen** (Rat-Verdikt b2, gemessen): ein lokaler Voll-Lauf brach nach 15 min ab — verbotene Vordergrund-Penetration; der post-Auflösungs-Wert ist nur in CI messbar (neues Binary lokal strukturell verwehrt). Gate-Botschaft `.github/workflows/ci-check.yml:88–92` benennt den post-Auflösungs-Wert bereits als gewollt; `dropped-baseline 3053` ist ein Brutto-Wert (Einheiten-Mismatch nach dem Fix).
- **Blockade:** neues Binary existiert nur in CI
- **Braucht:** `ci_manage view 35922188109` → Summary-Zeile `register_lookup --dropped: … Z dropped, R commit-resolved …` → `N = Z − R`; dann **ein** Commit: `--count`-Fix in `tools/register/src/bin/register_lookup.rs` (`run_dropped`: den `count_only`-Shortcut hinter die `distinctive_token`/`resolution_status`-Auflösung verschieben, `--count` druckt `dropped - resolved`) + `dropped-baseline N` in `docs/zustand/dropped-baseline.md`

## Benchmark

- Drei Taucher-Aufgaben parallel an **grind-flash** (billigste Klasse): zwei
  Register-/CDN-Eichungen (Punkte 1+2, ein Kontext) und die `run_dropped`-Prüfung
  (Punkt 6). Kein pro/max-Doppel — Routine-Klasse geschlossen 2026-09-16. Zwei
  **Rat**-Runden (pro/max) für die Abschluss-Entscheidung Punkt 6: erste Runde
  Variante (a) (Fix + Baseline in einem Commit), nach gemessenem Timeout des lokalen
  Voll-Laufs zweite Runde Verdikt **(b2)** — Fix zurücknehmen, Punkt offen mit
  CI-Eichung. Kein Klassen-Sieger neu zu doppeln.

## Geteilter Baum — eigener Pfad-Satz

- **Geändert:** `phi/sources.φ` (2 `sha256`-Zeilen), `phi/harvest.φ` (2 Arme
  `asset present` + notes)
- `docs/handover/handover-2026-09-23-mountain-folge146.md` (neu)
- Move `handover-2026-09-23-mountain-folge145.md` → `archiv/` (eigene Linie, atomar)
- **Fremd/unberührt:** mycelium (`phi/sources.φ` neracoos-Hunk, `phi/blocked_sources.φ`,
  `phi/footprints.φ`, `phi/witnesses.φ`, `phi/pipeline/index.φ`,
  `phi/pipeline/catalog/korpora_heim.φ`, `folge148`→`archiv`, `folge149`)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
