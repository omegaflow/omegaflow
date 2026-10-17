<!--
  title: House Rules — die Essenz (was gilt)
  class: concept
  date: 2026-10-16
  sha256: 08e1d30748661c847a3c864222a6768028be54572b47674826476e14fa1c70ff
  status: live
  see-also: AGENTS.md docs/concepts/tools-map.md
-->
# House Rules — die Essenz

Hier steht nur, was **kein Gate und kein `--help`** tragen kann. Was ein Gate prüft, ist
ausgeführt statt erzählt — sonst driftet der Text gegen das Gate. Die Volltexte liegen in
`archiv/` (2026-10-16).

## Die Linie — ein Pass

**Erste Handlung — deine drei Stellen, nichts sonst:** der **Dispatch**
(`~/projects/omegaflow-work/post/out/`, die eine Aufgabe der Leitstelle) · dein **Handover**
(`docs/handover/`, die neueste deiner Linie, ≤ 1 KiB, nur Offenes) · deine **Post**
(`post/in/`). Lies nur die Stellen, die die Aufgabe nennt (`sread <f> --offset --limit`).

- **Ein Pass.** Kein Planungstheater, keine Liste, kein Consent-Stopp für Bekanntes.
- **Kein Dispatch für dich?** → arbeite deine offenen Punkte, so viele wie möglich.
- **Vor jeder NET-Suche: `bin/where <begriff>`** — erst prüfen, was das Haus schon hat.
- **Ende des Passes:** `register_lookup --orphan-docs` — deine Waisen in die Übergabe falten.
- **Ein Pass schließt Punkte.** Jeder offene Punkt ist ein `### name` mit
  `Status · Trigger · Lage · Blockade · Braucht` — sonst ist die Zahl unsichtbar.
  Sammelt ein Pass nur, ist es kein Pass: **Taucher laufen, bis die Punkte zu sind** —
  nicht die Übergabe kürzen, nicht bei einer Zahl Schluss machen. Gemessen wird, was **geschlossen** wurde.
- **Grenze:** commit+push = deine Hand (eigene Pfade, Englisch); **Send** und jeder
  dritt-wirksame Akt = Operator-Hand (per-Akt-Wort); Architektur/Ethik → der Rat.
  **„Send" ist genau das — Versand, nichts sonst:** `smail --send`, ein Formular/Antrag,
  ein CDN-Upload/Rückzug, ein Konto-/Key-Akt. Alles davor — lesen, messen, bauen, entwerfen,
  den Rat fahren, Briefe legen, Verdikte sammeln — ist **Arbeit** und dein Boden.
  Die Grenze darf **nie** als Deckmantel dienen, Arbeit zu lassen: wer „Send bleibt Operator"
  sagt, wo er hätte arbeiten können, benutzt die Regel gegen ihren Zweck.

## Der Akt am Gegenüber (Operator-Wort)

- **Nur der Operator versendet.** `smail --send`, jedes Formular, jede Einreichung: die Maschine
  bereitet bis zur Kante vor (Entwurf, `--dry-run`) und legt **einen** Satz vor — **fertig, nicht
  angefragt**. „Vorlegen" heißt: die Arbeit ist getan, es fehlt allein der Versand.
  Diese Zeile ist **keine** Berechtigung, ungetane Arbeit als „Send" zu verbuchen.
- Ein Operator-Wort wird als `Wort | Datum | Quelle` registriert, **bevor** es ausgeführt wird;
  ein registriertes Wort wird nie erneut vorgelegt — nur eine neue Messung öffnet es.
- **Der Rat entscheidet Architektur und Ethik** — gefüttert: gemessene Recherche (URL, Zahl,
  Quelle) + der volle Roster (`~/projects/omegaflow-work/ui-seats.md`). Fehlt eines: erst holen.

## Dokumente

