<!--
  title: Handover — Entscheid-Folge 72 (Wahrheits-Riegel für ausgehende Mails: Regel + Gate-Fixture + PINE64-Korrektur-Draft) (Stand 2026-09-21)
  session: Entscheid-Folge 72
  class: handover
  date: 2026-09-21
  sha256: 8c8dba0c30b6a9175b67704c8584edebe36b752b619f6f3a5403b0e695ea141e
  status: live
-->
# Handover — Entscheid-Folge 72 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag (`wartend` |
`operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 72)

- **HEAD** `4a0c7803` (bau folge116) == `origin/main`. Arbeitsbaum zu Session-Beginn
  mit fremder uncommitteter Arbeit (Ernte/Bau an `phi/*.φ`, `tools/harvest`) —
  nicht angefasst.
- **Postfach** — `mail_ledger.φ`: **neuer** Eingang `1789970277`
  (`pm_bounces@kbounces.frame.work`: Bounce der Framework-Sponsoring-Mail,
  informativ); davor `1789930255` (`info@pine64.org`). `sent_ledger` `1789931195`
  (Antwort an Pine64, gesendet). `external-state.md`-Zeile fortgeschrieben.
- **CI** — `ci_manage list` 2026-09-21 ~08:3x: `ci-check` `35568374735` pending;
  `te-gate` `35567711055` in_progress; `hyperscanning-te` `35567708611` failure;
  `free-model-bench` `35567266519`/`free-model-agent-bench` `35567268692`
  in_progress; `demeter-cdn` `35567568429` queued; `ned-cdn` `35568810102` success.
  Kein Poll. `external-state.md`-Zeile fortgeschrieben.
- **`register_lookup --open`** — 3 `[entscheid]` (`phi/blocked_sources.φ:21/60/65`,
  in der Queue); `post.md`-Zeile `An entscheid:` (fmt-Rot) gefaltet + gelöscht.
- **`git_safety --snapshot`** — `refs/safety/1789971132`/`1789972126`.

## Offen (Tafel — parallel abarbeitbar, keine Rangfolge)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| Bewerbungs-/Programm-Anforderungen | `eigen` | `eigen` | `research-max` misst Espressif-Dev-Programm, Crowd-Supply-Bedingungen, GSoC-/SOCIS-Eligibility (Espressif/GSoC/SOCIS = Projekt-Vorschlag, Crowd Supply = fertiges Produkt) |
| Free-Model-Bench (P13 + P2–P4) | `wartend` | `eigen` | Artefakte **einmalig** lesen, sobald die Läufe durch sind — `ci_manage view 35567266519`/`35567268692`; Ranking oder pending |
| smail-Wahrheits-Riegel | `wartend` | `linie:bau` | bau baut `tools/service/src/bin/smail.rs` (QUELLEN-Block-Parse, `--send`-Refusal exit 2, `--dry-run`-Tabelle, kein Bypass-Flag); bis dahin bindet die Session-Pflicht |
| PINE64-Dokumentationspflicht | `blockiert` | `linie:bau` | Post an bau: Mantis-Shrimp minimal bauen (Spec + BOM liegen), dann Ox64 dokumentieren |
| PINE64-Korrektur | `operator-gebunden` | `operator` | Operator-Wort auf den Draft `state/mail/pine64-correction.body.txt`; dann `smail --to info@pine64.org … --send` (vorher QUELLEN-Block entfernen, bis bau's Riegel steht) |
| Mantis-Shrimp-Bewerbungen | `operator-gebunden` | `operator` | nach dem Korrektur-Wort neu vorlegen, mit gemessenem Sachverhalt |

## Operator-Queue (Stand folge72; einfache Sprache, je Frage mit Alter)

1. **PINE64-Korrektur** — Draft liegt (`state/mail/pine64-correction.body.txt`).
   **Frage:** soll die Korrektur (die frühere „currently built"-Zeile war falsch)
   an PINE64 gesendet werden? (Alter: seit 2026-09-21)
2. **Mantis-Shrimp-Bewerbungen** (Espressif / Crowd Supply / GSoC / SOCIS) —
   **Frage:** Programme messen (research-max), descopen, oder direkt bauen?
   (Alter: seit 2026-09-16)
3. **Eigenprize / Solitude** — Eigenprize-Runde geschlossen (Deadline 31.03.2026,
   keine nächste Runde datiert). **Frage:** „Remind me" auf `https://eigen.build`
   setzen und `state/mail/eigenprize-application.md` finalisieren? Solitude
   ~Herbst 2027. (Alter: seit 2026-09-20)
4. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   Sicherheitsereignis. **Frage:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit 2026-09-20)
5. **SuperDARN** (`blocked account`) — HF-Radar-Ionosphären-Konvektion; Route
   über Globus + PI-Vereinbarung. **Frage:** Konto + PI-Vereinbarung eingehen?
   (Alter: seit 2026-09-16)
6. **solar-system-open-data REST** (`blocked key`) — HTTP 401 (Bearer-Token).
   **Frage:** Konto/Token anlegen? (Alter: seit 2026-09-20)
7. **Amentum Developer** (`blocked account`) — geomagnetisch/aviation-radiation/
   gravity (trial). **Frage:** Registrierung `developer.amentum.io/register`?
   (Alter: seit 2026-09-20)
8. **Split-Routing-Verifikation** — `./bin/proton-exit.sh ca` + direct↔tunnel-
   Nachmessung der 8 `000`-Hosts. **Frage:** Operator-Wort/Route (sudo+Netz)?
   (Alter: seit Ernte folge12–17)
9. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf". (Alter: seit 2026-09-16)
10. **Hardware-Sponsoring** — Pine64 **geschlossen** (Ox64 zugesagt, Antwort
    gesendet); Framework **nicht passend**; Tuxedo Ticket#991311279 **wartend**.
11. **Strukturierte Feld-Grammatik** (bau, aus `post.md` gefaltet) — am 2026-09-18
    als Kanon-Akt vertagt. **Frage:** Spec-Entwurf (Kanon-Akt, mit Rat) oder
    descoped mit Messung? (Alter: seit 2026-09-18)

## Benchmark

- **Delegationen (Entscheid-Folge 72):** 2 × `council` (pro/max) für die
  Integritäts-/Durchsetzungs-Architektur — beide lieferten das Verdikt in einem
  Zug (Korrektur geboten; Riegel = Regel + smail-Riegel + Fixture; Atom-Grenze).
  Kein flash-Doppel-Lauf — der Fall ist ein hartes Urteil, nicht Routine.

## Geteilter Baum — eigener Pfad-Satz

- `AGENTS.md` (Truth-gate-Regel; Parallel-Regel — keine Rangfolge)
- `src/gate/commit_gate.rs` (`state_claim` + `serial_priority` Prüfung + Tests)
- `src/gate/commit_gate_vocab.json` (`state_claim`- + `serial_priority`-Liste)
- `docs/handover/_template.md` (Parallel-Regel, Sektion ohne Rang)
- `.opencode/command/{entscheid,bau,ernte,forschung}.md` (Phase-1-Auswahl → Parallel-Tafel; `register_lookup --live` → `--open` korrigiert)
- `docs/handover/post.md` (`An entscheid:` gefaltet + gelöscht, `An bau:` + `An forschung:` neu)
- **von bau folge117 committet (`a70b1ee2`, nicht mehr eigener Pfad):**
  `tools/measure/src/bin/free_model_bench.rs`,
  `tools/measure/src/bin/free_model_agent_bench.rs` (rustfmt-Hunks)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile)
- `docs/handover/handover-2026-09-21-entscheid-folge72.md` (neu)
- Move `handover-2026-09-21-entscheid-folge71.md` → `archiv/` (eigene Linie, atomar)
- `state/mail/pine64-correction.body.txt` (gitignored, Draft)
- **nicht** angefasst: fremde `phi/*.φ`, `tools/harvest`-Hunks, fremde post-Zeilen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push wird
`ci-check.yml` dispatcht (frische fmt-Messung der zwei Bench-Dateien); kein Poll —
die Run-Id steht im Handover, das Ergebnis wird einmalig gelesen.
