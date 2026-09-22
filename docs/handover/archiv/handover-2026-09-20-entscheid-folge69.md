<!--
  title: Handover — Entscheid-Folge 69 (Stehender Pass fortgeschrieben; Dropped-Sweep geschlossen; Split-Routing aus Post gefaltet) (Stand 2026-09-20)
  session: Entscheid-Folge 69
  class: handover
  date: 2026-09-20
  sha256: 39eba49ccb624b96f402051e3a77cb1ead7afcb74e99099043f5d9801782ed39
  status: live
-->
# Handover — Entscheid-Folge 69 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 69)

- **HEAD** `9f8e4bdc` == `origin/main` (ernte folge124, fremd). Arbeitsbaum:
  **fremd** `tools/register/src/bin/register_lookup.rs` (`M`, nicht angefasst).
- **Postfach** — `mail_ledger.φ`: letzter Eingang `1789930255` (`info@pine64.org`:
  Ox64 zugesagt, Versanddaten/Telefon erbeten) — **unverändert**; Antwort gesendet
  (`sent_ledger` `1789931195`). `post.md`: nach der Faltung 3 Zeilen (An bau ×2,
  An ernte) — keine mehr an entscheid.
- **CI** — `ci_manage list` 2026-09-20 ~22:2x: `health-check` `35535074295`
  pending; `ci-check` `35534996972` pending, `35534657914` in_progress;
  `tools-build` `35534657902` success; `te-gate` `35534898200` in_progress;
  `free-model-agent-bench` `35534406541` **pending** (P13), `35532892161`
  in_progress; `free-model-bench` `35532890353` pending. Kein Poll.
- **`register_lookup --open`** — 585 offene Zeilen, 3 `[entscheid]`
  (`phi/blocked_sources.φ:21/60/65`, in der Queue); `post.md` offen: 1 (gefaltet).
- **`register_lookup --dropped`** — 436 Paare, 3329 Kandidaten, 1766 dropped, 164
  commit-resolved; die zwei echten Stränge (bau Feld-Grammatik, ernte
  Rosetta-Idempotenz) sind durch forschung/ernte bereits als Post abgegeben,
  Split-Routing an entscheid → der Punkt „ernte-Historie-Analyse" ist geschlossen.

## Operator-Queue (Stand folge69; einfache Sprache, je Frage mit Alter)

3. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   Sicherheitsereignis (Mail 2026-09-20). **Frage:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit
   2026-09-20)
4. **SuperDARN** (`blocked account`) — HF-Radar-Ionosphären-Konvektion; Route
   über Globus + PI-Vereinbarung (`superdarn.ca/piagreement`). **Frage:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
5. **solar-system-open-data REST** (`blocked key`) — `api.le-systeme-solaire.net`
   HTTP 401 (Bearer-Token); Körperdaten ohne Konto nicht abrufbar. **Frage:**
   Konto/Token anlegen? (Alter: seit 2026-09-20)
6. **Amentum Developer** (`blocked account`) — geomagnetisch/aviation-radiation/
   gravity (trial); Registrierung HTTP 200, Zugang nur mit Konto. **Frage:**
   Registrierung `developer.amentum.io/register`? (Alter: seit 2026-09-20)
7. **Split-Routing-Verifikation** (aus `post.md` gefaltet, Ernte folge12–17) —
   `./bin/proton-exit.sh ca` + direct↔tunnel-Nachmessung der 8 `000`-Hosts.
   **Frage:** Operator-Wort/Route (sudo+Netz, nicht in der Ernte-Session)? (Alter:
   seit Ernte folge12–17)
8. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf" (Ziel-Site aktiv
   → Cookie-Editor-Export → `state/cookies/<host>.json`). (Alter: seit 2026-09-16)

## Offen

- **P2/P3/P4 Bench-Läufe** — `wartend` (Auslöser: Run-Abschluss `35532890353` /
  `35532892161`; `ci_manage view` einmalig, Artefakt `free-model-bench.tsv` lesen,
  Ranking eintragen).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027), Rubin-Review (Umzug 2026-09-24). `blockiert` —
  TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block, SuperDARN-Globus,
  GitHub-PII/GC (#4761801), Sonden-Antworten, `ci-check`.

## Benchmark

- **Delegationen (Entscheid-Folge 69):** keine Sub-Agenten — Stehender Pass,
  Dropped-Sweep und Post-Faltung sind eigene, lesende Arbeit (flash-first, kein
  pro/max nötig).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge69.md` (neu)
- Move `handover-2026-09-20-entscheid-folge68.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (entscheid-Zeile gefaltet + gelöscht)
- `docs/zustand/external-state.md` (CI-Zeile + Postfach-Zeile fortgeschrieben)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der P13-Lauf
`35534406541` ist weiter `pending`; kein Dispatch, kein Poll — das Artefakt wird
einmalig gelesen, sobald der Lauf abgeschlossen ist.
