<!--
  title: Handover — Entscheid-Folge 58 (Stand 2026-09-19)
  session: Entscheid-Folge 58
  class: handover
  date: 2026-09-19
  sha256: b11b2adf6a1abfbb3be86541419a7c46a3430d45ede83693923e1153d57ed0d4
  status: live
-->
# Handover — Entscheid-Folge 58 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage. Der
Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl; Wartestellungen
(`wartend`) sind kein Auswahlpunkt, sondern nennen nur ihren Auslöser. Jeder Punkt
trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Entscheid-Folge 58)

- **HEAD** `7d4a01e2` (== `origin/main`, „bau folge97: fold the green
  ds007822/gedi_l2a verdicts …"). Safety-Net `refs/safety/1789856812` (Start).
  Seit Folge 57 (`1fd18b31`) über `46bea4dc` (folge57), `f345b7e1`/`860a15a6`/
  `7d4a01e2` (bau 97) auf `7d4a01e2`. **Der Fleet-Deadlock ist aufgelöst** —
  bau97 hat seinen gestagten Satz committet.
- **CI** — `ci_manage list` (~22:29Z): **pending** `ci-check` `35473313318`
  @`7d4a01e2`, `te-gate` `35472169687`, `hyperscanning-te` `35472171277`;
  **in_progress** `allwise-cdn` `35471283884`; **success** `fmt-apply`
  `35472907001` (format-Fix), `harvest-dispatch` `35473313313`/`35473227084`,
  `openneuro-cdn` `35471360435` (ds007822), `harvest` `35471362190`/`35470954779`/
  `35470953233`, `placebo-ave-cdn` `35470938996`, `auto-dispatch` `35470928196`;
  **failure** `openneuro-eeg-probe` `35473247219`, `paper-check` `35470928180`;
  **cancelled** die ci-check-Kette (`35473248876`→…, per-ref-Concurrency).
  Zustand-Ledger (`external-state.md`) auf `7d4a01e2` fortgeschrieben.
- **Postfach** — jüngster Ledger-Eingang `1789853943` (Rubin-Forum Summary,
  informativ); davor `1789795811` (Rubin AGN DP2). Kein entscheid-relevanter
  Eingang; Intervall 2⁶ min nicht abgelaufen → Eintrag zitiert, nicht neu gemessen.
- **Post** — `post.md` trägt 2 Zeilen an forschung: bau97-Deadlock aufgelöst
  (Hunks frei), Riss 4 (Ksg kein off-path). Die überholte fmt-apply-Zeile gelöscht.
- **Baum** — forschungs 4 `timeout-minutes`-Hunks in `.github/workflows/ci-check.yml`
  sind seit dem bau97-Commit **frei** (unstaged); drei fremde
  `handover-2026-09-16-*`-Renames (nicht unsere, nicht angefasst).

## Offen

- **F1 — Rat-Konvention** „keine gestagten Fremd-Hunks in geteilten Dateien am
  Sessionende" als AGENTS.md-Zeile (härtester undatierter Punkt). **`operator-gebunden`**
  (Rat-Beschluss). (Schritt: Ratssitz `council` oder Operator-Wort.)
- **Pipeline-Port force-Gate — binär A/B** (seit folge46). **`operator-gebunden`**.
- **vC-Permeabilität — Vollzug** (Operator-Maschine). **`operator-gebunden`**.
- **register_lookup-Symlink** — PATH-Eingriff. **`operator-gebunden`**.
- **opencode-Browser-Bridge** — kein Target (`browser_targets` leer). **`operator-gebunden`**.
- **Limadou PI-Freigabe** — per-act consent. **`operator-gebunden`**.
- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}, dann n setzen
  oder Driver-Design ändern; Floor bleibt. An forschung gepostet. **`wartend`**.
- **`tools/harvest/**` fehlt in den ci-check-Push-Pfaden** — baus benannter
  Konfound; Aufnahme ist ein Bau-Atom. An bau gepostet. **`wartend`**.
- `termin` — **Lasair-LSST** (API 502, Backend server-seitig; Trigger
  Banner-Wechsel). **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC, Rubin-Review, Sonden-Antworten.

## Geteilter Baum — eigener Pfad-Satz

- `docs/zustand/external-state.md` (Postfach- + CI-Zeile; CI `1fd18b31` → `7d4a01e2`)
- `docs/handover/post.md` (bau97-Deadlock-Zeile aktualisiert, fmt-apply-Zeile gelöscht)
- `docs/handover/handover-2026-09-19-entscheid-folge58.md` (neu)
- Move `handover-2026-09-19-entscheid-folge57.md` → `archiv/`
- Fremd uncommittet (nicht angefasst): forschungs `ci-check.yml`-Timeout-Hunks,
  die drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Kein Dispatch — kein abarbeitbarer Bau-/Recherche-Punkt in dieser Linie; alle
  offenen Punkte sind `operator-gebunden` oder `wartend`. Der stehende Pass
  (HEAD-Wechsel `1fd18b31` → `7d4a01e2`) lief in der Hauptsession. Routineklasse
  geschlossen (`grind-flash` $0.0008, 2026-09-16). Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
