<!--
  title: House Rules — die Essenz (was gilt)
  class: concept
  date: 2026-10-19
  sha256: bb12a16eaffc40d6b2f889497f1af677bda5e32aab02b2e71d1870fcaded7752
  status: live
  see-also: AGENTS.md docs/concepts/tools-map.md docs/granit.md docs/council.yaml
-->
# House Rules — die Essenz

Hier steht nur, was **kein Gate und kein `--help`** tragen kann. Was ein Gate prüft, ist
ausgeführt statt erzählt. Seit der Streichrunde (2026-10-19, Rat R-2026-10-19-a, F2) ist dies
die **eine** Denk-Quelle: die Rat-Anweisung ist darin aufgegangen und liegt als *gestrichen*
im Archiv der Werkstatt (`zustand/rat/archiv/`).

## Die Linie — ein Pass

**Erste Handlung — deine drei Stellen, nichts sonst:** der **Dispatch**
(`~/projects/omegaflow-work/post/`, die eine Aufgabe der Leitstelle, Name trägt die Richtung) ·
dein **Handover** (`~/projects/omegaflow-work/handover/`, die neueste deiner Linie, ≤ 1 KiB,
nur Offenes) · deine **Post** (dasselbe `post/`). Lies nur die Stellen, die die Aufgabe nennt
(`sread <f> --offset --limit`).

- **Ein Pass.** Kein Planungstheater, keine Liste, kein Consent-Stopp für Bekanntes.
- **Kein Dispatch für dich?** → arbeite deine offenen Punkte, so viele wie möglich — nicht so wenige wie nötig.
- **Vor jeder NET-Suche: `bin/where <begriff>`** — erst prüfen, was das Haus schon hat.
- **Ende des Passes:** `register_lookup --orphan-docs` — deine Waisen in die Übergabe falten.
- **Ein Pass schließt Punkte — so viele wie möglich.** Jeder offene Punkt ist ein `### name` mit
  `Status · Trigger · Lage · Blockade · Braucht` — sonst ist die Zahl unsichtbar.
  „Mindestens ein Punkt" ist die Hintertür: **Taucher laufen, bis die Punkte zu sind**.
  Gemessen wird, was **geschlossen** wurde.
- **Bringschuld vor der Übergabe.** Jeder offene Punkt nennt unter `Braucht` den wörtlichen,
  kopierbaren Schritt, der ihn löst; die Übergabe ist die Bringschuld, nicht die Kür.
- **Grenze:** commit+push = deine Hand (eigene Pfade, Englisch); **Send** und jeder
  dritt-wirksame Akt = Operator-Hand (per-Akt-Wort); Architektur/Ethik → der Rat.
  **„Send" ist genau das — Versand, nichts sonst:** `smail --send`, ein Formular/Antrag,
  ein CDN-Upload/Rückzug, ein Konto-/Key-Akt. Alles davor ist **Arbeit** und dein Boden.
  Die Grenze darf **nie** als Deckmantel dienen, Arbeit zu lassen.

## Der Akt am Gegenüber (Operator-Wort)

- **Nur der Operator versendet.** Die Maschine bereitet bis zur Kante vor (Entwurf,
  `--dry-run`) und legt **einen** Satz vor — **fertig, nicht angefragt**.
- Ein Operator-Wort wird als `Wort | Datum | Quelle` registriert, **bevor** es ausgeführt wird;
  ein registriertes Wort wird nie erneut vorgelegt — nur eine neue Messung öffnet es.
- **Der Rat entscheidet Architektur und Ethik** — gefüttert: gemessene Recherche (URL, Zahl,
  Quelle) + der volle Roster. Fehlt eines: erst holen.

## Der Rat

- **Vor jeder Runde wird gefragt** — freie Schwarm-Sitze, UI-Chats und paid gleichermaßen;
  erst das Operator-Ja startet die Runde. Bei paid setzt `bin/rat_guard --grant` den Grant;
  ohne frischen Grant bricht der Aufruf ab. **Eine Runde ohne diese Kette ist ein Regelverstoß,
  das Ergebnis zählt nicht.**
