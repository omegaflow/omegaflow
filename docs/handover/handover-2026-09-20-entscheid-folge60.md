<!--
  title: Handover — Entscheid-Folge 60 (Stand 2026-09-20)
  session: Entscheid-Folge 60
  class: handover
  date: 2026-09-20
  sha256: ee2cc92df93745faecb6bfbbaa186f81a6a7c6391be647986a7b5b7c501e91fc
  status: live
-->
# Handover — Entscheid-Folge 60 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage. Der Planungs-Pass
nennt die offenen Punkte als nummerierte Auswahl; Wartestellungen (`wartend`) sind
kein Auswahlpunkt, sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`). Das Handover
wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 60)

- **HEAD** `c5f9fc68` (== `origin/main`, „forschung folge104: fix the paper-check
  red …"). Start war `f7364835`; **forschung folge104 hat während dieser Session
  committet und gepusht** (`f7364835` → `c5f9fc68`). Safety-Net
  `refs/safety/1789859694` (Start). Die vorige Übergabe (folge59) wird ins Archiv
  gefaltet.
- **CI** — `ci_manage list` (2026-09-20 ~01:19Z @`c5f9fc68`): **pending**
  `ci-check` `35476511983` @`c5f9fc68`, `te-gate` `35475226890` @`5b406e16`,
  `harvest` `35476248712`; **in_progress** `harvest` `35476235446`, `ci-check`
  `35475257980`; **success** `paper-check` `35476512043` (header-title-Fix grün),
  `quake-feeds-cdn` `35475632601`, `ned-cdn` `35475525110`, `ps1-cdn`
  `35475033451`, `swpc-mirror-cdn` `35474661305`, `harvest` `35474641409`,
  `openneuro-eeg-probe` `35474638076` (bau98 companion-fix grün),
  `harvest-dispatch` `35474610602`/`35474630076`, `auto-dispatch` `35474630080`;
  **cancelled** die ci-check-Kette (per-ref-Concurrency). **Kein Rot** im
  aktuellen Fenster. Zustand-Ledger (`external-state.md`) CI-Zeile auf
  `c5f9fc68` fortgeschrieben.
- **Postfach** — `mail_ledger.φ` (101 Zeilen): jüngster Eingang unverändert
  `1789853943` (Rubin-Forum Summary, informativ); kein neuer Eingang. Zitiert nach
  Forschung-Folge 104 (frisch gemessen); Intervall 2⁶ min war abgelaufen, der
  Ledger-Tail unverändert.
- **Post** — `post.md` leer (nur Header); keine Zeile an `entscheid`.
- **Baum** — fremd uncommittet (nicht angefasst): die drei
  `handover-2026-09-16-*`-Renames (`D` + `??` in `archiv/`).

## Offen

- **F1 — Rat-Konvention** „keine gestagten Fremd-Hunks in geteilten Dateien am
  Sessionende" als AGENTS.md-Zeile (härtester undatierter Punkt). **`operator-gebunden`**
  (Rat-Beschluss). Begleiter (Meta-Befund): für **verwaiste uncommittete
  Baumzustände** (das 09-16-Limbo, unten) existiert kein Trigger, nur die
  Safety-Nets — eine Wiedervorlage-Regel „verwaister Baumzustand älter als N Tage"
  gehört als F1-Begleiter ins selbe Operator-Wort. (Schritt: Ratssitz `council`
  oder Operator-Wort.)
- **Pipeline-Port force-Gate — binär A/B** (seit folge46, 14 Folgen). **`operator-gebunden`**.
- **vC-Permeabilität — Vollzug** (Operator-Maschine). **`operator-gebunden`**.
- **register_lookup-Symlink** — PATH-Eingriff. **`operator-gebunden`**.
- **opencode-Browser-Bridge** — kein Target (`browser_targets` leer). **`operator-gebunden`**.
- **Limadou PI-Freigabe** — per-act consent. **`operator-gebunden`**.
- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}, dann n setzen
  oder Driver-Design ändern; Floor bleibt. An forschung gepostet. **`wartend`**.
- **09-16-Limbo** — drei Handover-Renames (`entscheid24`/`forschung44`/
  `forschung51` → `archiv/`) seit vier Tagen uncommittet, für alle Linien „fremd",
  ohne Besitzer. Nicht angefasst (fremd). **`operator-gebunden`** (F1-Begleiter:
  Wiedervorlage-Regel für verwaiste Baumzustände).
- `termin` — **Lasair-LSST** (API 502, Backend server-seitig; Trigger
  Banner-Wechsel). **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC, Rubin-Review, Sonden-Antworten.

## Quer-Linien (Meta-Pass, alle vier lebenden Handover gelesen)

Vier Befunde gegen den Baum gehalten (GLM-Meta-Überblick):

- **icesat2_atl03 falsch geparkt** — ernte folge105 führt ihn als `wartend`, aber
  der Schritt ist vollständig ausführbar und `phi/harvest.φ:79` ist erntes eigener
  Pfad; Lauf `35476248712` ist **cancelled** (head `95769e75`, openneuro-Block),
  kein icesat2-Dispatch. → Post `An ernte` (unten).
- **Staler Punkt** `tools/harvest/**` — bau98 `8c7b885e` hat die push-Pfade längst
  committet (`.github/workflows/ci-check.yml +1`); der Punkt ist hier **gelöscht**.
- **Doppel-Tracking** `opencode-Browser-Bridge` + `Limadou PI-Freigabe` in ernte
  folge105 **und** entscheid — operator-gebundene Punkte gehören zur entscheid-Linie.
  → Post `An ernte` (unten).
- **Flyby Path 2** — das versiegelte Blatt (`flyby-path-2-preregistration.md`,
  Seal 2026-08-22; `-falsification-metric-addendum.md`, Seal 2026-09-03) ist **kein
  Auftrag**; vor dem 28.09. muss die Feldfüllung am Perigäum-Tubus registriert
  werden, sonst ist die Demonstration post-hoc. → Post `An forschung` (unten).

## Geteilter Baum — eigener Pfad-Satz

- `docs/zustand/external-state.md` (CI-Zeile `f7364835` → `c5f9fc68`)
- `docs/handover/handover-2026-09-20-entscheid-folge60.md` (neu)
- Move `handover-2026-09-20-entscheid-folge59.md` → `archiv/`
- `docs/handover/post.md` (drei `An ernte`/`An forschung`-Zeilen — **nicht
  committet**, um erntes uncommitteten `post.md`-Hunk nicht zu sweepen; der
  Empfänger faltet die Zeilen in seinem Commit)
- Fremd uncommittet (nicht angefasst): die drei `handover-2026-09-16-*`-Renames,
  erntes `phi/harvest.φ`/`phi/pipeline/ledger.φ`/`phi/sources.φ`/`post.md`-Hunks,
  `handover-2026-09-20-ernte-folge105.md`.

## Benchmark

- Kein Dispatch — kein abarbeitbarer Bau-/Recherche-Punkt in dieser Linie; alle
  offenen Punkte sind `operator-gebunden` oder `wartend`. Der stehende Pass
  (HEAD-Wechsel `f7364835` → `c5f9fc68`) lief in der Hauptsession. Routineklasse
  geschlossen (`grind-flash` $0.0008, 2026-09-16). Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
