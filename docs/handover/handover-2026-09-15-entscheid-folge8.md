<!--
  title: Handover — Entscheid-Folge VIII (Stand 2026-09-15)
  session: Entscheid-Folge VIII
  class: handover
  date: 2026-09-15
  sha256: f9ca756dee28d08ae51b299b9ab87dabe1a43d49b85f5fa4bef9963bf9aa1ac5
  status: live
-->
# Handover — Entscheid-Folge VIII (2026-09-15)

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

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind als Nachricht an ihre Linie
überführt (nie in ein fremdes Handover geschrieben).

## Zugangsanfragen — Notwendigkeits-Abgleich beim Operator (Chat übergeben)

Die Notwendigkeit einer Zugangsanfrage entscheidet sich am Quellen-Register, nicht
an der Korrespondenz. Der Abgleich (14 versandte Mails aus `state/mail/mail_ledger.φ`
+ Register-Befund je Anfrage) liegt beim Operator zur Weitergabe an die Ernte-Linie;
Ernte misst je Anfrage und trägt das Verdikt in `folge28` ein. (Schritt: Operator
übergibt den Abgleich; Ernte misst.)

## NIM-Spezialmodelle — riva-translate angedockt, parse/embedding offen

- `nvidia/riva-translate-4b-instruct-v2` als reine Ruf-Stufe gebaut:
  `tools/service/src/bin/riva_translate.rs` (std+curl, system-Tag `<from>-<to>`,
  OpenAI-Antwortformat; 6 Tests grün, de→en am echten NIM-Pfad gemessen). Key:
  `NVIDIA_API_KEY` in env oder `.secrets.local` (dort noch nicht eingetragen).
  Kein Tool-Ruf → kein Agent. (Schritt: `NVIDIA_API_KEY` in `.secrets.local`
  eintragen; ersten Konsumenten benennen — welche fremdsprachige Quelle.)
- `nvidia/nemotron-parse-2.0` gemessen: VLM, leerer `content` auf Text-Chat →
  keine Text-Ruf-Stufe. Nur bei einem Dokument-/Bild-Parse-Konsumenten neu messen.
  (Schritt: Konsument benennen — Operator.)
- Embedding/Rerank über NIM: `404` gemessen, semantische Nähe nicht bedienbar →
  pending über einen anderen Weg. (Schritt: anderen Embedding-Pfad benennen.)

## Free-Gold-Recherche — `free-research` installiert, Transport instabil

- Rat-Verdikt (2026-09-15): der strikte Filter (Preis frei ∧ Vertrag trägt ∧
  Verhalten gemessen) schließt kilo (anonym), openrouter free (Prompt-Training),
  GLM (gestrichen) und den opencode-Provider (kein Key) aus; einziger
  free-gold-Kandidat ist `nvidia/nemotron-3-super-120b-a12b`. Kalibrier-Gate 7/7
  (Grounded-QA, Fabrikations-Falle mit ABSENT bestanden). Agent `free-research`
  in der Global-Config installiert (read-only: edit/task deny, `read` verweigert
  `.secrets.local`/`.env`/`state/**`, bash nur archive_search + proton-wg).
- Route-Benchmark (2026-09-15, 6 offene Ernte-Kandidaten × 2 Reasoning-Konfigs,
  Wahrheit = curl-Status): **0/6 Beispiel-URLs verifiziert** in beiden Konfigs.
  Thinking an (Default): 5/6 ehrliches ABSENT, Median 7,1 s. Thinking aus
  (`chat_template_kwargs.enable_thinking=false`): 0 ABSENT, zu jedem Kandidaten
  eine Route, aber keine lebende Datei (Median 1,8 s) → ohne Tools fabriziert das
  Modell plausible URLs. Keine harte Nuss geknackt; nur die goes_abi-Route war
  korrekt (Compiler-eigene S3-Route), die Beispiel-Datei erfand einen Tag.
  Konsequenz: `free-research` nur mit Tools (webfetch/archive_search) und Thinking
  an; jede URL vor dem Nennen verifizieren — die reine Modellkenntnis trägt nicht.
- Offen: der Transport liefert 46 % HTTP 503 → Retry nötig; dauerhafte Zuordnung
  erst nach einer Stabilitäts- und Tool-Messung. (Schritt: opencode neu starten;
  mit Tools gegen eine echte Ernte-Frage messen.)

## Warten auf Rückmeldung (extern gebunden — kein Datum)

- adoption — Drei-Mail-Block (Toth/Turyshev/Markwardt) nicht gesendet; der Blocker
  Bande-Split ist geschlossen. (Schritt: senden — Operator.)
- GitHub Support — User→Org / HTTP 422: Antwort offen. (Schritt: Postfach prüfen.)
- Rubin RSP-Datenrechte — Antwort an Shaughnessy (SLAC) gesendet 2026-09-15;
  Antwort offen.
- NSE/Haug — Anfrage raus, Antwort offen (Keimer).
- GAVO — geschlossen: Demleitner hat die ivoa-Umstellung am 2026-09-14 bestätigt
  (`state/mail/mail_ledger.φ`).

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
