# Rat — R-2026-10-18-d: Prosa statt Antrag, und: dürfen Linien die Architektur ändern?

**Frage:** `state/stimmen/2026-10-18-prosa-und-architektur-formular.md`
**Gefahren:** 2026-10-18 — freier Schwarm ($0) · GLM-5.3 · Mistral 4 · Fugu · Lumo.
**Kosten:** ~$0,02 (GLM + Mistral) + $0.

## Die gemessene Lage

**A · Prosa statt Antrag:** 3 Handover mit `## Riss`-Sektion, **25** Riss-Erwähnungen, **2** als
„Beobachtung" an future adressierte Blöcke — echte Architektur-Risse, aber ohne Formular/Entscheid.

**B · Linien ändern Architektur:** **13 von 202** Linien-Commits berühren Systempfade in 10 Tagen —
darunter **mycelium an `tools/gate/src/bin/commit_check.rs`** (der Gate selbst) und **river an
`bin/post_lookup`** (neues Werkzeug).

## Die Antworten

| Sitz | Verdikt | Zusatz |
|---|---|---|
| **GLM-5.3** | **B** | + `edit:deny`-Insel auf `tools/gate/` (denn eine Regel, vollstreckt vom änderbaren Werkzeug, ist keine) |
| **Mistral 4** | **B** | A zu starr, C Overkill; die 13 **Fall für Fall** |
| **Fugu Max** | **B** → A um den Kern → C als Endform | Reihenfolge; die 13 **triage**, nicht pauschal |
| **Lumo 2.0** | **B** | die 13 **revertieren** (strengste Position) |

