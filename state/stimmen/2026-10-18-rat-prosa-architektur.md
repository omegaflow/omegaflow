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
