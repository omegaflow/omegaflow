<!--
  title: Survey — Der Offen-Stand-Riss: der Zähler sieht eine leere Menge
  class: survey
  date: 2026-10-17
  sha256: 728aa406ede2198921e3f95183d376dcdd9dfc381f500a0f899cc438ddda9139
  status: live
-->
# Survey — Der Offen-Stand-Riss: `count_open_points` zählt die Bullets nicht

**Frage.** Misst die Burn-Bilanz (`closed <n>`) den offenen Stand wahr — oder zählt sie ins Leere?

**Messung (Datei:Zeile).**
- `tools/gate/src/bin/commit_check.rs:188` `fn count_open_points` zählt **nur**
  `t.starts_with("### ")` unter einem `## `-Abschnitt, dessen Rest mit `offen` beginnt
  (`:193–199`).
- `tools/gate/src/bin/commit_check.rs:204` `fn claimed_closed` liest das `closed <n>` aus der
  Burn-Zeile.
- `:511–519` vergleicht `claimed` gegen `measured = was.saturating_sub(now)`; `:506–509` warnt
  „the open did not shrink".

**Momentwert (gemessen 2026-10-17).** `docs/handover/handover-2026-10-16-mycelium-folge361.md`
führt `## Offen — eigen` als **4** `- `-Bullets; der Zähler liest **0**. Zahl gegen Tat:
- der Pass räumte real den `ci-gate`-`format`-Kern (drei Dateien, `a582cb5be`) und 5 Orphan-Docs
  (54 → 49, `d8d6df38a`);
- die Burn-Zeile sieht davon nichts: `closed 0`, Warnung „a pass closes at least one point",
  `0 → 0`.

**Die Migrations-Falle (gemessen).** Wandelte ich die vier Bullets in `### name` um, stiege
`count_open_points` `0 → 4`; `:501–505` failt hart: „The open grew (0 -> 4) — a pass closes
points". **Das Gate sperrt das Format, das die Regel verlangt** (`docs/concepts/house-rules.md:26`
fordert `### name`).

**Der zweite Riss — Kappe gegen Pfad-Zwang.** `house-rules.md:27` verlangt ≤ 1 KiB;
`house-rules.md:51` verlangt „jeder offene Punkt nennt seine Datei(en)". Die fünf Survey-Pfade
(zur Entwaisung — `register_lookup --orphan-docs` matcht den Basename ≥ 8 Zeichen,
`tools/register/src/bin/register_lookup.rs:829–836`) treiben folge361 auf **1500 B**. Kappe kürzen
heißt Waisen erzeugen; das Gate meldet `handover-cap: … (cap 1024)` — **nicht blockend** (`:495`),
also ein benannter, kein harter Riss.

**Warum nicht meine Hand.** `count_open_points` und die Kappe sind per Operator-/Rat-Wort in Code
gegossen (`f0d7543a2`, `ad4195c82`, `f161f4408`); was `closed` **bedeutet**, ist Register-Architektur.

**Rat-Frage (nicht entschieden, nicht still `pending`).** Drei Wege:
- **A — Zähler erweitern:** `count_open_points` zählt auch `- `-Bullets unter `## Offen` → keine
  Migration, keine Falle.
- **B — Migration + Ausnahme:** Handover auf `### name` umstellen, den Migrations-Commit einmalig
  von „no growth" ausnehmen.
- **C — Status quo benennen:** `###`-only bleibt; `closed 0` ist ehrlich (der Pass hat keine
  `###`-Punkte) — aber der Zähler misst dauerhaft leer.

Ablage der Wahl: `post/in/2026-10-17-mycelium-an-future-ratsanfrage-offen-zaehlung-und-kappe.md`
(Punkt `xfer:mycelium-361-3`).

**Momentwert ist live.** Der Befund hängt an `commit_check.rs:188`, nicht an der Zahl — `0` ist
stabil, solange die Handover Bullets führen.
