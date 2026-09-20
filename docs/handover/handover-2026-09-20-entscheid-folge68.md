<!--
  title: Handover — Entscheid-Folge 68 (Stehender Pass fortgeschrieben; P13-Funding-Recherche dispatcht, Verdikt ausstehend; Operator-Queue um drei blockierte Quellen erweitert) (Stand 2026-09-20)
  session: Entscheid-Folge 68
  class: handover
  date: 2026-09-20
  sha256: 8690f3a2f4be0ad892677170a902d1ca27497efd90a59b4446d648db5c31a29d
  status: live
-->
# Handover — Entscheid-Folge 68 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 68)

- **HEAD** `276034b2` == `origin/main` — der folge67-Commit selbst (Handover +
  Werk committet/gepusht). Seither kein Fremd-Commit. Arbeitsbaum: **fremd**
  `phi/pipeline/ledger.φ` + `tools/register/src/bin/register_lookup.rs` (beide
  `M`, nicht angefasst).
- **Postfach** — `mail_digest`: 114 records, letzter Eingang `1789930255`
  (`info@pine64.org`: Ox64 zugesagt, bittet um Versanddaten/Telefon) —
  **unverändert, kein neuer Eingang** seit folge67. Antwort an Pine64 gesendet
  (`sent_ledger` `1789931195`). `post.md` **leer**.
- **CI** — Watchdog-Snapshot 21:22; `ci_manage list` ~22:1xZ: **neu**
  `free-model-agent-bench` `35534406541` **pending** @`276034b2` — der nach dem
  folge67-Commit dispatchte `funding-research`-Lauf (Workflow dispatch-only, kein
  Push-Trigger → der Dispatch ist erfolgt); `ci-check` `35531572974` in_progress /
  `35534406083` pending; `tools-build` `35534406080` in_progress; `te-gate`
  `35531196101` in_progress (>2 h — Watchdog entscheidet); `free-model-bench`
  `35527517605` in_progress, `35532890353` pending; `hyperscanning-te`
  `35533691654` **failure**; `ps1-cdn`/`allwise-cdn` in_progress. Kein Poll.
- **`register_lookup --open`** — 577 offene Zeilen, 10 `zustand` fällig, 16
  disposition; davon **3 `[entscheid]`** (`phi/blocked_sources.φ:21/60/65`) — in
  die Operator-Queue gefaltet (unten). `post.md` offen: 0.

## Operator-Queue (Stand folge68; einfache Sprache, je Frage mit Alter)

1. **PII-History-Rewrite** — Operator-Wort 2026-09-20: **Nein / ans Ende
   geschoben.** (Alter: seit 2026-09-16)
2. **Sponsoring-Konten** — Operator-Wort 2026-09-20: **Nein / erst nach dem
   Preprint.** (Alter: seit 2026-09-17)
3. **Mantis-Shrimp-Hardware-Bewerbungen** (Espressif / Crowd Supply /
   GSoC-OpenAstronomy-ESA SOCIS) — offen. (Alter: seit 2026-09-16)
4. **Pflichtenfreies Funding** — Eigenprize-Runde geschlossen (Deadline
   31.03.2026, keine nächste Runde datiert). **Frage:** den „Remind me"-Eintrag
   auf `https://eigen.build` setzen und den Entwurf
   `state/mail/eigenprize-application.md` finalisieren? Solitude: nächster Call
   ~Herbst 2027. (Alter: seit 2026-09-20)
5. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   Sicherheitsereignis (Mail 2026-09-20). **Frage:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit
   2026-09-20)
6. **SuperDARN** (`blocked account`) — HF-Radar-Ionosphären-Konvektion; Route
   über Globus + PI-Vereinbarung (`superdarn.ca/piagreement`). **Frage:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
