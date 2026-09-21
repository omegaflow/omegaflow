<!--
  title: Handover — Bau-Folge 121 (Stand 2026-09-21)
  session: Bau-Folge 121
  class: handover
  date: 2026-09-21
  sha256: 3543036ee71c9b8e822b52df8d2f220616f3af08b3e1dfe518521bcc5d6f619b
  status: live
-->
# Handover — Bau-Folge 121 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `a476ccfb` == `origin/main`. Der Baum trägt fremde uncommittete Arbeit
  (`docs/surveys/survey-funding-erkundung.md`) — nicht angefasst.
- **Postfach** — kein neuer Ledger-Eingang; `1789973288` (Tuxedo-decline) zitiert
  (`external-state.md:20`), nicht fällig.
- **CI** — Watchdog 10:11:01 + `ci_manage view`: `radnet-cdn 35573513812`
  in_progress; `glm-l2-cdn 35576757352` **failure** (diagnostiziert, siehe Atom);
  `ps1-cdn 35569486280`, `ci-check 35574757717` in_progress. Kein Poll.
- **Safety-Net** `refs/safety/1789978584`.

## Messung dieses Atoms (kein offener Punkt)

- **`archive_search --verdict` repariert** (post.md-Zeile eingelöst): die Wurzel
  gemessen — `tools/utils/src/bin/archive_search/net.rs` `curl_fetch` setzt
  `Fetch.complete = out.status.success()`, aber `--verdict` las den Flag nie. Ein
  transient gescheiterter Transfer (curl exit 28) liefert `%{http_code}` `000`
  und wurde als `HTTP 0 — absent` gedruckt. Die Post-Diagnose (Redirect-/HEAD-
  Handling auf Trailing-Slash) war ungemessen — der Trailing-Slash war nie die
  Ursache; `curl_args` sendet `-sL -g --max-time 30 <url>` unverändert. Fix:
  `curl_args` extrahiert, `VERDICT_ATTEMPTS = 3` + `is_transient`/`retry_transient`/
  `verdict_probe`; ein unvollständiger Transfer wird wiederholt und, bleibt er es,
  als `pending — no response` gemeldet — nie ein fabriziertes `HTTP 0/308 absent`.
  3 Gate-Tests. `cargo check -p omegaflow-utils --all-targets` 0 Fehler/0
  Warnungen. Live: direkt `000`/exit 28 flaky, Proton-Exit 200/5740 — Route
  gefunden.
- **radnet.bin manifestiert:** `archive_search --sniff
  …/data.epa.gov/radnet.bin` → HTTP 200, 17408568 B, sha256
  `128ac4fd695f63a2e5fd156e1b416408880577555acf9e676d0e76b242a31c34`; im
  `phi/sources.φ` radnet-Block registriert.
- **glm-l2-cdn-Fehllauf diagnostiziert:** `35576757352` failure =
  `error[E0433] cannot find module hfrnet_rtv` an `src/archivar/extract.rs:350` —
  der rote Parent `0475d67f` (bau folge120 committete die hfrnet_rtv-Referenzen
  ohne `mod.rs`/`hfrnet_rtv.rs`); der Fix landete mit ernte folge129 `2ae4978a`.
  Am grünen HEAD neu dispatcht: Lauf `35579950220`.

## Offen (aufgeschlüsselt)

### 1. glm_l2.bin CDN-Manifestation
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Compiler + Workflow stehen; am grünen HEAD neu dispatcht als Lauf
  `35579950220`; das Asset ist noch nicht manifestiert (`--sniff` 404).
- **Blockade:** Run-Abschluss (CI).
- **Braucht:** `ci_manage view 35579950220` / Watchdog-Snapshot; bei success
  `archive_search --sniff …/download/noaa-goes18/glm_l2.bin`, sha256 in
  `phi/sources.φ`.

### 2. PINE64 / Mantis-Shrimp (Ox64-Dokumentation)
- **Status:** `blockiert` | **Bindung:** `linie:entscheid`
- **Lage:** Ox64 von Pine64 zugesagt (Hardware beidseitig geschlossen); die
  Presence-Hardware „Mantis-Shrimp" ist ungebaut; Spec
  `docs/specs/mantis-shrimp-bom.md` + BOM liegen. `entscheid-folge76:97` trägt die
  Doku-Pflicht als `blockiert | linie:bau`, den Förderweg als `operator-gebunden`.
- **Blockade:** die Hardware existiert nicht; kein Förderweg (entscheid-folge73/76).
- **Braucht:** Operator-Wort — Prototyp bauen oder descopen (via entscheid-Linie);
  bis dahin kein Schritt.

## Benchmark

- **Bau-Folge 121:** `grind-flash` ×1 (archive_search `--verdict`-Fix, ~2 min).
  Flash-first; keine Eskalation nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/utils/src/bin/archive_search/net.rs`,
  `phi/sources.φ` (radnet-sha256), `docs/handover/post.md`, neues
  `docs/handover/handover-2026-09-21-bau-folge121.md`, Move
  `handover-2026-09-21-bau-folge120.md` → `archiv/`.
- **Fremd (nicht angefasst):** `docs/surveys/survey-funding-erkundung.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