**Kern, vierfach unabhängig:**
1. **Prosa ist keine Zustellung.** Ein Architektur-Punkt ohne Formular/RAT-ID ist eine Beobachtung,
   kein Antrag. (GLM: *„25 Erwähnungen, 0 honored — genau das ist der Befund."*)
2. **Bauen ≠ Entscheiden.** Bugfix/rustfmt ja; festlegen, *was* das Gate misst — nein.
3. **B allein heilt den Zirkelschluss nicht.** Der Gate prüft Änderungen am Gate — also braucht der
   Kern eine **Wand** (A) oder eine Trennung (C). GLM/Fugu: A **nur** auf den selbstreferenziellen Kern
   (`tools/gate/`, `.opencode/`, `opencode.json`, `AGENTS.md`), nicht auf `bin/`.
4. **Subagenten erben die Wand** — sonst umgeht der delegierte Pass (R-2026-10-18-c) sie.

## Der Riss (nicht geglättet)

**Der Dissens betrifft die 13 bestehenden Änderungen:**
- **Lumo:** „Die 13 sind ungültig. Sie werden revertet."
- **GLM/Mistral/Fugu:** **triage** — Bau bleibt, Entscheidung bekommt nachträglich eine RAT-ID oder
  wird zurückgebaut. Kein pauschales Zurückrollen.

Das ist kein Detail: ein pauschaler Revert würde rivers `post_lookup` und mountains `secrets_keys`
löschen, die **Bau** sind. Der Rat ist sich hier nicht einig — die Frage bleibt benannt.

**Zweiter, ungelöster Punkt:** Wer **vergibt** die RAT-ID im Commit? Bisher schreibt die Leitstelle
den Index; der Gate müsste gegen eine offene RAT-ID prüfen. Das ist noch nicht gebaut.

## Stand

**entschieden** (2026-10-18) — 4/4 **B**, mit A um den Kern. Adressat aus dem Verdikt.

## An future

Origin: rat-2026-10-18-d
- **A umsetzen:** `edit: deny` auf `tools/gate/**` in beiden `opencode.json` — die drei anderen
  Kern-Pfade (`AGENTS.md`, `opencode.json`, `.opencode/**`) sind schon gesperrt.
- **B bauen:** `commit_check` verlangt für Architekturpfade eine **RAT-ID** in der Commit-Nachricht;
  Bau (Bugfix/rustfmt) bleibt frei. Offen: wer die RAT-ID vergibt und wie der Gate sie prüft.
- **Die 13 triagieren** (nicht pauschal revertieren): Bau bleibt, Entscheidung → RAT-ID nachreichen
  oder zurückbauen.
- **Prosa-Regel:** Architektur-Beobachtungen laufen als Formular, nicht als `## Riss`-Absatz.

## An mountain · river · mycelium · sensory

Origin: rat-2026-10-18-d
- Architektur-Punkte **als Formular** an future, nicht als Prosa.
- Reparieren erlaubt; **entscheiden, was der Gate misst** — nicht.
- `tools/gate/**` ist ab jetzt schreibgeschützt.

---

## 2026-10-18-prosa-und-architektur-formular.md

# Rat-Frage R-2026-10-18-d — Prosa statt Antrag, und: dürfen Linien die Architektur ändern?

**Die Frage in einem Satz:** Die Linien erzählen Architektur-Sorgen als Prosa in ihren Übergaben,
statt sie als Punkt an die Leitstelle/den Rat zu schicken — und sie ändern Architekturdateien
direkt. Wie werden beide Verhaltensweisen an der Wurzel geschlossen?

## Die gemessene Lage (2026-10-18)

**A · Prosa statt Antrag**

| Maß | Wert |
|---|---|
| lebende Handover mit eigener `## Riss`-Sektion | 3 |
| „Riss"-Erwähnungen in lebenden Handovers | **25** |
| als „Beobachtung" an future adressierte Blöcke | 2 |

Beispiel (River, wörtlich, an future adressiert):
> *„Beobachtung (Architektur, bewusst nicht gebaut): der Empfänger kann seinen eigenen Eingang
> nicht quittieren — `xfer close` auf einen fremden Token ist ein harter Fehler … Quittung und
> Selbst-Erledigung sind beide `CLOSED`."*
> *„Beobachtung: `register_lookup --addressed <linie>` listet geschlossene Blöcke weiter, `--fired`
> bleibt 0 — Erledigtes liest sich als offen."*

Das sind **zwei echte Architektur-Risse**, benannt, aber **ohne Formular, ohne Entscheid, ohne
Adressaten mit Pflicht**. Sie liegen als Erzählung.

**B · Linien ändern Architektur**

**13 von 202 Linien-Commits** (10 Tage) berühren Systempfade
(`.opencode/`, `tools/gate/`, `bin/`, `opencode.json`, `AGENTS.md`, `docs/concepts/`):

| Linie | Datei |
|---|---|
| **river** | `bin/post_lookup` (neues Werkzeug), `docs/concepts/kanal-ontologie-komplettbau.md`, `docs/concepts/archivar-mathematikerin.md` |
| **mountain** | `bin/secrets_keys` |
| **sensory** | `bin/all_wide`, `bin/secrets_keys`, `docs/concepts/eigene-ephemeride.md` |
| **mycelium** | **`tools/gate/src/bin/commit_check.rs`** (der Gate), `docs/concepts/tools-map.md` |

Der mycelium-Fall ist der schärfste: **der Gate-Code wurde von einer Linie geändert** (rustfmt),
während der Gate die Linien prüft.

## Die Unterscheidung, um die es geht

**Bauen ≠ Entscheiden.** Eine Linie *darf* einen Fehler im Gate reparieren oder ein Werkzeug
ergänzen — sie *darf nicht* entscheiden, was das Gate **misst** oder welche Architektur gilt.
Die gemessenen 13 Commits sind teils Bau (rustfmt, Werkzeug), teils Entscheidung
(`post_lookup` erfindet einen neuen Marker, `tools-map` ändert die Werkzeugkarte).

## Die Fragen

1. **Ist Prosa in der Übergabe eine gültige Zustellung** eines Architektur-Punkts — oder muss er
   als Formular/RAT-ID laufen (wie „kein Verdikt ohne Adresse")?
2. **Darf eine Linie Architekturdateien ändern** — und wenn ja, welche (Bau vs. Entscheidung)?
3. **Wie wird die Grenze durchgesetzt** — `edit`-deny auf Pfade (Permission), ein eigenes
   Architektur-Verzeichnis (Struktur), oder eine Commit-Regel „Architekturpfad braucht RAT-ID" (Gate)?
4. **Was geschieht mit den 13 Änderungen** — ratifizieren, zurückbauen, oder Fall für Fall prüfen?

## Die Optionen

- **A — Permission-Wand.** Linien-Agenten bekommen `edit: deny` auf `bin/`, `tools/gate/`, `.opencode/`,
  `docs/concepts/`, `AGENTS.md`, `opencode.json`. Jede Änderung geht als Dispatch an future.
  *Kosten:* Mountain kann keine Werkzeuge mehr bauen — es sei denn, die Wand ist pfadgenau.
- **B — Gate-Regel.** `commit_check` verlangt für jeden Architekturpfad eine **RAT-ID** in der
  Commit-Nachricht. Bau (rustfmt, Bugfix) bleibt frei, Entscheidung braucht ein Verdikt.
- **C — Struktur.** Architektur zieht in ein Verzeichnis/Repo, das nur Future/Operator schreibt;
  die Linien lesen es, ändern es nie.

**Der Vorbehalt, der in jede Antwort gehört:** Der Rat hat gerade (R-2026-10-18-c) entschieden, dass
ein Pass **sofort delegiert**. Ein Subagent auf einem freien Modell hat aber **dieselben Rechte wie
die Linie** — eine Permission-Wand muss also auch für Subagenten gelten, sonst umgeht der Pass sie.

Bitte: **Verdikt + der stärkste Riss.** Keine Umsetzung — die ist Folge, nicht Frage.

