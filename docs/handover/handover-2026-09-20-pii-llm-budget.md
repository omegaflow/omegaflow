<!--
  title: Handover — PII-Audit & LLM-Budget (Stand 2026-09-20)
  session: PII-Audit & LLM-Budget
  class: handover
  date: 2026-09-20
  sha256: 9216ce0773669cf6e245440877bde3f12f1e0836f7c9244312e2d4a391d8493d
  status: live
-->
# Handover — PII-Audit & LLM-Budget (2026-09-20)

## PII-Historie (härtester undatierter Punkt)

- `operator-gebunden` — **History-Rewrite**: die private Operator-Adresse steht
  in 1.472 Commits (Autor/Committer) und in älteren Blobs; HEAD ist redigiert
  (`99c6500d`, fünf Dateien → `<operator-adresse>`), die Historie nicht. Kein
  anderes PII im Baum (kein IBAN, keine Adresse, keine Bank-/Gesundheitsdaten).
  (Schritt: `docs/auftrag/auftrag-pii-history-rewrite.md` — destruktiv, braucht
  Operator-Wort + Force-Push.)

## LLM-Nutzung & Budget

- **Nutzungsmuster (gemessen)**: DeepSeek 28,97 Mrd. Tokens / 30 T, Ø 153k
  Tokens/Request; `v4-pro` = 48 % der Tokens; opencode.db (gepruned): cache_read
  ≈ 34× input, eine Session 15,4 h = 88 % des cache_read. Treiber: Session-Länge ×
  Kontext, nicht der Token-Preis.
- **Gebremst**: Balance-Alert $20 aktiv (2026-09-20); Cap ≤ $50/Woche + off-peak
  als Zeile in `docs/zustand/external-state.md`. Konto konsolidiert auf DeepSeek.
- `operator-gebunden` — **`pro` nur für die harten Atome**, Session beim
  Atom-Wechsel schließen (die 15-h-Session ist das Gegenbeispiel). (Schritt:
  Dispatch-Praxis der Linien-Session.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
