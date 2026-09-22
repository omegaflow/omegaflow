<!--
  title: Handover — Operator-Entscheidungen (2026-09-20)
  session: Operator-Entscheidungen (2026-09-20)
  class: handover
  date: 2026-09-20
  sha256: 13def2441a34a800bf361f29df91526dc658efc2583b7d396d8128e9a270e6e6
  status: live
-->
# Handover — Operator-Entscheidungen (2026-09-20)

## LLM-Nutzung

- `operator-gebunden` — **`pro` nur für die harten Atome**, Session beim
  Atom-Wechsel schließen (die 15-h-Session ist das Gegenbeispiel). (Schritt:
  Dispatch-Praxis der Linien-Session.)

## Operator-Entscheidungen (2026-09-20)

- **Pipeline force-gate: B** (skip + Review, kein Default). Ein Block ohne
  `force`-Direktiv bleibt `pending`/Review (`default_kernel_for("") = None`,
  `src/mathematikerin/force.rs:68`, `src/archivar/port.rs:18`); der ehrliche
  dritte Zustand ist `UNCERTAIN`→Review (`port.rs:1357/1509`). **Der Kernel-Riss**
  (thermal/diffusion/advective: `default_kernel_for` `force.rs:64-66` vs.
  `kernel_id_for_force` `force.rs:36-37`; live `sources.φ` folgt erfc) wird als
  **Widerspruch getragen und gemessen**, nie geglättet.
- **vC-Permeabilität — `termin` (wartet auf die Sensoren):** Operator führt den
  **versteckten sensor-getriebenen Lauf** aus
  (`OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`), danach
  `perm_target_probe --live <pfad>` (`tools/measure/src/bin/perm_target_probe.rs`).
  **Bedingung (Operator-Wort 2026-09-20): erst wenn alles fertig ist und Smartwatch
  + Mantis-Shrimp-Sensoren angeschlossen sind.** Ohne Sensoren ist der Lauf der
  descopte Ruhepunkt `(0,0,0,PERM_GROUND)`; der CI-Lauf ebenso. Lokale
  Funktionsläufe sind der Session strukturell verweigert.
- **Roadmap (Operator-Wort 2026-09-20):** alle Fäden auf 0 → Sensoren anschließen →
  Messlauf → **Mails + Publikation als letzter Schritt.**
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
