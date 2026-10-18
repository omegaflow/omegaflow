# Rat-Frage R-2026-10-17-i — Was zählt der Offen-Stand?

**Die Frage in einem Satz:** Wenn das Gate „offene Punkte" zählt und die Burn-Zeile `closed <n>` bilanziert — was soll gezählt werden, damit die Zahl die *Tat* misst und nicht eine leere Menge?

## Die gemessene Lage (2026-10-17)

1. **Der Zähler liest nur `### `.** `count_open_points` (`tools/gate/src/bin/commit_check.rs:188`) zählt ausschließlich `### `-Überschriften unter `## Offen*`. Die lebenden Handover führen „Offen — eigen" als `- `-Bullets → der Zähler liest **0**.
2. **Folge:** Die Burn-Zeile `closed <n>` bilanziert eine leere Menge; die Prüfung „a pass closes at least one point" sieht `0 → 0`. Beleg: ein Commit, der real einen CI-Rot-Kern und 5 Orphan-Docs räumte, meldete nur `the open did not shrink (0)`.
3. **Migrationsfalle:** Wandelt man die vier Bullets in `### name` um, springt der Zähler `0 → 4` → hartes „the open grew (0 -> 4)". Das Gate sperrt das richtige Format.
4. **Kappe vs. Pfad:** Die 1-KiB-Kappe und die Pflicht „jeder offene Punkt nennt seine Datei(en)" vertragen sich nicht, wenn fünf Survey-Pfade in einen Punkt müssen. (Nachgemessen: vier Bullets + fünf Pfade = 970 B, Kappe 1024 → passt; der frühere 1024-B-Stand kam von **geschlossenem Transport**, nicht von den offenen Punkten.)
5. **Die dritte Größe fehlt:** Drei Transport-Token wurden in einem Pass belegt geschlossen; `count_open_points` sieht davon **nichts**. Die Burn-Zeile musste `closed 0` sagen, sonst failt `commit_check` mit „the Burn claims `closed 3`, the diff shows `closed 0`".

## Die Fragen

1. Soll `count_open_points` auch `- `-Bullets unter `## Offen*` zählen (wie die lebenden Handover sie schreiben) — oder müssen die Handover auf `### name` migriert werden?
2. Wie ist der Henne/Ei-Konflikt der Migration aufzulösen — die Umstellung *ist* Wachstum und failt am Gate?
3. Gilt die 1-KiB-Kappe gegen den Pfad-Zwang — oder ist die Kappe zu klein/falsch dimensioniert?
4. Soll `closed <n>` auch **Ledger-Schließungen** (Transport-Token) zählen — oder bleibt `closed` ausschließlich die `###`-Punkt-Differenz und die Transport-Bilanz lebt getrennt?

## Die Optionen (für 1 und 4; 2/3 folgen daraus)

- **A** — Zähler erweitern: `- `-Bullets zählen, `closed` zählt auch Ledger-Schließungen. Die Zahl misst die echte offene Menge.
- **B** — Migration + Ausnahme: Handover auf `### name`, der Migrations-Commit einmalig von „no growth" ausgenommen (Operator-/Rat-Wort).
- **C** — Status quo benennen: `###`-only bleibt; `closed 0` ist dann ehrlich, aber das Gate misst dauerhaft eine leere Menge.

Bitte: **Verdikt + der stärkste Riss, den du siehst.** Keine Umsetzung — die ist Folge, nicht Frage.