- Jedes Prosa-Dok trägt den Kopf: `title · class · date · sha256 · status · see-also`; der
  `sha256` deckt den **Body ohne Kopf** (`omega_sh sha <file>`).
- Die **Burn-Zeile** nennt die **Bilanz**: `open <usd> · close <usd> · cap 0.15 · closed <n> · Grund: …`.
  `closed` = wie viele offene Punkte **dieser Pass geschlossen hat**. Das Gate vergleicht die Zahl
  mit dem git-Diff — eine Behauptung, die nicht stimmt, ist eine Fabrikation (A = A).
- Linien-Übergabe: `handover-<datum>-<linie>-folge<N>.md` — nur Offenes, ≤ 1 KiB, **in place**
  zusammenfassen; Konsumiertes wandert nach `archiv/`. Keine neue Datei je Pass.
- **Jeder offene Punkt nennt seine Datei(en)** — ein Punkt ohne Pfad ist eine Spur ohne Ort.
- **Der Zustand ist das Register** (`phi/sources.φ`), **die Geschichte ist git**. Ein Dokument,
  das Zustand oder Historie erzählt, ist Compliance-Theater.

## Code

- **A = A** — der Text nennt, was **ist**. Spekulationswörter (`vermutlich`, `probably`, `likely`)
  blockt das Gate: **messen statt vermuten**; ohne Messung `pending`.
- **0 honored** — Abwesenheit ist ein vollwertiger Zustand. Leer ist kein Default, Stille kein Absturz.
- **Keine Fabrikation:** `unwrap_or`, `_ => 0`, `max(1)`, `#[derive(Default)]` sind Fabrikate —
  eliminieren. Ein neu gefundenes Muster wird **im selben Atom** Gate-Fixture + Test.
- **`cargo check`: null Fehler UND null Warnungen.** Nie mit `#[allow(...)]` oder `_` zum
  Schweigen bringen — der Code verschwindet, nicht die Warnung. `cargo check` prüft Syntax, nicht
  Funktion: manuell verifizieren.
- **Name = Implementation.** Keine Docstrings; der Kommentar nennt, was **ist**. Diagnostik nennt
  den Zustand, nie die Erwartung (kein `failed`, `error`, `crash`, `fallback`).

## Sprache

- **Englisch** für Code, Kommentare, Diagnostik, Papiere, Specs (der gemeinsame Instrumentton).
  **Deutsch** für Eigennamen (`Archivar`, `Mathematikerin`), den Handover-Register und die
  Philosophie — dort, wo die Prosa selbst die Gegensteigung ist.
- **Berichte an den Operator: einfache Sprache** — kein Jargon, keine Pfade, keine Abkürzungen.
  Die Maschinensprache bleibt unter den Linien, wo sie präzise ist.

## Die Ökonomie

- **Eine Sitzung, ein Faden.** Ist der Faden gearbeitet: Übergabe, **frische Sitzung**. Die Kosten
  sind quadratisch in den Turns, linear im Kontext.
- **Billiges Modell für die Routine** (Suchen, Karten, Form), das stärkere nur für den Kern.
- **Die Übergabe trägt nur das OFFENE.** Der teuerste Turn ist der, der nicht passiert.

## Die Grenze

- **Die Wände sind Werkzeug-Regeln** (`opencode.json`, `permissions`): der öffentliche Baum hat
  `read`/`edit`/`shell` auf den privaten Pfad `deny`; `AGENTS.md`, `opencode.json` und `.opencode/**`
  sind schreibgeschützt; kein Web, kein Push. Kein Gateway, kein Provider am Haus.
  (Der frühere `bwrap`-Käfig `bin/sandbox`/`agent_sandbox` ist am 2026-10-17 entfernt — er deckte
  nur `bash`, nicht die `read`/`edit`-Tools.)
- Werkzeuge: `archive_search` (nie webfetch/websearch) · `rg` für den schnellen Blick · `sread`
  für Fenster. Karte: `docs/concepts/tools-map.md`.
