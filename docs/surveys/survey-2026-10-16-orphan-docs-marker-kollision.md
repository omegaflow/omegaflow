<!--
  title: Survey — Orphan-Docs: Marker-Kollision (nicht Aufgaben)
  class: survey
  date: 2026-10-16
  sha256: 101614e66c59638d55f4e0fb483c8602b95b90cff041d17d03ff4e0982e858e0
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

**Momentwert.** `register_lookup --orphan-docs` = **53** (gemessen 2026-10-17 an `278fb1202`; Lauf
`./target/release/register_lookup --orphan-docs`, zweimal identisch). Der Zähler ist **live**: erste
Messung 2026-10-16 = 62, Minuten später 56 (andere Linien committen parallel Träger) — eine
**Momentaufnahme, kein Bestand**. Die Zahl ist **keine Aufgabenliste**.

**Verdikt.** Eine belastbare Trennung *Aufgabe / Lärm* ist mit der jetzigen Heuristik **nicht
berechenbar** — das ist der Befund, kein Messfehler. Der Zähler mischt Kanon-Vokabel (`pending`,
`offen`) und echte Träger-Lücke. **Er ist kein Aufgabenregister.**

**Rat (Architektur) — nicht meine Hand.** Drei Wege:
1. `--owner`-Filter für `--orphan-docs` (analog `--orphans --owner <linie>`).
2. Marker nur in **echter Statuszeile** (nicht in Tabellenzelle/Prosa).
3. `pending`/`offen` aus der Marker-Liste nehmen (Kanon-Wörter).

Bis dahin bleibt der Zähler ein **Lärm-Signal**; wer trägt, entscheidet die Leitstelle bzw. je Urheber.

**Grenze.** Eine abschließende Per-Linie-Zuordnung der *echten* Punkte ist hier nicht gemessen
(Näherung) — die Register-Heuristik kann sie nicht liefern. Der Momentwert:
`./target/release/register_lookup --orphan-docs`.
