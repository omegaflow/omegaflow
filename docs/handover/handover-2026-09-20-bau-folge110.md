<!--
  title: Handover — Bau-Folge 110 (Stand 2026-09-20)
  session: Bau-Folge 110
  class: handover
  date: 2026-09-20
  sha256: ec9239f79d41f3b460f77fad3d15ab3d1e769202d93db9ee531cf4bd502034f0
  status: live
-->
# Handover — Bau-Folge 110 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `68a8240f` (bau folge109) — **Vorfahr von `origin/main` `b9112e5d`**,
  also gepusht; die Basis bewegte sich während der Session (Push der
  forschung-Linie → `84aa5147` == `origin/main`). Arbeitsbaum fremd:
  `docs/handover/post.md` `M`, `docs/zustand/external-state.md` `M`,
  `handover-…-entscheid-folge64.md` `??`, `handover-…-forschung-folge117.md` `??`,
  `folge63`-Move staged — **nicht angefasst**. Snapshot Start
  `refs/safety/1789925297`, Ende `refs/safety/1789925861`.
- **Postfach** — neuer Eingang `1789922257` (Pine64 „Re: Developer hardware
  request — RISC-V sensor node für omegaflow"), davor letzter `1789918147`; kein
  bau-Eingang. `post.md` leer (keine Nachrichtenzeile).
- **CI** — `ci-check`-Kette unruhig: Watchdog-Snapshot 19:14 aktiv
  `35523145456` @`5c61e16e`, rot (attempt 1) `35520538766` @`fa44f315` (**main**),
  `35517957987`; `ci_manage list` live: viele `ci-check` `cancelled`,
  `35526010713` in_progress. Kein sauberes Verdikt am HEAD.
- **Zustand-Ledger** — Postfach- und CI-Status-Zeile in `external-state.md` sind
  fällig, liegen aber in der Datei mit fremdem uncommitteten Hunk
  (Free-Model-Zeile + Header-sha256) und bleiben darum unangetastet; die Werte
  stehen als Messung oben.

## Offen

- **Queue-Korpora Re-Lauf (bau-Register, operator-gebunden)** — die 7
  `parser-gap`-Korpora (`phi/pipeline/ledger.φ:70–94`) brauchen den lokalen
  Release-Binär-Lauf (`--port` + `--probe`) nach dem `port.rs`-Fix; die
  `pos`-ohne-`body`-Blöcke hängen am selben Lauf-Ort. An `entscheid` geroutet
  (Operator-Queue). (Schritt: Operator-Wort für den Lauf-Ort, dann `--port` +
  `--probe`, dann die Ledger-Notes fortschreiben.) · `operator-gebunden`
- **`ci-check`-Verdikt am HEAD** — (Schritt: nach Push den Run einmalig aus
  `/tmp/opencode/ci_status.md` lesen; bei Rot `ci_manage log <id>`.) · `wartend`

## Benchmark

- **Bau-Folge 110**: Atom „NIST-WebBook-Query-Route disponieren" — Force-Gate-
  Urteil + Register-Disposition. Die Klasse „Force-Gate-Litmus" ist per
  Profil-Tabelle `grind-pro` (Urteil), kein Doppellauf gegen flash — das Verdikt
  ist eine Urteilsklasse, keine Routine-Messung. Der `grind-pro`-Taucher maß die
  Query-Route (`?Name=`/`?Formula=`/`?ID=` 200 text/html, Registry/positionslos;
  `?JCAMP=<id>&Index=&Type=IR|Mass` 200 `chemical/x-jcamp-dx`, statisches
  Referenzspektrum) → **`declined`**. Kein HTML-Arm gebaut.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/blocked_sources.φ` (NIST-`blocked parser-def`-Eintrag
  entfernt), `phi/declined_sources.φ` (WebBook-Referenz-Note präzisiert),
  neues `docs/handover/handover-2026-09-20-bau-folge110.md`, Move
  `handover-2026-09-20-bau-folge109.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/post.md`,
  `docs/zustand/external-state.md`, `docs/handover/handover-2026-09-20-entscheid-folge64.md`,
  `docs/handover/handover-2026-09-20-forschung-folge117.md`, der `folge63`-Move,
  die während der Session fremd entstandenen `phi/pipeline/probe_hapi_proposed.φ`
  + `phi/pipeline/probe_wave.φ`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
