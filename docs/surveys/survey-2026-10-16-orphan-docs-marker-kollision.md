<!--
  title: Survey — Orphan-Docs: Marker-Kollision (nicht Aufgaben)
  class: survey
  date: 2026-10-16
  sha256: b982402d05043f6f0fed73dd95e0fd7cdccf9c74489733960ddbb30b26c5f877
  status: done
-->
# Survey — Orphan-Docs: Marker-Kollision, nicht Aufgaben

**Frage.** Sind die vom Zähler gemeldeten `register_lookup --orphan-docs`-Docs offene Aufgaben —
oder lexikalischer Lärm?

**Messung.** Das Gate steht in `tools/register/src/bin/register_lookup.rs` (`orphan_docs`:839,
`OPEN_MARKERS`:31). Es matcht die Wörter
`offen · pending · wartet · ausstehend · n(ä|ae)chster schritt · wiedervorlage · blocked · request-only`
als **eigenständiges Wort in Statuskontext** (Inline-Code wird vorher gestrippt,
`marker_in_status_context`:97), über `.md` unter `docs/{auftrag,blatt,concepts,paper,plans,surveys,specs}`,
und meldet das Doc, wenn **keine** lebende Übergabe seinen Pfad nennt.

**Riss (A=A).** `OPEN_MARKERS` (`tools/register/src/bin/register_lookup.rs:31`) führt die Wörter
`offen` und `pending` — beide **Haus-Kanon**: `pending` ist der 0-Kanon-Zustand (Wert existiert,
Ernte fehlt), `offen` heißt offene Lizenz / Open Data. Das Gate zählt damit die **eigene Vokabel** als
„offen". Das ist kein Messfehler, sondern ein **struktureller Riss** zwischen Register-Gate und
0-Kanon: ein Kanon-Zustand wird als Arbeit gelesen. Gemessen:
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — **27** Marker, **alle** die Kanon-Vokabel
  `pending` (`| … | pending |`-Zellen, „cells 1–25 `pending`", „never 0.0"). **Kein** ungetragener Punkt.
- `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` — **52** Marker: „offen" = offene
  **Lizenz**, „`pending` (Timeout)" = HTTP-/Mess-**Zustand** in Tabellenzellen.
- Die **12** `docs/paper/*.md` sind durchweg `pending`-Kollisionen, kein Auftrag.
- **Daneben echt:** `docs/auftrag/auftrag-flyby2-kette.md` (`## Frist / Owner / nächster Schritt`),
  `docs/auftrag/auftrag-2026-10-11-wartend-aufloesung.md` (`## Teil C — offene Messlücken`).

**Momentwert (mit Schwelle).** `register_lookup --orphan-docs` = **56** (gemessen 2026-10-17 an
`faa73108a`, `./target/release/register_lookup --orphan-docs`). Der Zähler ist **live**: 2026-10-16 =
**62**, Minuten später 56, am 2026-10-17 53 und 56 — **Schwelle 53–62** (parallel committende Linien
tragen ihre Docs). Der Wert ist eine **Momentaufnahme, kein Bestand** — und **keine Aufgabenliste**.

**Verdikt.** Eine belastbare Trennung *Aufgabe / Lärm* ist mit der jetzigen Heuristik **nicht
berechenbar** — das ist der Befund, kein Messfehler. Der Zähler mischt Kanon-Vokabel (`pending`,
`offen`) und echte Träger-Lücke. **Er ist kein Aufgabenregister.**

**Rat-Stand R4 (2026-10-17) — 5 Sitze, 4 Positionen** (`state/rat/2026-10-17-rat-offene-fragen.md`).
Der Rat ist gespalten; ich entscheide nicht allein:
- **Leitstelle hausweit** — DeepSeek V4 Pro, Lumo 2.0 Max. DeepSeek: `pending`/`offen` fallen aus der
  Marker-Liste, Marker nur in echter Statuszeile. Lumo: Leitstelle + `--owner`-Filter, `pending` und
  `offen` **getrennt** listen.
- **Abgeleitete Abfrage** — Sakana Fugu: jede Linie trägt ihre eigenen Docs; die Leitstelle nur als
  **residualer** Träger (`pending`). `pending`/`offen` bleiben Kanon, zählen aber **nur in einer
  Status-Grammatik**, nie als Substring.
- **Beides geschichtet** — GLM-5.3: „die Dichotomie ‚Leitstelle ODER Linie' ist ein **falscher
  Rahmen**"; `--orphan-docs` soll **`channel_ref`-Abwesenheit** messen, nicht Marker-Präsenz.
- **Linienlokal mit Verweigerungsvorbehalt** — Mistral Large 4: die Leitstelle nur als „letzte
  Adresse der Messung"; `pending` fällt aus `OPEN_MARKERS`.

**Zwei einige Befunde** (tragen alle vier Positionen):
1. `--orphan-docs` **misst die falsche Sache**: Marker-Präsenz statt Träger-`channel_ref`-Abwesenheit
   (GLM, Fugu, Mistral, unabhängig).
2. Die Kanon-Vokabel **`pending`/`offen` in `OPEN_MARKERS`** erzeugt den Lärm (alle).

Der verbindende Kern (Fugu): eine **geborgte Adresse** — der Träger als gespeichertes Flag statt
abgeleiteter Abfrage. **Kein stilles `pending`:** der Riss bleibt offen; die drei Wege (Owner-Filter ·
Status-Grammatik statt Substring · `pending`/`offen` aus der Liste) stehen dem Rat bzw. dem
Operator-Wort anheim.

**Grenze.** Eine abschließende Per-Linie-Zuordnung der *echten* Punkte ist hier nicht gemessen
(Näherung) — die Register-Heuristik kann sie nicht liefern. Der Momentwert:
`./target/release/register_lookup --orphan-docs`.
