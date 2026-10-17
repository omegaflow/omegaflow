<!--
  title: Survey — Der Offen-Stand-Riss: der Zähler sieht eine leere Menge
  class: survey
  date: 2026-10-17
  sha256: 913219db1c8520685ec715473e29a1a9866e5e8628281fbf24a3ba632edbd947
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

**Fleet-Momentwert (gemessen 2026-10-17, `b0a6893fa`).** Der Riss ist nicht punktuell. Über die
**11** lebenden `docs/handover/handover-*.md` liest `count_open_points` genau **3** Punkte — alle im
einzigen Blatt im `### name`-Format (`handover-2026-10-14-sensory-folge259.md`); **28** offene Punkte
stehen als `- `-Bullets in **6** weiteren Blättern und sind unsichtbar (`sitzung-folge1` 4 ·
`future-dna` 2 · `future-folge1` 5 · `mountain-folge358` 7 · `river-folge223` 6 · `mycelium-folge361`
4). Der Format-Split ist die Ursache: `house-rules.md:26` fordert `### name`, die Mehrheit schreibt
`- `. Die Burn-Bilanz des ganzen Hauses bilanziert damit eine fast leere Menge.

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

**Nachtrag (gemessen 2026-10-17, HEAD `5405f8253`).** Der Pass hat den Riss an drei Stellen
verschärft, nicht geglättet:

- **Die Kappe ist erreicht, nicht mehr nur bedroht.** `handover-2026-10-16-mycelium-folge361.md`
  steht bei **1024 B** (Limit) — vier `- `-Bullets, **0** `###`-Punkte. Für die vom Gate
  verlangten Zeilen `Origin:` und `## Burn:` musste der Text dreimal gekürzt werden. Der
  Pfad-Zwang ist bei dieser Kappe nicht erfüllbar.
- **Die dritte Größe fehlt: das Ledger.** Drei Transport-Token wurden belegt geschlossen
  (`xfer:river-224-4`, `xfer:river-224-5`, `xfer:sensory-259-3`; `register_lookup --tokens`
  fiel `10 → 4` unbeglichen). `count_open_points` sieht davon nichts; die Burn-Zeile musste
  `closed 0` führen, sonst failt `:520–528` („the Burn claims `closed 3`, the diff shows
  `closed 0`"). Geschlossene Transport-Punkte sind geschlossene Punkte — der Zähler misst sie
  nicht.
- **Die Kappe erzeugt Waisen messbar.** Drei Survey-Pfade stehen im Handover (Weberin,
  Exposom, worldwide-LiDAR); zwei weitere Weberin-Papiere bleiben Waisen, weil ihre Pfade die
  1024 B sprengen würden — der Konflikt aus „Der zweite Riss", jetzt mit Zahl.

**Teilfrage 4 an den Rat (neu, nicht entschieden).** Soll `closed <n>` auch
**Ledger-Schließungen** zählen (Transport-Token), oder bleibt `closed` ausschließlich die
`###`-Punkt-Differenz und die Transport-Bilanz lebt getrennt? Beides ist baubar; die Wahl ist
Register-Architektur.