- **Der Rat ist die fünf Stimmen, nicht die Modelle.** Kanonisch sind `docs/granit.md` (Axiome)
  und `docs/council.yaml` (Stimmen) — in den Prompt kommt der **Wortlaut** dieser beiden Dateien,
  nie eine gekürzte Zeile.
- **Recherche geht vor jeden Sitz.** `archive_search <kw> --<kanal>` (`--arxiv --openalex
  --semanticscholar --crossref --openaire --datacite --zenodo --dblp --github --crates
  --stackexchange`); Websuche ist keine Literaturbasis. Pflicht je Befund: **Host/URL · Werkzeug ·
  Datum · Zahl**. Hausdokumente allein zählen nicht. „Offen" ist ein voller Ausgang, aber nur als
  **gemessenes** Offen.
- **Reihenfolge:** Recherche → Formular (`zustand/rat/<datum>-<thema>-formular.md`) → 5 freie
  Schwarm-Sitze (`shuf` aus der frischen Alive-Liste, Auswahl **vorher veröffentlicht**) → 5 UI-Sitze
  → paid **nur** bei Uneinigkeit und nur mit Operator-Ja. Ein untauglicher Sitz wird **ersetzt**
  und der Ersatz veröffentlicht — „untauglich ausweisen" reicht nicht.
- **`reasoning_effort: low`, `max_tokens` ≥ 6000.** `high`/`max` kappt die Antwort; ohne Level
  kommen abgeschnittene Verdikte.
- **Leiter:** `free > Mistral inklusive > DeepSeek paid > Mistral paid`. DeepInfra nur für PII
  und Frontier, **nie** Linienarbeit.
- **Der Schwarm trägt nicht.** Sein Verdikt ist Mindermeinung, seine Risse sind der Wert.
- **Streich-Test:** eine Stimme ist tragend, wenn ihr Riss das Verdikt ändert oder stürzt, sobald
  man ihn streicht. Wer nur bestätigt, sitzt die nächste Runde aus.

## Dokumente

- Jedes Prosa-Dok trägt den Kopf: `title · class · date · sha256 · status · see-also`; der
  `sha256` deckt den **Body ohne Kopf** (`omega_sh sha <file>`).
- Die **Burn-Zeile** nennt die **Bilanz**: `open <usd> · close <usd> · cap 0.15 · closed <n> · Grund: …`.
  `closed` = wie viele offene Punkte **dieser Pass geschlossen hat**; das Gate vergleicht die Zahl
  mit dem git-Diff (A = A).
- Linien-Übergabe: `handover-<datum>-<linie>-folge<N>.md` — nur Offenes, ≤ 1 KiB, **in place**
  zusammenfassen; Konsumiertes wandert nach `handover/archiv/`.
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
  Schweigen bringen. `cargo check` prüft Syntax, nicht Funktion: manuell verifizieren.
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
- **Ablage:** `$OMEGAFLOW_TMP` hat genau vier Ordner (`ci/`, `out/`, `downloads/`, `scratch/`) mit
  TTL 7/14/2 Tage; was bleibt, wandert in `sources/`, `experiments/` oder `docs/`, nie zurück.
  `bin/layout_check` misst das; `bin/sources_index` erzeugt die Karte über den CDN-Cache.

## Die Grenze

- **Die Wände sind Werkzeug-Regeln** (`opencode.json`, `permissions`): der öffentliche Baum hat
  `read`/`edit`/`shell` auf den privaten Pfad `deny`; `AGENTS.md`, `opencode.json`, `.opencode/**`
  und `tools/gate/**` sind schreibgeschützt — **über `edit` UND `shell`**. Kein Web, kein Push.
  Kein Gateway, kein Provider am Haus.
- **`opencode.json` ist lokal, nicht versioniert** (`.gitignore`, Operator-Wort 2026-10-19): die
  Wand lebt auf der Maschine, nicht im Klon. Ein frischer Klon hat keine Schranken — wer dort
  arbeitet, setzt sie zuerst (die Vorlage gehört ins private Haus, nie ins öffentliche Repo).
- Werkzeuge: `archive_search` (nie webfetch/websearch) · `rg` für den schnellen Blick · `sread`
  für Fenster. Karte: `docs/concepts/tools-map.md`.
