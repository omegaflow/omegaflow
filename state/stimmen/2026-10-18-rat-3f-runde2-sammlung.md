# Sammlung Runde 2 — F1/F2/F3 (2026-10-18)

**Drei Sitze** haben die drei geschlossenen Messlücken gesehen und **neu abgestimmt**:
Fugu Max High (UI) · Lumo 2.0 Max (UI) · DeepSeek V4 Pro `#low` (paid).
Der **Schwarm** wurde in Runde 2 nicht erneut gefahren; seine Stimmen aus Runde 1 stehen als Kontext daneben.

Rohmaterial: `2026-10-18-rat-3f-fugu-runde2.txt` · `2026-10-18-rat-3f-lumo-runde2.txt` ·
`2026-10-18-rat-3f-runde2-deepseek.txt`. Fragebogen: `2026-10-18-rat-3f-runde2-formular.md`.

## Die Bilanz je Frage

### F1 — Quittung im Token-Ledger

| Quelle | Runde 1 | Runde 2 | geändert |
|---|---|---|---|
| Schwarm | A | *(nicht gefahren)* | — |
| Fugu (UI) | B + S2 | **A + S2** | ja |
| Lumo (UI) | C + S2 | **A + S2** | ja |
| DeepSeek (paid) | pending + S2 | **C + S2** | ja (aus pending) |

**Der Befund kippt die UI-Sitze auf A.** Fugu und Lumo wechseln beide von einer Nicht-A-Stimme
zu **A + S2**; ihr altes Argument (der Empfänger kann nicht quittieren / read-only ableiten)
trägt angesichts des Befunds nicht mehr — der Arm scheiterte nie, er fiel per Haus-Regel.
Nur DeepSeek bleibt bei **C + S2** und nennt als Riss, dass der neue Akt „erneut ein solcher
Eingriff" wäre. → **2:1 für A** (+ Schwarm-A aus Runde 1). **S2** ist in allen drei Stimmen gesetzt.
F1 ist damit **rat-seitig auf A konvergiert**, offen bleibt die Operator-Schließung.

### F2 — `register-coverage`-Concurrency

| Quelle | Runde 1 | Runde 2 | geändert |
|---|---|---|---|
| Schwarm | A | *(nicht gefahren)* | — |
| Fugu (UI) | B | **A** | ja |
| Lumo (UI) | B | **B** | nein |
| DeepSeek (paid) | C | **B** | ja |

**F2 bleibt gespalten.** Fugu kippt auf **A** („per-SHA bindet jeden Lauf an genau einen SHA");
Lumo hält **B**; DeepSeek wechselt von C auf **B**. → **2:1 für B** (Fugu allein auf A).
Alle drei zitieren denselben Riss: der Mid-Write-Riss trifft **nicht** das Gate; die echten
Writer (`register_ttl_apply.rs:176`, `register_terms_apply.rs:161`, `register_block_remove.rs:107`)
truncaten in place und brauchen **temp+rename** — ein eigener, belegter Punkt. **F2 offen.**

### F3 — Register liest `state/stimmen` nicht

| Quelle | Runde 1 | Runde 2 | Zusatz (RAT-ID vorher?) |
|---|---|---|---|
| Schwarm | A | *(nicht gefahren)* | — |
| Fugu (UI) | B | **A** | ja |
| Lumo (UI) | C | **A** | nein |
| DeepSeek (paid) | B | **A** | nein |

**F3 konvergiert auf A — 3:0.** Alle drei kippen: A ist ohne RAT-ID baubar (Hook-Test
`tools/register/**` → exit 0), B ist auf einen `## An`-Block je Handover gedeckelt
(`HANDOVER_CAP = 1024` gegen 521–542 B/Block; 7 von 9 Handovern schon darüber).
Der **Zusatz** (muss die Wahl als RAT-ID im Index stehen, *bevor* sie gebaut wird?) ist
**1:2 nein** — nur Fugu sagt ja. Riss bei allen: ob `state/stimmen/` als neue Dokumentwurzel
im Reader Architektur-Konsequenzen hat (Index-Pflege) und wer Schreib-/Indexkonsistenz garantiert.

## Der Befund

1. **F1 und F3 konvergieren auf A** (F1 2:1 + Schwarm; F3 3:0 + Schwarm), nachdem die Messlücken
   geschlossen sind. Das alte „UI gegen Schwarm"-Muster aus Runde 1 ist **gekippt**.
2. **F2 bleibt offen** (2:1 für B gegen Fugu-A); der entscheidende Riss — in-place-Writer brauchen
   temp+rename — ist von keiner Option adressiert und bleibt ein **eigener Punkt**.
3. **Der Zusatz zu F3** ist mehrheitlich **nein**: der Bau braucht keine vorherige RAT-ID;
   die RAT-ID ist Adresse, nicht Zulassung.
4. **Schließung liegt beim Operator.** Die Leitstelle hat die Runde gefahren und belegt;
   verdikt- und schließfähig ist der Stand, entschieden ist er nicht.