7. **solar-system-open-data REST** (`blocked key`) — `api.le-systeme-solaire.net`
   HTTP 401 (Bearer-Token) 2026-09-20; Körperdaten ohne Konto nicht abrufbar.
   **Frage:** Konto/Token anlegen? (Alter: seit 2026-09-20)
8. **Amentum Developer** (`blocked account`) — geomagnetisch/aviation-radiation/
   gravity (trial); Registrierung HTTP 200, Zugang nur mit Konto. **Frage:**
   Registrierung `developer.amentum.io/register`? (Alter: seit 2026-09-20)
9. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf" (Ziel-Site aktiv
   → Cookie-Editor-Export → `state/cookies/<host>.json`). (Alter: seit 2026-09-16)
10. **Hardware-Sponsoring** — Pine64 **geschlossen** (Ox64 zugesagt); Framework
    **nicht passend** (`survey-funding-pflichtfrei.md` §E); Tuxedo Ticket#991311279
    **wartend**.

## Offen

- **P13 Free-Model-Funding-Recherche** (härtester undatierter Punkt) — Harness
  erweitert (`free_model_agent_bench.rs`, `--task funding-research`), Workflow mit
  `task`-Input + Chromium + `.answers`-Artefakt; **Dispatch erfolgt**
  (`35534406541` @`276034b2`, pending). `wartend` (Auslöser: Run-Abschluss).
  (Schritt: `ci_manage view 35534406541` **einmalig**, Artefakt
  `free-model-agent-bench.tsv.answers/<provider>__<id>.md` lesen, Funde vorlegen;
  keine Poll-Schleife.)
- **Eigenprize / Solitude** — `termin`; Eigenprize nächste Runde **ohne Datum**
  (Reminder auf `eigen.build`), Solitude ~Herbst 2027. Entwürfe existieren
  (`state/mail/eigenprize-application.md`, `state/mail/solitude-application.md`,
  gitignored). (Schritt: Reminder setzen + Entwurf finalisieren; per-Akt-Consent
  vor Absenden.)
- **P2/P3/P4 Bench-Läufe** — `wartend` (Auslöser: Run-Abschluss `35527517605` /
  `35532890353` / `35532892161`; `ci_manage view` einmalig, Artefakt
  `free-model-bench.tsv` lesen, Ranking eintragen).
- **ernte-Historie-Analyse** — `register_lookup --dropped` ist gebaut
  (Forschung-Folge 124), das Binär auf `tools-latest` noch stale → liest
  `pending`. (Schritt: nach dem nächsten tools-build `register_lookup --dropped`.)
- **Mantis-Shrimp-Bewerbungen**, **nvidia/zai Free-Status**, **Chrome-DevTools-MCP**
  (forschung), **Benchmark-Klassen B–D**, **F2-flare-Gate**, **09-16-Limbo** —
  `wartend` (Auslöser: Bedarf).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027), Rubin-Review (Umzug 2026-09-24). `blockiert` —
  TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block, SuperDARN-Globus,
  GitHub-PII/GC (#4761801), Sonden-Antworten, `register_lookup`-Binary + `ci-check`.

## Benchmark

- **Delegationen (Entscheid-Folge 68):** keine Sub-Agenten — der Stehende Pass und
  die Operator-Queue-Registratur sind eigene, lesende Arbeit (flash-first, kein
  pro/max nötig).
- **Klasse „pflichtenfreies Funding"** — geschlossen (research-max Sieger, folge66).
- **Neue Klasse (offen):** „105 freie Modelle als Funding-Rechercheure" — der
  `funding-research`-Lauf `35534406541`; Sieger nach Anteil verifizierter Wege.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge68.md` (neu)
- Move `handover-2026-09-20-entscheid-folge67.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (CI-Zeile + Postfach-Zeile fortgeschrieben)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der P13-Dispatch ist
erfolgt (`35534406541`); kein weiterer Dispatch, kein Poll — das Artefakt wird
einmalig gelesen, sobald der Lauf abgeschlossen ist.
