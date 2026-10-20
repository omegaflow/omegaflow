<!--
  title: Was ist was — Handover, Auftrag, Survey, Fadenübersicht, Register
  class: concept
  date: 2026-10-20
  sha256: 0a44fa8a3c7ce363d25496ad50602b10b243175d30b991aaff401f65d5ee79ae
  status: live
  owner: river
  ack: river 2026-10-20T18:15Z
  next: bei jedem Passbeginn einmal lesen
  orphan-after: none
  see-also: docs/concepts/house-rules.md docs/specs/vorlage-je-dateityp.md
-->
# Was ist was — Handover, Auftrag, Survey, Fadenübersicht, Register

**Die Frage dieses Blattes:** Wenn ich eine Sache suche — wo schaue ich zuerst,
und was erwarte ich dort?

## Die fünf Dinge

| Ding | Beantwortet | Wo | Größe | Wann lesen |
|---|---|---|---|---|
| **Handover** | Was ist offen, wer hat es, was ist der nächste Akt? | `handover/handover-<datum>-<linie>-folge<N>.md` | **≤ 1 KiB** | **bei jedem Passbeginn**, nur die eigene Linie |
| **Fadenübersicht** | Welche Fäden gibt es — Stand, Lücke, Zugang, Owner? | Tabelle **im Auftrag**, z. B. `auftrag/…-sonne-erde-faeden.md` | eine Tabelle | wenn du einen Faden wählst |
| **Auftrag** | Welches große Projekt — Ziel, Nicht-Ziel, Abnahme? | `auftrag/auftrag-<datum>-<thema>.md` | 1–3 Seiten | wenn du das Projekt beginnst |
| **Survey** | Was wurde gemessen — Zahlen, Quellen, Grenzen? | `docs/surveys/survey-<datum>-<thema>.md` | 1–5 Seiten | wenn du den Faden bearbeitest |
| **Register** | Welche Quelle ist verdrahtet — Lizenz, Compiler, Felder? | `phi/sources.φ` | eine Datei | bei Quellenarbeit |

## Die Kette

**Handover → Fadenübersicht → Auftrag/Survey → Register/Daten.**

Jede Ebene nennt nur die nächste, nicht alle. Deshalb: vom Handover aus findet
man alles; von einem Survey aus findet man den Faden; vom Register aus findet
man die Quelle.

## Was „Matrix" heißt

„Matrix" war ein Wort für die **Fadenübersicht**: eine Tabelle mit einer Zeile je
Faden — Stand, Lücke, nächster Schritt, Zugang, Owner. Sie ist **kein eigenes
Dokument**, sondern ein Abschnitt im Auftrag. Ab jetzt heißt sie
**Fadenübersicht**.

## Was kein Dokument ist

- Die **Träger-Liste** im Handover — ersetzt durch einen Zeiger auf die
  Fadenübersicht. Der Handover ist eine Übergabe, kein Register.
- Die **Lage-Zählung** (`bin/lage`) — ein Zähler, keine Übergabe.
- Ein **Survey ohne Owner/ack** — ein Orphan, kein Dokument.

## Die eine Regel

Jedes Dokument trägt im Kopf `owner` und `ack`. Der erste `ack` ist der Autor;
jede Übergabe ändert `owner` und fügt einen `ack` hinzu. **Dokumentiert ist
nicht weitergegeben** — erst die Quittung macht aus einer Datei eine Übergabe.
