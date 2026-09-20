<!--
  title: Handover — PII-Audit & LLM-Budget (Stand 2026-09-20)
  session: PII-Audit & LLM-Budget
  class: handover
  date: 2026-09-20
  sha256: a5db7533288f3ed119bcf9f0db720b260f576c7b4f3e3ff6ac9e87b0109fb98f
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
- **Entscheidung (Operator, 2026-09-20): nicht-kommerziell — kein
  OSI-Relizenzieren.** PolyForm/CC BY-NC-SA bleiben. Folge: alle
  **OSI-Lizenz-gated** Förderer entfallen (Prototype Fund, Software Sprint, NGI,
  FLOSS/fund, Sovereign Tech Fund, NumFOCUS, CZI, Wikimedia). Es bleiben:
  Spenden, Hardware-Sponsoring, Research-Credits, Free-Tiers, Steuer, Altgerät.
  Rust-Foundation-Community-Grants sind **eingestellt** (nur noch Maintainers
  Fund, für Rust-Sprach-Maintainer).
- `operator-gebunden` — **Sponsoring erst mit Sichtbarkeit**: GitHub Sponsors /
  Open Collective aufsetzen, aber ohne Publikum ~0 €. (Schritt: Nadel III
  schließen → Preprint → dann Antrag/Sponsoring.)
- `operator-gebunden` — **Mantis-Shrimp-Hardware**: Espressif-Sponsorship +
  Crowd Supply; **GSoC/OpenAstronomy** + **ESA SOCIS** als Mentoring-Orga.
  (Schritt: Formular/Bewerbung, Operator-Wort.)

## Operator-Entscheidungen (2026-09-20)

- **Pipeline force-gate: B** (skip + Review, kein Default). Ein Block ohne
  `force`-Direktiv bleibt `pending`/Review (`default_kernel_for("") = None`,
  `src/mathematikerin/force.rs:68`, `src/archivar/port.rs:18`); der ehrliche
  dritte Zustand ist `UNCERTAIN`→Review (`port.rs:1357/1509`). **Der Kernel-Riss**
  (thermal/diffusion/advective: `default_kernel_for` `force.rs:64-66` vs.
  `kernel_id_for_force` `force.rs:36-37`; live `sources.φ` folgt erfc) wird als
  **Widerspruch getragen und gemessen**, nie geglättet.
- **vC-Permeabilität:** Operator führt den **versteckten sensor-getriebenen Lauf**
  aus (`OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`), danach
  `perm_target_probe --live <pfad>` (`tools/measure/src/bin/perm_target_probe.rs`).
  Der CI-Lauf ist laut Rat **descoped** (analytischer Ruhepunkt, null
  Verteilungsinfo). Lokale Funktionsläufe sind der Session strukturell verweigert.
- **register_lookup-Symlink:** erledigt (liegt in `~/.local/bin`, auf PATH).
- **opencode-Browser-Brücke:** verbunden.
- **Limadou CSES-02-PI-Freigabe:** Thread existiert bereits — Anfrage 23.08.2026,
  Sotgius Antwort: das Zugriffsverfahren wird für die **CSES-02-Aufnahme
  überarbeitet**, warten. Entwürfe `state/mail/sotgiu-reply.md` (danken+warten) und
  `limadou-pi-nachfassen.md`. Diese Session sendete irrtümlich ein **Duplikat**
  (`01a0beb8-…`) und danach eine **Korrektur** (`01a0bebe-…`). **Lehre:** der
  Planungs-Pass liest die **Post** (Inbox/Ledger) **vor** dem Handeln — hier
  versäumt; jeder Außen-Akt prüft zuerst den bestehenden Thread.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
