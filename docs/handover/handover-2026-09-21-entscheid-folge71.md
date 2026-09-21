<!--
  title: Handover — Entscheid-Folge 71 (Stehender Pass fortgeschrieben; Free-Model-Bench all-null gemessen — zwei Harness-Defekte geheilt, Klasse bleibt pending) (Stand 2026-09-21)
  session: Entscheid-Folge 71
  class: handover
  date: 2026-09-21
  sha256: cefdb022caeeae12024085f4621ac6e9ed742f7b3011f6a954545d58d4dcbf7e
  status: live
-->
# Handover — Entscheid-Folge 71 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 71)

- **HEAD** `0fd1c5a9` (ernte folge125, fremd) == `origin/main` (folge70 notierte
  `ff433f9d`). Arbeitsbaum zu Session-Beginn **clean**.
- **Postfach** — `mail_ledger.φ`: letzter Eingang `1789930255` (`info@pine64.org`:
  Ox64 zugesagt, Versanddaten/Telefon erbeten) — **unverändert**; Antwort
  operator-gebunden (Dritt-Write). `sent_ledger` `1789931195`. Eintrag
  `external-state.md` (Postfach) fortgeschrieben.
- **CI** — Watchdog-Snapshot + `ci_manage list` 2026-09-21: die drei Bench-Läufe
  **completed success** — `35534406541` @`276034b2` (P13 funding-research),
  `35532890353` @`7b4753ef` (free-model-bench T7), `35532892161` @`7b4753ef`
  (agent-bench t7). Rot auf fremden Linien: `ci-check` `35537130867`/`35534657914`/
  `35531572974` failure, `hyperscanning-te` `35537110267`/`35533691654` failure,
  `rpw-cdn` `35536016815`/`35536005969` failure, `health-check` `35535074295`
  failure; `35543181033` health-check in_progress. Kein Poll.
- **`register_lookup --open`** — 3 `[entscheid]` (`phi/blocked_sources.φ:21/60/65`,
  in der Queue); `post.md`-Zeile `An entscheid: Ksg off-path` gefaltet + gelöscht.
- **`git_safety --snapshot`** — „working tree equals HEAD — nothing to record".

## Operator-Queue (Stand folge71; einfache Sprache, je Frage mit Alter)

1. **Pine64 / Ox64** — Ox64 zugesagt, Pine64 bittet um **Versanddaten +
   Telefonnummer**. **Frage:** soll die Antwort (Adresse/Telefon) gesendet werden?
   (Alter: seit 2026-09-20)
2. **Mantis-Shrimp-Hardware-Bewerbungen** (Espressif / Crowd Supply /
   GSoC-OpenAstronomy-ESA SOCIS) — offen. (Alter: seit 2026-09-16)
3. **Pflichtenfreies Funding** — Eigenprize-Runde geschlossen (Deadline
   31.03.2026, keine nächste Runde datiert). **Frage:** „Remind me" auf
   `https://eigen.build` setzen und den Entwurf
   `state/mail/eigenprize-application.md` finalisieren? Solitude ~Herbst 2027.
   (Alter: seit 2026-09-20)
4. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   Sicherheitsereignis (Mail 2026-09-20). **Frage:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit
   2026-09-20)
5. **SuperDARN** (`blocked account`) — HF-Radar-Ionosphären-Konvektion; Route
   über Globus + PI-Vereinbarung (`superdarn.ca/piagreement`). **Frage:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
6. **solar-system-open-data REST** (`blocked key`) — `api.le-systeme-solaire.net`
   HTTP 401 (Bearer-Token); Körperdaten ohne Konto nicht abrufbar. **Frage:**
   Konto/Token anlegen? (Alter: seit 2026-09-20)
7. **Amentum Developer** (`blocked account`) — geomagnetisch/aviation-radiation/
   gravity (trial); Registrierung HTTP 200, Zugang nur mit Konto. **Frage:**
   Registrierung `developer.amentum.io/register`? (Alter: seit 2026-09-20)
8. **Split-Routing-Verifikation** — `./bin/proton-exit.sh ca` +
   direct↔tunnel-Nachmessung der 8 `000`-Hosts. **Frage:** Operator-Wort/Route
   (sudo+Netz, nicht in der Ernte-Session)? (Alter: seit Ernte folge12–17)
9. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf" (Ziel-Site aktiv
   → Cookie-Editor-Export → `state/cookies/<host>.json`). (Alter: seit 2026-09-16)
10. **Hardware-Sponsoring** — Pine64 **geschlossen** (Ox64 zugesagt, Antwort
    offen → Nr. 1); Framework **nicht passend**
    (`survey-funding-pflichtfrei.md` §E); Tuxedo Ticket#991311279 **wartend**.
