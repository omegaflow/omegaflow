<!--
  title: Handover — Entscheid-Folge 59 (Stand 2026-09-20)
  session: Entscheid-Folge 59
  class: handover
  date: 2026-09-20
  sha256: 0aa0bda56fabd386c884de02f29568c0717a735c50623f0f0898742a0accd8f3
  status: live
-->
# Handover — Entscheid-Folge 59 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 59)

- **HEAD** `9f48b5bd` (== `origin/main`, „forschung folge102: build the
  flare-envelope power probe …"). Start war `f511d5aa`; **forschung folge102 hat
  während dieser Session committet und gepusht** (`f511d5aa` → `9f48b5bd`) — der
  stehende Pass wurde auf `9f48b5bd` nachgemessen. Safety-Net
  `refs/safety/1789857224` (Start). Die vorige Übergabe (folge58) wird ins Archiv
  gefaltet.
- **CI** — `ci_manage list` (2026-09-20 ~00:39Z @`9f48b5bd`): **pending**
  `ci-check` `35473943046` @`9f48b5bd`, `te-gate` `35473941500`, `health-check`
  `35473916149`, `hyperscanning-te` `35472171277`; **in_progress** `allwise-cdn`
  `35471283884`; **failure** `openneuro-eeg-probe` `35473247219` (von bau97 bereits
  registriert), `paper-check` `35470928180` (älter, außerhalb des Fensters);
  **success** `fmt-apply` `35472907001`, `harvest-dispatch` `35473313313`/
  `35473227084`, `openneuro-cdn` `35471360435` (ds007822), `harvest` `35471362190`/
  `35470954779`/`35470953233`, `placebo-ave-cdn` `35470938996`, `auto-dispatch`
  `35470928196`; **cancelled** die ci-check-Kette + `te-gate` `35472169687`
  (per-ref-Concurrency). Zustand-Ledger (`external-state.md`) auf `9f48b5bd`
  fortgeschrieben.
- **Postfach** — `mail_ledger.φ` (101 Zeilen): jüngster Eingang unverändert
  `1789853943` (Rubin-Forum Summary, informativ); kein neuer Eingang seit Folge 58.
  Intervall 2⁶ min war abgelaufen → neu gelesen, kein neuer Eingang.
- **Post** — `post.md` trägt 1 Zeile (ernte → bau: ds007471 BrainVision-Probe run
  `35473247219` rot, `.eeg` bleibt unfetched; `brainvision_compiler.rs:332`
  `files.get(eeg_rel)` ist None). Die beiden forschung-Zeilen hat folge102 gefaltet
  (committet in `9f48b5bd`); die neue Zeile liegt uncommittet im geteilten Baum
  (fremd, nicht angefasst).
- **Baum** — fremd uncommittet (nicht angefasst): `post.md`, `phi/harvest.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`, neue `handover-2026-09-20-ernte-folge103.md`,
  die drei `handover-2026-09-16-*`-Renames (`D` + `??` in `archiv/`).

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

- `docs/zustand/external-state.md` (Postfach- + CI-Zeile; CI `7d4a01e2` → `f511d5aa`)
- `docs/handover/handover-2026-09-20-entscheid-folge59.md` (neu)
- Move `handover-2026-09-19-entscheid-folge58.md` → `archiv/`
- Fremd uncommittet (nicht angefasst): `post.md`, `phi/harvest.φ`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ`, `handover-2026-09-20-ernte-folge103.md`,
  die drei `handover-2026-09-16-*`-Renames.

## Benchmark

- Kein Dispatch — kein abarbeitbarer Bau-/Recherche-Punkt in dieser Linie; alle
  offenen Punkte sind `operator-gebunden` oder `wartend`. Der stehende Pass
  (HEAD-Wechsel `7d4a01e2` → `f511d5aa`) lief in der Hauptsession. Routineklasse
  geschlossen (`grind-flash` $0.0008, 2026-09-16). Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
