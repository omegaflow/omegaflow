<!--
  title: Handover — PII-Audit & LLM-Budget (Stand 2026-09-20)
  session: PII-Audit & LLM-Budget
  class: handover
  date: 2026-09-20
  sha256: 7fb28f8bd9300b1c52ba078def126f5173a3480b9b7f0cf6e356f2930e1a652e
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

## Funding (pflichtfreie Wege)

- **Angefragt (2026-09-20, Resend):** Research-Credits an DeepSeek
  `api-service@deepseek.com` (`01a0be5b-a410…`), Moonshot `api-service@moonshot.ai`
  (`…a506…`), Z.ai `user_feedback@z.ai` (`…a5fe…`); Hardware an Framework
  `support@frame.work` (`…a6e6…`), Slimbook `info@slimbook.com` (`01a0be7a-…a82`),
  PINE64 `sales@pine64.org` + `info@pine64eu.com` (`01a0be7b-8250…`/`…8366…`).
  **Tuxedo-Formular** vom Operator abgeschickt (2026-09-20). Entwürfe lokal
  `state/mail/`. Geprüft ohne Einzel-Sponsoring: Framework (org), System76
  (Marketing), NovaCustom/Star Labs (kein Programm), KDE/GNOME (nur eigene
  Contributor), PINE64 (ARM/RISC-V — als Mantis-Shrimp-Plattform, Spec
  `docs/specs/mantis-shrimp-bom.md`).
- **Erkundung:** `docs/surveys/survey-funding-erkundung.md`. Realismus-Lesart:
  große Fellowships (Shuttleworth/Mozilla) brauchen **öffentliche Wirkung** —
  fehlt noch; die konkretesten DE-Türen (Prototype Fund, Fellow-Programm Freies
  Wissen) tragen **Release-/Publikationspflicht**. Pflichtfrei bleiben:
  Free-Tier-APIs (Gemini/Groq/Cerebras/NVIDIA/Cloudflare/Mistral/Cohere),
  GitHub Sponsors/Open Collective/Patreon, Hardware-Sponsoring, Steuer, Altgerät.
- `operator-gebunden` — **Sponsoring erst mit Sichtbarkeit**: GitHub Sponsors /
  Open Collective aufsetzen, aber ohne Publikum ~0 €. (Schritt: Nadel III
  schließen → Preprint → dann Antrag/Sponsoring.)
- `operator-gebunden` — Framework-/Tuxedo-**Sponsoring-Formular** ausfüllen
  (Framework-Formular auf `frame.work/contact-us`; Tuxedo nutzt Formular).
  (Schritt: Browser-Formular, Operator-Wort.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
