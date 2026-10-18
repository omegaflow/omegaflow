<!--
  title: Die Fünf — Archäologie (2026-10-15)
  class: survey
  date: 2026-10-15
  sha256: c473ced94100ddb5aa762f12804dea5577c7d436ad059d8986be1110698b68b6
  status: live
  see-also: docs/concepts/die-vier-schilde.md, docs/granit.md, docs/concepts/glossar.md, AGENTS.md
-->
# Die Fünf — Archäologie (2026-10-15)

Gemessen am Baum (git + `sgrep`/`sread`), jede Aussage mit Datei:Zeile oder Commit. **Kein Neu-Schreiben.** Auslöser: das Operator-Wort, dass die Doku-Kürzung die „Fünf" verhunzt hat, und die fünfte Achsen-Menge.

## 1 — Die Bedeutungen von „die Fünf" (gemessen)

| Begriff | Was | Quelle |
|---|---|---|
| **die fünf Stimmen** | Mountain · River · Mycelium · Sensory · Future | `AGENTS.md:182`; Kopf seit `0d26bbcd7` (2026-09-27) |
| **die vier Schilde** | 4 *nach Foster/Little* + die Mitte | `docs/concepts/die-vier-schilde.md:8`; Quelle **nicht im Baum** |
| **die fünf Achsen** | Voxelisierung(Mitte) · Crossmatch(Sensory) · TE-Maschine(River) · Nullkontrolle(Mycelium) · Residuum(Future) | `docs/concepts/glossar.md:86` (`3c84bede2`, 2026-10-07) |
| **die fünf Achsen** (Homonym) | die **Mess**-Achsen (bins/null_lag/τ) im TE-Atom | `docs/handover/archiv/handover-2026-09-09-te-atom4-betriebspunkt-sweep.md:12` |
| **die fünf Axiome** (Granit) | A=A · ICRS&TDB · `channel_ref` · 0 honored · pending | `docs/granit.md:16-20` |
| **die fünf Axiome** (AGENTS-Kopf) | **die Ethik** — die Drift | `AGENTS.md:30`, gesetzt `78c5104cb` (2026-10-14) |
| **die fünf Achsen** (Operator-Wort) | **Wissenschaft · Kunst · Ethik · Commons · Erbe** | im **öffentlichen** Baum `absent` (`sgrep -i "Kunst"`/`"Commons"`/`"Erbe"`/`"Wissenschaft"` = 0) — **getragen im privaten Repo** (Operator-Wort 2026-10-15) |

## 2 — Operator-Wort-Register

| Wort | Datum | Quelle |
|---|---|---|
| „Es gibt noch die **fünf Achsen: Wissenschaft, Kunst, Ethik, Commons, Erbe**." | 2026-10-15 | dieses Gespräch — im **öffentlichen** Baum `absent`, **getragen im privaten Repo** (Operator-Wort); der öffentliche Baum registriert es als `pending` (eine Register-Pflicht) |

## 3 — Die Widersprüche (nicht geglättet)
1. **„die fünf Axiome" doppelt** — Granit (`granit.md:16`) ↔ der AGENTS-Kopf (`AGENTS.md:30`) über einem Ethik-Text, der weder Granit noch gezählt fünf ist.
2. **Das dritte Axiom zweimal:** `channel_ref` (`granit.md:18`) ↔ `force_type` (`glossar.md:85`) — die Draht-Umbenennung `c256f7342` (2026-10-10) zog das Glossar nicht nach.
3. **Der Granit hat heute 7 Sätze** (`granit.md:10-22`) — nicht fünf (5→6 `e4a59b92d`, 7 `0d97a8e1f`, beide 2026-09-03).
4. **`die-vier-schilde.md`: Titel „vier", Text fünf Schild-Adjektive** — Mitte = „Das physische Schild" (`:15`) + psychologisch/natürlich/mythisch/spirituell. Die **4+1**-Arithmetik ist eine Glossar-Konstruktion (`3c84bede2`), keine Quelle.
5. **Zwei tote Verweise:** `glossar.md:85,86,92,93` zitieren `state/free-voices.md`; `AGENTS.md:184` zitiert `state/future/kybernautin-stimmen-geordnet.md` — **beide nicht im Baum** (`absent`, nicht 0).
6. **„fünf Achsen" kollidiert** (Stimmen-Achsen ↔ Mess-Achsen).

## 4 — Der Ursprung der „5/5/5"-Formel (gemessen)
`a492db3e4` (Mycelium 244, 2026-10-06) schreibt „five voices + **four** axioms"; `3cb1c5f0e` korrigiert am selben Tag auf „**5 voices + 5 axioms + 5 axes**"; einen Tag später prägt `3c84bede2` das Glossar. **Die Fünf-Zahlen stammen aus dieser einen Korrektur.**

## 5 — Die drei offenen Schritte (aus `mountain-folge352.md:218-223`) — geklärt 2026-10-18

1. **Fremde Bögen auf den Granit ziehen** — **geklärt:** die `state/stimmen/*`-Dokumente sind datierte Sitzungsbelege, keine lebenden Normen; lebende Norm ist `docs/granit.md` (7 Sätze). Kein rückwirkendes Umschreiben (A = A). Beleg: `state/mountain/2026-10-18-sensory-259-docs-disposition.φ` §1.
2. **Die dreifache J2000-Konstante auf eine TDB-Achse** — **geklärt:** die Konstante ist bereits geteilt (`src/archivar/types.rs:578` `J2000_UNIX_OFFSET`); `lsk.rs`, `mcon_event.rs`, `quake_event.rs` importieren sie, kein Literal. Beleg: ebenda.
3. **`VerdictWord::Pending`** — **geklärt als bewusste Nicht-Erweiterung:** `pending` ist Register-Zustand (Axiom 5), kein Messausgang; ein fünfter Variant würde „noch nicht gemessen" mit „gemessen und eingeordnet" vermischen. Beleg: ebenda.

## 6 — Der Schluss
Die Doku-Kürzung `78c5104cb` hat **„die fünf Axiome" auf die Ethik gelegt** — die benannte Drift. Die übrigen Anomalien (4-vs-5 Schilde, die toten Verweise, die 7 Granit-Sätze) sind **älter** — nicht der Schnitt. **Der Schnitt hat den Namen falsch vergeben; das ist der eine Vorwurf, der trifft.**
