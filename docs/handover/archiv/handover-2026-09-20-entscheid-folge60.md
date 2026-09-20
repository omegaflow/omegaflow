<!--
  title: Handover — Entscheid-Folge 60 (Stand 2026-09-20)
  session: Entscheid-Folge 60
  class: handover
  date: 2026-09-20
  sha256: 9776feb2be5193cf84c02cea8a383568e6f94b57dfa3799ba1cd5a484ea6cdea
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
- **Post** — `post.md` trägt jetzt sechs Zeilen (dieses Atom): `An ernte` ×2
  (icesat2-Tag, Doppel-Tracking), `An forschung` ×2 (Flyby-Auftrag, folge44/51-Move),
  `An bau` ×2 (register_lookup-Binary, ci-check-Glue). Keine Zeile an `entscheid`.
- **Baum** — nach dem Meta-Pass: `entscheid24`-Move gestaged (eigene Linie);
  fremd bleiben die `forschung44/51`-Paare (an forschung gepostet). HEAD inzwischen
  `4a3ffe31` (ernte105).

## Offen

- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}, dann n setzen
  oder Driver-Design ändern; Floor bleibt. An forschung gepostet. **`wartend`**.
- **09-16-Limbo** — `entscheid24` → `archiv/` in diesem Atom committet (eigene
  Linie, atomarer Move); `forschung44/51` an forschung gepostet (Dateiname trägt
  den Besitzer). **`wartend`** (forschung-Pass).
- `termin` — **Lasair-LSST** (API 502, Backend server-seitig; Trigger
  Banner-Wechsel). **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC, Rubin-Review, Sonden-Antworten.

## Operator-Queue (einmal vorlegen beim Operator-Rückkehr; einfache Sprache)

F1 (Aufräum-Regel), die Queue-Mechanik und die Klartext-Regel sind mit
Operator-Wort 2026-09-20 in `AGENTS.md` gesetzt. Verbleibend, in einfacher Sprache:

1. **Pipeline-Port force-gate — A oder B?** (ältester, seit folge46) — *Lage:* ein
   Schalter beim Quellen-Einlesen ordnet eine Kraft zwei möglichen Weisen zu;
   welche physikalisch gilt, ist offen. *Frage:* welche der zwei? *Ja:* der Schalter
   wird fest gesetzt. *Offen:* die zwei Optionen + ihre Folge müssen vor dem
   Vorlegen präzisiert werden (`phi/pipeline/`).
2. **vC-Permeabilität — Vollzug** — *Lage:* ein gemessener Wert soll auf deiner
   Maschine in Betrieb gehen; diese Maschine kann es nicht. *Frage:* führen wir ihn
   auf deiner Maschine aus? *Ja:* du startest den Lauf; *Nein:* bleibt offen.
3. **register_lookup-Symlink** — *Lage:* das Werkzeug soll im System-Pfad liegen.
   *Frage:* erlaubst du den PATH-Eingriff? *Ja:* Symlink wird gesetzt; *Nein:* bleibt.
4. **opencode-Browser-Bridge** — *Lage:* die Browser-Brücke hat kein verbundenes
   Fenster. *Frage:* verbindest du die Extension? *Ja:* Brücke nutzbar; *Nein:* bleibt.
5. **Limadou PI-Freigabe** — *Lage:* eine Datenfreigabe braucht deine Zustimmung
   pro Anfrage. *Frage:* senden wir die Anfrage? *Ja:* die Anfrage geht raus
   (per-act); *Nein:* bleibt.

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

## Reibung / Myzel (Untersuchung 2026-09-20 — Wissenschaft + Gremium)

Gemessene Reibungskarte (Wissenschaft) + Rat + 2× Forschung (~28 `archive_search`):

- **`register_lookup`-Binary veraltet** — Quelle implementiert `--open`
  (`tools/register/src/bin/register_lookup.rs:1324/1331`), PATH-Binary kennt nur
  `--live`/`--history` (gemessen). Jeder Pass liest durch dieses Instrument →
  „kein wählbarer Punkt" kann ungemessene Null sein. → Post `An bau`. **`wartend`**.
- **`ci-check` 80,6 % cancelled** (29/36, 0 success/100) — Queue + Glue-Period
  (Loss → Delay), Push joined statt reset. → Post `An bau`. **`wartend`**.
- **Per-Linie `post`-Kanäle** (CRDT/Stigmergie: kommutative Appends statt Mutex) —
  **nicht** präemptiv splitten (Rat: temporalen Fix zuerst, eine gemessene Woche);
  der hunk-scharfe Commit ist die benannte Struktur-Lücke. **`wartend`**.
- **Operator-Queue** (Ashby: requisite variety → am Rand komprimieren, zentral
  batchen, ein Trigger statt 14 Wiederholungen). **`operator-gebunden`** (F1).
- **Own-line atomare Moves** — `entscheid24` committet; `forschung44/51` gepostet.

Kanon der drei Stimmen: Berg (Besitz/atomare Move/Vielfalt), Fluss (Kollision =
Commit-Granularität), Myzel (stigmergischer Trace), Sinn (negative Deklaration
trägt Coverage), Zukunft (Homöostat, Viability-Kernel). Konvergenz-Satz: *das
Myzel ist auf der Schreibseite konfliktfrei und auf der Leseseite ein kalibriertes,
Coverage-tragendes Instrument — eine Nachricht wird nur bei Schwellen-Übertritt
gesendet und trägt ihre eigene Vollendung.*

## Geteilter Baum — eigener Pfad-Satz

- `docs/zustand/external-state.md` (CI-Zeile `f7364835` → `c5f9fc68`)
- `docs/handover/handover-2026-09-20-entscheid-folge60.md` (Offen + Reibung/Myzel +
  Operator-Queue)
- `AGENTS.md` (F1-Bullet in der Write-Boundary; Operator-Queue- + Klartext-Regel)
- `docs/handover/post.md` (sechs Zeilen: `An ernte`×2, `An forschung`×2, `An bau`×2)
- Move `handover-2026-09-16-entscheid-folge24.md` → `archiv/` (eigene Linie, atomar)
- (früherer Commit `d230d6f6`: `external-state.md` CI-Zeile, folge60 neu, Move folge59)
- Fremd uncommittet (nicht angefasst): `forschung44/51`-Paare (an forschung
  gepostet), erntes `phi/*.φ`-Hunks.

## Benchmark

- **Reibungs-/Myzel-Untersuchung:** 4 Dispatches — `council` (Gremium, pro/max,
  Architektur), `general` (gemessene Reibungskarte, flash), 2× `research-max`
  (Berg/Myzel/Zukunft + Fluss/Sinn, je pro/max, ~28 `archive_search`-Aufrufe).
  Der Rat lieferte den Rang, die Forschung den Konvergenz-Satz. Pro/max
  gerechtfertigt: Architektur-Urteil (5 Stimmen) + mehrstufige Quellenforschung;
  die Reibungskarte war flash-Klasse. Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