11. **Strukturierte Feld-Grammatik** (bau, aus `post.md` gefaltet) — Lage: am
    2026-09-18 als Kanon-Akt vertagt, seither in keiner Übergabe; kein Spec, kein
    Code, keine Messung im Baum. **Frage:** soll bau einen Spec-Entwurf für eine
    strukturierte Grammatik der φ-Feld-Direktiven vorlegen (Kanon-Akt, mit Rat) —
    oder den Strang „nie gebaut, nicht gebraucht" schließen (descoped mit
    Messung)? (Alter: seit 2026-09-18)
12. **Ksg off-path** (Forschung-Linie, aus `post.md` gefaltet) — der
    Familien-Screen ruft nur `topological_te_estimate`, nie Ksg. **Frage:** Ksg
    verdrahten oder mit gemessenem „nicht auf dem Pfad" descopen? (Alter: seit
    2026-09-20)

## Offen

- **Free-Model-Bench (P13 + P2–P4)** (härtester undatierter Punkt) — Auslöser
  gefeuert (drei Läufe success), Artefakte **einmalig** gelesen: **all-null**.
  `free-model-bench.tsv` (T7): 0 Pass, alle Zeilen `http_400`/`http_404`/
  `http_401`/`wrong_answer`; `free-model-agent-bench.tsv`: alle `no_output`/
  `empty`, `tool_calls=pending`, `answer_chars 0`. Ursache gemessen: (a)
  `free_model_bench.rs` `body_with()` sendet nie ein `model`-Feld → jeder strikte
  Endpoint antwortet 400; (b) der Agent-Bench ruft `opencode run --agent general`,
  `general` ist aber Subagent → `exit=1` (CI-Log `35534406541`: `! agent "general"
  is a subagent … Falling back to default agent`), 0 Antworten. **Beide geheilt**
  (model-Feld ergänzt; `--agent plan` = read-only Primary). `cargo check -p
  omegaflow-measure` clean. Kein Ranking möglich — Klasse bleibt **pending**.
  (Schritt: nach dem Push `gh workflow run free-model-bench.yml -f task=T7` +
  `gh workflow run free-model-agent-bench.yml -f task=funding-research`; Run-Ids
  registrieren; Artefakte einmalig lesen.)
- **Eigenprize / Solitude** — `termin`; Eigenprize nächste Runde **ohne Datum**
  (Reminder auf `eigen.build`), Solitude ~Herbst 2027. Entwürfe existieren
  (`state/mail/eigenprize-application.md`, `state/mail/solitude-application.md`,
  gitignored). (Schritt: Reminder setzen + Entwurf finalisieren; per-Akt-Consent
  vor Absenden.)
- **Mantis-Shrimp-Bewerbungen**, **nvidia/zai Free-Status**, **Chrome-DevTools-MCP**
  (forschung), **Benchmark-Klassen B–D**, **F2-flare-Gate**, **09-16-Limbo** —
  `wartend` (Auslöser: Bedarf).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027), Rubin-Review (Umzug 2026-09-24). `blockiert` —
  TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block, SuperDARN-Globus,
  GitHub-PII/GC (#4761801), Sonden-Antworten, `ci-check`.

## Benchmark

- **Delegationen (Entscheid-Folge 71):** 1 × `general` (flash) für die
  Bench-Artefakt-Diagnose — lieferte die Ursache in einem Zug (`body_with` ohne
  `model`; `--agent general` = Subagent); keine Eskalation, kein pro/max. Klasse
  „Routine-Bench-Artefakt-Diagnose" flash-first, kein Doppel-Lauf.
- **Klasse „pflichtenfreies Funding"** — geschlossen (research-max Sieger, folge66).
- **Klasse „105 freie Modelle als Funding-Rechercheure"** — offen; die drei
  abgeschlossenen Läufe sind all-null (Harness-Defekt, kein Modell-Verdikt); nach
  der Heilung neu zu messen.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge71.md` (neu)
- Move `handover-2026-09-20-entscheid-folge70.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (An-entscheid-Zeile gefaltet + gelöscht)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile fortgeschrieben — eigene Zeilen)
- `tools/measure/src/bin/free_model_bench.rs` (`model`-Feld ergänzt)
- `tools/measure/src/bin/free_model_agent_bench.rs` (`--agent general` → `--agent plan`)
- **nicht** angefasst: fremde Zeilen/Hunks in `external-state.md`, die `An bau`-Zeile
  in `post.md` (bau-Linie).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push werden
die zwei geheilter Bench-Workflows dispatcht; kein Poll — die Run-Ids stehen im
Handover, das Artefakt wird einmalig gelesen, sobald der Lauf abgeschlossen ist.
